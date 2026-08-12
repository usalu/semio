//! 🔺️ Sparse diff builder for `ChangeNodeKindUnit` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeKindUnit, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { node_kind: Some(BlockKindIdentity { unit: payload.new_unit.clone(), ..base.node_kind.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
