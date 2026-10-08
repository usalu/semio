//! 🔺️ Diff for `ChangeObjectKindLabel`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeObjectKindLabel, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_label == base.object_kind.label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Object kind label is already \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { label: Some(payload.new_label.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
