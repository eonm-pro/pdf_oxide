use pdf_oxide::document::PdfDocument;
use std::time::Instant;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    // Test 1: Fresh doc, extract_text page 0
    {
        let mut doc = PdfDocument::open(&path).unwrap();
        let t = Instant::now();
        let text = doc.extract_text(0).unwrap();
        println!(
            "Fresh doc, extract_text(0): {:.2}ms ({} chars)",
            t.elapsed().as_secs_f64() * 1000.0,
            text.len()
        );
    }

    // Test 2: Fresh doc, extract_text page 1
    {
        let mut doc = PdfDocument::open(&path).unwrap();
        let t = Instant::now();
        let text = doc.extract_text(1).unwrap();
        println!(
            "Fresh doc, extract_text(1): {:.2}ms ({} chars)",
            t.elapsed().as_secs_f64() * 1000.0,
            text.len()
        );
    }

    // Test 3: Fresh doc, extract_text page 50
    {
        let mut doc = PdfDocument::open(&path).unwrap();
        let t = Instant::now();
        let text = doc.extract_text(50).unwrap();
        println!(
            "Fresh doc, extract_text(50): {:.2}ms ({} chars)",
            t.elapsed().as_secs_f64() * 1000.0,
            text.len()
        );
    }

    // Test 4: Fresh doc, extract_text pages 0-5 sequentially
    {
        let mut doc = PdfDocument::open(&path).unwrap();
        for i in 0..6 {
            let t = Instant::now();
            let text = doc.extract_text(i).unwrap();
            println!(
                "Sequential page {}: {:.2}ms ({} chars)",
                i,
                t.elapsed().as_secs_f64() * 1000.0,
                text.len()
            );
        }
    }
}
