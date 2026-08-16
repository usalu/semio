//! 🔺️ Sparse diff builder for `RemoveRepresentationTag` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dRepresentationsDelta, Block5dRepresentationsPatch, Block5dRepresentationsPatchEntry};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockRepresentation};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveRepresentationTag, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation-tag", payload.id), vec![payload.id.clone()]);
    };
    let tags: Vec<String> = existing.tags.iter().filter(|tag| *tag != &payload.tag).cloned().collect();
    let replacement = BlockRepresentation { tags, ..existing.clone() };
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(Block5dRepresentationsDelta { patched: vec![Block5dRepresentationsPatchEntry { id: payload.id.clone(), patch: Block5dRepresentationsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
