//! 🏭️ Generates `report-strip.pdf`, a small, deliberately varied, fully deterministic PDF 1.7
//! document — built entirely through the real `lopdf` 0.44 object-graph library (the same crate
//! registered as `lopdf-pdf-1-7-mutate` in `../../🔣️oracle.json`), never through this
//! repository's own `encode_pdf`. No wall-clock, no randomness: byte-for-byte reproducible on
//! every run, which is what `test fixture reproduce` checks.
//!
//! Deliberately exercises the surface `pdf@1.7`'s 16 mutation kinds touch: three pages with
//! distinct `MediaBox`es (one also carries a `CropBox`, one a non-zero `Rotate`), each with real
//! extractable `Tj` text content; a non-empty `/Info` `Title`/`Author`; a custom, referenced
//! `/Outlines`-shaped dictionary object under the catalog (a `set-dict-entry`/`remove-dict-entry`
//! target, mirroring the real `set-dict-entry`/`remove-dict-entry` targets already proven against
//! the committed bachelor-thesis document); and a custom trailer entry
//! (`/SemioFixtureMarker`) distinct from the structural `/Size`/`/Root`/`/Info`/`/ID` entries, so
//! `set-trailer-entry`/`remove-trailer-entry` have a clean, non-structural target.
//!
//! Usage: `generate <output.pdf>`.

use lopdf::{dictionary, Document, Object, Stream};
use std::env;

fn text_content_stream(text: &str) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"BT /F1 12 Tf 72 720 Td (");
    for byte in text.as_bytes() {
        match byte {
            b'(' | b')' | b'\\' => {
                out.push(b'\\');
                out.push(*byte);
            }
            other => out.push(*other),
        }
    }
    out.extend_from_slice(b") Tj ET");
    out
}

fn main() {
    let out_path = env::args().nth(1).expect("usage: generate <output.pdf>");
    let bytes = build_pdf();
    std::fs::write(&out_path, &bytes).unwrap_or_else(|error| panic!("writing {out_path}: {error}"));
    eprintln!("wrote {} bytes to {out_path}", bytes.len());
}

fn build_pdf() -> Vec<u8> {
    let mut document = Document::with_version("1.7");

    let pages_id = document.new_object_id();

    let page_specs: [(f32, f32, Option<[f32; 4]>, i64, &str); 3] = [
        (612.0, 792.0, None, 0, "Report strip page one — cover."),
        (595.0, 842.0, Some([10.0, 10.0, 585.0, 832.0]), 0, "Report strip page two — body, with a crop box."),
        (612.0, 792.0, None, 90, "Report strip page three — rotated appendix."),
    ];

    let mut kids = Vec::new();
    for (width, height, crop_box, rotate, text) in page_specs {
        let content_id = document.add_object(Stream::new(dictionary! {}, text_content_stream(text)));
        let mut page_dict = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "MediaBox" => vec![0.into(), 0.into(), Object::Real(width), Object::Real(height)],
            "Rotate" => rotate,
            "Resources" => dictionary! {},
        };
        if let Some(crop) = crop_box {
            page_dict.set("CropBox", Object::Array(crop.iter().map(|value| Object::Real(*value)).collect::<Vec<_>>()));
        }
        let page_id = document.add_object(Object::Dictionary(page_dict));
        kids.push(page_id.into());
    }

    let count = kids.len() as i64;
    document.objects.insert(pages_id, Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => kids, "Count" => count }));

    // 🗂️ A referenced, `/Outlines`-shaped dictionary — a real `set-dict-entry`/`remove-dict-entry`
    // target distinct from any page, mirroring the already-proven pattern against the committed
    // bachelor-thesis document (its own `Outlines`/`Count` and `OpenAction`/`S` targets).
    let outlines_id = document.add_object(Object::Dictionary(dictionary! { "Type" => "Outlines", "Count" => 0 }));

    let catalog_id = document.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
        "Outlines" => outlines_id,
        "PageMode" => "UseOutlines",
    });

    let info_id = document.add_object(dictionary! {
        "Title" => Object::string_literal("Report Strip Fixture"),
        "Author" => Object::string_literal("semio-pdf-1-7-base-fixture-generator"),
    });

    document.trailer.set("Root", catalog_id);
    document.trailer.set("Info", info_id);
    document.trailer.set("SemioFixtureMarker", Object::Integer(7));

    let mut out = Vec::new();
    document.save_to(&mut out).expect("lopdf saves the document it just built");
    out
}
