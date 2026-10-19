//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — sets `meta.kindCatalogs`.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dMetaPatch;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    // 🗂️ `meta.kindCatalogs` lives on the document-root singleton `meta` (not a catalog member
    // addressed by id), so there is no missing-target case — only the no-op check applies.
    if payload.new_catalogs == base.meta.kind_catalogs {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Kind catalogs are unchanged.");
    }
    protocol::MutationOutcome::new(Puzzle3dDiff { meta: Some(Puzzle3dMetaPatch { kind_catalogs: Some(payload.new_catalogs.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
