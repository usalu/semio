//! 🧱️ 🧱️ Note play app commands command — `patch-blocks`.

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
#[dsl(keyword = "patch-blocks")]
pub struct PatchBlocks {
    pub block_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

/// 🩹️ Routes the inspector's typed field/value pair to the one narrow semantic mutation that
/// owns it — one mutation per (id, field) pair, batched into a single `Emit` so a multi-select
/// patch is still one undo step. Replaces the old `note_engine::patch_block_field`
/// whole-document-clone + whole-collection re-dump.
pub async fn handle(payload: &PatchBlocks, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    if payload.block_ids.is_empty() || payload.field.is_empty() {
        return Ok(Emit::default());
    }
    let document = doc.snapshot;
    let mut mutations = Vec::new();
    for id in &payload.block_ids {
        let Some(block) = find_block(&document.blocks, id) else { continue };
        let (x, y, width, height) = block_bounds(block);
        let mutation = match payload.field.as_str() {
            "name" => Some(rename_block(id.clone(), payload.value.clone())),
            "visible" => Some(change_block_visible(id.clone(), payload.value.parse::<bool>().unwrap_or(false))),
            "locked" => Some(change_block_locked(id.clone(), payload.value.parse::<bool>().unwrap_or(false))),
            "x" => Some(move_block_mutation(id.clone(), payload.value.parse::<f64>().unwrap_or(0.0), y)),
            "y" => Some(move_block_mutation(id.clone(), x, payload.value.parse::<f64>().unwrap_or(0.0))),
            "width" => Some(resize_block(id.clone(), payload.value.parse::<f64>().unwrap_or(0.0), height)),
            "height" => Some(resize_block(id.clone(), width, payload.value.parse::<f64>().unwrap_or(0.0))),
            "textContent" => Some(edit_block_text(id.clone(), vec![NoteTextParagraph { runs: vec![NoteTextRun { text: payload.value.clone(), bold: None, italic: None, underline: None, link: None }] }])),
            "textSize" => Some(change_block_font_size(id.clone(), payload.value.parse::<f64>().unwrap_or(18.0))),
            "mathTex" => Some(edit_block_math(id.clone(), payload.value.clone())),
            "inkWidth" => Some(change_block_ink_width(id.clone(), payload.value.parse::<f64>().unwrap_or(3.0))),
            "tableAddRow" => Some(insert_table_row(id.clone())),
            "tableRemoveRow" => Some(remove_table_row(id.clone())),
            "tableAddColumn" => Some(insert_table_column(id.clone())),
            "tableRemoveColumn" => Some(remove_table_column(id.clone())),
            _ => None,
        };
        if let Some(mutation) = mutation {
            mutations.push(mutation);
        }
    }
    if mutations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(mutations))
}
