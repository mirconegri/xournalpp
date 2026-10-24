use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::model::{Color, Document, Layer, Page, PageBackground, PageBackgroundType, Point, Stroke, StrokeStyle};

// Loads a document from a .xopp or .xoj file
pub fn load_xopp<P: AsRef<Path>>(path: P) -> Result<Document, String> {
    let mut file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut xml_bytes = Vec::new();

    // Try gzip decompression first; if not gzipped, read directly
    let mut decoder = GzDecoder::new(&mut file);
    if decoder.read_to_end(&mut xml_bytes).is_err() {
        xml_bytes.clear();
        file = File::open(&path).map_err(|e| format!("Failed to reopen file: {}", e))?;
        file.read_to_end(&mut xml_bytes).map_err(|e| format!("Failed to read raw file: {}", e))?;
    }

    let xml_str = String::from_utf8(xml_bytes).map_err(|e| format!("Invalid UTF-8 in file: {}", e))?;
    parse_xopp_xml(&xml_str)
}

// Saves a document into compressed .xopp format
pub fn save_xopp<P: AsRef<Path>>(doc: &Document, path: P) -> Result<(), String> {
    let xml = serialize_xopp_xml(doc);
    let file = File::create(&path).map_err(|e| format!("Failed to create output file: {}", e))?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(xml.as_bytes()).map_err(|e| format!("Failed to compress .xopp data: {}", e))?;
    encoder.finish().map_err(|e| format!("Failed to finalize gzip stream: {}", e))?;
    Ok(())
}

// Parses raw XML content into the document model
fn parse_xopp_xml(xml: &str) -> Result<Document, String> {
    let mut doc = Document {
        title: String::from("Untitled"),
        pages: Vec::new(),
        current_page_index: 0,
        is_modified: false,
    };

    let mut current_page: Option<Page> = None;
    let mut current_layer: Option<Layer> = None;

    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(ref e)) => match e.name().as_ref() {
                b"page" => {
                    let mut width = Page::A4_WIDTH;
                    let mut height = Page::A4_HEIGHT;
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"width" {
                            width = std::str::from_utf8(&attr.value).unwrap_or("595.275").parse().unwrap_or(width);
                        } else if attr.key.as_ref() == b"height" {
                            height = std::str::from_utf8(&attr.value).unwrap_or("841.889").parse().unwrap_or(height);
                        }
                    }
                    current_page = Some(Page {
                        width,
                        height,
                        background: PageBackground::default(),
                        layers: Vec::new(),
                        active_layer_index: 0,
                    });
                }
                b"background" => {
                    if let Some(ref mut page) = current_page {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"type" {
                                page.background.bg_type = match attr.value.as_ref() {
                                    b"plain" => PageBackgroundType::Plain,
                                    b"lined" => PageBackgroundType::Lined,
                                    b"grid" => PageBackgroundType::Grid,
                                    b"dotted" => PageBackgroundType::Dotted,
                                    _ => PageBackgroundType::Ruled,
                                };
                            }
                        }
                    }
                }
                b"layer" => {
                    current_layer = Some(Layer::new());
                }
                _ => {}
            },
            Ok(quick_xml::events::Event::Empty(ref e)) => match e.name().as_ref() {
                b"background" => {
                    if let Some(ref mut page) = current_page {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"type" {
                                page.background.bg_type = match attr.value.as_ref() {
                                    b"plain" => PageBackgroundType::Plain,
                                    b"lined" => PageBackgroundType::Lined,
                                    b"grid" => PageBackgroundType::Grid,
                                    b"dotted" => PageBackgroundType::Dotted,
                                    _ => PageBackgroundType::Ruled,
                                };
                            }
                        }
                    }
                }
                _ => {}
            },
            Ok(quick_xml::events::Event::End(ref e)) => match e.name().as_ref() {
                b"layer" => {
                    if let (Some(ref mut page), Some(layer)) = (&mut current_page, current_layer.take()) {
                        page.layers.push(layer);
                    }
                }
                b"page" => {
                    if let Some(mut page) = current_page.take() {
                        if page.layers.is_empty() {
                            page.layers.push(Layer::new());
                        }
                        doc.pages.push(page);
                    }
                }
                _ => {}
            },
            Ok(quick_xml::events::Event::Text(ref e)) => {
                // Parse coordinate stream if reading a stroke tag
                let text = e.unescape().unwrap_or_default();
                if let Some(ref mut layer) = current_layer {
                    if let Some(stroke) = parse_stroke_coords(&text) {
                        layer.add_stroke(stroke);
                    }
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error at position {}: {:?}", reader.buffer_position(), e)),
            _ => {}
        }
        buf.clear();
    }

    if doc.pages.is_empty() {
        doc.pages.push(Page::new_a4(PageBackgroundType::Ruled));
    }

    Ok(doc)
}

// Parses whitespace-separated pairs of coordinates into a Stroke
fn parse_stroke_coords(text: &str) -> Option<Stroke> {
    let tokens: Vec<f32> = text.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if tokens.len() < 2 {
        return None;
    }

    let mut points = Vec::with_capacity(tokens.len() / 2);
    for chunk in tokens.chunks(2) {
        if chunk.len() == 2 {
            points.push(Point::new(chunk[0], chunk[1], 1.0));
        }
    }

    Some(Stroke {
        points,
        color: Color::BLACK,
        width: 1.4,
        style: StrokeStyle::Solid,
        is_highlighter: false,
    })
}

// Serializes the document tree into valid .xopp XML
fn serialize_xopp_xml(doc: &Document) -> String {
    let mut xml = String::from("<?xml version=\"1.0\" standalone=\"no\"?>\n<xournal version=\"0.4.8\">\n");
    xml.push_str(&format!("  <title>{}</title>\n", doc.title));

    for page in &doc.pages {
        let bg_str = match page.background.bg_type {
            PageBackgroundType::Plain => "plain",
            PageBackgroundType::Lined => "lined",
            PageBackgroundType::Ruled => "ruled",
            PageBackgroundType::Grid => "graph",
            PageBackgroundType::Dotted => "dotted",
        };

        xml.push_str(&format!("  <page width=\"{:.3}\" height=\"{:.3}\">\n", page.width, page.height));
        xml.push_str(&format!("    <background type=\"{}\" color=\"#ffffff\"/>\n", bg_str));

        for layer in &page.layers {
            xml.push_str("    <layer>\n");
            for stroke in &layer.strokes {
                let tool = if stroke.is_highlighter { "highlighter" } else { "pen" };
                let hex = format!("#{:06x}", stroke.color.to_hex_rgb());
                xml.push_str(&format!("      <stroke tool=\"{}\" color=\"{}\" width=\"{:.2}\">\n        ", tool, hex, stroke.width));

                for (i, p) in stroke.points.iter().enumerate() {
                    xml.push_str(&format!("{:.2} {:.2} ", p.x, p.y));
                    if (i + 1) % 10 == 0 {
                        xml.push_str("\n        ");
                    }
                }
                xml.push_str("\n      </stroke>\n");
            }
            xml.push_str("    </layer>\n");
        }
        xml.push_str("  </page>\n");
    }

    xml.push_str("</xournal>\n");
    xml
}
