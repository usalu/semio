//! 📤️ Serialize `s.stdio.semio/v1/mesh` into `s.stdio.gltf/2.0/*` — mirror image of the sibling
//! deserializer leaf. Builds one packed little-endian buffer + tightly-packed accessors/
//! bufferViews per attribute (no interleaving — simplest correct layout; every accessor gets its
//! own bufferView), leaning on the gltf artifact's own `engine::encode_data_uri` for texture bytes
//! and on `ArtifactPack`/`serialize_gltf_document` (called downstream by the generic
//! `serializer_entry_of` erasure, not here) to embed the geometry buffer as a `data:` uri on
//! actual JSON emission (buffers left with `uri: None` here, per that function's own contract).
//!
//! 🔖 Documented lossiness (mirrors the deserializer's list): `SemioMaterial`'s scalar-only PBR
//! fields produce a material with no texture references (`base_color_texture` etc. always `None`);
//! `SemioTexture`s are emitted as `images`/`textures` entries but nothing in this schema
//! (materials have no texture indices) ever references them by index -- still real, valid gltf,
//! just unreferenced, exactly mirroring what the deserializer harvests independently of material
//! texture refs. `SemioMeshSnapshot` has no scene graph -- `scenes`/`nodes` are left empty.

use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTopology};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_gltf::engine::{encode_data_uri, GltfAccessorType, GltfComponentType};
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfAccessor, GltfAlphaMode, GltfBuffer, GltfBufferView, GltfDocument, GltfImage, GltfMaterial, GltfMesh, GltfPbrMetallicRoughness, GltfPrimitive, GltfSourceForm, GltfTexture};
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use semio_s_artifact_stdio_gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
use std::collections::HashMap;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };

//#region 🔖️Topology
/// 🔺️ `SemioTopology` -> gltf `primitive.mode` (§5.19.4) -- total (every semio variant has a real
/// gltf mode; the partial direction is the deserializer's `LINE_LOOP` gap, not this one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn topology_to_gltf_mode(topology: SemioTopology) -> u64 {
    match topology {
        SemioTopology::Points => 0,
        SemioTopology::Lines => 1,
        SemioTopology::LineStrip => 3,
        SemioTopology::Triangles => 4,
        SemioTopology::TriangleStrip => 5,
        SemioTopology::TriangleFan => 6,
    }
}
//#endregion 🔖️Topology

//#region 🔖️AccessorBuilder
/// 🏗️ Appends `values` (already flattened row-major) as `component_type`-encoded little-endian
/// bytes to `buf`, registers one tightly-packed `bufferView` + `accessor` for them, and returns
/// the new accessor's index. Only `Float`/`UnsignedInt` are exercised by this codec (positions/
/// normals/uvs/colors as Float, indices as UnsignedInt).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_accessor(buf: &mut Vec<u8>, buffer_views: &mut Vec<GltfBufferView>, accessors: &mut Vec<GltfAccessor>, component_type: GltfComponentType, accessor_type: GltfAccessorType, values: &[f64], count: usize) -> usize {
    let byte_offset = buf.len();
    for &v in values {
        match component_type {
            GltfComponentType::Float => buf.extend_from_slice(&(v as f32).to_le_bytes()),
            GltfComponentType::UnsignedInt => buf.extend_from_slice(&(v as u32).to_le_bytes()),
            other => unreachable!("push_accessor: mesh<->gltf codec only emits Float/UnsignedInt, got {other:?}"),
        }
    }
    let byte_length = buf.len() - byte_offset;
    let bv_idx = buffer_views.len();
    buffer_views.push(GltfBufferView { buffer: 0, byte_offset, byte_length, byte_stride: None, target: None, name: None, extensions: None, extras: None });
    let acc_idx = accessors.len();
    accessors.push(GltfAccessor { buffer_view: Some(bv_idx), byte_offset: 0, component_type, normalized: false, count, kind: accessor_type, max: None, min: None, sparse: None, name: None, extensions: None, extras: None });
    acc_idx
}
//#endregion 🔖️AccessorBuilder

pub struct SemioMeshToGltf;

impl ArtifactSerializer for SemioMeshToGltf {
    type From = SemioMeshSnapshot;
    type Into = GltfSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut buf: Vec<u8> = Vec::new();
        let mut buffer_views: Vec<GltfBufferView> = Vec::new();
        let mut accessors: Vec<GltfAccessor> = Vec::new();

        let material_index_of: HashMap<&str, usize> = from.materials.iter().enumerate().map(|(i, m)| (m.id.as_str(), i)).collect();

        let mut gltf_meshes = Vec::with_capacity(from.meshes.len());
        for mesh in &from.meshes {
            let mut gprims = Vec::with_capacity(mesh.primitives.len());
            for prim in &mesh.primitives {
                if prim.positions.is_empty() {
                    return Err(store::PackError::Schema(format!("SemioMeshToGltf: primitive {:?} has no positions; gltf POSITION is mandatory", prim.id)));
                }
                if !prim.normals.is_empty() && prim.normals.len() != prim.positions.len() {
                    return Err(store::PackError::Schema(format!("SemioMeshToGltf: primitive {:?} normals length {} != positions length {}", prim.id, prim.normals.len(), prim.positions.len())));
                }
                if !prim.uvs.is_empty() && prim.uvs.len() != prim.positions.len() {
                    return Err(store::PackError::Schema(format!("SemioMeshToGltf: primitive {:?} uvs length {} != positions length {}", prim.id, prim.uvs.len(), prim.positions.len())));
                }
                if !prim.colors.is_empty() && prim.colors.len() != prim.positions.len() {
                    return Err(store::PackError::Schema(format!("SemioMeshToGltf: primitive {:?} colors length {} != positions length {}", prim.id, prim.colors.len(), prim.positions.len())));
                }

                let mut attributes = Vec::new();
                let pos_values: Vec<f64> = prim.positions.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
                let pos_idx = push_accessor(&mut buf, &mut buffer_views, &mut accessors, GltfComponentType::Float, GltfAccessorType::Vec3, &pos_values, prim.positions.len());
                attributes.push(("POSITION".to_string(), pos_idx));

                if !prim.normals.is_empty() {
                    let v: Vec<f64> = prim.normals.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
                    let idx = push_accessor(&mut buf, &mut buffer_views, &mut accessors, GltfComponentType::Float, GltfAccessorType::Vec3, &v, prim.normals.len());
                    attributes.push(("NORMAL".to_string(), idx));
                }
                if !prim.uvs.is_empty() {
                    let v: Vec<f64> = prim.uvs.iter().flat_map(|p| [p.u, p.v]).collect();
                    let idx = push_accessor(&mut buf, &mut buffer_views, &mut accessors, GltfComponentType::Float, GltfAccessorType::Vec2, &v, prim.uvs.len());
                    attributes.push(("TEXCOORD_0".to_string(), idx));
                }
                if !prim.colors.is_empty() {
                    let v: Vec<f64> = prim.colors.iter().flat_map(|c| [c.r as f64, c.g as f64, c.b as f64, c.a as f64]).collect();
                    let idx = push_accessor(&mut buf, &mut buffer_views, &mut accessors, GltfComponentType::Float, GltfAccessorType::Vec4, &v, prim.colors.len());
                    attributes.push(("COLOR_0".to_string(), idx));
                }

                let indices = if !prim.indices.is_empty() {
                    let v: Vec<f64> = prim.indices.iter().map(|&i| i as f64).collect();
                    Some(push_accessor(&mut buf, &mut buffer_views, &mut accessors, GltfComponentType::UnsignedInt, GltfAccessorType::Scalar, &v, prim.indices.len()))
                } else {
                    None
                };

                let material = match &prim.material_id {
                    Some(id) => {
                        let idx = material_index_of.get(id.as_str()).copied().ok_or_else(|| store::PackError::Schema(format!("SemioMeshToGltf: primitive {:?} references unknown material {id:?}", prim.id)))?;
                        Some(idx)
                    }
                    None => None,
                };

                gprims.push(GltfPrimitive { attributes, indices, material, mode: Some(topology_to_gltf_mode(prim.topology)), targets: Vec::new(), extensions: None, extras: None });
            }
            gltf_meshes.push(GltfMesh { primitives: gprims, weights: Vec::new(), name: Some(mesh.id.clone()), extensions: None, extras: None });
        }

        let gltf_materials: Vec<GltfMaterial> = from
            .materials
            .iter()
            .map(|m| GltfMaterial {
                name: Some(m.id.clone()),
                pbr_metallic_roughness: Some(GltfPbrMetallicRoughness {
                    base_color_factor: [m.base_color.r as f64, m.base_color.g as f64, m.base_color.b as f64, m.base_color.a as f64],
                    base_color_texture: None,
                    metallic_factor: m.metallic as f64,
                    roughness_factor: m.roughness as f64,
                    metallic_roughness_texture: None,
                    extensions: None,
                    extras: None,
                }),
                normal_texture: None,
                occlusion_texture: None,
                emissive_texture: None,
                emissive_factor: [0.0, 0.0, 0.0],
                alpha_mode: GltfAlphaMode::Opaque,
                alpha_cutoff: 0.5,
                double_sided: false,
                extensions: None,
                extras: None,
            })
            .collect();

        let mut gltf_images = Vec::with_capacity(from.textures.len());
        let mut gltf_textures = Vec::with_capacity(from.textures.len());
        for tex in &from.textures {
            let mime = if tex.mime.is_empty() { "application/octet-stream".to_string() } else { tex.mime.clone() };
            let img_idx = gltf_images.len();
            gltf_images.push(GltfImage { uri: Some(encode_data_uri(&mime, &tex.bytes)), mime_type: Some(tex.mime.clone()), buffer_view: None, name: Some(tex.id.clone()), extensions: None, extras: None });
            gltf_textures.push(GltfTexture { sampler: None, source: Some(img_idx), name: Some(tex.id.clone()), extensions: None, extras: None });
        }

        let mut document = GltfDocument { meshes: gltf_meshes, materials: gltf_materials, images: gltf_images, textures: gltf_textures, accessors, buffer_views, ..GltfDocument::default() };
        let buffers = if buf.is_empty() {
            Vec::new()
        } else {
            document.buffers.push(GltfBuffer { byte_length: buf.len(), uri: None, name: None, extensions: None, extras: None });
            vec![buf]
        };

        Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Json })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
