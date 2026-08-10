//! 🔺️ Raster artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::raster::schema::diff::{
    RasterAssetsDelta, RasterDiff, RasterLayerPatchEntry, RasterLayersDelta, RasterStringList,
};
use crate::artifacts::raster::engine::{layer_node_id, locate_layer};
use crate::artifacts::raster::schema::RasterArtifact;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

#[allow(unused_imports)]

//#region 🔖️Tree
pub fn remove_layer_from_tree(layers: &mut Vec<RasterLayerNode>, target_id: &str) -> Option<RasterLayerNode> {
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

//#region 🔖️Apply
impl RasterDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &RasterArtifact) -> RasterArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.layers {
            next.layers = apply_layers_delta(&next.layers, delta);
        }
        if let Some(assets) = &self.assets {
            for (key, value) in &assets.entries {
                match value {
                    Some(asset) => {
                        next.assets.insert(key.clone(), asset.clone());
                    }
                    None => {
                        next.assets.remove(key);
                    }
                }
            }
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = self.brush_size {
            next.brush_size = value;
        }
        if let Some(value) = self.brush_opacity {
            next.brush_opacity = value;
        }
        if let Some(value) = &self.composite_viewport {
            next.composite_viewport = value.clone();
        }
        if let Some(value) = self.camera_x {
            next.camera_x = value;
        }
        if let Some(value) = self.camera_y {
            next.camera_y = value;
        }
        if let Some(value) = self.camera_zoom {
            next.camera_zoom = value;
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.hovered_id {
            next.hovered_id = value.clone();
        }
        next
    }
}

pub fn apply_layers_delta(layers: &[RasterLayerNode], delta: &RasterLayersDelta) -> Vec<RasterLayerNode> {
    let mut next = layers.to_vec();
    for id in &delta.removed {
        remove_layer_from_tree(&mut next, id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        apply_layer_patch_entry(&mut next, entry);
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next.into_iter().map(|layer| (layer_node_id(&layer).to_string(), layer)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(layer) = by_id.remove(id) {
                ordered.push(layer);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn apply_layer_patch_entry(layers: &mut Vec<RasterLayerNode>, entry: &RasterLayerPatchEntry) {
    patch_layer_in_tree(layers, &entry.id, &entry.patch);
}

impl MutationDiff<RasterSnapshot> for RasterDiff {
    fn apply(&self, snapshot: &RasterSnapshot) -> RasterSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.layers {
            next.layers = apply_layers_delta(&next.layers, delta);
        }
        if let Some(assets) = &self.assets {
            for (key, value) in &assets.entries {
                match value {
                    Some(asset) => {
                        next.assets.insert(key.clone(), asset.clone());
                    }
                    None => {
                        next.assets.remove(key);
                    }
                }
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(title);
        take!(selected_ids);
        take!(active_utility_id);
        take!(brush_size);
        take!(brush_opacity);
        take!(composite_viewport);
        take!(camera_x);
        take!(camera_y);
        take!(camera_zoom);
        take!(locale);
        take!(hovered_id);
        match (&mut self.layers, other.layers) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.layers = Some(src),
            _ => {}
        }
        match (&mut self.assets, other.assets) {
            (Some(dst), Some(src)) => {
                dst.entries.extend(src.entries);
            }
            (None, Some(src)) => self.assets = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
pub fn diff_set_snapshot(snapshot: &RasterSnapshot) -> RasterDiff {
    RasterDiff {
        artifact: Some(Box::new(RasterArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

pub fn diff_from_snapshot(snapshot: RasterSnapshot) -> RasterDiff {
    diff_set_snapshot(&snapshot)
}

pub fn diff_add_layer(layer: RasterLayerNode) -> RasterDiff {
    RasterDiff {
        layers: Some(RasterLayersDelta {
            added: vec![layer],
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn diff_remove_layer(layer_id: &str) -> RasterDiff {
    RasterDiff {
        layers: Some(RasterLayersDelta {
            removed: vec![layer_id.to_string()],
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn diff_patch_layer(layer_id: &str, patch: RasterLayerPatch) -> RasterDiff {
    RasterDiff {
        layers: Some(RasterLayersDelta {
            patched: vec![RasterLayerPatchEntry {
                id: layer_id.to_string(),
                patch,
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn diff_move_layer(snapshot: &RasterSnapshot, layer_id: &str, parent_id: Option<String>, index: usize) -> RasterDiff {
    let mut probe = snapshot.clone();
    if let Some(node) = remove_layer_from_tree(&mut probe.layers, layer_id) {
        insert_layer(&mut probe.layers, parent_id.as_deref(), index, node);
        diff_from_snapshot(probe)
    } else {
        RasterDiff::default()
    }
}
//#endregion 🔖️Builders
