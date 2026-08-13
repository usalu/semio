//! 🗂️ 🗂️ Draw play app commands command — `set-selected-opacity`.

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::artifacts::draw::schema::{create_draw_boolean_layer, create_layer_by_kind, find_draw_layer, find_draw_layer_location, layer_id};
use crate::artifacts::draw::op::{draw_op_for_layer_field, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "selected-opacity")]
pub struct SetSelectedOpacity {
    pub value: f64,
}

pub fn handle(payload: &SetSelectedOpacity, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let operations: Vec<DrawMutation> = config.selected_ids.iter().filter(|id| find_draw_layer(document, id).is_some()).map(|id| crate::artifacts::draw::mutations::set_layer_opacity(id.clone(), payload.value)).collect();
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(operations, "opacity"))
}
