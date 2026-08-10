use crate::artifacts::json::{JsonSnapshot};
use crate::artifacts::json::schema::mutations::{JsonMutation, apply_json_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut JsonSnapshot, mutation: &JsonMutation) {
    apply_json_mutation(projection, mutation);
}
