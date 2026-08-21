use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use protocol::Mutation;

/// 🔺️ Diff helper for set-colorspace — a root-scoped singleton field; `colorspace` already equal
/// to `base.colorspace` is `mutation.no-op` (Warning, empty diff). No `mutation.invariant` check:
/// every `SemioColorspace` enum value is structurally valid, there is no narrower domain to
/// violate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(base: &SemioImageSnapshot, colorspace: SemioColorspace) -> protocol::MutationOutcome<SemioImageDiff> {
    if base.colorspace == colorspace {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Colorspace is already this value.".to_string());
    }
    Mutation::diff(&SemioImageMutation::SetColorspace { colorspace }, base)
}
