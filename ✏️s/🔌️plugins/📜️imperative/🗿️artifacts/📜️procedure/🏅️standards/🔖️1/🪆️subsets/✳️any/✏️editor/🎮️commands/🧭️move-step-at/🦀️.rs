//! 🔧️ Procedure command `move-step-at`: step `id` of the addressed scope moves to `index`.

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::schema::operations::{move_step, path_ref_in};
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "move-step-at")]
pub struct MoveStepAt {
    pub id: String,
    pub index: usize,
    pub owner: Option<String>,
    pub slot: Option<String>,
}

/// 🚚️ One flow-child edit re-wiring the scope's chain (a new first step re-points the body edge).
pub fn handle(payload: &MoveStepAt, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    crate::procedure_edit_emit(doc, |path| {
        let path_ref = path_ref_in(path, payload.owner.as_deref(), payload.slot.as_deref());
        move_step(path, &path_ref, &payload.id, payload.index);
    })
}
