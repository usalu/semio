
use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::DiffCodec;

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
        materials: vec![SemioMaterial { id: "mat1".into(), base_color: SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, metallic: 0.0, roughness: 1.0 }],
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
        materials: vec![SemioMaterial { id: "mat1".into(), base_color: SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, metallic: 1.0, roughness: 0.0 }],
        textures: vec![SemioTexture { id: "tex1".into(), mime: "image/jpeg".into(), bytes: vec![4, 5] }],
        ..Default::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn between_apply_and_inverse_round_trip() {
    let a = snapshot_a();
    let b = snapshot_b();
    let d = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b);
    assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    let inv = d.inverse(&a);
    assert_eq!(inv.apply(&d.apply(&a).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"), a);
    assert!(<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn absorb_composes_two_sequential_diffs() {
    let a = snapshot_a();
    let mid = snapshot_b();
    let mut after = mid.clone();
    after.materials[0].metallic = 0.42;
    let mut d1 = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &mid);
    let d2 = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&mid, &after);
    let applied_before_absorb = d1.apply(&a).expect("apply must succeed for a well-formed fixture");
    d1.absorb(d2.clone());
    assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), d2.apply(&applied_before_absorb).expect("apply must succeed for a well-formed fixture"));
    assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), after);
}

/// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` round-trips through both
/// `print_diff`/`parse_diff` and `encode_diff`/`decode_diff`, over a real `between()` result
/// exercising the nested mesh -> primitive triple plus materials/textures.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot_a();
    let b = snapshot_b();
    let cases =
        vec![SemioMeshDiff::default(), <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b), <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&b, &a), <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioMeshDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioMeshDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }

    // Confirm nested + tri-state coverage genuinely got exercised above.
    let diff_ab = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b);
    let meshes = diff_ab.meshes.as_ref().expect("meshes diff present");
    let mesh_mod = meshes.modified.iter().find(|m| m.key == "m1").expect("m1 modified");
    let prims = mesh_mod.diff.primitives.as_ref().expect("primitives diff present");
    let prim_mod = prims.modified.iter().find(|p| p.key == "p1").expect("p1 modified");
    assert_eq!(prim_mod.diff.material_id, Some(None), "material_id tri-state Some(None) not exercised");
    let diff_ba = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&b, &a);
    let prim_mod_ba = diff_ba.meshes.as_ref().unwrap().modified[0].diff.primitives.as_ref().unwrap().modified.iter().find(|p| p.key == "p1").unwrap();
    assert_eq!(prim_mod_ba.diff.material_id, Some(Some("mat1".to_string())), "material_id tri-state Some(Some(_)) not exercised");
}
