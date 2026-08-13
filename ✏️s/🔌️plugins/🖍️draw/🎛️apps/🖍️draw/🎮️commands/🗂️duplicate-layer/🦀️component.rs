//! 🗂️ 🗂️ Draw play app commands command — `duplicate-layer`.

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::artifacts::draw::schema::{create_draw_boolean_layer, create_layer_by_kind, find_draw_layer, find_draw_layer_location, layer_id};
use crate::artifacts::draw::op::{draw_op_for_layer_field, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

pub fn handle(payload: &DuplicateLayer, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    if payload.layer_id.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![crate::artifacts::draw::mutations::duplicate_layer(payload.layer_id.clone())]))
}
