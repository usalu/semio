//! 🔺️ Diff for `ChangePartKindVariant`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_part_kind_variant::ChangePartKindVariant, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_variant == base.part_kind.variant {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Part kind variant is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { variant: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_variant.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
