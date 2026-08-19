//! 🔺️ Sparse diff builder for `RenameNodeKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RenameNodeKind, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    // 🪪️ `node_kind` is the document's single root kind (not a catalog member addressed by id), so
    // there is no missing-target case and no collection to collide with — only the no-op check applies.
    if payload.new_name == base.node_kind.name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node kind name is already \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { name: payload.new_name.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
