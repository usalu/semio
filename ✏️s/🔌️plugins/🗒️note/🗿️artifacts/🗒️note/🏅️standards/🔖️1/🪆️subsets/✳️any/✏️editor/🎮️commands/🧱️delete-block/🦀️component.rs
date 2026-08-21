//! 🧱️ 🧱️ Note play app commands command — `delete-block`.

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
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-block")]
pub struct DeleteBlock {
    pub block_id: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the deleted block's own removal from
// the "blocks" domain's selection is now the framework's job (`revalidate_interaction_state_after_document_change`
// prunes stale ids against `interaction_topology` after every document dispatch) — this handler no
// longer touches selection at all.
pub async fn handle(payload: &DeleteBlock, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_block_mutation(payload.block_id.clone())]))
}
