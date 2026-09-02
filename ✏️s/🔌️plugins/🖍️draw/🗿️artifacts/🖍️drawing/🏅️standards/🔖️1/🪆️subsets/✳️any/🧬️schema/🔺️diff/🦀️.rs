//! 🧬️ Drawing diff schema — sparse field delta over the artifact + its `apply`/`absorb` pure
//! transform (design.md rule 3: `🧬️schema` keeps types + pure transforms; the facet's grammar spec
//! asset moved to `🚪️io/🔺️diff/📝️text/🦀️.rs`, but `apply`/`absorb` are not a byte-boundary
//! codec — they transform already-decoded `DrawingDiff`/`DrawingSnapshot` values — so they stayed here).

use crate::artifacts::drawing::schema::{insert_layer, layer_base_mut, remove_layer_from_tree, update_layer_in_tree, DrawingArtifact};
use crate::artifacts::drawing::{DrawingArtboard, DrawingImageAsset, DrawingLayerNode, DrawingSnapshot, FillStyle, StrokeStyle};
use protocol::MutationDiff;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the drawing artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingDiff {
    #[state(artifact)]
    pub artifact: Option<Box<DrawingArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub layers: Option<DrawingLayersDelta>,
    #[state(artifact)]
    pub assets: Option<DrawingAssetsDelta>,
    #[state(artifact)]
    pub artboard: Option<Option<DrawingArtboard>>,
    #[state(presence)]
    pub selected_ids: Option<DrawingStringList>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(config)]
    pub engagement_input: Option<String>,
    #[state(config)]
    pub camera_x: Option<f64>,
    #[state(config)]
    pub camera_y: Option<f64>,
    #[state(config)]
    pub camera_zoom: Option<f64>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(artifact)]
    pub hovered_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingAssetsDelta {
    pub entries: BTreeMap<String, Option<DrawingImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `layers`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingLayersDelta {
    pub added: Vec<DrawingLayerAddition>,
    pub removed: Vec<String>,
    pub patched: Vec<DrawingLayerPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// ➕️ One inserted layer with its real target location (parent-aware — a bare `Vec<DrawingLayerNode>`
/// can only ever describe a root-level append, which silently dropped nested `create`/`reorder`
/// targets into group children; `create-layer`/`reorder-layer`'s handcrafted diffs need the real
/// address to stay sparse instead of falling back to a whole-snapshot capture).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerAddition {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: DrawingLayerNode,
}

/// 🩹 One patched layer entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerPatchEntry {
    pub id: String,
    pub patch: DrawingLayerPatch,
}

/// 🩹 Sparse layer field patch (JSON blobs for complex nested values).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingLayerPatch {
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub blend_mode: Option<String>,
    pub transform_json: Option<String>,
    pub fill_json: Option<String>,
    pub stroke_json: Option<String>,
    pub boolean_operation: Option<String>,
    pub trace_params_json: Option<String>,
    pub layer_json: Option<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
impl DrawingDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &DrawingArtifact) -> protocol::MutationApplyResult<DrawingArtifact> {
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
                apply_assets_delta(&mut next.assets, assets).map_err(|error| error.under(["assets"]))?;
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
        })
    }
}

/// 🧩 Applies an identified-collection delta to a layer tree (root + nested removes/patches).
pub fn apply_layers_delta(layers: &[DrawingLayerNode], delta: &DrawingLayersDelta) -> protocol::MutationApplyResult<Vec<DrawingLayerNode>> {
    for (index, id) in delta.removed.iter().enumerate() {
        if !contains_layer(layers, id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed layer does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, entry) in delta.patched.iter().enumerate() {
        if !contains_layer(layers, &entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched layer does not exist").at(["patched".to_string(), index.to_string()]));
        }
        if delta.removed.contains(&entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "layer cannot be removed and patched").at(["patched".to_string(), index.to_string()]));
        }
        if delta.patched[..index].iter().any(|prior| prior.id == entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next = layers.to_vec();
    for id in &delta.removed {
        remove_layer_from_tree(&mut next, id);
    }
    for (position, item) in delta.added.iter().enumerate() {
        if contains_layer(&next, crate::artifacts::drawing::schema::layer_id(&item.layer)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added layer identity already exists").at(["added".to_string(), position.to_string()]));
        }
        let container_len = layer_container_len(&next, item.parent_id.as_deref())
            .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "added layer parent group does not exist").at(["added".to_string(), position.to_string(), "parentId".to_string()]))?;
        if item.index > container_len {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("layer insertion index {} exceeds length {container_len}", item.index)).at(["added".to_string(), position.to_string(), "index".to_string()]));
        }
        insert_layer(&mut next, item.parent_id.as_deref(), item.index, item.layer.clone());
    }
    for (index, entry) in delta.patched.iter().enumerate() {
        apply_layer_patch_entry(&mut next, entry).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    if let Some(order) = &delta.reordered {
        if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|layer| crate::artifacts::drawing::schema::layer_id(layer) == id)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "root layer reorder must be a complete unique permutation").at(["reordered"]));
        }
        let mut by_id: BTreeMap<_, _> = next.into_iter().map(|layer| (crate::artifacts::drawing::schema::layer_id(&layer).to_string(), layer)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered root layer does not exist").at(["reordered".to_string(), id.clone()]))?);
        }
        next = ordered;
    }
    validate_unique_layer_ids(&next)?;
    Ok(next)
}

fn contains_layer(layers: &[DrawingLayerNode], id: &str) -> bool {
    layers.iter().any(|layer| crate::artifacts::drawing::schema::layer_id(layer) == id || matches!(layer, DrawingLayerNode::Group(group) if contains_layer(&group.children, id)))
}

fn validate_unique_layer_ids(layers: &[DrawingLayerNode]) -> protocol::MutationApplyResult<()> {
    fn visit<'a>(layers: &'a [DrawingLayerNode], ids: &mut std::collections::BTreeSet<&'a str>) -> bool {
        for layer in layers {
            if !ids.insert(crate::artifacts::drawing::schema::layer_id(layer)) {
                return false;
            }
            if let DrawingLayerNode::Group(group) = layer {
                if !visit(&group.children, ids) {
                    return false;
                }
            }
        }
        true
    }
    if !visit(layers, &mut std::collections::BTreeSet::new()) {
        return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "resulting layer tree contains duplicate identities").at(["identities"]));
    }
    Ok(())
}

fn layer_container_len(layers: &[DrawingLayerNode], parent_id: Option<&str>) -> Option<usize> {
    match parent_id {
        None => Some(layers.len()),
        Some(parent_id) => layers.iter().find_map(|layer| match layer {
            DrawingLayerNode::Group(group) if group.base.id == parent_id => Some(group.children.len()),
            DrawingLayerNode::Group(group) => layer_container_len(&group.children, Some(parent_id)),
            _ => None,
        }),
    }
}

fn apply_layer_patch_entry(layers: &mut Vec<DrawingLayerNode>, entry: &DrawingLayerPatchEntry) -> protocol::MutationApplyResult<()> {
    let mut result = Ok(());
    if !update_layer_in_tree(layers, &entry.id, &mut |layer| {
        result = apply_layer_patch(layer, &entry.patch);
    }) {
        return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched layer does not exist after structural edits").at([&entry.id]));
    }
    result
}

fn apply_layer_patch(layer: &mut DrawingLayerNode, patch: &DrawingLayerPatch) -> protocol::MutationApplyResult<()> {
    if let Some(layer_json) = &patch.layer_json {
        let replacement = dsl::json::from_json_str::<DrawingLayerNode>(layer_json).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-value", format!("layer patch is not valid JSON: {error}")).at(["layerJson"]))?;
        if crate::artifacts::drawing::schema::layer_id(&replacement) != crate::artifacts::drawing::schema::layer_id(layer) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "layer patch cannot change the target identity").at(["layerJson"]));
        }
        *layer = replacement;
        return Ok(());
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
        base.transform = dsl::json::from_json_str(transform_json).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-value", format!("transform is not valid JSON: {error}")).at(["transformJson"]))?;
    }
    if let Some(fill_json) = &patch.fill_json {
        base.attributes.fill = dsl::json::from_json_str::<Option<FillStyle>>(fill_json).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-value", format!("fill is not valid JSON: {error}")).at(["fillJson"]))?;
    }
    if let Some(stroke_json) = &patch.stroke_json {
        base.attributes.stroke = dsl::json::from_json_str::<Option<StrokeStyle>>(stroke_json).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-value", format!("stroke is not valid JSON: {error}")).at(["strokeJson"]))?;
    }
    if let Some(operation) = &patch.boolean_operation {
        let DrawingLayerNode::Boolean(boolean) = layer else {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "boolean operation patch requires a boolean layer").at(["booleanOperation"]));
        };
        boolean.operation = operation.clone();
    }
    if let Some(params_json) = &patch.trace_params_json {
        let DrawingLayerNode::Trace(trace) = layer else {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "trace parameters patch requires a trace layer").at(["traceParamsJson"]));
        };
        trace.params = dsl::json::from_json_str(params_json).map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-value", format!("trace parameters are not valid JSON: {error}")).at(["traceParamsJson"]))?;
    }
    Ok(())
}

/// 🩹 Merges an incoming `DrawingLayerPatch` into an existing one, field by field — `incoming` wins
/// wherever it sets a field, matching `DrawingDiff::absorb`'s own `take!` semantics.
fn merge_layer_patch(dst: &mut DrawingLayerPatch, mut src: DrawingLayerPatch) {
    macro_rules! take {
        ($field:ident) => {
            if src.$field.is_some() {
                dst.$field = src.$field.take();
            }
        };
    }
    take!(visible);
    take!(locked);
    take!(name);
    take!(opacity);
    take!(blend_mode);
    take!(transform_json);
    take!(fill_json);
    take!(stroke_json);
    take!(boolean_operation);
    take!(trace_params_json);
    take!(layer_json);
}

fn apply_assets_delta(assets: &mut BTreeMap<String, DrawingImageAsset>, delta: &DrawingAssetsDelta) -> protocol::MutationApplyResult<()> {
    for (key, value) in &delta.entries {
        if value.is_none() && !assets.contains_key(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed asset does not exist").at([key.as_str()]));
        }
    }
    let mut candidate = assets.clone();
    for (key, value) in &delta.entries {
        match value {
            Some(asset) => {
                candidate.insert(key.clone(), asset.clone());
            }
            None => {
                candidate.remove(key);
            }
        }
    }
    *assets = candidate;
    Ok(())
}

impl MutationDiff<DrawingSnapshot> for DrawingDiff {
    fn apply(&self, snapshot: &DrawingSnapshot) -> protocol::MutationApplyResult<DrawingSnapshot> {
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
                apply_assets_delta(&mut next.assets, assets).map_err(|error| error.under(["assets"]))?;
            }
            if let Some(artboard) = &self.artboard {
                next.artboard = artboard.clone();
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
                // 🐛️ Fixed pre-existing absorb-law breach: a bare `dst.patched.extend(src.patched)`
                // appended a SECOND `DrawingLayerPatchEntry` for a layer already patched by `dst`,
                // which `apply_layers_delta`'s own `duplicate-target` guard then rejects — so
                // `absorb(d1, d2).apply(base)` errored while `d2.apply(d1.apply(base))` (the law's
                // other side) succeeded. Merging a same-id patch's fields into the existing entry
                // (later field wins, matching every other `take!`-style field in this fn) keeps
                // `patched` free of duplicate ids and restores the law.
                for incoming in src.patched {
                    match dst.patched.iter_mut().find(|entry| entry.id == incoming.id) {
                        Some(existing) => merge_layer_patch(&mut existing.patch, incoming.patch),
                        None => dst.patched.push(incoming),
                    }
                }
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
pub fn diff_set_snapshot(snapshot: &DrawingSnapshot) -> DrawingDiff {
    DrawingDiff { artifact: Some(Box::new(DrawingArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

/// 🩹 Layer visibility patch.
pub fn diff_set_layer_visible(layer_id: &str, visible: bool) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { visible: Some(visible), ..Default::default() })
}

/// 🔒️ Layer locked patch.
pub fn diff_set_layer_locked(layer_id: &str, locked: bool) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { locked: Some(locked), ..Default::default() })
}

/// 🏷️ Layer name patch.
pub fn diff_set_layer_name(layer_id: &str, name: &str) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { name: Some(name.to_string()), ..Default::default() })
}

/// 🌫️ Layer opacity patch.
pub fn diff_set_layer_opacity(layer_id: &str, opacity: f64) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { opacity: Some(opacity), ..Default::default() })
}

/// 🖌️ Layer blend-mode patch.
pub fn diff_set_layer_blend_mode(layer_id: &str, blend_mode: &str) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { blend_mode: Some(blend_mode.to_string()), ..Default::default() })
}

/// ↔️ Layer transform patch.
pub fn diff_set_layer_transform(layer_id: &str, transform: &crate::artifacts::drawing::DrawingTransform) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { transform_json: Some(dsl::json::to_json_string(transform)), ..Default::default() })
}

/// 🎨 Layer fill patch.
pub fn diff_set_fill(layer_id: &str, fill: &Option<FillStyle>) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { fill_json: Some(dsl::json::to_json_string(fill)), ..Default::default() })
}

/// ✏️ Layer stroke patch.
pub fn diff_set_stroke(layer_id: &str, stroke: &Option<StrokeStyle>) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { stroke_json: Some(dsl::json::to_json_string(stroke)), ..Default::default() })
}

/// 🔀 Boolean operation patch.
pub fn diff_set_boolean_operation(layer_id: &str, boolean_operation: &str) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { boolean_operation: Some(boolean_operation.to_string()), ..Default::default() })
}

/// 🖼️ Trace params patch.
pub fn diff_set_trace_params(layer_id: &str, params: &crate::artifacts::drawing::DrawingTraceParams) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { trace_params_json: Some(dsl::json::to_json_string(params)), ..Default::default() })
}

/// 🌱️ Layer insertion at a real (parent, index) address — root when `parent_id` is `None`.
pub fn diff_create_layer(parent_id: Option<&str>, index: usize, layer: DrawingLayerNode) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { added: vec![DrawingLayerAddition { parent_id: parent_id.map(str::to_string), index, layer }], ..Default::default() }), ..Default::default() }
}

/// 🔃 Move an existing layer to a new (parent, index) address — remove-then-insert, both sparse.
pub fn diff_reorder_layer(layer_id: &str, parent_id: Option<&str>, index: usize, layer: DrawingLayerNode) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { removed: vec![layer_id.to_string()], added: vec![DrawingLayerAddition { parent_id: parent_id.map(str::to_string), index, layer }], ..Default::default() }), ..Default::default() }
}

/// ➖️ Layer remove.
pub fn diff_remove_layer(layer_id: &str) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { removed: vec![layer_id.to_string()], ..Default::default() }), ..Default::default() }
}

/// 🔃 Root reorder by id list.
pub fn diff_reorder_layers(order: Vec<String>) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { reordered: Some(order), ..Default::default() }), ..Default::default() }
}

fn layer_base_patch(layer_id: &str, patch: DrawingLayerPatch) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { patched: vec![DrawingLayerPatchEntry { id: layer_id.to_string(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🧬️ Whole-snapshot replacement when a sparse delta cannot express a tree edit.
pub fn diff_from_snapshot(snapshot: DrawingSnapshot) -> DrawingDiff {
    diff_set_snapshot(&snapshot)
}

/// 📋 Selected-ids UI delta helper.
pub fn diff_selected_ids(ids: Vec<String>) -> DrawingDiff {
    DrawingDiff { selected_ids: Some(DrawingStringList { values: ids }), ..Default::default() }
}

/// 🗂️ Assets delta helper.
pub fn diff_assets(entries: DrawingAssetsDelta) -> DrawingDiff {
    DrawingDiff { assets: Some(entries), ..Default::default() }
}
//#endregion 🔖️Builders
