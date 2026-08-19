//! 🔺️ Sparse diff builder for `RenameRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dRepresentationsDelta, Block5dRepresentationsPatch, Block5dRepresentationsPatchEntry};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockRepresentation};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RenameRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation", payload.id), vec![payload.id.clone()]);
    };
    let replacement = BlockRepresentation { name: payload.new_name.clone(), ..existing.clone() };
    if replacement == *existing {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(Block5dRepresentationsDelta { patched: vec![Block5dRepresentationsPatchEntry { id: payload.id.clone(), patch: Block5dRepresentationsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
