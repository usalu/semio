use crate::artifacts::json::schema::mutations::JsonMutation;
use crate::artifacts::json::JsonSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &JsonSnapshot, mutation: &JsonMutation) -> Vec<JsonMutation> {
    <JsonMutation as Mutation<JsonSnapshot>>::inverse(mutation, base).await
}
