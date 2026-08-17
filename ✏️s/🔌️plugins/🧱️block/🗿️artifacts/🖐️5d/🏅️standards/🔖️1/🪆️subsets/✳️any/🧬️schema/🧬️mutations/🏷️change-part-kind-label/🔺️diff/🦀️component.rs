//! 🔺️ Sparse diff builder for `ChangePartKindLabel` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangePartKindLabel, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_label == base.part_kind.label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Part kind label is already \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { label: payload.new_label.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
