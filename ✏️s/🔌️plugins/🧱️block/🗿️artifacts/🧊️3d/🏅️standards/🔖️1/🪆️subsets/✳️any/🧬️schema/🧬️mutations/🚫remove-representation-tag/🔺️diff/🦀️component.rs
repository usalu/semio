//! 🔺️ Sparse diff builder for `RemoveRepresentationTag` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dRepresentationsDelta, Block3dRepresentationsPatch, Block3dRepresentationsPatchEntry};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::BlockRepresentation;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RemoveRepresentationTag, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation-tag", payload.id), vec![payload.id.clone()]);
    };
    let tags: Vec<String> = existing.tags.iter().filter(|tag| *tag != &payload.tag).cloned().collect();
    let replacement = BlockRepresentation { tags, ..existing.clone() };
    protocol::MutationOutcome::new(Block3dDiff {
        representations: Some(Block3dRepresentationsDelta { patched: vec![Block3dRepresentationsPatchEntry { id: payload.id.clone(), patch: Block3dRepresentationsPatch { replacement: Some(replacement) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
