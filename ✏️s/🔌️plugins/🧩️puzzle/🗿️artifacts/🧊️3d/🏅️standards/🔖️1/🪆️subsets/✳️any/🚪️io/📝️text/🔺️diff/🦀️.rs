//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Puzzle3dDiffText = String;

semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff);

#[cfg(test)]
#[test]
fn history_edit_puzzle3d_diff_records_roundtrip_the_neutral_empty_delta() {
    use protocol::{DiffBinary, DiffText};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();
    assert!(corpus["diffEmpty"].as_object().unwrap().is_empty());
    let expected = crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff::default();
    let text = expected.print_diff();
    assert_eq!(crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff::parse_diff(&text).unwrap(), expected);
    let binary = expected.encode_diff().unwrap();
    assert_eq!(crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff::decode_diff(&binary).unwrap(), expected);
}
