//! 🧱️ 🧱️ Note play app commands command — `add-block`.

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
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new block used to also become the
// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
// lowpoly's `add-primitive`).
pub async fn handle(payload: &AddBlock, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let block = create_block_by_kind(&mut ctx.id_owner, &payload.kind, payload.x, payload.y);
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::create_block(block, None, None)]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::note::testkit::{dispatch, note_app};
    use crate::editor::note::NoteCommand;

    #[semio_framework_async_macros::async_test]
    async fn add_block_action_emits_one_op_and_grows_projection() {
        let mut app = note_app();
        let result = dispatch(&mut app, NoteCommand::AddBlock(AddBlock { kind: "text".into(), x: 80.0, y: 80.0 }));
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.blocks.len(), 1);
        assert_eq!(crate::artifacts::note::schema::block_kind(&projection.blocks[0]), "text");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_block_then_undo_round_trip() {
        use semio_framework_plugin::testkit;
        let mut app = note_app();
        testkit::assert_undo_redo_round_trip(&mut app, NoteCommand::AddBlock(AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("snapshot").blocks.len(), 0, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_blocks_table_row_and_column_ops_clamp_at_one() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::AddBlock(AddBlock { kind: "table".into(), x: 0.0, y: 0.0 }));
        let table_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();

        for (field, expected_rows, expected_columns) in [("tableAddRow", 3, 3), ("tableAddColumn", 3, 4), ("tableRemoveRow", 2, 4), ("tableRemoveRow", 1, 4), ("tableRemoveRow", 1, 4), ("tableRemoveColumn", 1, 3)] {
            dispatch(&mut app, NoteCommand::PatchBlocks(crate::editor::note::commands::patch_blocks::PatchBlocks { block_ids: vec![table_id.clone()], field: field.into(), value: String::new() }));
            let projection = app.snapshot().expect("snapshot");
            let block = find_block(&projection.blocks, &table_id).unwrap();
            if let NoteBlockNode::Table { rows, columns, .. } = block {
                assert_eq!(rows.len(), expected_rows, "field {field}");
                assert_eq!(columns.len(), expected_columns, "field {field}");
            } else {
                panic!("expected table block");
            }
        }
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the source block is selected via
    /// the framework's injected `interactionSelect` verb now (`select_blocks`), not an app command —
    /// requires `note_app_with_registry()` (see that helper's own doc comment).
    #[semio_framework_async_macros::async_test]
    async fn duplicate_selection_clones_with_offset() {
        use crate::editor::note::testkit::{note_app_with_registry, select_blocks};
        let mut app = note_app_with_registry();
        dispatch(&mut app, NoteCommand::AddBlock(AddBlock { kind: "text".into(), x: 10.0, y: 10.0 }));
        let source_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
        select_blocks(&mut app, &[&source_id]);

        let result = dispatch(&mut app, NoteCommand::DuplicateSelection(crate::editor::note::commands::duplicate_selection::DuplicateSelection {}));
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.blocks.len(), 2);
        let clone = projection.blocks.iter().find(|block| block_id(block) != source_id).expect("clone block");
        let (x, y, ..) = crate::artifacts::note::schema::block_bounds(clone);
        assert_eq!((x, y), (34.0, 34.0));
    }
}
//#endregion 🧪️Tests
