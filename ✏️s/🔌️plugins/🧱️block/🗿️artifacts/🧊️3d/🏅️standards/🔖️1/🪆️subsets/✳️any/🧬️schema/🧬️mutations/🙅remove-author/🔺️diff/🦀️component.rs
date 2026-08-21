//! 🔺️ Sparse diff builder for `RemoveAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dAuthorList;
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::BlockAuthor;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RemoveAuthor, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.authors.iter().any(|author| author.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "author", payload.id), vec![payload.id.clone()]);
    }
    let values: Vec<BlockAuthor> = base.authors.iter().filter(|author| author.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(Block3dDiff { authors: Some(Block3dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
