//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Puzzle5dDiffText = String;

semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff);

#[cfg(test)]
#[test]
fn history_edit_puzzle5d_diff_records_roundtrip_the_neutral_empty_delta() {
    use protocol::{DiffBinary, DiffText};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();
    assert!(corpus["diffEmpty"].as_object().unwrap().is_empty());
    let expected = crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff::default();
    let text = expected.print_diff();
    assert_eq!(crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff::parse_diff(&text).unwrap(), expected);
    let binary = expected.encode_diff().unwrap();
    assert_eq!(crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff::decode_diff(&binary).unwrap(), expected);
}

#[cfg(test)]
#[test]
fn history_edit_puzzle5d_diff_preserves_explicit_clear_and_replacement(){
 use protocol::{DiffBinary,DiffText};
 use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🪆️optional/🧫️fixtures/🔣️.json")).unwrap();
 for invalid in corpus["invalidDocuments"].as_array().unwrap(){assert!(Puzzle5dDiff::parse_diff(invalid.as_str().unwrap()).is_err(),"invalid terminal diff: {invalid}");}
 for sample in corpus["samples"].as_array().unwrap(){
  let label=match sample["state"].as_str().unwrap(){"omitted"=>None,"clear"=>Some(None),"replace"=>Some(Some(sample["json"]["label"].as_str().unwrap().into())),_=>unreachable!()};
  let expected=Puzzle5dDiff{label,..Default::default()};
  assert_eq!(Puzzle5dDiff::parse_diff(&expected.print_diff()).unwrap(),expected);
  assert_eq!(Puzzle5dDiff::decode_diff(&expected.encode_diff().unwrap()).unwrap(),expected);
 }
 let cleared=Puzzle5dDiff{kind_catalogs:Some(None),kind_catalogs_extra:Some(None),..Default::default()};
 assert_eq!(Puzzle5dDiff::parse_diff(&cleared.print_diff()).unwrap(),cleared);
 assert_eq!(Puzzle5dDiff::decode_diff(&cleared.encode_diff().unwrap()).unwrap(),cleared);
}
