use crate::artifacts::svg::schema::mutations::{apply_svg_mutation, SvgMutation};
use crate::artifacts::svg::SvgSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SvgSnapshot, mutation: &SvgMutation) {
    apply_svg_mutation(projection, mutation);
}
