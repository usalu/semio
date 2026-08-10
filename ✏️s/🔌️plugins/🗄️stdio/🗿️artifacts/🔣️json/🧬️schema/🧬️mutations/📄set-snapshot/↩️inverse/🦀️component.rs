use crate::artifacts::json::{JsonSnapshot};
use crate::artifacts::json::schema::mutations::JsonMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &JsonSnapshot, mutation: &JsonMutation) -> Vec<JsonMutation> {
    <JsonMutation as Mutation<JsonSnapshot>>::inverse(mutation, base)
}
