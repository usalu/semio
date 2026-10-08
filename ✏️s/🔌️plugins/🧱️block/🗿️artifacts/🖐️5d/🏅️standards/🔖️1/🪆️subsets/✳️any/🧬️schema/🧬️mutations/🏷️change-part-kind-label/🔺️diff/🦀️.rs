//! 🔺️ Diff for `ChangePartKindLabel`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePartKindLabel, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_label == base.part_kind.label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Part kind label is already \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { label: Some(payload.new_label.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
