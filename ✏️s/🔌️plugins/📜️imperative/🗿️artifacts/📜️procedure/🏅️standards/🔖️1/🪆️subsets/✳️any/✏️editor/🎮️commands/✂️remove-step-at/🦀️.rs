//! 🔧️ Procedure command `remove-step-at`: step `id` of the addressed scope and its nested bodies leave the program.

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{move_step, path_ref_in, remove_step};
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-step-at")]
pub struct RemoveStepAt {
    pub id: String,
    pub owner: Option<String>,
    pub slot: Option<String>,
}

/// ➖️ One flow-child edit removing the step from its scope; an unknown id is no edit.
pub fn handle(payload: &RemoveStepAt, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| {
        let path_ref = path_ref_in(path, payload.owner.as_deref(), payload.slot.as_deref());
        remove_step(path, &path_ref, &payload.id);
    })
}
