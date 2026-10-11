//! 🔧️ Procedure command `remove-step`: the root-scope step `id` and its nested bodies leave the program; the framework
//! prunes its id from the `steps` selection.

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{move_step, remove_step};
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub id: String,
}

/// ➖️ One flow-child edit: the step's edges, then its nodes, then the re-chained scope; an unknown id is no edit.
pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| remove_step(path, &Default::default(), &payload.id))
}
