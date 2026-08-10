use crate::artifacts::zip::{ZipSnapshot};
use crate::artifacts::zip::schema::mutations::ZipMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &ZipSnapshot, mutation: &ZipMutation) -> Vec<ZipMutation> {
    <ZipMutation as Mutation<ZipSnapshot>>::inverse(mutation, base)
}
