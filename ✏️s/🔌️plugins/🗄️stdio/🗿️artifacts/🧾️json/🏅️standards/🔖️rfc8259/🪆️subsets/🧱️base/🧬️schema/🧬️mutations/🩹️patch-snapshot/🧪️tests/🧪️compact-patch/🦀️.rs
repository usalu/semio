//! 🧪️ `🩹️patch-snapshot` — the compact JSON snapshot patch against the neutral large-field oracle
//! (`📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json`) and its text codec's refusal of malformed hex.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn compact_snapshot_patch_matches_neutral_large_field_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let row = &fixture["nativePilots"]["json"];
    let mut original = row["before"].clone();
    *original.pointer_mut(row["largeValuePath"].as_str().unwrap()).unwrap() = "x".repeat(row["largeValueBytes"].as_u64().unwrap() as usize).into();
    let base: JsonSnapshot = pack::json::from_json_str(&original.to_string()).unwrap();
    let event: editing::SnapshotEditEvent = pack::json::from_json_str(&row["event"].to_string()).unwrap();
    let patch = editing::prepare_snapshot_patch(&base, &event).unwrap();
    let mutation = JsonMutation::PatchSnapshot(PatchSnapshot { patch });
    let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).unwrap();
    let mut expected = original.clone();
    *expected.pointer_mut(row["event"]["path"].as_str().unwrap()).unwrap() = row["event"]["value"].clone();
    let actual: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&next))).unwrap();
    assert_eq!(actual, expected);
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse.len(), 1);
    assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).unwrap(), base);
    let maximum = row["maximumPatchBytes"].as_u64().unwrap() as usize;
    assert!(mutation.encode_op().unwrap().len() < maximum);
    assert!(inverse[0].encode_op().unwrap().len() < maximum);
    assert_eq!(JsonMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(JsonMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    println!("[TRACE] json compact patch preserves unrelated {} byte field through forward/inverse codecs", row["largeValueBytes"]);
}

#[test]
fn malformed_unicode_hex_is_rejected_without_panicking() {
    assert!(text::parse("patch-snapshot patch=€0").is_err());
}
