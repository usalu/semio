//! 🧱️ 🧱️ Note play app commands command — `duplicate-block`.

use crate::op::NoteMutation;
use crate::schema::mutations::{duplicate_block as duplicate_block_mutation, duplicate_blocks as duplicate_blocks_mutation};
use crate::schema::{clone_block, find_block, offset_block_tree};
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Helpers
/// 🧬️ Clones each of `ids` (present in `document`), offsets the clone by `(24, 24)` — the shared body
/// of `DuplicateBlock`/`DuplicateSelection`. Placement (right after each source, same parent) is
/// computed by `duplicate-block(s)`'s own diff from `base`, so this only builds the finished clone
/// VALUES (deterministic ids/offsets), never touches the tree itself. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the clones used to also become the new
/// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
/// framework's own injected `interactionSelect` handling, never by an app command's `Emit`.
fn duplicate_blocks(document: &NoteSnapshot, ids: &[String], id_owner: &mut crate::schema::NoteIdOwner) -> Emit<NoteMutation, semio_framework_plugin::NoConfigMutation> {
    let mut source_ids = Vec::new();
    let mut blocks = Vec::new();
    for source_id in ids {
        if let Some(block) = find_block(&document.blocks, source_id) {
            let mut cloned = clone_block(id_owner, block);
            offset_block_tree(&mut cloned, 24.0, 24.0);
            source_ids.push(source_id.clone());
            blocks.push(cloned);
        }
    }
    if blocks.is_empty() {
        return Emit::default();
    }
    let mutation = if blocks.len() == 1 { duplicate_block_mutation(source_ids.remove(0), blocks.remove(0)) } else { duplicate_blocks_mutation(source_ids, blocks) };
    Emit::mutations(vec![mutation])
}
//#endregion 🔖️Helpers

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "duplicate-block")]
pub struct DuplicateBlock {
    pub block_id: String,
}

pub fn handle(payload: &DuplicateBlock, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    Ok(duplicate_blocks(doc.snapshot, std::slice::from_ref(&payload.block_id), &mut ctx.id_owner))
}
