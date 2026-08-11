//! 🧬️ GltfMutation — document mutation dispatch. Ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, F4: real named variants
//! (Set/Insert/Remove per highest-value array, per the recipe's explicit priority list) replacing
//! the `{ NoMutation, SetSnapshot }` stub. Every variant's `diff()` is handcrafted directly
//! (constructs the sparse [`GltfDiff`] by hand) — apply-and-capture is banned.

use crate::artifacts::gltf::schema::diff::{
    diff_set_snapshot, GltfAccessorDiff, GltfAccessorsDiff, GltfAdded, GltfAssetDiff, GltfBufferBytesDiff,
    GltfBufferDiff, GltfBuffersDiff, GltfDiff, GltfMaterialDiff, GltfMaterialsDiff, GltfMeshDiff, GltfMeshesDiff,
    GltfModified, GltfNodeDiff, GltfNodesDiff, GltfSceneDiff, GltfScenesDiff, ItemDiff as _,
};
use crate::artifacts::gltf::schema::diff::GltfAnimationsDiff;
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` grammar primitives + value codecs, reused verbatim from
/// `🔺️diff/component.rs`'s `HandcraftedDiffCodec` region (`enc_gltf_snapshot`/`dec_gltf_snapshot`
/// needs every one of these; SvgMutation reuses SvgDiff's the same way) — see that region's doc
/// comment for the full derive-rejection citation shared by both sides of this artifact.
use crate::artifacts::gltf::schema::diff::{
    dec_accessor, dec_animation, dec_asset, dec_buffer, dec_bytes, dec_gltf_snapshot, dec_material, dec_mesh,
    dec_node, dec_scene, enc_accessor, enc_animation, enc_asset, enc_buffer, enc_bytes, enc_gltf_snapshot,
    enc_material, enc_mesh, enc_node, enc_scene,
};
use crate::artifacts::gltf::schema::snapshot::{
    GltfAccessor, GltfAnimation, GltfAsset, GltfBuffer, GltfMaterial, GltfMesh, GltfNode, GltfScene,
};
use crate::artifacts::gltf::GltfSnapshot;
use protocol::{Mutation, OpText};
#[cfg(test)]
use protocol::OpBinary;
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
    SetSnapshot { snapshot: GltfSnapshot },
    SetAsset { asset: GltfAsset },

    InsertScene { index: usize, scene: GltfScene },
    RemoveScene { index: usize },
    SetScene { index: usize, scene: GltfScene },

    InsertNode { index: usize, node: GltfNode },
    RemoveNode { index: usize },
    SetNode { index: usize, node: GltfNode },

    InsertMesh { index: usize, mesh: GltfMesh },
    RemoveMesh { index: usize },
    SetMesh { index: usize, mesh: GltfMesh },

    InsertAccessor { index: usize, accessor: GltfAccessor },
    RemoveAccessor { index: usize },
    SetAccessor { index: usize, accessor: GltfAccessor },

    InsertMaterial { index: usize, material: GltfMaterial },
    RemoveMaterial { index: usize },
    SetMaterial { index: usize, material: GltfMaterial },

    /// 📦️ Touches BOTH `document.buffers[index]` (metadata) and `GltfSnapshot::buffers[index]`
    /// (raw payload bytes) together -- they are two index-aligned collections (per the recipe's
    /// explicit "buffers: Vec<Vec<u8>> stays as-is" instruction), kept in sync by this one
    /// mutation the same way the builder's `add_buffer` already couples them.
    InsertBuffer { index: usize, buffer: GltfBuffer, bytes: Vec<u8> },
    RemoveBuffer { index: usize },
    SetBuffer { index: usize, buffer: GltfBuffer, bytes: Vec<u8> },

    InsertAnimation { index: usize, animation: GltfAnimation },
    RemoveAnimation { index: usize },
    SetAnimation { index: usize, animation: GltfAnimation },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_gltf_mutation(snapshot: &mut GltfSnapshot, mutation: &GltfMutation) -> GltfDiff {
    let __diff = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&__diff, snapshot);
    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<GltfSnapshot> for GltfMutation {
    type Diff = GltfDiff;

    fn diff(&self, base: &GltfSnapshot) -> Self::Diff {
        let doc = &base.document;
        match self {
            GltfMutation::NoMutation => GltfDiff::default(),
            GltfMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            GltfMutation::SetAsset { asset } => {
                let d = GltfAssetDiff::between(&doc.asset, asset);
                GltfDiff { asset: (!d.is_empty()).then_some(d), ..Default::default() }
            }

            GltfMutation::InsertScene { index, scene } => {
                let at = (*index).min(doc.scenes.len());
                GltfDiff { scenes: Some(GltfScenesDiff { added: vec![GltfAdded { index: at, item: scene.clone() }], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::RemoveScene { index } => {
                GltfDiff { scenes: Some(GltfScenesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::SetScene { index, scene } => {
                let modified = doc.scenes.get(*index).map(|cur| vec![GltfModified { index: *index, diff: GltfSceneDiff::between(cur, scene) }]).unwrap_or_default();
                GltfDiff { scenes: Some(GltfScenesDiff { modified, ..Default::default() }), ..Default::default() }
            }

            GltfMutation::InsertNode { index, node } => {
                let at = (*index).min(doc.nodes.len());
                GltfDiff { nodes: Some(GltfNodesDiff { added: vec![GltfAdded { index: at, item: node.clone() }], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::RemoveNode { index } => {
                GltfDiff { nodes: Some(GltfNodesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::SetNode { index, node } => {
                let modified = doc.nodes.get(*index).map(|cur| vec![GltfModified { index: *index, diff: GltfNodeDiff::between(cur, node) }]).unwrap_or_default();
                GltfDiff { nodes: Some(GltfNodesDiff { modified, ..Default::default() }), ..Default::default() }
            }

            GltfMutation::InsertMesh { index, mesh } => {
                let at = (*index).min(doc.meshes.len());
                GltfDiff { meshes: Some(GltfMeshesDiff { added: vec![GltfAdded { index: at, item: mesh.clone() }], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::RemoveMesh { index } => {
                GltfDiff { meshes: Some(GltfMeshesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::SetMesh { index, mesh } => {
                let modified = doc.meshes.get(*index).map(|cur| vec![GltfModified { index: *index, diff: GltfMeshDiff::between(cur, mesh) }]).unwrap_or_default();
                GltfDiff { meshes: Some(GltfMeshesDiff { modified, ..Default::default() }), ..Default::default() }
            }

            GltfMutation::InsertAccessor { index, accessor } => {
                let at = (*index).min(doc.accessors.len());
                GltfDiff { accessors: Some(GltfAccessorsDiff { added: vec![GltfAdded { index: at, item: accessor.clone() }], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::RemoveAccessor { index } => {
                GltfDiff { accessors: Some(GltfAccessorsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::SetAccessor { index, accessor } => {
                let modified = doc.accessors.get(*index).map(|cur| vec![GltfModified { index: *index, diff: GltfAccessorDiff::between(cur, accessor) }]).unwrap_or_default();
                GltfDiff { accessors: Some(GltfAccessorsDiff { modified, ..Default::default() }), ..Default::default() }
            }

            GltfMutation::InsertMaterial { index, material } => {
                let at = (*index).min(doc.materials.len());
                GltfDiff { materials: Some(GltfMaterialsDiff { added: vec![GltfAdded { index: at, item: material.clone() }], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::RemoveMaterial { index } => {
                GltfDiff { materials: Some(GltfMaterialsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::SetMaterial { index, material } => {
                let modified = doc.materials.get(*index).map(|cur| vec![GltfModified { index: *index, diff: GltfMaterialDiff::between(cur, material) }]).unwrap_or_default();
                GltfDiff { materials: Some(GltfMaterialsDiff { modified, ..Default::default() }), ..Default::default() }
            }

            GltfMutation::InsertBuffer { index, buffer, bytes } => {
                let at = (*index).min(doc.buffers.len());
                GltfDiff {
                    buffers: Some(GltfBuffersDiff { added: vec![GltfAdded { index: at, item: buffer.clone() }], ..Default::default() }),
                    buffer_bytes: Some(GltfBufferBytesDiff { added: vec![GltfAdded { index: at, item: bytes.clone() }], ..Default::default() }),
                    ..Default::default()
                }
            }
            GltfMutation::RemoveBuffer { index } => {
                GltfDiff {
                    buffers: Some(GltfBuffersDiff { removed: vec![*index], ..Default::default() }),
                    buffer_bytes: Some(GltfBufferBytesDiff { removed: vec![*index], ..Default::default() }),
                    ..Default::default()
                }
            }
            GltfMutation::SetBuffer { index, buffer, bytes } => {
                let modified = doc.buffers.get(*index).map(|cur| vec![GltfModified { index: *index, diff: GltfBufferDiff::between(cur, buffer) }]).unwrap_or_default();
                let bytes_modified = base.buffers.get(*index).map(|_| vec![GltfModified { index: *index, diff: bytes.clone() }]).unwrap_or_default();
                GltfDiff {
                    buffers: Some(GltfBuffersDiff { modified, ..Default::default() }),
                    buffer_bytes: Some(GltfBufferBytesDiff { modified: bytes_modified, ..Default::default() }),
                    ..Default::default()
                }
            }

            GltfMutation::InsertAnimation { index, animation } => {
                let at = (*index).min(doc.animations.len());
                GltfDiff { animations: Some(GltfAnimationsDiff { added: vec![GltfAdded { index: at, item: animation.clone() }], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::RemoveAnimation { index } => {
                GltfDiff { animations: Some(GltfAnimationsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() }
            }
            GltfMutation::SetAnimation { index, animation } => {
                let modified = doc.animations.get(*index).map(|_| vec![GltfModified { index: *index, diff: animation.clone() }]).unwrap_or_default();
                GltfDiff { animations: Some(GltfAnimationsDiff { modified, ..Default::default() }), ..Default::default() }
            }
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<Self> {
        let doc = &base.document;
        match self {
            GltfMutation::NoMutation => vec![GltfMutation::NoMutation],
            GltfMutation::SetSnapshot { .. } => vec![GltfMutation::SetSnapshot { snapshot: base.clone() }],
            GltfMutation::SetAsset { .. } => vec![GltfMutation::SetAsset { asset: doc.asset.clone() }],

            GltfMutation::InsertScene { index, .. } => vec![GltfMutation::RemoveScene { index: (*index).min(doc.scenes.len()) }],
            GltfMutation::RemoveScene { index } => match doc.scenes.get(*index) {
                Some(scene) => vec![GltfMutation::InsertScene { index: *index, scene: scene.clone() }],
                None => vec![GltfMutation::NoMutation],
            },
            GltfMutation::SetScene { index, .. } => match doc.scenes.get(*index) {
                Some(scene) => vec![GltfMutation::SetScene { index: *index, scene: scene.clone() }],
                None => vec![GltfMutation::NoMutation],
            },

            GltfMutation::InsertNode { index, .. } => vec![GltfMutation::RemoveNode { index: (*index).min(doc.nodes.len()) }],
            GltfMutation::RemoveNode { index } => match doc.nodes.get(*index) {
                Some(node) => vec![GltfMutation::InsertNode { index: *index, node: node.clone() }],
                None => vec![GltfMutation::NoMutation],
            },
            GltfMutation::SetNode { index, .. } => match doc.nodes.get(*index) {
                Some(node) => vec![GltfMutation::SetNode { index: *index, node: node.clone() }],
                None => vec![GltfMutation::NoMutation],
            },

            GltfMutation::InsertMesh { index, .. } => vec![GltfMutation::RemoveMesh { index: (*index).min(doc.meshes.len()) }],
            GltfMutation::RemoveMesh { index } => match doc.meshes.get(*index) {
                Some(mesh) => vec![GltfMutation::InsertMesh { index: *index, mesh: mesh.clone() }],
                None => vec![GltfMutation::NoMutation],
            },
            GltfMutation::SetMesh { index, .. } => match doc.meshes.get(*index) {
                Some(mesh) => vec![GltfMutation::SetMesh { index: *index, mesh: mesh.clone() }],
                None => vec![GltfMutation::NoMutation],
            },

            GltfMutation::InsertAccessor { index, .. } => vec![GltfMutation::RemoveAccessor { index: (*index).min(doc.accessors.len()) }],
            GltfMutation::RemoveAccessor { index } => match doc.accessors.get(*index) {
                Some(accessor) => vec![GltfMutation::InsertAccessor { index: *index, accessor: accessor.clone() }],
                None => vec![GltfMutation::NoMutation],
            },
            GltfMutation::SetAccessor { index, .. } => match doc.accessors.get(*index) {
                Some(accessor) => vec![GltfMutation::SetAccessor { index: *index, accessor: accessor.clone() }],
                None => vec![GltfMutation::NoMutation],
            },

            GltfMutation::InsertMaterial { index, .. } => vec![GltfMutation::RemoveMaterial { index: (*index).min(doc.materials.len()) }],
            GltfMutation::RemoveMaterial { index } => match doc.materials.get(*index) {
                Some(material) => vec![GltfMutation::InsertMaterial { index: *index, material: material.clone() }],
                None => vec![GltfMutation::NoMutation],
            },
            GltfMutation::SetMaterial { index, .. } => match doc.materials.get(*index) {
                Some(material) => vec![GltfMutation::SetMaterial { index: *index, material: material.clone() }],
                None => vec![GltfMutation::NoMutation],
            },

            GltfMutation::InsertBuffer { index, .. } => vec![GltfMutation::RemoveBuffer { index: (*index).min(doc.buffers.len()) }],
            GltfMutation::RemoveBuffer { index } => match (doc.buffers.get(*index), base.buffers.get(*index)) {
                (Some(buffer), Some(bytes)) => vec![GltfMutation::InsertBuffer { index: *index, buffer: buffer.clone(), bytes: bytes.clone() }],
                _ => vec![GltfMutation::NoMutation],
            },
            GltfMutation::SetBuffer { index, .. } => match (doc.buffers.get(*index), base.buffers.get(*index)) {
                (Some(buffer), Some(bytes)) => vec![GltfMutation::SetBuffer { index: *index, buffer: buffer.clone(), bytes: bytes.clone() }],
                _ => vec![GltfMutation::NoMutation],
            },

            GltfMutation::InsertAnimation { index, .. } => vec![GltfMutation::RemoveAnimation { index: (*index).min(doc.animations.len()) }],
            GltfMutation::RemoveAnimation { index } => match doc.animations.get(*index) {
                Some(animation) => vec![GltfMutation::InsertAnimation { index: *index, animation: animation.clone() }],
                None => vec![GltfMutation::NoMutation],
            },
            GltfMutation::SetAnimation { index, .. } => match doc.animations.get(*index) {
                Some(animation) => vec![GltfMutation::SetAnimation { index: *index, animation: animation.clone() }],
                None => vec![GltfMutation::NoMutation],
            },
        }
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

        GltfMutation::InsertMesh { index, mesh } => format!("insert-mesh index={index} mesh={}", enc_mesh(mesh)),
        GltfMutation::RemoveMesh { index } => format!("remove-mesh index={index}"),
        GltfMutation::SetMesh { index, mesh } => format!("set-mesh index={index} mesh={}", enc_mesh(mesh)),

        GltfMutation::InsertAccessor { index, accessor } => format!("insert-accessor index={index} accessor={}", enc_accessor(accessor)),
        GltfMutation::RemoveAccessor { index } => format!("remove-accessor index={index}"),
        GltfMutation::SetAccessor { index, accessor } => format!("set-accessor index={index} accessor={}", enc_accessor(accessor)),

        GltfMutation::InsertMaterial { index, material } => format!("insert-material index={index} material={}", enc_material(material)),
        GltfMutation::RemoveMaterial { index } => format!("remove-material index={index}"),
        GltfMutation::SetMaterial { index, material } => format!("set-material index={index} material={}", enc_material(material)),

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
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("gltf mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
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

        "insert-mesh" => Ok(GltfMutation::InsertMesh { index: idx("index")?, mesh: dec_mesh(arg("mesh")?)? }),
        "remove-mesh" => Ok(GltfMutation::RemoveMesh { index: idx("index")? }),
        "set-mesh" => Ok(GltfMutation::SetMesh { index: idx("index")?, mesh: dec_mesh(arg("mesh")?)? }),

        "insert-accessor" => Ok(GltfMutation::InsertAccessor { index: idx("index")?, accessor: dec_accessor(arg("accessor")?)? }),
        "remove-accessor" => Ok(GltfMutation::RemoveAccessor { index: idx("index")? }),
        "set-accessor" => Ok(GltfMutation::SetAccessor { index: idx("index")?, accessor: dec_accessor(arg("accessor")?)? }),

        "insert-material" => Ok(GltfMutation::InsertMaterial { index: idx("index")?, material: dec_material(arg("material")?)? }),
        "remove-material" => Ok(GltfMutation::RemoveMaterial { index: idx("index")? }),
        "set-material" => Ok(GltfMutation::SetMaterial { index: idx("index")?, material: dec_material(arg("material")?)? }),

        "insert-buffer" => Ok(GltfMutation::InsertBuffer { index: idx("index")?, buffer: dec_buffer(arg("buffer")?)?, bytes: dec_bytes(arg("bytes")?)? }),
        "remove-buffer" => Ok(GltfMutation::RemoveBuffer { index: idx("index")? }),
        "set-buffer" => Ok(GltfMutation::SetBuffer { index: idx("index")?, buffer: dec_buffer(arg("buffer")?)?, bytes: dec_bytes(arg("bytes")?)? }),

        "insert-animation" => Ok(GltfMutation::InsertAnimation { index: idx("index")?, animation: dec_animation(arg("animation")?)? }),
        "remove-animation" => Ok(GltfMutation::RemoveAnimation { index: idx("index")? }),
        "set-animation" => Ok(GltfMutation::SetAnimation { index: idx("index")?, animation: dec_animation(arg("animation")?)? }),

        other => Err(format!("gltf mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for GltfMutation {
    fn print_op(&self) -> String {
        print_gltf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_gltf_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification `GltfDiff`'s hand-rolled `DiffCodec`
/// uses.
impl protocol::OpBinary for GltfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
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
                nodes: vec![GltfNode { mesh: Some(0), ..GltfNode::default() }, GltfNode { mesh: Some(1), ..GltfNode::default() }],
                meshes: vec![GltfMesh::default(), GltfMesh::default()],
                accessors: vec![GltfAccessor {
                    buffer_view: None, byte_offset: 0, component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                    normalized: false, count: 3, kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                    max: None, min: None, sparse: None, name: None, extensions: None, extras: None,
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
            GltfMutation::SetScene { index: 0, scene: GltfScene { nodes: vec![9], name: Some("renamed".into()), ..GltfScene::default() } },
            GltfMutation::InsertNode { index: 1, node: GltfNode { mesh: Some(1), ..GltfNode::default() } },
            GltfMutation::RemoveNode { index: 0 },
            GltfMutation::SetNode { index: 0, node: GltfNode { mesh: None, name: Some("n".into()), ..GltfNode::default() } },
            GltfMutation::InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } },
            GltfMutation::RemoveMesh { index: 0 },
            GltfMutation::SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } },
            GltfMutation::InsertAccessor { index: 0, accessor: GltfAccessor { buffer_view: None, byte_offset: 0, component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte, normalized: false, count: 1, kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar, max: None, min: None, sparse: None, name: None, extensions: None, extras: None } },
            GltfMutation::RemoveAccessor { index: 0 },
            GltfMutation::SetAccessor { index: 0, accessor: GltfAccessor { buffer_view: None, byte_offset: 0, component_type: crate::artifacts::gltf::engine::GltfComponentType::Float, normalized: true, count: 9, kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3, max: None, min: None, sparse: None, name: None, extensions: None, extras: None } },
            GltfMutation::InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), ..GltfMaterial::default() } },
            GltfMutation::RemoveMaterial { index: 0 },
            GltfMutation::SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } },
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
            let actual_diff = apply_gltf_mutation(&mut s, &m);
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
            GltfMutation::InsertScene { index: 0, scene: GltfScene { nodes: vec![5], ..GltfScene::default() } },
            GltfMutation::RemoveScene { index: 0 },
            GltfMutation::SetScene { index: 0, scene: GltfScene { nodes: vec![7], name: Some("z".into()), ..GltfScene::default() } },
            GltfMutation::InsertNode { index: 0, node: GltfNode { mesh: Some(0), ..GltfNode::default() } },
            GltfMutation::RemoveNode { index: 1 },
            GltfMutation::SetNode { index: 1, node: GltfNode { mesh: None, ..GltfNode::default() } },
            GltfMutation::InsertMesh { index: 0, mesh: GltfMesh::default() },
            GltfMutation::RemoveMesh { index: 0 },
            GltfMutation::InsertMaterial { index: 0, material: GltfMaterial::default() },
            GltfMutation::RemoveMaterial { index: 0 },
            GltfMutation::InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 1, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1] },
            GltfMutation::RemoveBuffer { index: 0 },
            GltfMutation::InsertAnimation { index: 0, animation: GltfAnimation::default() },
            GltfMutation::RemoveAnimation { index: 0 },
        ];
        for m in variants {
            let (_, forward_diff) = {
                let mut s = base.clone();
                let d = apply_gltf_mutation(&mut s, &m);
                (s, d)
            };
            let mutated = MutationDiff::apply(&forward_diff, &base);
            let inverses = <GltfMutation as Mutation<GltfSnapshot>>::inverse(&m, &base);
            let mut back = mutated.clone();
            for inv in &inverses {
                let d = apply_gltf_mutation(&mut back, inv);
                let _ = d;
            }
            assert_eq!(back, base, "inverse of {m:?} did not restore base");
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
        s.document.buffer_views = vec![crate::artifacts::gltf::schema::snapshot::GltfBufferView {
            buffer: 0, byte_offset: 0, byte_length: 4, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None,
        }];
        s.document.textures = vec![crate::artifacts::gltf::schema::snapshot::GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }];
        s.document.images = vec![crate::artifacts::gltf::schema::snapshot::GltfImage { uri: Some("tex.png".into()), ..Default::default() }];
        s.document.samplers = vec![crate::artifacts::gltf::schema::snapshot::GltfSampler::default()];
        s.document.skins = vec![crate::artifacts::gltf::schema::snapshot::GltfSkin { joints: vec![0, 1], ..Default::default() }];
        s.document.cameras = vec![crate::artifacts::gltf::schema::snapshot::GltfCamera {
            projection: crate::artifacts::gltf::schema::snapshot::GltfCameraProjection::Orthographic(
                crate::artifacts::gltf::schema::snapshot::GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: None },
            ),
            name: Some("cam0".into()),
            extensions: None,
            extras: Some(crate::artifacts::gltf::schema::snapshot::GltfJson::Object(vec![("k".into(), crate::artifacts::gltf::schema::snapshot::GltfJson::Number(1.0))])),
        }];
        s.document.extensions = Some(crate::artifacts::gltf::schema::snapshot::GltfJson::Array(vec![
            crate::artifacts::gltf::schema::snapshot::GltfJson::Null,
            crate::artifacts::gltf::schema::snapshot::GltfJson::Bool(false),
        ]));
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

            GltfMutation::InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } },
            GltfMutation::RemoveMesh { index: 0 },
            GltfMutation::SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } },

            GltfMutation::InsertAccessor {
                index: 0,
                accessor: GltfAccessor { buffer_view: None, byte_offset: 0, component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte, normalized: false, count: 1, kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar, max: None, min: None, sparse: None, name: None, extensions: None, extras: None },
            },
            GltfMutation::RemoveAccessor { index: 0 },
            GltfMutation::SetAccessor {
                index: 0,
                accessor: GltfAccessor { buffer_view: Some(0), byte_offset: 4, component_type: crate::artifacts::gltf::engine::GltfComponentType::Float, normalized: true, count: 9, kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3, max: Some(vec![1.0]), min: Some(vec![-1.0]), sparse: None, name: None, extensions: None, extras: None },
            },

            GltfMutation::InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), double_sided: true, ..GltfMaterial::default() } },
            GltfMutation::RemoveMaterial { index: 0 },
            GltfMutation::SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } },

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
    //#endregion 🔖️HandcraftedOpCodecTests
}
//#endregion 🧪️Tests
