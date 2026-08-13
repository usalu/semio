//! 🗂️ 🗂️ Draw play app commands command — `delete-layer`.

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::artifacts::draw::schema::{create_draw_boolean_layer, create_layer_by_kind, find_draw_layer, find_draw_layer_location, layer_id};
use crate::artifacts::draw::op::{draw_op_for_layer_field, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: String,
}

pub fn handle(payload: &DeleteLayer, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    if payload.layer_id.is_empty() || find_draw_layer(document, &payload.layer_id).is_none() {
        return Ok(Emit::default());
    }
    let remaining: Vec<String> = config.selected_ids.iter().filter(|id| **id != payload.layer_id).cloned().collect();
    Ok(Emit { artifact_mutations: vec![crate::artifacts::draw::mutations::delete_layer(payload.layer_id.clone())], config_mutations: vec![DrawConfigMutation::SetSelection { ids: remaining }], ..Default::default() })
}
