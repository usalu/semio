//! 🔺️ Diff for `RenamePartKind`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RenamePartKind, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    // 🪪️ `part_kind` is the document's single root kind (not a catalog member addressed by id), so
    // there is no missing-target case and no collection to collide with — only the no-op check applies.
    if payload.new_name == base.part_kind.name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Part kind name is already \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { name: Some(payload.new_name.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
