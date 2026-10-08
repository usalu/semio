//! 🔺️ Diff for `ChangeObjectKindVariant`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeObjectKindVariant, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_variant == base.object_kind.variant {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Object kind variant is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { variant: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_variant.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
