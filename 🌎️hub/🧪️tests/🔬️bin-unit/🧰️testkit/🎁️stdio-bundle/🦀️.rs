//! 🎁️ The native openable stdio bundle fixture: a published trusted-catalog generation under a fixture-owned data root.

use super::*;

pub fn native_openable_stdio_bundle() -> std::path::PathBuf {
    let root = tempdir("native-openable-stdio");
    let stage = root.join("generation-stage");
    std::fs::create_dir_all(stage.join("components")).expect("stdio component directory");
    std::fs::create_dir_all(stage.join("descriptors")).expect("stdio descriptor directory");
    let component = b"\0asm\x0d\x00\x01\x00";
    let component_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(component));
    let component_blake3 = blake3::hash(component).to_hex().to_string();
    let receipts = semio_hub_stdio_catalog::native_codec_factory_receipts().expect("artifact-owned stdio receipts");
    let viewer = semio_framework_plugin::Viewer::builder(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_plugin::StandardId("rfc8259"), subset: semio_framework_plugin::SubsetId::ANY })
        .document(["semio", "stdio", "json"])
        .mode("view", semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), "eye")
        .default_mode_id("view")
        .window_kind_def(<semio_framework_plugin::app::TreeWindowKit as semio_framework_plugin::app::WindowKit>::window_kind())
        .build_definition();
    let mut viewer = viewer;
    viewer.artifact_kinds = semio_hub_stdio_catalog::native_codec_artifact_kinds().into_iter().filter(|kind| kind.id == "s.stdio.json").collect();
    let mut manifest = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::new("stdio", "Stdio Fixture", receipts[0].package_version).manifest;
    manifest.artifact_kinds = semio_hub_stdio_catalog::native_codec_artifact_kinds();
    manifest.apps.push(viewer.clone());
    manifest.topic_contributions.push(semio_hub_stdio_catalog::native_artifact_catalog_contribution().expect("synthetic fixture retains full catalog semantics"));
    let receipt_pairs = receipts.iter().map(|receipt| (receipt.artifact_kind.clone(), receipt.schema.clone())).collect::<std::collections::BTreeSet<_>>();
    let authored: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json")).expect("schema-qualified factory roster");
    let expected_pairs = authored["receipts"].as_array().unwrap().iter().map(|receipt| (receipt["artifact_kind"].as_str().unwrap().to_owned(), receipt["artifact_schema"].as_str().unwrap().to_owned())).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(receipt_pairs.len(), receipts.len(), "every executable owner receipt has one unique kind/schema pair");
    assert_eq!(receipt_pairs, expected_pairs, "every descriptor kind/schema pair has its exact executable owner receipt");
    assert_eq!(manifest.artifact_kinds.iter().map(|kind| kind.id.as_str()).collect::<std::collections::BTreeSet<_>>(), receipts.iter().map(|receipt| receipt.artifact_kind.as_str()).collect(), "every descriptor artifact kind has executable owner coverage");
    assert_eq!(manifest.artifact_kinds.len(), receipts.iter().map(|receipt| receipt.artifact_kind.as_str()).collect::<std::collections::BTreeSet<_>>().len(), "descriptor artifact kinds are unique");
    let pair_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧩️compositions/🗄️stdio/📇️catalog/🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️receipt-pairs.json")).expect("neutral receipt pair corpus");
    let expected = pair_fixture["pairs"].as_array().unwrap().iter().map(|pair| (pair[0].as_str().unwrap().to_owned(), pair[1].as_str().unwrap().to_owned())).collect::<std::collections::BTreeSet<_>>();
    let kinds = pair_fixture["kinds"].as_array().unwrap().iter().map(|kind| kind.as_str().unwrap().to_owned()).collect::<std::collections::BTreeSet<_>>();
    for row in pair_fixture["cases"].as_array().unwrap() {
        let mut pairs = pair_fixture["pairs"].as_array().unwrap().iter().map(|pair| (pair[0].as_str().unwrap().to_owned(), pair[1].as_str().unwrap().to_owned())).collect::<Vec<_>>();
        match row["mutation"].as_str().unwrap() {
            "none" => {}
            "reverse" => pairs.reverse(),
            "missing-schema" => { pairs.remove(1); }
            "missing-kind" => { pairs.pop(); }
            "duplicate" => pairs[1] = pairs[0].clone(),
            "foreign-schema" => pairs[1].1 = "foreign.schema".into(),
            "foreign-kind" => pairs[1].0 = "foreign.artifact".into(),
            "extra" => pairs.push(("s.stdio.pdf".into(), "foreign.schema".into())),
            mutation => panic!("unknown receipt pair mutation {mutation}"),
        }
        let actual = pairs.iter().cloned().collect::<std::collections::BTreeSet<_>>();
        let actual_kinds = pairs.iter().map(|pair| pair.0.clone()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(actual.len() == pairs.len() && actual == expected && actual_kinds == kinds, row["accepted"].as_bool().unwrap(), "neutral receipt pair {}", row["id"]);
    }
    assert_eq!(viewer.id, "s.stdio.json@rfc8259/*#viewer", "the synthetic viewer opens the manifest-declared kind its own dialect names");
    let viewer_id = viewer.id.clone();
    let window_id = viewer.window_kinds.iter().find(|window| window.id == "framework.window.tree").expect("descriptor-owned JSON viewer window").id.clone();
    let descriptor = semio_framework::PackageDescriptor {
        descriptor_version: 1,
        package_id: "semio:stdio".into(),
        role: semio_framework::PackageRole::Plugin,
        manifest,
        activation_events: Vec::new(),
        capability_requests: Vec::new(),
        extension_points: Vec::new(),
        execution: semio_framework::ExecutionMode::Isolated,
        execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        quotas: semio_framework::kernel::QuotaSchema::default(),
        contributions: semio_framework::ContributionSet::default(),
        assets: Vec::new(),
    };
    let descriptor_bytes = directory::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor));
    let descriptor_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&descriptor_bytes));
    let receipt = directory::os_component_receipt::ComponentReceiptV1 {
        schema: directory::os_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
        plugin_id: "stdio".into(),
        package_id: "semio:stdio".into(),
        profile: directory::os_component_receipt::ComponentProfileV1::Release,
        target: directory::os_component_receipt::COMPONENT_TARGET.into(),
        wasm_sha256: component_sha256.clone(),
        wasm_bytes: component.len() as u64,
        semantic_descriptor_sha256: descriptor_sha256.clone(),
        toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
        source_tree_digest: "3".repeat(64),
        provenance_sha256: "4".repeat(64),
    };
    let receipt_bytes = receipt.encode_canonical();
    let receipt_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&receipt_bytes));
    let json = receipts.iter().find(|receipt| receipt.factory_id == "stdio.native.json.v1").expect("JSON receipt");
    let native_codecs = receipts
        .iter()
        .map(|receipt| {
            serde_json::json!({
                "artifactKind": receipt.artifact_kind,
                "artifactSchema": receipt.schema,
                "packSchemaHash": os_directory::io::binary::artifact_hash::hex_lower(&receipt.pack_schema_hash)
            })
        })
        .collect::<Vec<_>>();
    let version = receipts[0].package_version;
    let target = serde_json::json!({
        "artifactKind": json.artifact_kind,
        "artifactSchema": json.schema,
        "packSchemaHash": os_directory::io::binary::artifact_hash::hex_lower(&json.pack_schema_hash),
        "surfaceId": viewer_id,
        "appId": viewer.id,
        "windowKindId": window_id,
        "role": "viewer",
        "rendererTarget": "wasm",
        "parentDialect": {
            "artifactKind": viewer.dialect.artifact_kind,
            "standard": viewer.dialect.standard,
            "subset": viewer.dialect.subset
        },
        "grant": { "read": true, "write": false, "observe": true }
    });
    let mut bundle = serde_json::json!({
        "schemaVersion": 4,
        "profiles": [{
            "id": "stdio-native-openable-v1",
            "selectedClosure": [{ "pluginId": "stdio", "packageId": "semio:stdio", "version": version }],
            "selectedClosureSha256": "11".repeat(32),
            "openTargets": [{
                "package": { "pluginId": "stdio", "packageId": "semio:stdio", "version": version },
                "target": target
            }],
            "generationId": "22".repeat(32)
        }],
        "packages": [{
            "pluginId": "stdio",
            "packageId": "semio:stdio",
            "version": version,
            "role": "plugin",
            "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
            "dependencies": [],
            "component": {
                "path": "components/stdio.wasm",
                "byteLength": component.len(),
                "sha256": component_sha256,
                "blake3": component_blake3
            },
            "descriptor": {
                "path": "descriptors/stdio.descriptor.semio",
                "byteLength": descriptor_bytes.len(),
                "sha256": descriptor_sha256
            },
            "receipt": receipt,
            "nativeCodecs": native_codecs,
            "openTargets": [target]
        }]
    });
    bundle["packages"][0]["browserActor"] = serde_json::json!({
        "kind":"closed-browser-actor", "schema":"semio.os.closed-browser-actor.v1", "codegenPolicy":"semio.os.browser-jco-1.34.0-jspi.v1",
        "path":"closed-actor.mjs", "byteLength":component.len(), "sha256":component_sha256,
        "sourceComponentSha256":component_sha256, "sourceDescriptorByteSha256":descriptor_sha256, "policySha256":"41".repeat(32), "importInterfaces":[]
    });
    std::fs::write(stage.join("closed-actor.mjs"), component).expect("synthetic actor, never executed");
    let plugin_module_files = vec![("stdio/🌉️bridge.js".to_owned(), b"export {};\n".to_vec()), ("stdio/🔣️.json".to_owned(), b"{}\n".to_vec()), ("stdio/🛂️.descriptor.semio".to_owned(), descriptor_bytes.clone()), ("stdio/🧾️receipt.json".to_owned(), receipt_bytes)];
    let plugin_module_source = semio_hub_trusted_catalog_contract::plugin_module::TrustedPluginModuleSourceV1 { plugin_id: "stdio", package_id: "semio:stdio", version: &*version, component_sha256: &component_sha256, descriptor_byte_sha256: &descriptor_sha256, receipt_sha256: &receipt_sha256 };
    let plugin_module = semio_hub_trusted_catalog_contract::plugin_module::write_plugin_module_bundle(&stage, "plugin-module-stdio.json", plugin_module_source, "stdio", &plugin_module_files).expect("stdio plugin module");
    bundle["packages"][0]["pluginModule"] = serde_json::to_value(plugin_module).expect("stdio plugin module record");
    let carried = serde_json::to_vec(&bundle).expect("provisional stdio bundle");
    let (selected_closure_sha256, generation_id) = semio_hub_trusted_catalog_descriptor::trusted_profile_digests_json(&carried, "stdio-native-openable-v1").expect("stdio profile digests");
    bundle["profiles"][0]["selectedClosureSha256"] = selected_closure_sha256.into();
    bundle["profiles"][0]["generationId"] = generation_id.into();
    std::fs::write(stage.join("components/stdio.wasm"), component).expect("write stdio component");
    std::fs::write(stage.join("descriptors/stdio.descriptor.semio"), descriptor_bytes).expect("write stdio descriptor");
    let bundle_bytes = serde_json::to_vec_pretty(&bundle).expect("stdio bundle json");
    std::fs::write(stage.join("trusted-catalog.json"), &bundle_bytes).expect("write stdio bundle");
    let trusted_root = root.join("trusted-catalog");
    let generations = trusted_root.join("generations");
    std::fs::create_dir_all(&generations).expect("trusted generation owner");
    let generation = bundle["profiles"][0]["generationId"].as_str().expect("generation id");
    std::fs::rename(stage, generations.join(generation)).expect("publish trusted generation");
    let bundle_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&bundle_bytes));
    let current_bytes = format!(
        r#"{{"profileId":"stdio-native-openable-v1","generationId":"{generation}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#
    )
    .into_bytes();
    std::fs::write(trusted_root.join("current.json"), current_bytes).expect("publish current pointer");
    std::fs::canonicalize(root).expect("canonical fixture-owned data root")
}
