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
//! - Texture bindings retain their actual image source IDs. Sampler settings, alternate UV sets,
//!   normal scale, occlusion strength and other glTF material extensions remain outside this contract.

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_gltf::engine::{decode_accessor, decode_data_uri, GltfComponentType};
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfDocument, GltfImage, GltfPrimitive};
use semio_s_artifact_stdio_gltf::GltfSnapshot;

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

/// 🖼️️ Resolves one `image`'s raw bytes: embedded `bufferView` first, then a `data:` uri; an
/// external image payloads require a supplied buffer view and otherwise fail explicitly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_image_bytes(document: &GltfDocument, buffers: &[Vec<u8>], image: &GltfImage) -> Result<Vec<u8>, store::PackError> {
    if let Some(index) = image.buffer_view {
        let view = document.buffer_views.get(index).ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SemioMeshFromGltf: image buffer view is out of bounds")))?;
        let buffer = buffers.get(view.buffer).ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SemioMeshFromGltf: image buffer is unavailable")))?;
        let end = view.byte_offset.checked_add(view.byte_length).ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SemioMeshFromGltf: image buffer range overflow")))?;
        return buffer.get(view.byte_offset..end).map(|bytes| bytes.to_vec()).ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SemioMeshFromGltf: image bytes are out of bounds")));
    }
    match image.uri.as_deref() {
        Some(uri) if uri.starts_with("data:") => decode_data_uri(uri).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("SemioMeshFromGltf: invalid image data URI: {error}")))),
        _ => Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SemioMeshFromGltf: image bytes must be embedded or supplied through a buffer view"))),
    }
}
fn texture_id(document: &GltfDocument, index: Option<usize>) -> Result<Option<String>, store::PackError> {
    index.map(|index| {
        let source = document.textures.get(index).and_then(|texture| texture.source).ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SemioMeshFromGltf: material texture source is unavailable")))?;
        if source >= document.images.len() { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "SemioMeshFromGltf: material image source is out of bounds"))); }
        Ok(format!("tex-{source}"))
    }).transpose()
}
//#endregion 🔖️AccessorHelpers

//#region 🔖️PrimitiveMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_primitive(document: &GltfDocument, buffers: &[Vec<u8>], prim: &GltfPrimitive, id: String, material_id: Option<String>) -> Result<SemioPrimitive, String> {
    let topology = gltf_mode_to_topology(prim.mode)?;
    let position = decode_accessor(document, buffers, find_attr(&prim.attributes, "POSITION").ok_or("primitive missing mandatory POSITION attribute")?)?;
    if position.accessor_type.components() != 3 || position.count == 0 || position.count > u32::MAX as usize || position.components.iter().any(|value| !value.is_finite() || !(*value as f32).is_finite()) { return Err("POSITION needs a finite three-component vertex domain".into()); }
    let positions = position.components.chunks_exact(3).map(|values| SemioPoint3 { x: values[0], y: values[1], z: values[2] }).collect();
    let normals = match find_attr(&prim.attributes, "NORMAL") {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            if values.accessor_type.components() != 3 || values.count != position.count || values.components.iter().any(|value| !value.is_finite()) || values.components.chunks_exact(3).any(|value| value.iter().map(|value| value * value).sum::<f64>() <= 1e-24) { return Err("NORMAL needs one finite nonzero three-component tuple per vertex".into()); }
            values.components.chunks_exact(3).map(|values| SemioPoint3 { x: values[0], y: values[1], z: values[2] }).collect()
        }, None => Vec::new(),
    };
    let uvs = match find_attr(&prim.attributes, "TEXCOORD_0") {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            if values.accessor_type.components() != 2 || values.count != position.count || values.components.iter().any(|value| !value.is_finite() || !(*value as f32).is_finite()) { return Err("TEXCOORD_0 needs one finite two-component tuple per vertex".into()); }
            values.components.chunks_exact(2).map(|values| SemioUv { u: values[0], v: values[1] }).collect()
        }, None => Vec::new(),
    };
    let colors = match find_attr(&prim.attributes, "COLOR_0") {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            let width = values.accessor_type.components();
            if !matches!(width, 3 | 4) || values.count != position.count || values.components.iter().any(|value| !value.is_finite() || !(0.0..=1.0).contains(value)) { return Err("COLOR_0 needs one normalized three- or four-component tuple per vertex".into()); }
            values.components.chunks_exact(width).map(|values| SemioRgba { r: values[0] as f32, g: values[1] as f32, b: values[2] as f32, a: if width == 4 { values[3] as f32 } else { 1.0 } }).collect()
        }, None => Vec::new(),
    };
    let indices = match prim.indices {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            let maximum = match values.component_type { GltfComponentType::UnsignedByte => u8::MAX as f64, GltfComponentType::UnsignedShort => u16::MAX as f64, GltfComponentType::UnsignedInt => u32::MAX as f64, _ => return Err("primitive indices need an unsigned integer component type".into()) };
            if values.accessor_type.components() != 1 || values.normalized || values.components.iter().any(|value| !value.is_finite() || value.fract() != 0.0 || *value < 0.0 || *value >= position.count as f64 || *value == maximum) { return Err("primitive indices need unnormalized unsigned scalar values inside the vertex domain".into()); }
            values.components.iter().map(|value| *value as u32).collect()
        }, None => Vec::new(),
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
                Ok(SemioMaterial {
                    id: format!("mat-{i}"),
                    base_color: SemioRgba { r: pbr.base_color_factor[0] as f32, g: pbr.base_color_factor[1] as f32, b: pbr.base_color_factor[2] as f32, a: pbr.base_color_factor[3] as f32 },
                    metallic: pbr.metallic_factor as f32,
                    roughness: pbr.roughness_factor as f32,
                    base_color_texture: texture_id(document, pbr.base_color_texture.as_ref().map(|info| info.index))?,
                    metallic_roughness_texture: texture_id(document, pbr.metallic_roughness_texture.as_ref().map(|info| info.index))?,
                    normal_texture: texture_id(document, m.normal_texture.as_ref().map(|info| info.index))?,
                    occlusion_texture: texture_id(document, m.occlusion_texture.as_ref().map(|info| info.index))?,
                    emissive_texture: texture_id(document, m.emissive_texture.as_ref().map(|info| info.index))?,
                })
            })
            .collect::<Result<_, store::PackError>>()?;

        let textures: Vec<SemioTexture> = document.images.iter().enumerate().map(|(i, img)| Ok(SemioTexture { id: format!("tex-{i}"), mime: img.mime_type.clone().unwrap_or_else(|| img.uri.as_deref().and_then(|uri| uri.strip_prefix("data:")).and_then(|value| value.split_once(';')).map(|(mime,_)|mime.to_owned()).unwrap_or_default()), bytes: resolve_image_bytes(document, &from.buffers, img)? })).collect::<Result<_, store::PackError>>()?;

        let mut meshes = Vec::with_capacity(document.meshes.len());
        for (mi, gmesh) in document.meshes.iter().enumerate() {
            let mesh_id = gmesh.name.clone().unwrap_or_else(|| format!("mesh-{mi}"));
            let mut primitives = Vec::with_capacity(gmesh.primitives.len());
            for (pi, prim) in gmesh.primitives.iter().enumerate() {
                let material_id = prim.material.map(|idx| format!("mat-{idx}"));
                let sp = decode_primitive(document, &from.buffers, prim, format!("{mesh_id}-prim-{pi}"), material_id).map_err(|e| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("SemioMeshFromGltf: mesh {mi} primitive {pi}: {e}"))))?;
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
