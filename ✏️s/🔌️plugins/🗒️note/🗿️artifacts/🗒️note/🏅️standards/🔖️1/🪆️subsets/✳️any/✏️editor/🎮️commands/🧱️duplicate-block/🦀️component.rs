//! 🧱️ 🧱️ Note play app commands command — `duplicate-block`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::schema::mutations::{
    change_block_font_size, change_block_ink_width, change_block_locked, change_block_visible, delete_block as delete_block_mutation, delete_blocks as delete_blocks_mutation, duplicate_block as duplicate_block_mutation,
    duplicate_blocks as duplicate_blocks_mutation, edit_block_math, edit_block_text, insert_table_column, insert_table_row, move_block as move_block_mutation, move_block_to_container, remove_table_column, remove_table_row, rename_block,
    resize_block,
};
use crate::artifacts::note::schema::{block_bounds, block_id, block_id_from_tree_row_id, clone_block, create_block_by_kind, find_block, offset_block_tree};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
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
async fn duplicate_blocks(document: &NoteSnapshot, ids: &[String], id_owner: &mut crate::artifacts::note::schema::NoteIdOwner) -> Emit<NoteMutation, NoteConfigMutation> {
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

pub async fn handle(payload: &DuplicateBlock, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(duplicate_blocks(doc.snapshot, std::slice::from_ref(&payload.block_id), &mut ctx.id_owner))
}
