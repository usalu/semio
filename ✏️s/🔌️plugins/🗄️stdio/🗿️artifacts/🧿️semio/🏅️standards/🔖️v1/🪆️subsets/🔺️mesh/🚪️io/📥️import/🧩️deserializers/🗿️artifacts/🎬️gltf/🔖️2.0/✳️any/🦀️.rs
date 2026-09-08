//! 📥️ Deserialize `s.stdio.semio/v1/mesh` from `s.stdio.gltf/2.0/*` — gltf is the RICHEST source
//! in the mesh↔{gltf,stl,obj,ply,las} family: multi-primitive meshes, PBR materials, and textures
//! all map fully. Reuses the gltf artifact's own `engine::decode_accessor`/`decode_data_uri` (byte
//! walk already solved there) — this file only maps the already-typed `GltfDocument` shape onto
//! `SemioMeshSnapshot`.
//!
//! 🔖 Documented lossiness (real, honest impedance mismatches — never silently fabricated):
//! - gltf's scene graph (`nodes`/`scenes`/node transforms/`skins`/`animations`/`cameras`) has no
//!   counterpart in `SemioMeshSnapshot` (geometry + material + texture only) — dropped on import.
//! - `GltfPrimitive.mode == 2` (`LINE_LOOP`) has no `SemioTopology` variant — a primitive using it
//!   is a hard `Err`, never silently downgraded to `LineStrip`.
//! - `SemioMaterial` is scalar-PBR-only (`base_color`/`metallic`/`roughness`) — gltf material
//!   texture references (`baseColorTexture`, `normalTexture`, `occlusionTexture`,
//!   `emissiveTexture`) and non-PBR fields (`emissiveFactor`, `alphaMode`, `doubleSided`) are
//!   dropped on import.
//! - `SemioTexture` carries `id`/`mime`/`bytes` only — gltf `sampler` wrap/filter settings and the
//!   `texture -> image` indirection collapse to one texture per gltf `image`.
//! - An `image.uri` that is neither a `data:` uri nor backed by a `bufferView` (i.e. an external
//!   file/network reference) resolves to empty `bytes` — this artifact has no filesystem/network
//!   access, matching the gltf engine's own `resolve_document_buffers` precedent for external
//!   buffer uris.

use semio_s_artifact_stdio_gltf::engine::{decode_accessor, decode_data_uri, GltfComponentType};
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfDocument, GltfImage, GltfPrimitive};
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

//#region 🔖️Topology
/// 🔺️ gltf `primitive.mode` (§5.19.4, default 4/TRIANGLES when absent) -> `SemioTopology`. Mode 2
/// (`LINE_LOOP`) is a real, honest gap — `SemioTopology` has no closed-loop line variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gltf_mode_to_topology(mode: Option<u64>) -> Result<SemioTopology, String> {
    match mode.unwrap_or(4) {
        0 => Ok(SemioTopology::Points),
        1 => Ok(SemioTopology::Lines),
        2 => Err("gltf primitive mode 2 (LINE_LOOP) has no SemioTopology counterpart".into()),
        3 => Ok(SemioTopology::LineStrip),
        4 => Ok(SemioTopology::Triangles),
        5 => Ok(SemioTopology::TriangleStrip),
        6 => Ok(SemioTopology::TriangleFan),
        other => Err(format!("unsupported gltf primitive mode {other}")),
    }
}
//#endregion 🔖️Topology

//#region 🔖️AccessorHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_attr(attributes: &[(String, usize)], name: &str) -> Option<usize> {
    attributes.iter().find(|(n, _)| n == name).map(|(_, idx)| *idx)
}

/// 🔢️ Scales an already-`f64`-widened integer component back into `[0,1]` (or `[-1,1]`) per the
/// glTF 2.0 §3.9.2 normalized-integer rule; `decode_accessor` deliberately leaves this to callers
/// (its own doc comment: "every component already widened to `f64`... regardless of source
/// `componentType`", no normalization applied).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalize_component(v: f64, component_type: GltfComponentType, normalized: bool) -> f64 {
    if !normalized {
        return v;
    }
    match component_type {
        GltfComponentType::Byte => (v / 127.0).max(-1.0),
        GltfComponentType::UnsignedByte => v / 255.0,
        GltfComponentType::Short => (v / 32767.0).max(-1.0),
        GltfComponentType::UnsignedShort => v / 65535.0,
        GltfComponentType::UnsignedInt | GltfComponentType::Float => v,
    }
}

/// 🖼️️ Resolves one `image`'s raw bytes: embedded `bufferView` first, then a `data:` uri; an
/// external (file/network) uri is a documented gap (see module doc comment) -> empty bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_image_bytes(document: &GltfDocument, buffers: &[Vec<u8>], image: &GltfImage) -> Vec<u8> {
    if let Some(bv_idx) = image.buffer_view {
        if let Some(bv) = document.buffer_views.get(bv_idx) {
            if let Some(buf) = buffers.get(bv.buffer) {
                let start = bv.byte_offset;
                let end = start + bv.byte_length;
                if end <= buf.len() {
                    return buf[start..end].to_vec();
                }
            }
        }
        return Vec::new();
    }
    match &image.uri {
        Some(uri) if uri.starts_with("data:") => decode_data_uri(uri).unwrap_or_default(),
        _ => Vec::new(),
    }
}
//#endregion 🔖️AccessorHelpers

//#region 🔖️PrimitiveMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_primitive(document: &GltfDocument, buffers: &[Vec<u8>], prim: &GltfPrimitive, id: String, material_id: Option<String>) -> Result<SemioPrimitive, String> {
    let topology = gltf_mode_to_topology(prim.mode)?;

    let pos_idx = find_attr(&prim.attributes, "POSITION").ok_or("primitive missing mandatory POSITION attribute")?;
    let pos_acc = decode_accessor(document, buffers, pos_idx)?;
    let positions: Vec<SemioPoint3> = pos_acc.components.chunks(3).map(|c| SemioPoint3 { x: c[0], y: c[1], z: c[2] }).collect();

    let normals: Vec<SemioPoint3> = match find_attr(&prim.attributes, "NORMAL") {
        Some(idx) => decode_accessor(document, buffers, idx)?.components.chunks(3).map(|c| SemioPoint3 { x: c[0], y: c[1], z: c[2] }).collect(),
        None => Vec::new(),
    };

    let uvs: Vec<SemioUv> = match find_attr(&prim.attributes, "TEXCOORD_0") {
        Some(idx) => decode_accessor(document, buffers, idx)?.components.chunks(2).map(|c| SemioUv { u: c[0], v: c[1] }).collect(),
        None => Vec::new(),
    };

    let colors: Vec<SemioRgba> = match find_attr(&prim.attributes, "COLOR_0") {
        Some(idx) => {
            let acc = decode_accessor(document, buffers, idx)?;
            let nc = acc.accessor_type.components();
            if nc != 3 && nc != 4 {
                return Err(format!("COLOR_0 accessor must be VEC3 or VEC4, got {nc}-component"));
            }
            acc.components
                .chunks(nc)
                .map(|c| SemioRgba {
                    r: normalize_component(c[0], acc.component_type, acc.normalized) as f32,
                    g: normalize_component(c[1], acc.component_type, acc.normalized) as f32,
                    b: normalize_component(c[2], acc.component_type, acc.normalized) as f32,
                    a: if nc == 4 { normalize_component(c[3], acc.component_type, acc.normalized) as f32 } else { 1.0 },
                })
                .collect()
        }
        None => Vec::new(),
    };

    let indices: Vec<u32> = match prim.indices {
        Some(idx) => decode_accessor(document, buffers, idx)?.components.iter().map(|&v| v.round() as u32).collect(),
        None => Vec::new(),
    };

    Ok(SemioPrimitive { id, topology, positions, normals, uvs, colors, indices, material_id })
}
//#endregion 🔖️PrimitiveMapping

pub struct SemioMeshFromGltf;

impl ArtifactDeserializer for SemioMeshFromGltf {
    type From = GltfSnapshot;
    type Into = SemioMeshSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let document = &from.document;

        let materials: Vec<SemioMaterial> = document
            .materials
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let pbr = m.pbr_metallic_roughness.clone().unwrap_or_default();
                SemioMaterial {
                    id: format!("mat-{i}"),
                    base_color: SemioRgba { r: pbr.base_color_factor[0] as f32, g: pbr.base_color_factor[1] as f32, b: pbr.base_color_factor[2] as f32, a: pbr.base_color_factor[3] as f32 },
                    metallic: pbr.metallic_factor as f32,
                    roughness: pbr.roughness_factor as f32,
                }
            })
            .collect();

        let textures: Vec<SemioTexture> = document.images.iter().enumerate().map(|(i, img)| SemioTexture { id: format!("tex-{i}"), mime: img.mime_type.clone().unwrap_or_default(), bytes: resolve_image_bytes(document, &from.buffers, img) }).collect();

        let mut meshes = Vec::with_capacity(document.meshes.len());
        for (mi, gmesh) in document.meshes.iter().enumerate() {
            let mesh_id = gmesh.name.clone().unwrap_or_else(|| format!("mesh-{mi}"));
            let mut primitives = Vec::with_capacity(gmesh.primitives.len());
            for (pi, prim) in gmesh.primitives.iter().enumerate() {
                let material_id = prim.material.map(|idx| format!("mat-{idx}"));
                let sp = decode_primitive(document, &from.buffers, prim, format!("{mesh_id}-prim-{pi}"), material_id).map_err(|e| store::PackError::Schema(format!("SemioMeshFromGltf: mesh {mi} primitive {pi}: {e}")))?;
                primitives.push(sp);
            }
            meshes.push(SemioMesh { id: mesh_id, primitives });
        }

        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes, materials, textures })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
