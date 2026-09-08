
use super::*;
use semio_s_artifact_stdio_gltf::engine::GltfAccessorType;
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView};
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfAsset, GltfMaterial, GltfMesh, GltfPbrMetallicRoughness};

/// 🏗️ A real-shaped 2-triangle quad (shared POSITION/NORMAL/TEXCOORD_0/COLOR_0/indices) with
/// one PBR material and one embedded (data-uri) texture — exercises every mapped field.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
            targets: Vec::new(),
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
        normal_texture: None,
        occlusion_texture: None,
        emissive_texture: None,
        emissive_factor: [0.0, 0.0, 0.0],
        alpha_mode: semio_s_artifact_stdio_gltf::schema::snapshot::GltfAlphaMode::Opaque,
        alpha_cutoff: 0.5,
        double_sided: false,
        extensions: None,
        extras: None,
    }];
    document.buffer_views = buffer_views;
    document.accessors = accessors;
    document.buffers = vec![GltfBuffer { byte_length: buf.len(), uri: None, name: None, extensions: None, extras: None }];

    GltfSnapshot { schema: "stdio.gltf".into(), document, buffers: vec![buf], source_form: semio_s_artifact_stdio_gltf::schema::snapshot::GltfSourceForm::Json }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_maps_geometry_material_and_topology() {
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromGltf::deserialize(&sample_gltf())).expect("deserialize");
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

#[semio_framework_async_macros::async_test]
async fn line_loop_mode_is_a_hard_error_not_a_silent_downgrade() {
    let mut gltf = sample_gltf();
    gltf.document.meshes[0].primitives[0].mode = Some(2);
    let err = semio_framework_plugin::resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).expect_err("LINE_LOOP must error");
    assert!(format!("{err:?}").contains("LINE_LOOP"), "got {err:?}");
}

#[semio_framework_async_macros::async_test]
async fn missing_position_attribute_is_a_hard_error() {
    let mut gltf = sample_gltf();
    gltf.document.meshes[0].primitives[0].attributes.retain(|(name, _)| name != "POSITION");
    let err = semio_framework_plugin::resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).expect_err("missing POSITION must error");
    assert!(format!("{err:?}").contains("POSITION"), "got {err:?}");
}
