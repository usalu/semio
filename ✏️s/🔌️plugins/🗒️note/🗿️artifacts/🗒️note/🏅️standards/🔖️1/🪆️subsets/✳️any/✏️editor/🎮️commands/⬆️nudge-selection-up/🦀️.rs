//! ⬆️ Note play app command — `nudge-selection-up`: one keyboard nudge as ONE ink tool transaction of ONE `drag-blocks`.

use crate::editor::note::commands::nudge_selection::{nudge, NUDGE_STEP};
use crate::op::NoteMutation;
use crate::NoteSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "nudge-selection-up")]
pub struct NudgeSelectionUp {}

pub fn handle(_payload: &NudgeSelectionUp, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    nudge(doc, ctx, "nudgeSelectionUp", 0.0, -NUDGE_STEP)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
