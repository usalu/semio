//! 🧱️ 🧱️ Note play app commands command — `move-block`.

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-block")]
pub struct MoveBlock {
    pub block_id: String,
    pub target_row_id: String,
    pub drop_position: String,
}

/// 🚚 Reparents `block_id` into `target_row_id`'s container at the drop-appropriate index —
/// dispatches `move-block-to-container` (hierarchy move), never a whole-`blocks` vec swap.
pub async fn handle(payload: &MoveBlock, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let document = doc.snapshot;
    if find_block(&document.blocks, &payload.block_id).is_none() {
        return Ok(Emit::default());
    }
    let target_id = block_id_from_tree_row_id(&payload.target_row_id);
    let parent_id = target_id.as_ref().and_then(|id| find_block(&document.blocks, id).and_then(|entry| if matches!(entry, NoteBlockNode::Group { .. }) { Some(id.clone()) } else { None }));
    let index = if payload.drop_position == "before" {
        0
    } else if let Some(ref parent) = parent_id {
        find_block(&document.blocks, parent)
            .and_then(|entry| match entry {
                NoteBlockNode::Group { children, .. } => Some(children.len()),
                _ => None,
            })
            .unwrap_or(0)
    } else {
        document.blocks.len()
    };
    Ok(Emit::mutations(vec![move_block_to_container(payload.block_id.clone(), parent_id, index)]))
}
