//! 🧩️ 🧩️ Imperative play app commands command — `set-contributions`.

use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::ProcedureSnapshot;
use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "contributions")]
pub struct SetContributions {
    pub json: String,
}

pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    Ok(Emit::config(vec![ImperativeConfigMutation::SetContributions { json: payload.json.clone() }]))
}
