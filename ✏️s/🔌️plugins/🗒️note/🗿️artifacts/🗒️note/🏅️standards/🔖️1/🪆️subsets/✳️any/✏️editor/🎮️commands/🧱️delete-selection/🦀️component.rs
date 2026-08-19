//! 🧱️ 🧱️ Note play app commands command — `delete-selection`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{block_bounds, block_id, block_id_from_tree_row_id, clone_block, create_block_by_kind, find_block, offset_block_tree};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::schema::mutations::{
    change_block_font_size, change_block_ink_width, change_block_locked, change_block_visible, delete_block as delete_block_mutation, delete_blocks as delete_blocks_mutation,
    duplicate_block as duplicate_block_mutation, duplicate_blocks as duplicate_blocks_mutation, edit_block_math, edit_block_text, insert_table_column, insert_table_row,
    move_block as move_block_mutation, move_block_to_container, remove_table_column, remove_table_row, rename_block, resize_block,
};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ctx.selected_block_ids` is the
// "blocks" domain's current selection, resolved once by `ArtifactEditor::handle` — clearing it back to
// empty after the delete is the framework's job (pruned automatically once the ids no longer exist).
pub async fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    if ctx.selected_block_ids.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![delete_blocks_mutation(ctx.selected_block_ids.clone())]))
}
