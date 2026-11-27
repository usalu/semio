use super::*;
use serde_json::Value;

fn cases() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../🧪️tests/🔬️unit/📁️cases.json")).expect("authored format witness")
}

/// 🔮️ lopdf (third-party, test-only) reads the board's pages and every visible label back out of the bytes.
#[test]
fn the_board_is_a_pdf_carrying_every_visible_label() {
    let board: Puzzle2dSnapshot = semio_framework_pack_json::from_json_str(&cases()["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("board json");
    let pdf = serialize_bytes(&board).expect("pdf");
    assert!(pdf.starts_with(b"%PDF-1.4"));
    let document = lopdf::Document::load_mem(&pdf).expect("independent PDF parse");
    let pages: Vec<u32> = document.get_pages().keys().copied().collect();
    assert_eq!(pages.len(), cases()["page"]["pages"].as_u64().unwrap() as usize);
    let text = document.extract_text(&pages).expect("independent PDF page text");
    for label in cases()["page"]["labels"].as_array().unwrap() {
        assert!(text.contains(label.as_str().unwrap()), "PDF page is missing visible label {label}: {text}");
    }
}

#[test]
fn the_pdf_route_extends_the_subset_routes() {
    let pdf = |rows: &[IoEntry]| rows.iter().filter(|entry| entry.into.artifact_kind == "s.stdio.pdf").count();
    assert_eq!(pdf(routes::entries()), 0);
    assert_eq!(pdf(entries()), 1);
    assert_eq!(entries().len(), routes::entries().len() + 1);
}
