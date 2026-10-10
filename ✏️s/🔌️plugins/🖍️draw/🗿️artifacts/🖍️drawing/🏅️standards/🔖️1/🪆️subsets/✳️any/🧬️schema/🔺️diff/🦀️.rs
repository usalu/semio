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

/// 🧩 Positional delta for the layer TREE (the shape of the framework's `protocol::list_delta`, addressed per container): `removed`
/// rows carry their BASE address, `inserted` rows their AFTER address, `moved` rows both (a move may cross containers); no order
/// list and no anchor is ever carried. Every index is a coordinate of the base or of the after child list of its parent.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct DrawingLayersDelta {
    pub removed: Vec<DrawingLayerRemoval>,
    pub inserted: Vec<DrawingLayerInsertion>,
    pub moved: Vec<DrawingLayerRelocation>,
    pub modified: Vec<DrawingLayerModification>,
}

/// 📍️ A position in the layer tree: the child list of `parent_id` (the root list when absent) at `index`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerAddress {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➖️ One removed layer subtree and the BASE address the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerRemoval {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➕️ One inserted layer subtree and its AFTER address.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerInsertion {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: DrawingLayerNode,
}

/// ↕️ One repositioned layer subtree: its BASE address and its AFTER address — the layer itself is never carried.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerRelocation {
    pub id: String,
    pub from: DrawingLayerAddress,
    pub to: DrawingLayerAddress,
}

/// 🩹 One modified layer entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerModification {
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

/// 🔷️ One sparse authored coordinate edit, independent of transform and appearance.
#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
pub struct DrawingShapeCoordinatePatch {
    pub field:crate::schema::shape_geometry::ShapeCoordinateField,
    pub index:Option<usize>,
    pub value:f64,
}

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
    pub font_family: Option<crate::DrawingFontFamily>,
    pub image_key: Option<String>,
    pub image_width: Option<f64>,
    pub image_height: Option<f64>,
    #[value(default,skip_serializing_if="Vec::is_empty")]
    #[cfg_attr(test,serde(default,skip_serializing_if="Vec::is_empty"))]
    pub shape_coordinates:Vec<DrawingShapeCoordinatePatch>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
/// 🧩 Applies a positional layer delta to a layer tree: leaving rows (removed and moved) are checked at their base address and
/// lifted out, then every container with entering rows (inserted and moved) is rebuilt slot by slot — entering rows take their
/// after index, surviving siblings fill the remaining slots in order — and the patches write last.
pub fn apply_layers_delta(layers: &PagedList<DrawingLayerNode, {usize::MAX}>, delta: &DrawingLayersDelta) -> protocol::MutationApplyResult<PagedList<DrawingLayerNode, {usize::MAX}>> {
    let at_base = |parent: &Option<String>, index: usize, id: &str| container_of(layers, parent.as_deref()).and_then(|children| children.iter().nth(index)).is_some_and(|node| layer_id(node).eq_text(id));
    let mut leaving: Vec<&str> = Vec::new();
    for (index, row) in delta.removed.iter().enumerate() {
        if !at_base(&row.parent_id, row.index, &row.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed layer is not at its base address").at(["removed".to_string(), index.to_string()]));
        }
        if leaving.contains(&row.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is removed or moved more than once").at(["removed".to_string(), index.to_string()]));
        }
        leaving.push(&row.id);
    }
    for (index, row) in delta.moved.iter().enumerate() {
        if !at_base(&row.from.parent_id, row.from.index, &row.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "moved layer is not at its base address").at(["moved".to_string(), index.to_string()]));
        }
        if leaving.contains(&row.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is removed or moved more than once").at(["moved".to_string(), index.to_string()]));
        }
        leaving.push(&row.id);
    }
    for (index, entry) in delta.modified.iter().enumerate() {
        if !contains_layer(layers, &entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "modified layer does not exist").at(["modified".to_string(), index.to_string()]));
        }
        if delta.removed.iter().any(|row| row.id == entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "layer cannot be removed and modified").at(["modified".to_string(), index.to_string()]));
        }
        if delta.modified[..index].iter().any(|prior| prior.id == entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is modified more than once").at(["modified".to_string(), index.to_string()]));
        }
    }
    let mut next = layers.clone();
    let mut carried: BTreeMap<String, DrawingLayerNode> = BTreeMap::new();
    for row in &delta.removed {
        remove_layer_from_tree(&mut next, &row.id);
    }
    for (index, row) in delta.moved.iter().enumerate() {
        let node = take_layer(&mut next, &row.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "moved layer was removed with an ancestor").at(["moved".to_string(), index.to_string()]))?;
        carried.insert(row.id.clone(), node);
    }
    let mut pending: Vec<(Option<String>, Vec<(usize, DrawingLayerNode)>)> = Vec::new();
    let mut enter = |parent: &Option<String>, index: usize, node: DrawingLayerNode| match pending.iter_mut().find(|(container, _)| container == parent) {
        Some((_, rows)) => rows.push((index, node)),
        None => pending.push((parent.clone(), vec![(index, node)])),
    };
    for row in &delta.inserted {
        enter(&row.parent_id, row.index, row.layer.clone());
    }
    for row in &delta.moved {
        if let Some(node) = carried.remove(&row.id) {
            enter(&row.to.parent_id, row.to.index, node);
        }
    }
    while !pending.is_empty() {
        let before = pending.len();
        let mut waiting = Vec::new();
        for (parent, rows) in std::mem::take(&mut pending) {
            match container_mut(&mut next, parent.as_deref()) {
                Some(children) => fill_slots(children, rows)?,
                None => waiting.push((parent, rows)),
            }
        }
        if waiting.len() == before {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "an entering layer's parent group does not exist").at(["inserted"]));
        }
        pending = waiting;
    }
    for (index, entry) in delta.modified.iter().enumerate() {
        apply_layer_patch_entry(&mut next, entry).map_err(|error| error.under(["modified".to_string(), index.to_string()]))?;
    }
    validate_unique_layer_ids(&next)?;
    Ok(next)
}

/// 🧩 Rebuilds one child list: the entering rows take their after index, the surviving rows fill the free slots in order.
fn fill_slots(children: &mut PagedList<DrawingLayerNode, {usize::MAX}>, entering: Vec<(usize, DrawingLayerNode)>) -> protocol::MutationApplyResult<()> {
    let survivors = std::mem::take(children);
    let after_len = survivors.len() + entering.len();
    let mut slots: Vec<Option<DrawingLayerNode>> = (0..after_len).map(|_| None).collect();
    for (index, (at, node)) in entering.into_iter().enumerate() {
        match slots.get_mut(at) {
            None => return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "entering layer lies past the end of the after list").at(["inserted".to_string(), index.to_string()])),
            Some(slot) if slot.is_some() => return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "two layers take the same after index").at(["inserted".to_string(), index.to_string()])),
            Some(slot) => *slot = Some(node),
        }
    }
    let mut rest = survivors.into_iter();
    for slot in slots.iter_mut().filter(|slot| slot.is_none()) {
        *slot = rest.next();
    }
    let mut rebuilt = PagedList::default();
    for node in slots.into_iter().flatten() {
        rebuilt.push(node);
    }
    *children = rebuilt;
    Ok(())
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

fn container_of<'a>(layers: &'a PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<&str>) -> Option<&'a PagedList<DrawingLayerNode, {usize::MAX}>> {
    match parent_id {
        None => Some(layers),
        Some(parent_id) => layers.iter().find_map(|layer| match layer {
            DrawingLayerNode::Group(group) if group.base.id == parent_id => Some(&group.children),
            DrawingLayerNode::Group(group) => container_of(&group.children, Some(parent_id)),
            _ => None,
        }),
    }
}

fn container_mut<'a>(layers: &'a mut PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<&str>) -> Option<&'a mut PagedList<DrawingLayerNode, {usize::MAX}>> {
    let Some(parent_id) = parent_id else { return Some(layers) };
    for layer in layers.iter_mut() {
        if let DrawingLayerNode::Group(group) = layer {
            if group.base.id == parent_id {
                return Some(&mut group.children);
            }
            if let Some(found) = container_mut(&mut group.children, Some(parent_id)) {
                return Some(found);
            }
        }
    }
    None
}

/// 🧩 Lifts the layer `id` (with its subtree) out of the tree.
fn take_layer(layers: &mut PagedList<DrawingLayerNode, {usize::MAX}>, id: &str) -> Option<DrawingLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer).eq_text(id)) {
        return Some(layers.remove(index));
    }
    layers.iter_mut().find_map(|layer| match layer {
        DrawingLayerNode::Group(group) => take_layer(&mut group.children, id),
        _ => None,
    })
}

/// 🧭️ The `(parent id, index)` address of the layer `id`.
fn address_of(layers: &PagedList<DrawingLayerNode, {usize::MAX}>, id: &str) -> Option<(Option<String>, usize)> {
    fn walk(list: &PagedList<DrawingLayerNode, {usize::MAX}>, parent: Option<&str>, id: &str) -> Option<(Option<String>, usize)> {
        for (index, node) in list.iter().enumerate() {
            if layer_id(node).eq_text(id) {
                return Some((parent.map(str::to_owned), index));
            }
            if let DrawingLayerNode::Group(group) = node {
                if let Some(found) = walk(&group.children, Some(&layer_id(node).to_string_owner()), id) {
                    return Some(found);
                }
            }
        }
        None
    }
    walk(layers, None, id)
}

fn apply_layer_patch_entry(layers: &mut PagedList<DrawingLayerNode, {usize::MAX}>, entry: &DrawingLayerModification) -> protocol::MutationApplyResult<()> {
    let mut result = Ok(());
    if !update_layer_in_tree(layers, &entry.id, &mut |layer| {
        result = apply_layer_patch(layer, &entry.patch);
    }) {
        return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "modified layer does not exist after structural edits").at([&entry.id]));
    }
    result
}

fn apply_layer_patch(layer: &mut DrawingLayerNode, patch: &DrawingLayerPatch) -> protocol::MutationApplyResult<()> {
    if let Some(modified) = &patch.layer {
        let replacement = modified.clone();
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
    if !patch.shape_coordinates.is_empty() {
        let DrawingLayerNode::Shape(shape)=layer else{return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target","Shape target has another kind"));};
        for coordinate in &patch.shape_coordinates {
            crate::schema::shape_geometry::set_shape_coordinate(shape,coordinate.field,coordinate.index,coordinate.value).map_err(|message|protocol::MutationApplyError::new("mutation.apply.invalid-value",message))?;
        }
    }
    if patch.image_key.is_some() || patch.image_width.is_some() || patch.image_height.is_some() {
        let DrawingLayerNode::Image(image)=layer else{return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target","Image target has another kind"));};
        for dimension in [patch.image_width,patch.image_height].into_iter().flatten() {if !dimension.is_finite()||dimension<=0.0{return Err(protocol::MutationApplyError::new("mutation.apply.invalid-value","Invalid image dimension"));}}
        if let Some(key)=&patch.image_key{image.image_key=key.clone().into();}
        if let Some(width)=patch.image_width{image.width=width;}
        if let Some(height)=patch.image_height{image.height=height;}
    }
    if patch.text_content.is_some() || patch.text_size.is_some() || patch.font_family.is_some() {
        let DrawingLayerNode::Text(text) = layer else { return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "Text target has another kind")); };
        if let Some(size) = patch.text_size {
            if !size.is_finite() || size <= 0.0 { return Err(protocol::MutationApplyError::new("mutation.apply.invalid-value", "Invalid text size")); }
            text.size = size;
        }
        if let Some(content) = &patch.text_content { text.content = content.clone().into(); }
        if let Some(family)=patch.font_family{text.font_family=family;}
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
            DrawingLayerNode::Shape(shape)=>{
                for coordinate in &self.shape_coordinates {
                    if let Ok(value)=crate::schema::shape_geometry::shape_coordinate(shape,&coordinate.field,coordinate.index){inverse.shape_coordinates.push(DrawingShapeCoordinatePatch{field:coordinate.field,index:coordinate.index,value});}
                }
            }
            DrawingLayerNode::Image(image) => {
                if self.image_key.is_some(){inverse.image_key=Some(image.image_key.to_string_owner());}
                if self.image_width.is_some(){inverse.image_width=Some(image.width);}
                if self.image_height.is_some(){inverse.image_height=Some(image.height);}
            }
            DrawingLayerNode::Text(text) => {
                if self.text_content.is_some() {
                    inverse.text_content = Some(text.content.to_string_owner());
                }
                if self.font_family.is_some(){inverse.font_family=Some(text.font_family);}
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
        take!(font_family);
        take!(image_key);
        take!(image_width);
        take!(image_height);
        for coordinate in src.shape_coordinates {
            if let Some(current)=self.shape_coordinates.iter_mut().find(|current|current.field==coordinate.field&&current.index==coordinate.index){*current=coordinate;}else{self.shape_coordinates.push(coordinate);}
        }
    }
}

type Layers = PagedList<DrawingLayerNode, {usize::MAX}>;

fn ids_of(node: &DrawingLayerNode) -> BTreeSet<String> {
    let nested: BTreeSet<String> = match node {
        DrawingLayerNode::Group(group) => group.children.iter().flat_map(ids_of).collect(),
        _ => BTreeSet::new(),
    };
    nested.into_iter().chain(std::iter::once(layer_id(node).to_string_owner())).collect()
}

/// 🔑️ One row of a per-container list during `absorb`: an inserted subtree, or the arrival of a layer that moved in.
#[derive(Clone, Debug, PartialEq)]
struct Slot {
    id: String,
    node: Option<DrawingLayerNode>,
}

impl protocol::list_delta::Keyed for Slot {
    type Key = String;
    fn key(&self) -> String {
        self.id.clone()
    }
}

type ContainerParts = protocol::list_delta::Parts<Slot, protocol::list_delta::NoPatch>;

/// 🧹 Drops the move rows that leave their layer exactly where the surviving siblings already put it: a move inside one container
/// whose base index minus the other leaving rows before it equals its after index minus the other entering rows before it.
fn without_identity_moves(moved: Vec<DrawingLayerRelocation>, removed: &[DrawingLayerRemoval], inserted: &[DrawingLayerInsertion]) -> Vec<DrawingLayerRelocation> {
    let mut kept = moved;
    loop {
        let identity = kept.iter().position(|row| {
            if row.from.parent_id != row.to.parent_id {
                return false;
            }
            let container = &row.from.parent_id;
            let others = kept.iter().filter(|other| other.id != row.id);
            let leaving = removed.iter().filter(|gone| gone.parent_id == *container && gone.index < row.from.index).count() + others.clone().filter(|other| other.from.parent_id == *container && other.from.index < row.from.index).count();
            let entering = inserted.iter().filter(|born| born.parent_id == *container && born.index < row.to.index).count() + others.filter(|other| other.to.parent_id == *container && other.to.index < row.to.index).count();
            row.from.index + entering == row.to.index + leaving
        });
        match identity {
            Some(at) => {
                kept.remove(at);
            }
            None => return kept,
        }
    }
}

/// 🔎️ Finds the group `id` inside a set of inserted subtrees, mutably.
fn group_children_mut<'a>(inserted: &'a mut [DrawingLayerInsertion], id: &str) -> Option<&'a mut Layers> {
    inserted.iter_mut().find_map(|row| match &mut row.layer {
        DrawingLayerNode::Group(group) => {
            if group.base.id == id {
                Some(&mut group.children)
            } else {
                container_mut(&mut group.children, Some(id))
            }
        }
        _ => None,
    })
}

/// 🩹 Applies `patch` to the layer `id` somewhere inside `node`'s subtree.
fn patch_inside(node: &mut DrawingLayerNode, entry: &DrawingLayerModification) -> bool {
    if layer_id(node).eq_text(&entry.id) {
        return apply_layer_patch(node, &entry.patch).is_ok();
    }
    match node {
        DrawingLayerNode::Group(group) => group.children.iter_mut().any(|child| patch_inside(child, entry)),
        _ => false,
    }
}

impl DrawingLayersDelta {
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| entry.patch.is_empty())
    }

    fn normalize(&mut self) {
        self.modified.sort_by(|left, right| left.id.cmp(&right.id));
    }

    /// 🧱️ The per-container lists of the delta: a leaving row (removed or moved out) is a `removed` entry of its container; an
    /// entering row (inserted or moved in) an `inserted` entry of its container.
    fn container_parts(&self) -> BTreeMap<Option<String>, ContainerParts> {
        let mut parts: BTreeMap<Option<String>, ContainerParts> = BTreeMap::new();
        for row in &self.removed {
            parts.entry(row.parent_id.clone()).or_default().removed.push((row.id.clone(), row.index));
        }
        for row in &self.moved {
            parts.entry(row.from.parent_id.clone()).or_default().removed.push((row.id.clone(), row.from.index));
            parts.entry(row.to.parent_id.clone()).or_default().inserted.push((row.to.index, Slot { id: row.id.clone(), node: None }));
        }
        for row in &self.inserted {
            parts.entry(row.parent_id.clone()).or_default().inserted.push((row.index, Slot { id: layer_id(&row.layer).to_string_owner(), node: Some(row.layer.clone()) }));
        }
        parts
    }

    /// 🧩 Folds the edits of `later` that reach INTO subtrees this delta inserts straight into those subtrees: a mid-state container
    /// that does not exist in the base cannot carry a base coordinate. The leaving rows are lifted out of their containers first
    /// (every index is a mid coordinate, so one container's rows go from the highest index down), then the entering rows take their
    /// after slots. A layer that moves from the base INTO a subtree inserted here cannot be expressed and stays a move row, which
    /// then refuses at apply.
    fn fold_into_inserted(&mut self, later: &mut Self) {
        let territory: BTreeSet<String> = self.inserted.iter().flat_map(|row| ids_of(&row.layer)).collect();
        let inside = |parent: &Option<String>| parent.as_deref().is_some_and(|parent| territory.contains(parent));
        let mut leaving: BTreeMap<String, Vec<(usize, String, bool)>> = BTreeMap::new();
        for row in std::mem::take(&mut later.removed) {
            if inside(&row.parent_id) {
                leaving.entry(row.parent_id.clone().unwrap_or_default()).or_default().push((row.index, row.id, true));
            } else {
                later.removed.push(row);
            }
        }
        let mut leaving_moves = Vec::new();
        for row in std::mem::take(&mut later.moved) {
            if inside(&row.from.parent_id) {
                leaving.entry(row.from.parent_id.clone().unwrap_or_default()).or_default().push((row.from.index, row.id.clone(), false));
                leaving_moves.push(row);
            } else {
                later.moved.push(row);
            }
        }
        let mut lifted: BTreeMap<String, DrawingLayerNode> = BTreeMap::new();
        for (container, mut rows) in leaving {
            rows.sort_by(|left, right| right.0.cmp(&left.0));
            let Some(children) = group_children_mut(&mut self.inserted, &container) else { continue };
            for (index, id, removed) in rows {
                if children.iter().nth(index).is_some_and(|node| layer_id(node).eq_text(&id)) {
                    let node = children.remove(index);
                    if !removed {
                        lifted.insert(id, node);
                    }
                }
            }
        }
        let mut entering: BTreeMap<String, Vec<(usize, DrawingLayerNode)>> = BTreeMap::new();
        for row in leaving_moves {
            match lifted.remove(&row.id) {
                Some(node) if inside(&row.to.parent_id) => entering.entry(row.to.parent_id.clone().unwrap_or_default()).or_default().push((row.to.index, node)),
                Some(node) => later.inserted.push(DrawingLayerInsertion { parent_id: row.to.parent_id, index: row.to.index, layer: node }),
                None => later.moved.push(row),
            }
        }
        for insertion in std::mem::take(&mut later.inserted) {
            if inside(&insertion.parent_id) {
                entering.entry(insertion.parent_id.clone().unwrap_or_default()).or_default().push((insertion.index, insertion.layer));
            } else {
                later.inserted.push(insertion);
            }
        }
        for (container, rows) in entering {
            if let Some(children) = group_children_mut(&mut self.inserted, &container) {
                let _ = fill_slots(children, rows);
            }
        }
    }

    /// ➕️ Composes `self` (base→mid) with `later` (mid→after): per container the framework's positional algebra coalesces the
    /// rows (insert∘remove cancels, insert∘move lands at its final slot, move∘move is one move, move∘remove removes at the base
    /// address, remove∘insert of one id is a replacement); a layer that left one container and entered another stays one move row.
    fn absorb(&mut self, mut later: Self) {
        self.fold_into_inserted(&mut later);
        let carried: BTreeMap<String, DrawingLayerNode> = self.inserted.iter().map(|row| (layer_id(&row.layer).to_string_owner(), row.layer.clone())).collect();
        let mut right = later.container_parts();
        let mut combined: BTreeMap<Option<String>, ContainerParts> = BTreeMap::new();
        let mut left = self.container_parts();
        let containers: BTreeSet<Option<String>> = left.keys().chain(right.keys()).cloned().collect();
        for container in containers {
            let mut parts = left.remove(&container).unwrap_or_default();
            parts.absorb(right.remove(&container).unwrap_or_default());
            combined.insert(container, parts);
        }
        let mut leaving: Vec<(Option<String>, String, usize)> = Vec::new();
        let mut entering: Vec<(Option<String>, usize, Slot)> = Vec::new();
        for (container, parts) in combined {
            leaving.extend(parts.removed.into_iter().map(|(id, index)| (container.clone(), id, index)));
            entering.extend(parts.inserted.into_iter().map(|(index, slot)| (container.clone(), index, slot)));
        }
        let mut inserted = Vec::new();
        let mut moved = Vec::new();
        for (container, index, slot) in entering {
            match slot.node {
                Some(layer) => inserted.push(DrawingLayerInsertion { parent_id: container, index, layer }),
                None => {
                    if let Some(at) = leaving.iter().position(|(_, id, _)| *id == slot.id) {
                        let (from_parent, id, from_index) = leaving.remove(at);
                        moved.push(DrawingLayerRelocation { id, from: DrawingLayerAddress { parent_id: from_parent, index: from_index }, to: DrawingLayerAddress { parent_id: container, index } });
                    } else if let Some(layer) = carried.get(&slot.id) {
                        inserted.push(DrawingLayerInsertion { parent_id: container, index, layer: layer.clone() });
                    }
                }
            }
        }
        let removed: Vec<DrawingLayerRemoval> = leaving.into_iter().map(|(parent_id, id, index)| DrawingLayerRemoval { id, parent_id, index }).collect();
        let moved = without_identity_moves(moved, &removed, &inserted);
        let born: BTreeSet<String> = carried.keys().cloned().chain(inserted.iter().flat_map(|row| ids_of(&row.layer))).collect();
        let gone: BTreeSet<&str> = removed.iter().map(|row| row.id.as_str()).filter(|id| !inserted.iter().any(|row| layer_id(&row.layer).eq_text(*id))).collect();
        let mut modified = std::mem::take(&mut self.modified);
        modified.retain(|entry| !gone.contains(entry.id.as_str()));
        for entry in later.modified {
            if gone.contains(entry.id.as_str()) {
                continue;
            }
            if born.contains(&entry.id) {
                inserted.iter_mut().any(|row| patch_inside(&mut row.layer, &entry));
                continue;
            }
            match modified.iter_mut().find(|existing| existing.id == entry.id) {
                Some(existing) => existing.patch.absorb(entry.patch),
                None => modified.push(entry),
            }
        }
        *self = Self { removed, inserted, moved, modified };
        self.normalize();
    }

    /// 🔁️ The negative delta, read row by row from the BASE tree: inserted rows are removed at their after address, removed rows
    /// are reinserted at their base address with the subtree the base holds there, moves run backwards, patches restore the base
    /// fields. Nothing is applied or simulated.
    fn inverse(&self, base: &Layers) -> Self {
        let inserted_ids: BTreeSet<String> = self.inserted.iter().flat_map(|row| ids_of(&row.layer)).collect();
        let removed = self
            .inserted
            .iter()
            .filter(|row| row.parent_id.as_ref().is_none_or(|parent| !inserted_ids.contains(parent)))
            .map(|row| DrawingLayerRemoval { id: layer_id(&row.layer).to_string_owner(), parent_id: row.parent_id.clone(), index: row.index })
            .collect();
        let inserted = self
            .removed
            .iter()
            .filter_map(|row| container_of(base, row.parent_id.as_deref()).and_then(|children| children.iter().nth(row.index)).map(|node| DrawingLayerInsertion { parent_id: row.parent_id.clone(), index: row.index, layer: node.clone() }))
            .collect();
        let moved = self.moved.iter().map(|row| DrawingLayerRelocation { id: row.id.clone(), from: row.to.clone(), to: row.from.clone() }).collect();
        let modified = self
            .modified
            .iter()
            .filter(|entry| !inserted_ids.contains(&entry.id))
            .filter_map(|entry| find_node(base, &entry.id).map(|source| DrawingLayerModification { id: entry.id.clone(), patch: entry.patch.inverse(source) }))
            .collect();
        let mut inverse = Self { removed, inserted, moved, modified };
        inverse.normalize();
        inverse
    }
}

fn find_node<'a>(layers: &'a Layers, id: &str) -> Option<&'a DrawingLayerNode> {
    layers.iter().find_map(|node| {
        if layer_id(node).eq_text(id) {
            return Some(node);
        }
        match node {
            DrawingLayerNode::Group(group) => find_node(&group.children, id),
            _ => None,
        }
    })
}

impl DrawingAssetsDelta {
    /// 🔁️ The entries that restore the BASE value of every key this delta touches.
    fn inverse(&self, base: &PagedMap<DrawingImageAsset, {usize::MAX}>) -> Self {
        Self { entries: self.entries.keys().map(|key| (key.clone(), base.get(key).cloned())).collect() }
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
    let modified = entries.into_iter().map(|(id, segments)| DrawingLayerModification { id, patch: DrawingLayerPatch { path_segments: Some(segments.into_iter().collect()), ..Default::default() } }).collect();
    DrawingDiff { layers: Some(DrawingLayersDelta { modified, ..Default::default() }), ..Default::default() }
}





/// 🔀 Boolean operation patch.
pub fn diff_set_boolean_operation(layer_id: &(impl std::fmt::Display + ?Sized), boolean_operation: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { boolean_operation: Some(boolean_operation.to_string()), ..Default::default() })
}



/// 🌱️ Layer insertion into `parent_id` (root when `None`) at the after index `index` (clamped to the end of the child list).
pub fn diff_create_layer(base: &PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<&PagedUtf8<{usize::MAX}>>, index: usize, layer: DrawingLayerNode) -> DrawingDiff {
    let parent = parent_id.map(PagedUtf8::to_string_owner);
    let length = container_of(base, parent.as_deref()).map_or(0, |children| children.len());
    DrawingDiff { layers: Some(DrawingLayersDelta { inserted: vec![DrawingLayerInsertion { parent_id: parent, index: index.min(length), layer }], ..Default::default() }), ..Default::default() }
}

/// 🔃 Move the layer `layer_id` to the after index `index` of `parent_id`'s child list — one tree-aware move row from its base
/// address, never the layer itself. Empty when the layer is not in `base`.
pub fn diff_reorder_layer(base: &PagedList<DrawingLayerNode, {usize::MAX}>, layer_id: &(impl std::fmt::Display + ?Sized), parent_id: Option<&PagedUtf8<{usize::MAX}>>, index: usize) -> DrawingDiff {
    let id = layer_id.to_string();
    let Some((from_parent, from_index)) = address_of(base, &id) else { return DrawingDiff::default() };
    let parent = parent_id.map(PagedUtf8::to_string_owner);
    let length = container_of(base, parent.as_deref()).map_or(0, |children| children.len());
    let room = if parent == from_parent { length.saturating_sub(1) } else { length };
    DrawingDiff {
        layers: Some(DrawingLayersDelta {
            moved: vec![DrawingLayerRelocation { id, from: DrawingLayerAddress { parent_id: from_parent, index: from_index }, to: DrawingLayerAddress { parent_id: parent, index: index.min(room) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// ➖️ Layer remove: one removal row at the layer's base address. Empty when the layer is not in `base`.
pub fn diff_remove_layer(base: &PagedList<DrawingLayerNode, {usize::MAX}>, layer_id: &(impl std::fmt::Display + ?Sized)) -> DrawingDiff {
    let id = layer_id.to_string();
    let Some((parent_id, index)) = address_of(base, &id) else { return DrawingDiff::default() };
    DrawingDiff { layers: Some(DrawingLayersDelta { removed: vec![DrawingLayerRemoval { id, parent_id, index }], ..Default::default() }), ..Default::default() }
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


/// 🌀️ Sparse authored fill-rule delta.
pub fn diff_set_layer_fill_rule(layer_id:&(impl std::fmt::Display + ?Sized),fill_rule:crate::FillRule)->DrawingDiff {layer_base_patch(layer_id,DrawingLayerPatch {fill_rule:Some(fill_rule),..Default::default()})}

pub fn diff_set_group_isolation(layer_id:&(impl std::fmt::Display + ?Sized),isolation:bool)->DrawingDiff {layer_base_patch(layer_id,DrawingLayerPatch {isolation:Some(isolation),..Default::default()})}

/// ↔️ Layer transform patch.
pub fn diff_set_layer_transform(layer_id: &(impl std::fmt::Display + ?Sized), transform: &crate::DrawingTransform) -> DrawingDiff {
    layer_base_patch(layer_id, DrawingLayerPatch { transform: Some(transform.clone()), ..Default::default() })
}

/// ↔️ Several layers' transform patches in one sparse delta, in the given order.
pub fn diff_set_layer_transforms(entries: impl IntoIterator<Item = (String, crate::DrawingTransform)>) -> DrawingDiff {
    let modified = entries.into_iter().map(|(id, transform)| DrawingLayerModification { id, patch: DrawingLayerPatch { transform: Some(transform), ..Default::default() } }).collect();
    DrawingDiff { layers: Some(DrawingLayersDelta { modified, ..Default::default() }), ..Default::default() }
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
    DrawingDiff { layers: Some(DrawingLayersDelta { modified: vec![DrawingLayerModification { id: layer_id.to_string(), patch }], ..Default::default() }), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
