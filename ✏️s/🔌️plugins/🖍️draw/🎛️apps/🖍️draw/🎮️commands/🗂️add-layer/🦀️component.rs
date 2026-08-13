//! 🗂️ 🗂️ Draw play app commands command — `add-layer`.

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::artifacts::draw::schema::{create_draw_boolean_layer, create_layer_by_kind, find_draw_layer, find_draw_layer_location, layer_id};
use crate::artifacts::draw::op::{draw_op_for_layer_field, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-layer")]
pub struct AddLayer {
    pub kind: String,
}

pub fn handle(payload: &AddLayer, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_by_kind(&payload.kind);
    let select_id = layer_id(&layer).to_string();
    Ok(Emit {
        artifact_mutations: vec![crate::artifacts::draw::mutations::create_layer(None, Some(document.layers.len()), layer)],
        config_mutations: vec![DrawConfigMutation::SetSelection { ids: vec![select_id] }],
        ..Default::default()
    })
}
