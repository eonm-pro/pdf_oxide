use pdf_oxide::document::PdfDocument;
use std::time::Instant;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    // Measure open vs first extraction separately
    let t_open = Instant::now();
    let mut doc = PdfDocument::open(&path).unwrap();
    let open_ms = t_open.elapsed().as_secs_f64() * 1000.0;
    println!("open():         {:.2}ms", open_ms);

    let t_count = Instant::now();
    let pages = doc.page_count().unwrap();
    let count_ms = t_count.elapsed().as_secs_f64() * 1000.0;
    println!("page_count():   {:.2}ms ({} pages)", count_ms, pages);

    // extract_chars (includes font loading but no text assembly)
    let t_chars = Instant::now();
    let chars = doc.extract_chars(0).unwrap();
    let chars_ms = t_chars.elapsed().as_secs_f64() * 1000.0;
    println!("extract_chars(0): {:.2}ms ({} chars) [FIRST CALL]", chars_ms, chars.len());

    // extract_text (should be fast now - fonts already loaded)
    let t_text = Instant::now();
    let text = doc.extract_text(0).unwrap();
    let text_ms = t_text.elapsed().as_secs_f64() * 1000.0;
    println!("extract_text(0):  {:.2}ms ({} chars) [WARM]", text_ms, text.len());

    // Try on algorithms PDF too if we have a second arg
    println!("\n--- Now trying extract_spans ---");
    let mut doc2 = PdfDocument::open(&path).unwrap();

    let t_spans = Instant::now();
    let spans = doc2.extract_spans(0).unwrap();
    let spans_ms = t_spans.elapsed().as_secs_f64() * 1000.0;
    println!("extract_spans(0): {:.2}ms ({} spans) [COLD]", spans_ms, spans.len());

    let t_spans2 = Instant::now();
    let spans2 = doc2.extract_spans(0).unwrap();
    let spans_ms2 = t_spans2.elapsed().as_secs_f64() * 1000.0;
    println!("extract_spans(0): {:.2}ms ({} spans) [WARM]", spans_ms2, spans2.len());

    // Try getting page dimensions (should be cheap, no content parsing)
    let mut doc3 = PdfDocument::open(&path).unwrap();
    let t_dim = Instant::now();
    let _dim_ms = t_dim.elapsed().as_secs_f64() * 1000.0;

    // Then extract text
    let t_after = Instant::now();
    let _text2 = doc3.extract_text(0).unwrap();
    let _after_ms = t_after.elapsed().as_secs_f64() * 1000.0;
}
