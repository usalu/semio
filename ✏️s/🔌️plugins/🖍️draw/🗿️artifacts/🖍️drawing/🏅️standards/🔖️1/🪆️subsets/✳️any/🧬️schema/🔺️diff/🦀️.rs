//! 🧬️ Drawing diff schema — sparse field delta over the artifact + its `apply`/`absorb` pure
//! transform (design.md rule 3: `🧬️schema` keeps types + pure transforms; the facet's grammar spec
//! asset moved to `🚪️io/📝️text/🔺️diff/🦀️.rs`, but `apply`/`absorb` are not a byte-boundary
//! codec — they transform already-decoded `DrawingDiff`/`DrawingSnapshot` values — so they stayed here).

use crate::schema::{insert_layer, layer_base, layer_base_mut, layer_id, remove_layer_from_tree, update_layer_in_tree};
use crate::{DrawingArtboard, DrawingImageAsset, DrawingLayerNode, DrawingSnapshot, FillStyle, StrokeStyle};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::{list::PagedList, paged::{PagedMap, PagedUtf8, Utf8Text}};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the drawing artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingDiff {
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
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingAssetsDelta {
    pub entries: BTreeMap<String, Option<DrawingImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `layers`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerPatchEntry {
    pub id: String,
    pub patch: DrawingLayerPatch,
}

/// 🩹 Explicit replacement preserving an absent patch and a cleared value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(default)]
pub struct DrawingFillPatch { pub value: Option<FillStyle> }

/// 🩹 Explicit replacement preserving an absent patch and a cleared value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(default)]
pub struct DrawingStrokePatch { pub value: Option<StrokeStyle> }

/// 🩹 Typed layer changes.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingLayerPatch {
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub blend_mode: Option<String>,
    pub fill_rule: Option<crate::FillRule>,
    #[value(skip_serializing_if="Option::is_none")]
    #[cfg_attr(test,serde(skip_serializing_if="Option::is_none"))]
    pub isolation:Option<bool>,
    pub transform: Option<crate::DrawingTransform>,
    pub fill: Option<DrawingFillPatch>,
    pub stroke: Option<DrawingStrokePatch>,
    pub boolean_operation: Option<String>,
    pub trace_params: Option<crate::DrawingTraceParams>,
    pub layer: Option<DrawingLayerNode>,
    pub path_segments: Option<PagedList<crate::PathSegment, {usize::MAX}>>,
    pub text_content: Option<String>,
    pub text_size: Option<f64>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
/// 🧩 Applies an identified-collection delta to a layer tree (root + nested removes/patches).
pub fn apply_layers_delta(layers: &PagedList<DrawingLayerNode, {usize::MAX}>, delta: &DrawingLayersDelta) -> protocol::MutationApplyResult<PagedList<DrawingLayerNode, {usize::MAX}>> {
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
    let mut next = layers.clone();
    for id in &delta.removed {
        remove_layer_from_tree(&mut next, id);
    }
    for (position, item) in delta.added.iter().enumerate() {
        if contains_layer(&next, crate::schema::layer_id(&item.layer)) {
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
        if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|layer| crate::schema::layer_id(layer) == id)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "root layer reorder must be a complete unique permutation").at(["reordered"]));
        }
        let mut by_id = PagedMap::<DrawingLayerNode, {usize::MAX}>::default();
        for layer in next {
            by_id.insert(crate::schema::layer_id(&layer).clone(), layer);
        }
        let mut ordered = PagedList::default();
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered root layer does not exist").at(["reordered".to_string(), id.clone()]))?);
        }
        next = ordered;
    }
    validate_unique_layer_ids(&next)?;
    Ok(next)
}

fn contains_layer(layers: &PagedList<DrawingLayerNode, {usize::MAX}>, id: &(impl Utf8Text + ?Sized)) -> bool {
    layers.iter().any(|layer| crate::schema::layer_id(layer).eq_text(id) || matches!(layer, DrawingLayerNode::Group(group) if contains_layer(&group.children, id)))
}

fn validate_unique_layer_ids(layers: &PagedList<DrawingLayerNode, {usize::MAX}>) -> protocol::MutationApplyResult<()> {
    fn visit<'a>(layers: &'a PagedList<DrawingLayerNode, {usize::MAX}>, ids: &mut std::collections::BTreeSet<&'a PagedUtf8<{usize::MAX}>>) -> bool {
        for layer in layers {
            if !ids.insert(crate::schema::layer_id(layer)) {
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

fn layer_container_len(layers: &PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<&str>) -> Option<usize> {
    match parent_id {
        None => Some(layers.len()),
        Some(parent_id) => layers.iter().find_map(|layer| match layer {
            DrawingLayerNode::Group(group) if group.base.id == parent_id => Some(group.children.len()),
            DrawingLayerNode::Group(group) => layer_container_len(&group.children, Some(parent_id)),
            _ => None,
        }),
    }
}

fn apply_layer_patch_entry(layers: &mut PagedList<DrawingLayerNode, {usize::MAX}>, entry: &DrawingLayerPatchEntry) -> protocol::MutationApplyResult<()> {
    let mut result = Ok(());
    if !update_layer_in_tree(layers, &entry.id, &mut |layer| {
        result = apply_layer_patch(layer, &entry.patch);
    }) {
        return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched layer does not exist after structural edits").at([&entry.id]));
    }
    result
}

fn apply_layer_patch(layer: &mut DrawingLayerNode, patch: &DrawingLayerPatch) -> protocol::MutationApplyResult<()> {
    if let Some(patched) = &patch.layer {
        let replacement = patched.clone();
        if crate::schema::layer_id(&replacement) != crate::schema::layer_id(layer) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "layer patch cannot change the target identity").at(["layer"]));
        }
        *layer = replacement;
        return Ok(());
    }
    if let Some(segments) = &patch.path_segments {
        if !segments.iter().all(crate::schema::valid_path_segment) { return Err(protocol::MutationApplyError::new("mutation.apply.invalid-value", "Invalid path geometry")); }
        let DrawingLayerNode::Path(path) = layer else { return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "Geometry target is not a path")); };
        path.segments = segments.clone();
    }
    if patch.text_content.is_some() || patch.text_size.is_some() {
        let DrawingLayerNode::Text(text) = layer else { return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "Text target has another kind")); };
        if let Some(size) = patch.text_size {
            if !size.is_finite() || size <= 0.0 { return Err(protocol::MutationApplyError::new("mutation.apply.invalid-value", "Invalid text size")); }
            text.size = size;
        }
        if let Some(content) = &patch.text_content { text.content = content.clone().into(); }
    }
    if let Some(isolation)=patch.isolation {
        let DrawingLayerNode::Group(group)=layer else {return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target","Isolation needs a group"));};
        group.isolation=isolation;
    }
    let base = layer_base_mut(layer);
    if let Some(visible) = patch.visible {
        base.visible = visible;
    }
    if let Some(locked) = patch.locked {
        base.locked = locked;
    }
    if let Some(name) = &patch.name {
        base.name = name.clone().into();
    }
    if let Some(opacity) = patch.opacity {
        base.opacity = opacity;
    }
    if let Some(fill_rule) = patch.fill_rule {base.attributes.fill_rule=fill_rule;}
    if let Some(blend_mode) = &patch.blend_mode {
        if !crate::DRAWING_BLEND_MODES.contains(&blend_mode.as_str()) { return Err(protocol::MutationApplyError::new("mutation.apply.invalid-value", "Unsupported blend mode.").at(["blendMode"])); }
        base.blend_mode = blend_mode.clone().into();
    }
    if let Some(transform) = &patch.transform {
        base.transform = transform.clone();
    }
    if let Some(fill) = &patch.fill {
        base.attributes.fill = fill.value.clone();
    }
    if let Some(stroke) = &patch.stroke {
        base.attributes.stroke = stroke.value.clone();
    }
    if let Some(operation) = &patch.boolean_operation {
        let DrawingLayerNode::Boolean(boolean) = layer else {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "boolean operation patch requires a boolean layer").at(["booleanOperation"]));
        };
        boolean.operation = operation.clone().into();
    }
    if let Some(params_json) = &patch.trace_params {
        let DrawingLayerNode::Trace(trace) = layer else {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "trace parameters patch requires a trace layer").at(["traceParams"]));
        };
        trace.params = params_json.clone();
    }
    Ok(())
}

fn apply_assets_delta(assets: &mut PagedMap<DrawingImageAsset, {usize::MAX}>, delta: &DrawingAssetsDelta) -> protocol::MutationApplyResult<()> {
    for (key, value) in &delta.entries {
        if value.is_none() && assets.get(key).is_none() {
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

impl DrawingLayerPatch {
    /// 🕳️ Whether the patch sets no field.
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// 🔁️ The patch that restores exactly the fields `self` sets, read from the BASE layer.
    fn inverse(&self, base: &DrawingLayerNode) -> Self {
        let mut inverse = Self::default();
        if self.layer.is_some() {
            inverse.layer = Some(base.clone());
            return inverse;
        }
        let source = layer_base(base);
        if self.visible.is_some() {
            inverse.visible = Some(source.visible);
        }
        if self.locked.is_some() {
            inverse.locked = Some(source.locked);
        }
        if self.name.is_some() {
            inverse.name = Some(source.name.to_string_owner());
        }
        if self.opacity.is_some() {
            inverse.opacity = Some(source.opacity);
        }
        if self.blend_mode.is_some() {
            inverse.blend_mode = Some(source.blend_mode.to_string_owner());
        }
        if self.fill_rule.is_some() {
            inverse.fill_rule = Some(source.attributes.fill_rule);
        }
        if self.transform.is_some() {
            inverse.transform = Some(source.transform.clone());
        }
        if self.fill.is_some() {
            inverse.fill = Some(DrawingFillPatch { value: source.attributes.fill.clone() });
        }
        if self.stroke.is_some() {
            inverse.stroke = Some(DrawingStrokePatch { value: source.attributes.stroke.clone() });
        }
        match base {
            DrawingLayerNode::Group(group) if self.isolation.is_some() => inverse.isolation = Some(group.isolation),
            DrawingLayerNode::Boolean(boolean) if self.boolean_operation.is_some() => inverse.boolean_operation = Some(boolean.operation.to_string_owner()),
            DrawingLayerNode::Trace(trace) if self.trace_params.is_some() => inverse.trace_params = Some(trace.params.clone()),
            DrawingLayerNode::Path(path) if self.path_segments.is_some() => inverse.path_segments = Some(path.segments.clone()),
            DrawingLayerNode::Text(text) => {
                if self.text_content.is_some() {
                    inverse.text_content = Some(text.content.to_string_owner());
                }
                if self.text_size.is_some() {
                    inverse.text_size = Some(text.size);
                }
            }
            _ => {}
        }
        inverse
    }

    /// ➕️ Composes `self` then `src` on the same layer: a later field wins, a later whole-layer replacement supersedes
    /// everything, and field edits after an earlier replacement are folded into the replacement layer.
    fn absorb(&mut self, mut src: Self) {
        if src.layer.is_some() {
            *self = Self { layer: src.layer.take(), ..Self::default() };
            return;
        }
        if let Some(node) = self.layer.as_mut() {
            let mut folded = node.clone();
            if apply_layer_patch(&mut folded, &src).is_ok() {
                *node = folded;
                return;
            }
        }
        macro_rules! take {
            ($field:ident) => {
                if src.$field.is_some() {
                    self.$field = src.$field.take();
                }
            };
        }
        take!(visible);
        take!(locked);
        take!(name);
        take!(opacity);
        take!(blend_mode);
        take!(fill_rule);
        take!(isolation);
        take!(transform);
        take!(fill);
        take!(stroke);
        take!(boolean_operation);
        take!(trace_params);
        take!(path_segments);
        take!(text_content);
        take!(text_size);
    }
}

type Layers = PagedList<DrawingLayerNode, {usize::MAX}>;

struct Placed<'a> {
    id: String,
    parent: Option<String>,
    index: usize,
    node: &'a DrawingLayerNode,
}

fn flatten<'a>(layers: &'a Layers, parent: Option<&str>, into: &mut Vec<Placed<'a>>) {
    for (index, node) in layers.iter().enumerate() {
        let id = layer_id(node).to_string_owner();
        into.push(Placed { id: id.clone(), parent: parent.map(str::to_owned), index, node });
        if let DrawingLayerNode::Group(group) = node {
            flatten(&group.children, Some(&id), into);
        }
    }
}

fn collect_ids(node: &DrawingLayerNode, into: &mut BTreeSet<String>) {
    into.insert(layer_id(node).to_string_owner());
    if let DrawingLayerNode::Group(group) = node {
        for child in &group.children {
            collect_ids(child, into);
        }
    }
}

fn remove_nested(node: &mut DrawingLayerNode, id: &str) -> Option<BTreeSet<String>> {
    let DrawingLayerNode::Group(group) = node else { return None };
    if let Some(position) = group.children.iter().position(|child| layer_id(child).eq_text(id)) {
        let removed = group.children.remove(position);
        let mut gone = BTreeSet::new();
        collect_ids(&removed, &mut gone);
        return Some(gone);
    }
    group.children.iter_mut().find_map(|child| remove_nested(child, id))
}

fn shallow(node: &DrawingLayerNode) -> DrawingLayerNode {
    let mut copy = node.clone();
    if let DrawingLayerNode::Group(group) = &mut copy {
        group.children = PagedList::default();
    }
    copy
}

enum LayerChange {
    Same,
    Patch(DrawingLayerPatch),
    Replace,
}

fn compare_layers(base: &DrawingLayerNode, other: &DrawingLayerNode) -> LayerChange {
    let (source, target) = (layer_base(base), layer_base(other));
    let mut patch = DrawingLayerPatch::default();
    if source.visible != target.visible {
        patch.visible = Some(target.visible);
    }
    if source.locked != target.locked {
        patch.locked = Some(target.locked);
    }
    if source.name != target.name {
        patch.name = Some(target.name.to_string_owner());
    }
    if source.opacity != target.opacity {
        patch.opacity = Some(target.opacity);
    }
    if source.blend_mode != target.blend_mode {
        patch.blend_mode = Some(target.blend_mode.to_string_owner());
    }
    if source.attributes.fill_rule != target.attributes.fill_rule {
        patch.fill_rule = Some(target.attributes.fill_rule);
    }
    if source.transform != target.transform {
        patch.transform = Some(target.transform.clone());
    }
    if source.attributes.fill != target.attributes.fill {
        patch.fill = Some(DrawingFillPatch { value: target.attributes.fill.clone() });
    }
    if source.attributes.stroke != target.attributes.stroke {
        patch.stroke = Some(DrawingStrokePatch { value: target.attributes.stroke.clone() });
    }
    match (base, other) {
        (DrawingLayerNode::Group(left), DrawingLayerNode::Group(right)) if left.isolation != right.isolation => patch.isolation = Some(right.isolation),
        (DrawingLayerNode::Boolean(left), DrawingLayerNode::Boolean(right)) if left.operation != right.operation => patch.boolean_operation = Some(right.operation.to_string_owner()),
        (DrawingLayerNode::Trace(left), DrawingLayerNode::Trace(right)) if left.params != right.params => patch.trace_params = Some(right.params.clone()),
        (DrawingLayerNode::Path(left), DrawingLayerNode::Path(right)) if !left.segments.iter().eq(right.segments.iter()) => patch.path_segments = Some(right.segments.clone()),
        (DrawingLayerNode::Text(left), DrawingLayerNode::Text(right)) => {
            if left.content != right.content {
                patch.text_content = Some(right.content.to_string_owner());
            }
            if left.size != right.size {
                patch.text_size = Some(right.size);
            }
        }
        _ => {}
    }
    let mut check = shallow(base);
    if apply_layer_patch(&mut check, &patch).is_err() || check != shallow(other) {
        return LayerChange::Replace;
    }
    if patch.is_empty() {
        LayerChange::Same
    } else {
        LayerChange::Patch(patch)
    }
}

impl DrawingLayersDelta {
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.patched.iter().all(|entry| entry.patch.is_empty()) && self.reordered.is_none()
    }

    fn normalize(&mut self) {
        self.removed.sort();
        self.removed.dedup();
        self.patched.sort_by(|left, right| left.id.cmp(&right.id));
    }

    fn drop_subtree(&mut self, gone: &mut BTreeSet<String>) {
        while let Some(nested) = self.added.iter().position(|entry| entry.parent_id.as_ref().is_some_and(|parent| gone.contains(parent))) {
            let entry = self.added.remove(nested);
            collect_ids(&entry.layer, gone);
        }
        self.patched.retain(|entry| !gone.contains(&entry.id));
    }

    fn cancel_added(&mut self, id: &str) -> bool {
        if let Some(at) = self.added.iter().position(|entry| layer_id(&entry.layer).eq_text(id)) {
            let entry = self.added.remove(at);
            for later in self.added[at..].iter_mut().filter(|later| later.parent_id == entry.parent_id && later.index > entry.index) {
                later.index -= 1;
            }
            let mut gone = BTreeSet::new();
            collect_ids(&entry.layer, &mut gone);
            self.drop_subtree(&mut gone);
            return true;
        }
        let Some(mut gone) = self.added.iter_mut().find_map(|entry| remove_nested(&mut entry.layer, id)) else { return false };
        self.drop_subtree(&mut gone);
        true
    }

    /// ➕️ Composes `self` (base→mid) with `other` (mid→after): create∘delete cancels, patch∘delete drops the patch,
    /// delete∘create keeps both, patch∘patch merges per layer, and a later root reorder supersedes an earlier one.
    fn absorb(&mut self, other: Self) {
        for id in other.removed {
            if self.cancel_added(&id) {
                continue;
            }
            self.patched.retain(|entry| entry.id != id);
            if !self.removed.contains(&id) {
                self.removed.push(id);
            }
        }
        self.added.extend(other.added);
        for incoming in other.patched {
            match self.patched.iter_mut().find(|entry| entry.id == incoming.id) {
                Some(existing) => existing.patch.absorb(incoming.patch),
                None => self.patched.push(incoming),
            }
        }
        if other.reordered.is_some() {
            self.reordered = other.reordered;
        }
        self.normalize();
    }

    /// 🔁️ The negative delta read from the BASE layer tree: added roots are removed, removed roots are re-inserted at
    /// their base address in ascending order, patches restore the base fields, a root reorder restores the base order.
    fn inverse(&self, base: &Layers) -> Self {
        let mut placed = Vec::new();
        flatten(base, None, &mut placed);
        let by_id: BTreeMap<&str, &Placed> = placed.iter().map(|entry| (entry.id.as_str(), entry)).collect();
        let mut added_ids = BTreeSet::new();
        for entry in &self.added {
            collect_ids(&entry.layer, &mut added_ids);
        }
        let removed = self.added.iter().filter(|entry| entry.parent_id.as_ref().is_none_or(|parent| !added_ids.contains(parent))).map(|entry| layer_id(&entry.layer).to_string_owner()).collect();
        let covered = |entry: &&Placed| {
            let mut parent = entry.parent.as_deref();
            while let Some(id) = parent {
                if self.removed.iter().any(|removed| removed == id) {
                    return false;
                }
                parent = by_id.get(id).and_then(|ancestor| ancestor.parent.as_deref());
            }
            true
        };
        let mut restored: Vec<&Placed> = self.removed.iter().filter_map(|id| by_id.get(id.as_str()).copied()).filter(covered).collect();
        restored.sort_by(|left, right| (&left.parent, left.index).cmp(&(&right.parent, right.index)));
        let added = restored.into_iter().map(|entry| DrawingLayerAddition { parent_id: entry.parent.clone(), index: entry.index, layer: entry.node.clone() }).collect();
        let patched = self.patched.iter().filter(|entry| !added_ids.contains(&entry.id)).filter_map(|entry| by_id.get(entry.id.as_str()).map(|source| DrawingLayerPatchEntry { id: entry.id.clone(), patch: entry.patch.inverse(source.node) })).collect();
        let reordered = self.reordered.as_ref().map(|_| base.iter().map(|layer| layer_id(layer).to_string_owner()).collect());
        let mut inverse = Self { added, removed, patched, reordered };
        inverse.normalize();
        inverse
    }

    /// 🧭️ The delta from `base` to `other`: absent ids are removed, new ids added, moved or kind-changed layers
    /// removed and re-added, the rest patched field by field; a changed root order is a full root reorder.
    fn between(base: &Layers, other: &Layers) -> Self {
        let (mut base_placed, mut other_placed) = (Vec::new(), Vec::new());
        flatten(base, None, &mut base_placed);
        flatten(other, None, &mut other_placed);
        let base_by: BTreeMap<&str, &Placed> = base_placed.iter().map(|entry| (entry.id.as_str(), entry)).collect();
        let other_by: BTreeMap<&str, &Placed> = other_placed.iter().map(|entry| (entry.id.as_str(), entry)).collect();
        let mut moved = BTreeSet::new();
        let mut changes = BTreeMap::new();
        for entry in &other_placed {
            let Some(source) = base_by.get(entry.id.as_str()) else { continue };
            if source.parent != entry.parent || std::mem::discriminant(source.node) != std::mem::discriminant(entry.node) {
                moved.insert(entry.id.clone());
                continue;
            }
            match compare_layers(source.node, entry.node) {
                LayerChange::Replace if matches!(entry.node, DrawingLayerNode::Group(_)) => {
                    moved.insert(entry.id.clone());
                }
                change => {
                    changes.insert(entry.id.clone(), change);
                }
            }
        }
        let parents: BTreeSet<&Option<String>> = other_placed.iter().map(|entry| &entry.parent).collect();
        let mut reorder_root = false;
        for parent in parents {
            let stable = |entries: &[Placed], counterpart: &BTreeMap<&str, &Placed>| -> Vec<String> {
                entries.iter().filter(|entry| &entry.parent == parent && !moved.contains(&entry.id) && counterpart.get(entry.id.as_str()).is_some_and(|other| &other.parent == parent)).map(|entry| entry.id.clone()).collect()
            };
            let (before, after) = (stable(&base_placed, &other_by), stable(&other_placed, &base_by));
            if before != after {
                match parent {
                    None => reorder_root = true,
                    Some(_) => moved.extend(after),
                }
            }
        }
        let mut delta = Self::default();
        fn walk_removed(list: &Layers, other_by: &BTreeMap<&str, &Placed>, moved: &BTreeSet<String>, into: &mut Vec<String>) {
            for node in list {
                let id = layer_id(node).to_string_owner();
                if !other_by.contains_key(id.as_str()) || moved.contains(&id) {
                    into.push(id);
                } else if let DrawingLayerNode::Group(group) = node {
                    walk_removed(&group.children, other_by, moved, into);
                }
            }
        }
        walk_removed(base, &other_by, &moved, &mut delta.removed);
        fn walk_other(list: &Layers, parent: Option<&str>, base_by: &BTreeMap<&str, &Placed>, moved: &BTreeSet<String>, changes: &mut BTreeMap<String, LayerChange>, into: &mut DrawingLayersDelta) {
            for (index, node) in list.iter().enumerate() {
                let id = layer_id(node).to_string_owner();
                if moved.contains(&id) || !base_by.contains_key(id.as_str()) {
                    into.added.push(DrawingLayerAddition { parent_id: parent.map(str::to_owned), index, layer: node.clone() });
                    continue;
                }
                match changes.remove(&id) {
                    Some(LayerChange::Patch(patch)) => into.patched.push(DrawingLayerPatchEntry { id: id.clone(), patch }),
                    Some(LayerChange::Replace) => into.patched.push(DrawingLayerPatchEntry { id: id.clone(), patch: DrawingLayerPatch { layer: Some(node.clone()), ..Default::default() } }),
                    _ => {}
                }
                if let DrawingLayerNode::Group(group) = node {
                    walk_other(&group.children, Some(&id), base_by, moved, changes, into);
                }
            }
        }
        walk_other(other, None, &base_by, &moved, &mut changes, &mut delta);
        delta.added.sort_by(|left, right| (&left.parent_id, left.index).cmp(&(&right.parent_id, right.index)));
        if reorder_root {
            delta.reordered = Some(other.iter().map(|layer| layer_id(layer).to_string_owner()).collect());
        }
        delta.normalize();
        delta
    }
}

impl DrawingAssetsDelta {
    /// 🔁️ The entries that restore the BASE value of every key this delta touches.
    fn inverse(&self, base: &PagedMap<DrawingImageAsset, {usize::MAX}>) -> Self {
        Self { entries: self.entries.keys().map(|key| (key.clone(), base.get(key).cloned())).collect() }
    }

    /// 🧭️ The entries that turn `base` into `other`.
    fn between(base: &PagedMap<DrawingImageAsset, {usize::MAX}>, other: &PagedMap<DrawingImageAsset, {usize::MAX}>) -> Self {
        let keys: BTreeSet<String> = base.keys().chain(other.keys()).map(|key| key.to_string_owner()).collect();
        Self { entries: keys.into_iter().filter(|key| base.get(key) != other.get(key)).map(|key| (key.clone(), other.get(&key).cloned())).collect() }
    }
}

impl MutationDiff<DrawingSnapshot> for DrawingDiff {
    fn apply(&self, snapshot: &DrawingSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<DrawingSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone().into();
        }
        if let Some(id) = &self.id {
            next.id = id.clone().into();
        }
        if let Some(title) = &self.title {
            next.title = title.clone().map(Into::into);
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
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
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
        match (&mut self.layers, other.layers) {
            (Some(dst), Some(src)) => dst.absorb(src),
            (None, Some(mut src)) => {
                src.normalize();
                self.layers = Some(src);
            }
            _ => {}
        }
        if self.layers.as_ref().is_some_and(DrawingLayersDelta::is_empty) {
            self.layers = None;
        }
        match (&mut self.assets, other.assets) {
            (Some(dst), Some(src)) => dst.entries.extend(src.entries),
            (None, Some(src)) => self.assets = Some(src),
            _ => {}
        }
    }
}

impl protocol::DiffAlgebra<DrawingSnapshot> for DrawingDiff {
    fn inverse(&self, base: &DrawingSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.to_string_owner()),
            id: self.id.as_ref().map(|_| base.id.to_string_owner()),
            title: self.title.as_ref().map(|_| base.title.as_ref().map(|title| title.to_string_owner())),
            layers: self.layers.as_ref().map(|delta| delta.inverse(&base.layers)),
            assets: self.assets.as_ref().map(|delta| delta.inverse(&base.assets)),
            artboard: self.artboard.as_ref().map(|_| base.artboard.clone()),
        }
    }

    fn between(base: &DrawingSnapshot, other: &DrawingSnapshot) -> Self {
        let layers = DrawingLayersDelta::between(&base.layers, &other.layers);
        let assets = DrawingAssetsDelta::between(&base.assets, &other.assets);
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.to_string_owner()),
            id: (base.id != other.id).then(|| other.id.to_string_owner()),
            title: (base.title != other.title).then(|| other.title.as_ref().map(|title| title.to_string_owner())),
            layers: (!layers.is_empty()).then_some(layers),
            assets: (!assets.entries.is_empty()).then_some(assets),
            artboard: (base.artboard != other.artboard).then(|| other.artboard.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.id.is_none() && self.title.is_none() && self.artboard.is_none() && self.layers.as_ref().is_none_or(DrawingLayersDelta::is_empty) && self.assets.as_ref().is_none_or(|assets| assets.entries.is_empty())
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 🩹 Layer visibility patch.
pub fn diff_set_layer_visible(layer_id: &(impl std::fmt::Display + ?Sized), visible: bool) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { visible: Some(visible), ..Default::default() })
}

/// 🔒️ Layer locked patch.
pub fn diff_set_layer_locked(layer_id: &(impl std::fmt::Display + ?Sized), locked: bool) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { locked: Some(locked), ..Default::default() })
}

/// 🏷️ Layer name patch.
pub fn diff_set_layer_name(layer_id: &(impl std::fmt::Display + ?Sized), name: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { name: Some(name.to_string()), ..Default::default() })
}

/// 🌫️ Layer opacity patch.
pub fn diff_set_layer_opacity(layer_id: &(impl std::fmt::Display + ?Sized), opacity: f64) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { opacity: Some(opacity), ..Default::default() })
}

/// 🖌️ Layer blend-mode patch.
pub fn diff_set_layer_blend_mode(layer_id: &(impl std::fmt::Display + ?Sized), blend_mode: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { blend_mode: Some(blend_mode.to_string()), ..Default::default() })
}





/// ✏️ Several paths' geometry patches in one sparse delta, in the given order.
pub fn diff_set_path_geometries<S: IntoIterator<Item = crate::PathSegment>>(entries: impl IntoIterator<Item = (String, S)>) -> DrawingDiff {
    let patched = entries.into_iter().map(|(id, segments)| DrawingLayerPatchEntry { id, patch: DrawingLayerPatch { path_segments: Some(segments.into_iter().collect()), ..Default::default() } }).collect();
    DrawingDiff { layers: Some(DrawingLayersDelta { patched, ..Default::default() }), ..Default::default() }
}





/// 🔀 Boolean operation patch.
pub fn diff_set_boolean_operation(layer_id: &(impl std::fmt::Display + ?Sized), boolean_operation: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { boolean_operation: Some(boolean_operation.to_string()), ..Default::default() })
}



/// 🌱️ Layer insertion at a real (parent, index) address — root when `parent_id` is `None`.
pub fn diff_create_layer(parent_id: Option<&PagedUtf8<{usize::MAX}>>, index: usize, layer: DrawingLayerNode) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { added: vec![DrawingLayerAddition { parent_id: parent_id.map(PagedUtf8::to_string_owner), index, layer }], ..Default::default() }), ..Default::default() }
}

/// 🔃 Move an existing layer to a new (parent, index) address — remove-then-insert, both sparse.
pub fn diff_reorder_layer(layer_id: &(impl std::fmt::Display + ?Sized), parent_id: Option<&PagedUtf8<{usize::MAX}>>, index: usize, layer: DrawingLayerNode) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { removed: vec![layer_id.to_string()], added: vec![DrawingLayerAddition { parent_id: parent_id.map(PagedUtf8::to_string_owner), index, layer }], ..Default::default() }), ..Default::default() }
}

/// ➖️ Layer remove.
pub fn diff_remove_layer(layer_id: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { removed: vec![layer_id.to_string()], ..Default::default() }), ..Default::default() }
}

/// 🔃 Root reorder by id list.
pub fn diff_reorder_layers(order: Vec<String>) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { reordered: Some(order), ..Default::default() }), ..Default::default() }
}



/// 🗂️ Assets delta helper.
pub fn diff_assets(entries: DrawingAssetsDelta) -> DrawingDiff {
    DrawingDiff { assets: Some(entries), ..Default::default() }
}
//#endregion 🔖️Builders

/// ✏️ Replaces only the path geometry facet.
pub fn diff_set_path_geometry(layer_id: &(impl std::fmt::Display + ?Sized), segments: &(impl crate::schema::geometry::editing::PathGeometrySource + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { path_segments: Some(segments.path_segments().cloned().collect()), ..Default::default() })
}

/// 📝️ Replaces the editable text facet without touching its layer base.
pub fn diff_set_text(layer_id: &(impl std::fmt::Display + ?Sized), content: &(impl std::fmt::Display + ?Sized), size: f64) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { text_content: Some(content.to_string()), text_size: Some(size), ..Default::default() })
}

/// 🌀️ Sparse authored fill-rule delta.
pub fn diff_set_layer_fill_rule(layer_id:&(impl std::fmt::Display + ?Sized),fill_rule:crate::FillRule)->DrawingDiff {layer_base_patch(layer_id,DrawingLayerPatch {fill_rule:Some(fill_rule),..Default::default()})}

pub fn diff_set_group_isolation(layer_id:&(impl std::fmt::Display + ?Sized),isolation:bool)->DrawingDiff {layer_base_patch(layer_id,DrawingLayerPatch {isolation:Some(isolation),..Default::default()})}

/// ↔️ Layer transform patch.
pub fn diff_set_layer_transform(layer_id: &(impl std::fmt::Display + ?Sized), transform: &crate::DrawingTransform) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { transform: Some(transform.clone()), ..Default::default() })
}

/// ↔️ Several layers' transform patches in one sparse delta, in the given order.
pub fn diff_set_layer_transforms(entries: impl IntoIterator<Item = (String, crate::DrawingTransform)>) -> DrawingDiff {
    let patched = entries.into_iter().map(|(id, transform)| DrawingLayerPatchEntry { id, patch: DrawingLayerPatch { transform: Some(transform), ..Default::default() } }).collect();
    DrawingDiff { layers: Some(DrawingLayersDelta { patched, ..Default::default() }), ..Default::default() }
}

/// 🎨 Layer fill patch.
pub fn diff_set_fill(layer_id: &(impl std::fmt::Display + ?Sized), fill: &Option<FillStyle>) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { fill: Some(DrawingFillPatch { value: fill.clone() }), ..Default::default() })
}

/// ✏️ Layer stroke patch.
pub fn diff_set_stroke(layer_id: &(impl std::fmt::Display + ?Sized), stroke: &Option<StrokeStyle>) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { stroke: Some(DrawingStrokePatch { value: stroke.clone() }), ..Default::default() })
}

/// 🖼️ Trace params patch.
pub fn diff_set_trace_params(layer_id: &(impl std::fmt::Display + ?Sized), params: &crate::DrawingTraceParams) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { trace_params: Some(params.clone()), ..Default::default() })
}

pub(crate) fn layer_base_patch(layer_id: &(impl std::fmt::Display + ?Sized), patch: DrawingLayerPatch) -> DrawingDiff {
    DrawingDiff { layers: Some(DrawingLayersDelta { patched: vec![DrawingLayerPatchEntry { id: layer_id.to_string(), patch }], ..Default::default() }), ..Default::default() }
}
