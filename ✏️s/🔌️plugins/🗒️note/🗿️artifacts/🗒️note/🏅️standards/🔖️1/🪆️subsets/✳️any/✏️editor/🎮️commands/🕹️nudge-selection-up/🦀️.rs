//! 🕹️ 🕹️ Note play app commands command — `nudge-selection-up`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::schema::{block_id, flatten_blocks};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use std::collections::HashSet;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Helpers
/// ✂️ Nudge step magnitudes: `1px` fine, `10px` fast.
const NUDGE_STEP: f64 = 1.0;
const NUDGE_STEP_FAST: f64 = 10.0;

/// 🧬️ Offsets every unlocked selected block by `(dx, dy)` — one `drag-blocks` mutation for the
/// whole gesture (real multi-select drag), never a whole-`blocks` vec swap.
async fn nudge(document: &NoteSnapshot, selected_ids: &[String], dx: f64, dy: f64) -> Emit<NoteMutation, NoteConfigMutation> {
    if selected_ids.is_empty() {
        return Emit::default();
    }
    let selected: HashSet<String> = selected_ids.iter().cloned().collect();
    let ids: Vec<String> = flatten_blocks(&document.blocks)
        .into_iter()
        .filter(|block| selected.contains(block_id(block)))
        .filter(|block| {
            !matches!(
                block,
                NoteBlockNode::Group { locked: true, .. }
                    | NoteBlockNode::Text { locked: true, .. }
                    | NoteBlockNode::Image { locked: true, .. }
                    | NoteBlockNode::Table { locked: true, .. }
                    | NoteBlockNode::Math { locked: true, .. }
                    | NoteBlockNode::Ink { locked: true, .. }
            )
        })
        .map(|block| block_id(block).to_string())
        .collect();
    if ids.is_empty() {
        return Emit::default();
    }
    Emit::mutations(vec![crate::artifacts::note::schema::mutations::drag_blocks(ids, dx, dy)])
}
//#endregion 🔖️Helpers

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "nudge-selection-up")]
pub struct NudgeSelectionUp {}

pub async fn handle(_payload: &NudgeSelectionUp, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(nudge(doc.snapshot, &ctx.selected_block_ids, 0.0, -NUDGE_STEP))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::note::schema::{block_bounds, block_id};
    use crate::editor::note::testkit::{dispatch, note_app_with_registry, select_blocks};
    use crate::editor::note::NoteCommand;

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
            let mut app = note_app_with_registry();
            dispatch(&mut app, NoteCommand::AddBlock(crate::editor::note::commands::add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }));
            let new_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
            select_blocks(&mut app, &[&new_id]);
            let operations = dispatch(&mut app, command.clone()).mutations.len();
            assert_eq!(operations, 1, "{command:?} should emit one operation");
            let projection = app.snapshot().expect("snapshot");
            let (x, y, ..) = block_bounds(&projection.blocks[0]);
            assert_eq!((x, y), (expected_dx, expected_dy), "{command:?} moved block to unexpected position");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn nudge_fast_actions_use_ten_pixel_step() {
        let mut app = note_app_with_registry();
        dispatch(&mut app, NoteCommand::AddBlock(crate::editor::note::commands::add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }));
        let new_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
        select_blocks(&mut app, &[&new_id]);
        dispatch(&mut app, NoteCommand::NudgeSelectionRightFast(crate::editor::note::commands::nudge_selection_right_fast::NudgeSelectionRightFast {}));
        let projection = app.snapshot().expect("snapshot");
        let (x, y, ..) = block_bounds(&projection.blocks[0]);
        assert_eq!((x, y), (10.0, 0.0));
    }
}
//#endregion 🧪️Tests
