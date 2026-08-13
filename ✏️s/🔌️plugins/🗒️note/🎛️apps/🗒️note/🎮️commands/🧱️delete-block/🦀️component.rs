//! 🧱️ 🧱️ Note play app commands command — `delete-block`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
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
#[dsl(keyword = "delete-block")]
pub struct DeleteBlock {
    pub block_id: String,
}

pub fn handle(payload: &DeleteBlock, _doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let selection: Vec<String> = cfg.snapshot.selected_block_ids.iter().filter(|id| **id != payload.block_id).cloned().collect();
    Ok(Emit { artifact_mutations: vec![delete_block_mutation(payload.block_id.clone())], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: selection }], ..Default::default() })
}
