//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — patches `meta.kindCatalogs`.
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceKindCatalogs, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    // 🗂️ `meta.kindCatalogs` lives on the document-root singleton `meta` (not a catalog member
    // addressed by id), so there is no missing-target case — only the no-op check applies.
    if payload.new_catalogs == base.meta.kind_catalogs {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Kind catalogs are unchanged.");
    }
    let mut meta = base.meta.clone();
    meta.kind_catalogs = payload.new_catalogs.clone();
    protocol::MutationOutcome::new(Puzzle2dDiff { meta: Some(meta), ..Default::default() })
}
//#endregion 🔖️Diff
