//! 🔺️ Sparse diff builder for `ChangeManifestId` — patches the document `meta.manifestId`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dMetaPatch};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeManifestId, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    // 🗂️ `meta.manifestId` lives on the document-root singleton `meta` (not a catalog member
    // addressed by id), so there is no missing-target case — only the no-op check applies.
    if payload.new_manifest_id == base.meta.manifest_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Manifest id is unchanged.");
    }
    protocol::MutationOutcome::new(Puzzle2dDiff { meta: Some(Puzzle2dMetaPatch { manifest_id: Some(payload.new_manifest_id.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
