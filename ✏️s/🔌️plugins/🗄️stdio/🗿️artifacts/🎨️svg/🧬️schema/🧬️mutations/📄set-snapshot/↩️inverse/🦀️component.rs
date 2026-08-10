use crate::artifacts::svg::{SvgSnapshot};
use crate::artifacts::svg::schema::mutations::SvgMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SvgSnapshot, mutation: &SvgMutation) -> Vec<SvgMutation> {
    <SvgMutation as Mutation<SvgSnapshot>>::inverse(mutation, base)
}
