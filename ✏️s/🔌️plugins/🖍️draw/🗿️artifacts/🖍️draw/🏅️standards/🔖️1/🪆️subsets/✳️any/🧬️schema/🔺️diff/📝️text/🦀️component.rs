//! 🔺️ Draw artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::draw::schema::diff::{
    DrawAssetsDelta, DrawDiff, DrawLayerAddition, DrawLayerPatch, DrawLayerPatchEntry, DrawLayersDelta, DrawStringList,
};
use crate::artifacts::draw::schema::{insert_layer, layer_base_mut, remove_layer_from_tree, update_layer_in_tree};
use crate::artifacts::draw::schema::DrawArtifact;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot, FillStyle, StrokeStyle};
use protocol::MutationDiff;


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

#[allow(unused_imports)]

//#region 🔖️Apply
impl DrawDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &DrawArtifact) -> DrawArtifact {
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
        if let Some(artboard) = &self.artboard {
            next.artboard = artboard.clone();
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
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

/// 🧩 Applies an identified-collection delta to a layer tree (root + nested removes/patches).
pub fn apply_layers_delta(layers: &[DrawLayerNode], delta: &DrawLayersDelta) -> Vec<DrawLayerNode> {
    let mut next = layers.to_vec();
    for id in &delta.removed {
        remove_layer_from_tree(&mut next, id);
    }
    for item in &delta.added {
        insert_layer(&mut next, item.parent_id.as_deref(), item.index, item.layer.clone());
    }
    for entry in &delta.patched {
        apply_layer_patch_entry(&mut next, entry);
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next
            .into_iter()
            .map(|layer| (crate::artifacts::draw::schema::layer_id(&layer).to_string(), layer))
            .collect();
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

fn apply_layer_patch_entry(layers: &mut Vec<DrawLayerNode>, entry: &DrawLayerPatchEntry) {
    update_layer_in_tree(layers, &entry.id, &mut |layer| {
        apply_layer_patch(layer, &entry.patch);
    });
}

fn apply_layer_patch(layer: &mut DrawLayerNode, patch: &DrawLayerPatch) {
    if let Some(layer_json) = &patch.layer_json {
        if let Ok(replacement) = serde_json::from_str::<DrawLayerNode>(layer_json) {
            *layer = replacement;
            return;
        }
    }
    let base = layer_base_mut(layer);
    if let Some(visible) = patch.visible {
        base.visible = visible;
    }
    if let Some(locked) = patch.locked {
        base.locked = locked;
    }
    if let Some(name) = &patch.name {
        base.name = name.clone();
    }
    if let Some(opacity) = patch.opacity {
        base.opacity = opacity;
    }
    if let Some(blend_mode) = &patch.blend_mode {
        base.blend_mode = blend_mode.clone();
    }
    if let Some(transform_json) = &patch.transform_json {
        if let Ok(transform) = serde_json::from_str(transform_json) {
            base.transform = transform;
        }
    }
    if let Some(fill_json) = &patch.fill_json {
        base.attributes.fill = serde_json::from_str::<Option<FillStyle>>(fill_json).ok().flatten();
    }
    if let Some(stroke_json) = &patch.stroke_json {
        base.attributes.stroke = serde_json::from_str::<Option<StrokeStyle>>(stroke_json).ok().flatten();
    }
    if let Some(operation) = &patch.boolean_operation {
        if let DrawLayerNode::Boolean(boolean) = layer {
            boolean.operation = operation.clone();
        }
    }
    if let Some(params_json) = &patch.trace_params_json {
        if let DrawLayerNode::Trace(trace) = layer {
            if let Ok(params) = serde_json::from_str(params_json) {
                trace.params = params;
            }
        }
    }
}

impl MutationDiff<DrawSnapshot> for DrawDiff {
    fn apply(&self, snapshot: &DrawSnapshot) -> DrawSnapshot {
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
        if let Some(artboard) = &self.artboard {
            next.artboard = artboard.clone();
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
        take!(artboard);
        take!(selected_ids);
        take!(active_utility_id);
        take!(engagement_input);
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
/// 🖼️ Whole-artifact replacement from a snapshot (UI fields defaulted).
pub fn diff_set_snapshot(snapshot: &DrawSnapshot) -> DrawDiff {
    DrawDiff {
        artifact: Some(Box::new(DrawArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

/// 🩹 Layer visibility patch.
pub fn diff_set_layer_visible(layer_id: &str, visible: bool) -> DrawDiff {
    layer_base_patch(layer_id, DrawLayerPatch { visible: Some(visible), ..Default::default() })
}

/// 🔒️ Layer locked patch.
pub fn diff_set_layer_locked(layer_id: &str, locked: bool) -> DrawDiff {
    layer_base_patch(layer_id, DrawLayerPatch { locked: Some(locked), ..Default::default() })
}

/// 🏷️ Layer name patch.
pub fn diff_set_layer_name(layer_id: &str, name: &str) -> DrawDiff {
    layer_base_patch(layer_id, DrawLayerPatch { name: Some(name.to_string()), ..Default::default() })
}

/// 🌫️ Layer opacity patch.
pub fn diff_set_layer_opacity(layer_id: &str, opacity: f64) -> DrawDiff {
    layer_base_patch(layer_id, DrawLayerPatch { opacity: Some(opacity), ..Default::default() })
}

/// 🖌️ Layer blend-mode patch.
pub fn diff_set_layer_blend_mode(layer_id: &str, blend_mode: &str) -> DrawDiff {
    layer_base_patch(layer_id, DrawLayerPatch { blend_mode: Some(blend_mode.to_string()), ..Default::default() })
}

/// ↔️ Layer transform patch.
pub fn diff_set_layer_transform(layer_id: &str, transform: &crate::artifacts::draw::DrawTransform) -> DrawDiff {
    layer_base_patch(
        layer_id,
        DrawLayerPatch {
            transform_json: Some(serde_json::to_string(transform).unwrap_or_default()),
            ..Default::default()
        },
    )
}

/// 🎨 Layer fill patch.
pub fn diff_set_fill(layer_id: &str, fill: &Option<FillStyle>) -> DrawDiff {
    layer_base_patch(
        layer_id,
        DrawLayerPatch {
            fill_json: Some(serde_json::to_string(fill).unwrap_or_else(|_| "null".into())),
            ..Default::default()
        },
    )
}

/// ✏️ Layer stroke patch.
pub fn diff_set_stroke(layer_id: &str, stroke: &Option<StrokeStyle>) -> DrawDiff {
    layer_base_patch(
        layer_id,
        DrawLayerPatch {
            stroke_json: Some(serde_json::to_string(stroke).unwrap_or_else(|_| "null".into())),
            ..Default::default()
        },
    )
}

/// 🔀 Boolean operation patch.
pub fn diff_set_boolean_operation(layer_id: &str, boolean_operation: &str) -> DrawDiff {
    layer_base_patch(
        layer_id,
        DrawLayerPatch {
            boolean_operation: Some(boolean_operation.to_string()),
            ..Default::default()
        },
    )
}

/// 🖼️ Trace params patch.
pub fn diff_set_trace_params(layer_id: &str, params: &crate::artifacts::draw::DrawTraceParams) -> DrawDiff {
    layer_base_patch(
        layer_id,
        DrawLayerPatch {
            trace_params_json: Some(serde_json::to_string(params).unwrap_or_default()),
            ..Default::default()
        },
    )
}

/// 🌱️ Layer insertion at a real (parent, index) address — root when `parent_id` is `None`.
pub fn diff_create_layer(parent_id: Option<&str>, index: usize, layer: DrawLayerNode) -> DrawDiff {
    DrawDiff {
        layers: Some(DrawLayersDelta {
            added: vec![DrawLayerAddition { parent_id: parent_id.map(str::to_string), index, layer }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// 🔃 Move an existing layer to a new (parent, index) address — remove-then-insert, both sparse.
pub fn diff_reorder_layer(layer_id: &str, parent_id: Option<&str>, index: usize, layer: DrawLayerNode) -> DrawDiff {
    DrawDiff {
        layers: Some(DrawLayersDelta {
            removed: vec![layer_id.to_string()],
            added: vec![DrawLayerAddition { parent_id: parent_id.map(str::to_string), index, layer }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// ➖️ Layer remove.
pub fn diff_remove_layer(layer_id: &str) -> DrawDiff {
    DrawDiff {
        layers: Some(DrawLayersDelta {
            removed: vec![layer_id.to_string()],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// 🔃 Root reorder by id list.
pub fn diff_reorder_layers(order: Vec<String>) -> DrawDiff {
    DrawDiff {
        layers: Some(DrawLayersDelta {
            reordered: Some(order),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn layer_base_patch(layer_id: &str, patch: DrawLayerPatch) -> DrawDiff {
    DrawDiff {
        layers: Some(DrawLayersDelta {
            patched: vec![DrawLayerPatchEntry {
                id: layer_id.to_string(),
                patch,
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// 🧬️ Whole-snapshot replacement when a sparse delta cannot express a tree edit.
pub fn diff_from_snapshot(snapshot: DrawSnapshot) -> DrawDiff {
    diff_set_snapshot(&snapshot)
}

/// 📋 Selected-ids UI delta helper.
pub fn diff_selected_ids(ids: Vec<String>) -> DrawDiff {
    DrawDiff {
        selected_ids: Some(DrawStringList { values: ids }),
        ..Default::default()
    }
}

/// 🗂️ Assets delta helper.
pub fn diff_assets(entries: DrawAssetsDelta) -> DrawDiff {
    DrawDiff {
        assets: Some(entries),
        ..Default::default()
    }
}
//#endregion 🔖️Builders
