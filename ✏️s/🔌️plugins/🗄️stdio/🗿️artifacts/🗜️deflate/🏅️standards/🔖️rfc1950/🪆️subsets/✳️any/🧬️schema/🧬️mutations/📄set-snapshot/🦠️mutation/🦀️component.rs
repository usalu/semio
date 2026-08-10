use crate::artifacts::deflate::{DeflateSnapshot};
use crate::artifacts::deflate::schema::mutations::{DeflateMutation, apply_deflate_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut DeflateSnapshot, mutation: &DeflateMutation) {
    apply_deflate_mutation(projection, mutation);
}
