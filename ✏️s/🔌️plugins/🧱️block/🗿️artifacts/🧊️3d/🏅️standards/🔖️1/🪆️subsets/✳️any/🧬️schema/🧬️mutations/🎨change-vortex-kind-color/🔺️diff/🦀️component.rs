//! 🔺️ Sparse diff builder for `ChangeVortexKindColor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dVortexKindsDelta, Block3dVortexKindsPatch, Block3dVortexKindsPatchEntry};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::{Block3dVortexKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeVortexKindColor, base: &Block3dSnapshot) -> Block3dDiff {
    let Some(existing) = base.vortex_kinds.iter().find(|item| item.id == payload.id) else { return Block3dDiff::default(); };
    let replacement = Block3dVortexKind { color: payload.new_color.clone(), ..existing.clone() };
    Block3dDiff { vortex_kinds: Some(Block3dVortexKindsDelta { patched: vec![Block3dVortexKindsPatchEntry { id: payload.id.clone(), patch: Block3dVortexKindsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
