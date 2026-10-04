//! 🔧️ Procedure command `add-step-at`: a new step of `kind` in the addressed scope (a control step's body slot, else the
//! root) at `index`, published as the composed `flow` child's leaves (design §20.15).

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{insert_step, next_step_id, path_ref_in};
use crate::mutations::ProcedureMutation;
use crate::{Dictionary, ProcedureSnapshot, Step};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "add-step-at")]
pub struct AddStepAt {
    pub kind: String,
    pub index: Option<usize>,
    pub owner: Option<String>,
    pub slot: Option<String>,
}

/// ➕️ One flow-child edit inserting the step and chaining it into its scope (a body's first step re-points the body edge).
pub fn handle(payload: &AddStepAt, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| {
        let path_ref = path_ref_in(path, payload.owner.as_deref(), payload.slot.as_deref());
        let step = Step { id: next_step_id(path), kind: payload.kind.clone(), params: Dictionary::new(), bodies: Default::default() };
        insert_step(path, &path_ref, payload.index, step);
    })
}
