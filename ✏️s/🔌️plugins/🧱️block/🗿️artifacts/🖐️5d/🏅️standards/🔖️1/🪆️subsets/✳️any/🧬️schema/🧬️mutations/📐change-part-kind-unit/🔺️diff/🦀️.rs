//! 🔺️ Diff for `ChangePartKindUnit`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_part_kind_unit::ChangePartKindUnit, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_unit == base.part_kind.unit {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Part kind unit is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { unit: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_unit.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
