use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::Mutation;

/// 🔺️ Diff helper for set-frame-pixels — an absent BASE frame `index` is `mutation.target-missing`
/// (Error, empty diff). A `rgba8` buffer whose length does not match `base.width * base.height *
/// 4` is `mutation.invariant` (Fatal, empty diff) — the same row-major RGBA8 domain rule this
/// subset's own `🚪️io` serializers/deserializers already enforce for every format.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, index: usize, rgba8: Vec<u8>) -> protocol::MutationOutcome<SemioImageDiff> {
    if index >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {index} does not exist."), [index.to_string()]);
    }
    let expected_len = base.width as usize * base.height as usize * 4;
    if rgba8.len() != expected_len {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Frame index {index} pixel buffer has {} byte(s), expected {expected_len} (width*height*4).", rgba8.len()), [index.to_string()]);
    }
    Mutation::diff(&SemioImageMutation::SetFramePixels { index, rgba8 }, base)
}
