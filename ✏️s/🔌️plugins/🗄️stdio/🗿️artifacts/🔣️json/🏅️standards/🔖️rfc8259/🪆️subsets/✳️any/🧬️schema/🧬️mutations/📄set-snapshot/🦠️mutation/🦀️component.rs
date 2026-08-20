use crate::artifacts::json::schema::mutations::{apply_json_mutation, JsonMutation};
use crate::artifacts::json::JsonSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut JsonSnapshot, mutation: &JsonMutation) {
    apply_json_mutation(projection, mutation);
}
