use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::model::{Color, Document, Layer, Page, PageBackground, PageBackgroundType, Point, Stroke, StrokeStyle};

// Loads a document from a .xopp or .xoj file (gzip or raw XML)
pub fn load_xopp<P: AsRef<Path>>(path: P) -> Result<Document, String> {
    let raw = fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;
    let xml_bytes = decompress_or_pass(&raw)?;
    let xml_str = String::from_utf8(xml_bytes).map_err(|e| format!("Invalid UTF-8: {}", e))?;
    parse_xopp_xml(&xml_str)
}

// Try gzip decompression; fall back to raw bytes if not gzipped
fn decompress_or_pass(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = GzDecoder::new(data);
    let mut out = Vec::new();
    match decoder.read_to_end(&mut out) {
        Ok(_) => Ok(out),
        Err(_) => Ok(data.to_vec()),
    }
}

// Saves a document into compressed .xopp format
pub fn save_xopp<P: AsRef<Path>>(doc: &Document, path: P) -> Result<(), String> {
    let xml = serialize_xopp_xml(doc);
    let file = fs::File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(xml.as_bytes()).map_err(|e| format!("Compress error: {}", e))?;
    encoder.finish().map_err(|e| format!("Finalize error: {}", e))?;
    Ok(())
}

// Active stroke being parsed, holding attributes from the opening tag
struct StrokeBuilder {
    color: Color,
    width: f32,
    is_highlighter: bool,
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
    let mut current_stroke: Option<StrokeBuilder> = None;

    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(ref e)) => match e.name().as_ref() {
                b"title" => {
                    // Read title text on the next event
                }
                b"page" => {
                    let mut width = Page::A4_WIDTH;
                    let mut height = Page::A4_HEIGHT;
                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"width" => {
                                width = parse_attr_f32(&attr.value, width);
                            }
                            b"height" => {
                                height = parse_attr_f32(&attr.value, height);
                            }
                            _ => {}
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
                    apply_background_attrs(e, &mut current_page);
                }
                b"layer" => {
                    current_layer = Some(Layer::new());
                }
                b"stroke" => {
                    let mut color = Color::BLACK;
                    let mut width = 1.4_f32;
                    let mut is_highlighter = false;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"color" => {
                                color = parse_xopp_color(&attr.value);
                            }
                            b"width" => {
                                width = parse_attr_f32(&attr.value, width);
                            }
                            b"tool" => {
                                if attr.value.as_ref() == b"highlighter" {
                                    is_highlighter = true;
                                }
                            }
                            _ => {}
                        }
                    }
                    current_stroke = Some(StrokeBuilder { color, width, is_highlighter });
                }
                _ => {}
            },
            Ok(quick_xml::events::Event::Empty(ref e)) => {
                if e.name().as_ref() == b"background" {
                    apply_background_attrs(e, &mut current_page);
                }
            }
            Ok(quick_xml::events::Event::End(ref e)) => match e.name().as_ref() {
                b"stroke" => {
                    current_stroke = None;
                }
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
                let text = e.unescape().unwrap_or_default();
                // Only parse coordinates when inside a <stroke> tag
                if let (Some(ref mut layer), Some(ref sb)) = (&mut current_layer, &current_stroke) {
                    if let Some(stroke) = parse_stroke_coords(&text, sb) {
                        layer.add_stroke(stroke);
                    }
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(format!("XML error at {}: {:?}", reader.buffer_position(), e)),
            _ => {}
        }
        buf.clear();
    }

    if doc.pages.is_empty() {
        doc.pages.push(Page::new_a4(PageBackgroundType::Ruled));
    }

    Ok(doc)
}

// Extract background type from element attributes and apply to current page
fn apply_background_attrs(e: &quick_xml::events::BytesStart, current_page: &mut Option<Page>) {
    if let Some(ref mut page) = current_page {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref() == b"type" {
                page.background.bg_type = match attr.value.as_ref() {
                    b"plain" => PageBackgroundType::Plain,
                    b"lined" => PageBackgroundType::Lined,
                    b"grid" | b"graph" => PageBackgroundType::Grid,
                    b"dotted" => PageBackgroundType::Dotted,
                    _ => PageBackgroundType::Ruled,
                };
            }
        }
    }
}

// Parse a float from a byte attribute, returning fallback on failure
fn parse_attr_f32(value: &[u8], fallback: f32) -> f32 {
    std::str::from_utf8(value)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback)
}

// Parse xopp color strings like "#000000ff" or named colors
fn parse_xopp_color(value: &[u8]) -> Color {
    let s = std::str::from_utf8(value).unwrap_or("#000000ff");
    match s {
        "black" => Color::BLACK,
        "blue" => Color::BLUE,
        "red" => Color::RED,
        "green" => Color::GREEN,
        "yellow" => Color::HIGHLIGHTER_YELLOW,
        _ if s.starts_with('#') => {
            let hex = s.trim_start_matches('#');
            let val = u32::from_str_radix(hex, 16).unwrap_or(0x000000ff);
            match hex.len() {
                6 => Color::from_hex_rgb(val),
                8 => {
                    let r = ((val >> 24) & 0xFF) as u8;
                    let g = ((val >> 16) & 0xFF) as u8;
                    let b = ((val >> 8) & 0xFF) as u8;
                    let a = (val & 0xFF) as u8;
                    Color::rgba(r, g, b, a)
                }
                _ => Color::BLACK,
            }
        }
        _ => Color::BLACK,
    }
}

// Build a stroke from whitespace-separated coordinate pairs using parsed attributes
fn parse_stroke_coords(text: &str, sb: &StrokeBuilder) -> Option<Stroke> {
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
        color: sb.color,
        width: sb.width,
        style: StrokeStyle::Solid,
        is_highlighter: sb.is_highlighter,
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
                let hex = if stroke.color.a == 255 {
                    format!("#{:06x}ff", stroke.color.to_hex_rgb())
                } else {
                    format!("#{:02x}{:02x}{:02x}{:02x}", stroke.color.r, stroke.color.g, stroke.color.b, stroke.color.a)
                };
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
