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
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterAssetsDelta {
    pub entries: BTreeMap<String, Option<SemioImageSnapshot>>,
}

/// 🧩 Identified-collection delta for `layers` — every entry is tree-aware (`parent_id: None` means
/// the document root) so `create-layer`/`reorder-layers` never fall back to whole-snapshot capture,
/// even when the target lives inside a nested `Group`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterLayersDelta {
    pub added: Vec<RasterLayerInsertion>,
    pub removed: Vec<String>,
    pub patched: Vec<RasterLayerPatchEntry>,
    pub moved: Vec<RasterLayerMove>,
}

/// ➕ One inserted layer (`create-layer`) — carries its own tree address so insertion into a nested
/// `Group` is expressible sparsely.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerInsertion {
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: RasterLayerNode,
}

/// 🔀 One repositioned layer (`reorder-layers`) — remove-then-insert at a tree address, never a
/// flat top-level-only reorder.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerMove {
    pub id: String,
    pub parent_id: Option<String>,
    pub index: usize,
}

/// 🩹 One patched layer entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerPatchEntry {
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

/// 🔀 The MOVE phase's insert: the node was just lifted out of the tree, so the valid positions are
/// `0..=len` of the container it lands in, and a request past the end means "last". A move's index
/// was validated against the tree ITS diff was built from; once `absorb` coalesces that move with a
/// later removal of a sibling, the same relative position is simply one slot shorter, and refusing it
/// as `mutation.apply.invalid-index` broke `absorb(d1, d2).apply(base) == d2.apply(d1.apply(base))`
/// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). An UNKNOWN parent is still refused — only
/// the index saturates, never the address.
pub fn reposition_layer(layers: &mut Vec<RasterLayerNode>, parent_id: Option<&str>, index: usize, layer: RasterLayerNode) -> bool {
    match parent_id {
        None => {
            let position = index.min(layers.len());
            layers.insert(position, layer);
            true
        }
        Some(parent_id) => {
            for node in layers.iter_mut() {
                if let RasterLayerNode::Group { id, children, .. } = node {
                    if id == parent_id {
                        let position = index.min(children.len());
                        children.insert(position, layer);
                        return true;
                    }
                    if reposition_layer(children, Some(parent_id), index, layer.clone()) {
                        return true;
                    }
                }
            }
            false
        }
    }
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
    let mut identities: std::collections::BTreeSet<String> = crate::standards::v1::subsets::any::schema::flatten_raster_layers(layers).into_iter().map(|node| layer_node_id(node).to_string()).collect();
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
        let removed = remove_layer_from_tree(&mut next, id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "removed layer does not exist after structural edits").at(["removed", id.as_str()]))?;
        crate::retire_raster_layer(removed);
    }
    for (index, entry) in delta.patched.iter().enumerate() {
        apply_layer_patch_entry(&mut next, entry).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    for (index, mv) in delta.moved.iter().enumerate() {
        let node = remove_layer_from_tree(&mut next, &mv.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "moved layer does not exist after structural edits").at(["moved".to_string(), index.to_string()]))?;
        if !reposition_layer(&mut next, mv.parent_id.as_deref(), mv.index, node) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "moved layer parent or index is invalid").at(["moved".to_string(), index.to_string()]));
        }
    }
    for (index, insertion) in delta.added.iter().enumerate() {
        if !insert_layer(&mut next, insertion.parent_id.as_deref(), insertion.index, insertion.layer.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "added layer parent or index is invalid").at(["added".to_string(), index.to_string()]));
        }
    }
    let next_ids: Vec<_> = crate::standards::v1::subsets::any::schema::flatten_raster_layers(&next).into_iter().map(layer_node_id).collect();
    if next_ids.iter().enumerate().any(|(index, id)| next_ids[..index].contains(id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "resulting layer tree contains duplicate identities").at(["identities"]));
    }
    Ok(next)
}

fn apply_layer_patch_entry(layers: &mut [RasterLayerNode], entry: &RasterLayerPatchEntry) -> protocol::MutationApplyResult<()> {
    patch_layer_in_tree(layers, &entry.id, &entry.patch).map(|_| ()).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched layer does not exist"))
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
    fn apply(&self, snapshot: &RasterSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RasterSnapshot> {
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
    }

    fn retire_cold(self) {
        let RasterDiff { schema: _, id: _, title: _, layers, assets: _ } = self;
        if let Some(layers) = layers {
            for insertion in layers.added {
                crate::retire_raster_layer(insertion.layer);
            }
        }
    }

    fn retire_projection(projection: RasterSnapshot) {
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(projection);
    }
}

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

fn id_take(forest: &mut Vec<IdNode>, target: &str) -> Option<IdNode> {
    if let Some(index) = forest.iter().position(|node| node.id == target) {
        return Some(forest.remove(index));
    }
    forest.iter_mut().find_map(|node| id_take(&mut node.children, target))
}

fn id_put(forest: &mut Vec<IdNode>, parent: Option<&str>, index: usize, node: IdNode) {
    match parent {
        None => forest.insert(index.min(forest.len()), node),
        Some(parent) => {
            fn find<'a>(forest: &'a mut Vec<IdNode>, parent: &str) -> Option<&'a mut Vec<IdNode>> {
                for candidate in forest.iter_mut() {
                    if candidate.id == parent {
                        return Some(&mut candidate.children);
                    }
                    if let Some(found) = find(&mut candidate.children, parent) {
                        return Some(found);
                    }
                }
                None
            }
            if let Some(children) = find(forest, parent) {
                children.insert(index.min(children.len()), node);
            }
        }
    }
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

fn subtree_ids(layer: &RasterLayerNode, into: &mut std::collections::BTreeSet<String>) {
    into.insert(layer_node_id(layer).to_string());
    if let RasterLayerNode::Group { children, .. } = layer {
        for child in children {
            subtree_ids(child, into);
        }
    }
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

fn inverse_layers(delta: &RasterLayersDelta, base: &[RasterLayerNode]) -> RasterLayersDelta {
    let mut forest = id_forest(base);
    let mut restored: Vec<(Option<String>, usize, String)> = Vec::new();
    for id in &delta.removed {
        let Some((parent, index)) = id_address(&forest, id, None) else { continue };
        if id_ancestors(&forest, id).iter().any(|ancestor| delta.removed.contains(ancestor)) {
            continue;
        }
        restored.push((parent, index, id.clone()));
    }
    restored.sort_by(|left, right| (&left.0, left.1).cmp(&(&right.0, right.1)));
    let added = restored.into_iter().filter_map(|(parent_id, index, id)| find_layer(base, &id).map(|layer| RasterLayerInsertion { parent_id, index, layer: layer.clone() })).collect();
    for id in &delta.removed {
        id_take(&mut forest, id);
    }
    let mut undo = Vec::new();
    for movement in &delta.moved {
        let Some((parent_id, index)) = id_address(&forest, &movement.id, None) else { continue };
        undo.push(RasterLayerMove { id: movement.id.clone(), parent_id, index });
        if let Some(node) = id_take(&mut forest, &movement.id) {
            id_put(&mut forest, movement.parent_id.as_deref(), movement.index, node);
        }
    }
    undo.reverse();
    let mut inserted = std::collections::BTreeSet::new();
    for insertion in &delta.added {
        subtree_ids(&insertion.layer, &mut inserted);
    }
    let removed = delta.added.iter().filter(|insertion| insertion.parent_id.as_ref().is_none_or(|parent| !inserted.contains(parent))).map(|insertion| layer_node_id(&insertion.layer).to_string()).collect();
    let patched = delta.patched.iter().filter_map(|entry| find_layer(base, &entry.id).map(|node| RasterLayerPatchEntry { id: entry.id.clone(), patch: inverse_layer_patch(node, &entry.patch) })).collect();
    RasterLayersDelta { added, removed, patched, moved: undo }
}

fn layer_patch_between(base: &RasterLayerNode, other: &RasterLayerNode) -> Option<RasterLayerPatch> {
    let mut patch = RasterLayerPatch::default();
    match (base, other) {
        (
            RasterLayerNode::Pixel { name: base_name, visible: base_visible, locked: base_locked, opacity: base_opacity, blend_mode: base_blend, transform: base_transform, mask: base_mask, width: base_width, height: base_height, image_key: base_image, .. },
            RasterLayerNode::Pixel { name, visible, locked, opacity, blend_mode, transform, mask, width, height, image_key, .. },
        ) => {
            patch.name = (base_name != name).then(|| name.clone());
            patch.visible = (base_visible != visible).then_some(*visible);
            patch.locked = (base_locked != locked).then_some(*locked);
            patch.opacity = (base_opacity != opacity).then_some(*opacity);
            patch.blend_mode = (base_blend != blend_mode).then(|| blend_mode.clone());
            patch.transform = (base_transform != transform).then(|| transform.clone());
            patch.mask_content = (base_mask != mask).then(|| crate::RasterMaskContent { mask: mask.clone() });
            patch.pixel_content = (base_width != width || base_height != height || base_image != image_key).then(|| crate::RasterPixelContent { image_key: image_key.clone(), width: *width, height: *height });
        }
        (
            RasterLayerNode::Group { name: base_name, visible: base_visible, locked: base_locked, opacity: base_opacity, blend_mode: base_blend, transform: base_transform, mask: base_mask, .. },
            RasterLayerNode::Group { name, visible, locked, opacity, blend_mode, transform, mask, .. },
        ) => {
            patch.name = (base_name != name).then(|| name.clone());
            patch.visible = (base_visible != visible).then_some(*visible);
            patch.locked = (base_locked != locked).then_some(*locked);
            patch.opacity = (base_opacity != opacity).then_some(*opacity);
            patch.blend_mode = (base_blend != blend_mode).then(|| blend_mode.clone());
            patch.transform = (base_transform != transform).then(|| transform.clone());
            patch.mask_content = (base_mask != mask).then(|| crate::RasterMaskContent { mask: mask.clone() });
        }
        (
            RasterLayerNode::Adjustment { name: base_name, visible: base_visible, locked: base_locked, opacity: base_opacity, blend_mode: base_blend, transform: base_transform, adjustment_kind: base_kind, params: base_params, .. },
            RasterLayerNode::Adjustment { name, visible, locked, opacity, blend_mode, transform, adjustment_kind, params, .. },
        ) => {
            if base_transform != transform || base_params != params {
                return None;
            }
            patch.name = (base_name != name).then(|| name.clone());
            patch.visible = (base_visible != visible).then_some(*visible);
            patch.locked = (base_locked != locked).then_some(*locked);
            patch.opacity = (base_opacity != opacity).then_some(*opacity);
            patch.blend_mode = (base_blend != blend_mode).then(|| blend_mode.clone());
            patch.adjustment_kind = (base_kind != adjustment_kind).then(|| adjustment_kind.clone());
        }
        _ => return None,
    }
    Some(patch)
}

fn layers_between(base: &[RasterLayerNode], other: &[RasterLayerNode]) -> RasterLayersDelta {
    let (base_flat, other_flat) = (flatten_raster_layers(base), flatten_raster_layers(other));
    let base_forest = id_forest(base);
    let other_forest = id_forest(other);
    let base_ids: std::collections::BTreeSet<&str> = base_flat.iter().map(|node| layer_node_id(node)).collect();
    let other_ids: std::collections::BTreeSet<&str> = other_flat.iter().map(|node| layer_node_id(node)).collect();
    let coarse = || RasterLayersDelta {
        removed: base.iter().map(|layer| layer_node_id(layer).to_string()).collect(),
        added: other.iter().enumerate().map(|(index, layer)| RasterLayerInsertion { parent_id: None, index, layer: layer.clone() }).collect(),
        ..Default::default()
    };
    let removed_roots: Vec<&str> = base_ids.iter().copied().filter(|id| !other_ids.contains(id) && !id_ancestors(&base_forest, id).iter().any(|ancestor| !other_ids.contains(ancestor.as_str()))).collect();
    let added_roots: Vec<&str> = other_ids.iter().copied().filter(|id| !base_ids.contains(id) && !id_ancestors(&other_forest, id).iter().any(|ancestor| !base_ids.contains(ancestor.as_str()))).collect();
    let mut delta = RasterLayersDelta::default();
    for node in &other_flat {
        let id = layer_node_id(node);
        let Some(source) = find_layer(base, id) else { continue };
        if id_ancestors(&base_forest, id).iter().any(|ancestor| !other_ids.contains(ancestor.as_str())) || id_ancestors(&other_forest, id).iter().any(|ancestor| !base_ids.contains(ancestor.as_str())) {
            return coarse();
        }
        let Some(patch) = layer_patch_between(source, node) else { return coarse() };
        if patch != RasterLayerPatch::default() {
            delta.patched.push(RasterLayerPatchEntry { id: id.to_string(), patch });
        }
    }
    delta.removed = removed_roots.iter().map(|id| id.to_string()).collect();
    let mut working = id_forest(base);
    for id in &delta.removed {
        id_take(&mut working, id);
    }
    fn containers<'a>(forest: &'a [IdNode], parent: Option<&'a str>, into: &mut Vec<(Option<&'a str>, &'a [IdNode])>) {
        into.push((parent, forest));
        for node in forest {
            containers(&node.children, Some(&node.id), into);
        }
    }
    let mut targets = Vec::new();
    containers(&other_forest, None, &mut targets);
    for (parent, wanted) in targets {
        let persistent: Vec<&str> = wanted.iter().map(|node| node.id.as_str()).filter(|id| base_ids.contains(id)).collect();
        for (position, id) in persistent.iter().enumerate() {
            let current = id_address(&working, id, None);
            if current.as_ref().map(|(p, i)| (p.as_deref(), *i)) == Some((parent, position)) {
                continue;
            }
            delta.moved.push(RasterLayerMove { id: id.to_string(), parent_id: parent.map(str::to_string), index: position });
            if let Some(node) = id_take(&mut working, id) {
                id_put(&mut working, parent, position, node);
            }
        }
    }
    for id in &added_roots {
        let Some((parent_id, index)) = id_address(&other_forest, id, None) else { continue };
        if let Some(layer) = find_layer(other, id) {
            delta.added.push(RasterLayerInsertion { parent_id, index, layer: layer.clone() });
        }
    }
    delta.added.sort_by(|left, right| (&left.parent_id, left.index).cmp(&(&right.parent_id, right.index)));
    delta
}

impl RasterLayersDelta {
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.moved.is_empty() && self.patched.iter().all(|entry| entry.patch == RasterLayerPatch::default())
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
        }
    }

    fn between(base: &RasterSnapshot, other: &RasterSnapshot) -> Self {
        let layers = layers_between(&base.layers, &other.layers);
        let mut entries = BTreeMap::new();
        for key in other.assets.keys() {
            let (before, after) = (crate::raster_image(&base.assets, key), crate::raster_image(&other.assets, key));
            if !base.assets.contains_key(key) || before != after {
                entries.insert(key.to_string(), after);
            }
        }
        for key in base.assets.keys() {
            if !other.assets.contains_key(key) {
                entries.insert(key.to_string(), None);
            }
        }
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            id: (base.id != other.id).then(|| other.id.clone()),
            title: (base.title != other.title).then(|| other.title.clone()),
            layers: (!layers.is_empty()).then_some(layers),
            assets: (!entries.is_empty()).then_some(RasterAssetsDelta { entries }),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.id.is_none() && self.title.is_none() && self.layers.as_ref().is_none_or(RasterLayersDelta::is_empty) && self.assets.as_ref().is_none_or(|assets| assets.entries.is_empty())
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

/// 🧩️ Sequential coalesce of two layer deltas. `apply` runs the phases `removed → patched → moved →
/// added` ONCE, so a naive concatenation produces a delta that applies differently from the two
/// deltas in sequence: two patches of one layer became two `patched` entries (refused as
/// `mutation.apply.duplicate-target`) and two moves of one layer became two `moved` entries. Each
/// identity therefore carries at most one entry per phase here, and an edit that lands on a layer
/// THIS delta inserts folds into the insertion itself — the insertion happens after the patch/move
/// phases, so a separate entry could never find its target.
fn absorb_layers_delta(dst: &mut RasterLayersDelta, src: RasterLayersDelta) {
    for id in src.removed {
        if let Some(position) = dst.added.iter().position(|insertion| layer_node_id(&insertion.layer) == id) {
            // 🫧 Inserted by this delta and removed by the next: the layer never reaches the document.
            let insertion = dst.added.remove(position);
            crate::retire_raster_layer(insertion.layer);
        } else if !dst.removed.contains(&id) {
            dst.removed.push(id.clone());
        }
        dst.patched.retain(|entry| entry.id != id);
        dst.moved.retain(|entry| entry.id != id);
    }
    for entry in src.patched {
        if let Some(insertion) = dst.added.iter_mut().find(|insertion| layer_node_id(&insertion.layer) == entry.id) {
            patch_layer_in_tree(std::slice::from_mut(&mut insertion.layer), &entry.id, &entry.patch);
            continue;
        }
        match dst.patched.iter_mut().find(|existing| existing.id == entry.id) {
            Some(existing) => absorb_layer_patch(&mut existing.patch, entry.patch),
            None => dst.patched.push(entry),
        }
    }
    for moved in src.moved {
        if let Some(insertion) = dst.added.iter_mut().find(|insertion| layer_node_id(&insertion.layer) == moved.id) {
            insertion.parent_id = moved.parent_id;
            insertion.index = moved.index;
            continue;
        }
        match dst.moved.iter_mut().find(|existing| existing.id == moved.id) {
            Some(existing) => *existing = moved,
            None => dst.moved.push(moved),
        }
    }
    for insertion in src.added {
        let id = layer_node_id(&insertion.layer).to_string();
        match dst.added.iter().position(|existing| layer_node_id(&existing.layer) == id) {
            Some(position) => {
                let displaced = std::mem::replace(&mut dst.added[position], insertion);
                crate::retire_raster_layer(displaced.layer);
            }
            None => dst.added.push(insertion),
        }
    }
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
