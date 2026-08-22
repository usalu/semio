//! 🔺️ Sparse diff builder for `ChangeDomain` — patches the document `domain`.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeDomain, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    // 🌐️ `domain` is a document-root singleton field (not a catalog member addressed by id), so
    // there is no missing-target case — only the no-op check applies.
    if payload.new_domain == base.domain {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Domain is already \"{}\".", payload.new_domain));
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { domain: Some(payload.new_domain.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
