//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use crate::standards::v1::subsets::block::schema::mutations::drag_blocks::diff::*;
use crate::standards::v1::subsets::block::schema::mutations::drag_blocks::DragBlocks;
use crate::NoteDiff;
use crate::NoteSnapshot;

pub fn diff(payload: &DragBlocks, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let mut delta = crate::schema::diff::NoteBlocksDelta::default();
    let mut missing = Vec::new();
    for id in &payload.ids {
        let Some(block) = crate::schema::find_block(&base.blocks, id) else {
            missing.push(id.clone());
            continue;
        };
        let mut moved = block.clone();
        crate::schema::offset_block_tree(&mut moved, payload.dx, payload.dy);
        delta.patched.push(crate::schema::diff::NoteBlockPatchEntry { id: id.clone(), patch: crate::schema::diff::NoteBlockPatch { block_json: Some(semio_framework_pack_json::to_json_string(&moved)) } });
    }
    if delta.patched.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} requested block(s) exist.", payload.ids.len()), payload.ids.clone());
    }
    let outcome = protocol::MutationOutcome::new(NoteDiff { blocks: Some(delta), ..Default::default() });
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} requested block(s) did not exist and were skipped.", missing.len(), payload.ids.len())).at(missing)])
    }
}
}
pub use mutations_codec::*;
