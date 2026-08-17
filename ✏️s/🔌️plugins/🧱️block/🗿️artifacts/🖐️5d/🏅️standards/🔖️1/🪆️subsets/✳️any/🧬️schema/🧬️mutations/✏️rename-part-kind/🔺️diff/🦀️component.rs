//! 🔺️ Sparse diff builder for `RenamePartKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenamePartKind, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    // 🪪️ `part_kind` is the document's single root kind (not a catalog member addressed by id), so
    // there is no missing-target case and no collection to collide with — only the no-op check applies.
    if payload.new_name == base.part_kind.name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Part kind name is already \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { name: payload.new_name.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
