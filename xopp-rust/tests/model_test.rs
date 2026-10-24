use xopp_rust::model::{Color, Document, Page, PageBackgroundType, Point, Stroke};
use xopp_rust::io::{load_xopp, save_xopp};

#[test]
fn test_document_creation_and_pages() {
    let mut doc = Document::default();
    assert_eq!(doc.pages.len(), 1);
    doc.add_page();
    assert_eq!(doc.pages.len(), 2);
    assert_eq!(doc.current_page_index, 1);
}

#[test]
fn test_stroke_hit_detection() {
    let mut stroke = Stroke::new(Color::BLACK, 2.0, false);
    stroke.add_point(Point::new(10.0, 10.0, 1.0));
    stroke.add_point(Point::new(50.0, 50.0, 1.0));

    assert!(stroke.hits(&Point::new(30.0, 30.0, 1.0), 3.0));
    assert!(!stroke.hits(&Point::new(100.0, 100.0, 1.0), 3.0));
}
