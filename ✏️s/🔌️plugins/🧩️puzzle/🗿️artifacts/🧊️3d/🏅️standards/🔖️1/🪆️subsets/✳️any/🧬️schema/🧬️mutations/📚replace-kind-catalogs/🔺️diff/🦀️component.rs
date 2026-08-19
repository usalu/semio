//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — patches the document `meta.kindCatalogs`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    // 🗂️ `meta.kindCatalogs` lives on the document-root singleton `meta` (not a catalog member
    // addressed by id), so there is no missing-target case — only the no-op check applies.
    if payload.new_catalogs == base.meta.kind_catalogs {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Kind catalogs are unchanged.");
    }
    let mut meta = base.meta.clone();
    meta.kind_catalogs = payload.new_catalogs.clone();
    protocol::MutationOutcome::new(Puzzle3dDiff { meta: Some(meta), ..Default::default() })
}
//#endregion 🔖️Diff
