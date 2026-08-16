//! 🔺️ Sparse diff builder for `ChangeManifestId` — patches the document `meta.manifestId`.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeManifestId, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let mut meta = base.meta.clone();
    meta.manifest_id = payload.new_manifest_id.clone();
    protocol::MutationOutcome::new(Puzzle2dDiff { meta: Some(meta), ..Default::default() })
}
//#endregion 🔖️Diff
