//! 🗂️ 🗂️ Draw play app commands command — `drop-layer-kind`.

use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::artifacts::draw::schema::{create_layer_by_kind, find_draw_layer, find_draw_layer_location};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️DocumentHelpers
fn resolve_reorder_target(document: &DrawSnapshot, target_row_id: &str, drop_position: &str) -> (Option<String>, usize) {
    if target_row_id == "draw-play-layers" || target_row_id == "draw-play-layers.empty" {
        return (None, document.layers.len());
    }
    if let Some(layer_id_value) = crate::artifacts::draw::schema::draw_play_layer_id_from_tree_row_id(target_row_id) {
        if let Some(layer) = find_draw_layer(document, &layer_id_value) {
            if drop_position == "inside" {
                if let crate::artifacts::draw::DrawLayerNode::Group(group) = layer {
                    return (Some(group.base.id.clone()), group.children.len());
                }
            }
            if let Some(location) = find_draw_layer_location(document, &layer_id_value) {
                let index = if drop_position == "before" { location.index } else { location.index + 1 };
                return (location.parent_id, index);
            }
        }
    }
    (None, document.layers.len())
}
//#endregion 🔖️DocumentHelpers











#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "drop-layer-kind")]
pub struct DropLayerKind {
    pub kind: String,
    pub target_row_id: String,
    pub drop_position: String,
}

pub fn handle(payload: &DropLayerKind, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_by_kind(&payload.kind);
    let (parent_id, index) = resolve_reorder_target(document, &payload.target_row_id, &payload.drop_position);
    Ok(Emit { artifact_mutations: vec![crate::artifacts::draw::mutations::create_layer(parent_id, Some(index), layer)], ..Default::default() })
}
