//! 👁️ 👁️ Imperative play app commands command — `set-selection`.

use crate::apps::imperative::config::ImperativeConfigMutation;
use crate::apps::imperative::engine::ImperativeHost;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

use crate::apps::imperative::config::ImperativeConfig;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
    Ok(Emit::config(vec![ImperativeConfigMutation::SetSelectedSteps { ids: payload.ids.clone() }]))
}
