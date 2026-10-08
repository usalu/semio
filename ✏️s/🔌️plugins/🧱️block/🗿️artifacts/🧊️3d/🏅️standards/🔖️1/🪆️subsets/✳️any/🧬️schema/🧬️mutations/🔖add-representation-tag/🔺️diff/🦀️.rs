//! 🔺️ Diff for `AddRepresentationTag`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta, BlockRepresentationsPatchEntry, BlockRepresentationPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::AddRepresentationTag, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation-tag", payload.id), vec![payload.id.clone()]);
    };
    if existing.tags.contains(&payload.tag) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("representation tag \"{}\" already present", payload.tag));
    }
    let patch = match payload.index.map(|index| index as usize).filter(|index| *index < existing.tags.len()) {
        Some(index) => {
            let tail = &existing.tags[index..];
            BlockRepresentationPatch { tags_removed: tail.to_vec(), tags_added: std::iter::once(payload.tag.clone()).chain(tail.iter().cloned()).collect(), ..Default::default() }
        }
        None => BlockRepresentationPatch { tags_added: vec![payload.tag.clone()], ..Default::default() },
    };
    protocol::MutationOutcome::new(Block3dDiff { representations: Some(BlockRepresentationsDelta { patched: vec![BlockRepresentationsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
