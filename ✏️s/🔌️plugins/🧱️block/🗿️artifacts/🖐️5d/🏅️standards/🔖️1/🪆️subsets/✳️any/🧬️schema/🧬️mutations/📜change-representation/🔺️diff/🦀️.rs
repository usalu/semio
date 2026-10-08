//! 🔺️ Diff for `ChangeRepresentationDescription`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta, BlockRepresentationsPatchEntry, BlockRepresentationPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRepresentationDescription, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation", payload.id), vec![payload.id.clone()]);
    };
    if existing.description == payload.new_description {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = BlockRepresentationPatch { description: Some(payload.new_description.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(BlockRepresentationsDelta { patched: vec![BlockRepresentationsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
