//! 🔺️ Diff for `RenameObjectKind`.

use crate::BlockKindIdentity;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::Block3dDiff;

//#region 🔖️Diff
pub async fn diff(payload: &super::RenameObjectKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    // 🪪️ `object_kind` is the document's single root kind (not a catalog member addressed by id), so
    // there is no missing-target case and no collection to collide with — only the no-op check applies.
    if payload.new_name == base.object_kind.name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Object kind name is already \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(BlockKindIdentity { name: payload.new_name.clone(), ..base.object_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
