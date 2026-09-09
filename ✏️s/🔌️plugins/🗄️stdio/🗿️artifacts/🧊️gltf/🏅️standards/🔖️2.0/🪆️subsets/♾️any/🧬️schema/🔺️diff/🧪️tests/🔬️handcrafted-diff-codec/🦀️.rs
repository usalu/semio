use super::*;
use crate::STDIO_GLTF_DOCUMENT_SCHEMA;
use protocol::DiffCodec;

//#region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node_tristate_a() -> GltfNode {
    GltfNode {
        children: vec![0, 1],
        mesh: Some(0),
        camera: Some(0),
        skin: Some(0),
        matrix: Some([1.0; 16]),
        translation: None,
        rotation: None,
        scale: None,
        weights: vec![0.5, 0.5],
        name: Some("n-a".into()),
        extensions: Some(GltfJson::Bool(true)),
        extras: None,
    }
}
/// 🎯️ Every one of `node_tristate_a`'s nullable fields flips the OTHER way (`Some -> None` OR
/// `None -> Some`), so `between()` exercises `Some(None)` on `mesh`/`camera`/`skin`/`matrix`/
/// `extensions` AND `Some(Some(_))` on `translation`/`rotation`/`scale`/`extras` in one pair.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node_tristate_b() -> GltfNode {
    GltfNode {
        children: vec![2],
        mesh: None,
        camera: None,
        skin: None,
        matrix: None,
        translation: Some([1.0, 2.0, 3.0]),
        rotation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: Some([2.0, 2.0, 2.0]),
        weights: vec![],
        name: None,
        extensions: None,
        extras: Some(GltfJson::Array(vec![GltfJson::Null, GltfJson::Number(-1.5), GltfJson::String("x".into())])),
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn accessor_sparse_a() -> GltfAccessor {
    GltfAccessor { buffer_view: Some(1), byte_offset: 0, component_type: GltfComponentType::UnsignedShort, normalized: true, count: 4, kind: GltfAccessorType::Vec2, max: None, min: None, sparse: None, name: None, extensions: None, extras: None }
}
/// 🎯️ `sparse` flips `None -> Some(GltfSparseAccessor{..})` -- the one accessor field not
/// exercised by `sweep_a`/`sweep_b` above.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn accessor_sparse_b() -> GltfAccessor {
    GltfAccessor {
        sparse: Some(GltfSparseAccessor { count: 2, indices: GltfSparseIndices { buffer_view: 2, byte_offset: 0, component_type: GltfComponentType::UnsignedByte }, values: GltfSparseValues { buffer_view: 3, byte_offset: 0 } }),
        max: Some(vec![1.0, 1.0]),
        ..accessor_sparse_a()
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn material_textures_a() -> GltfMaterial {
    GltfMaterial::default()
}
/// 🎯️ Every optional texture slot (`pbr_metallic_roughness`/`normal_texture`/
/// `occlusion_texture`/`emissive_texture`) flips `None -> Some(_)` -- none of which
/// `sweep_a`/`sweep_b` above touch (both leave `GltfMaterial::default()`'s texture slots at
/// `None`).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn material_textures_b() -> GltfMaterial {
    GltfMaterial {
        pbr_metallic_roughness: Some(GltfPbrMetallicRoughness { base_color_texture: Some(GltfTextureInfo { index: 0, tex_coord: 1, extensions: None, extras: None }), ..Default::default() }),
        normal_texture: Some(GltfNormalTextureInfo { index: 1, tex_coord: 0, scale: 2.0, extensions: None, extras: None }),
        occlusion_texture: Some(GltfOcclusionTextureInfo { index: 2, tex_coord: 0, strength: 0.5, extensions: None, extras: None }),
        emissive_texture: Some(GltfTextureInfo { index: 3, tex_coord: 0, extensions: None, extras: None }),
        alpha_mode: GltfAlphaMode::Mask,
        ..GltfMaterial::default()
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn buffer_uri_a() -> GltfBuffer {
    GltfBuffer { byte_length: 4, uri: Some("data:...".into()), name: None, extensions: None, extras: None }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn buffer_uri_b() -> GltfBuffer {
    GltfBuffer { uri: None, ..buffer_uri_a() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn camera_orthographic() -> GltfCamera {
    GltfCamera { projection: GltfCameraProjection::Orthographic(GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: Some(GltfJson::Null) }), name: None, extensions: None, extras: None }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn tristate_snapshot_a() -> GltfSnapshot {
    GltfSnapshot {
        schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        document: GltfDocument { asset: GltfAsset::default(), nodes: vec![node_tristate_a()], accessors: vec![accessor_sparse_a()], materials: vec![material_textures_a()], buffers: vec![buffer_uri_a()], cameras: vec![], ..GltfDocument::default() },
        buffers: vec![vec![1, 2, 3]],
        source_form: GltfSourceForm::Json,
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn tristate_snapshot_b() -> GltfSnapshot {
    GltfSnapshot {
        schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        document: GltfDocument {
            asset: GltfAsset::default(),
            nodes: vec![node_tristate_b()],
            accessors: vec![accessor_sparse_b()],
            materials: vec![material_textures_b()],
            buffers: vec![buffer_uri_b()],
            cameras: vec![camera_orthographic()],
            ..GltfDocument::default()
        },
        buffers: vec![vec![1, 2, 3]],
        source_form: GltfSourceForm::Json,
    }
}
//#endregion 🔖️Fixtures

/// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `GltfDiff` grammar. Exercises a
/// representative SUBSET of the 42 tri-state fields (not literally all 42, per the F6 brief):
/// (1) `sweep_a()`/`sweep_b()` (this file's own `field_sweep_covers_every_mutable_field`
/// fixture, reused verbatim) — every top-level `GltfDiff` field populated at least once
/// (`asset`/`scene`/all 14 collections/`extensions_used`/`extensions_required`/`extensions`/
/// `extras`/`source_form`), a `Perspective` camera, and every `GltfAssetDiff` tri-state field
/// going `Some -> None`; (2) `tristate_snapshot_a/b` — the tri-state fields `sweep_a`/`sweep_b`
/// do NOT touch: `GltfNodeDiff::mesh/camera/skin/matrix` going `Some(Some) -> Some(None)`,
/// `translation/rotation/scale` going `Some(None) -> Some(Some)` (both tri-state directions on
/// the SAME collection-modified entry), `GltfAccessorDiff::sparse` going `None -> Some`,
/// `GltfMaterialDiff::pbr_metallic_roughness/normal_texture/occlusion_texture/emissive_texture`
/// all going `None -> Some`, `GltfBufferDiff::uri` going `Some -> None`, an `Orthographic`
/// camera (the OTHER `GltfCameraProjection` variant `sweep_b` doesn't use), and `GltfJson`'s
/// `Null`/`Number`/`Array` variants (`sweep_a`/`sweep_b` only exercise `Bool`/`String`/
/// `Object`) -- together (1)+(2) cover every `GltfJson` variant, both `GltfCameraProjection`
/// variants, and at least one tri-state field per STRONG entity diff type
/// (`Asset`/`Scene`/`Node`/`Mesh`/`Accessor`/`Material`/`Buffer`), which is the representative
/// slice this law test commits to (documented here, not literally all 42 occurrences).
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    let sweep_a = tests::sweep_a();
    let sweep_b = tests::sweep_b();
    let tri_a = tristate_snapshot_a();
    let tri_b = tristate_snapshot_b();

    let cases = vec![
        GltfDiff::default(),
        <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_b),
        <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_b, &sweep_a),
        <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&tri_a, &tri_b),
        <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&tri_b, &tri_a),
    ];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = GltfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = GltfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
