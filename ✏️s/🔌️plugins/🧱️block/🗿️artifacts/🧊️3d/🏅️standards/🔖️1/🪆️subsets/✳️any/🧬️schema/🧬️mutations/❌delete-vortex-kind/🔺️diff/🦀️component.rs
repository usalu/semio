//! 🔺️ Sparse diff builder for `DeleteVortexKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dVortexKindsDelta};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::{Block3dVortexKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteVortexKind, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { vortex_kinds: Some(Block3dVortexKindsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
