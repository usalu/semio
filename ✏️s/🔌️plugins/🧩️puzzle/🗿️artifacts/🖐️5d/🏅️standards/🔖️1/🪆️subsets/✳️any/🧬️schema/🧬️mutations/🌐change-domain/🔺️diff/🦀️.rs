//! 🔺️ Sparse diff builder for `ChangeDomain` — patches the document `domain`.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDomain, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if payload.new_domain == base.domain {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Domain is already \"{}\".", payload.new_domain));
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { domain: Some(payload.new_domain.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
