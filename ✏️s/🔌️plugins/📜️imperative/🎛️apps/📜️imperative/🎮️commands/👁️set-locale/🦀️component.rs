//! 👁️ 👁️ Imperative play app commands command — `set-locale`.

use crate::apps::imperative::config::ImperativeConfigMutation;
use crate::apps::imperative::engine::ImperativeHost;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

use crate::apps::imperative::config::ImperativeConfig;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
    Ok(Emit::config(vec![ImperativeConfigMutation::SetLocale { value: payload.value.clone() }]))
}
