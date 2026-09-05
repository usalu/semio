use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::Mutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_dimensions;

/// 🔺️ Diff helper for set-dimensions — a root-scoped singleton pair; `width`/`height` both
/// already equal to `base`'s is `mutation.no-op` (Warning, empty diff). No `mutation.invariant`
/// check: `0x0` is `SemioImageSnapshot::default()`'s own resting state, so zero is a genuinely
/// valid value here, not a domain violation to invent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, width: u32, height: u32) -> protocol::MutationOutcome<SemioImageDiff> {
    if base.width == width && base.height == height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Dimensions are already this value.".to_string());
    }
    Mutation::diff(&SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width, height }), base)
}
