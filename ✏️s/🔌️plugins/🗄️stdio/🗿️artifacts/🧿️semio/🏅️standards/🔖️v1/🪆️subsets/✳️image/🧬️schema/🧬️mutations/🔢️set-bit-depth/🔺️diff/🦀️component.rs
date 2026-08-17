use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-bit-depth — a root-scoped singleton field; `bit_depth` already equal to
/// `base.bit_depth` is `mutation.no-op` (Warning, empty diff). No `mutation.invariant` check: `0`
/// is `SemioImageSnapshot::default()`'s own resting state and every `u8` value is structurally
/// valid, so there is no narrower domain to violate here.
pub fn diff(base: &SemioImageSnapshot, bit_depth: u8) -> protocol::MutationOutcome<SemioImageDiff> {
    if base.bit_depth == bit_depth {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bit depth is already this value.".to_string());
    }
    Mutation::diff(&SemioImageMutation::SetBitDepth { bit_depth }, base)
}
