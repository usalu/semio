//! 🧾️ Canonical language-neutral operation wire witness through the independent JSON oracle.
use crate::{FromValue, ToValue};

/// 🧾️ Asserts a committed language-agnostic wire witness (`…/🧾️wire-witness/🦠️mutation/🔣️.json`) IS the canonical
/// Rust wire of its operation: `FromValue` decodes it and `ToValue` re-encodes exactly the committed JSON under the
/// independent `serde_json` oracle (key order free, number spelling exact — `3.0` for an `f64`). The owner-side half of
/// the `schema-mutation-payload-parity` witness rule; the lint validates the same file against the leaf schema.
pub fn assert_wire_witness<Op>(witness: &str) -> Op
where
    Op: FromValue + ToValue,
{
    let committed: serde_json::Value = serde_json::from_str(witness).unwrap_or_else(|error| panic!("wire witness is not JSON: {error}\n{witness}"));
    let operation = Op::from_value(committed.clone().into()).unwrap_or_else(|error| panic!("wire witness does not decode: {error}\n{witness}"));
    assert_eq!(serde_json::Value::from(operation.to_value()), committed, "wire witness is not the canonical Rust wire");
    operation
}


/// 🎟️ Reads the independently authored physical close authority used by original native conformance laws.
pub fn physical_test_close_grant() -> crate::RetainedCloneGrant {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).expect("plain physical close policy");
    let policy=&fixture["physicalCloseGrant"];
    crate::RetainedCloneGrant { maximum_items:policy["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap() as usize }
}
