use crate::artifacts::json::schema::mutations::JsonMutation;
use crate::artifacts::json::JsonSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &JsonSnapshot, mutation: &JsonMutation) -> Vec<JsonMutation> {
    <JsonMutation as Mutation<JsonSnapshot>>::inverse(mutation, base)
}
