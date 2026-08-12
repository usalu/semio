//! 🕹️ Note play app commands — nudge the selection by a fixed step in a direction (or an arbitrary
//! `(dx, dy)`). Document-mutating.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::engine::{block_id, flatten_blocks};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

//#region 🔖️Helpers
/// ✂️ Nudge step magnitudes: `1px` fine, `10px` fast.
const NUDGE_STEP: f64 = 1.0;
const NUDGE_STEP_FAST: f64 = 10.0;

/// 🧬️ Offsets every unlocked selected block by `(dx, dy)` — one `drag-blocks` mutation for the
/// whole gesture (real multi-select drag), never a whole-`blocks` vec swap.
fn nudge(document: &NoteSnapshot, config: &NoteConfig, dx: f64, dy: f64) -> Emit<NoteMutation, NoteConfigMutation> {
    if config.selected_block_ids.is_empty() {
        return Emit::default();
    }
    let selected: HashSet<String> = config.selected_block_ids.iter().cloned().collect();
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

//#region 🔖️NudgeSelection
pub mod nudge_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "nudge-selection")]
    pub struct NudgeSelection {
        pub dx: f64,
        pub dy: f64,
    }

    pub fn handle(payload: &NudgeSelection, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(nudge(doc.snapshot, cfg.snapshot, payload.dx, payload.dy))
    }
}
//#endregion 🔖️NudgeSelection

//#region 🔖️DirectionalNudges
macro_rules! directional_nudge {
    ($module:ident, $Payload:ident, $key:literal, $dx:expr, $dy:expr) => {
        pub mod $module {
            use super::*;

            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
            #[dsl(keyword = $key)]
            pub struct $Payload {}

            pub fn handle(_payload: &$Payload, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
                Ok(nudge(doc.snapshot, cfg.snapshot, $dx, $dy))
            }
        }
    };
}

directional_nudge!(nudge_selection_up, NudgeSelectionUp, "nudge-selection-up", 0.0, -NUDGE_STEP);
directional_nudge!(nudge_selection_down, NudgeSelectionDown, "nudge-selection-down", 0.0, NUDGE_STEP);
directional_nudge!(nudge_selection_left, NudgeSelectionLeft, "nudge-selection-left", -NUDGE_STEP, 0.0);
directional_nudge!(nudge_selection_right, NudgeSelectionRight, "nudge-selection-right", NUDGE_STEP, 0.0);
directional_nudge!(nudge_selection_up_fast, NudgeSelectionUpFast, "nudge-selection-up-fast", 0.0, -NUDGE_STEP_FAST);
directional_nudge!(nudge_selection_down_fast, NudgeSelectionDownFast, "nudge-selection-down-fast", 0.0, NUDGE_STEP_FAST);
directional_nudge!(nudge_selection_left_fast, NudgeSelectionLeftFast, "nudge-selection-left-fast", -NUDGE_STEP_FAST, 0.0);
directional_nudge!(nudge_selection_right_fast, NudgeSelectionRightFast, "nudge-selection-right-fast", NUDGE_STEP_FAST, 0.0);
//#endregion 🔖️DirectionalNudges

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app};
    use crate::apps::note::NoteCommand;
    use crate::artifacts::note::engine::block_bounds;

    #[test]
    fn nudge_direction_actions_move_selection_without_args() {
        for (command, expected_dx, expected_dy) in [
            (NoteCommand::NudgeSelectionUp(nudge_selection_up::NudgeSelectionUp {}), 0.0, -1.0),
            (NoteCommand::NudgeSelectionDown(nudge_selection_down::NudgeSelectionDown {}), 0.0, 1.0),
            (NoteCommand::NudgeSelectionLeft(nudge_selection_left::NudgeSelectionLeft {}), -1.0, 0.0),
            (NoteCommand::NudgeSelectionRight(nudge_selection_right::NudgeSelectionRight {}), 1.0, 0.0),
        ] {
            let mut app = note_app();
            // `addBlock` selects the freshly added block, so the nudge below has something in
            // `cfg.selected_block_ids` to move.
            dispatch(&mut app, NoteCommand::AddBlock(crate::apps::note::commands::block::add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }));
            let operations = dispatch(&mut app, command.clone()).mutations.len();
            assert_eq!(operations, 1, "{command:?} should emit one operation");
            let projection = app.snapshot().expect("snapshot");
            let (x, y, ..) = block_bounds(&projection.blocks[0]);
            assert_eq!((x, y), (expected_dx, expected_dy), "{command:?} moved block to unexpected position");
        }
    }

    #[test]
    fn nudge_fast_actions_use_ten_pixel_step() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::AddBlock(crate::apps::note::commands::block::add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }));
        dispatch(&mut app, NoteCommand::NudgeSelectionRightFast(nudge_selection_right_fast::NudgeSelectionRightFast {}));
        let projection = app.snapshot().expect("snapshot");
        let (x, y, ..) = block_bounds(&projection.blocks[0]);
        assert_eq!((x, y), (10.0, 0.0));
    }
}
//#endregion 🧪️Tests
