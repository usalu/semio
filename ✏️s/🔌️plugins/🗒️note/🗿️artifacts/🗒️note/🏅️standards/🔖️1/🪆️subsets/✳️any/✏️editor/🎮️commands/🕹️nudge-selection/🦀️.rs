//! 🕹️ Note play app command — `nudge-selection`, and the keyboard nudge every directional verb runs: ONE ink tool
//! transaction of ONE relative `drag-blocks` leaf over every unlocked selected block.

use crate::editor::note::commands::ink_apply_events::{note_ink_dispatch, NoteInkPhase, NOTE_INK_GESTURE_KEY};
use crate::op::NoteMutation;
use crate::schema::{block_id, block_locked, flatten_blocks};
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfigMutation};
use semio_framework_tool_machine::ToolYield;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Nudge
/// ✂️ Nudge step magnitudes: `1px` fine, `10px` fast.
pub(crate) const NUDGE_STEP: f64 = 1.0;
pub(crate) const NUDGE_STEP_FAST: f64 = 10.0;

/// 🧬️ The unlocked selected blocks in document order — what a nudge moves.
fn nudge_targets(document: &NoteSnapshot, selected_ids: &[String]) -> Vec<String> {
    flatten_blocks(&document.blocks).into_iter().filter(|block| selected_ids.iter().any(|id| id == block_id(block)) && !block_locked(block)).map(|block| block_id(block).to_string()).collect()
}

/// ⌨️ One keyboard nudge of `verb` by `(dx, dy)`: ONE ink tool transaction of ONE `drag-blocks` over every unlocked
/// selected block — an open canvas gesture it interrupts is aborted `captureLost`; nothing selected, all locked or a
/// zero step leaves zero trace.
pub(crate) fn nudge(doc: &ArtifactView<'_, NoteSnapshot>, ctx: &mut crate::editor::note::NoteDispatchCtx, verb: &str, dx: f64, dy: f64) -> Result<Emit<NoteMutation, NoConfigMutation>, Fault> {
    let ids = nudge_targets(doc.snapshot, &ctx.selected_block_ids);
    if ids.is_empty() || !dx.is_finite() || !dy.is_finite() || (dx, dy) == (0.0, 0.0) {
        return Ok(Emit::default());
    }
    note_ink_dispatch(doc, &mut ctx.window_transient.ink_tool, verb, NoteInkPhase::Once, |_| Ok(vec![ToolYield::upsert(NOTE_INK_GESTURE_KEY, crate::schema::mutations::drag_blocks(ids, dx, dy))]))
}
//#endregion 🔖️Nudge

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "nudge-selection")]
pub struct NudgeSelection {
    pub dx: f64,
    pub dy: f64,
}

pub fn handle(payload: &NudgeSelection, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoConfigMutation>, Fault> {
    nudge(doc, ctx, "nudgeSelection", payload.dx, payload.dy)
}
