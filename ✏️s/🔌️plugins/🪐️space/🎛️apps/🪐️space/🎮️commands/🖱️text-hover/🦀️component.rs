//! 🖱️ 🖱️ S Studio app command — `text-hover`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Hover
/// 🔁️ Shared body for `node_graph_hover` and `text_hover` — both decode an optional `{nodeId}` JSON
/// blob (or accept the raw string as-is) into a `SetHover` config operation.
fn hover_operation(hover_json: &Option<String>) -> Vec<SpaceConfigMutation> {
    let node_id = hover_json.as_deref().and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok().and_then(|parsed| parsed.get("nodeId").and_then(|id| id.as_str().map(str::to_string))).or_else(|| Some(text.to_string())));
    vec![SpaceConfigMutation::SetHover { node_id }]
}

//#endregion 🔖️Hover

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "text-hover")]
pub struct TextHover {
    pub hover_json: Option<String>,
}

pub fn handle(payload: &TextHover, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(hover_operation(&payload.hover_json)))
}
