use pdf_oxide::document::PdfDocument;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <pdf_file> [<pdf_file> ...]", args[0]);
        std::process::exit(1);
    }

    let mut pass = 0;
    let mut fail = 0;

    for path in &args[1..] {
        print!("Testing: {} ... ", path);

        // Test 1: Open
        let mut doc = match PdfDocument::open(path) {
            Ok(d) => d,
            Err(e) => {
                println!("FAIL (open): {}", e);
                fail += 1;
                continue;
            },
        };

        let page_count = match doc.page_count() {
            Ok(c) => c,
            Err(e) => {
                println!("FAIL (page_count): {}", e);
                fail += 1;
                continue;
            },
        };

        let mut page_errors = Vec::new();

        for page in 0..page_count {
            // Test 2: extract_text (this is what segfaulted)
            match doc.extract_text(page) {
                Ok(text) => {
                    let chars = text.chars().count();
                    // Also try extract_images to cover the image parsing path
                    match doc.extract_images(page) {
                        Ok(imgs) => {
                            // Try converting each image to check for crashes
                            for (i, img) in imgs.iter().enumerate() {
                                match img.to_dynamic_image() {
                                    Ok(_) => {},
                                    Err(e) => {
                                        page_errors.push(format!(
                                            "p{} img{} to_dynamic_image: {}",
                                            page, i, e
                                        ));
                                    },
                                }
                            }
                        },
                        Err(e) => {
                            page_errors.push(format!("p{} extract_images: {}", page, e));
                        },
                    }
                    let _ = chars; // used for reporting below
                },
                Err(e) => {
                    page_errors.push(format!("p{} extract_text: {}", page, e));
                },
            }
        }

        if page_errors.is_empty() {
            println!("OK ({} pages)", page_count);
            pass += 1;
        } else {
            println!("ISSUES ({} pages, {} errors):", page_count, page_errors.len());
            for err in &page_errors {
                println!("  - {}", err);
            }
            fail += 1;
        }
    }

    println!("\n=== Results: {} pass, {} fail ===", pass, fail);
    if fail > 0 {
        std::process::exit(1);
    }
}
