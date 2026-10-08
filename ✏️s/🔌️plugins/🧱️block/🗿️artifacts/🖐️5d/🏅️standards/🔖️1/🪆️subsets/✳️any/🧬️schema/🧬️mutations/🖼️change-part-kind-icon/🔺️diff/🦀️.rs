//! 🔺️ Diff for `ChangePartKindIcon`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePartKindIcon, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_icon == base.part_kind.icon {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Part kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { icon: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_icon.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
