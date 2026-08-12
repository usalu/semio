//! 🔺️ Sparse diff builder for `ChangeNodeKindVariant` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeKindVariant, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { node_kind: Some(BlockKindIdentity { variant: payload.new_variant.clone(), ..base.node_kind.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
