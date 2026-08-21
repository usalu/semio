//! 🗂️ 🗂️ DAG play app commands command — `node-graph-viewport`.

use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

pub async fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::config(vec![DagConfigMutation::SetCamera { x: payload.x, y: payload.y, zoom: payload.zoom }]))
}
