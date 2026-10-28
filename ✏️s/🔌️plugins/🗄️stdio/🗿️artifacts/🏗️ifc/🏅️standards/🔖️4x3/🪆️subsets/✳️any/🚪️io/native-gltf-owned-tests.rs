//! 🧪️ Original natural-input frontiers quote physical work before accepting any source byte.
use super::*;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;

fn source(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}
fn glb(text: &str, bin: &[u8]) -> Vec<u8> {
    let json_len = (text.len() + 3) & !3;
    let bin_len = (bin.len() + 3) & !3;
    let mut out = Vec::new();
    out.extend_from_slice(b"glTF");
    for n in [2, 28 + json_len + bin_len, json_len, 0x4e4f534a] {
        out.extend_from_slice(&(n as u32).to_le_bytes());
    }
    out.extend_from_slice(text.as_bytes());
    out.resize(20 + json_len, b' ');
    out.extend_from_slice(&(bin_len as u32).to_le_bytes());
    out.extend_from_slice(&0x004e4942u32.to_le_bytes());
    out.extend_from_slice(bin);
    out.resize(28 + json_len + bin_len, 0);
    out
}
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 16_000_000, maximum_copy_bytes: 1024 * 1024 * 1024, maximum_capacity_bytes: 128 * 1024 * 1024, maximum_release_bytes: 128 * 1024 * 1024, maximum_depth: 512 }
}
fn retire<T: RetireOwned>(value: T) {
    let mut r = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    while !r.terminal_is_empty() {
        r.step(grant()).unwrap();
    }
}
fn decode_run<T>(operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<T, ValueError>) -> Result<T, ValueError> {
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let identity = (native.ledger_identity(), native.retirement_recipient_identity());
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let result = operation(&mut owner);
    drop(owner);
    while native.has_retirement_owner() {
        assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
        native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
    }
    result
}
fn encode_run<T>(operation: impl FnOnce(&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<T, ValueError>) -> Result<T, ValueError> {
    let mut callback = |_| true;
    let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut native = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, grant());
    let result = operation(&mut owner);
    drop(owner);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant()).unwrap();
    }
    result
}
fn bounds(graph: &IfcSnapshot) -> ([f64; 3], [f64; 3], usize, usize) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    let mut vertices = 0;
    let mut triangles = 0;
    for e in &graph.entities {
        if e.name == "IFCCARTESIANPOINTLIST3D" {
            let rows = e.args[0].as_aggregate().unwrap();
            vertices += rows.len();
            for row in rows {
                for (k, v) in row.as_aggregate().unwrap().iter().enumerate() {
                    let IfcValue::Real(n) = v else { panic!() };
                    lo[k] = lo[k].min(*n);
                    hi[k] = hi[k].max(*n);
                }
            }
        } else if e.name == "IFCTRIANGULATEDFACESET" {
            triangles += e.args[3].as_aggregate().unwrap().len();
        }
    }
    (lo, hi, vertices, triangles)
}
fn product_placements(graph: &IfcSnapshot) {
    let context = graph.entities.iter().find(|entity| entity.name == "IFCGEOMETRICREPRESENTATIONCONTEXT").unwrap();
    let axis = context.args[4].as_reference().unwrap();
    let axis_row = graph.entities.iter().find(|entity| entity.id == axis).unwrap();
    assert_eq!(axis_row.name, "IFCAXIS2PLACEMENT3D");
    assert!(matches!(axis_row.args[1], IfcValue::Unset) && matches!(axis_row.args[2], IfcValue::Unset));
    let origin = graph.entities.iter().find(|entity| Some(entity.id) == axis_row.args[0].as_reference()).unwrap();
    assert_eq!(origin.name, "IFCCARTESIANPOINT");
    assert!(matches!(&origin.args[0],IfcValue::Aggregate(values)if values.len()==3&&values.iter().all(|value|matches!(value,IfcValue::Real(number)if *number==0.))));
    for product in graph.entities.iter().filter(|entity| entity.name == "IFCBUILDINGELEMENTPROXY") {
        let id = product.args[5].as_reference().expect("represented product placement");
        let placement = graph.entities.iter().find(|entity| entity.id == id).unwrap();
        assert_eq!(placement.name, "IFCLOCALPLACEMENT");
        assert!(matches!(placement.args[0], IfcValue::Unset));
        assert_eq!(placement.args[1].as_reference(), Some(axis));
        assert!(product.args[6].as_reference().is_some());
    }
    assert_eq!(graph.entities.iter().filter(|entity| entity.name == "IFCLOCALPLACEMENT").count(), 1);
}
#[test]
fn original_native_placement_census_consumes_neutral_checked_counts() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/native-gltf-placement-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        assert_eq!(graph::entity_capacity(row["primitives"].as_u64().unwrap() as usize).unwrap(), row["entities"].as_u64().unwrap() as usize);
    }
    retire(graph::entity_capacity(usize::MAX).unwrap_err());
}

#[test]
fn neutral_owned_contract_declares_all_five_axes_and_official_sources() {
    let contract = include_str!("🧪️tests/native-gltf-owned.feature");
    for word in ["items", "copy bytes", "capacity bytes", "release bytes", "depth", "TriangleWithoutIndices", "BoxInterleaved", "NegativeScaleTest", "same recipient"] {
        assert!(contract.contains(word), "{word}");
    }
}
#[test]
fn original_native_placement_import_denials_and_cancelled_prefixes_keep_the_same_recipient() {
    let source = decode_run(|owner| read_natural(include_bytes!("🧪️tests/gltf-samples/Triangle.gltf"), owner)).unwrap();
    for axis in 0..5 {
        let mut callback = |_| true;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let identity = (native.ledger_identity(), native.retirement_recipient_identity());
        let mut denied = grant();
        match axis {
            0 => denied.maximum_items = 0,
            1 => denied.maximum_copy_bytes = 0,
            2 => denied.maximum_capacity_bytes = 0,
            3 => denied.maximum_release_bytes = 0,
            _ => denied.maximum_depth = 0,
        }
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, denied);
        let error = import::receive(&source, &mut owner).unwrap_err();
        drop(owner);
        while native.has_retirement_owner() {
            assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
            native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
        }
        retire(error);
    }
    let completed = std::cell::Cell::new(0);
    let total;
    {
        let mut callback = |event: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| {
            completed.set(event.completed);
            true
        };
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        let graph = import::receive(&source, &mut owner).unwrap();
        total = completed.get();
        product_placements(&graph);
        drop(owner);
        while native.has_retirement_owner() {
            native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
        }
        retire(graph);
    }
    assert!(total > 4);
    for prefix in [0, total / 4, total / 2, total * 3 / 4, total - 1] {
        let mut callback = |event: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| event.completed < prefix;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let identity = (native.ledger_identity(), native.retirement_recipient_identity());
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        let error = import::receive(&source, &mut owner).unwrap_err();
        assert_eq!(error.kind, ValueRefusalKind::Canceled);
        drop(owner);
        while native.has_retirement_owner() {
            assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
            native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
        }
        retire(error);
    }
    retire(source);
}

#[test]
fn natural_catalog_reads_integral_exponents_last_keys_and_escaped_keys() {
    let input = source("\u{feff}{\"scene\":9,\"sc\\u0065ne\":1e0,\"asset\":{\"version\":\"2.0\"}}");
    let mut events = 0;
    let mut observe = |_| {
        events += 1;
        true
    };
    let mut native = NativeDecodeControl::new(1 << 20, &mut observe);
    let mut recipient = NativeDecodeRetirementRecipient::new();
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let value = owner
        .receive::<NaturalReceiving, usize>(|slot, native, body| {
            *slot = Some(NaturalReceiving::default());
            let mut port = Port { native, body };
            let state = slot.as_mut().unwrap();
            state.framing = framing(&input, &mut port)?;
            state.catalog.receive(&input, state.framing, &mut port)?;
            let scene = state.catalog.member(&input, 0, "scene", &mut port)?.unwrap();
            state.catalog.usize(&input, scene, &mut port)
        })
        .unwrap();
    assert_eq!(value, 1);
    drop(owner);
    drop(native);
    assert!(events > input.len());
}

#[test]
fn natural_catalog_rejects_fractional_indices_and_invalid_padding() {
    for text in ["{\"scene\":1.5}", "{\"scene\":-1}"] {
        let input = source(text);
        let mut observe = |_| true;
        let mut native = NativeDecodeControl::new(1 << 20, &mut observe);
        let mut recipient = NativeDecodeRetirementRecipient::new();
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        let result = owner.receive::<NaturalReceiving, usize>(|slot, native, body| {
            *slot = Some(NaturalReceiving::default());
            let mut port = Port { native, body };
            let state = slot.as_mut().unwrap();
            let shape = framing(&input, &mut port)?;
            state.catalog.receive(&input, shape, &mut port)?;
            let index = state.catalog.member(&input, 0, "scene", &mut port)?.unwrap();
            state.catalog.usize(&input, index, &mut port)
        });
        let error = result.unwrap_err();
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue);
        drop(owner);
        while native.has_retirement_owner() {
            native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
        }
        retire(error);
    }
}

#[test]
fn natural_receiving_consumes_the_shared_numeric_member_version_bundle() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/gltf-json-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let text = format!("{}{}", row["prefix"].as_str().unwrap(), row["json"].as_str().unwrap());
        let input = glb(&text, &[0]);
        let mut observe = |_| true;
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut observe);
        let mut recipient = NativeDecodeRetirementRecipient::new();
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        let result = read_natural(&input, &mut owner);
        drop(owner);
        if row["accepted"].as_bool().unwrap_or_else(|| row["accept"].as_bool().unwrap()) {
            retire(result.unwrap());
        } else {
            retire(result.unwrap_err());
        }
        while native.has_retirement_owner() {
            native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
        }
    }
}

#[test]
fn natural_received_rows_live_in_the_actual_native_gltf_snapshot() {
    let input=br#"{"asset":{"version":"2.0"},"scene":0,"scenes":[{"nodes":[0]}],"nodes":[{"mesh":0,"matrix":[1,0,0,0,0,1,0,0,0,0,1,0,1,2,3,1]}],"meshes":[{"primitives":[{"attributes":{"POSITION":0}}]}],"buffers":[{"byteLength":12,"uri":"data:application/octet-stream;base64,AAAAAAAAAAAAAAAA"}],"bufferViews":[{"buffer":0,"byteLength":12}],"accessors":[{"bufferView":0,"componentType":5126,"count":1,"type":"VEC3"}]}"#;
    let mut observe = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let snapshot = read_natural(input, &mut owner).unwrap();
    assert_eq!(snapshot.schema, "stdio.gltf");
    assert_eq!(snapshot.buffers[0], [0; 12]);
    assert_eq!(snapshot.document.nodes[0].matrix.unwrap()[12..15], [1., 2., 3.]);
    drop(owner);
    assert!(!native.has_retirement_owner());
    retire(snapshot);
}

#[test]
fn original_owned_official_geometry_rows_roundtrip_through_real_models_and_canonical_packs() {
    use pack::snapshot::ArtifactPack;
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/gltf-samples/cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let file = row["file"].as_str().unwrap();
        let bytes: &[u8] = match file {
            "Triangle.gltf" => include_bytes!("🧪️tests/gltf-samples/Triangle.gltf"),
            "TriangleWithoutIndices.gltf" => include_bytes!("🧪️tests/gltf-samples/TriangleWithoutIndices.gltf"),
            "BoxInterleaved.glb" => include_bytes!("🧪️tests/gltf-samples/BoxInterleaved.glb"),
            "NegativeScaleTest.glb" => include_bytes!("🧪️tests/gltf-samples/NegativeScaleTest.glb"),
            _ => panic!(),
        };
        let source = decode_run(|owner| read_natural(bytes, owner)).unwrap();
        let graph = decode_run(|owner| import::receive(&source, owner)).unwrap();
        product_placements(&graph);
        assert_eq!(graph.entities.len(), graph::entity_capacity(row["primitives"].as_u64().unwrap() as usize).unwrap());
        let (lo, hi, vertices, triangles) = bounds(&graph);
        assert_eq!(vertices as u64, row["vertices"].as_u64().unwrap(), "{file}");
        assert_eq!(triangles as u64, row["triangles"].as_u64().unwrap(), "{file}");
        for k in 0..3 {
            for (actual, expected) in [(lo[k], row["ifcBounds"][0][k].as_f64().unwrap()), (hi[k], row["ifcBounds"][1][k].as_f64().unwrap())] {
                assert!((actual - expected).abs() <= 8. * f64::EPSILON * actual.abs().max(expected.abs()).max(1.), "{file} {actual} {expected}");
            }
        }
        let target = decode_run(|owner| export::receive(&graph, owner)).unwrap();
        let native = encode_run(|owner| gltf_io::binary::snapshot::owned_pack::geometry::encode_geometry_pack(&target, owner)).unwrap();
        assert_eq!(native, target.encode_pack(), "{file}");
        let received = decode_run(|owner| gltf_io::binary::snapshot::owned_pack::geometry::receive_geometry_pack(&native, owner)).unwrap();
        let restored = decode_run(|owner| import::receive(&received, owner)).unwrap();
        product_placements(&restored);
        assert_eq!(bounds(&restored).3, triangles);
        let glb = encode_run(|owner| write::encode(&target, owner)).unwrap();
        let independent = super::super::decode_glb_checked(&glb).unwrap();
        assert_eq!(independent.buffers, target.buffers, "{file}");
        if let Ok(folder) = std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
            let stem = file.split('.').next().unwrap();
            let root = std::path::Path::new(&folder);
            std::fs::create_dir_all(root).unwrap();
            std::fs::write(root.join(format!("native-owned-{stem}.glb")), &glb).unwrap();
            std::fs::write(root.join(format!("native-owned-{stem}.ifc")), crate::io::write_ifc4x3_step(&graph).unwrap()).unwrap();
        }
        retire((source, graph, target, native, received, restored, glb, independent));
    }
    if let Ok(folder) = std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
        std::fs::write(std::path::Path::new(&folder).join("native-owned-exchange.json"), br#"{"nativeExecuted":true,"producer":"original-native-gltf-owned-tests","cases":4,"fidelity":"static-geometry-only"}"#).unwrap();
    }
}

#[test]
fn original_geometry_native_source_keeps_material_and_large_extras_custody() {
    use pack::snapshot::ArtifactPack;
    let mut source = gltf_io::import::parse_gltf_document(include_bytes!("🧪️tests/gltf-samples/Triangle.gltf")).unwrap();
    let mut material = GltfMaterial::default();
    material.name = Some("material".repeat(2048));
    source.document.materials.push(material);
    source.document.meshes[0].primitives[0].material = Some(0);
    source.document.extras = Some(GltfJson::String("x".repeat(65536)));
    let wire = source.encode_pack();
    let received = decode_run(|owner| gltf_io::binary::snapshot::owned_pack::geometry::receive_geometry_pack(&wire, owner)).unwrap();
    assert!(received.document.materials.is_empty());
    assert!(received.document.extras.is_none());
    assert!(received.document.meshes[0].primitives[0].material.is_none());
    assert_eq!(received.buffers, source.buffers);
    let graph = decode_run(|owner| import::receive(&received, owner)).unwrap();
    assert_eq!(bounds(&graph).3, 1);
    retire((source, wire, received, graph));
}

#[test]
fn original_geometry_completed_work_cancellation_retains_real_vertex_prefix_and_same_recipient() {
    let mut source = gltf_io::import::parse_gltf_document(include_bytes!("🧪️tests/gltf-samples/TriangleWithoutIndices.gltf")).unwrap();
    source.buffers[0] = [0u8; 12].repeat(6144);
    source.document.buffers[0].byte_length = source.buffers[0].len();
    source.document.buffer_views[0].byte_length = source.buffers[0].len();
    source.document.accessors[0].count = 6144;
    let threshold = 10_000;
    let mut completed = 0;
    let mut callback = |event: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| {
        completed = completed.max(event.completed);
        event.completed < threshold
    };
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let identity = (native.ledger_identity(), native.retirement_recipient_identity());
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let error = import::receive(&source, &mut owner).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    assert!(owner.progress().copied_items > 1000);
    drop(owner);
    assert!(native.has_retirement_owner());
    while native.has_retirement_owner() {
        assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
        native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
    }
    drop(native);
    assert!(completed >= threshold);
    retire((source, error));
}

#[test]
fn material_heavy_native_receiving_cancellation_keeps_the_original_pack_recipient() {
    use pack::snapshot::ArtifactPack;
    let mut source = gltf_io::import::parse_gltf_document(include_bytes!("🧪️tests/gltf-samples/Triangle.gltf")).unwrap();
    source.document.extras = Some(GltfJson::String("metadata".repeat(8192)));
    let wire = source.encode_pack();
    let mut callback = |event: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| event.completed < 128;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let identity = (native.ledger_identity(), native.retirement_recipient_identity());
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let error = gltf_io::binary::snapshot::owned_pack::geometry::receive_geometry_pack(&wire, &mut owner).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    assert!(owner.progress().copied_items > 0);
    drop(owner);
    assert!(native.has_retirement_owner());
    while native.has_retirement_owner() {
        assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
        native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
    }
    retire((source, wire, error));
}

#[test]
fn natural_publication_refuses_fields_outside_its_baked_geometry_contract() {
    let source = crate::io::parse_ifc4x3_step(include_str!("🧪️tests/triangulated.ifc")).unwrap();
    for field in 0..5 {
        let mut target = decode_run(|owner| export::receive(&source, owner)).unwrap();
        match field {
            0 => target.document.asset.generator = Some("authored generator".into()),
            1 => target.document.nodes[0].name = Some("authored node".into()),
            2 => target.document.accessors[0].name = Some("authored accessor".into()),
            3 => target.document.buffers[0].extras = Some(GltfJson::Bool(true)),
            _ => target.document.meshes[0].primitives[0].material = Some(0),
        }
        let error = encode_run(|owner| write::encode(&target, owner)).unwrap_err();
        assert_eq!(error.kind, ValueRefusalKind::UnsupportedOwner);
        retire((target, error));
    }
    retire(source);
}

#[test]
fn original_natural_denials_and_malformed_selected_fields_publish_no_artifact() {
    let input = include_bytes!("🧪️tests/gltf-samples/Triangle.gltf");
    for axis in 0..5 {
        let mut callback = |_| true;
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        let mut recipient = NativeDecodeRetirementRecipient::new();
        native.install_retirement_recipient(&mut recipient).unwrap();
        let identity = (native.ledger_identity(), native.retirement_recipient_identity());
        let mut g = grant();
        match axis {
            0 => g.maximum_items = 0,
            1 => g.maximum_copy_bytes = 0,
            2 => g.maximum_capacity_bytes = 0,
            3 => g.maximum_release_bytes = 0,
            _ => g.maximum_depth = 0,
        };
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, g);
        let error = read_natural(input, &mut owner).unwrap_err();
        drop(owner);
        while native.has_retirement_owner() {
            assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
            native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
        }
        retire(error);
    }
    for input in [br#"{"asset":{"version":"2.0"},"nodes":[{"mesh":-1}]}"#.as_slice(), br#"{"asset":{"version":"2.0"},"accessors":[{"componentType":5126,"count":1,"type":"VEC3","normalized":1}]}"#.as_slice()] {
        let error = decode_run(|owner| read_natural(input, owner)).unwrap_err();
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue);
        retire(error);
    }
}

#[test]
fn original_natural_glb_padding_checks_actual_bytes() {
    let text = r#"{"asset":{"version":"2.0","generator":"xx"},"buffers":[{"byteLength":1}]}"#;
    assert_ne!(text.len() % 4, 0);
    let valid = glb(text, &[0]);
    retire(decode_run(|owner| read_natural(&valid, owner)).unwrap());
    for json_padding in [true, false] {
        let mut input = valid.clone();
        let offset = if json_padding { 20 + text.len() } else { input.len() - 1 };
        input[offset] = if json_padding { b'\t' } else { 1 };
        let error = decode_run(|owner| read_natural(&input, owner)).unwrap_err();
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue);
        retire(error);
    }
}

#[test]
fn native_static_product_classification_consumes_the_shared_declared_vectors() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/geometry-product-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        assert_eq!(crate::validation::metadata::is_subtype(row["entity"].as_str().unwrap(), "IFCPRODUCT"), row["isProduct"].as_bool().unwrap());
    }
}

#[test]
fn real_registered_owned_gltf_pack_and_glb_routes_publish_actual_native_models() {
    use pack::snapshot::ArtifactPack;
    let source = crate::io::parse_ifc4x3_step(include_str!("🧪️tests/triangulated.ifc")).unwrap();
    let input = IoPayload::Binary(source.encode_pack());
    let mut dc = |_| true;
    let mut ec = |_| true;
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(128 * 1024 * 1024, &mut dc);
    let mut encode = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut ec);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let mut control = IoRunControl::new(&mut decode, &mut encode, grant());
    let entries = super::super::io_entries();
    assert_eq!(entries.len(), 10);
    let native = entries.iter().find(|e| e.from == crate::io::IFC4X3_DIALECT && e.into == super::super::GLTF_DIALECT).unwrap();
    let outcome = (native.run)(&input, &mut control).unwrap();
    assert_eq!(outcome.diagnostics.len(), 1);
    assert_eq!(outcome.diagnostics[0].severity, Severity::Warning);
    let IoPayload::Binary(wire) = &outcome.value else { panic!() };
    let target = GltfSnapshot::decode_pack(wire).unwrap();
    assert_eq!(target.document.accessors[0].min, Some(vec![.5,0.,-.5]));
    assert_eq!(target.document.accessors[0].max, Some(vec![1.5,2.,.5]));
    let importer = entries.iter().find(|e| e.from == super::super::GLTF_DIALECT && e.into == crate::io::IFC4X3_DIALECT).unwrap();
    let restored = (importer.run)(&outcome.value, &mut control).unwrap();
    let IoPayload::Binary(wire) = &restored.value else { panic!() };
    let graph = IfcSnapshot::decode_pack(wire).unwrap();
    assert_eq!(bounds(&graph),([.5,-.5,0.],[1.5,.5,2.],8,12));
    let binary = semio_framework_os_kernel::io::io_schema::CARRIER_BINARY;
    let exporter = entries.iter().find(|e| e.from == crate::io::IFC4X3_DIALECT && e.into == binary).unwrap();
    let glb = (exporter.run)(&input, &mut control).unwrap();
    let importer = entries.iter().find(|e| e.from == binary && e.into == crate::io::IFC4X3_DIALECT).unwrap();
    let imported = (importer.run)(&glb.value, &mut control).unwrap();
    let IoPayload::Binary(wire) = &imported.value else { panic!() };
    let graph2 = IfcSnapshot::decode_pack(wire).unwrap();
    assert_eq!(bounds(&graph2), bounds(&graph));
    drop(control);
    assert!(!decode.has_retirement_owner() && !encode.has_retirement_owner());
    retire((source, input, outcome, target, restored, graph, glb, imported, graph2));
}

#[test]
fn original_owned_small_nonsingular_scale_and_ifc_context_coordinates_are_real_geometry() {
    let mut source = gltf_io::import::parse_gltf_document(include_bytes!("🧪️tests/gltf-samples/Triangle.gltf")).unwrap();
    source.document.nodes[0].scale = Some([1e-6; 3]);
    let graph = decode_run(|owner| import::receive(&source, owner)).unwrap();
    assert_eq!(bounds(&graph).1, [1e-6, 0., 1e-6]);
    let mut ifc = crate::io::parse_ifc4x3_step(include_str!("🧪️tests/triangulated.ifc")).unwrap();
    ifc.entities.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity {
        id: 20000,
        name: "IFCCARTESIANPOINT".into(),
        args: vec![IfcValue::Aggregate(vec![IfcValue::Real(1000.), IfcValue::Real(0.), IfcValue::Real(0.)])],
        complex: vec![],
    });
    ifc.entities.iter_mut().find(|e| e.id == 210).unwrap().args[0] = IfcValue::Reference(20000);
    let target = decode_run(|owner| export::receive(&ifc, owner)).unwrap();
    assert_eq!(target.document.accessors[0].min, Some(vec![1.5,0.,-.5]));
    assert_eq!(target.document.accessors[0].max, Some(vec![2.5,2.,.5]));
    retire((source, graph, ifc, target));
}

#[test]
fn original_glb_publication_keeps_final_wire_prefix_and_capacity_refusals_in_its_original_encoder() {
    use std::cell::Cell;
    let graph = crate::io::parse_ifc4x3_step(include_str!("🧪️tests/triangulated.ifc")).unwrap();
    let mut target = decode_run(|owner| export::receive(&graph, owner)).unwrap();
    target.document.meshes[0].name = Some("geometry".repeat(4096));
    let expected = encode_run(|owner| write::encode(&target, owner)).unwrap();
    let cancel = Cell::new(true);
    let turns = Cell::new(0);
    let mut callback = |event: semio_framework_value::native_encoding::NativeEncodeProgress| {
        if cancel.get() && event.owned_bytes >= expected.len() {
            turns.set(turns.get() + 1);
            turns.get() < 16
        } else {
            true
        }
    };
    let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut native = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, grant());
    let error = write::encode(&target, &mut owner).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    assert!(owner.progress().retained_capacity_bytes >= expected.len());
    drop(owner);
    assert!(native.has_retirement_owner());
    cancel.set(false);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant()).unwrap();
    }
    drop(native);
    let mut callback = |_| true;
    let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut native = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut g = grant();
    g.maximum_capacity_bytes = expected.len() - 1;
    let mut owner = semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, g);
    let refused = write::encode(&target, &mut owner).unwrap_err();
    assert_eq!(refused.kind, ValueRefusalKind::OwnershipLimit);
    assert!(owner.progress().retained_capacity_bytes < expected.len());
    drop(owner);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant()).unwrap();
    }
    retire((graph, target, expected, error, refused));
}

#[test]
fn original_registered_route_preserves_native_cancellation_terminal() {
    use pack::snapshot::ArtifactPack;
    let source = crate::io::parse_ifc4x3_step(include_str!("🧪️tests/triangulated.ifc")).unwrap();
    let input = IoPayload::Binary(source.encode_pack());
    let mut dc = |event: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| event.completed < 128;
    let mut ec = |_| true;
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(128 * 1024 * 1024, &mut dc);
    let mut encode = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut ec);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let identity = (decode.ledger_identity(), decode.retirement_recipient_identity());
    let mut control = IoRunControl::new(&mut decode, &mut encode, grant());
    let entry = super::super::io_entries().iter().find(|e| e.from == crate::io::IFC4X3_DIALECT && e.into == super::super::GLTF_DIALECT).unwrap();
    let error = (entry.run)(&input, &mut control).unwrap_err();
    assert_eq!(error.cause.kind, ValueRefusalKind::Canceled);
    drop(control);
    while decode.has_retirement_owner() {
        assert_eq!((decode.ledger_identity(), decode.retirement_recipient_identity()), identity);
        decode.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
    }
    while encode.has_retirement_owner() {
        encode.close_retirement_recipient(grant()).unwrap();
    }
    retire((source, input, error));
}

#[test]
fn original_ifc_axis_cases_admit_only_normative_three_dimensional_frames() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity;
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/axis-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap().iter().filter(|row| row["dimension"] == 3) {
        let mut source = crate::io::parse_ifc4x3_step(include_str!("🧪️tests/triangulated.ifc")).unwrap();
        let mut coordinate = |key: &str, id: u64| {
            if row[key].is_null() {
                return IfcValue::Unset;
            }
            source.entities.push(IfcEntity {
                id,
                name: row[key]["kind"].as_str().unwrap().into(),
                args: vec![IfcValue::Aggregate(row[key]["ratios"].as_array().unwrap().iter().map(|n| IfcValue::Real(n.as_f64().unwrap())).collect())],
                complex: vec![],
            });
            IfcValue::Reference(id)
        };
        let args = vec![coordinate("origin", 20001), coordinate("axis", 20002), coordinate("ref", 20003)];
        source.entities.iter_mut().find(|entity| entity.id == 210).unwrap().args = args;
        let projected = decode_run(|owner| export::receive(&source, owner));
        if row.get("error").is_some() {
            let error = projected.unwrap_err();
            assert_eq!(error.kind, ValueRefusalKind::InvalidValue, "{}", row["id"]);
            retire((source, error));
            continue;
        }
        let target = projected.unwrap();
        let matrix = row["matrix"].as_array().unwrap();
        let (mut minimum, mut maximum) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for x in [500., 1500.] {
            for y in [-500., 500.] {
                for z in [0., 2000.] {
                    let world: [f64; 3] = std::array::from_fn(|k| (matrix[k].as_f64().unwrap() * x + matrix[4 + k].as_f64().unwrap() * y + matrix[8 + k].as_f64().unwrap() * z + matrix[12 + k].as_f64().unwrap()) * 0.001);
                    let gltf = [world[0], world[2], -world[1]];
                    for k in 0..3 {
                        minimum[k] = minimum[k].min(gltf[k]);
                        maximum[k] = maximum[k].max(gltf[k]);
                    }
                }
            }
        }
        for k in 0..3 {
            assert!((target.document.accessors[0].min.as_ref().unwrap()[k] - minimum[k]).abs() < 1e-6, "{}", row["id"]);
            assert!((target.document.accessors[0].max.as_ref().unwrap()[k] - maximum[k]).abs() < 1e-6, "{}", row["id"]);
        }
        retire((source, target));
    }
}
