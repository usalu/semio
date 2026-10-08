use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot_a() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        meshes: vec![SemioMesh {
            id: "m1".into(),
            primitives: vec![SemioPrimitive {
                id: "p1".into(),
                topology: SemioTopology::Triangles,
                positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }],
                normals: vec![],
                uvs: vec![],
                colors: vec![],
                indices: vec![0],
                material_id: Some("mat1".into()),
            }],
        }],
        materials: vec![SemioMaterial { id: "mat1".into(), base_color: SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, metallic: 0.0, roughness: 1.0, ..Default::default() }],
        textures: vec![SemioTexture { id: "tex1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
        ..Default::default()
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot_b() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        meshes: vec![SemioMesh {
            id: "m1".into(),
            primitives: vec![SemioPrimitive {
                id: "p1".into(),
                topology: SemioTopology::Lines,
                positions: vec![SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }],
                normals: vec![SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                uvs: vec![SemioUv { u: 0.5, v: 0.5 }],
                colors: vec![SemioRgba { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }],
                indices: vec![0, 1],
                material_id: None,
            }],
        }],
        materials: vec![SemioMaterial { id: "mat1".into(), base_color: SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, metallic: 1.0, roughness: 0.0, ..Default::default() }],
        textures: vec![SemioTexture { id: "tex1".into(), mime: "image/jpeg".into(), bytes: vec![4, 5] }],
        ..Default::default()
    }
}

/// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` round-trips through both
/// `print_diff`/`parse_diff` and `encode_diff`/`decode_diff`, over a real `between()` result
/// exercising the nested mesh -> primitive triple plus materials/textures.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioMeshDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioMeshDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn material_texture_refs_roundtrip_sparse_clear_inverse_and_absorb() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📸️snapshot/🧫️fixtures/🎨️material-textures/🔣️.json")).unwrap();
    let before = snapshot_a();
    let mut set = before.clone();
    set.materials[0].base_color_texture = Some(fixture["bindings"]["baseColorTexture"].as_str().unwrap().into());
    set.materials[0].metallic_roughness_texture = Some(fixture["bindings"]["metallicRoughnessTexture"].as_str().unwrap().into());
    set.materials[0].normal_texture = Some(fixture["bindings"]["normalTexture"].as_str().unwrap().into());
    set.materials[0].occlusion_texture = Some(fixture["bindings"]["occlusionTexture"].as_str().unwrap().into());
    set.materials[0].emissive_texture = Some(fixture["bindings"]["emissiveTexture"].as_str().unwrap().into());

    let textures = |change: SemioMaterialDiff| SemioMeshDiff { materials: Some(SemioMaterialsDiff { modified: vec![NamedModified { key: "mat1".into(), diff: change }], ..Default::default() }), ..Default::default() };
    let diff = textures(SemioMaterialDiff {
        base_color_texture: Some(Some(fixture["bindings"]["baseColorTexture"].as_str().unwrap().into())),
        metallic_roughness_texture: Some(Some(fixture["bindings"]["metallicRoughnessTexture"].as_str().unwrap().into())),
        normal_texture: Some(Some(fixture["bindings"]["normalTexture"].as_str().unwrap().into())),
        occlusion_texture: Some(Some(fixture["bindings"]["occlusionTexture"].as_str().unwrap().into())),
        emissive_texture: Some(Some(fixture["bindings"]["emissiveTexture"].as_str().unwrap().into())),
        ..Default::default()
    });
    assert_eq!(protocol::apply_diff(&diff, &before).unwrap(), set);
    assert_eq!(protocol::apply_diff(&diff.inverse(&before), &set).unwrap(), before);
    let mut cleared = set.clone();
    cleared.materials[0].normal_texture = None;
    let clear = textures(SemioMaterialDiff { normal_texture: Some(None), ..Default::default() });
    assert_eq!(clear.materials.as_ref().unwrap().modified[0].diff.normal_texture, Some(None));
    assert_eq!(protocol::apply_diff(&clear.inverse(&set), &cleared).unwrap(), set);
    for value in [&diff, &clear] {
        assert_eq!(SemioMeshDiff::parse_diff(&value.print_diff()).unwrap(), *value);
        assert_eq!(SemioMeshDiff::decode_diff(&value.encode_diff().unwrap()).unwrap(), *value);
    }
    let mut combined = diff;
    combined.absorb(clear);
    assert_eq!(protocol::apply_diff(&combined, &before).unwrap(), cleared);
    println!("[DEBUG] Five Semio material texture references survive sparse codecs, inverse and absorb");
}
