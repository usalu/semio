//! 🔺️ Sparse diff builder for `ChangeObjectKindIcon` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::BlockKindIdentity;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeObjectKindIcon, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_icon == base.object_kind.icon {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Object kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(BlockKindIdentity { icon: payload.new_icon.clone(), ..base.object_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
