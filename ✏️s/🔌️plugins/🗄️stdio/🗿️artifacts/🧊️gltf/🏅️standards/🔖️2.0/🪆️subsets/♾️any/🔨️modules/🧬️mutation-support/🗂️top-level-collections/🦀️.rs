//! 🔒 Read-only typed-reference scan and sparse row mechanics private to the top-level collection leaves. Nothing here writes
//! into a snapshot: every function reads `base` and answers the sparse rows the owning leaf declares.
use crate::schema::diff::*;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};

/// 🧭️ The 13 top-level glTF collections other entities address by index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltfTopLevelFamily {
    Scenes,
    Nodes,
    Meshes,
    Accessors,
    BufferViews,
    Buffers,
    Materials,
    Textures,
    Images,
    Samplers,
    Skins,
    Animations,
    Cameras,
}

//#region 🔖️ListValues
/// 📋️ `items` with `item` inserted at `position`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn with_inserted<T: Clone>(items: &[T], position: usize, item: T) -> Vec<T> {
    let mut next = items.to_vec();
    next.insert(position, item);
    next
}
/// 📋️ `items` without the entry at `index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn without<T: Clone>(items: &[T], index: usize) -> Vec<T> {
    let mut next = items.to_vec();
    next.remove(index);
    next
}
/// 📋️ `items` with the entry at `from` moved to `to`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn with_moved<T: Clone>(items: &[T], from: usize, to: usize) -> Vec<T> {
    let mut next = items.to_vec();
    let item = next.remove(from);
    next.insert(to, item);
    next
}
/// 📋️ `items` with the entry at `index` replaced by `item`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn with_replaced<T: Clone>(items: &[T], index: usize, item: T) -> Vec<T> {
    let mut next = items.to_vec();
    next[index] = item;
    next
}
/// 📋️ `items` rebuilt as `order` (`order[new] = old`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn permuted<T: Clone>(items: &[T], order: &[usize]) -> Vec<T> {
    order.iter().map(|old| items[*old].clone()).collect()
}
//#endregion 🔖️ListValues

//#region 🔖️RowBuilders
/// 🩹 `diff` unless it names nothing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn changed<D: Default + PartialEq>(diff: D) -> Option<D> {
    (diff != D::default()).then_some(diff)
}
/// 🩹 One modified row at base `index`, or nothing when the row names no field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn patch<T, D: Default + PartialEq>(index: usize, diff: D) -> Option<GltfCollectionDiff<T, D>> {
    changed(diff).map(|diff| GltfCollectionDiff { removed: Vec::new(), modified: vec![GltfModified { index, diff }], added: Vec::new() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row<T, D: Default>(slot: &mut Option<GltfCollectionDiff<T, D>>, index: usize) -> &mut D {
    let collection = slot.get_or_insert_with(GltfCollectionDiff::default);
    let at = collection.modified.iter().position(|entry| entry.index == index).unwrap_or_else(|| {
        collection.modified.push(GltfModified { index, diff: D::default() });
        collection.modified.len() - 1
    });
    &mut collection.modified[at].diff
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn put<T: Clone + PartialEq>(slot: &mut Option<GltfCollectionDiff<T, T>>, index: usize, base: &T, next: T) {
    if &next != base {
        slot.get_or_insert_with(GltfCollectionDiff::default).modified.push(GltfModified { index, diff: next });
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sorted<T, D>(slot: &mut Option<GltfCollectionDiff<T, D>>) {
    if let Some(collection) = slot {
        collection.modified.sort_by_key(|entry| entry.index);
    }
}
//#endregion 🔖️RowBuilders

//#region 🔖️IndexRemaps
/// 🔢️ Where a reference lands after a collection insert at `position`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn after_insert(position: usize) -> impl FnMut(usize) -> Option<usize> {
    move |value| Some(if value >= position { value + 1 } else { value })
}
/// 🔢️ Where a reference lands after the entry at `index` is deleted; a reference to it is dropped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn after_delete(index: usize) -> impl FnMut(usize) -> Option<usize> {
    move |value| (value != index).then_some(if value > index { value - 1 } else { value })
}
/// 🔢️ Where a reference lands after the entry at `from` moves to `to`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn after_move(from: usize, to: usize) -> impl FnMut(usize) -> Option<usize> {
    move |value| Some(moved_index(value, from, to))
}
/// 🔢️ The index `value` takes when the entry at `from` moves to `to`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn moved_index(value: usize, from: usize, to: usize) -> usize {
    if value == from {
        to
    } else if from < to && value > from && value <= to {
        value - 1
    } else if to < from && value >= to && value < from {
        value + 1
    } else {
        value
    }
}
/// 🔢️ Where a reference lands after the collection is rebuilt as `order` (`order[new] = old`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn after_reorder(order: &[usize]) -> impl FnMut(usize) -> Option<usize> + '_ {
    move |value| order.iter().position(|candidate| *candidate == value)
}
/// 🔁️ The permutation that undoes `order`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_order(order: &[usize]) -> Vec<usize> {
    let mut inverse = vec![0; order.len()];
    for (new, old) in order.iter().enumerate() {
        inverse[*old] = new;
    }
    inverse
}
//#endregion 🔖️IndexRemaps

//#region 🔖️ReferenceScan
type Remap<'a> = &'a mut dyn FnMut(usize) -> Option<usize>;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn optional(value: Option<usize>, remap: Remap) -> Option<usize> {
    value.and_then(|index| remap(index))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn list(values: &[usize], remap: Remap) -> Vec<usize> {
    values.iter().filter_map(|index| remap(*index)).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pairs(values: &[(String, usize)], remap: Remap) -> Vec<(String, usize)> {
    values.iter().filter_map(|(semantic, index)| remap(*index).map(|mapped| (semantic.clone(), mapped))).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn texture_info(info: &Option<GltfTextureInfo>, remap: Remap) -> Option<GltfTextureInfo> {
    info.as_ref().and_then(|info| remap(info.index).map(|index| GltfTextureInfo { index, ..info.clone() }))
}

/// 🧵️ The sparse rows that rebind every typed reference to `family` through `remap`: a referrer is named only when one of its
/// references lands elsewhere (or is dropped, when `remap` answers `None`). Entries of `family` itself that refer to `family`
/// (node children) are named like any other referrer; the owning leaf folds or drops them.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rewire(base: &GltfSnapshot, family: GltfTopLevelFamily, remap: Remap) -> GltfDiff {
    let document = &base.document;
    let mut diff = GltfDiff::default();
    match family {
        GltfTopLevelFamily::Scenes => {
            let scene = optional(document.scene, remap);
            if scene != document.scene {
                diff.scene = Some(scene);
            }
        }
        GltfTopLevelFamily::Nodes => {
            for (index, scene) in document.scenes.iter().enumerate() {
                let nodes = list(&scene.nodes, remap);
                if nodes != scene.nodes {
                    row(&mut diff.scenes, index).nodes = Some(nodes);
                }
            }
            for (index, node) in document.nodes.iter().enumerate() {
                let children = list(&node.children, remap);
                if children != node.children {
                    row(&mut diff.nodes, index).children = Some(children);
                }
            }
            for (index, skin) in document.skins.iter().enumerate() {
                let next = GltfSkin { skeleton: optional(skin.skeleton, remap), joints: list(&skin.joints, remap), ..skin.clone() };
                put(&mut diff.skins, index, skin, next);
            }
            for (index, animation) in document.animations.iter().enumerate() {
                let channels = animation.channels.iter().map(|channel| GltfAnimationChannel { target: GltfAnimationChannelTarget { node: optional(channel.target.node, remap), ..channel.target.clone() }, ..channel.clone() }).collect();
                put(&mut diff.animations, index, animation, GltfAnimation { channels, ..animation.clone() });
            }
        }
        GltfTopLevelFamily::Meshes => {
            for (index, node) in document.nodes.iter().enumerate() {
                let mesh = optional(node.mesh, remap);
                if mesh != node.mesh {
                    row(&mut diff.nodes, index).mesh = Some(mesh);
                }
            }
        }
        GltfTopLevelFamily::Accessors => {
            for (index, mesh) in document.meshes.iter().enumerate() {
                let primitives: Vec<GltfPrimitive> = mesh
                    .primitives
                    .iter()
                    .map(|primitive| GltfPrimitive {
                        attributes: pairs(&primitive.attributes, remap),
                        indices: optional(primitive.indices, remap),
                        targets: primitive.targets.iter().map(|target| GltfMorphTarget(pairs(&target.0, remap))).collect(),
                        ..primitive.clone()
                    })
                    .collect();
                if primitives != mesh.primitives {
                    row(&mut diff.meshes, index).primitives = Some(primitives);
                }
            }
            for (index, skin) in document.skins.iter().enumerate() {
                put(&mut diff.skins, index, skin, GltfSkin { inverse_bind_matrices: optional(skin.inverse_bind_matrices, remap), ..skin.clone() });
            }
            for (index, animation) in document.animations.iter().enumerate() {
                let samplers = animation.samplers.iter().map(|sampler| GltfAnimationSampler { input: remap(sampler.input).unwrap_or(sampler.input), output: remap(sampler.output).unwrap_or(sampler.output), ..sampler.clone() }).collect();
                put(&mut diff.animations, index, animation, GltfAnimation { samplers, ..animation.clone() });
            }
        }
        GltfTopLevelFamily::BufferViews => {
            for (index, accessor) in document.accessors.iter().enumerate() {
                let buffer_view = optional(accessor.buffer_view, remap);
                if buffer_view != accessor.buffer_view {
                    row(&mut diff.accessors, index).buffer_view = Some(buffer_view);
                }
                if let Some(sparse) = &accessor.sparse {
                    let next = GltfSparseAccessor {
                        indices: GltfSparseIndices { buffer_view: remap(sparse.indices.buffer_view).unwrap_or(sparse.indices.buffer_view), ..sparse.indices.clone() },
                        values: GltfSparseValues { buffer_view: remap(sparse.values.buffer_view).unwrap_or(sparse.values.buffer_view), ..sparse.values.clone() },
                        ..sparse.clone()
                    };
                    if &next != sparse {
                        row(&mut diff.accessors, index).sparse = Some(Some(next));
                    }
                }
            }
            for (index, image) in document.images.iter().enumerate() {
                put(&mut diff.images, index, image, GltfImage { buffer_view: optional(image.buffer_view, remap), ..image.clone() });
            }
        }
        GltfTopLevelFamily::Buffers => {
            for (index, view) in document.buffer_views.iter().enumerate() {
                put(&mut diff.buffer_views, index, view, GltfBufferView { buffer: remap(view.buffer).unwrap_or(view.buffer), ..view.clone() });
            }
        }
        GltfTopLevelFamily::Materials => {
            for (index, mesh) in document.meshes.iter().enumerate() {
                let primitives: Vec<GltfPrimitive> = mesh.primitives.iter().map(|primitive| GltfPrimitive { material: optional(primitive.material, remap), ..primitive.clone() }).collect();
                if primitives != mesh.primitives {
                    row(&mut diff.meshes, index).primitives = Some(primitives);
                }
            }
        }
        GltfTopLevelFamily::Textures => {
            for (index, material) in document.materials.iter().enumerate() {
                let pbr = material.pbr_metallic_roughness.as_ref().map(|pbr| GltfPbrMetallicRoughness { base_color_texture: texture_info(&pbr.base_color_texture, remap), metallic_roughness_texture: texture_info(&pbr.metallic_roughness_texture, remap), ..pbr.clone() });
                let normal = material.normal_texture.as_ref().and_then(|info| remap(info.index).map(|mapped| GltfNormalTextureInfo { index: mapped, ..info.clone() }));
                let occlusion = material.occlusion_texture.as_ref().and_then(|info| remap(info.index).map(|mapped| GltfOcclusionTextureInfo { index: mapped, ..info.clone() }));
                let emissive = texture_info(&material.emissive_texture, remap);
                if pbr != material.pbr_metallic_roughness {
                    row(&mut diff.materials, index).pbr_metallic_roughness = Some(pbr);
                }
                if normal != material.normal_texture {
                    row(&mut diff.materials, index).normal_texture = Some(normal);
                }
                if occlusion != material.occlusion_texture {
                    row(&mut diff.materials, index).occlusion_texture = Some(occlusion);
                }
                if emissive != material.emissive_texture {
                    row(&mut diff.materials, index).emissive_texture = Some(emissive);
                }
            }
        }
        GltfTopLevelFamily::Images => {
            for (index, texture) in document.textures.iter().enumerate() {
                put(&mut diff.textures, index, texture, GltfTexture { source: optional(texture.source, remap), ..texture.clone() });
            }
        }
        GltfTopLevelFamily::Samplers => {
            for (index, texture) in document.textures.iter().enumerate() {
                put(&mut diff.textures, index, texture, GltfTexture { sampler: optional(texture.sampler, remap), ..texture.clone() });
            }
        }
        GltfTopLevelFamily::Skins => {
            for (index, node) in document.nodes.iter().enumerate() {
                let skin = optional(node.skin, remap);
                if skin != node.skin {
                    row(&mut diff.nodes, index).skin = Some(skin);
                }
            }
        }
        GltfTopLevelFamily::Animations => {}
        GltfTopLevelFamily::Cameras => {
            for (index, node) in document.nodes.iter().enumerate() {
                let camera = optional(node.camera, remap);
                if camera != node.camera {
                    row(&mut diff.nodes, index).camera = Some(camera);
                }
            }
        }
    }
    sorted(&mut diff.scenes);
    sorted(&mut diff.nodes);
    sorted(&mut diff.meshes);
    sorted(&mut diff.accessors);
    sorted(&mut diff.buffer_views);
    sorted(&mut diff.materials);
    sorted(&mut diff.textures);
    sorted(&mut diff.images);
    sorted(&mut diff.skins);
    sorted(&mut diff.animations);
    diff
}

/// 🛑️ Refuses to delete `index` of `family` while a typed reference to it exists that no mutation can bind back, so the deletion
/// stays exactly invertible: the references the document lets a bind mutation restore (node mesh/camera/skin, primitive
/// material/indices/attributes, scene roots, node children, the default scene) are dropped with the entry and restored by the
/// inverse; every other reference must be removed first.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn require_unreferenced(base: &GltfSnapshot, family: GltfTopLevelFamily, index: usize) -> Result<(), GltfTopLevelMutationRejection> {
    let document = &base.document;
    let in_use = |path: &str| Err(reject("gltf.reference.in-use", path, "deletion would drop a typed reference no mutation can restore"));
    match family {
        GltfTopLevelFamily::Nodes => {
            if document.skins.iter().any(|skin| skin.skeleton == Some(index) || skin.joints.contains(&index)) {
                return in_use("document/skins");
            }
            if document.animations.iter().any(|animation| animation.channels.iter().any(|channel| channel.target.node == Some(index))) {
                return in_use("document/animations");
            }
        }
        GltfTopLevelFamily::Accessors => {
            if document.skins.iter().any(|skin| skin.inverse_bind_matrices == Some(index)) {
                return in_use("document/skins");
            }
            if document.animations.iter().any(|animation| animation.samplers.iter().any(|sampler| sampler.input == index || sampler.output == index)) {
                return in_use("document/animations");
            }
        }
        GltfTopLevelFamily::BufferViews => {
            if document.accessors.iter().any(|accessor| accessor.buffer_view == Some(index) || accessor.sparse.as_ref().is_some_and(|sparse| sparse.indices.buffer_view == index || sparse.values.buffer_view == index)) {
                return in_use("document/accessors");
            }
            if document.images.iter().any(|image| image.buffer_view == Some(index)) {
                return in_use("document/images");
            }
        }
        GltfTopLevelFamily::Buffers => {
            if document.buffer_views.iter().any(|view| view.buffer == index) {
                return in_use("document/bufferViews");
            }
        }
        GltfTopLevelFamily::Textures => {
            let refers = |info: &Option<GltfTextureInfo>| info.as_ref().is_some_and(|info| info.index == index);
            if document.materials.iter().any(|material| {
                material.pbr_metallic_roughness.as_ref().is_some_and(|pbr| refers(&pbr.base_color_texture) || refers(&pbr.metallic_roughness_texture))
                    || material.normal_texture.as_ref().is_some_and(|info| info.index == index)
                    || material.occlusion_texture.as_ref().is_some_and(|info| info.index == index)
                    || refers(&material.emissive_texture)
            }) {
                return in_use("document/materials");
            }
        }
        GltfTopLevelFamily::Images => {
            if document.textures.iter().any(|texture| texture.source == Some(index)) {
                return in_use("document/textures");
            }
        }
        GltfTopLevelFamily::Samplers => {
            if document.textures.iter().any(|texture| texture.sampler == Some(index)) {
                return in_use("document/textures");
            }
        }
        GltfTopLevelFamily::Scenes | GltfTopLevelFamily::Meshes | GltfTopLevelFamily::Materials | GltfTopLevelFamily::Skins | GltfTopLevelFamily::Animations | GltfTopLevelFamily::Cameras => {}
    }
    Ok(())
}
//#endregion 🔖️ReferenceScan

//#region 🔖️RecordReferences
/// 📏️ How many entries each top-level collection holds once the record under validation is in place.
#[derive(Clone, Copy, Debug)]
pub struct GltfLengths {
    pub scenes: usize,
    pub nodes: usize,
    pub meshes: usize,
    pub accessors: usize,
    pub buffer_views: usize,
    pub buffers: usize,
    pub materials: usize,
    pub textures: usize,
    pub images: usize,
    pub samplers: usize,
    pub skins: usize,
    pub animations: usize,
    pub cameras: usize,
}
impl GltfLengths {
    /// 📏️ The collection lengths of `base` after `family` gained one entry.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn after_insert(base: &GltfSnapshot, family: GltfTopLevelFamily) -> Self {
        let document = &base.document;
        let grow = |candidate: GltfTopLevelFamily, length: usize| length + usize::from(candidate == family);
        Self {
            scenes: grow(GltfTopLevelFamily::Scenes, document.scenes.len()),
            nodes: grow(GltfTopLevelFamily::Nodes, document.nodes.len()),
            meshes: grow(GltfTopLevelFamily::Meshes, document.meshes.len()),
            accessors: grow(GltfTopLevelFamily::Accessors, document.accessors.len()),
            buffer_views: grow(GltfTopLevelFamily::BufferViews, document.buffer_views.len()),
            buffers: grow(GltfTopLevelFamily::Buffers, document.buffers.len()),
            materials: grow(GltfTopLevelFamily::Materials, document.materials.len()),
            textures: grow(GltfTopLevelFamily::Textures, document.textures.len()),
            images: grow(GltfTopLevelFamily::Images, document.images.len()),
            samplers: grow(GltfTopLevelFamily::Samplers, document.samplers.len()),
            skins: grow(GltfTopLevelFamily::Skins, document.skins.len()),
            animations: grow(GltfTopLevelFamily::Animations, document.animations.len()),
            cameras: grow(GltfTopLevelFamily::Cameras, document.cameras.len()),
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn within(values: impl IntoIterator<Item = usize>, length: usize, path: &str) -> Result<(), GltfTopLevelMutationRejection> {
    match values.into_iter().find(|value| *value >= length) {
        Some(value) => Err(reject("gltf.mutation.reference-out-of-range", path, format!("reference {value} does not address an entry (length {length})"))),
        None => Ok(()),
    }
}
/// 🔎️ A created scene's roots address existing nodes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_scene(scene: &GltfScene, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(scene.nodes.iter().copied(), lengths.nodes, "document/scenes/nodes")
}
/// 🔎️ A created node at `at` addresses existing nodes (never itself), meshes, cameras and skins.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_node(node: &GltfNode, at: usize, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(node.children.iter().copied(), lengths.nodes, "document/nodes/children")?;
    if node.children.contains(&at) {
        return Err(reject("gltf.mutation.node-cycle", "document/nodes/children", "a node cannot parent itself"));
    }
    within(node.mesh, lengths.meshes, "document/nodes/mesh")?;
    within(node.camera, lengths.cameras, "document/nodes/camera")?;
    within(node.skin, lengths.skins, "document/nodes/skin")
}
/// 🔎️ A created primitive addresses existing accessors and materials.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_primitive(primitive: &GltfPrimitive, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(primitive.attributes.iter().map(|(_, accessor)| *accessor), lengths.accessors, "document/meshes/primitives/attributes")?;
    within(primitive.indices, lengths.accessors, "document/meshes/primitives/indices")?;
    within(primitive.material, lengths.materials, "document/meshes/primitives/material")?;
    primitive.targets.iter().try_for_each(|target| check_target(target, lengths))
}
/// 🔎️ A created morph target addresses existing accessors.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_target(target: &GltfMorphTarget, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(target.0.iter().map(|(_, accessor)| *accessor), lengths.accessors, "document/meshes/primitives/targets")
}
/// 🔎️ A created mesh addresses existing accessors and materials and carries finite weights.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_mesh(mesh: &GltfMesh, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    if !mesh.weights.iter().all(|weight| weight.is_finite()) {
        return Err(reject("gltf.mutation.invalid-morph-weights", "document/meshes/weights", "weights must be finite"));
    }
    mesh.primitives.iter().try_for_each(|primitive| check_primitive(primitive, lengths))
}
/// 🔎️ A created accessor addresses existing buffer views.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_accessor(accessor: &GltfAccessor, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(accessor.buffer_view, lengths.buffer_views, "document/accessors/bufferView")?;
    within(accessor.sparse.iter().flat_map(|sparse| [sparse.indices.buffer_view, sparse.values.buffer_view]), lengths.buffer_views, "document/accessors/sparse")
}
/// 🔎️ A created buffer view addresses an existing buffer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_buffer_view(view: &GltfBufferView, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within([view.buffer], lengths.buffers, "document/bufferViews/buffer")
}
/// 🔎️ A created material addresses existing textures.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_material(material: &GltfMaterial, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    let pbr = material.pbr_metallic_roughness.iter().flat_map(|pbr| [&pbr.base_color_texture, &pbr.metallic_roughness_texture]).flatten().map(|info| info.index);
    let others = material.normal_texture.iter().map(|info| info.index).chain(material.occlusion_texture.iter().map(|info| info.index)).chain(material.emissive_texture.iter().map(|info| info.index));
    within(pbr.chain(others), lengths.textures, "document/materials/textures")
}
/// 🔎️ A created texture addresses an existing sampler and image.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_texture(texture: &GltfTexture, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(texture.sampler, lengths.samplers, "document/textures/sampler")?;
    within(texture.source, lengths.images, "document/textures/source")
}
/// 🔎️ A created image addresses an existing buffer view.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_image(image: &GltfImage, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(image.buffer_view, lengths.buffer_views, "document/images/bufferView")
}
/// 🔎️ A created skin addresses existing accessors and nodes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_skin(skin: &GltfSkin, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(skin.inverse_bind_matrices, lengths.accessors, "document/skins/inverseBindMatrices")?;
    within(skin.skeleton.into_iter().chain(skin.joints.iter().copied()), lengths.nodes, "document/skins/joints")
}
/// 🔎️ A created animation addresses its own samplers and existing accessors and nodes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_animation(animation: &GltfAnimation, lengths: &GltfLengths) -> Result<(), GltfTopLevelMutationRejection> {
    within(animation.channels.iter().map(|channel| channel.sampler), animation.samplers.len(), "document/animations/channels/sampler")?;
    within(animation.channels.iter().filter_map(|channel| channel.target.node), lengths.nodes, "document/animations/channels/target")?;
    within(animation.samplers.iter().flat_map(|sampler| [sampler.input, sampler.output]), lengths.accessors, "document/animations/samplers")
}
//#endregion 🔖️RecordReferences
