//! 🔺️ Diff for `RenamePartKind`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::rename_part_kind::RenamePartKind, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_name == base.part_kind.name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Part kind name is already \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { name: Some(payload.new_name.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
