use crate::artifacts::txt::schema::mutations::{apply_txt_mutation, TxtMutation};
use crate::artifacts::txt::TxtSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut TxtSnapshot, mutation: &TxtMutation) {
    apply_txt_mutation(projection, mutation);
}
