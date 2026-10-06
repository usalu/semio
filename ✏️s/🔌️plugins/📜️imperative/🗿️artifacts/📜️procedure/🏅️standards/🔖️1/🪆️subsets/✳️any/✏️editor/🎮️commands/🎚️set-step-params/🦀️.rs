//! 🔧️ Procedure command `set-step-params`: the root-scope step `id` takes `params`; each changed key is one absolute
//! `set-node-param` (a dropped key one `remove-node-param`) of the composed `flow` child.

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{set_step_params};
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-step-params")]
pub struct SetStepParams {
    pub id: String,
    pub params: std::collections::BTreeMap<String, crate::standards::v1::subsets::any::io::text::snapshot::ValueDsl>,
}

/// 🎚️ One flow-child edit of the step's changed params; an unknown id or unchanged params is no edit.
pub fn handle(payload: &SetStepParams, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| set_step_params(path, &Default::default(), &payload.id, crate::standards::v1::subsets::any::io::text::snapshot::value_dsl_map_to_dictionary(&payload.params)))
}
