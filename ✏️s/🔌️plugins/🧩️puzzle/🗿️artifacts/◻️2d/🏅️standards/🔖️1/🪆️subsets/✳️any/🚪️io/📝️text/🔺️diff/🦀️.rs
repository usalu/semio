//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Puzzle2dDiffText = String;

semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff);

#[cfg(test)]
#[test]
fn history_edit_puzzle2d_diff_records_roundtrip_neutral_authored_deltas(){
 use protocol::{DiffBinary,DiffText};
 use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for sample in corpus["samples"].as_array().unwrap(){
  let expected=Puzzle2dDiff{schema:sample["json"]["schema"].as_str().map(str::to_owned),..Default::default()};
  assert_eq!(Puzzle2dDiff::parse_diff(sample["document"].as_str().unwrap()).unwrap(),expected);
  assert_eq!(Puzzle2dDiff::parse_diff(&expected.print_diff()).unwrap(),expected);
  assert_eq!(Puzzle2dDiff::decode_diff(&expected.encode_diff().unwrap()).unwrap(),expected);
 }
 for invalid in corpus["invalidDocuments"].as_array().unwrap(){assert!(Puzzle2dDiff::parse_diff(invalid.as_str().unwrap()).is_err());}
}
