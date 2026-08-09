//! 🖼️ Raster play app commands — layer-tree mutations (add/drop/visibility/delete/duplicate/patch/move).
//! All real, undoable document mutations except `setLayerVisible`/`toggleLayerVisible`/`deleteLayer`/
//! `duplicateLayer`/`moveLayer`, which also touch selection as a config side effect.

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::apps::raster::layer_id_from_tree_row_id;
use crate::artifacts::raster::engine::{clone_layer, create_layer_of_kind, find_layer, layer_node_id, layer_patch_for_field, layer_visible};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shared
/// 🩹️ Builds `PatchLayer` operations for a `patchLayer`/`patchLayers` field write across ids — shared by
/// both payloads below (the only two consumers).
fn raster_patch_layer_operations(document: &RasterSnapshot, layer_ids: &[String], field: &str, value: &Value) -> Vec<RasterMutation> {
    layer_ids
        .iter()
        .filter_map(|layer_id| {
            let prior = find_layer(&document.layers, layer_id)?;
            let patch = layer_patch_for_field(field, value, prior)?;
            Some(RasterMutation::PatchLayer { layer_id: layer_id.clone(), patch })
        })
        .collect()
}

/// 🩹️ Parses a `patchLayer`/`patchLayers` wire `value` as JSON text (falling back to a plain JSON string
/// when it isn't valid JSON) — mirrors `draw_ui::patch_value_json`.
fn patch_value_json(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()))
}
//#endregion 🔖️Shared

//#region 🔖️AddLayer
pub mod add_layer {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-layer")]
    pub struct AddLayer {
        pub kind: String,
    }

    pub fn handle(payload: &AddLayer, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let document = doc.snapshot;
        let layer = create_layer_of_kind(&payload.kind);
        let select_id = layer_node_id(&layer).to_string();
        Ok(Emit {
            document_mutations: vec![RasterMutation::AddLayer { parent_id: None, index: document.layers.len(), layer: Box::new(layer) }],
            config_mutations: vec![RasterConfigMutation::SetSelection { ids: vec![select_id] }],
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
    }

    pub fn handle(payload: &DropLayerKind, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let document = doc.snapshot;
        let layer = create_layer_of_kind(&payload.kind);
        let select_id = layer_node_id(&layer).to_string();
        Ok(Emit {
            document_mutations: vec![RasterMutation::AddLayer { parent_id: None, index: document.layers.len(), layer: Box::new(layer) }],
            config_mutations: vec![RasterConfigMutation::SetSelection { ids: vec![select_id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️DropLayerKind

//#region 🔖️SetLayerVisible
pub mod set_layer_visible {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-layer-visible")]
    pub struct SetLayerVisible {
        pub layer_id: String,
        pub visible: Option<bool>,
    }

    pub fn handle(payload: &SetLayerVisible, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let document = doc.snapshot;
        let Some(layer) = find_layer(&document.layers, &payload.layer_id) else { return Ok(Emit::default()) };
        let resolved = payload.visible.unwrap_or_else(|| !layer_visible(layer));
        Ok(Emit::mutations(vec![RasterMutation::PatchLayer { layer_id: payload.layer_id.clone(), patch: RasterLayerPatch { visible: Some(resolved), ..Default::default() } }]))
    }
}
//#endregion 🔖️SetLayerVisible

//#region 🔖️ToggleLayerVisible
pub mod toggle_layer_visible {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-layer-visible")]
    pub struct ToggleLayerVisible {
        pub layer_id: String,
    }

    pub fn handle(payload: &ToggleLayerVisible, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let document = doc.snapshot;
        let Some(layer) = find_layer(&document.layers, &payload.layer_id) else { return Ok(Emit::default()) };
        let resolved = !layer_visible(layer);
        Ok(Emit::mutations(vec![RasterMutation::PatchLayer { layer_id: payload.layer_id.clone(), patch: RasterLayerPatch { visible: Some(resolved), ..Default::default() } }]))
    }
}
//#endregion 🔖️ToggleLayerVisible

//#region 🔖️DeleteLayer
pub mod delete_layer {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-layer")]
    pub struct DeleteLayer {
        pub layer_id: String,
    }

    pub fn handle(payload: &DeleteLayer, doc: &DocumentView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let document = doc.snapshot;
        if find_layer(&document.layers, &payload.layer_id).is_none() {
            return Ok(Emit::default());
        }
        let remaining: Vec<String> = cfg.snapshot.selected_ids.iter().filter(|id| **id != payload.layer_id).cloned().collect();
        Ok(Emit { document_mutations: vec![RasterMutation::RemoveLayer { layer_id: payload.layer_id.clone() }], config_mutations: vec![RasterConfigMutation::SetSelection { ids: remaining }], ..Default::default() })
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

    pub fn handle(payload: &DuplicateLayer, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let document = doc.snapshot;
        match find_layer(&document.layers, &payload.layer_id) {
            Some(layer) => {
                let copy = clone_layer(layer);
                let select_id = layer_node_id(&copy).to_string();
                Ok(Emit {
                    document_mutations: vec![RasterMutation::AddLayer { parent_id: None, index: document.layers.len(), layer: Box::new(copy) }],
                    config_mutations: vec![RasterConfigMutation::SetSelection { ids: vec![select_id] }],
                    ..Default::default()
                })
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️DuplicateLayer

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

    pub fn handle(payload: &PatchLayer, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let json_value = patch_value_json(&payload.value);
        let operations = raster_patch_layer_operations(doc.snapshot, std::slice::from_ref(&payload.layer_id), &payload.field, &json_value);
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::mutations(operations))
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

    pub fn handle(payload: &PatchLayers, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let json_value = patch_value_json(&payload.value);
        let operations = raster_patch_layer_operations(doc.snapshot, &payload.layer_ids, &payload.field, &json_value);
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::mutations(operations))
        }
    }
}
//#endregion 🔖️PatchLayers

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

    pub fn handle(payload: &MoveLayer, doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let document = doc.snapshot;
        if find_layer(&document.layers, &payload.layer_id).is_none() {
            return Ok(Emit::default());
        }
        let parent_id = layer_id_from_tree_row_id(&payload.target_row_id).and_then(|id| find_layer(&document.layers, &id).and_then(|entry| matches!(entry, RasterLayerNode::Group { .. }).then_some(id)));
        let index = if payload.drop_position == "before" {
            0
        } else if let Some(parent) = &parent_id {
            match find_layer(&document.layers, parent) {
                Some(RasterLayerNode::Group { children, .. }) => children.len(),
                _ => 0,
            }
        } else {
            document.layers.len()
        };
        Ok(Emit::mutations(vec![RasterMutation::MoveLayer { layer_id: payload.layer_id.clone(), parent_id, index }]))
    }
}
//#endregion 🔖️MoveLayer
