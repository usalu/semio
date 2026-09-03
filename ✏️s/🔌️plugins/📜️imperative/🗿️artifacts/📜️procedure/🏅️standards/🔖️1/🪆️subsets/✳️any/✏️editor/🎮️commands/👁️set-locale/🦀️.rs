//! 👁️ 👁️ Imperative play app commands command — `set-locale`.

use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::ProcedureSnapshot;
use crate::editor::procedure::config::ImperativeConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::editor::procedure::config::ImperativeConfig;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    Ok(Emit::config(vec![ImperativeConfigMutation::SetLocale { value: payload.value.clone() }]))
}
