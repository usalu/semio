//! 🧬️ GltfMutation — document mutation dispatch. Ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, F4: real named variants
//! (Set/Insert/Remove per highest-value array, per the recipe's explicit priority list) replacing
//! the `{ NoMutation, SetSnapshot }` stub. Every variant's `diff()` is handcrafted directly
//! (constructs the sparse [`GltfDiff`] by hand) — apply-and-capture is banned.

/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` grammar primitives + value codecs, reused verbatim from
/// `🔺️diff/component.rs`'s `HandcraftedDiffCodec` region (`enc_gltf_snapshot`/`dec_gltf_snapshot`
/// needs every one of these; SvgMutation reuses SvgDiff's the same way) — see that region's doc
/// comment for the full derive-rejection citation shared by both sides of this artifact.
use crate::artifacts::gltf::schema::diff::{
    dec_accessor, dec_animation, dec_asset, dec_buffer, dec_bytes, dec_gltf_snapshot, dec_material, dec_mesh, dec_node, dec_scene, enc_accessor, enc_animation, enc_asset, enc_buffer, enc_bytes, enc_gltf_snapshot, enc_material, enc_mesh, enc_node,
    enc_scene,
};
use crate::artifacts::gltf::schema::diff::GltfDiff;
/// 🧪️ P2-FG3: real binary value codecs for `GltfMutation`'s `OpBinary` — reused verbatim from
/// `🔺️diff/component.rs`'s `RealBinary*` regions (same intra-artifact reuse the TEXT `enc_*`/
/// `dec_*` imports above already establish).
use crate::artifacts::gltf::schema::diff::{
    gltf_bin_err, read_bin_accessor, read_bin_animation, read_bin_asset, read_bin_blob, read_bin_buffer, read_bin_gltf_snapshot, read_bin_material, read_bin_mesh, read_bin_node, read_bin_scene, write_bin_accessor, write_bin_animation,
    read_bin_option, write_bin_asset, write_bin_blob, write_bin_buffer, write_bin_gltf_snapshot, write_bin_material, write_bin_mesh, write_bin_node, write_bin_option, write_bin_scene,
};
use crate::artifacts::gltf::schema::snapshot::{GltfAccessor, GltfAnimation, GltfAsset, GltfBuffer, GltfDocument, GltfMaterial, GltfMesh, GltfNode, GltfScene};
use crate::artifacts::gltf::GltfSnapshot;
use protocol::{Mutation, OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.gltf`. Highest-value arrays per the recipe (scenes,
/// nodes, meshes, accessors, materials, buffers, animations) get real Insert/Remove/Set triads;
/// the remaining arrays (bufferViews, textures, images, samplers, skins, cameras) are reachable
/// only via `SetSnapshot` in this wave -- see `deviations` in the wave report.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GltfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: GltfSnapshot,
    },
    SetAsset {
        asset: GltfAsset,
    },

    InsertScene {
        index: usize,
        scene: GltfScene,
    },
    RemoveScene {
        index: usize,
    },
    SetScene {
        index: usize,
        scene: GltfScene,
    },

    InsertNode {
        index: usize,
        node: GltfNode,
    },
    RemoveNode {
        index: usize,
    },
    SetNode {
        index: usize,
        node: GltfNode,
    },
    TransformNode {
        index: usize,
        matrix: Option<[f64; 16]>,
        translation: Option<[f64; 3]>,
        rotation: Option<[f64; 4]>,
        scale: Option<[f64; 3]>,
    },
    ReparentNode {
        index: usize,
        parent: Option<usize>,
        scene: Option<usize>,
        position: usize,
    },
    BindNodeMesh {
        index: usize,
        mesh: Option<usize>,
    },

    InsertMesh {
        index: usize,
        mesh: GltfMesh,
    },
    RemoveMesh {
        index: usize,
    },
    SetMesh {
        index: usize,
        mesh: GltfMesh,
    },

    InsertAccessor {
        index: usize,
        accessor: GltfAccessor,
    },
    RemoveAccessor {
        index: usize,
    },
    SetAccessor {
        index: usize,
        accessor: GltfAccessor,
    },

    InsertMaterial {
        index: usize,
        material: GltfMaterial,
    },
    RemoveMaterial {
        index: usize,
    },
    SetMaterial {
        index: usize,
        material: GltfMaterial,
    },
    BindPrimitiveMaterial {
        mesh: usize,
        primitive: usize,
        material: Option<usize>,
    },

    /// 📦️ Touches BOTH `document.buffers[index]` (metadata) and `GltfSnapshot::buffers[index]`
    /// (raw payload bytes) together -- they are two index-aligned collections (per the recipe's
    /// explicit "buffers: Vec<Vec<u8>> stays as-is" instruction), kept in sync by this one
    /// mutation the same way the builder's `add_buffer` already couples them.
    InsertBuffer {
        index: usize,
        buffer: GltfBuffer,
        bytes: Vec<u8>,
    },
    RemoveBuffer {
        index: usize,
    },
    SetBuffer {
        index: usize,
        buffer: GltfBuffer,
        bytes: Vec<u8>,
    },

    InsertAnimation {
        index: usize,
        animation: GltfAnimation,
    },
    RemoveAnimation {
        index: usize,
    },
    SetAnimation {
        index: usize,
        animation: GltfAnimation,
    },
}
//#endregion 🔖️Mutations

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

fn reject(code: &str, path: impl Into<String>, detail: impl Into<String>) -> GltfMutationRejection {
    GltfMutationRejection { code: code.into(), path: path.into(), detail: detail.into() }
}

fn check_index(path: impl Into<String>, index: usize, len: usize) -> Result<(), GltfMutationRejection> {
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
enum IndexFamily {
    Scene,
    Node,
    Mesh,
    Accessor,
    Material,
    Buffer,
}

fn shift_insert(index: &mut usize, at: usize) {
    if *index >= at {
        *index += 1;
    }
}

fn shift_remove(index: &mut usize, at: usize) {
    if *index > at {
        *index -= 1;
    }
}

fn remap_references(doc: &mut GltfDocument, family: IndexFamily, at: usize, inserting: bool) {
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

fn remove_checked<T>(items: &mut Vec<T>, family: IndexFamily, index: usize, doc: &GltfDocument, path: &str) -> Result<T, GltfMutationRejection> {
    check_index(path, index, items.len())?;
    if let Some(reference) = reference_to(doc, family, index) {
        return Err(reject("gltf.reference.in-use", path, format!("referenced by {reference}")));
    }
    Ok(items.remove(index))
}

fn locate_node_owner(doc: &GltfDocument, target: usize) -> Result<(Option<usize>, Option<usize>, usize), GltfMutationRejection> {
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

fn semantic_snapshot(base: &GltfSnapshot, mutation: &GltfMutation) -> Result<GltfSnapshot, GltfMutationRejection> {
    validate_gltf_references(base).map_err(|error| reject("gltf.mutation.invalid-base", error.path.clone(), error.to_string()))?;
    let mut next = base.clone();
    let doc = &mut next.document;
    match mutation {
        GltfMutation::NoMutation => return Ok(next),
        GltfMutation::SetSnapshot { snapshot } => {
            validate_gltf_references(snapshot)?;
            return Ok(snapshot.clone());
        }
        GltfMutation::SetAsset { asset } => doc.asset = asset.clone(),
        GltfMutation::InsertScene { index, scene } => {
            if *index > doc.scenes.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", format!("index {index}, length {}", doc.scenes.len())));
            }
            remap_references(doc, IndexFamily::Scene, *index, true);
            doc.scenes.insert(*index, scene.clone());
        }
        GltfMutation::RemoveScene { index } => {
            let frozen = doc.clone();
            remove_checked(&mut doc.scenes, IndexFamily::Scene, *index, &frozen, "document/scenes")?;
            remap_references(doc, IndexFamily::Scene, *index, false);
        }
        GltfMutation::SetScene { index, scene } => {
            check_index("document/scenes", *index, doc.scenes.len())?;
            doc.scenes[*index] = scene.clone();
        }
        GltfMutation::InsertNode { index, node } => {
            if *index > doc.nodes.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", "document/nodes", format!("index {index}, length {}", doc.nodes.len())));
            }
            remap_references(doc, IndexFamily::Node, *index, true);
            let mut node = node.clone();
            node.children.iter_mut().for_each(|child| shift_insert(child, *index));
            doc.nodes.insert(*index, node);
        }
        GltfMutation::RemoveNode { index } => {
            let frozen = doc.clone();
            remove_checked(&mut doc.nodes, IndexFamily::Node, *index, &frozen, "document/nodes")?;
            remap_references(doc, IndexFamily::Node, *index, false);
        }
        GltfMutation::SetNode { index, node } => {
            check_index("document/nodes", *index, doc.nodes.len())?;
            doc.nodes[*index] = node.clone();
        }
        GltfMutation::TransformNode { index, matrix, translation, rotation, scale } => {
            check_index("document/nodes", *index, doc.nodes.len())?;
            if matrix.is_some() && (translation.is_some() || rotation.is_some() || scale.is_some()) {
                return Err(reject("gltf.node.transform-exclusive", format!("document/nodes/{index}"), "matrix and TRS cannot coexist"));
            }
            if matrix.iter().flatten().chain(translation.iter().flatten()).chain(rotation.iter().flatten()).chain(scale.iter().flatten()).any(|value| !value.is_finite()) {
                return Err(reject("gltf.node.transform-nonfinite", format!("document/nodes/{index}"), "transform contains a non-finite number"));
            }
            let node = &mut doc.nodes[*index];
            node.matrix = *matrix;
            node.translation = *translation;
            node.rotation = *rotation;
            node.scale = *scale;
        }
        GltfMutation::ReparentNode { index, parent, scene, position } => {
            check_index("document/nodes", *index, doc.nodes.len())?;
            if parent.is_some() && scene.is_some() {
                return Err(reject("gltf.node.owner-exclusive", format!("document/nodes/{index}"), "parent and scene cannot both be selected"));
            }
            if let Some(parent) = parent {
                check_index("document/nodes", *parent, doc.nodes.len())?;
                if *parent == *index {
                    return Err(reject("gltf.node.self-parent", format!("document/nodes/{index}"), "node cannot parent itself"));
                }
            }
            if let Some(scene) = scene {
                check_index("document/scenes", *scene, doc.scenes.len())?;
            }
            locate_node_owner(doc, *index)?;
            for node in &mut doc.nodes {
                node.children.retain(|child| *child != *index);
            }
            for root in &mut doc.scenes {
                root.nodes.retain(|node| *node != *index);
            }
            if let Some(parent) = parent {
                if *position > doc.nodes[*parent].children.len() {
                    return Err(reject("gltf.mutation.insert-out-of-range", format!("document/nodes/{parent}/children"), format!("position {position}, length {}", doc.nodes[*parent].children.len())));
                }
                doc.nodes[*parent].children.insert(*position, *index);
            } else if let Some(scene) = scene {
                if *position > doc.scenes[*scene].nodes.len() {
                    return Err(reject("gltf.mutation.insert-out-of-range", format!("document/scenes/{scene}/nodes"), format!("position {position}, length {}", doc.scenes[*scene].nodes.len())));
                }
                doc.scenes[*scene].nodes.insert(*position, *index);
            }
        }
        GltfMutation::BindNodeMesh { index, mesh } => {
            check_index("document/nodes", *index, doc.nodes.len())?;
            if let Some(mesh) = mesh {
                check_index("document/meshes", *mesh, doc.meshes.len())?;
            }
            doc.nodes[*index].mesh = *mesh;
        }
        GltfMutation::InsertMesh { index, mesh } => {
            if *index > doc.meshes.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", "document/meshes", format!("index {index}, length {}", doc.meshes.len())));
            }
            remap_references(doc, IndexFamily::Mesh, *index, true);
            doc.meshes.insert(*index, mesh.clone());
        }
        GltfMutation::RemoveMesh { index } => {
            let frozen = doc.clone();
            remove_checked(&mut doc.meshes, IndexFamily::Mesh, *index, &frozen, "document/meshes")?;
            remap_references(doc, IndexFamily::Mesh, *index, false);
        }
        GltfMutation::SetMesh { index, mesh } => {
            check_index("document/meshes", *index, doc.meshes.len())?;
            doc.meshes[*index] = mesh.clone();
        }
        GltfMutation::InsertAccessor { index, accessor } => {
            if *index > doc.accessors.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", "document/accessors", format!("index {index}, length {}", doc.accessors.len())));
            }
            remap_references(doc, IndexFamily::Accessor, *index, true);
            doc.accessors.insert(*index, accessor.clone());
        }
        GltfMutation::RemoveAccessor { index } => {
            let frozen = doc.clone();
            remove_checked(&mut doc.accessors, IndexFamily::Accessor, *index, &frozen, "document/accessors")?;
            remap_references(doc, IndexFamily::Accessor, *index, false);
        }
        GltfMutation::SetAccessor { index, accessor } => {
            check_index("document/accessors", *index, doc.accessors.len())?;
            doc.accessors[*index] = accessor.clone();
        }
        GltfMutation::InsertMaterial { index, material } => {
            if *index > doc.materials.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", "document/materials", format!("index {index}, length {}", doc.materials.len())));
            }
            remap_references(doc, IndexFamily::Material, *index, true);
            doc.materials.insert(*index, material.clone());
        }
        GltfMutation::RemoveMaterial { index } => {
            let frozen = doc.clone();
            remove_checked(&mut doc.materials, IndexFamily::Material, *index, &frozen, "document/materials")?;
            remap_references(doc, IndexFamily::Material, *index, false);
        }
        GltfMutation::SetMaterial { index, material } => {
            check_index("document/materials", *index, doc.materials.len())?;
            doc.materials[*index] = material.clone();
        }
        GltfMutation::BindPrimitiveMaterial { mesh, primitive, material } => {
            check_index("document/meshes", *mesh, doc.meshes.len())?;
            check_index(format!("document/meshes/{mesh}/primitives"), *primitive, doc.meshes[*mesh].primitives.len())?;
            if let Some(material) = material {
                check_index("document/materials", *material, doc.materials.len())?;
            }
            doc.meshes[*mesh].primitives[*primitive].material = *material;
        }
        GltfMutation::InsertBuffer { index, buffer, bytes } => {
            if *index > doc.buffers.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", "document/buffers", format!("index {index}, length {}", doc.buffers.len())));
            }
            remap_references(doc, IndexFamily::Buffer, *index, true);
            doc.buffers.insert(*index, buffer.clone());
            next.buffers.insert(*index, bytes.clone());
        }
        GltfMutation::RemoveBuffer { index } => {
            let frozen = doc.clone();
            remove_checked(&mut doc.buffers, IndexFamily::Buffer, *index, &frozen, "document/buffers")?;
            next.buffers.remove(*index);
            remap_references(doc, IndexFamily::Buffer, *index, false);
        }
        GltfMutation::SetBuffer { index, buffer, bytes } => {
            check_index("document/buffers", *index, doc.buffers.len())?;
            doc.buffers[*index] = buffer.clone();
            next.buffers[*index] = bytes.clone();
        }
        GltfMutation::InsertAnimation { index, animation } => {
            if *index > doc.animations.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", "document/animations", format!("index {index}, length {}", doc.animations.len())));
            }
            doc.animations.insert(*index, animation.clone());
        }
        GltfMutation::RemoveAnimation { index } => {
            check_index("document/animations", *index, doc.animations.len())?;
            doc.animations.remove(*index);
        }
        GltfMutation::SetAnimation { index, animation } => {
            check_index("document/animations", *index, doc.animations.len())?;
            doc.animations[*index] = animation.clone();
        }
    }
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

//#region 🔖️MutationTrait
impl Mutation<GltfSnapshot> for GltfMutation {
    type Diff = GltfDiff;

    fn diff(&self, base: &GltfSnapshot) -> Self::Diff {
        plan_gltf_mutation(base, self).unwrap_or_default()
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<Self> {
        if plan_gltf_mutation(base, self).is_err() {
            return Vec::new();
        }
        let doc = &base.document;
        match self {
            GltfMutation::NoMutation => Vec::new(),
            GltfMutation::SetSnapshot { .. } => vec![GltfMutation::SetSnapshot { snapshot: base.clone() }],
            GltfMutation::SetAsset { .. } => vec![GltfMutation::SetAsset { asset: doc.asset.clone() }],

            GltfMutation::InsertScene { index, .. } => vec![GltfMutation::RemoveScene { index: *index }],
            GltfMutation::RemoveScene { index } => match doc.scenes.get(*index) {
                Some(scene) => vec![GltfMutation::InsertScene { index: *index, scene: scene.clone() }],
                None => Vec::new(),
            },
            GltfMutation::SetScene { index, .. } => match doc.scenes.get(*index) {
                Some(scene) => vec![GltfMutation::SetScene { index: *index, scene: scene.clone() }],
                None => Vec::new(),
            },

            GltfMutation::InsertNode { index, .. } => vec![GltfMutation::RemoveNode { index: *index }],
            GltfMutation::RemoveNode { index } => match doc.nodes.get(*index) {
                Some(node) => vec![GltfMutation::InsertNode { index: *index, node: node.clone() }],
                None => Vec::new(),
            },
            GltfMutation::SetNode { index, .. } => match doc.nodes.get(*index) {
                Some(node) => vec![GltfMutation::SetNode { index: *index, node: node.clone() }],
                None => Vec::new(),
            },
            GltfMutation::TransformNode { index, .. } => {
                let node = &doc.nodes[*index];
                vec![GltfMutation::TransformNode { index: *index, matrix: node.matrix, translation: node.translation, rotation: node.rotation, scale: node.scale }]
            }
            GltfMutation::ReparentNode { index, .. } => match locate_node_owner(doc, *index) {
                Ok((parent, scene, position)) => vec![GltfMutation::ReparentNode { index: *index, parent, scene, position }],
                Err(_) => Vec::new(),
            },
            GltfMutation::BindNodeMesh { index, .. } => vec![GltfMutation::BindNodeMesh { index: *index, mesh: doc.nodes[*index].mesh }],

            GltfMutation::InsertMesh { index, .. } => vec![GltfMutation::RemoveMesh { index: *index }],
            GltfMutation::RemoveMesh { index } => match doc.meshes.get(*index) {
                Some(mesh) => vec![GltfMutation::InsertMesh { index: *index, mesh: mesh.clone() }],
                None => Vec::new(),
            },
            GltfMutation::SetMesh { index, .. } => match doc.meshes.get(*index) {
                Some(mesh) => vec![GltfMutation::SetMesh { index: *index, mesh: mesh.clone() }],
                None => Vec::new(),
            },

            GltfMutation::InsertAccessor { index, .. } => vec![GltfMutation::RemoveAccessor { index: *index }],
            GltfMutation::RemoveAccessor { index } => match doc.accessors.get(*index) {
                Some(accessor) => vec![GltfMutation::InsertAccessor { index: *index, accessor: accessor.clone() }],
                None => Vec::new(),
            },
            GltfMutation::SetAccessor { index, .. } => match doc.accessors.get(*index) {
                Some(accessor) => vec![GltfMutation::SetAccessor { index: *index, accessor: accessor.clone() }],
                None => Vec::new(),
            },

            GltfMutation::InsertMaterial { index, .. } => vec![GltfMutation::RemoveMaterial { index: *index }],
            GltfMutation::RemoveMaterial { index } => match doc.materials.get(*index) {
                Some(material) => vec![GltfMutation::InsertMaterial { index: *index, material: material.clone() }],
                None => Vec::new(),
            },
            GltfMutation::SetMaterial { index, .. } => match doc.materials.get(*index) {
                Some(material) => vec![GltfMutation::SetMaterial { index: *index, material: material.clone() }],
                None => Vec::new(),
            },
            GltfMutation::BindPrimitiveMaterial { mesh, primitive, .. } => vec![GltfMutation::BindPrimitiveMaterial { mesh: *mesh, primitive: *primitive, material: doc.meshes[*mesh].primitives[*primitive].material }],

            GltfMutation::InsertBuffer { index, .. } => vec![GltfMutation::RemoveBuffer { index: *index }],
            GltfMutation::RemoveBuffer { index } => match (doc.buffers.get(*index), base.buffers.get(*index)) {
                (Some(buffer), Some(bytes)) => vec![GltfMutation::InsertBuffer { index: *index, buffer: buffer.clone(), bytes: bytes.clone() }],
                _ => Vec::new(),
            },
            GltfMutation::SetBuffer { index, .. } => match (doc.buffers.get(*index), base.buffers.get(*index)) {
                (Some(buffer), Some(bytes)) => vec![GltfMutation::SetBuffer { index: *index, buffer: buffer.clone(), bytes: bytes.clone() }],
                _ => Vec::new(),
            },

            GltfMutation::InsertAnimation { index, .. } => vec![GltfMutation::RemoveAnimation { index: *index }],
            GltfMutation::RemoveAnimation { index } => match doc.animations.get(*index) {
                Some(animation) => vec![GltfMutation::InsertAnimation { index: *index, animation: animation.clone() }],
                None => Vec::new(),
            },
            GltfMutation::SetAnimation { index, .. } => match doc.animations.get(*index) {
                Some(animation) => vec![GltfMutation::SetAnimation { index: *index, animation: animation.clone() }],
                None => Vec::new(),
            },
        }
    }

    fn validate(&self, snapshot: &GltfSnapshot) -> Result<(), String> {
        plan_gltf_mutation(snapshot, self).map(|_| ()).map_err(|error| error.to_string())
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `GltfMutation` — CONFIRMED by a real
/// `cargo check -p semio-s-plugin-stdio --lib` failure with `#[derive(dsl::DslOps)]` temporarily
/// added to this enum (33 `E0277` errors, captured in `f6-gltf-mutation-derive-check1.txt` in the
/// ticket folder, then reverted): `SetSnapshot{snapshot: GltfSnapshot}` recursively requires
/// `DslField` on `GltfAsset`/`GltfScene`/`GltfNode`/`GltfMesh`/`GltfAccessor`/`GltfMaterial`/
/// `GltfBuffer`/`GltfAnimation`/`GltfSnapshot` itself, none of which are `DslRecord`-derived, and
/// even fully deriving all of them would still fail once the walk reaches `GltfJson`/
/// `GltfCameraProjection` (real data-carrying enums, no `DslField` impl possible — see
/// `🔺️diff/component.rs`'s `HandcraftedDiffCodec` doc comment for the full citation). Reuses the
/// diff module's `pub(crate)` grammar primitives and value codecs (`hex_encode`/`enc_asset`/
/// `enc_scene`/.../`enc_gltf_snapshot`/...) rather than duplicating them a second time in this
/// file — same intra-artifact reuse `SvgMutation` uses off `SvgDiff`. Grammar: `keyword arg=value
/// ...` (space-separated), one match arm per variant, matching the derive's own handcrafted-wrapper
/// convention (`f6-recon-report.md` §2) in shape even though nothing here actually derives
/// `DslVariants`.
fn enc_optional_index(value: Option<usize>) -> String {
    value.map(|value| value.to_string()).unwrap_or_else(|| "-".into())
}

fn dec_optional_index(value: &str) -> Result<Option<usize>, String> {
    if value == "-" { Ok(None) } else { value.parse().map(Some).map_err(|error: std::num::ParseIntError| error.to_string()) }
}

fn enc_optional_array<const N: usize>(value: Option<[f64; N]>) -> String {
    value.map(|values| values.into_iter().map(|value| value.to_bits().to_string()).collect::<Vec<_>>().join(",")).unwrap_or_else(|| "-".into())
}

fn dec_optional_array<const N: usize>(value: &str) -> Result<Option<[f64; N]>, String> {
    if value == "-" {
        return Ok(None);
    }
    let values = value.split(',').map(|part| part.parse::<u64>().map(f64::from_bits).map_err(|error| error.to_string())).collect::<Result<Vec<_>, _>>()?;
    values.try_into().map(Some).map_err(|values: Vec<f64>| format!("expected {N} values, got {}", values.len()))
}

fn print_gltf_mutation(m: &GltfMutation) -> String {
    match m {
        GltfMutation::NoMutation => "no-mutation".to_string(),
        GltfMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_gltf_snapshot(snapshot)),
        GltfMutation::SetAsset { asset } => format!("set-asset asset={}", enc_asset(asset)),

        GltfMutation::InsertScene { index, scene } => format!("insert-scene index={index} scene={}", enc_scene(scene)),
        GltfMutation::RemoveScene { index } => format!("remove-scene index={index}"),
        GltfMutation::SetScene { index, scene } => format!("set-scene index={index} scene={}", enc_scene(scene)),

        GltfMutation::InsertNode { index, node } => format!("insert-node index={index} node={}", enc_node(node)),
        GltfMutation::RemoveNode { index } => format!("remove-node index={index}"),
        GltfMutation::SetNode { index, node } => format!("set-node index={index} node={}", enc_node(node)),
        GltfMutation::TransformNode { index, matrix, translation, rotation, scale } => format!(
            "transform-node index={index} matrix={} translation={} rotation={} scale={}",
            enc_optional_array(*matrix),
            enc_optional_array(*translation),
            enc_optional_array(*rotation),
            enc_optional_array(*scale)
        ),
        GltfMutation::ReparentNode { index, parent, scene, position } => format!("reparent-node index={index} parent={} scene={} position={position}", enc_optional_index(*parent), enc_optional_index(*scene)),
        GltfMutation::BindNodeMesh { index, mesh } => format!("bind-node-mesh index={index} mesh={}", enc_optional_index(*mesh)),

        GltfMutation::InsertMesh { index, mesh } => format!("insert-mesh index={index} mesh={}", enc_mesh(mesh)),
        GltfMutation::RemoveMesh { index } => format!("remove-mesh index={index}"),
        GltfMutation::SetMesh { index, mesh } => format!("set-mesh index={index} mesh={}", enc_mesh(mesh)),

        GltfMutation::InsertAccessor { index, accessor } => format!("insert-accessor index={index} accessor={}", enc_accessor(accessor)),
        GltfMutation::RemoveAccessor { index } => format!("remove-accessor index={index}"),
        GltfMutation::SetAccessor { index, accessor } => format!("set-accessor index={index} accessor={}", enc_accessor(accessor)),

        GltfMutation::InsertMaterial { index, material } => format!("insert-material index={index} material={}", enc_material(material)),
        GltfMutation::RemoveMaterial { index } => format!("remove-material index={index}"),
        GltfMutation::SetMaterial { index, material } => format!("set-material index={index} material={}", enc_material(material)),
        GltfMutation::BindPrimitiveMaterial { mesh, primitive, material } => format!("bind-primitive-material mesh={mesh} primitive={primitive} material={}", enc_optional_index(*material)),

        GltfMutation::InsertBuffer { index, buffer, bytes } => format!("insert-buffer index={index} buffer={} bytes={}", enc_buffer(buffer), enc_bytes(bytes)),
        GltfMutation::RemoveBuffer { index } => format!("remove-buffer index={index}"),
        GltfMutation::SetBuffer { index, buffer, bytes } => format!("set-buffer index={index} buffer={} bytes={}", enc_buffer(buffer), enc_bytes(bytes)),

        GltfMutation::InsertAnimation { index, animation } => format!("insert-animation index={index} animation={}", enc_animation(animation)),
        GltfMutation::RemoveAnimation { index } => format!("remove-animation index={index}"),
        GltfMutation::SetAnimation { index, animation } => format!("set-animation index={index} animation={}", enc_animation(animation)),
    }
}

fn parse_gltf_mutation(line: &str) -> Result<GltfMutation, String> {
    if line == "no-mutation" {
        return Ok(GltfMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("gltf mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("gltf mutation: missing arg '{k}' for '{keyword}'"));
    let idx = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(GltfMutation::SetSnapshot { snapshot: dec_gltf_snapshot(arg("snapshot")?)? }),
        "set-asset" => Ok(GltfMutation::SetAsset { asset: dec_asset(arg("asset")?)? }),

        "insert-scene" => Ok(GltfMutation::InsertScene { index: idx("index")?, scene: dec_scene(arg("scene")?)? }),
        "remove-scene" => Ok(GltfMutation::RemoveScene { index: idx("index")? }),
        "set-scene" => Ok(GltfMutation::SetScene { index: idx("index")?, scene: dec_scene(arg("scene")?)? }),

        "insert-node" => Ok(GltfMutation::InsertNode { index: idx("index")?, node: dec_node(arg("node")?)? }),
        "remove-node" => Ok(GltfMutation::RemoveNode { index: idx("index")? }),
        "set-node" => Ok(GltfMutation::SetNode { index: idx("index")?, node: dec_node(arg("node")?)? }),
        "transform-node" => Ok(GltfMutation::TransformNode {
            index: idx("index")?,
            matrix: dec_optional_array(arg("matrix")?)?,
            translation: dec_optional_array(arg("translation")?)?,
            rotation: dec_optional_array(arg("rotation")?)?,
            scale: dec_optional_array(arg("scale")?)?,
        }),
        "reparent-node" => Ok(GltfMutation::ReparentNode { index: idx("index")?, parent: dec_optional_index(arg("parent")?)?, scene: dec_optional_index(arg("scene")?)?, position: idx("position")? }),
        "bind-node-mesh" => Ok(GltfMutation::BindNodeMesh { index: idx("index")?, mesh: dec_optional_index(arg("mesh")?)? }),

        "insert-mesh" => Ok(GltfMutation::InsertMesh { index: idx("index")?, mesh: dec_mesh(arg("mesh")?)? }),
        "remove-mesh" => Ok(GltfMutation::RemoveMesh { index: idx("index")? }),
        "set-mesh" => Ok(GltfMutation::SetMesh { index: idx("index")?, mesh: dec_mesh(arg("mesh")?)? }),

        "insert-accessor" => Ok(GltfMutation::InsertAccessor { index: idx("index")?, accessor: dec_accessor(arg("accessor")?)? }),
        "remove-accessor" => Ok(GltfMutation::RemoveAccessor { index: idx("index")? }),
        "set-accessor" => Ok(GltfMutation::SetAccessor { index: idx("index")?, accessor: dec_accessor(arg("accessor")?)? }),

        "insert-material" => Ok(GltfMutation::InsertMaterial { index: idx("index")?, material: dec_material(arg("material")?)? }),
        "remove-material" => Ok(GltfMutation::RemoveMaterial { index: idx("index")? }),
        "set-material" => Ok(GltfMutation::SetMaterial { index: idx("index")?, material: dec_material(arg("material")?)? }),
        "bind-primitive-material" => Ok(GltfMutation::BindPrimitiveMaterial { mesh: idx("mesh")?, primitive: idx("primitive")?, material: dec_optional_index(arg("material")?)? }),

        "insert-buffer" => Ok(GltfMutation::InsertBuffer { index: idx("index")?, buffer: dec_buffer(arg("buffer")?)?, bytes: dec_bytes(arg("bytes")?)? }),
        "remove-buffer" => Ok(GltfMutation::RemoveBuffer { index: idx("index")? }),
        "set-buffer" => Ok(GltfMutation::SetBuffer { index: idx("index")?, buffer: dec_buffer(arg("buffer")?)?, bytes: dec_bytes(arg("bytes")?)? }),

        "insert-animation" => Ok(GltfMutation::InsertAnimation { index: idx("index")?, animation: dec_animation(arg("animation")?)? }),
        "remove-animation" => Ok(GltfMutation::RemoveAnimation { index: idx("index")? }),
        "set-animation" => Ok(GltfMutation::SetAnimation { index: idx("index")?, animation: dec_animation(arg("animation")?)? }),

        other => Err(format!("gltf mutation: unknown keyword {other:?}")),
    }
}

/// 🧪️ P2-FG3: representative `GltfMutation` cases — one per variant (28 total, `NoMutation`
/// through `SetAnimation`, `GltfMutation`'s own declaration order) — used by this artifact's own
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests (⚙️engine/component.rs),
/// mirroring json's own `demo_mutation_cases()` role in its pilot report.
pub(crate) fn demo_mutation_cases() -> Vec<GltfMutation> {
    vec![
        GltfMutation::NoMutation,
        GltfMutation::SetSnapshot { snapshot: crate::artifacts::gltf::engine::demo_gltf_snapshot() },
        GltfMutation::SetAsset { asset: GltfAsset { version: "2.1".into(), generator: None, copyright: Some("(c)".into()), min_version: None, extensions: None, extras: None } },
        GltfMutation::InsertScene { index: 1, scene: crate::artifacts::gltf::schema::snapshot::GltfScene { nodes: vec![1], name: Some("s".into()), ..Default::default() } },
        GltfMutation::RemoveScene { index: 0 },
        GltfMutation::SetScene { index: 0, scene: crate::artifacts::gltf::schema::snapshot::GltfScene { nodes: vec![9], name: None, ..Default::default() } },
        GltfMutation::InsertNode { index: 1, node: GltfNode { mesh: Some(1), matrix: Some([0.0; 16]), ..GltfNode::default() } },
        GltfMutation::RemoveNode { index: 0 },
        GltfMutation::SetNode { index: 0, node: GltfNode { mesh: None, camera: Some(2), name: Some("n".into()), ..GltfNode::default() } },
        GltfMutation::TransformNode { index: 0, matrix: None, translation: Some([1.0, 2.0, 3.0]), rotation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some([2.0, 2.0, 2.0]) },
        GltfMutation::ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 },
        GltfMutation::BindNodeMesh { index: 0, mesh: Some(1) },
        GltfMutation::InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } },
        GltfMutation::RemoveMesh { index: 0 },
        GltfMutation::SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } },
        GltfMutation::InsertAccessor {
            index: 0,
            accessor: GltfAccessor {
                buffer_view: None,
                byte_offset: 0,
                component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte,
                normalized: false,
                count: 1,
                kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar,
                max: None,
                min: None,
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            },
        },
        GltfMutation::RemoveAccessor { index: 0 },
        GltfMutation::SetAccessor {
            index: 0,
            accessor: GltfAccessor {
                buffer_view: Some(0),
                byte_offset: 4,
                component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                normalized: true,
                count: 9,
                kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                max: Some(vec![1.0]),
                min: Some(vec![-1.0]),
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            },
        },
        GltfMutation::InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), double_sided: true, ..GltfMaterial::default() } },
        GltfMutation::RemoveMaterial { index: 0 },
        GltfMutation::SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } },
        GltfMutation::BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) },
        GltfMutation::InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 2, uri: Some("data:...".into()), name: None, extensions: None, extras: None }, bytes: vec![7, 8] },
        GltfMutation::RemoveBuffer { index: 0 },
        GltfMutation::SetBuffer { index: 0, buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] },
        GltfMutation::InsertAnimation { index: 0, animation: GltfAnimation { name: Some("a".into()), ..GltfAnimation::default() } },
        GltfMutation::RemoveAnimation { index: 0 },
        GltfMutation::SetAnimation { index: 0, animation: GltfAnimation { name: Some("renamed-anim".into()), ..GltfAnimation::default() } },
    ]
}

impl protocol::OpText for GltfMutation {
    fn print_op(&self) -> String {
        print_gltf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_gltf_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ P2-FG3: real binary op-frame — upgraded from the F6-era `print_op().into_bytes()` text-as-
/// binary shortcut (18 standards, gltf among them, were still on this shortcut per the P2-W0
/// census). Matches `../💾️binary/📡️component.protocol.semio`'s real fixed header exactly:
/// `format u8` (the repo-wide `store::pack_rt::OP_BINARY_FORMAT` convention byte) + `tag u8` (this
/// variant's own ordinal, `GltfMutation`'s declaration order, `NoMutation`=0) — both individually,
/// genuinely protocol-walkable — then one opaque `payload bytes` tail (`§2.5`'s recursive/opaque-
/// tail pattern: the payload itself IS real, fully structured binary on the Rust side via this
/// artifact's own `write_bin_*`/`read_bin_*` value codecs, just not further protocol-walkable past
/// the fixed 2-byte header, `protocol-prim-ref-recursion`).
fn write_bin_array<const N: usize>(writer: &mut dsl::ByteWriter, values: &[f64; N]) {
    for value in values {
        writer.write_f64_le(*value);
    }
}

fn read_bin_array<const N: usize>(reader: &mut dsl::ByteReader) -> Result<[f64; N], dsl::PackError> {
    let mut values = [0.0; N];
    for value in &mut values {
        *value = reader.read_f64_le()?;
    }
    Ok(values)
}

impl protocol::OpBinary for GltfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        w.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        let tag: u8 = match self {
            GltfMutation::NoMutation => 0,
            GltfMutation::SetSnapshot { .. } => 1,
            GltfMutation::SetAsset { .. } => 2,
            GltfMutation::InsertScene { .. } => 3,
            GltfMutation::RemoveScene { .. } => 4,
            GltfMutation::SetScene { .. } => 5,
            GltfMutation::InsertNode { .. } => 6,
            GltfMutation::RemoveNode { .. } => 7,
            GltfMutation::SetNode { .. } => 8,
            GltfMutation::InsertMesh { .. } => 9,
            GltfMutation::RemoveMesh { .. } => 10,
            GltfMutation::SetMesh { .. } => 11,
            GltfMutation::InsertAccessor { .. } => 12,
            GltfMutation::RemoveAccessor { .. } => 13,
            GltfMutation::SetAccessor { .. } => 14,
            GltfMutation::InsertMaterial { .. } => 15,
            GltfMutation::RemoveMaterial { .. } => 16,
            GltfMutation::SetMaterial { .. } => 17,
            GltfMutation::InsertBuffer { .. } => 18,
            GltfMutation::RemoveBuffer { .. } => 19,
            GltfMutation::SetBuffer { .. } => 20,
            GltfMutation::InsertAnimation { .. } => 21,
            GltfMutation::RemoveAnimation { .. } => 22,
            GltfMutation::SetAnimation { .. } => 23,
            GltfMutation::TransformNode { .. } => 24,
            GltfMutation::ReparentNode { .. } => 25,
            GltfMutation::BindNodeMesh { .. } => 26,
            GltfMutation::BindPrimitiveMaterial { .. } => 27,
        };
        w.write_u8(tag);
        match self {
            GltfMutation::NoMutation => {}
            GltfMutation::SetSnapshot { snapshot } => write_bin_gltf_snapshot(&mut w, snapshot),
            GltfMutation::SetAsset { asset } => write_bin_asset(&mut w, asset),
            GltfMutation::InsertScene { index, scene } => {
                w.write_varint_u64(*index as u64);
                write_bin_scene(&mut w, scene);
            }
            GltfMutation::RemoveScene { index } => w.write_varint_u64(*index as u64),
            GltfMutation::SetScene { index, scene } => {
                w.write_varint_u64(*index as u64);
                write_bin_scene(&mut w, scene);
            }
            GltfMutation::InsertNode { index, node } => {
                w.write_varint_u64(*index as u64);
                write_bin_node(&mut w, node);
            }
            GltfMutation::RemoveNode { index } => w.write_varint_u64(*index as u64),
            GltfMutation::SetNode { index, node } => {
                w.write_varint_u64(*index as u64);
                write_bin_node(&mut w, node);
            }
            GltfMutation::TransformNode { index, matrix, translation, rotation, scale } => {
                w.write_varint_u64(*index as u64);
                write_bin_option(&mut w, matrix, write_bin_array);
                write_bin_option(&mut w, translation, write_bin_array);
                write_bin_option(&mut w, rotation, write_bin_array);
                write_bin_option(&mut w, scale, write_bin_array);
            }
            GltfMutation::ReparentNode { index, parent, scene, position } => {
                w.write_varint_u64(*index as u64);
                write_bin_option(&mut w, parent, |w, value| w.write_varint_u64(*value as u64));
                write_bin_option(&mut w, scene, |w, value| w.write_varint_u64(*value as u64));
                w.write_varint_u64(*position as u64);
            }
            GltfMutation::BindNodeMesh { index, mesh } => {
                w.write_varint_u64(*index as u64);
                write_bin_option(&mut w, mesh, |w, value| w.write_varint_u64(*value as u64));
            }
            GltfMutation::InsertMesh { index, mesh } => {
                w.write_varint_u64(*index as u64);
                write_bin_mesh(&mut w, mesh);
            }
            GltfMutation::RemoveMesh { index } => w.write_varint_u64(*index as u64),
            GltfMutation::SetMesh { index, mesh } => {
                w.write_varint_u64(*index as u64);
                write_bin_mesh(&mut w, mesh);
            }
            GltfMutation::InsertAccessor { index, accessor } => {
                w.write_varint_u64(*index as u64);
                write_bin_accessor(&mut w, accessor);
            }
            GltfMutation::RemoveAccessor { index } => w.write_varint_u64(*index as u64),
            GltfMutation::SetAccessor { index, accessor } => {
                w.write_varint_u64(*index as u64);
                write_bin_accessor(&mut w, accessor);
            }
            GltfMutation::InsertMaterial { index, material } => {
                w.write_varint_u64(*index as u64);
                write_bin_material(&mut w, material);
            }
            GltfMutation::RemoveMaterial { index } => w.write_varint_u64(*index as u64),
            GltfMutation::SetMaterial { index, material } => {
                w.write_varint_u64(*index as u64);
                write_bin_material(&mut w, material);
            }
            GltfMutation::BindPrimitiveMaterial { mesh, primitive, material } => {
                w.write_varint_u64(*mesh as u64);
                w.write_varint_u64(*primitive as u64);
                write_bin_option(&mut w, material, |w, value| w.write_varint_u64(*value as u64));
            }
            GltfMutation::InsertBuffer { index, buffer, bytes } => {
                w.write_varint_u64(*index as u64);
                write_bin_buffer(&mut w, buffer);
                write_bin_blob(&mut w, bytes);
            }
            GltfMutation::RemoveBuffer { index } => w.write_varint_u64(*index as u64),
            GltfMutation::SetBuffer { index, buffer, bytes } => {
                w.write_varint_u64(*index as u64);
                write_bin_buffer(&mut w, buffer);
                write_bin_blob(&mut w, bytes);
            }
            GltfMutation::InsertAnimation { index, animation } => {
                w.write_varint_u64(*index as u64);
                write_bin_animation(&mut w, animation);
            }
            GltfMutation::RemoveAnimation { index } => w.write_varint_u64(*index as u64),
            GltfMutation::SetAnimation { index, animation } => {
                w.write_varint_u64(*index as u64);
                write_bin_animation(&mut w, animation);
            }
        }
        Ok(w.into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let format = r.read_u8().map_err(gltf_bin_err)?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "gltf op format", offset: 0, detail: format!("expected format {}, got {format}", store::pack_rt::OP_BINARY_FORMAT) });
        }
        let tag = r.read_u8().map_err(gltf_bin_err)?;
        let idx = |r: &mut dsl::ByteReader| -> Result<usize, protocol::ProtocolError> { Ok(r.read_varint_u64().map_err(gltf_bin_err)? as usize) };
        let mutation = match tag {
            0 => GltfMutation::NoMutation,
            1 => GltfMutation::SetSnapshot { snapshot: read_bin_gltf_snapshot(&mut r).map_err(gltf_bin_err)? },
            2 => GltfMutation::SetAsset { asset: read_bin_asset(&mut r).map_err(gltf_bin_err)? },
            3 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertScene { index, scene: read_bin_scene(&mut r).map_err(gltf_bin_err)? }
            }
            4 => GltfMutation::RemoveScene { index: idx(&mut r)? },
            5 => {
                let index = idx(&mut r)?;
                GltfMutation::SetScene { index, scene: read_bin_scene(&mut r).map_err(gltf_bin_err)? }
            }
            6 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertNode { index, node: read_bin_node(&mut r).map_err(gltf_bin_err)? }
            }
            7 => GltfMutation::RemoveNode { index: idx(&mut r)? },
            8 => {
                let index = idx(&mut r)?;
                GltfMutation::SetNode { index, node: read_bin_node(&mut r).map_err(gltf_bin_err)? }
            }
            9 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertMesh { index, mesh: read_bin_mesh(&mut r).map_err(gltf_bin_err)? }
            }
            10 => GltfMutation::RemoveMesh { index: idx(&mut r)? },
            11 => {
                let index = idx(&mut r)?;
                GltfMutation::SetMesh { index, mesh: read_bin_mesh(&mut r).map_err(gltf_bin_err)? }
            }
            12 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertAccessor { index, accessor: read_bin_accessor(&mut r).map_err(gltf_bin_err)? }
            }
            13 => GltfMutation::RemoveAccessor { index: idx(&mut r)? },
            14 => {
                let index = idx(&mut r)?;
                GltfMutation::SetAccessor { index, accessor: read_bin_accessor(&mut r).map_err(gltf_bin_err)? }
            }
            15 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertMaterial { index, material: read_bin_material(&mut r).map_err(gltf_bin_err)? }
            }
            16 => GltfMutation::RemoveMaterial { index: idx(&mut r)? },
            17 => {
                let index = idx(&mut r)?;
                GltfMutation::SetMaterial { index, material: read_bin_material(&mut r).map_err(gltf_bin_err)? }
            }
            18 => {
                let index = idx(&mut r)?;
                let buffer = read_bin_buffer(&mut r).map_err(gltf_bin_err)?;
                let bytes = read_bin_blob(&mut r).map_err(gltf_bin_err)?;
                GltfMutation::InsertBuffer { index, buffer, bytes }
            }
            19 => GltfMutation::RemoveBuffer { index: idx(&mut r)? },
            20 => {
                let index = idx(&mut r)?;
                let buffer = read_bin_buffer(&mut r).map_err(gltf_bin_err)?;
                let bytes = read_bin_blob(&mut r).map_err(gltf_bin_err)?;
                GltfMutation::SetBuffer { index, buffer, bytes }
            }
            21 => {
                let index = idx(&mut r)?;
                GltfMutation::InsertAnimation { index, animation: read_bin_animation(&mut r).map_err(gltf_bin_err)? }
            }
            22 => GltfMutation::RemoveAnimation { index: idx(&mut r)? },
            23 => {
                let index = idx(&mut r)?;
                GltfMutation::SetAnimation { index, animation: read_bin_animation(&mut r).map_err(gltf_bin_err)? }
            }
            24 => GltfMutation::TransformNode {
                index: idx(&mut r)?,
                matrix: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
                translation: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
                rotation: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
                scale: read_bin_option(&mut r, read_bin_array).map_err(gltf_bin_err)?,
            },
            25 => GltfMutation::ReparentNode {
                index: idx(&mut r)?,
                parent: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)?,
                scene: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)?,
                position: idx(&mut r)?,
            },
            26 => GltfMutation::BindNodeMesh { index: idx(&mut r)?, mesh: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)? },
            27 => GltfMutation::BindPrimitiveMaterial {
                mesh: idx(&mut r)?,
                primitive: idx(&mut r)?,
                material: read_bin_option(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)?,
            },
            other => return Err(protocol::ProtocolError::Malformed { what: "gltf op tag", offset: 0, detail: format!("unknown tag {other}") }),
        };
        if r.remaining() != 0 {
            return Err(protocol::ProtocolError::Malformed { what: "gltf op trailing bytes", offset: (bytes.len() - r.remaining()) as u64, detail: format!("{} trailing bytes", r.remaining()) });
        }
        Ok(mutation)
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::schema::snapshot::GltfDocument;
    use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
    use protocol::MutationDiff;

    fn base_snapshot() -> GltfSnapshot {
        GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset { version: "2.0".into(), ..GltfAsset::default() },
                scenes: vec![GltfScene { nodes: vec![0], name: Some("s0".into()), ..GltfScene::default() }],
                nodes: vec![GltfNode { mesh: Some(0), ..GltfNode::default() }, GltfNode::default()],
                meshes: vec![GltfMesh { primitives: vec![Default::default()], ..Default::default() }, GltfMesh::default()],
                accessors: vec![GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                    normalized: false,
                    count: 3,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                }],
                materials: vec![GltfMaterial::default()],
                buffers: vec![GltfBuffer { byte_length: 4, uri: None, name: None, extensions: None, extras: None }],
                animations: vec![GltfAnimation::default()],
                ..GltfDocument::default()
            },
            buffers: vec![vec![1, 2, 3, 4]],
            source_form: crate::artifacts::gltf::schema::snapshot::GltfSourceForm::Json,
        }
    }

    /// 🧪️ `mutation_diff_law`: ∀ variant, `m.diff(base).apply(base) == { apply_gltf_mutation(&mut
    /// s, m); s }`, and the returned diff equals `m.diff(base)`.
    #[test]
    fn mutation_diff_law_holds_for_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            GltfMutation::NoMutation,
            GltfMutation::SetAsset { asset: GltfAsset { version: "2.1".into(), ..GltfAsset::default() } },
            GltfMutation::InsertScene { index: 1, scene: GltfScene { nodes: vec![1], ..GltfScene::default() } },
            GltfMutation::RemoveScene { index: 0 },
            GltfMutation::SetScene { index: 0, scene: GltfScene { nodes: vec![1], name: Some("renamed".into()), ..GltfScene::default() } },
            GltfMutation::InsertNode { index: 1, node: GltfNode { mesh: Some(1), ..GltfNode::default() } },
            GltfMutation::RemoveNode { index: 1 },
            GltfMutation::SetNode { index: 0, node: GltfNode { mesh: None, name: Some("n".into()), ..GltfNode::default() } },
            GltfMutation::TransformNode { index: 1, matrix: None, translation: Some([1.0, 2.0, 3.0]), rotation: None, scale: None },
            GltfMutation::ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 },
            GltfMutation::BindNodeMesh { index: 1, mesh: Some(0) },
            GltfMutation::InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } },
            GltfMutation::RemoveMesh { index: 1 },
            GltfMutation::SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } },
            GltfMutation::InsertAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte,
                    normalized: false,
                    count: 1,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            },
            GltfMutation::RemoveAccessor { index: 0 },
            GltfMutation::SetAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                    normalized: false,
                    count: 9,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            },
            GltfMutation::InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), ..GltfMaterial::default() } },
            GltfMutation::RemoveMaterial { index: 0 },
            GltfMutation::SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } },
            GltfMutation::BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) },
            GltfMutation::InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 2, uri: None, name: None, extensions: None, extras: None }, bytes: vec![7, 8] },
            GltfMutation::RemoveBuffer { index: 0 },
            GltfMutation::SetBuffer { index: 0, buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] },
            GltfMutation::InsertAnimation { index: 0, animation: GltfAnimation { name: Some("a".into()), ..GltfAnimation::default() } },
            GltfMutation::RemoveAnimation { index: 0 },
            GltfMutation::SetAnimation { index: 0, animation: GltfAnimation { name: Some("renamed-anim".into()), ..GltfAnimation::default() } },
        ];
        for m in variants {
            let expected_diff = m.diff(&base);
            let mut s = base.clone();
            let actual_diff = apply_gltf_mutation(&mut s, &m).expect("valid mutation");
            assert_eq!(actual_diff, expected_diff, "diff mismatch for mutation {m:?}");
            assert_eq!(s, MutationDiff::apply(&expected_diff, &base), "apply(base) mismatch for mutation {m:?}");
        }
    }

    /// 🧪️ `inverse_law` (mutation level): every variant's `inverse(base)` round-trips.
    #[test]
    fn inverse_law_mutation_level_round_trips_for_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            GltfMutation::SetAsset { asset: GltfAsset { version: "9.9".into(), ..GltfAsset::default() } },
            GltfMutation::InsertScene { index: 0, scene: GltfScene { nodes: vec![1], ..GltfScene::default() } },
            GltfMutation::RemoveScene { index: 0 },
            GltfMutation::SetScene { index: 0, scene: GltfScene { nodes: vec![1], name: Some("z".into()), ..GltfScene::default() } },
            GltfMutation::InsertNode { index: 0, node: GltfNode { mesh: Some(0), ..GltfNode::default() } },
            GltfMutation::RemoveNode { index: 1 },
            GltfMutation::SetNode { index: 1, node: GltfNode { mesh: None, ..GltfNode::default() } },
            GltfMutation::TransformNode { index: 1, matrix: None, translation: Some([4.0, 5.0, 6.0]), rotation: None, scale: None },
            GltfMutation::ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 },
            GltfMutation::BindNodeMesh { index: 1, mesh: Some(0) },
            GltfMutation::InsertMesh { index: 0, mesh: GltfMesh::default() },
            GltfMutation::RemoveMesh { index: 1 },
            GltfMutation::InsertMaterial { index: 0, material: GltfMaterial::default() },
            GltfMutation::RemoveMaterial { index: 0 },
            GltfMutation::BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) },
            GltfMutation::InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 1, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1] },
            GltfMutation::RemoveBuffer { index: 0 },
            GltfMutation::InsertAnimation { index: 0, animation: GltfAnimation::default() },
            GltfMutation::RemoveAnimation { index: 0 },
        ];
        for m in variants {
            let (_, forward_diff) = {
                let mut s = base.clone();
                let d = apply_gltf_mutation(&mut s, &m).expect("valid mutation");
                (s, d)
            };
            let mutated = MutationDiff::apply(&forward_diff, &base);
            let inverses = <GltfMutation as Mutation<GltfSnapshot>>::inverse(&m, &base);
            let mut back = mutated.clone();
            for inv in &inverses {
                let d = apply_gltf_mutation(&mut back, inv).expect("valid inverse");
                let _ = d;
            }
            assert_eq!(back, base, "inverse of {m:?} did not restore base");
        }
    }

    #[test]
    fn structural_insert_transports_references_and_inverse_restores_exactly() {
        let base = base_snapshot();
        let mutation = GltfMutation::InsertNode { index: 0, node: GltfNode::default() };
        let diff = plan_gltf_mutation(&base, &mutation).expect("valid insertion");
        let next = diff.apply(&base);
        assert_eq!(next.document.scenes[0].nodes, vec![1]);
        assert_eq!(next.document.nodes[1].mesh, Some(0));
        let inverse = mutation.inverse(&base);
        let mut restored = next;
        for operation in inverse {
            apply_gltf_mutation(&mut restored, &operation).expect("valid inverse");
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn inserted_node_payload_uses_the_pre_insertion_index_namespace() {
        let base = base_snapshot();
        let inserted = GltfNode { children: vec![1], ..GltfNode::default() };
        let next = semantic_snapshot(&base, &GltfMutation::InsertNode { index: 0, node: inserted }).expect("valid node insertion");
        assert_eq!(next.document.nodes[0].children, vec![2]);
        assert_eq!(next.document.scenes[0].nodes, vec![1]);
    }

    #[test]
    fn referenced_remove_and_out_of_range_insert_are_rejected_without_effect() {
        let base = base_snapshot();
        let referenced = plan_gltf_mutation(&base, &GltfMutation::RemoveNode { index: 0 }).expect_err("scene root is referenced");
        assert_eq!(referenced.code, "gltf.reference.in-use");
        assert!(referenced.detail.contains("document/scenes/0/nodes/0"));
        let out_of_range = plan_gltf_mutation(&base, &GltfMutation::InsertMesh { index: 99, mesh: GltfMesh::default() }).expect_err("index must not clamp");
        assert_eq!(out_of_range.code, "gltf.mutation.insert-out-of-range");
        let mut unchanged = base.clone();
        assert!(apply_gltf_mutation(&mut unchanged, &GltfMutation::RemoveNode { index: 0 }).is_err());
        assert_eq!(unchanged, base);
    }

    #[test]
    fn buffer_metadata_payload_misalignment_is_rejected() {
        let base = base_snapshot();
        let mutation = GltfMutation::SetBuffer {
            index: 0,
            buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None },
            bytes: vec![1, 2, 3],
        };
        let rejection = plan_gltf_mutation(&base, &mutation).expect_err("short payload must be rejected");
        assert_eq!(rejection.code, "gltf.buffer.byte-length");
    }

    #[test]
    fn accessor_transport_includes_morph_target_dependencies() {
        use crate::artifacts::gltf::schema::snapshot::GltfMorphTarget;
        let mut base = base_snapshot();
        let primitive = &mut base.document.meshes[0].primitives[0];
        primitive.attributes = vec![("POSITION".into(), 0)];
        primitive.indices = Some(0);
        primitive.targets = vec![GltfMorphTarget(vec![("POSITION".into(), 0)])];
        let accessor = base.document.accessors[0].clone();
        let next = semantic_snapshot(&base, &GltfMutation::InsertAccessor { index: 0, accessor }).expect("valid accessor insertion");
        let primitive = &next.document.meshes[0].primitives[0];
        assert_eq!(primitive.attributes[0].1, 1);
        assert_eq!(primitive.indices, Some(1));
        assert_eq!(primitive.targets[0].0[0].1, 1);
    }

    #[test]
    fn semantic_operations_report_stable_regions_and_round_trip() {
        use protocol::DiffRegions as _;
        let base = base_snapshot();
        let operations = [
            GltfMutation::TransformNode { index: 1, matrix: None, translation: Some([1.0, 2.0, 3.0]), rotation: None, scale: None },
            GltfMutation::ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 },
            GltfMutation::BindNodeMesh { index: 1, mesh: Some(0) },
            GltfMutation::BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) },
        ];
        for operation in operations {
            let diff = plan_gltf_mutation(&base, &operation).expect("semantic operation");
            assert!(!diff.touches().paths.is_empty(), "missing touched paths for {operation:?}");
            let next = diff.apply(&base);
            let mut restored = next;
            for inverse in operation.inverse(&base) {
                apply_gltf_mutation(&mut restored, &inverse).expect("semantic inverse");
            }
            assert_eq!(restored, base, "inverse mismatch for {operation:?}");
        }
    }

    //#region 🔖️HandcraftedOpCodecTests
    /// 🎯️ A snapshot with `bufferViews`/`textures`/`images`/`samplers`/`skins`/`cameras` populated
    /// (`base_snapshot()` above has none of these -- they're WEAK collections only reachable via
    /// `SetSnapshot` per F4's variant vocabulary, so `SetSnapshot`'s `OpText`/`OpBinary` needs a
    /// dedicated fixture to actually exercise `enc_buffer_view`/`enc_texture`/`enc_image`/
    /// `enc_sampler`/`enc_skin`/`enc_camera` — including `GltfCameraProjection::Orthographic`, the
    /// variant `field_sweep`'s `sweep_b` (🔺️diff/component.rs) does not use).
    fn full_snapshot() -> GltfSnapshot {
        let mut s = base_snapshot();
        s.document.buffer_views = vec![crate::artifacts::gltf::schema::snapshot::GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 4, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None }];
        s.document.textures = vec![crate::artifacts::gltf::schema::snapshot::GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }];
        s.document.images = vec![crate::artifacts::gltf::schema::snapshot::GltfImage { uri: Some("tex.png".into()), ..Default::default() }];
        s.document.samplers = vec![crate::artifacts::gltf::schema::snapshot::GltfSampler::default()];
        s.document.skins = vec![crate::artifacts::gltf::schema::snapshot::GltfSkin { joints: vec![0, 1], ..Default::default() }];
        s.document.cameras = vec![crate::artifacts::gltf::schema::snapshot::GltfCamera {
            projection: crate::artifacts::gltf::schema::snapshot::GltfCameraProjection::Orthographic(crate::artifacts::gltf::schema::snapshot::GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: None }),
            name: Some("cam0".into()),
            extensions: None,
            extras: Some(crate::artifacts::gltf::schema::snapshot::GltfJson::Object(vec![("k".into(), crate::artifacts::gltf::schema::snapshot::GltfJson::Number(1.0))])),
        }];
        s.document.extensions = Some(crate::artifacts::gltf::schema::snapshot::GltfJson::Array(vec![crate::artifacts::gltf::schema::snapshot::GltfJson::Null, crate::artifacts::gltf::schema::snapshot::GltfJson::Bool(false)]));
        s
    }

    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `GltfMutation` grammar --
    /// every variant, incl. `SetSnapshot` against `full_snapshot()` (exercises every WEAK
    /// collection's item codec plus `GltfCameraProjection::Orthographic` and 4 of the 6 `GltfJson`
    /// variants at once) and a representative Insert/Remove/Set per STRONG-entity array (the same
    /// entities `diff_codec_text_binary_roundtrip_law`'s `sweep_a`/`sweep_b`/`tristate_snapshot_*`
    /// fixtures cover on the diff side, per `🔺️diff/component.rs`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            GltfMutation::NoMutation,
            GltfMutation::SetSnapshot { snapshot: full_snapshot() },
            GltfMutation::SetAsset { asset: GltfAsset { version: "2.1".into(), generator: None, copyright: Some("(c)".into()), min_version: None, extensions: None, extras: None } },
            GltfMutation::InsertScene { index: 1, scene: GltfScene { nodes: vec![1], name: Some("s".into()), ..GltfScene::default() } },
            GltfMutation::RemoveScene { index: 0 },
            GltfMutation::SetScene { index: 0, scene: GltfScene { nodes: vec![9], name: None, ..GltfScene::default() } },
            GltfMutation::InsertNode { index: 1, node: GltfNode { mesh: Some(1), matrix: Some([0.0; 16]), ..GltfNode::default() } },
            GltfMutation::RemoveNode { index: 0 },
            GltfMutation::SetNode { index: 0, node: GltfNode { mesh: None, camera: Some(2), name: Some("n".into()), ..GltfNode::default() } },
            GltfMutation::TransformNode { index: 1, matrix: None, translation: Some([1.25, -2.5, 3.75]), rotation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some([1.0, 2.0, 3.0]) },
            GltfMutation::ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 },
            GltfMutation::BindNodeMesh { index: 1, mesh: None },
            GltfMutation::InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } },
            GltfMutation::RemoveMesh { index: 0 },
            GltfMutation::SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } },
            GltfMutation::InsertAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte,
                    normalized: false,
                    count: 1,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            },
            GltfMutation::RemoveAccessor { index: 0 },
            GltfMutation::SetAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: Some(0),
                    byte_offset: 4,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                    normalized: true,
                    count: 9,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                    max: Some(vec![1.0]),
                    min: Some(vec![-1.0]),
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            },
            GltfMutation::InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), double_sided: true, ..GltfMaterial::default() } },
            GltfMutation::RemoveMaterial { index: 0 },
            GltfMutation::SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } },
            GltfMutation::BindPrimitiveMaterial { mesh: 0, primitive: 0, material: None },
            GltfMutation::InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 2, uri: Some("data:...".into()), name: None, extensions: None, extras: None }, bytes: vec![7, 8] },
            GltfMutation::RemoveBuffer { index: 0 },
            GltfMutation::SetBuffer { index: 0, buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] },
            GltfMutation::InsertAnimation { index: 0, animation: GltfAnimation { name: Some("a".into()), ..GltfAnimation::default() } },
            GltfMutation::RemoveAnimation { index: 0 },
            GltfMutation::SetAnimation { index: 0, animation: GltfAnimation { name: Some("renamed-anim".into()), ..GltfAnimation::default() } },
        ];
        let _ = &base;
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = GltfMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = GltfMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }

    #[test]
    fn op_codecs_reject_unknown_text_and_trailing_binary() {
        assert!(GltfMutation::parse_op("invent-node index=0").is_err());
        let mut bytes = GltfMutation::BindNodeMesh { index: 1, mesh: None }.encode_op().expect("encode");
        bytes.push(0xff);
        assert!(GltfMutation::decode_op(&bytes).is_err());
    }
    //#endregion 🔖️HandcraftedOpCodecTests
}
//#endregion 🧪️Tests
