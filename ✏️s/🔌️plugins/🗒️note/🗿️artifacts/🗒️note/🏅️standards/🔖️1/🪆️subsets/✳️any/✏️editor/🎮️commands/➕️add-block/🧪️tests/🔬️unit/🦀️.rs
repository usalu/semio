
use super::*;
use crate::NoteBlockNode;
use crate::editor::note::NoteCommand;
use crate::editor::note::testkit::{dispatch, note_app};
use crate::schema::{block_id, find_block};

#[semio_framework_async_macros::async_test]
async fn add_block_action_emits_one_op_and_grows_projection() {
    let mut app = note_app().await;
    let result = dispatch(&mut app, NoteCommand::AddBlock(AddBlock { kind: "text".into(), x: 80.0, y: 80.0 })).await;
    assert_eq!(result.mutations.len(), 1);
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.blocks.len(), 1);
    assert_eq!(crate::schema::block_kind(&projection.blocks[0]), "text");
}

#[semio_framework_async_macros::async_test]
async fn add_block_then_undo_round_trip() {
    use semio_framework_plugin::testkit;
    let mut app = note_app().await;
    testkit::assert_undo_redo_round_trip(&mut app, NoteCommand::AddBlock(AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("snapshot").blocks.len(), 0, 1).await;
}

#[semio_framework_async_macros::async_test]
async fn patch_blocks_table_row_and_column_ops_clamp_at_one() {
    let mut app = note_app().await;
    dispatch(&mut app, NoteCommand::AddBlock(AddBlock { kind: "table".into(), x: 0.0, y: 0.0 })).await;
    let table_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();

    for (field, expected_rows, expected_columns) in [("tableAddRow", 3, 3), ("tableAddColumn", 3, 4), ("tableRemoveRow", 2, 4), ("tableRemoveRow", 1, 4), ("tableRemoveRow", 1, 4), ("tableRemoveColumn", 1, 3)] {
        dispatch(&mut app, NoteCommand::PatchBlocks(crate::editor::note::commands::patch_blocks::PatchBlocks { block_ids: vec![table_id.clone()], field: field.into(), value: String::new() })).await;
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
    let mut app = note_app_with_registry().await;
    dispatch(&mut app, NoteCommand::AddBlock(AddBlock { kind: "text".into(), x: 10.0, y: 10.0 })).await;
    let source_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
    select_blocks(&mut app, &[&source_id]).await;

    let result = dispatch(&mut app, NoteCommand::DuplicateSelection(crate::editor::note::commands::duplicate_selection::DuplicateSelection {})).await;
    assert_eq!(result.mutations.len(), 1);
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.blocks.len(), 2);
    let clone = projection.blocks.iter().find(|block| block_id(block) != source_id).expect("clone block");
    let (x, y, ..) = crate::schema::block_bounds(clone);
    assert_eq!((x, y), (34.0, 34.0));
}
