//! 🧩️ 🧩️ Imperative play app commands command — `set-contributions`.

use crate::editor::imperative::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "contributions")]
pub struct SetContributions {
    pub json: String,
}

pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
    Ok(Emit::config(vec![ImperativeConfigMutation::SetContributions { json: payload.json.clone() }]))
}
