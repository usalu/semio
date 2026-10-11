//! 🔧️ Procedure command `set-step-params-at`: step `id` of the addressed scope takes `params` (absolute per key).

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{path_ref_in, set_step_params};
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-step-params-at")]
pub struct SetStepParamsAt {
    pub id: String,
    pub owner: Option<String>,
    pub slot: Option<String>,
    pub params: std::collections::BTreeMap<String, crate::standards::v1::subsets::any::io::text::snapshot::ValueDsl>,
}

/// 🎚️ One flow-child edit of the step's changed params; an unknown id or unchanged params is no edit.
pub fn handle(payload: &SetStepParamsAt, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| {
        let path_ref = path_ref_in(path, payload.owner.as_deref(), payload.slot.as_deref());
        set_step_params(path, &path_ref, &payload.id, crate::standards::v1::subsets::any::io::text::snapshot::value_dsl_map_to_dictionary(&payload.params));
    })
}
