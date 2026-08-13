//! 🗂️ 🗂️ Draw play app commands command — `combine-boolean`.

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::artifacts::draw::schema::{create_draw_boolean_layer, create_layer_by_kind, find_draw_layer, find_draw_layer_location, layer_id};
use crate::artifacts::draw::op::{draw_op_for_layer_field, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "combine-boolean")]
pub struct CombineBoolean {
    pub operation: String,
    pub ids: Vec<String>,
}

pub fn handle(payload: &CombineBoolean, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let ids: Vec<String> = if payload.ids.is_empty() { config.selected_ids.clone() } else { payload.ids.clone() };
    if ids.len() < 2 {
        return Ok(Emit::default());
    }
    let layer = create_draw_boolean_layer("Boolean", &payload.operation, ids);
    let select_id = layer_id(&layer).to_string();
    Ok(Emit {
        artifact_mutations: vec![crate::artifacts::draw::mutations::create_layer(None, Some(document.layers.len()), layer)],
        config_mutations: vec![DrawConfigMutation::SetSelection { ids: vec![select_id] }],
        ..Default::default()
    })
}
