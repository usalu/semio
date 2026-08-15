//! 🧭️ GLTF mutation validation, reference transport, and semantic planning.

use super::*;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

//#region 🛂️SemanticPlanning
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GltfMutationRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}

impl std::fmt::Display for GltfMutationRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}: {}", self.code, self.path, self.detail)
    }
}

pub(crate) fn reject(code: &str, path: impl Into<String>, detail: impl Into<String>) -> GltfMutationRejection {
    GltfMutationRejection { code: code.into(), path: path.into(), detail: detail.into() }
}

pub(crate) fn check_index(path: impl Into<String>, index: usize, len: usize) -> Result<(), GltfMutationRejection> {
    let path = path.into();
    (index < len).then_some(()).ok_or_else(|| reject("gltf.reference.out-of-range", path, format!("index {index}, length {len}")))
}

fn validate_material_references(material: &GltfMaterial, index: usize, texture_len: usize) -> Result<(), GltfMutationRejection> {
    let mut refs = Vec::new();
    if let Some(pbr) = &material.pbr_metallic_roughness {
        if let Some(info) = &pbr.base_color_texture {
            refs.push(("pbrMetallicRoughness/baseColorTexture", info.index));
        }
        if let Some(info) = &pbr.metallic_roughness_texture {
            refs.push(("pbrMetallicRoughness/metallicRoughnessTexture", info.index));
        }
    }
    if let Some(info) = &material.normal_texture {
        refs.push(("normalTexture", info.index));
    }
    if let Some(info) = &material.occlusion_texture {
        refs.push(("occlusionTexture", info.index));
    }
    if let Some(info) = &material.emissive_texture {
        refs.push(("emissiveTexture", info.index));
    }
    for (field, target) in refs {
        check_index(format!("document/materials/{index}/{field}"), target, texture_len)?;
    }
    Ok(())
}

/// 🛂️ Validates every typed glTF index dependency and the node hierarchy.
pub fn validate_gltf_references(snapshot: &GltfSnapshot) -> Result<(), GltfMutationRejection> {
    let doc = &snapshot.document;
    if doc.buffers.len() != snapshot.buffers.len() {
        return Err(reject("gltf.buffer.alignment", "buffers", format!("{} metadata entries, {} payload entries", doc.buffers.len(), snapshot.buffers.len())));
    }
    if let Some(scene) = doc.scene {
        check_index("document/scene", scene, doc.scenes.len())?;
    }
    for (buffer_index, (buffer, bytes)) in doc.buffers.iter().zip(&snapshot.buffers).enumerate() {
        let unresolved_external = bytes.is_empty() && buffer.uri.as_deref().is_some_and(|uri| !uri.starts_with("data:"));
        if !unresolved_external && (bytes.len() < buffer.byte_length || bytes.len() > buffer.byte_length.saturating_add(3)) {
            return Err(reject("gltf.buffer.byte-length", format!("document/buffers/{buffer_index}/byteLength"), format!("declared {}, resolved {}", buffer.byte_length, bytes.len())));
        }
    }
    for (scene_index, scene) in doc.scenes.iter().enumerate() {
        for (slot, node) in scene.nodes.iter().copied().enumerate() {
            check_index(format!("document/scenes/{scene_index}/nodes/{slot}"), node, doc.nodes.len())?;
        }
    }
    let mut parent_count = vec![0usize; doc.nodes.len()];
    for (node_index, node) in doc.nodes.iter().enumerate() {
        for (slot, child) in node.children.iter().copied().enumerate() {
            check_index(format!("document/nodes/{node_index}/children/{slot}"), child, doc.nodes.len())?;
            parent_count[child] += 1;
            if parent_count[child] > 1 {
                return Err(reject("gltf.node.multiple-parents", format!("document/nodes/{child}"), "node occurs in more than one parent child list"));
            }
        }
        if let Some(mesh) = node.mesh {
            check_index(format!("document/nodes/{node_index}/mesh"), mesh, doc.meshes.len())?;
        }
        if let Some(camera) = node.camera {
            check_index(format!("document/nodes/{node_index}/camera"), camera, doc.cameras.len())?;
        }
        if let Some(skin) = node.skin {
            check_index(format!("document/nodes/{node_index}/skin"), skin, doc.skins.len())?;
        }
        if node.matrix.is_some() && (node.translation.is_some() || node.rotation.is_some() || node.scale.is_some()) {
            return Err(reject("gltf.node.transform-exclusive", format!("document/nodes/{node_index}"), "matrix and TRS cannot coexist"));
        }
    }
    let mut indegree = parent_count;
    let mut ready: std::collections::VecDeque<usize> = indegree.iter().enumerate().filter_map(|(index, count)| (*count == 0).then_some(index)).collect();
    let mut visited = 0usize;
    while let Some(index) = ready.pop_front() {
        visited += 1;
        for child in &doc.nodes[index].children {
            indegree[*child] -= 1;
            if indegree[*child] == 0 {
                ready.push_back(*child);
            }
        }
    }
    if visited != doc.nodes.len() {
        let index = indegree.iter().position(|count| *count != 0).unwrap_or(0);
        return Err(reject("gltf.node.cycle", format!("document/nodes/{index}"), "node hierarchy contains a cycle"));
    }
    for (mesh_index, mesh) in doc.meshes.iter().enumerate() {
        for (primitive_index, primitive) in mesh.primitives.iter().enumerate() {
            for (semantic, accessor) in &primitive.attributes {
                check_index(format!("document/meshes/{mesh_index}/primitives/{primitive_index}/attributes/{semantic}"), *accessor, doc.accessors.len())?;
            }
            for (target_index, target) in primitive.targets.iter().enumerate() {
                for (semantic, accessor) in &target.0 {
                    check_index(format!("document/meshes/{mesh_index}/primitives/{primitive_index}/targets/{target_index}/{semantic}"), *accessor, doc.accessors.len())?;
                }
            }
            if let Some(accessor) = primitive.indices {
                check_index(format!("document/meshes/{mesh_index}/primitives/{primitive_index}/indices"), accessor, doc.accessors.len())?;
            }
            if let Some(material) = primitive.material {
                check_index(format!("document/meshes/{mesh_index}/primitives/{primitive_index}/material"), material, doc.materials.len())?;
            }
        }
    }
    for (accessor_index, accessor) in doc.accessors.iter().enumerate() {
        if accessor.normalized && matches!(accessor.component_type, crate::artifacts::gltf::engine::GltfComponentType::Float) {
            return Err(reject("gltf.accessor.normalized-float", format!("document/accessors/{accessor_index}/normalized"), "FLOAT accessors cannot be normalized"));
        }
        if let Some(view) = accessor.buffer_view {
            check_index(format!("document/accessors/{accessor_index}/bufferView"), view, doc.buffer_views.len())?;
        }
        if let Some(sparse) = &accessor.sparse {
            check_index(format!("document/accessors/{accessor_index}/sparse/indices/bufferView"), sparse.indices.buffer_view, doc.buffer_views.len())?;
            check_index(format!("document/accessors/{accessor_index}/sparse/values/bufferView"), sparse.values.buffer_view, doc.buffer_views.len())?;
        }
    }
    for (view_index, view) in doc.buffer_views.iter().enumerate() {
        check_index(format!("document/bufferViews/{view_index}/buffer"), view.buffer, doc.buffers.len())?;
        let end = view.byte_offset.checked_add(view.byte_length).ok_or_else(|| reject("gltf.buffer-view.range-overflow", format!("document/bufferViews/{view_index}"), "byteOffset + byteLength overflowed"))?;
        if end > doc.buffers[view.buffer].byte_length {
            return Err(reject("gltf.buffer-view.out-of-range", format!("document/bufferViews/{view_index}"), format!("end {end}, buffer byteLength {}", doc.buffers[view.buffer].byte_length)));
        }
    }
    for (material_index, material) in doc.materials.iter().enumerate() {
        validate_material_references(material, material_index, doc.textures.len())?;
    }
    for (texture_index, texture) in doc.textures.iter().enumerate() {
        if let Some(sampler) = texture.sampler {
            check_index(format!("document/textures/{texture_index}/sampler"), sampler, doc.samplers.len())?;
        }
        if let Some(source) = texture.source {
            check_index(format!("document/textures/{texture_index}/source"), source, doc.images.len())?;
        }
    }
    for (image_index, image) in doc.images.iter().enumerate() {
        if let Some(view) = image.buffer_view {
            check_index(format!("document/images/{image_index}/bufferView"), view, doc.buffer_views.len())?;
        }
    }
    for (skin_index, skin) in doc.skins.iter().enumerate() {
        if let Some(accessor) = skin.inverse_bind_matrices {
            check_index(format!("document/skins/{skin_index}/inverseBindMatrices"), accessor, doc.accessors.len())?;
        }
        if let Some(node) = skin.skeleton {
            check_index(format!("document/skins/{skin_index}/skeleton"), node, doc.nodes.len())?;
        }
        for (slot, node) in skin.joints.iter().copied().enumerate() {
            check_index(format!("document/skins/{skin_index}/joints/{slot}"), node, doc.nodes.len())?;
        }
    }
    for (animation_index, animation) in doc.animations.iter().enumerate() {
        for (channel_index, channel) in animation.channels.iter().enumerate() {
            check_index(format!("document/animations/{animation_index}/channels/{channel_index}/sampler"), channel.sampler, animation.samplers.len())?;
            if let Some(node) = channel.target.node {
                check_index(format!("document/animations/{animation_index}/channels/{channel_index}/target/node"), node, doc.nodes.len())?;
            }
        }
        for (sampler_index, sampler) in animation.samplers.iter().enumerate() {
            check_index(format!("document/animations/{animation_index}/samplers/{sampler_index}/input"), sampler.input, doc.accessors.len())?;
            check_index(format!("document/animations/{animation_index}/samplers/{sampler_index}/output"), sampler.output, doc.accessors.len())?;
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub(crate) enum IndexFamily {
    Scene,
    Node,
    Mesh,
    Accessor,
    Material,
    Buffer,
}

pub(crate) fn shift_insert(index: &mut usize, at: usize) {
    if *index >= at {
        *index += 1;
    }
}

fn shift_remove(index: &mut usize, at: usize) {
    if *index > at {
        *index -= 1;
    }
}

pub(crate) fn remap_references(doc: &mut GltfDocument, family: IndexFamily, at: usize, inserting: bool) {
    let remap = |index: &mut usize| if inserting { shift_insert(index, at) } else { shift_remove(index, at) };
    match family {
        IndexFamily::Scene => {
            if let Some(index) = &mut doc.scene {
                remap(index);
            }
        }
        IndexFamily::Node => {
            for scene in &mut doc.scenes {
                scene.nodes.iter_mut().for_each(&remap);
            }
            for node in &mut doc.nodes {
                node.children.iter_mut().for_each(&remap);
            }
            for skin in &mut doc.skins {
                if let Some(index) = &mut skin.skeleton {
                    remap(index);
                }
                skin.joints.iter_mut().for_each(&remap);
            }
            for animation in &mut doc.animations {
                for channel in &mut animation.channels {
                    if let Some(index) = &mut channel.target.node {
                        remap(index);
                    }
                }
            }
        }
        IndexFamily::Mesh => {
            for node in &mut doc.nodes {
                if let Some(index) = &mut node.mesh {
                    remap(index);
                }
            }
        }
        IndexFamily::Accessor => {
            for mesh in &mut doc.meshes {
                for primitive in &mut mesh.primitives {
                    for (_, index) in &mut primitive.attributes {
                        remap(index);
                    }
                    for target in &mut primitive.targets {
                        for (_, index) in &mut target.0 {
                            remap(index);
                        }
                    }
                    if let Some(index) = &mut primitive.indices {
                        remap(index);
                    }
                }
            }
            for skin in &mut doc.skins {
                if let Some(index) = &mut skin.inverse_bind_matrices {
                    remap(index);
                }
            }
            for animation in &mut doc.animations {
                for sampler in &mut animation.samplers {
                    remap(&mut sampler.input);
                    remap(&mut sampler.output);
                }
            }
        }
        IndexFamily::Material => {
            for mesh in &mut doc.meshes {
                for primitive in &mut mesh.primitives {
                    if let Some(index) = &mut primitive.material {
                        remap(index);
                    }
                }
            }
        }
        IndexFamily::Buffer => {
            for view in &mut doc.buffer_views {
                remap(&mut view.buffer);
            }
        }
    }
}

fn reference_to(doc: &GltfDocument, family: IndexFamily, target: usize) -> Option<String> {
    match family {
        IndexFamily::Scene => doc.scene.filter(|index| *index == target).map(|_| "document/scene".into()),
        IndexFamily::Node => {
            for (i, scene) in doc.scenes.iter().enumerate() {
                if let Some(slot) = scene.nodes.iter().position(|index| *index == target) {
                    return Some(format!("document/scenes/{i}/nodes/{slot}"));
                }
            }
            for (i, node) in doc.nodes.iter().enumerate() {
                if let Some(slot) = node.children.iter().position(|index| *index == target) {
                    return Some(format!("document/nodes/{i}/children/{slot}"));
                }
            }
            for (i, skin) in doc.skins.iter().enumerate() {
                if skin.skeleton == Some(target) {
                    return Some(format!("document/skins/{i}/skeleton"));
                }
                if let Some(slot) = skin.joints.iter().position(|index| *index == target) {
                    return Some(format!("document/skins/{i}/joints/{slot}"));
                }
            }
            for (i, animation) in doc.animations.iter().enumerate() {
                for (j, channel) in animation.channels.iter().enumerate() {
                    if channel.target.node == Some(target) {
                        return Some(format!("document/animations/{i}/channels/{j}/target/node"));
                    }
                }
            }
            None
        }
        IndexFamily::Mesh => doc.nodes.iter().enumerate().find(|(_, node)| node.mesh == Some(target)).map(|(i, _)| format!("document/nodes/{i}/mesh")),
        IndexFamily::Accessor => {
            for (i, mesh) in doc.meshes.iter().enumerate() {
                for (j, primitive) in mesh.primitives.iter().enumerate() {
                    if let Some((semantic, _)) = primitive.attributes.iter().find(|(_, index)| *index == target) {
                        return Some(format!("document/meshes/{i}/primitives/{j}/attributes/{semantic}"));
                    }
                    for (target_index, morph) in primitive.targets.iter().enumerate() {
                        if let Some((semantic, _)) = morph.0.iter().find(|(_, index)| *index == target) {
                            return Some(format!("document/meshes/{i}/primitives/{j}/targets/{target_index}/{semantic}"));
                        }
                    }
                    if primitive.indices == Some(target) {
                        return Some(format!("document/meshes/{i}/primitives/{j}/indices"));
                    }
                }
            }
            for (i, skin) in doc.skins.iter().enumerate() {
                if skin.inverse_bind_matrices == Some(target) {
                    return Some(format!("document/skins/{i}/inverseBindMatrices"));
                }
            }
            for (i, animation) in doc.animations.iter().enumerate() {
                for (j, sampler) in animation.samplers.iter().enumerate() {
                    if sampler.input == target || sampler.output == target {
                        return Some(format!("document/animations/{i}/samplers/{j}"));
                    }
                }
            }
            None
        }
        IndexFamily::Material => {
            for (i, mesh) in doc.meshes.iter().enumerate() {
                for (j, primitive) in mesh.primitives.iter().enumerate() {
                    if primitive.material == Some(target) {
                        return Some(format!("document/meshes/{i}/primitives/{j}/material"));
                    }
                }
            }
            None
        }
        IndexFamily::Buffer => doc.buffer_views.iter().enumerate().find(|(_, view)| view.buffer == target).map(|(i, _)| format!("document/bufferViews/{i}/buffer")),
    }
}

pub(crate) fn remove_checked<T>(items: &mut Vec<T>, family: IndexFamily, index: usize, doc: &GltfDocument, path: &str) -> Result<T, GltfMutationRejection> {
    check_index(path, index, items.len())?;
    if let Some(reference) = reference_to(doc, family, index) {
        return Err(reject("gltf.reference.in-use", path, format!("referenced by {reference}")));
    }
    Ok(items.remove(index))
}

pub(crate) fn locate_node_owner(doc: &GltfDocument, target: usize) -> Result<(Option<usize>, Option<usize>, usize), GltfMutationRejection> {
    let mut owners = Vec::new();
    for (parent, node) in doc.nodes.iter().enumerate() {
        for (position, child) in node.children.iter().enumerate() {
            if *child == target {
                owners.push((Some(parent), None, position));
            }
        }
    }
    for (scene, value) in doc.scenes.iter().enumerate() {
        for (position, node) in value.nodes.iter().enumerate() {
            if *node == target {
                owners.push((None, Some(scene), position));
            }
        }
    }
    if owners.len() > 1 {
        return Err(reject("gltf.node.ambiguous-owner", format!("document/nodes/{target}"), "node occurs in multiple hierarchy/root lists"));
    }
    Ok(owners.into_iter().next().unwrap_or((None, None, 0)))
}

/// 🧩️ Executable semantic command owned by its mutation leaf.
pub(crate) trait GltfSemanticMutation {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection>;

    fn plan(&self, base: &GltfSnapshot) -> Result<GltfDiff, GltfMutationRejection> {
        validate_gltf_references(base).map_err(|error| reject("gltf.mutation.invalid-base", error.path.clone(), error.to_string()))?;
        let mut next = base.clone();
        self.apply(&mut next)?;
        validate_gltf_references(&next)?;
        Ok(<GltfDiff as protocol::os_spr::command::DiffAlgebra<GltfSnapshot>>::between(base, &next))
    }
}

impl GltfSemanticMutation for GltfMutation {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        match self {
            GltfMutation::NoMutation(payload) => payload.apply(snapshot),
            GltfMutation::SetSnapshot(payload) => payload.apply(snapshot),
            GltfMutation::SetAsset(payload) => payload.apply(snapshot),
            GltfMutation::InsertScene(payload) => payload.apply(snapshot),
            GltfMutation::RemoveScene(payload) => payload.apply(snapshot),
            GltfMutation::SetScene(payload) => payload.apply(snapshot),
            GltfMutation::InsertNode(payload) => payload.apply(snapshot),
            GltfMutation::RemoveNode(payload) => payload.apply(snapshot),
            GltfMutation::SetNode(payload) => payload.apply(snapshot),
            GltfMutation::InsertMesh(payload) => payload.apply(snapshot),
            GltfMutation::RemoveMesh(payload) => payload.apply(snapshot),
            GltfMutation::SetMesh(payload) => payload.apply(snapshot),
            GltfMutation::InsertAccessor(payload) => payload.apply(snapshot),
            GltfMutation::RemoveAccessor(payload) => payload.apply(snapshot),
            GltfMutation::SetAccessor(payload) => payload.apply(snapshot),
            GltfMutation::InsertMaterial(payload) => payload.apply(snapshot),
            GltfMutation::RemoveMaterial(payload) => payload.apply(snapshot),
            GltfMutation::SetMaterial(payload) => payload.apply(snapshot),
            GltfMutation::InsertBuffer(payload) => payload.apply(snapshot),
            GltfMutation::RemoveBuffer(payload) => payload.apply(snapshot),
            GltfMutation::SetBuffer(payload) => payload.apply(snapshot),
            GltfMutation::InsertAnimation(payload) => payload.apply(snapshot),
            GltfMutation::RemoveAnimation(payload) => payload.apply(snapshot),
            GltfMutation::SetAnimation(payload) => payload.apply(snapshot),
            GltfMutation::TransformNode(payload) => payload.apply(snapshot),
            GltfMutation::ReparentNode(payload) => payload.apply(snapshot),
            GltfMutation::BindNodeMesh(payload) => payload.apply(snapshot),
            GltfMutation::BindPrimitiveMaterial(payload) => payload.apply(snapshot),
        }
    }
}

pub(crate) fn semantic_snapshot(base: &GltfSnapshot, mutation: &GltfMutation) -> Result<GltfSnapshot, GltfMutationRejection> {
    validate_gltf_references(base).map_err(|error| reject("gltf.mutation.invalid-base", error.path.clone(), error.to_string()))?;
    let mut next = base.clone();
    mutation.apply(&mut next)?;
    validate_gltf_references(&next)?;
    Ok(next)
}

/// 🧭 Plans one validated semantic mutation and returns its exact structural diff.
pub fn plan_gltf_mutation(base: &GltfSnapshot, mutation: &GltfMutation) -> Result<GltfDiff, GltfMutationRejection> {
    let next = semantic_snapshot(base, mutation)?;
    Ok(<GltfDiff as protocol::os_spr::command::DiffAlgebra<GltfSnapshot>>::between(base, &next))
}
//#endregion 🛂️SemanticPlanning

//#region 🔖️Apply
/// ▶️ Applies a validated semantic mutation or returns its deterministic rejection.
pub fn apply_gltf_mutation(snapshot: &mut GltfSnapshot, mutation: &GltfMutation) -> Result<GltfDiff, GltfMutationRejection> {
    let diff = plan_gltf_mutation(snapshot, mutation)?;
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    Ok(diff)
}
//#endregion 🔖️Apply
