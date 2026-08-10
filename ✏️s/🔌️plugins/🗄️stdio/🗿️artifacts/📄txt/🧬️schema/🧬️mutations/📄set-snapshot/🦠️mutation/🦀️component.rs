use crate::artifacts::txt::{TxtSnapshot};
use crate::artifacts::txt::schema::mutations::{TxtMutation, apply_txt_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut TxtSnapshot, mutation: &TxtMutation) {
    apply_txt_mutation(projection, mutation);
}
