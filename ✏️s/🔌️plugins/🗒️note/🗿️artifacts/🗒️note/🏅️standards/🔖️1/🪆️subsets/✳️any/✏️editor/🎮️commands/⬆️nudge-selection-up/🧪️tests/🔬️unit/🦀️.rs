use super::*;
use crate::editor::note::testkit::{dispatch, note_app_with_registry, select_blocks};
use crate::editor::note::NoteCommand;
use crate::schema::{block_bounds, block_id};

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `addBlock` no longer auto-selects
/// the freshly added block (selection is framework-owned now) — `select_blocks` dispatches the
/// injected `interactionSelect` verb against the "blocks" domain instead, requiring
/// `note_app_with_registry()` (see that helper's own doc comment).
#[semio_framework_async_macros::async_test]
async fn nudge_direction_actions_move_selection_without_args() {
    for (command, expected_dx, expected_dy) in [
        (NoteCommand::NudgeSelectionUp(NudgeSelectionUp {}), 0.0, -1.0),
        (NoteCommand::NudgeSelectionDown(crate::editor::note::commands::nudge_selection_down::NudgeSelectionDown {}), 0.0, 1.0),
        (NoteCommand::NudgeSelectionLeft(crate::editor::note::commands::nudge_selection_left::NudgeSelectionLeft {}), -1.0, 0.0),
        (NoteCommand::NudgeSelectionRight(crate::editor::note::commands::nudge_selection_right::NudgeSelectionRight {}), 1.0, 0.0),
    ] {
        let mut app = note_app_with_registry().await;
        dispatch(&mut app, NoteCommand::AddBlock(crate::editor::note::commands::add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 })).await;
        let new_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
        select_blocks(&mut app, &[&new_id]).await;
        let operations = dispatch(&mut app, command.clone()).await.mutations.len();
        assert_eq!(operations, 1, "{command:?} should emit one operation");
        let projection = app.snapshot().expect("snapshot");
        let (x, y, ..) = block_bounds(&projection.blocks[0]);
        assert_eq!((x, y), (expected_dx, expected_dy), "{command:?} moved block to unexpected position");
    }
}

#[semio_framework_async_macros::async_test]
async fn nudge_fast_actions_use_ten_pixel_step() {
    let mut app = note_app_with_registry().await;
    dispatch(&mut app, NoteCommand::AddBlock(crate::editor::note::commands::add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 })).await;
    let new_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
    select_blocks(&mut app, &[&new_id]).await;
    dispatch(&mut app, NoteCommand::NudgeSelectionRightFast(crate::editor::note::commands::nudge_selection_right_fast::NudgeSelectionRightFast {})).await;
    let projection = app.snapshot().expect("snapshot");
    let (x, y, ..) = block_bounds(&projection.blocks[0]);
    assert_eq!((x, y), (10.0, 0.0));
}
