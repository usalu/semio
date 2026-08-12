//! 🔺️ Sparse diff builder for `ResizeVortex` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dVorticesDelta, Block3dVorticesPatch, Block3dVorticesPatchEntry};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::{Block3dVortexTemplate};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ResizeVortex, base: &Block3dSnapshot) -> Block3dDiff {
    let Some(existing) = base.vortices.iter().find(|item| item.id == payload.id) else { return Block3dDiff::default(); };
    let replacement = Block3dVortexTemplate { radius: payload.new_radius, ..existing.clone() };
    Block3dDiff { vortices: Some(Block3dVorticesDelta { patched: vec![Block3dVorticesPatchEntry { id: payload.id.clone(), patch: Block3dVorticesPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
