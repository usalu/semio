//! 🔺️ Sparse diff builder for `ChangeManifestId` — patches the document `meta.manifestId`.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeManifestId, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    // 🗂️ `meta.manifestId` lives on the document-root singleton `meta` (not a catalog member
    // addressed by id), so there is no missing-target case — only the no-op check applies.
    if payload.new_manifest_id == base.meta.manifest_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Manifest id is unchanged.");
    }
    let mut meta = base.meta.clone();
    meta.manifest_id = payload.new_manifest_id.clone();
    protocol::MutationOutcome::new(Puzzle2dDiff { meta: Some(meta), ..Default::default() })
}
//#endregion 🔖️Diff
