//! Dump font info for a page
use pdf_oxide::document::PdfDocument;
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: check_fonts <pdf> [page]");
    let page_num: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let path = Path::new(&path);

    let mut doc = PdfDocument::open(path).unwrap();

    // Use extract_spans to see what fonts are used
    let spans = doc.extract_spans(page_num).unwrap();

    let mut fonts: std::collections::HashMap<String, (usize, usize)> =
        std::collections::HashMap::new();
    for span in &spans {
        let entry = fonts.entry(span.font_name.clone()).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += span.text.len();
    }

    let mut font_list: Vec<_> = fonts.into_iter().collect();
    font_list.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    println!("Page {} fonts:", page_num);
    for (name, (count, chars)) in &font_list {
        println!("  {} — {} spans, {} chars", name, count, chars);
    }

    // Try extract_chars for detailed info
    let chars = doc.extract_chars(page_num).unwrap();
    println!("\nTotal chars: {}", chars.len());

    // Check for images on the page (cover pages often have big images)
    let images = doc.extract_images(page_num).unwrap();
    println!("Images: {}", images.len());
    for img in &images {
        println!("  {}x{} {:?}", img.width(), img.height(), img.color_space());
    }
}
