//! 🧱️ 🧱️ Note play app commands command — `delete-selection`.

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
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

pub fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let config = cfg.snapshot;
    if config.selected_block_ids.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![delete_blocks_mutation(config.selected_block_ids.clone())], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
}
