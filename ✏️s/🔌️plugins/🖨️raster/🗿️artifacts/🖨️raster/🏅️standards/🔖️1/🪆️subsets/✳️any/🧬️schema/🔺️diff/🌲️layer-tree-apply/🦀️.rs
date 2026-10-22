//! 🌲️ Validation and application of layer patches, layer deltas, asset deltas and pixel regions onto a snapshot.

use super::*;

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
            if params.len()+added-removed>semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY {return Err(protocol::MutationApplyError::new("mutation.apply.parameter-capacity","adjustment parameter capacity exceeded"));}
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
        inverse.mask_content = Some(semio_s_artifact_raster_raster_model::RasterMaskContent { mask: std::mem::replace(mask, content.mask.clone()) });
    }
    match node {
        RasterLayerNode::Pixel { name, visible, locked, opacity, blend_mode, transform, width, height, image_key, .. } => {
            if let Some(content) = &patch.pixel_content {
                inverse.pixel_content = Some(semio_s_artifact_raster_raster_model::RasterPixelContent { image_key: image_key.clone(), width: *width, height: *height });
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
                    let value=previous.as_ref().and_then(semio_s_artifact_raster_raster_model::RasterAdjustmentNumber::from_parameter);
                    semio_s_artifact_raster_raster_model::RasterAdjustmentParameter {parameter:row.parameter.clone(),value}
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
pub(super) fn container_of<'a>(layers: &'a [RasterLayerNode], parent_id: Option<&str>) -> Option<&'a [RasterLayerNode]> {
    let Some(parent_id) = parent_id else { return Some(layers) };
    layers.iter().find_map(|layer| match layer {
        RasterLayerNode::Group { id, children, .. } if id.as_str() == parent_id => Some(children.as_slice()),
        RasterLayerNode::Group { children, .. } => container_of(children, Some(parent_id)),
        _ => None,
    })
}

pub(super) fn container_mut<'a>(layers: &'a mut Vec<RasterLayerNode>, parent_id: Option<&str>) -> Option<&'a mut Vec<RasterLayerNode>> {
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
pub(super) fn check_slots(survivors: usize, entering: &[(usize, RasterLayerNode)]) -> protocol::MutationApplyResult<()> {
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
pub(super) fn place_entering(children: &mut Vec<RasterLayerNode>, entering: Vec<(usize, RasterLayerNode)>) {
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
            semio_s_artifact_raster_raster_model::retire_raster_layer(removed);
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
                        semio_s_artifact_raster_raster_model::retire_raster_layers(rows.into_iter().map(|(_, node)| node).collect());
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
            semio_s_artifact_raster_raster_model::retire_raster_layers(next);
            return Err(error.under(["modified".to_string(), index.to_string()]));
        }
    }
    let next_ids: Vec<_> = flatten_raster_layers(&next).into_iter().map(layer_node_id).collect();
    if next_ids.iter().enumerate().any(|(index, id)| next_ids[..index].contains(id)) {
        drop(next_ids);
        semio_s_artifact_raster_raster_model::retire_raster_layers(next);
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
        semio_s_artifact_raster_raster_model::retire_raster_layers(rows.into_iter().map(|(_, node)| node).collect());
    }
    semio_s_artifact_raster_raster_model::retire_raster_layers(tree);
}

fn apply_layer_patch_entry(layers: &mut [RasterLayerNode], entry: &RasterLayerModification) -> protocol::MutationApplyResult<()> {
    patch_layer_in_tree(layers, &entry.id, &entry.patch).map(|_| ()).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "modified layer does not exist"))
}

fn validate_assets_delta<T>(assets: &semio_s_artifact_raster_raster_model::RasterOwnedMap<T>, delta: &RasterAssetsDelta) -> protocol::MutationApplyResult<()> {
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
    fn retire_cold(self) {
        let RasterDiff { schema: _, id: _, title: _, layers, assets: _, pixels: _ } = self;
        if let Some(layers) = layers {
            for insertion in layers.inserted {
                semio_s_artifact_raster_raster_model::retire_raster_layer(insertion.layer);
            }
        }
    }
    fn retire_projection(projection: RasterSnapshot) {
        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(projection);
    }
}

impl protocol::ColdMutationDiff<RasterSnapshot> for RasterDiff {
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

    /// 🧬️ Applies the sparse delta to a snapshot, POPULATED maps included. `RasterOwnedMap::clone` is
    /// a real deep copy bounded by construction (64 entries over 8 pages) and needs no retained page
    /// authority, so the blanket "populated Raster maps require the retained initialization
    /// authority" refusal that used to head this function only ever stopped the framework's own
    /// history folds: `redo` re-applies the reinstated edit's forwards through here, and so do a fold
    /// to base, a `.spr` reload and a remote ingest — every one of them was refused on a demo-shaped
    /// document (🖨️raster's redo clause, measured 2026-09-20). Asset REMOVAL is not retained either:
    /// the map hands back its exact `(key, child)` pair and the emptied page backing is released
    /// explicitly, so the history arithmetic can undo and redo `remove-layer-asset` like any verb.
    fn apply(&self, unpublished: &mut RasterSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let applied = (|snapshot: &RasterSnapshot| -> protocol::MutationApplyResult<RasterSnapshot> {
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
                semio_s_artifact_raster_raster_model::retire_raster_layers(displaced);
                match applied {
                    Ok(layers) => next.layers = layers,
                    Err(error) => {
                        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(next);
                        return Err(error.under(["layers"]));
                    }
                }
            }
            if let Some(assets) = &self.assets {
                for (key, value) in &assets.entries {
                    match value {
                        Some(asset) => {
                            next.assets.insert(key.clone(), semio_s_artifact_raster_raster_model::mint_raster_image_child(key, asset)).expect("unique Raster assets fit the preflighted map capacity");
                        }
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
                        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(displaced);
                        next = applied;
                    }
                    Err(error) => {
                        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(displaced);
                        return Err(error.under(["pixels"]));
                    }
                }
            }
            next
        })
    
        })(&*unpublished)?;
        *unpublished = applied;
        Ok(())
    }
}

//#region 🔖️PixelRegions
/// 🖌️ Applies one pixel region: the target image is rewritten in that rectangle, filed as a new content-addressed asset and
/// the layer (or its mask) repointed — the derived handle is minted here, from the applied pixels.
fn apply_pixel_region(snapshot: &RasterSnapshot, region: &RasterPixelRegion, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RasterSnapshot> {

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
