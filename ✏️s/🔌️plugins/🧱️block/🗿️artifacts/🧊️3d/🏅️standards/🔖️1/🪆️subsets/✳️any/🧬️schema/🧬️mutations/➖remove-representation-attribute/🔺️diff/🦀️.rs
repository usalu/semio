//! 🔺️ Diff for `RemoveRepresentationAttribute`.

use crate::{BlockAttribute, BlockRepresentation};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dRepresentationsDelta, Block3dRepresentationsPatch, Block3dRepresentationsPatchEntry};

//#region 🔖️Diff
pub async fn diff(payload: &super::RemoveRepresentationAttribute, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation-attribute", payload.id), vec![payload.id.clone()]);
    };
    let attributes: Vec<BlockAttribute> = existing.attributes.iter().filter(|attribute| attribute.key != payload.key).cloned().collect();
    let replacement = BlockRepresentation { attributes, ..existing.clone() };
    protocol::MutationOutcome::new(Block3dDiff {
        representations: Some(Block3dRepresentationsDelta { patched: vec![Block3dRepresentationsPatchEntry { id: payload.id.clone(), patch: Block3dRepresentationsPatch { replacement: Some(replacement) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
