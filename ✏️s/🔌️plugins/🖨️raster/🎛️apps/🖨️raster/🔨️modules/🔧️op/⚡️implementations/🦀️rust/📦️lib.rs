//! ⚡️ Raster app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use raster::{RasterCamera, RasterLayerNode, RasterLayerPatch, RasterProjection};
use raster_engine::{find_layer, layer_node_id, locate_layer, RasterConfig, RasterConfigViewportSize};
use serde::{Deserialize, Serialize};

//#region 🔖️Tree
fn remove_layer_from_tree(layers: &mut Vec<RasterLayerNode>, target_id: &str) -> Option<RasterLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_node_id(layer) == target_id) {
        return Some(layers.remove(index));
    }
    for layer in layers.iter_mut() {
        if let RasterLayerNode::Group { children, .. } = layer {
            if let Some(removed) = remove_layer_from_tree(children, target_id) {
                return Some(removed);
            }
        }
    }
    None
}

fn insert_layer(layers: &mut Vec<RasterLayerNode>, parent_id: Option<&str>, index: usize, layer: RasterLayerNode) {
    match parent_id {
        None => {
            let at = index.min(layers.len());
            layers.insert(at, layer);
        }
        Some(parent_id) => {
            for node in layers.iter_mut() {
                if let RasterLayerNode::Group { id, children, .. } = node {
                    if id == parent_id {
                        let at = index.min(children.len());
                        children.insert(at, layer);
                        return;
                    }
                    insert_layer(children, Some(parent_id), index, layer.clone());
                }
            }
        }
    }
}

fn apply_layer_patch(node: &mut RasterLayerNode, patch: &RasterLayerPatch) -> RasterLayerPatch {
    let mut inverse = RasterLayerPatch::default();
    match node {
        RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, width, height, .. } => {
            if let Some(value) = &patch.name {
                inverse.name = Some(name.clone());
                *name = value.clone();
            }
            if let Some(value) = patch.visible {
                inverse.visible = Some(*visible);
                *visible = value;
            }
            if let Some(value) = patch.opacity {
                inverse.opacity = Some(*opacity);
                *opacity = value;
            }
            if let Some(value) = &patch.blend_mode {
                inverse.blend_mode = Some(blend_mode.clone());
                *blend_mode = value.clone();
            }
            if let Some(value) = patch.transform_x {
                inverse.transform_x = Some(transform.x);
                transform.x = value;
            }
            if let Some(value) = patch.transform_y {
                inverse.transform_y = Some(transform.y);
                transform.y = value;
            }
            if let Some(value) = patch.width {
                inverse.width = Some(width.unwrap_or(512));
                *width = Some(value);
            }
            if let Some(value) = patch.height {
                inverse.height = Some(height.unwrap_or(512));
                *height = Some(value);
            }
        }
        RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, .. } => {
            if let Some(value) = &patch.name {
                inverse.name = Some(name.clone());
                *name = value.clone();
            }
            if let Some(value) = patch.visible {
                inverse.visible = Some(*visible);
                *visible = value;
            }
            if let Some(value) = patch.opacity {
                inverse.opacity = Some(*opacity);
                *opacity = value;
            }
            if let Some(value) = &patch.blend_mode {
                inverse.blend_mode = Some(blend_mode.clone());
                *blend_mode = value.clone();
            }
            if let Some(value) = patch.transform_x {
                inverse.transform_x = Some(transform.x);
                transform.x = value;
            }
            if let Some(value) = patch.transform_y {
                inverse.transform_y = Some(transform.y);
                transform.y = value;
            }
        }
        RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, adjustment_kind, .. } => {
            if let Some(value) = &patch.name {
                inverse.name = Some(name.clone());
                *name = value.clone();
            }
            if let Some(value) = patch.visible {
                inverse.visible = Some(*visible);
                *visible = value;
            }
            if let Some(value) = patch.opacity {
                inverse.opacity = Some(*opacity);
                *opacity = value;
            }
            if let Some(value) = &patch.blend_mode {
                inverse.blend_mode = Some(blend_mode.clone());
                *blend_mode = value.clone();
            }
            if let Some(value) = &patch.adjustment_kind {
                inverse.adjustment_kind = Some(adjustment_kind.clone());
                *adjustment_kind = value.clone();
            }
        }
    }
    inverse
}

fn patch_layer_in_tree(layers: &mut [RasterLayerNode], target_id: &str, patch: &RasterLayerPatch) -> Option<RasterLayerPatch> {
    for layer in layers.iter_mut() {
        if layer_node_id(layer) == target_id {
            return Some(apply_layer_patch(layer, patch));
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            if let Some(inverse) = patch_layer_in_tree(children, target_id, patch) {
                return Some(inverse);
            }
        }
    }
    None
}
//#endregion 🔖️Tree

//#region 🔖️Types
/// 🧩️ One atomic tree mutation — the building block of {@link RasterDiff}, kept ordered so a diff can
/// coalesce several edits (e.g. a multi-layer patch) while still inverting each mechanically.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "step", rename_all = "camelCase")]
pub enum RasterStep {
    AddLayer { parent_id: Option<String>, index: usize, layer: RasterLayerNode },
    RemoveLayer { layer_id: String },
    PatchLayer { layer_id: String, patch: RasterLayerPatch },
    MoveLayer { layer_id: String, parent_id: Option<String>, index: usize },
}

fn apply_step(layers: &mut Vec<RasterLayerNode>, step: &RasterStep) {
    match step {
        RasterStep::AddLayer { parent_id, index, layer } => insert_layer(layers, parent_id.as_deref(), *index, layer.clone()),
        RasterStep::RemoveLayer { layer_id } => {
            remove_layer_from_tree(layers, layer_id);
        }
        RasterStep::PatchLayer { layer_id, patch } => {
            patch_layer_in_tree(layers, layer_id, patch);
        }
        RasterStep::MoveLayer { layer_id, parent_id, index } => {
            if let Some(node) = remove_layer_from_tree(layers, layer_id) {
                insert_layer(layers, parent_id.as_deref(), *index, node);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RasterOperation {
    AddLayer {
        parent_id: Option<String>,
        index: usize,
        #[dsl(statements)]
        layer: Box<RasterLayerNode>,
    },
    RemoveLayer {
        #[dsl(key = "id")]
        layer_id: String,
    },
    PatchLayer {
        #[dsl(key = "id")]
        layer_id: String,
        #[dsl(block)]
        patch: RasterLayerPatch,
    },
    MoveLayer {
        #[dsl(key = "id")]
        layer_id: String,
        #[dsl(key = "parent")]
        parent_id: Option<String>,
        index: usize,
    },
    ReplaceDocument {
        #[dsl(block)]
        document: RasterProjection,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterDiff {
    pub steps: Vec<RasterStep>,
    pub replace: Option<Box<RasterProjection>>,
}

impl OperationDiff<RasterProjection> for RasterDiff {
    fn apply(&self, projection: &RasterProjection) -> RasterProjection {
        let mut next = self.replace.as_ref().map(|document| (**document).clone()).unwrap_or_else(|| projection.clone());
        for step in &self.steps {
            apply_step(&mut next.layers, step);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if let Some(replace) = other.replace {
            self.replace = Some(replace);
            self.steps.clear();
        }
        self.steps.extend(other.steps);
    }
}

fn step_diff(step: RasterStep) -> RasterDiff {
    RasterDiff { steps: vec![step], ..Default::default() }
}

impl Operation<RasterProjection> for RasterOperation {
    type Diff = RasterDiff;

    fn diff(&self, _projection: &RasterProjection) -> RasterDiff {
        match self {
            RasterOperation::AddLayer { parent_id, index, layer } => step_diff(RasterStep::AddLayer { parent_id: parent_id.clone(), index: *index, layer: (**layer).clone() }),
            RasterOperation::RemoveLayer { layer_id } => step_diff(RasterStep::RemoveLayer { layer_id: layer_id.clone() }),
            RasterOperation::PatchLayer { layer_id, patch } => step_diff(RasterStep::PatchLayer { layer_id: layer_id.clone(), patch: patch.clone() }),
            RasterOperation::MoveLayer { layer_id, parent_id, index } => step_diff(RasterStep::MoveLayer { layer_id: layer_id.clone(), parent_id: parent_id.clone(), index: *index }),
            RasterOperation::ReplaceDocument { document } => RasterDiff { replace: Some(Box::new(document.clone())), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &RasterProjection) -> Vec<Self> {
        match self {
            RasterOperation::AddLayer { layer, .. } => vec![RasterOperation::RemoveLayer { layer_id: layer_node_id(layer).to_string() }],
            RasterOperation::RemoveLayer { layer_id } => match (locate_layer(&projection.layers, layer_id), find_layer(&projection.layers, layer_id)) {
                (Some((parent_id, index)), Some(layer)) => vec![RasterOperation::AddLayer { parent_id, index, layer: Box::new(layer.clone()) }],
                _ => Vec::new(),
            },
            RasterOperation::PatchLayer { layer_id, patch } => {
                let mut probe = projection.layers.clone();
                match patch_layer_in_tree(&mut probe, layer_id, patch) {
                    Some(inverse) => vec![RasterOperation::PatchLayer { layer_id: layer_id.clone(), patch: inverse }],
                    None => Vec::new(),
                }
            }
            RasterOperation::MoveLayer { layer_id, .. } => match locate_layer(&projection.layers, layer_id) {
                Some((parent_id, index)) => vec![RasterOperation::MoveLayer { layer_id: layer_id.clone(), parent_id, index }],
                None => Vec::new(),
            },
            RasterOperation::ReplaceDocument { .. } => vec![RasterOperation::ReplaceDocument { document: projection.clone() }],
        }
    }
}

pub type RasterEnvelope = store::DocumentEnvelope<RasterProjection, RasterOperation>;
pub type RasterStore = store::DocumentStore<RasterProjection, RasterOperation>;
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `raster_engine::RasterConfig`'s operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `RasterPlayRuntime` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns — mirrors `shooting_op::ShootingConfigOperation`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum RasterConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RasterConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "hover")]
    SetHovered { id: Option<String> },
    #[dsl(key = "brush-size")]
    SetBrushSize { value: f64 },
    #[dsl(key = "brush-opacity")]
    SetBrushOpacity { value: f64 },
    #[dsl(key = "composite-viewport")]
    SetCompositeViewport {
        #[dsl(block)]
        viewport: Option<RasterConfigViewportSize>,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: RasterCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<RasterConfig> for RasterConfigOperation {
    type Diff = RasterConfig;

    fn diff(&self, base: &RasterConfig) -> RasterConfig {
        let mut next = base.clone();
        match self {
            RasterConfigOperation::Snapshot { config } => return config.clone(),
            RasterConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            RasterConfigOperation::SetHovered { id } => next.hovered_id = id.clone(),
            RasterConfigOperation::SetBrushSize { value } => next.brush_size = *value,
            RasterConfigOperation::SetBrushOpacity { value } => next.brush_opacity = value.clamp(0.0, 1.0),
            RasterConfigOperation::SetCompositeViewport { viewport } => next.composite_viewport = viewport.clone(),
            RasterConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            RasterConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            RasterConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &RasterConfig) -> Vec<Self> {
        vec![RasterConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use raster::{RasterImageAsset, RasterLayerMask, RasterTransform, RASTER_DOCUMENT_SCHEMA};
    use raster_engine::empty_raster_projection;
    use std::collections::BTreeMap;
    use store::{create_document_envelope, DocumentCommand};
    use vcs::apply_operation;

    fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
        RasterLayerNode::Pixel { id: id.into(), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(512), height: Some(512), image_key: None }
    }

    fn round_trip(projection: &RasterProjection, operation: &RasterOperation) -> RasterProjection {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation projection");
        forward
    }

    #[test]
    fn add_remove_patch_layer_round_trip() {
        let projection = empty_raster_projection();
        let added = round_trip(&projection, &RasterOperation::AddLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) });
        assert_eq!(added.layers.len(), 1);
        let patched = round_trip(&added, &RasterOperation::PatchLayer { layer_id: "l1".into(), patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() } });
        assert_eq!(raster_engine::layer_name(&patched.layers[0]), "Renamed");
        assert!(!raster_engine::layer_visible(&patched.layers[0]));
        let removed = round_trip(&patched, &RasterOperation::RemoveLayer { layer_id: "l1".into() });
        assert!(removed.layers.is_empty());
    }

    #[test]
    fn move_layer_into_group_round_trip() {
        let mut projection = empty_raster_projection();
        projection.layers.push(RasterLayerNode::Group { id: "g1".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() });
        projection.layers.push(pixel_layer("l1", "Base"));
        let moved = round_trip(&projection, &RasterOperation::MoveLayer { layer_id: "l1".into(), parent_id: Some("g1".into()), index: 0 });
        let RasterLayerNode::Group { children, .. } = &moved.layers[0] else { panic!("expected group") };
        assert_eq!(children.len(), 1);
        assert_eq!(layer_node_id(&children[0]), "l1");
    }

    #[test]
    fn replace_document_round_trip() {
        let projection = empty_raster_projection();
        let mut replacement = empty_raster_projection();
        replacement.layers.push(pixel_layer("l9", "Replaced"));
        let replaced = round_trip(&projection, &RasterOperation::ReplaceDocument { document: replacement.clone() });
        assert_eq!(replaced, replacement);
    }

    #[test]
    fn store_applies_layer_add() {
        let mut store = RasterStore::new(create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster", empty_raster_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![RasterOperation::AddLayer { parent_id: None, index: 0, layer: Box::new(pixel_layer("l1", "Base")) }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").layers.len(), 1);
    }

    //#region 🔖️OpText
    /// 📄️ Handcrafted document exercising every layer kind/field — this crate's own private copy (crate
    /// boundaries prevent reuse of the `dsl`/`pack` crates' own `#[cfg(test)]`-only copies).
    fn representative_raster_document() -> RasterProjection {
        let mut assets = BTreeMap::new();
        assets.insert("asset-1".into(), RasterImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into() });
        let mut params = BTreeMap::new();
        params.insert("brightness".into(), dsl::to_dsl_value(&serde_json::json!(0.06)).expect("dsl value"));
        params.insert("label".into(), dsl::to_dsl_value(&serde_json::json!("Warm \"Curve\"")).expect("dsl value"));
        params.insert("enabled".into(), dsl::to_dsl_value(&serde_json::json!(true)).expect("dsl value"));
        params.insert("fallback".into(), dsl::DslValue::Null);
        params.insert("curves".into(), dsl::to_dsl_value(&serde_json::json!([[0.0, 0.0], [0.25, 0.2], [1.0, 1.0]])).expect("dsl value"));
        params.insert("nested".into(), dsl::to_dsl_value(&serde_json::json!({ "inner": 1.5 })).expect("dsl value"));
        RasterProjection {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: "doc-1".into(),
            title: Some("Representative \"Doc\"".into()),
            assets,
            layers: vec![
                RasterLayerNode::Pixel {
                    id: "pixel-1".into(),
                    name: "Pixel One".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                    width: Some(256),
                    height: Some(256),
                    image_key: Some("asset-1".into()),
                },
                RasterLayerNode::Group {
                    id: "group-1".into(),
                    name: "Group / Nested".into(),
                    visible: false,
                    opacity: 0.5,
                    blend_mode: "screen".into(),
                    transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                    mask: None,
                    children: vec![
                        RasterLayerNode::Pixel {
                            id: "pixel-2".into(),
                            name: "Child Pixel".into(),
                            visible: true,
                            opacity: 0.75,
                            blend_mode: "multiply".into(),
                            transform: RasterTransform::default(),
                            mask: None,
                            width: None,
                            height: None,
                            image_key: None,
                        },
                        RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                    ],
                },
                RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
            ],
        }
    }

    #[test]
    fn raster_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&RasterOperation::AddLayer {
            parent_id: None,
            index: 0,
            layer: Box::new(RasterLayerNode::Pixel {
                id: "l1".into(),
                name: "Base".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(512),
                height: Some(512),
                image_key: None,
            }),
        });
        store::test_support::assert_op_line_round_trip(&RasterOperation::AddLayer {
            parent_id: Some("group-1".into()),
            index: 3,
            layer: Box::new(RasterLayerNode::Group {
                id: "g2".into(),
                name: "Nested".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: true, invert: false, width: Some(10), height: Some(20) }),
                children: vec![],
            }),
        });
        store::test_support::assert_op_line_round_trip(&RasterOperation::RemoveLayer { layer_id: "l1".into() });
        store::test_support::assert_op_line_round_trip(&RasterOperation::PatchLayer { layer_id: "l1".into(), patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() } });
        store::test_support::assert_op_line_round_trip(&RasterOperation::PatchLayer { layer_id: "adjust-1".into(), patch: RasterLayerPatch::default() });
        store::test_support::assert_op_line_round_trip(&RasterOperation::MoveLayer { layer_id: "l1".into(), parent_id: Some("g2".into()), index: 1 });
        store::test_support::assert_op_line_round_trip(&RasterOperation::MoveLayer { layer_id: "l1".into(), parent_id: None, index: 0 });
        store::test_support::assert_op_line_round_trip(&RasterOperation::ReplaceDocument { document: representative_raster_document() });
    }
    //#endregion 🔖️OpText

    #[test]
    fn raster_config_operation_round_trips_and_backwards_restores_snapshot() {
        let base = RasterConfig { selected_ids: vec!["a".into()], ..Default::default() };
        let operation = RasterConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_ids, vec!["a".to_string(), "b".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![RasterConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&forward), base);
    }

    #[test]
    fn raster_config_operation_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::Snapshot { config: RasterConfig::default() });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetHovered { id: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetHovered { id: None });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetBrushSize { value: 40.0 });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetBrushOpacity { value: 0.5 });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetCompositeViewport { viewport: Some(RasterConfigViewportSize { width: 640.0, height: 480.0 }) });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetCompositeViewport { viewport: None });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetCamera { camera: RasterCamera { x: 1.0, y: -2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetActiveUtility { utility_id: "paintBrush".into() });
        store::test_support::assert_op_line_round_trip(&RasterConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
