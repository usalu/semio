use crate::artifacts::json::schema::mutations::{apply_json_mutation, JsonMutation};
use crate::artifacts::json::JsonSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut JsonSnapshot, mutation: &JsonMutation) {
    apply_json_mutation(projection, mutation);
}
