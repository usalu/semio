//! 🔺️ Diff for `ChangeObjectKindUnit`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeObjectKindUnit, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_unit == base.object_kind.unit {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Object kind unit is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { unit: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_unit.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
