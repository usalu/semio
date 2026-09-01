//! 🔺️ Diff for `ChangeObjectKindDescription`.

use crate::BlockKindIdentity;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::Block3dDiff;

//#region 🔖️Diff
pub async fn diff(payload: &super::ChangeObjectKindDescription, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_description == base.object_kind.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Object kind description is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(BlockKindIdentity { description: payload.new_description.clone(), ..base.object_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
