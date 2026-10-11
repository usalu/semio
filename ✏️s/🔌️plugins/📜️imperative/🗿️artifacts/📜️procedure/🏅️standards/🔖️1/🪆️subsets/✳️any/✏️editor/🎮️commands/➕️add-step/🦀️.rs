//! 🔧️ Procedure command `add-step`: a new step of `kind` in the root scope at `index` (absent appends), published as the
//! composed `flow` child's leaves (design §20.15).

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{insert_step, next_step_id};
use crate::mutations::ProcedureMutation;
use crate::{Dictionary, ProcedureSnapshot, Step};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-step")]
pub struct AddStep {
    pub kind: String,
    pub index: Option<usize>,
}

/// ➕️ One flow-child edit inserting the step and chaining it into the root scope.
pub fn handle(payload: &AddStep, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| {
        let step = Step { id: next_step_id(path), kind: payload.kind.clone(), params: Dictionary::new(), bodies: Default::default() };
        insert_step(path, &Default::default(), payload.index, step);
    })
}
