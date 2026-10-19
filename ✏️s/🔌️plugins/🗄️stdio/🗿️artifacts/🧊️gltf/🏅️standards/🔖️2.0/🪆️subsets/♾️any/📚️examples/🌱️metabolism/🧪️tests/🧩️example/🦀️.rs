//! 🧪️ Real-fixture tests for `stdio.gltf` against `🧊️base.glb` (genuine glTF 2.0, KHR material
//! extensions declared, 271 meshes). Ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 fixtures/tests three-test
//! pattern: (a) real decode with non-trivial invariants, (b) decode→encode→decode equality,
//! (c) analyzer→builder round-trip (the project's core acceptance test). F4: rewritten against the
//! fully typed `GltfDocument` model -- no `serde_json::Value` indexing anywhere in this file.

use semio_framework_test_fixtures::fixture_path;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfMesh;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfNode;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfScene;
use crate::standards::v2_0::subsets::any::GltfAnalyzer;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::decode_accessor;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::encode_glb;
use crate::standards::v2_0::subsets::any::{GltfAccessorSpec, GltfBuilderConstruction as GltfBuilder};
use semio_framework_plugin::{io::AnalyzeSource, ArtifactBuilder};

/// 🗂️ The committed fixture, read when a test runs: the example leaf lives in the editor crate, which a test here consumes as a dependency, so it has no host-less fallback of its own.
static BASE_GLB: std::sync::LazyLock<&'static [u8]> = std::sync::LazyLock::new(|| Box::leak(std::fs::read(fixture_path!("../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🖼️assets/🌱️metabolism/🏙️metabolism-model/🧊️.glb")).expect("the committed fixture is readable").into_boxed_slice()));

/// 🧬️ The fixture decoded through the real `.glb` container codec, as the example leaf does.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decoded_snapshot() -> semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot {
    decode_glb(*BASE_GLB).unwrap_or_else(|error| panic!("metabolism example base.glb decodes: {error}"))
}

//#region 🔖️(a) RealDecode
#[semio_framework_async_macros::async_test]
async fn base_glb_decodes_with_real_non_trivial_invariants() {
    let snap = decoded_snapshot();
    let doc = &snap.document;

    assert_eq!(doc.asset.version, "2.0");
    assert_eq!(doc.meshes.len(), 271);
    assert_eq!(doc.nodes.len(), 271);
    assert_eq!(doc.accessors.len(), 1095);
    assert_eq!(doc.buffer_views.len(), 1095);
    assert_eq!(doc.materials.len(), 2);

    for khr in ["KHR_materials_transmission", "KHR_materials_clearcoat", "KHR_materials_ior", "KHR_materials_specular", "KHR_materials_emissive_strength", "KHR_texture_transform"] {
        assert!(doc.extensions_used.iter().any(|e| e == khr), "expected {khr} in extensionsUsed, got {:?}", doc.extensions_used);
    }

    assert_eq!(doc.materials[0].pbr_metallic_roughness.as_ref().unwrap().base_color_factor, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(doc.materials[0].alpha_mode, GltfAlphaMode::Opaque);

    assert!(doc.buffers[0].uri.is_none());
    assert_eq!(doc.buffers[0].byte_length, 48064);
    assert_eq!(snap.buffers[0].len(), 48064);

    let mut component_types = std::collections::BTreeSet::new();
    let mut accessor_types = std::collections::BTreeSet::new();
    for acc in &doc.accessors {
        component_types.insert(acc.component_type.code());
        accessor_types.insert(acc.kind.as_str().to_string());
    }
    assert_eq!(component_types, std::collections::BTreeSet::from([5125u64, 5126]));
    assert_eq!(accessor_types, std::collections::BTreeSet::from(["SCALAR".to_string(), "VEC2".to_string(), "VEC3".to_string()]));

    let acc0 = decode_accessor(doc, &snap.buffers, 0).expect("decode accessor 0");
    assert_eq!(acc0.component_type, GltfComponentType::Float);
    assert_eq!(acc0.accessor_type, GltfAccessorType::Vec3);
    assert_eq!(acc0.count, 2);
    let expected = [-4.75f64, 7.5, 10.45, -10.25, 7.5, 10.45];
    for (got, want) in acc0.components.iter().zip(expected.iter()) {
        assert!((got - want).abs() < 1e-4, "accessor 0 component {got} not within epsilon of {want}");
    }

    assert_eq!(doc.nodes[0], GltfNode { mesh: Some(0), ..GltfNode::default() });
    assert_eq!(doc.meshes[0].primitives[0].attributes, vec![("POSITION".to_string(), 0)]);
    assert_eq!(doc.meshes[0].primitives[0].material, Some(0));
    assert_eq!(doc.meshes[0].primitives[0].mode, Some(3));
}
//#endregion 🔖️(a) RealDecode

//#region 🔖️(b) DecodeEncodeDecode
#[semio_framework_async_macros::async_test]
async fn base_glb_decode_encode_decode_is_semantically_equal() {
    let original = decode_glb(*BASE_GLB).expect("decode base.glb");
    let reencoded = encode_glb(&original).expect("re-encode");
    assert_eq!(&reencoded[0..4], b"glTF");
    let redecoded = decode_glb(&reencoded).expect("re-decode");

    assert_eq!(redecoded.buffers, original.buffers, "buffer bytes must survive decode->encode->decode exactly");
    assert_eq!(redecoded.document.accessors, original.document.accessors);
    assert_eq!(redecoded.document.buffer_views, original.document.buffer_views);
    assert_eq!(redecoded.document.meshes, original.document.meshes);
    assert_eq!(redecoded.document.materials, original.document.materials);
    assert_eq!(redecoded.document.nodes, original.document.nodes);
    assert_eq!(redecoded.document.scenes, original.document.scenes);
    assert_eq!(redecoded.document.extensions_used, original.document.extensions_used);

    let count = original.document.accessors.len();
    for i in 0..count {
        let before = decode_accessor(&original.document, &original.buffers, i).unwrap();
        let after = decode_accessor(&redecoded.document, &redecoded.buffers, i).unwrap();
        assert_eq!(before, after, "accessor {i} mismatch after decode->encode->decode");
    }
}
//#endregion 🔖️(b) DecodeEncodeDecode

//#region 🔖️(c) AnalyzerBuilderRoundTrip
/// 🧪️ The project's core acceptance test: analyze the real fixture, then reconstruct an
/// equivalent document using ONLY typed builder calls (no direct snapshot/document
/// construction), then prove the rebuilt document decodes to the same real data as the original.
#[semio_framework_async_macros::async_test]
async fn analyzer_builder_round_trip_reconstructs_equivalent_document() {
    let analysis = GltfAnalyzer::analyze(&[AnalyzeSource::Binary(*BASE_GLB)]);
    let original = analysis.parts.snapshot.expect("analyzer decodes base.glb");
    let doc = &original.document;

    let mut builder = GltfBuilder::empty();
    builder.set_asset_version(&doc.asset.version);

    for (i, buf) in original.buffers.iter().enumerate() {
        assert_eq!(builder.add_buffer(buf.clone()), i);
    }

    for bv in &doc.buffer_views {
        builder.add_buffer_view(bv.buffer, bv.byte_offset, bv.byte_length, bv.byte_stride, bv.target);
    }

    for acc in &doc.accessors {
        let mut spec = GltfAccessorSpec::new(acc.component_type, acc.kind, acc.count);
        if let Some(bv) = acc.buffer_view {
            spec = spec.with_buffer_view(bv, acc.byte_offset);
        }
        if let (Some(min), Some(max)) = (&acc.min, &acc.max) {
            spec = spec.with_min_max(min.clone(), max.clone());
        }
        builder.add_accessor(spec);
    }

    for mat in &doc.materials {
        builder.add_material(mat.clone());
    }

    for mesh in &doc.meshes {
        let mesh_idx = builder.add_mesh();
        for prim in &mesh.primitives {
            let attrs: Vec<(&str, usize)> = prim.attributes.iter().map(|(k, v)| (k.as_str(), *v)).collect();
            builder.add_mesh_primitive(mesh_idx, &attrs, prim.indices, prim.material, prim.mode);
        }
    }

    for node in &doc.nodes {
        builder.add_node(node.mesh);
    }

    for scene in &doc.scenes {
        builder.add_scene(scene.nodes.clone(), scene.extensions.clone());
    }
    if let Some(scene) = doc.scene {
        builder.set_default_scene(scene);
    }
    if !doc.extensions_used.is_empty() {
        builder.set_extensions_used(doc.extensions_used.clone());
    }

    let rebuilt = builder.build().expect("build rebuilt snapshot");

    assert_eq!(rebuilt.document.accessors, doc.accessors);
    assert_eq!(rebuilt.document.buffer_views, doc.buffer_views);
    assert_eq!(rebuilt.document.materials, doc.materials);
    assert_eq!(rebuilt.document.nodes, doc.nodes);
    assert_eq!(rebuilt.document.scenes, doc.scenes);
    assert_eq!(rebuilt.document.scene, doc.scene);
    assert_eq!(rebuilt.document.extensions_used, doc.extensions_used);
    assert_eq!(rebuilt.buffers, original.buffers);

    let orig_meshes: &Vec<GltfMesh> = &doc.meshes;
    let rebuilt_meshes: &Vec<GltfMesh> = &rebuilt.document.meshes;
    assert_eq!(orig_meshes.len(), rebuilt_meshes.len());
    for (om, rm) in orig_meshes.iter().zip(rebuilt_meshes.iter()) {
        assert_eq!(om.primitives, rm.primitives);
    }
    let _: &Vec<GltfScene> = &doc.scenes;

    let count = doc.accessors.len();
    assert_eq!(count, 1095);
    for i in 0..count {
        let before = decode_accessor(doc, &original.buffers, i).unwrap();
        let after = decode_accessor(&rebuilt.document, &rebuilt.buffers, i).unwrap();
        assert_eq!(before, after, "accessor {i} diverged between original and analyzer->builder rebuilt document");
    }
}
//#endregion 🔖️(c) AnalyzerBuilderRoundTrip
