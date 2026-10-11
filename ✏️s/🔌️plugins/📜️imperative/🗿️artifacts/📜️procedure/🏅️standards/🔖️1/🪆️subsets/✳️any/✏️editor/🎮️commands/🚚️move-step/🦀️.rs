//! 🔧️ Procedure command `move-step`: the root-scope step `id` moves to `index`; the scope's sequence chain is re-wired.

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{move_step};
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub id: String,
    pub index: usize,
}

/// 🚚️ One flow-child edit re-wiring the root scope's sequence edges; an unknown id or unchanged order is no edit.
pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| move_step(path, &Default::default(), &payload.id, payload.index))
}
