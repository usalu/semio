use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// 🔺️ Diff helper for set-icc — a root-scoped singleton field; `icc` already equal to
/// `base.icc` (including `None == None`) is `mutation.no-op` (Warning, empty diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, icc: Option<Vec<u8>>) -> protocol::MutationOutcome<SemioImageDiff> {
    if base.icc == icc {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "ICC profile is already this value.".to_string());
    }
    Mutation::diff(&SemioImageMutation::SetIcc { icc }, base)
}
