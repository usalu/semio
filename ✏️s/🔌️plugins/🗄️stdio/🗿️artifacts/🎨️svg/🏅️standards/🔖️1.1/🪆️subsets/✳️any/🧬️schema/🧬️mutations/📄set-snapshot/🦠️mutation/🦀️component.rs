use crate::artifacts::svg::schema::mutations::{apply_svg_mutation, SvgMutation};
use crate::artifacts::svg::SvgSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut SvgSnapshot, mutation: &SvgMutation) {
    apply_svg_mutation(projection, mutation);
}
