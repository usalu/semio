//! 🔺️ Sparse diff builder for `RenameNodeKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameNodeKind, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { name: payload.new_name.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
