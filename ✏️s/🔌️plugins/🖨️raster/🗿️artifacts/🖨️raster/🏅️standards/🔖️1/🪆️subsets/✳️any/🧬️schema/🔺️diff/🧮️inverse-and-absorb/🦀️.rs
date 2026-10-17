//! 🧮️ The delta algebra: exact inverses of layer deltas and absorption of a delta into an earlier one.

use super::*;

//#region 🔖️Algebra
struct IdNode {
    pub(super) id: String,
    pub(super) children: Vec<IdNode>,
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
                inverse.mask_content = Some(semio_s_artifact_raster_raster_model::RasterMaskContent { mask: mask.clone() });
            }
            if patch.pixel_content.is_some() {
                inverse.pixel_content = Some(semio_s_artifact_raster_raster_model::RasterPixelContent { image_key: image_key.clone(), width: *width, height: *height });
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
                inverse.mask_content = Some(semio_s_artifact_raster_raster_model::RasterMaskContent { mask: mask.clone() });
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
                inverse.adjustment_parameters = Some(parameters.iter().map(|row| semio_s_artifact_raster_raster_model::RasterAdjustmentParameter { parameter: row.parameter.clone(), value: params.get(&row.parameter).and_then(semio_s_artifact_raster_raster_model::RasterAdjustmentNumber::from_parameter) }).collect());
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

pub(super) fn subtree_ids_of(layer: &RasterLayerNode) -> std::collections::BTreeSet<String> {
    let nested: std::collections::BTreeSet<String> = match layer {
        RasterLayerNode::Group { children, .. } => children.iter().flat_map(subtree_ids_of).collect(),
        _ => std::collections::BTreeSet::new(),
    };
    nested.into_iter().chain(std::iter::once(layer_node_id(layer).to_string())).collect()
}

/// 🔁️ The negative layers delta, read row by row from the BASE tree: an inserted subtree is removed at its after address, a
/// removed subtree comes back at its base address (cloned from the base), a move runs backwards, patches restore the base fields.
/// Nothing is applied or simulated.
pub(super) fn inverse_layers(delta: &RasterLayersDelta, base: &[RasterLayerNode]) -> RasterLayersDelta {
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
                (_, true) => semio_s_artifact_raster_raster_model::raster_image(&base.assets, key).map(|asset| (key.clone(), Some(asset))),
                (None, false) => None,
            }).collect() }),
            pixels: self.pixels.iter().rev().filter_map(|region| crate::inverse_pixel_region(region, base)).collect(),
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
    pub(super) id: String,
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
fn container_parts(removed: &[RasterLayerRemoval], moved: &[RasterLayerRelocation], inserted: Vec<(Option<String>, usize, String)>) -> BTreeMap<Option<String>, ContainerParts> {
    let mut parts: BTreeMap<Option<String>, ContainerParts> = BTreeMap::new();
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
    let mut leaving: BTreeMap<String, Vec<(usize, String, bool)>> = BTreeMap::new();
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
    let mut lifted: BTreeMap<String, RasterLayerNode> = BTreeMap::new();
    for (container, mut rows) in leaving {
        rows.sort_by(|left, right| right.0.cmp(&left.0));
        let Some(children) = born_children_mut(born, &container) else { continue };
        for (index, id, removed) in rows {
            if children.get(index).is_some_and(|node| layer_node_id(node) == id) {
                let node = children.remove(index);
                if removed {
                    semio_s_artifact_raster_raster_model::retire_raster_layer(node);
                } else {
                    lifted.insert(id, node);
                }
            }
        }
    }
    let mut entering: BTreeMap<String, Vec<(usize, RasterLayerNode)>> = BTreeMap::new();
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
            _ => semio_s_artifact_raster_raster_model::retire_raster_layers(rows.into_iter().map(|(_, node)| node).collect()),
        }
    }
    semio_s_artifact_raster_raster_model::retire_raster_layers(lifted.into_values().collect());
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
pub(super) fn absorb_layers_delta(dst: &mut RasterLayersDelta, mut src: RasterLayersDelta) {
    let mut born_rows = std::mem::take(&mut dst.inserted);
    fold_into_born(&mut born_rows, &mut src);
    let inserted_of = |rows: &[RasterLayerInsertion]| rows.iter().map(|row| (row.parent_id.clone(), row.index, layer_node_id(&row.layer).to_string())).collect::<Vec<_>>();
    let mut left = container_parts(&dst.removed, &dst.moved, inserted_of(&born_rows));
    let mut right = container_parts(&src.removed, &src.moved, inserted_of(&src.inserted));
    let mut nodes: BTreeMap<String, RasterLayerNode> = BTreeMap::new();
    for row in born_rows.into_iter().chain(std::mem::take(&mut src.inserted)) {
        if let Some(displaced) = nodes.insert(layer_node_id(&row.layer).to_string(), row.layer) {
            semio_s_artifact_raster_raster_model::retire_raster_layer(displaced);
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
    semio_s_artifact_raster_raster_model::retire_raster_layers(nodes.into_values().collect());
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
