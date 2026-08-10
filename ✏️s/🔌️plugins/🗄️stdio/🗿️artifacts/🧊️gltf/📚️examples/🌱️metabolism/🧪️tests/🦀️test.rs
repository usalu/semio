//! 🧪️ Real-fixture tests for `stdio.gltf` against `🧊️base.glb` (genuine glTF 2.0, KHR material
//! extensions declared, 271 meshes). Ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 fixtures/tests three-test
//! pattern: (a) real decode with non-trivial invariants, (b) decode→encode→decode equality,
//! (c) analyzer→builder round-trip (the project's core acceptance test).

use crate::artifacts::gltf::engine::{decode_accessor, decode_glb, encode_glb, GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::examples::metabolism::{decoded_snapshot, BASE_GLB_BYTES};
use crate::artifacts::gltf::standards::v2_0::subsets::any::analyzer::GltfAnalyzer;
use crate::artifacts::gltf::standards::v2_0::subsets::any::builder::{GltfAccessorSpec, GltfBuilder};
use semio_framework_plugin::{AnalyzeSource, ArtifactAnalyzer, ArtifactBuilder};

//#region 🔖️(a) RealDecode
#[test]
fn base_glb_decodes_with_real_non_trivial_invariants() {
    let snap = decoded_snapshot();
    let doc = &snap.document;

    assert_eq!(doc["asset"]["version"], "2.0");
    assert_eq!(doc["meshes"].as_array().unwrap().len(), 271);
    assert_eq!(doc["nodes"].as_array().unwrap().len(), 271);
    assert_eq!(doc["accessors"].as_array().unwrap().len(), 1095);
    assert_eq!(doc["bufferViews"].as_array().unwrap().len(), 1095);
    assert_eq!(doc["materials"].as_array().unwrap().len(), 2);

    // KHR material extensions genuinely declared on this fixture (Decision #4's "gltf with KHR
    // material extensions" fixture claim -- verified against the real JSON chunk, not assumed).
    let extensions_used: Vec<&str> = doc["extensionsUsed"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    for khr in ["KHR_materials_transmission", "KHR_materials_clearcoat", "KHR_materials_ior", "KHR_materials_specular", "KHR_materials_emissive_strength", "KHR_texture_transform"] {
        assert!(extensions_used.contains(&khr), "expected {khr} in extensionsUsed, got {extensions_used:?}");
    }

    // material[0]: a real pbrMetallicRoughness.baseColorFactor, not a stub.
    assert_eq!(doc["materials"][0]["pbrMetallicRoughness"]["baseColorFactor"], serde_json::json!([0.0, 0.0, 0.0, 1.0]));

    // buffer 0 is embedded (glb BIN chunk, no uri) and its real bytes were resolved.
    assert!(doc["buffers"][0].get("uri").is_none());
    assert_eq!(doc["buffers"][0]["byteLength"], 48064);
    assert_eq!(snap.buffers[0].len(), 48064);

    // real componentType/type variety actually present (5126 FLOAT VEC3/VEC2, 5125 UNSIGNED_INT
    // SCALAR indices) -- proves the typed accessor model, not just JSON pass-through.
    let mut component_types = std::collections::BTreeSet::new();
    let mut accessor_types = std::collections::BTreeSet::new();
    for acc in doc["accessors"].as_array().unwrap() {
        component_types.insert(acc["componentType"].as_u64().unwrap());
        accessor_types.insert(acc["type"].as_str().unwrap().to_string());
    }
    assert_eq!(component_types, std::collections::BTreeSet::from([5125u64, 5126]));
    assert_eq!(accessor_types, std::collections::BTreeSet::from(["SCALAR".to_string(), "VEC2".to_string(), "VEC3".to_string()]));

    // accessor 0's real decoded vertex data matches its own declared min/max exactly (2 verts).
    let acc0 = decode_accessor(doc, &snap.buffers, 0).expect("decode accessor 0");
    assert_eq!(acc0.component_type, GltfComponentType::Float);
    assert_eq!(acc0.accessor_type, GltfAccessorType::Vec3);
    assert_eq!(acc0.count, 2);
    // Real f32 buffer data widened to f64 -- compare against the accessor's own declared min/max
    // with an epsilon (10.45 has no exact f32 representation: it decodes to 10.449999809265137),
    // not a hand-typed literal that silently assumes exact round-tripping.
    let expected = [-4.75f64, 7.5, 10.45, -10.25, 7.5, 10.45];
    for (got, want) in acc0.components.iter().zip(expected.iter()) {
        assert!((got - want).abs() < 1e-4, "accessor 0 component {got} not within epsilon of {want}");
    }

    // node[0]/mesh[0] wiring: real, specific, not a placeholder.
    assert_eq!(doc["nodes"][0], serde_json::json!({ "mesh": 0 }));
    assert_eq!(doc["meshes"][0]["primitives"][0], serde_json::json!({ "attributes": { "POSITION": 0 }, "material": 0, "mode": 3 }));
}
//#endregion 🔖️(a) RealDecode

//#region 🔖️(b) DecodeEncodeDecode
#[test]
fn base_glb_decode_encode_decode_is_semantically_equal() {
    let original = decode_glb(BASE_GLB_BYTES).expect("decode base.glb");
    let reencoded = encode_glb(&original).expect("re-encode");
    assert_eq!(&reencoded[0..4], b"glTF");
    let redecoded = decode_glb(&reencoded).expect("re-decode");

    assert_eq!(redecoded.buffers, original.buffers, "buffer bytes must survive decode->encode->decode exactly");
    assert_eq!(redecoded.document["accessors"], original.document["accessors"]);
    assert_eq!(redecoded.document["bufferViews"], original.document["bufferViews"]);
    assert_eq!(redecoded.document["meshes"], original.document["meshes"]);
    assert_eq!(redecoded.document["materials"], original.document["materials"]);
    assert_eq!(redecoded.document["nodes"], original.document["nodes"]);
    assert_eq!(redecoded.document["scenes"], original.document["scenes"]);
    assert_eq!(redecoded.document["extensionsUsed"], original.document["extensionsUsed"]);

    // every one of the 1095 accessors decodes to identical real numeric data both times --
    // proves the round trip is byte-faithful for actual vertex content, not just JSON shape.
    let count = original.document["accessors"].as_array().unwrap().len();
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
#[test]
fn analyzer_builder_round_trip_reconstructs_equivalent_document() {
    let analysis = GltfAnalyzer::analyze(&[AnalyzeSource::Binary(BASE_GLB_BYTES)]);
    let original = analysis.parts.snapshot.expect("analyzer decodes base.glb");
    let doc = &original.document;

    let mut builder = GltfBuilder::empty();
    builder.set_asset_version(doc["asset"]["version"].as_str().unwrap());

    for (i, buf) in original.buffers.iter().enumerate() {
        assert_eq!(builder.add_buffer(buf.clone()), i);
    }

    for bv in doc["bufferViews"].as_array().unwrap() {
        let buffer = bv["buffer"].as_u64().unwrap() as usize;
        let byte_offset = bv.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let byte_length = bv["byteLength"].as_u64().unwrap() as usize;
        let byte_stride = bv.get("byteStride").and_then(|v| v.as_u64()).map(|v| v as usize);
        let target = bv.get("target").and_then(|v| v.as_u64());
        builder.add_buffer_view(buffer, byte_offset, byte_length, byte_stride, target);
    }

    for acc in doc["accessors"].as_array().unwrap() {
        let component_type = GltfComponentType::from_code(acc["componentType"].as_u64().unwrap()).unwrap();
        let accessor_type = GltfAccessorType::from_str(acc["type"].as_str().unwrap()).unwrap();
        let count = acc["count"].as_u64().unwrap() as usize;
        let mut spec = GltfAccessorSpec::new(component_type, accessor_type, count);
        if let Some(bv) = acc.get("bufferView").and_then(|v| v.as_u64()) {
            let byte_offset = acc.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            spec = spec.with_buffer_view(bv as usize, byte_offset);
        }
        if let (Some(min), Some(max)) = (acc.get("min"), acc.get("max")) {
            let min_v: Vec<f64> = min.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            let max_v: Vec<f64> = max.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            spec = spec.with_min_max(min_v, max_v);
        }
        builder.add_accessor(spec);
    }

    for mat in doc["materials"].as_array().unwrap() {
        builder.add_material(mat.clone());
    }

    for mesh in doc["meshes"].as_array().unwrap() {
        let mesh_idx = builder.add_mesh();
        for prim in mesh["primitives"].as_array().unwrap() {
            let attrs: Vec<(&str, usize)> = prim["attributes"].as_object().unwrap().iter().map(|(k, v)| (k.as_str(), v.as_u64().unwrap() as usize)).collect();
            let indices = prim.get("indices").and_then(|v| v.as_u64()).map(|v| v as usize);
            let material = prim.get("material").and_then(|v| v.as_u64()).map(|v| v as usize);
            let mode = prim.get("mode").and_then(|v| v.as_u64());
            builder.add_mesh_primitive(mesh_idx, &attrs, indices, material, mode);
        }
    }

    for node in doc["nodes"].as_array().unwrap() {
        let mesh = node.get("mesh").and_then(|v| v.as_u64()).map(|v| v as usize);
        builder.add_node(mesh);
    }

    for scene in doc["scenes"].as_array().unwrap() {
        let nodes: Vec<usize> = scene["nodes"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect();
        builder.add_scene(nodes, scene.get("extensions").cloned());
    }
    if let Some(scene) = doc.get("scene").and_then(|v| v.as_u64()) {
        builder.set_default_scene(scene as usize);
    }
    if let Some(ext) = doc.get("extensionsUsed").and_then(|v| v.as_array()) {
        let names: Vec<String> = ext.iter().map(|v| v.as_str().unwrap().to_string()).collect();
        builder.set_extensions_used(names);
    }

    let rebuilt = builder.build().expect("build rebuilt snapshot");

    // Structural spine this builder actually reconstructs (samplers/textures/images/skins/
    // animations/cameras have no typed constructor yet -- out of this wave's scope, and this
    // fixture has none of those that matter: 0 textures, 0 images, 0 skins, 0 animations).
    assert_eq!(rebuilt.document["accessors"], doc["accessors"]);
    assert_eq!(rebuilt.document["bufferViews"], doc["bufferViews"]);
    assert_eq!(rebuilt.document["materials"], doc["materials"]);
    assert_eq!(rebuilt.document["nodes"], doc["nodes"]);
    assert_eq!(rebuilt.document["scenes"], doc["scenes"]);
    assert_eq!(rebuilt.document["scene"], doc["scene"]);
    assert_eq!(rebuilt.document["extensionsUsed"], doc["extensionsUsed"]);
    assert_eq!(rebuilt.buffers, original.buffers);

    // mesh primitive-by-primitive equivalence (attributes/indices/material/mode).
    let orig_meshes = doc["meshes"].as_array().unwrap();
    let rebuilt_meshes = rebuilt.document["meshes"].as_array().unwrap();
    assert_eq!(orig_meshes.len(), rebuilt_meshes.len());
    for (om, rm) in orig_meshes.iter().zip(rebuilt_meshes.iter()) {
        assert_eq!(om["primitives"], rm["primitives"]);
    }

    // The core proof: real decoded vertex/index data for the ANALYZER-produced original and the
    // ONLY-typed-builder-calls REBUILT document must match for every one of the 1095 accessors --
    // not just JSON shape. `original` above already came from `GltfAnalyzer::analyze` (the
    // "analyzer" half of "analyzer->builder round-trip"); this closes the loop by re-decoding the
    // builder's own output through the same real `decode_accessor` codepath.
    let count = doc["accessors"].as_array().unwrap().len();
    assert_eq!(count, 1095);
    for i in 0..count {
        let before = decode_accessor(doc, &original.buffers, i).unwrap();
        let after = decode_accessor(&rebuilt.document, &rebuilt.buffers, i).unwrap();
        assert_eq!(before, after, "accessor {i} diverged between original and analyzer->builder rebuilt document");
    }
}
//#endregion 🔖️(c) AnalyzerBuilderRoundTrip
