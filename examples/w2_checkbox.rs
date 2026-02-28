//! Check what's leaking from checkbox appearance streams
use pdf_oxide::document::PdfDocument;
use pdf_oxide::object::Object;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = &env::args().nth(1).expect("Usage: w2_checkbox <pdf>");
    println!("=== Checkbox Appearance Stream Analysis: {} ===\n", path);

    let mut doc = PdfDocument::open(path)?;

    // Check page 1 (Copy A) annotations
    let page_obj = doc.get_page(1)?;
    let page_dict = page_obj.as_dict().unwrap();

    let annots = match page_dict.get("Annots") {
        Some(Object::Array(arr)) => arr.clone(),
        Some(Object::Reference(r)) => match doc.load_object(*r)? {
            Object::Array(arr) => arr,
            _ => vec![],
        },
        _ => vec![],
    };

    println!("Page 1 has {} annotations", annots.len());

    for (i, annot_obj) in annots.iter().enumerate() {
        let annot_ref = match annot_obj {
            Object::Reference(r) => *r,
            _ => continue,
        };
        let obj = doc.load_object(annot_ref)?;
        let dict = match obj.as_dict() {
            Some(d) => d,
            None => continue,
        };

        let subtype = dict
            .get("Subtype")
            .and_then(|s| s.as_name())
            .unwrap_or("unknown");

        // We only care about Widget annotations (form fields)
        if subtype != "Widget" {
            continue;
        }

        // Get field type
        let ft = dict.get("FT").and_then(|o| o.as_name()).unwrap_or("none");

        // Check /V value
        let v = dict.get("V");

        // Check /AP structure
        let has_ap = dict.contains_key("AP");

        // Get /T name
        let name = dict
            .get("T")
            .and_then(|o| match o {
                Object::String(s) => Some(String::from_utf8_lossy(s).to_string()),
                _ => None,
            })
            .unwrap_or_default();

        if ft == "Btn" || (has_ap && v.is_some()) {
            println!("\n  Widget #{} (ref {}): FT={}, name={}", i, annot_ref, ft, name);
            println!("    /V = {:?}", v);

            if let Some(ap_obj) = dict.get("AP") {
                let ap = match ap_obj {
                    Object::Reference(r) => doc.load_object(*r)?,
                    other => other.clone(),
                };
                if let Some(ap_dict) = ap.as_dict() {
                    println!("    /AP keys: {:?}", ap_dict.keys().collect::<Vec<_>>());

                    // Check /AP/N - should be dict with state names for checkboxes
                    if let Some(n_obj) = ap_dict.get("N") {
                        match n_obj {
                            Object::Reference(r) => println!("    /AP/N = stream ref {}", r),
                            Object::Dictionary(d) => {
                                println!(
                                    "    /AP/N = dict with keys: {:?}",
                                    d.keys().collect::<Vec<_>>()
                                );
                                // These should be state names like "Off", "Yes", "1", etc.
                            },
                            _ => println!("    /AP/N = {:?}", n_obj),
                        }
                    }
                }
            }
        }

        if i >= 50 {
            break;
        }
    }

    Ok(())
}
