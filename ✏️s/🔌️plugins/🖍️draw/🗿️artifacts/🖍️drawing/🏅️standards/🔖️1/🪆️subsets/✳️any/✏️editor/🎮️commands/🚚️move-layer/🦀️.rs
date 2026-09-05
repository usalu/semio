//! 🗂️ 🗂️ Drawing play app commands command — `move-layer`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::schema::{find_drawing_layer, find_drawing_layer_location};
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

//#region 🔖️DocumentHelpers
fn resolve_reorder_target(document: &DrawingSnapshot, target_row_id: &str, drop_position: &str) -> (Option<String>, usize) {
    if target_row_id == "drawing-play-layers" || target_row_id == "drawing-play-layers.empty" {
        return (None, document.layers.len());
    }
    if let Some(layer_id_value) = crate::artifacts::drawing::schema::drawing_play_layer_id_from_tree_row_id(target_row_id) {
        if let Some(layer) = find_drawing_layer(document, &layer_id_value) {
            if drop_position == "inside" {
                if let crate::artifacts::drawing::DrawingLayerNode::Group(group) = layer {
                    return (Some(group.base.id.clone()), group.children.len());
                }
            }
            if let Some(location) = find_drawing_layer_location(document, &layer_id_value) {
                let index = if drop_position == "before" { location.index } else { location.index + 1 };
                return (location.parent_id, index);
            }
        }
    }
    (None, document.layers.len())
}
//#endregion 🔖️DocumentHelpers

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-layer")]
pub struct MoveLayer {
    pub layer_id: String,
    pub target_row_id: String,
    pub drop_position: String,
}

pub fn handle(payload: &MoveLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, DrawingConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let (parent_id, index) = resolve_reorder_target(document, &payload.target_row_id, &payload.drop_position);
    Ok(Emit::mutations(vec![crate::artifacts::drawing::mutations::reorder_layer(payload.layer_id.clone(), parent_id, index)]))
}
