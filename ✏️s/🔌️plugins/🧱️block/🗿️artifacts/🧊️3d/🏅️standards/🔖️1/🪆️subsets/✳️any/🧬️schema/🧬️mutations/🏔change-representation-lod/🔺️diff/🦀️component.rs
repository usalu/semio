//! 🔺️ Sparse diff builder for `ChangeRepresentationLod` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dRepresentationsDelta, Block3dRepresentationsPatch, Block3dRepresentationsPatchEntry};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockRepresentation};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeRepresentationLod, base: &Block3dSnapshot) -> Block3dDiff {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else { return Block3dDiff::default(); };
    let replacement = BlockRepresentation { lod: payload.new_lod.clone(), ..existing.clone() };
    Block3dDiff { representations: Some(Block3dRepresentationsDelta { patched: vec![Block3dRepresentationsPatchEntry { id: payload.id.clone(), patch: Block3dRepresentationsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
