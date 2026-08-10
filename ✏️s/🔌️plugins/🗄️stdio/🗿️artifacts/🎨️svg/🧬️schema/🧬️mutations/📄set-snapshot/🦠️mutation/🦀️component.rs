use crate::artifacts::svg::{SvgSnapshot};
use crate::artifacts::svg::schema::mutations::{SvgMutation, apply_svg_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SvgSnapshot, mutation: &SvgMutation) {
    apply_svg_mutation(projection, mutation);
}
