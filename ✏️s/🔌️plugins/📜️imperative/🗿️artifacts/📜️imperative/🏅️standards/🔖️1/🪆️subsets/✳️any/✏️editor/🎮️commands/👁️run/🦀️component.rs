//! 👁️ 👁️ Imperative play app commands command — `run`.

use crate::editor::imperative::config::ImperativeConfigMutation;
use crate::editor::imperative::engine::ImperativeHost;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

use crate::editor::imperative::config::ImperativeConfig;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "run")]
pub struct Run {}

pub async fn handle(_payload: &Run, doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
    let host = ImperativeHost::from_snapshot(doc.snapshot.clone());
    let result = host.run();
    let json = serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
    Ok(Emit::config(vec![ImperativeConfigMutation::SetRunOutput { json }]))
}
