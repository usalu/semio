//! 🕹️ 🕹️ Note play app commands command — `nudge-selection-down-fast`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{block_id, flatten_blocks};
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



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "nudge-selection-down-fast")]
pub struct NudgeSelectionDownFast {}

pub fn handle(_payload: &NudgeSelectionDownFast, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(nudge(doc.snapshot, cfg.snapshot, 0.0, NUDGE_STEP_FAST))
}
