//! 🗂️ 🗂️ Draw play app commands command — `move-layer`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::{find_draw_layer, find_draw_layer_location};
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-layer")]
pub struct MoveLayer {
    pub layer_id: String,
    pub target_row_id: String,
    pub drop_position: String,
}

pub fn handle(payload: &MoveLayer, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let (parent_id, index) = resolve_reorder_target(document, &payload.target_row_id, &payload.drop_position);
    Ok(Emit::mutations(vec![crate::artifacts::draw::mutations::reorder_layer(payload.layer_id.clone(), parent_id, index)]))
}
