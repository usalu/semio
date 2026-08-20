use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::{apply_gif_mutation, GifMutation};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut GifSnapshot, mutation: &GifMutation) {
    apply_gif_mutation(projection, mutation);
}
