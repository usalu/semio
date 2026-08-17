//! 🔺️ Sparse diff builder for `ChangeObjectKindLabel` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeObjectKindLabel, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_label == base.object_kind.label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object kind label is already \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(BlockKindIdentity { label: payload.new_label.clone(), ..base.object_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
