//! 🔺️ Raster artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::raster::schema::diff::{RasterAssetsDelta, RasterDiff, RasterLayerInsertion, RasterLayerMove, RasterLayerPatchEntry, RasterLayersDelta};
use crate::artifacts::raster::schema::RasterArtifact;
use crate::artifacts::raster::schema::{find_layer, layer_node_id};
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
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

pub fn insert_layer(layers: &mut Vec<RasterLayerNode>, parent_id: Option<&str>, index: usize, layer: RasterLayerNode) -> bool {
    match parent_id {
        None => {
            if index > layers.len() {
                return false;
            }
            layers.insert(index, layer);
            true
        }
        Some(parent_id) => {
            for node in layers.iter_mut() {
                if let RasterLayerNode::Group { id, children, .. } = node {
                    if id == parent_id {
                        if index > children.len() {
                            return false;
                        }
                        children.insert(index, layer);
                        return true;
                    }
                    if insert_layer(children, Some(parent_id), index, layer.clone()) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

fn contains_layer(node: &RasterLayerNode, target_id: &str) -> bool {
    layer_node_id(node) == target_id || matches!(node, RasterLayerNode::Group { children, .. } if children.iter().any(|child| contains_layer(child, target_id)))
}

fn validate_layer_patch(node: &RasterLayerNode, patch: &RasterLayerPatch) -> protocol::MutationApplyResult<()> {
    let invalid = match node {
        RasterLayerNode::Pixel { .. } => patch.adjustment_kind.is_some(),
        RasterLayerNode::Group { .. } => patch.width.is_some() || patch.height.is_some() || patch.adjustment_kind.is_some(),
        RasterLayerNode::Adjustment { .. } => patch.transform_x.is_some() || patch.transform_y.is_some() || patch.width.is_some() || patch.height.is_some(),
    };
    if invalid {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "layer patch contains fields unsupported by the target layer kind"));
    }
    Ok(())
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
    pub fn apply_to_artifact(&self, artifact: &RasterArtifact) -> protocol::MutationApplyResult<RasterArtifact> {
        if !artifact.assets.is_empty() {
            return Err(protocol::MutationApplyError::new("mutation.apply.retained-owner-required", "populated Raster maps require the retained initialization authority"));
        }
        if self.assets.as_ref().is_some_and(|assets| assets.entries.values().any(Option::is_none)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.retained-owner-required", "asset removal requires the retained Raster initialization authority"));
        }
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
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
                next.layers = apply_layers_delta(&next.layers, delta).map_err(|error| error.under(["layers"]))?;
            }
            if let Some(assets) = &self.assets {
                validate_assets_delta(&next.assets, assets).map_err(|error| error.under(["assets"]))?;
                for (key, value) in &assets.entries {
                    match value {
                        Some(asset) => {
                            next.assets.insert(key.clone(), crate::artifacts::raster::mint_raster_asset_child(key, asset));
                        }
                        None => unreachable!("Raster asset removal was rejected before snapshot ownership was cloned"),
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
        })
    }
}

pub fn apply_layers_delta(layers: &[RasterLayerNode], delta: &RasterLayersDelta) -> protocol::MutationApplyResult<Vec<RasterLayerNode>> {
    let mut removed = std::collections::BTreeSet::new();
    for (index, id) in delta.removed.iter().enumerate() {
        if !removed.insert(id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
        if find_layer(layers, id).is_none() {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed layer does not exist").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut patched = std::collections::BTreeSet::new();
    for (index, entry) in delta.patched.iter().enumerate() {
        if !patched.insert(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "layer cannot be removed and patched").at(["patched".to_string(), index.to_string()]));
        }
        let node = find_layer(layers, &entry.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched layer does not exist").at(["patched".to_string(), index.to_string()]))?;
        validate_layer_patch(node, &entry.patch).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let mut moved = std::collections::BTreeSet::new();
    for (index, entry) in delta.moved.iter().enumerate() {
        if !moved.insert(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is moved more than once").at(["moved".to_string(), index.to_string()]));
        }
        if removed.contains(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "layer cannot be removed and moved").at(["moved".to_string(), index.to_string()]));
        }
        let node = find_layer(layers, &entry.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "moved layer does not exist").at(["moved".to_string(), index.to_string()]))?;
        if entry.parent_id.as_deref().is_some_and(|parent_id| contains_layer(node, parent_id)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "layer cannot be moved beneath itself").at(["moved".to_string(), index.to_string(), "parentId".to_string()]));
        }
    }
    let mut identities: std::collections::BTreeSet<String> = crate::artifacts::raster::schema::flatten_raster_layers(layers).into_iter().map(|node| layer_node_id(node).to_string()).collect();
    for id in &delta.removed {
        identities.remove(id);
    }
    for (index, insertion) in delta.added.iter().enumerate() {
        let id = layer_node_id(&insertion.layer);
        if !identities.insert(id.to_string()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added layer identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut next = layers.to_vec();
    for id in &delta.removed {
        remove_layer_from_tree(&mut next, id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "removed layer does not exist after structural edits").at(["removed", id.as_str()]))?;
    }
    for (index, entry) in delta.patched.iter().enumerate() {
        apply_layer_patch_entry(&mut next, entry).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    for (index, mv) in delta.moved.iter().enumerate() {
        let node = remove_layer_from_tree(&mut next, &mv.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "moved layer does not exist after structural edits").at(["moved".to_string(), index.to_string()]))?;
        if !insert_layer(&mut next, mv.parent_id.as_deref(), mv.index, node) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "moved layer parent or index is invalid").at(["moved".to_string(), index.to_string()]));
        }
    }
    for (index, insertion) in delta.added.iter().enumerate() {
        if !insert_layer(&mut next, insertion.parent_id.as_deref(), insertion.index, insertion.layer.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "added layer parent or index is invalid").at(["added".to_string(), index.to_string()]));
        }
    }
    let next_ids: Vec<_> = crate::artifacts::raster::schema::flatten_raster_layers(&next).into_iter().map(layer_node_id).collect();
    if next_ids.iter().enumerate().any(|(index, id)| next_ids[..index].contains(id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "resulting layer tree contains duplicate identities").at(["identities"]));
    }
    Ok(next)
}

fn apply_layer_patch_entry(layers: &mut [RasterLayerNode], entry: &RasterLayerPatchEntry) -> protocol::MutationApplyResult<()> {
    patch_layer_in_tree(layers, &entry.id, &entry.patch).map(|_| ()).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched layer does not exist"))
}

fn validate_assets_delta<T>(assets: &crate::artifacts::raster::RasterOwnedMap<T>, delta: &RasterAssetsDelta) -> protocol::MutationApplyResult<()> {
    for (key, value) in &delta.entries {
        if value.is_none() && !assets.contains_key(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed asset does not exist").at([key.as_str()]));
        }
    }
    Ok(())
}

impl MutationDiff<RasterSnapshot> for RasterDiff {
    fn apply(&self, snapshot: &RasterSnapshot) -> protocol::MutationApplyResult<RasterSnapshot> {
        if !snapshot.assets.is_empty() {
            return Err(protocol::MutationApplyError::new("mutation.apply.retained-owner-required", "populated Raster maps require the retained initialization authority"));
        }
        if self.assets.as_ref().is_some_and(|assets| assets.entries.values().any(Option::is_none)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.retained-owner-required", "asset removal requires the retained Raster initialization authority"));
        }
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
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
                next.layers = apply_layers_delta(&next.layers, delta).map_err(|error| error.under(["layers"]))?;
            }
            if let Some(assets) = &self.assets {
                validate_assets_delta(&next.assets, assets).map_err(|error| error.under(["assets"]))?;
                for (key, value) in &assets.entries {
                    match value {
                        Some(asset) => {
                            next.assets.insert(key.clone(), crate::artifacts::raster::mint_raster_asset_child(key, asset));
                        }
                        None => unreachable!("Raster asset removal was rejected before snapshot ownership was cloned"),
                    }
                }
            }
            next
        })
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
                dst.moved.extend(src.moved);
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
    RasterDiff { artifact: Some(Box::new(RasterArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

pub fn diff_from_snapshot(snapshot: RasterSnapshot) -> RasterDiff {
    diff_set_snapshot(&snapshot)
}

/// ➕ Sparse insertion diff — tree-aware (`parent_id: None` = document root), so `create-layer` never
/// needs to fall back to whole-snapshot capture even when inserting into a nested `Group`.
pub fn diff_add_layer(parent_id: Option<String>, index: usize, layer: RasterLayerNode) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { added: vec![RasterLayerInsertion { parent_id, index, layer }], ..Default::default() }), ..Default::default() }
}

pub fn diff_remove_layer(layer_id: &str) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { removed: vec![layer_id.to_string()], ..Default::default() }), ..Default::default() }
}

pub fn diff_patch_layer(layer_id: &str, patch: RasterLayerPatch) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { patched: vec![RasterLayerPatchEntry { id: layer_id.to_string(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔀 Sparse reposition diff (`reorder-layers`) — remove-then-insert at a tree address, built
/// directly from the payload; never clones/mutates/re-diffs the whole snapshot.
pub fn diff_move_layer(layer_id: &str, parent_id: Option<String>, index: usize) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { moved: vec![RasterLayerMove { id: layer_id.to_string(), parent_id, index }], ..Default::default() }), ..Default::default() }
}

/// 🖇️ Sparse asset-map insertion diff (`add-layer-asset`).
pub fn diff_add_asset(asset_id: &str, asset: crate::artifacts::raster::RasterImageAsset) -> RasterDiff {
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(asset_id.to_string(), Some(asset));
    RasterDiff { assets: Some(RasterAssetsDelta { entries }), ..Default::default() }
}

/// 🗂️ Sparse asset-map removal diff (`remove-layer-asset`).
pub fn diff_remove_asset(asset_id: &str) -> RasterDiff {
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(asset_id.to_string(), None);
    RasterDiff { assets: Some(RasterAssetsDelta { entries }), ..Default::default() }
}
//#endregion 🔖️Builders
