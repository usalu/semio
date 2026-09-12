//! 🕹️ 🕹️ Note play app commands command — `nudge-selection-down`.

use crate::op::NoteMutation;
use crate::schema::{block_id, flatten_blocks};
use crate::{NoteBlockNode, NoteSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashSet;

//#region 🔖️Helpers
/// ✂️ Nudge step magnitudes: `1px` fine, `10px` fast.
const NUDGE_STEP: f64 = 1.0;

/// 🧬️ Offsets every unlocked selected block by `(dx, dy)` — one `drag-blocks` mutation for the
/// whole gesture (real multi-select drag), never a whole-`blocks` vec swap.
fn nudge(document: &NoteSnapshot, selected_ids: &[String], dx: f64, dy: f64) -> Emit<NoteMutation, semio_framework_plugin::NoConfigMutation> {
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
    Emit::mutations(vec![crate::schema::mutations::drag_blocks(ids, dx, dy)])
}
//#endregion 🔖️Helpers

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "nudge-selection-down")]
pub struct NudgeSelectionDown {}

pub fn handle(_payload: &NudgeSelectionDown, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    Ok(nudge(doc.snapshot, &ctx.selected_block_ids, 0.0, NUDGE_STEP))
}
