//! 🔺️ Sparse diff builder for `CreateVortexKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dVortexKindsDelta};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::{Block3dVortexKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateVortexKind, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { vortex_kinds: Some(Block3dVortexKindsDelta { added: vec![payload.vortex_kind.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
