//! 🔺️ Sparse diff builder for `ChangeDomain` — patches the document `domain`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeDomain, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    // 🌐️ `domain` is a document-root singleton field (not a catalog member addressed by id), so
    // there is no missing-target case — only the no-op check applies.
    if payload.new_domain == base.domain {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Domain is already \"{}\".", payload.new_domain));
    }
    protocol::MutationOutcome::new(Puzzle3dDiff { domain: Some(payload.new_domain.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
