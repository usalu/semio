use super::*;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::NoteCommand;
use crate::schema::{block_id, find_block};
use crate::NoteBlockNode;

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
    use semio_framework_plugin::artifact_app_laws;
    let mut app = note_app().await;
    artifact_app_laws::assert_undo_redo_round_trip(&mut app, NoteCommand::AddBlock(AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("snapshot").blocks.len(), 0, 1).await;
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

/// 🕹️ Selection is framework-owned and reaches a command as `NoteDispatchCtx::selected_block_ids`, so this
/// law drives the handler with that resolved selection: the native app harness hands a migrated verb's tool
/// job an EMPTY `InteractionState` (ticket 26/09/17/NOTE-PLUGIN-END-TO-END), while the react shell delivers it
/// (see that ticket's interact probe).
#[semio_framework_async_macros::async_test]
async fn duplicate_selection_clones_with_offset() {
    use crate::schema::mutations::apply_note_mutation;
    let mut ids = crate::schema::NoteIdOwner::new("duplicate-test", 0);
    let source = crate::schema::create_block_by_kind(&mut ids, "text", 10.0, 10.0);
    let source_id = block_id(&source).to_string();
    let document = crate::NoteSnapshot { blocks: vec![source], ..crate::schema::empty_note_snapshot() };
    let history = semio_framework_plugin::HistoryView::empty();
    let mut ctx = crate::editor::note::NoteDispatchCtx { selected_block_ids: vec![source_id.clone()], id_owner: crate::schema::NoteIdOwner::new("duplicate-test", 1), view_state: None, window_transient: Default::default(), window_transient_owner: None };
    let emit = crate::editor::note::commands::duplicate_selection::handle(
        &crate::editor::note::commands::duplicate_selection::DuplicateSelection {},
        &semio_framework_plugin::ArtifactView::new(&document, &history),
        &semio_framework_plugin::ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None },
        &mut ctx,
    )
    .expect("duplicate selection");
    assert_eq!(emit.artifact_mutations.len(), 1, "one duplicate is one semantic mutation");
    let next = apply_note_mutation(&document, &emit.artifact_mutations[0]).expect("apply duplicate");
    assert_eq!(next.blocks.len(), 2);
    let clone = next.blocks.iter().find(|block| block_id(block) != source_id).expect("clone block");
    let (x, y, ..) = crate::schema::block_bounds(clone);
    assert_eq!((x, y), (34.0, 34.0));
}
