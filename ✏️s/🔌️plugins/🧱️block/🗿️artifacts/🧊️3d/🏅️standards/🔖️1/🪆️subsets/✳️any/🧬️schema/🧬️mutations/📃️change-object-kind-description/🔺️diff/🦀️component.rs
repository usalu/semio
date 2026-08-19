//! 🔺️ Sparse diff builder for `ChangeObjectKindDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeObjectKindDescription, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_description == base.object_kind.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Object kind description is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(BlockKindIdentity { description: payload.new_description.clone(), ..base.object_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
