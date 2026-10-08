//! 🔺️ Diff for `RemoveRepresentationTag`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta, BlockRepresentationsPatchEntry, BlockRepresentationPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveRepresentationTag, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation-tag", payload.id), vec![payload.id.clone()]);
    };
    if !existing.tags.contains(&payload.tag) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("representation tag \"{}\" is not present", payload.tag));
    }
    let patch = BlockRepresentationPatch { tags_removed: vec![payload.tag.clone()], ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { representations: BlockRepresentationsDelta { modified: vec![BlockRepresentationsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, ..Default::default() })
}
//#endregion 🔖️Diff
