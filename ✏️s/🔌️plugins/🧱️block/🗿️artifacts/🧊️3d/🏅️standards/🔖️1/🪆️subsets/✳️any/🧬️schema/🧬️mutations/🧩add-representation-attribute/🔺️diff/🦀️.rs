//! 🔺️ Diff for `AddRepresentationAttribute`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta, BlockRepresentationsPatchEntry, BlockRepresentationPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::AddRepresentationAttribute, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation-attribute", payload.id), vec![payload.id.clone()]);
    };
    if existing.attributes.iter().any(|attribute| attribute.key == payload.attribute.key) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("representation attribute \"{}\" already present", payload.attribute.key));
    }
    let patch = match payload.index.map(|index| index as usize).filter(|index| *index < existing.attributes.len()) {
        Some(index) => {
            let tail = &existing.attributes[index..];
            BlockRepresentationPatch { attributes_removed: tail.iter().map(|attribute| attribute.key.clone()).collect(), attributes_added: std::iter::once(payload.attribute.clone()).chain(tail.iter().cloned()).collect(), ..Default::default() }
        }
        None => BlockRepresentationPatch { attributes_added: vec![payload.attribute.clone()], ..Default::default() },
    };
    protocol::MutationOutcome::new(Block3dDiff { representations: Some(BlockRepresentationsDelta { patched: vec![BlockRepresentationsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
