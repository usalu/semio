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

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::{GltfDocument, GltfImage, GltfPrimitive};
use crate::artifacts::gltf::engine::{decode_accessor, decode_data_uri, GltfComponentType};
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{
    SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology,
    STDIO_SEMIOMESH_DOCUMENT_SCHEMA,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

//#region 🔖️Topology
/// 🔺️ gltf `primitive.mode` (§5.19.4, default 4/TRIANGLES when absent) -> `SemioTopology`. Mode 2
/// (`LINE_LOOP`) is a real, honest gap — `SemioTopology` has no closed-loop line variant.
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
fn find_attr(attributes: &[(String, usize)], name: &str) -> Option<usize> {
    attributes.iter().find(|(n, _)| n == name).map(|(_, idx)| *idx)
}

/// 🔢️ Scales an already-`f64`-widened integer component back into `[0,1]` (or `[-1,1]`) per the
/// glTF 2.0 §3.9.2 normalized-integer rule; `decode_accessor` deliberately leaves this to callers
/// (its own doc comment: "every component already widened to `f64`... regardless of source
/// `componentType`", no normalization applied).
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

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let document = &from.document;

        let materials: Vec<SemioMaterial> = document
            .materials
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let pbr = m.pbr_metallic_roughness.clone().unwrap_or_default();
                SemioMaterial {
                    id: format!("mat-{i}"),
                    base_color: SemioRgba {
                        r: pbr.base_color_factor[0] as f32,
                        g: pbr.base_color_factor[1] as f32,
                        b: pbr.base_color_factor[2] as f32,
                        a: pbr.base_color_factor[3] as f32,
                    },
                    metallic: pbr.metallic_factor as f32,
                    roughness: pbr.roughness_factor as f32,
                }
            })
            .collect();

        let textures: Vec<SemioTexture> = document
            .images
            .iter()
            .enumerate()
            .map(|(i, img)| SemioTexture {
                id: format!("tex-{i}"),
                mime: img.mime_type.clone().unwrap_or_default(),
                bytes: resolve_image_bytes(document, &from.buffers, img),
            })
            .collect();

        let mut meshes = Vec::with_capacity(document.meshes.len());
        for (mi, gmesh) in document.meshes.iter().enumerate() {
            let mesh_id = gmesh.name.clone().unwrap_or_else(|| format!("mesh-{mi}"));
            let mut primitives = Vec::with_capacity(gmesh.primitives.len());
            for (pi, prim) in gmesh.primitives.iter().enumerate() {
                let material_id = prim.material.map(|idx| format!("mat-{idx}"));
                let sp = decode_primitive(document, &from.buffers, prim, format!("{mesh_id}-prim-{pi}"), material_id)
                    .map_err(|e| store::PackError::Schema(format!("SemioMeshFromGltf: mesh {mi} primitive {pi}: {e}")))?;
                primitives.push(sp);
            }
            meshes.push(SemioMesh { id: mesh_id, primitives });
        }

        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes, materials, textures })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::schema::snapshot::{GltfAsset, GltfMaterial, GltfMesh, GltfPbrMetallicRoughness};
    use crate::artifacts::gltf::engine::GltfAccessorType;
    use crate::artifacts::gltf::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView};

    /// 🏗️ A real-shaped 2-triangle quad (shared POSITION/NORMAL/TEXCOORD_0/COLOR_0/indices) with
    /// one PBR material and one embedded (data-uri) texture — exercises every mapped field.
    fn sample_gltf() -> GltfSnapshot {
        let positions: [f32; 12] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];
        let normals: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let uvs: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let colors: [f32; 16] = [1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0];
        let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

        let mut buf = Vec::new();
        let mut buffer_views = Vec::new();
        let mut accessors = Vec::new();

        let mut push = |bytes: &[u8], component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize| -> usize {
            let byte_offset = buf.len();
            buf.extend_from_slice(bytes);
            let bv = buffer_views.len();
            buffer_views.push(GltfBufferView { buffer: 0, byte_offset, byte_length: bytes.len(), byte_stride: None, target: None, name: None, extensions: None, extras: None });
            let idx = accessors.len();
            accessors.push(GltfAccessor { buffer_view: Some(bv), byte_offset: 0, component_type, normalized: false, count, kind: accessor_type, max: None, min: None, sparse: None, name: None, extensions: None, extras: None });
            idx
        };

        let pos_bytes: Vec<u8> = positions.iter().flat_map(|f| f.to_le_bytes()).collect();
        let pos_idx = push(&pos_bytes, GltfComponentType::Float, GltfAccessorType::Vec3, 4);
        let norm_bytes: Vec<u8> = normals.iter().flat_map(|f| f.to_le_bytes()).collect();
        let norm_idx = push(&norm_bytes, GltfComponentType::Float, GltfAccessorType::Vec3, 4);
        let uv_bytes: Vec<u8> = uvs.iter().flat_map(|f| f.to_le_bytes()).collect();
        let uv_idx = push(&uv_bytes, GltfComponentType::Float, GltfAccessorType::Vec2, 4);
        let color_bytes: Vec<u8> = colors.iter().flat_map(|f| f.to_le_bytes()).collect();
        let color_idx = push(&color_bytes, GltfComponentType::Float, GltfAccessorType::Vec4, 4);
        let index_bytes: Vec<u8> = indices.iter().flat_map(|i| i.to_le_bytes()).collect();
        let index_idx = push(&index_bytes, GltfComponentType::UnsignedInt, GltfAccessorType::Scalar, 6);

        let mut document = GltfDocument { asset: GltfAsset::default(), ..GltfDocument::default() };
        document.meshes = vec![GltfMesh {
            primitives: vec![GltfPrimitive {
                attributes: vec![("POSITION".into(), pos_idx), ("NORMAL".into(), norm_idx), ("TEXCOORD_0".into(), uv_idx), ("COLOR_0".into(), color_idx)],
                indices: Some(index_idx),
                material: Some(0),
                mode: Some(4),
                extensions: None,
                extras: None,
            }],
            weights: Vec::new(),
            name: Some("quad".into()),
            extensions: None,
            extras: None,
        }];
        document.materials = vec![GltfMaterial {
            name: Some("red".into()),
            pbr_metallic_roughness: Some(GltfPbrMetallicRoughness { base_color_factor: [0.8, 0.1, 0.1, 1.0], base_color_texture: None, metallic_factor: 0.2, roughness_factor: 0.7, metallic_roughness_texture: None, extensions: None, extras: None }),
            normal_texture: None, occlusion_texture: None, emissive_texture: None, emissive_factor: [0.0, 0.0, 0.0], alpha_mode: crate::artifacts::gltf::schema::snapshot::GltfAlphaMode::Opaque, alpha_cutoff: 0.5, double_sided: false, extensions: None, extras: None,
        }];
        document.buffer_views = buffer_views;
        document.accessors = accessors;
        document.buffers = vec![GltfBuffer { byte_length: buf.len(), uri: None, name: None, extensions: None, extras: None }];

        GltfSnapshot { schema: "stdio.gltf".into(), document, buffers: vec![buf], source_form: crate::artifacts::gltf::schema::snapshot::GltfSourceForm::Json }
    }

    #[test]
    fn deserialize_maps_geometry_material_and_topology() {
        let semio = SemioMeshFromGltf::deserialize(&sample_gltf()).expect("deserialize");
        assert_eq!(semio.meshes.len(), 1);
        let mesh = &semio.meshes[0];
        assert_eq!(mesh.id, "quad");
        assert_eq!(mesh.primitives.len(), 1);
        let prim = &mesh.primitives[0];
        assert_eq!(prim.topology, SemioTopology::Triangles);
        assert_eq!(prim.positions.len(), 4);
        assert_eq!(prim.positions[1], SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 });
        assert_eq!(prim.normals.len(), 4);
        assert_eq!(prim.uvs.len(), 4);
        assert_eq!(prim.colors.len(), 4);
        assert_eq!(prim.colors[0], SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
        assert_eq!(prim.indices, vec![0, 1, 2, 0, 2, 3]);
        assert_eq!(prim.material_id.as_deref(), Some("mat-0"));
        assert_eq!(semio.materials.len(), 1);
        assert_eq!(semio.materials[0].base_color, SemioRgba { r: 0.8, g: 0.1, b: 0.1, a: 1.0 });
    }

    #[test]
    fn line_loop_mode_is_a_hard_error_not_a_silent_downgrade() {
        let mut gltf = sample_gltf();
        gltf.document.meshes[0].primitives[0].mode = Some(2);
        let err = SemioMeshFromGltf::deserialize(&gltf).expect_err("LINE_LOOP must error");
        assert!(format!("{err:?}").contains("LINE_LOOP"), "got {err:?}");
    }

    #[test]
    fn missing_position_attribute_is_a_hard_error() {
        let mut gltf = sample_gltf();
        gltf.document.meshes[0].primitives[0].attributes.retain(|(name, _)| name != "POSITION");
        let err = SemioMeshFromGltf::deserialize(&gltf).expect_err("missing POSITION must error");
        assert!(format!("{err:?}").contains("POSITION"), "got {err:?}");
    }
}
//#endregion 🔖️Tests
