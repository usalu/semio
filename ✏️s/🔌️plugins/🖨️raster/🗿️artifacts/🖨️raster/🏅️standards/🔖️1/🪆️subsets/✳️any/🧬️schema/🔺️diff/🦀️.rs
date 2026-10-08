//! 🧬️ Raster diff schema — sparse field delta over the artifact.

use crate::{SemioImageSnapshot, RasterLayerNode, RasterLayerPatch};
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the raster artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub layers: Option<RasterLayersDelta>,
    #[state(artifact)]
    pub assets: Option<RasterAssetsDelta>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub pixels: Vec<RasterPixelRegion>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterAssetsDelta {
    pub entries: BTreeMap<String, Option<SemioImageSnapshot>>,
}

/// 🖌️ One rectangle of pixel samples written into the image a layer (or its mask) shows: `width × height` RGBA8 samples,
/// row-major, at `(x, y)` of the target image. Applying it files the rewritten image as a new content-addressed asset and
/// repoints the layer — the handle is derived by the applier from the applied pixels, never carried by the diff.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterPixelRegion {
    pub layer_id: String,
    pub target: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub samples: Vec<u8>,
}

/// 🧩 Positional delta for the layer TREE (the shape of `protocol::list_delta`, addressed per container): `removed` rows carry
/// their BASE address, `inserted` rows their AFTER address, `moved` rows both (a move may cross containers); `parent_id: None`
/// means the document root. No order list and no anchor is ever carried; every index is a coordinate of the base or of the after
/// child list of its parent.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterLayersDelta {
    pub removed: Vec<RasterLayerRemoval>,
    pub inserted: Vec<RasterLayerInsertion>,
    pub moved: Vec<RasterLayerRelocation>,
    pub modified: Vec<RasterLayerModification>,
}

/// 📍️ A position in the layer tree: the child list of `parent_id` (the root list when absent) at `index`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerAddress {
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➖️ One removed layer subtree and the BASE address the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerRemoval {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➕ One inserted layer subtree (`create-layer`) and its AFTER address.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerInsertion {
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: RasterLayerNode,
}

/// ↕️ One repositioned layer subtree (`reorder-layers`): its BASE address and its AFTER address — the layer itself is never carried.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerRelocation {
    pub id: String,
    pub from: RasterLayerAddress,
    pub to: RasterLayerAddress,
}

/// 🩹 One modified layer entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerModification {
    pub id: String,
    pub patch: RasterLayerPatch,
}
//#endregion 🔖️DeltaHelpers

use crate::standards::v1::subsets::any::schema::find_layer;
use crate::standards::v1::subsets::any::schema::flatten_raster_layers;
use crate::standards::v1::subsets::any::schema::layer_node_id;
use crate::RasterSnapshot;
use protocol::MutationDiff;

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

fn contains_layer(node: &RasterLayerNode, target_id: &str) -> bool {
    layer_node_id(node) == target_id || matches!(node, RasterLayerNode::Group { children, .. } if children.iter().any(|child| contains_layer(child, target_id)))
}

fn validate_layer_patch(node: &RasterLayerNode, patch: &RasterLayerPatch) -> protocol::MutationApplyResult<()> {
    if patch.transform.is_some()&&(patch.transform_x.is_some()||patch.transform_y.is_some()){return Err(protocol::MutationApplyError::new("mutation.apply.ambiguous-transform","full and partial transforms cannot occur in one patch"));}
    if let Some(transform)=&patch.transform {semio_framework_pixels::compositing::inverse(transform.as_affine()).map_err(|_|protocol::MutationApplyError::new("mutation.apply.invalid-transform","transform must be invertible"))?;}
    let invalid = match node {
        RasterLayerNode::Pixel { .. } => patch.adjustment_kind.is_some() || patch.adjustment_parameters.is_some(),
        RasterLayerNode::Group { .. } => patch.pixel_content.is_some() || patch.width.is_some() || patch.height.is_some() || patch.adjustment_kind.is_some() || patch.adjustment_parameters.is_some(),
        RasterLayerNode::Adjustment { .. } => patch.mask_content.is_some() || patch.pixel_content.is_some() || patch.transform.is_some() || patch.transform_x.is_some() || patch.transform_y.is_some() || patch.width.is_some() || patch.height.is_some(),
    };
    if let Some(parameters) = &patch.adjustment_parameters {
        if parameters.len()>2 || parameters.iter().enumerate().any(|(index,row)| !matches!(row.parameter.as_str(),"brightness"|"contrast") || row.value.is_some_and(|v| !v.get().is_finite() || !(-1.0..=1.0).contains(&v.get())) || parameters[..index].iter().any(|prior| prior.parameter==row.parameter)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-parameter","invalid adjustment parameter patch"));
        }
        if let RasterLayerNode::Adjustment {params,..}=node {
            if parameters.iter().any(|row| params.get(&row.parameter).is_some_and(|v|v.as_f64().is_none())) {return Err(protocol::MutationApplyError::new("mutation.apply.parameter-type","existing adjustment parameter must be numeric"));}
            let added=parameters.iter().filter(|row| row.value.is_some()&&!params.contains_key(&row.parameter)).count();
            let removed=parameters.iter().filter(|row|row.value.is_none()&&params.contains_key(&row.parameter)).count();
            if params.len()+added-removed>crate::RASTER_OWNED_MAP_CAPACITY {return Err(protocol::MutationApplyError::new("mutation.apply.parameter-capacity","adjustment parameter capacity exceeded"));}
        }
    }
    if invalid {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "layer patch contains fields unsupported by the target layer kind"));
    }
    Ok(())
}

fn apply_layer_patch(node: &mut RasterLayerNode, patch: &RasterLayerPatch) -> RasterLayerPatch {
    let mut inverse = RasterLayerPatch::default();
    if let (Some(content), RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = (&patch.mask_content, &mut *node) {
        inverse.mask_content = Some(crate::RasterMaskContent { mask: std::mem::replace(mask, content.mask.clone()) });
    }
    match node {
        RasterLayerNode::Pixel { name, visible, locked, opacity, blend_mode, transform, width, height, image_key, .. } => {
            if let Some(content) = &patch.pixel_content {
                inverse.pixel_content = Some(crate::RasterPixelContent { image_key: image_key.clone(), width: *width, height: *height });
                *image_key = content.image_key.clone();
                *width = content.width;
                *height = content.height;
            }
            if let Some(value) = &patch.transform {
                inverse.transform = Some(transform.clone());
                *transform = value.clone();
            }
            if let Some(value) = &patch.name {
                inverse.name = Some(name.clone());
                *name = value.clone();
            }
            if let Some(value) = patch.visible {
                inverse.visible = Some(*visible);
                *visible = value;
            }
            if let Some(value)=patch.locked {inverse.locked=Some(*locked);*locked=value;}
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
        RasterLayerNode::Group { name, visible, locked, opacity, blend_mode, transform, .. } => {
            if let Some(value)=&patch.transform {inverse.transform=Some(std::mem::replace(transform,value.clone()));}
            if let Some(value) = &patch.name {
                inverse.name = Some(name.clone());
                *name = value.clone();
            }
            if let Some(value) = patch.visible {
                inverse.visible = Some(*visible);
                *visible = value;
            }
            if let Some(value)=patch.locked {inverse.locked=Some(*locked);*locked=value;}
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
        RasterLayerNode::Adjustment { name, visible, locked, opacity, blend_mode, adjustment_kind, params, .. } => {
            if let Some(parameters)=&patch.adjustment_parameters {
                inverse.adjustment_parameters=Some(parameters.iter().map(|row| {
                    let previous=params.remove_entry(&row.parameter).map(|mut entry| entry.take().1);
                    let value=previous.as_ref().and_then(crate::RasterAdjustmentNumber::from_parameter);
                    crate::RasterAdjustmentParameter {parameter:row.parameter.clone(),value}
                }).collect());
                for row in parameters {if let Some(next)=row.value {params.insert(row.parameter.clone(),next.literal()).expect("validated parameter fits owned map");}}
            }
            if let Some(value) = &patch.name {
                inverse.name = Some(name.clone());
                *name = value.clone();
            }
            if let Some(value) = patch.visible {
                inverse.visible = Some(*visible);
                *visible = value;
            }
            if let Some(value)=patch.locked {inverse.locked=Some(*locked);*locked=value;}
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

/// 📍️ The child list of `parent_id` (the root list when absent).
fn container_of<'a>(layers: &'a [RasterLayerNode], parent_id: Option<&str>) -> Option<&'a [RasterLayerNode]> {
    let Some(parent_id) = parent_id else { return Some(layers) };
    layers.iter().find_map(|layer| match layer {
        RasterLayerNode::Group { id, children, .. } if id.as_str() == parent_id => Some(children.as_slice()),
        RasterLayerNode::Group { children, .. } => container_of(children, Some(parent_id)),
        _ => None,
    })
}

fn container_mut<'a>(layers: &'a mut Vec<RasterLayerNode>, parent_id: Option<&str>) -> Option<&'a mut Vec<RasterLayerNode>> {
    let Some(parent_id) = parent_id else { return Some(layers) };
    for layer in layers.iter_mut() {
        if let RasterLayerNode::Group { id, children, .. } = layer {
            if id.as_str() == parent_id {
                return Some(children);
            }
            if let Some(found) = container_mut(children, Some(parent_id)) {
                return Some(found);
            }
        }
    }
    None
}

/// 🧮️ Checks that `entering` rows fit the after list of a child list that keeps `survivors` rows: every after index lies inside it,
/// no two rows take one slot.
fn check_slots(survivors: usize, entering: &[(usize, RasterLayerNode)]) -> protocol::MutationApplyResult<()> {
    let after = survivors + entering.len();
    for (index, (at, _)) in entering.iter().enumerate() {
        if *at >= after {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "entering layer lies past the end of the after list").at(["inserted".to_string(), index.to_string()]));
        }
        if entering[..index].iter().any(|(prior, _)| prior == at) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "two layers take the same after index").at(["inserted".to_string(), index.to_string()]));
        }
    }
    Ok(())
}

/// 🧩 Rebuilds one child list: the entering rows take their after index, the surviving rows fill the free slots in order. The
/// rows were validated by [`check_slots`].
fn place_entering(children: &mut Vec<RasterLayerNode>, entering: Vec<(usize, RasterLayerNode)>) {
    let survivors = std::mem::take(children);
    let mut slots: Vec<Option<RasterLayerNode>> = (0..survivors.len() + entering.len()).map(|_| None).collect();
    for (at, node) in entering {
        slots[at] = Some(node);
    }
    let mut rest = survivors.into_iter();
    for slot in slots.iter_mut().filter(|slot| slot.is_none()) {
        *slot = rest.next();
    }
    *children = slots.into_iter().flatten().collect();
}

/// 🧩 Applies a positional layer delta to a layer tree: leaving rows (removed and moved) are checked at their base address and
/// lifted out, then every container with entering rows (inserted and moved) is rebuilt slot by slot — entering rows take their
/// after index, surviving siblings fill the remaining slots in order — and the patches write last.
pub fn apply_layers_delta(layers: &[RasterLayerNode], delta: &RasterLayersDelta) -> protocol::MutationApplyResult<Vec<RasterLayerNode>> {
    let at_base = |parent: &Option<String>, index: usize, id: &str| container_of(layers, parent.as_deref()).and_then(|children| children.get(index)).is_some_and(|node| layer_node_id(node) == id);
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
        let node = find_layer(layers, &row.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "moved layer does not exist").at(["moved".to_string(), index.to_string()]))?;
        if row.to.parent_id.as_deref().is_some_and(|parent_id| contains_layer(node, parent_id)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "layer cannot be moved beneath itself").at(["moved".to_string(), index.to_string(), "to".to_string()]));
        }
    }
    let mut modified = std::collections::BTreeSet::new();
    for (index, entry) in delta.modified.iter().enumerate() {
        if !modified.insert(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "layer is modified more than once").at(["modified".to_string(), index.to_string()]));
        }
        if delta.removed.iter().any(|row| row.id == entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "layer cannot be removed and modified").at(["modified".to_string(), index.to_string()]));
        }
        let node = find_layer(layers, &entry.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "modified layer does not exist").at(["modified".to_string(), index.to_string()]))?;
        validate_layer_patch(node, &entry.patch).map_err(|error| error.under(["modified".to_string(), index.to_string()]))?;
    }
    let mut identities: std::collections::BTreeSet<String> = flatten_raster_layers(layers).into_iter().map(|node| layer_node_id(node).to_string()).collect();
    for row in &delta.removed {
        for id in find_layer(layers, &row.id).map(subtree_ids_of).unwrap_or_default() {
            identities.remove(&id);
        }
    }
    for (index, insertion) in delta.inserted.iter().enumerate() {
        if !identities.insert(layer_node_id(&insertion.layer).to_string()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "inserted layer identity already exists").at(["inserted".to_string(), index.to_string()]));
        }
    }
    let mut next = layers.to_vec();
    for row in &delta.removed {
        if let Some(removed) = remove_layer_from_tree(&mut next, &row.id) {
            crate::retire_raster_layer(removed);
        }
    }
    let mut pending: Vec<(Option<String>, Vec<(usize, RasterLayerNode)>)> = Vec::new();
    for (index, row) in delta.moved.iter().enumerate() {
        let Some(node) = remove_layer_from_tree(&mut next, &row.id) else {
            retire_pending(pending, next);
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "moved layer was removed with an ancestor").at(["moved".to_string(), index.to_string()]));
        };
        enter(&mut pending, &row.to.parent_id, row.to.index, node);
    }
    for insertion in &delta.inserted {
        enter(&mut pending, &insertion.parent_id, insertion.index, insertion.layer.clone());
    }
    while !pending.is_empty() {
        let before = pending.len();
        let mut waiting = Vec::new();
        let mut batch = std::mem::take(&mut pending).into_iter();
        while let Some((parent, rows)) = batch.next() {
            match container_mut(&mut next, parent.as_deref()) {
                Some(children) => match check_slots(children.len(), &rows) {
                    Ok(()) => place_entering(children, rows),
                    Err(error) => {
                        crate::retire_raster_layers(rows.into_iter().map(|(_, node)| node).collect());
                        retire_pending(batch.collect(), Vec::new());
                        retire_pending(waiting, next);
                        return Err(error);
                    }
                },
                None => waiting.push((parent, rows)),
            }
        }
        if waiting.len() == before {
            retire_pending(waiting, next);
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "an entering layer's parent group does not exist").at(["inserted"]));
        }
        pending = waiting;
    }
    for (index, entry) in delta.modified.iter().enumerate() {
        if let Err(error) = apply_layer_patch_entry(&mut next, entry) {
            crate::retire_raster_layers(next);
            return Err(error.under(["modified".to_string(), index.to_string()]));
        }
    }
    let next_ids: Vec<_> = flatten_raster_layers(&next).into_iter().map(layer_node_id).collect();
    if next_ids.iter().enumerate().any(|(index, id)| next_ids[..index].contains(id)) {
        drop(next_ids);
        crate::retire_raster_layers(next);
        return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "resulting layer tree contains duplicate identities").at(["identities"]));
    }
    Ok(next)
}

/// ➡️ Files `node` as an entering row of the container `parent`.
fn enter(pending: &mut Vec<(Option<String>, Vec<(usize, RasterLayerNode)>)>, parent: &Option<String>, index: usize, node: RasterLayerNode) {
    match pending.iter_mut().find(|(container, _)| container == parent) {
        Some((_, rows)) => rows.push((index, node)),
        None => pending.push((parent.clone(), vec![(index, node)])),
    }
}

/// 🫧 Closes the owners a refused apply still holds.
fn retire_pending(pending: Vec<(Option<String>, Vec<(usize, RasterLayerNode)>)>, tree: Vec<RasterLayerNode>) {
    for (_, rows) in pending {
        crate::retire_raster_layers(rows.into_iter().map(|(_, node)| node).collect());
    }
    crate::retire_raster_layers(tree);
}

fn apply_layer_patch_entry(layers: &mut [RasterLayerNode], entry: &RasterLayerModification) -> protocol::MutationApplyResult<()> {
    patch_layer_in_tree(layers, &entry.id, &entry.patch).map(|_| ()).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "modified layer does not exist"))
}

fn validate_assets_delta<T>(assets: &crate::RasterOwnedMap<T>, delta: &RasterAssetsDelta) -> protocol::MutationApplyResult<()> {
    let additional = delta.entries.iter().filter(|(key, value)| value.is_some() && !assets.contains_key(key)).count();
    assets.validate_additional_unique_entries(additional).map_err(|reason| protocol::MutationApplyError::new("mutation.apply.capacity", reason))?;
    for (key, value) in &delta.entries {
        if value.is_none() && !assets.contains_key(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed asset does not exist").at([key.as_str()]));
        }
    }
    Ok(())
}

impl MutationDiff<RasterSnapshot> for RasterDiff {
    /// 🧬️ Applies the sparse delta to a snapshot, POPULATED maps included. `RasterOwnedMap::clone` is
    /// a real deep copy bounded by construction (64 entries over 8 pages) and needs no retained page
    /// authority, so the blanket "populated Raster maps require the retained initialization
    /// authority" refusal that used to head this function only ever stopped the framework's own
    /// history folds: `redo` re-applies the reinstated edit's forwards through here, and so do a fold
    /// to base, a `.spr` reload and a remote ingest — every one of them was refused on a demo-shaped
    /// document (🖨️raster's redo clause, measured 2026-09-20). Asset REMOVAL is not retained either:
    /// the map hands back its exact `(key, child)` pair and the emptied page backing is released
    /// explicitly, so the history arithmetic can undo and redo `remove-layer-asset` like any verb.
    fn apply(&self, snapshot: &RasterSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RasterSnapshot> {
        Ok({
            if let Some(assets) = &self.assets {
                validate_assets_delta(&snapshot.assets, assets).map_err(|error| error.under(["assets"]))?;
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
                let displaced = std::mem::take(&mut next.layers);
                let applied = apply_layers_delta(&displaced, delta);
                crate::retire_raster_layers(displaced);
                match applied {
                    Ok(layers) => next.layers = layers,
                    Err(error) => {
                        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(next);
                        return Err(error.under(["layers"]));
                    }
                }
            }
            if let Some(assets) = &self.assets {
                for (key, value) in &assets.entries {
                    match value {
                        Some(asset) => {
                            next.assets.insert(key.clone(), crate::mint_raster_image_child(key, asset)).expect("unique Raster assets fit the preflighted map capacity");
                        }
                        // 🗑️ A removal used to be refused outright ("asset removal requires the retained
                        // Raster initialization authority"), which made `remove-layer-asset` unusable on
                        // every route the framework's own history arithmetic takes — undo, redo, a fold to
                        // base, a `.spr` reload, a remote ingest. It needs no retained authority: the map
                        // hands back the exact `(key, child)` pair, neither of which carries a drop guard,
                        // and the page backing the removal empties is released explicitly, the same
                        // `take_empty_page_backing` loop every drain in this artifact runs.
                        None => {
                            let mut removed = next.assets.remove_entry(key).expect("asset removal was validated against this projection before ownership was cloned");
                            let (removed_key, removed_child) = removed.take();
                            drop(removed_key);
                            drop(removed_child);
                        }
                    }
                }
                while let Some(page) = next.assets.take_empty_page_backing() {
                    page.release();
                }
            }
            for region in &self.pixels {
                let displaced = next;
                match apply_pixel_region(&displaced, region, capability) {
                    Ok(applied) => {
                        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(displaced);
                        next = applied;
                    }
                    Err(error) => {
                        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(displaced);
                        return Err(error.under(["pixels"]));
                    }
                }
            }
            next
        })
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
        match (&mut self.layers, other.layers) {
            (Some(dst), Some(src)) => absorb_layers_delta(dst, src),
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
        self.pixels.extend(other.pixels);
    }

    fn retire_cold(self) {
        let RasterDiff { schema: _, id: _, title: _, layers, assets: _, pixels: _ } = self;
        if let Some(layers) = layers {
            for insertion in layers.inserted {
                crate::retire_raster_layer(insertion.layer);
            }
        }
    }

    fn retire_projection(projection: RasterSnapshot) {
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(projection);
    }
}

//#region 🔖️PixelRegions
/// 🖌️ Applies one pixel region: the target image is rewritten in that rectangle, filed as a new content-addressed asset and
/// the layer (or its mask) repointed — the derived handle is minted here, from the applied pixels.
fn apply_pixel_region(snapshot: &RasterSnapshot, region: &RasterPixelRegion, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RasterSnapshot> {
    use crate::mutations::paint_stroke::{canvas, painted, painted_diff, Refusal};
    let refuse = |refusal: Refusal| match refusal {
        Refusal::Error(code, message) | Refusal::Fatal(code, message) => protocol::MutationApplyError::new(code, message),
    };
    let target = canvas(&region.layer_id, &region.target, snapshot).map_err(refuse)?;
    let (width, height) = (target.source.width, target.source.height);
    let Some(frame) = target.source.frames.first() else {
        return Err(protocol::MutationApplyError::new("mutation.apply.image-invalid", "The target image has no frame."));
    };
    let fits = region.width > 0
        && region.height > 0
        && region.x.checked_add(region.width).is_some_and(|end| end <= width)
        && region.y.checked_add(region.height).is_some_and(|end| end <= height)
        && region.samples.len() == (region.width as usize) * (region.height as usize) * 4;
    if !fits {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-region", "The pixel region lies outside the image or carries the wrong sample count."));
    }
    let mut pixels = frame.rgba8.clone();
    let row_bytes = (region.width as usize) * 4;
    for row in 0..region.height as usize {
        let at = ((region.y as usize + row) * width as usize + region.x as usize) * 4;
        pixels[at..at + row_bytes].copy_from_slice(&region.samples[row * row_bytes..(row + 1) * row_bytes]);
    }
    let filed = painted(target, pixels, &region.layer_id);
    let (diff, messages) = painted_diff(filed, snapshot, &region.layer_id, &region.target, None).into_parts();
    if messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        MutationDiff::<RasterSnapshot>::retire_cold(diff);
        return Err(protocol::MutationApplyError::new("mutation.apply.capacity", "The rewritten image cannot be filed in the asset pool."));
    }
    let applied = MutationDiff::apply(&diff, snapshot, capability);
    MutationDiff::<RasterSnapshot>::retire_cold(diff);
    applied
}
//#endregion 🔖️PixelRegions

//#region 🔖️Algebra
struct IdNode {
    id: String,
    children: Vec<IdNode>,
}

fn id_forest(layers: &[RasterLayerNode]) -> Vec<IdNode> {
    layers.iter().map(|layer| IdNode { id: layer_node_id(layer).to_string(), children: if let RasterLayerNode::Group { children, .. } = layer { id_forest(children) } else { Vec::new() } }).collect()
}

fn id_address(forest: &[IdNode], target: &str, parent: Option<&str>) -> Option<(Option<String>, usize)> {
    for (index, node) in forest.iter().enumerate() {
        if node.id == target {
            return Some((parent.map(str::to_string), index));
        }
        if let Some(found) = id_address(&node.children, target, Some(&node.id)) {
            return Some(found);
        }
    }
    None
}



fn id_ancestors(forest: &[IdNode], target: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut cursor = target.to_string();
    while let Some((Some(parent), _)) = id_address(forest, &cursor, None) {
        chain.push(parent.clone());
        cursor = parent;
    }
    chain
}

/// 🔁️ The patch that restores exactly the fields `patch` writes, read from the BASE `node` without mutating it — the same
/// swap `apply_layer_patch` performs, minus the write.
fn inverse_layer_patch(node: &RasterLayerNode, patch: &RasterLayerPatch) -> RasterLayerPatch {
    let mut inverse = RasterLayerPatch::default();
    match node {
        RasterLayerNode::Pixel { name, visible, locked, opacity, blend_mode, transform, mask, width, height, image_key, .. } => {
            if patch.mask_content.is_some() {
                inverse.mask_content = Some(crate::RasterMaskContent { mask: mask.clone() });
            }
            if patch.pixel_content.is_some() {
                inverse.pixel_content = Some(crate::RasterPixelContent { image_key: image_key.clone(), width: *width, height: *height });
            }
            if patch.transform.is_some() {
                inverse.transform = Some(transform.clone());
            }
            inverse.name = patch.name.as_ref().map(|_| name.clone());
            inverse.visible = patch.visible.map(|_| *visible);
            inverse.locked = patch.locked.map(|_| *locked);
            inverse.opacity = patch.opacity.map(|_| *opacity);
            inverse.blend_mode = patch.blend_mode.as_ref().map(|_| blend_mode.clone());
            inverse.transform_x = patch.transform_x.map(|_| transform.x);
            inverse.transform_y = patch.transform_y.map(|_| transform.y);
            inverse.width = patch.width.map(|_| width.unwrap_or(512));
            inverse.height = patch.height.map(|_| height.unwrap_or(512));
        }
        RasterLayerNode::Group { name, visible, locked, opacity, blend_mode, transform, mask, .. } => {
            if patch.mask_content.is_some() {
                inverse.mask_content = Some(crate::RasterMaskContent { mask: mask.clone() });
            }
            if patch.transform.is_some() {
                inverse.transform = Some(transform.clone());
            }
            inverse.name = patch.name.as_ref().map(|_| name.clone());
            inverse.visible = patch.visible.map(|_| *visible);
            inverse.locked = patch.locked.map(|_| *locked);
            inverse.opacity = patch.opacity.map(|_| *opacity);
            inverse.blend_mode = patch.blend_mode.as_ref().map(|_| blend_mode.clone());
            inverse.transform_x = patch.transform_x.map(|_| transform.x);
            inverse.transform_y = patch.transform_y.map(|_| transform.y);
        }
        RasterLayerNode::Adjustment { name, visible, locked, opacity, blend_mode, adjustment_kind, params, .. } => {
            if let Some(parameters) = &patch.adjustment_parameters {
                inverse.adjustment_parameters = Some(parameters.iter().map(|row| crate::RasterAdjustmentParameter { parameter: row.parameter.clone(), value: params.get(&row.parameter).and_then(crate::RasterAdjustmentNumber::from_parameter) }).collect());
            }
            inverse.name = patch.name.as_ref().map(|_| name.clone());
            inverse.visible = patch.visible.map(|_| *visible);
            inverse.locked = patch.locked.map(|_| *locked);
            inverse.opacity = patch.opacity.map(|_| *opacity);
            inverse.blend_mode = patch.blend_mode.as_ref().map(|_| blend_mode.clone());
            inverse.adjustment_kind = patch.adjustment_kind.as_ref().map(|_| adjustment_kind.clone());
        }
    }
    inverse
}

fn subtree_ids_of(layer: &RasterLayerNode) -> std::collections::BTreeSet<String> {
    let nested: std::collections::BTreeSet<String> = match layer {
        RasterLayerNode::Group { children, .. } => children.iter().flat_map(subtree_ids_of).collect(),
        _ => std::collections::BTreeSet::new(),
    };
    nested.into_iter().chain(std::iter::once(layer_node_id(layer).to_string())).collect()
}

/// 🔁️ The negative layers delta, read row by row from the BASE tree: an inserted subtree is removed at its after address, a
/// removed subtree comes back at its base address (cloned from the base), a move runs backwards, patches restore the base fields.
/// Nothing is applied or simulated.
fn inverse_layers(delta: &RasterLayersDelta, base: &[RasterLayerNode]) -> RasterLayersDelta {
    let forest = id_forest(base);
    let born: std::collections::BTreeSet<String> = delta.inserted.iter().flat_map(|insertion| subtree_ids_of(&insertion.layer)).collect();
    let removed = delta
        .inserted
        .iter()
        .filter(|insertion| insertion.parent_id.as_ref().is_none_or(|parent| !born.contains(parent)))
        .map(|insertion| RasterLayerRemoval { id: layer_node_id(&insertion.layer).to_string(), parent_id: insertion.parent_id.clone(), index: insertion.index })
        .collect();
    let inserted = delta
        .removed
        .iter()
        .filter(|row| !id_ancestors(&forest, &row.id).iter().any(|ancestor| delta.removed.iter().any(|other| &other.id == ancestor)))
        .filter_map(|row| container_of(base, row.parent_id.as_deref()).and_then(|children| children.get(row.index)).map(|node| RasterLayerInsertion { parent_id: row.parent_id.clone(), index: row.index, layer: node.clone() }))
        .collect();
    let moved = delta.moved.iter().map(|row| RasterLayerRelocation { id: row.id.clone(), from: row.to.clone(), to: row.from.clone() }).collect();
    let modified = delta.modified.iter().filter_map(|entry| find_layer(base, &entry.id).map(|node| RasterLayerModification { id: entry.id.clone(), patch: inverse_layer_patch(node, &entry.patch) })).collect();
    RasterLayersDelta { removed, inserted, moved, modified }
}

impl RasterLayersDelta {
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.inserted.is_empty() && self.removed.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| entry.patch == RasterLayerPatch::default())
    }
}

impl protocol::DiffAlgebra<RasterSnapshot> for RasterDiff {
    fn inverse(&self, base: &RasterSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            title: self.title.as_ref().map(|_| base.title.clone()),
            layers: self.layers.as_ref().map(|delta| inverse_layers(delta, &base.layers)),
            assets: self.assets.as_ref().map(|delta| RasterAssetsDelta { entries: delta.entries.iter().filter_map(|(key, value)| match (value, base.assets.contains_key(key)) {
                (Some(_), false) => Some((key.clone(), None)),
                (_, true) => crate::raster_image(&base.assets, key).map(|asset| (key.clone(), Some(asset))),
                (None, false) => None,
            }).collect() }),
            pixels: self.pixels.iter().rev().filter_map(|region| crate::mutations::paint_stroke::inverse_pixel_region(region, base)).collect(),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.id.is_none() && self.title.is_none() && self.layers.as_ref().is_none_or(RasterLayersDelta::is_empty) && self.assets.as_ref().is_none_or(|assets| assets.entries.is_empty()) && self.pixels.is_empty()
    }
}
//#endregion 🔖️Algebra

/// 🩹 Field-wise coalesce of two patches of the SAME layer — the later `Some` wins, an unwritten
/// field keeps what the earlier patch wrote. Mirrors the whole-diff `take!` macro one level down.
fn absorb_layer_patch(dst: &mut RasterLayerPatch, src: RasterLayerPatch) {
    macro_rules! take {
        ($field:ident) => {
            if src.$field.is_some() {
                dst.$field = src.$field;
            }
        };
    }
    take!(name);
    take!(visible);
    take!(locked);
    take!(opacity);
    take!(blend_mode);
    if let Some(mut transform)=src.transform {
        if let Some(x)=src.transform_x {transform.x=x;}
        if let Some(y)=src.transform_y {transform.y=y;}
        dst.transform=Some(transform);dst.transform_x=None;dst.transform_y=None;
    }else if let Some(transform)=dst.transform.as_mut(){
        if let Some(x)=src.transform_x {transform.x=x;}
        if let Some(y)=src.transform_y {transform.y=y;}
    }else{take!(transform_x);take!(transform_y);}
    take!(width);
    take!(height);
    if let Some(parameters)=src.adjustment_parameters {
        let target=dst.adjustment_parameters.get_or_insert_with(Vec::new);
        for parameter in parameters {
            if let Some(prior)=target.iter_mut().find(|prior|prior.parameter==parameter.parameter) {*prior=parameter;} else {target.push(parameter);}
        }
    }
    take!(adjustment_kind);
    take!(mask_content);
    take!(pixel_content);
}

/// 🔑️ One row of a per-container list during `absorb`: the id of an inserted subtree (`fresh`), or the arrival of a layer that moved in.
#[derive(Clone, Debug, PartialEq)]
struct Slot {
    id: String,
    fresh: bool,
}

impl protocol::list_delta::Keyed for Slot {
    type Key = String;
    fn key(&self) -> String {
        self.id.clone()
    }
}

type ContainerParts = protocol::list_delta::Parts<Slot, protocol::list_delta::NoPatch>;

/// 🧱️ The per-container lists of one delta: a leaving row (removed or moved out) is a `removed` entry of its container, an
/// entering row (inserted or moved in) an `inserted` entry of its container.
fn container_parts(removed: &[RasterLayerRemoval], moved: &[RasterLayerRelocation], inserted: Vec<(Option<String>, usize, String)>) -> std::collections::BTreeMap<Option<String>, ContainerParts> {
    let mut parts: std::collections::BTreeMap<Option<String>, ContainerParts> = std::collections::BTreeMap::new();
    for row in removed {
        parts.entry(row.parent_id.clone()).or_default().removed.push((row.id.clone(), row.index));
    }
    for row in moved {
        parts.entry(row.from.parent_id.clone()).or_default().removed.push((row.id.clone(), row.from.index));
        parts.entry(row.to.parent_id.clone()).or_default().inserted.push((row.to.index, Slot { id: row.id.clone(), fresh: false }));
    }
    for (parent_id, index, id) in inserted {
        parts.entry(parent_id).or_default().inserted.push((index, Slot { id, fresh: true }));
    }
    parts
}

/// 🔎️ The child list of the group `id` inside a set of inserted subtrees.
fn born_children_mut<'a>(rows: &'a mut [RasterLayerInsertion], id: &str) -> Option<&'a mut Vec<RasterLayerNode>> {
    for row in rows.iter_mut() {
        if let RasterLayerNode::Group { id: group_id, children, .. } = &mut row.layer {
            if group_id.as_str() == id {
                return Some(children);
            }
            if let Some(found) = container_mut(children, Some(id)) {
                return Some(found);
            }
        }
    }
    None
}

/// 🧩 Folds the edits of `later` that reach INTO subtrees this delta inserts straight into those subtrees: a mid-state container
/// that does not exist in the base cannot carry a base coordinate. The leaving rows are lifted out of their containers first (every
/// index is a mid coordinate), then the entering rows take their after slots. A layer that moves from the base INTO a subtree
/// inserted here cannot be expressed and stays a move row, which then refuses at apply.
fn fold_into_born(born: &mut Vec<RasterLayerInsertion>, later: &mut RasterLayersDelta) {
    let territory: std::collections::BTreeSet<String> = born.iter().flat_map(|row| subtree_ids_of(&row.layer)).collect();
    let inside = |parent: &Option<String>| parent.as_deref().is_some_and(|parent| territory.contains(parent));
    let mut leaving: std::collections::BTreeMap<String, Vec<(usize, String, bool)>> = std::collections::BTreeMap::new();
    for row in std::mem::take(&mut later.removed) {
        match &row.parent_id {
            parent if inside(parent) => leaving.entry(row.parent_id.clone().unwrap_or_default()).or_default().push((row.index, row.id, true)),
            _ => later.removed.push(row),
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
    let mut lifted: std::collections::BTreeMap<String, RasterLayerNode> = std::collections::BTreeMap::new();
    for (container, mut rows) in leaving {
        rows.sort_by(|left, right| right.0.cmp(&left.0));
        let Some(children) = born_children_mut(born, &container) else { continue };
        for (index, id, removed) in rows {
            if children.get(index).is_some_and(|node| layer_node_id(node) == id) {
                let node = children.remove(index);
                if removed {
                    crate::retire_raster_layer(node);
                } else {
                    lifted.insert(id, node);
                }
            }
        }
    }
    let mut entering: std::collections::BTreeMap<String, Vec<(usize, RasterLayerNode)>> = std::collections::BTreeMap::new();
    for row in leaving_moves {
        match lifted.remove(&row.id) {
            Some(node) if inside(&row.to.parent_id) => entering.entry(row.to.parent_id.clone().unwrap_or_default()).or_default().push((row.to.index, node)),
            Some(node) => later.inserted.push(RasterLayerInsertion { parent_id: row.to.parent_id, index: row.to.index, layer: node }),
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
        match born_children_mut(born, &container) {
            Some(children) if check_slots(children.len(), &rows).is_ok() => place_entering(children, rows),
            _ => crate::retire_raster_layers(rows.into_iter().map(|(_, node)| node).collect()),
        }
    }
    crate::retire_raster_layers(lifted.into_values().collect());
}

/// 🧹 Drops the move rows that leave their layer exactly where the surviving siblings already put it: a move inside one container
/// whose base index minus the other leaving rows before it equals its after index minus the other entering rows before it.
fn without_identity_moves(moved: Vec<RasterLayerRelocation>, removed: &[RasterLayerRemoval], inserted: &[RasterLayerInsertion]) -> Vec<RasterLayerRelocation> {
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

/// ➕️ Composes `dst` (base→mid) with `src` (mid→after) in place. Per container the framework's positional algebra coalesces the rows
/// (insert∘remove cancels, insert∘move lands at its final slot, move∘move is one move, move∘remove removes at the base address,
/// remove∘insert of one id is a replacement); a layer that left one container and entered another stays one move row. An edit that
/// lands on a layer THIS delta inserts folds into the inserted subtree itself.
fn absorb_layers_delta(dst: &mut RasterLayersDelta, mut src: RasterLayersDelta) {
    let mut born_rows = std::mem::take(&mut dst.inserted);
    fold_into_born(&mut born_rows, &mut src);
    let inserted_of = |rows: &[RasterLayerInsertion]| rows.iter().map(|row| (row.parent_id.clone(), row.index, layer_node_id(&row.layer).to_string())).collect::<Vec<_>>();
    let mut left = container_parts(&dst.removed, &dst.moved, inserted_of(&born_rows));
    let mut right = container_parts(&src.removed, &src.moved, inserted_of(&src.inserted));
    let mut nodes: std::collections::BTreeMap<String, RasterLayerNode> = std::collections::BTreeMap::new();
    for row in born_rows.into_iter().chain(std::mem::take(&mut src.inserted)) {
        if let Some(displaced) = nodes.insert(layer_node_id(&row.layer).to_string(), row.layer) {
            crate::retire_raster_layer(displaced);
        }
    }
    let born: std::collections::BTreeSet<String> = nodes.values().flat_map(subtree_ids_of).collect();
    let containers: std::collections::BTreeSet<Option<String>> = left.keys().chain(right.keys()).cloned().collect();
    let mut leaving: Vec<(Option<String>, String, usize)> = Vec::new();
    let mut entering: Vec<(Option<String>, usize, Slot)> = Vec::new();
    for container in containers {
        let mut parts = left.remove(&container).unwrap_or_default();
        parts.absorb(right.remove(&container).unwrap_or_default());
        leaving.extend(parts.removed.into_iter().map(|(id, index)| (container.clone(), id, index)));
        entering.extend(parts.inserted.into_iter().map(|(index, slot)| (container.clone(), index, slot)));
    }
    let mut inserted = Vec::new();
    let mut moved = Vec::new();
    for (container, index, slot) in entering {
        if !slot.fresh {
            if let Some(at) = leaving.iter().position(|(_, id, _)| *id == slot.id) {
                let (from_parent, id, from_index) = leaving.remove(at);
                moved.push(RasterLayerRelocation { id, from: RasterLayerAddress { parent_id: from_parent, index: from_index }, to: RasterLayerAddress { parent_id: container, index } });
                continue;
            }
        }
        if let Some(layer) = nodes.remove(&slot.id) {
            inserted.push(RasterLayerInsertion { parent_id: container, index, layer });
        }
    }
    crate::retire_raster_layers(nodes.into_values().collect());
    let removed: Vec<RasterLayerRemoval> = leaving.into_iter().map(|(parent_id, id, index)| RasterLayerRemoval { id, parent_id, index }).collect();
    let moved = without_identity_moves(moved, &removed, &inserted);
    let gone: std::collections::BTreeSet<&str> = removed.iter().map(|row| row.id.as_str()).filter(|id| !inserted.iter().any(|row| layer_node_id(&row.layer) == *id)).collect();
    let mut modified = std::mem::take(&mut dst.modified);
    modified.retain(|entry| !gone.contains(entry.id.as_str()));
    for entry in src.modified {
        if gone.contains(entry.id.as_str()) {
            continue;
        }
        if born.contains(&entry.id) {
            inserted.iter_mut().any(|row| patch_layer_in_tree(std::slice::from_mut(&mut row.layer), &entry.id, &entry.patch).is_some());
            continue;
        }
        match modified.iter_mut().find(|existing| existing.id == entry.id) {
            Some(existing) => absorb_layer_patch(&mut existing.patch, entry.patch),
            None => modified.push(entry),
        }
    }
    modified.sort_by(|left, right| left.id.cmp(&right.id));
    *dst = RasterLayersDelta { removed, inserted, moved, modified };
}

/// ➕ Sparse insertion diff — tree-aware (`parent_id: None` = document root), so `create-layer` never
/// needs to fall back to whole-snapshot capture even when inserting into a nested `Group`.
pub fn diff_add_layer(parent_id: Option<String>, index: usize, layer: RasterLayerNode) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { inserted: vec![RasterLayerInsertion { parent_id, index, layer }], ..Default::default() }), ..Default::default() }
}

/// ➖️ Sparse removal diff: one removal row at the layer's base address. Empty when the layer is not in `base`.
pub fn diff_remove_layer(base: &[RasterLayerNode], layer_id: &str) -> RasterDiff {
    let Some((parent_id, index)) = crate::standards::v1::subsets::any::schema::locate_layer(base, layer_id) else { return RasterDiff::default() };
    RasterDiff { layers: Some(RasterLayersDelta { removed: vec![RasterLayerRemoval { id: layer_id.to_string(), parent_id, index }], ..Default::default() }), ..Default::default() }
}

pub fn diff_patch_layer(layer_id: &str, patch: RasterLayerPatch) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { modified: vec![RasterLayerModification { id: layer_id.to_string(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔀 Sparse reposition diff (`reorder-layers`): one tree-aware move row from the layer's base address to the after address
/// `(parent_id, index)` — never the layer itself. The index saturates at the end of the destination list. Empty when the layer is
/// not in `base`.
pub fn diff_move_layer(base: &[RasterLayerNode], layer_id: &str, parent_id: Option<String>, index: usize) -> RasterDiff {
    let Some((from_parent, from_index)) = crate::standards::v1::subsets::any::schema::locate_layer(base, layer_id) else { return RasterDiff::default() };
    let length = container_of(base, parent_id.as_deref()).map_or(0, <[RasterLayerNode]>::len);
    let room = if parent_id == from_parent { length.saturating_sub(1) } else { length };
    RasterDiff {
        layers: Some(RasterLayersDelta {
            moved: vec![RasterLayerRelocation { id: layer_id.to_string(), from: RasterLayerAddress { parent_id: from_parent, index: from_index }, to: RasterLayerAddress { parent_id, index: index.min(room) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// 🖇️ Sparse asset-map insertion diff (`add-layer-asset`).
pub fn diff_add_asset(asset_id: &str, asset: crate::SemioImageSnapshot) -> RasterDiff {
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

#[cfg(test)]
#[path = "🧪️tests/🔬️asset-capacity-vectors/🦀️.rs"]
mod asset_capacity_vectors;

#[cfg(test)]
#[path = "🧪️tests/🔬️layer-tree-delta/🦀️.rs"]
mod layer_tree_delta;
