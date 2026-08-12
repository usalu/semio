//! 🗂️ Draw play app commands — layer tree mutation vocabulary (constitutional: was `ui`'s
//! `ContentOperations` region, layer-level rows).

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::commands::canvas::DrawSession;
use crate::artifacts::draw::schema::{create_draw_boolean_layer, create_layer_by_kind, find_draw_layer, find_draw_layer_location, layer_id};
use crate::artifacts::draw::op::{draw_op_for_layer_field, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

/// 🩹️ Parses a `PatchLayer`/`PatchLayers` wire `value` as JSON text (falling back to a plain JSON
/// string when it isn't valid JSON) so one `String` wire field covers every heterogeneous
/// `draw_op_for_layer_field` value type (bool/number/string) — mirrors
/// `shooting_protocol::ShootingCommand`'s `PatchShots`/`PatchAssets` shape.
fn patch_value_json(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()))
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️AddLayer
pub mod add_layer {
    use super::*;

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
}
//#endregion 🔖️AddLayer

//#region 🔖️DropLayerKind
pub mod drop_layer_kind {
    use super::*;

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
        let select_id = layer_id(&layer).to_string();
        Ok(Emit { artifact_mutations: vec![crate::artifacts::draw::mutations::create_layer(parent_id, Some(index), layer)], config_mutations: vec![DrawConfigMutation::SetSelection { ids: vec![select_id] }], ..Default::default() })
    }
}
//#endregion 🔖️DropLayerKind

//#region 🔖️MoveLayer
pub mod move_layer {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
}
//#endregion 🔖️MoveLayer

//#region 🔖️DeleteLayer
pub mod delete_layer {
    use super::*;

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
}
//#endregion 🔖️DeleteLayer

//#region 🔖️DuplicateLayer
pub mod duplicate_layer {
    use super::*;

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
}
//#endregion 🔖️DuplicateLayer

//#region 🔖️ToggleLayerVisible
pub mod toggle_layer_visible {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-layer-visible")]
    pub struct ToggleLayerVisible {
        pub layer_id: String,
    }

    pub fn handle(payload: &ToggleLayerVisible, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let document = doc.snapshot;
        match find_draw_layer(document, &payload.layer_id) {
            Some(layer) => {
                let visible = !crate::artifacts::draw::schema::layer_base(layer).visible;
                Ok(Emit::mutations(vec![crate::artifacts::draw::mutations::set_layer_visible(payload.layer_id.clone(), visible)]))
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ToggleLayerVisible

//#region 🔖️CombineBoolean
pub mod combine_boolean {
    use super::*;

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
}
//#endregion 🔖️CombineBoolean

//#region 🔖️PatchLayer
pub mod patch_layer {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-layer")]
    pub struct PatchLayer {
        pub layer_id: String,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchLayer, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let document = doc.snapshot;
        let json_value = patch_value_json(&payload.value);
        match draw_op_for_layer_field(document, &payload.layer_id, &payload.field, &json_value) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchLayer

//#region 🔖️PatchLayers
pub mod patch_layers {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-layers")]
    pub struct PatchLayers {
        pub layer_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchLayers, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let document = doc.snapshot;
        let json_value = patch_value_json(&payload.value);
        let operations: Vec<DrawMutation> = payload.layer_ids.iter().filter_map(|id| draw_op_for_layer_field(document, id, &payload.field, &json_value)).collect();
        if operations.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️PatchLayers

//#region 🔖️SetSelectedOpacity
pub mod set_selected_opacity {
    use super::*;

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
}
//#endregion 🔖️SetSelectedOpacity
