//! 🔺️ Raster artifact — diff surface + laws (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterProjection};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Tree
/// 🌳️ Tree mutation helpers shared by {@link apply_step} (diff application) and
/// `crate::artifacts::raster::op::RasterOperation::backwards` (which needs {@link patch_layer_in_tree}
/// to compute a `PatchLayer` inverse from the pre-operation projection) — `pub`, not `pub(crate)`, so the
/// sibling `🔧️op` node can reach them via `crate::artifacts::raster::diff::…`.
pub fn remove_layer_from_tree(layers: &mut Vec<RasterLayerNode>, target_id: &str) -> Option<RasterLayerNode> {
    if let Some(index) = layers.iter().position(|layer| crate::artifacts::raster::engine::layer_node_id(layer) == target_id) {
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

pub fn insert_layer(layers: &mut Vec<RasterLayerNode>, parent_id: Option<&str>, index: usize, layer: RasterLayerNode) {
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

pub fn patch_layer_in_tree(layers: &mut [RasterLayerNode], target_id: &str, patch: &RasterLayerPatch) -> Option<RasterLayerPatch> {
    for layer in layers.iter_mut() {
        if crate::artifacts::raster::engine::layer_node_id(layer) == target_id {
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

pub fn apply_step(layers: &mut Vec<RasterLayerNode>, step: &RasterStep) {
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterDiff {
    pub steps: Vec<RasterStep>,
    pub replace: Option<Box<RasterProjection>>,
}

impl OperationDiff<RasterProjection> for RasterDiff {
    fn apply(&self, projection: &RasterProjection) -> RasterProjection {
        let mut next = self.replace.as_ref().map_or_else(|| projection.clone(), |document| (**document).clone());
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

pub fn step_diff(step: RasterStep) -> RasterDiff {
    RasterDiff { steps: vec![step], ..Default::default() }
}
//#endregion 🔖️Types
