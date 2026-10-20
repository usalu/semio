/// 🎯️ Open target laws: 7 tests covering descriptor open targets follow the one pairing rule and validate as published; bundle rejects incomplete duplicate conflicting and escaping declarations; trusted profile generation binds zero…
use semio_framework_test_fixtures::fixture_path;

// 🎯️ Laws of the one pairing rule for open targets, bundle declarations, profile generation and the long laws module.

#[test]
fn descriptor_open_targets_follow_the_one_pairing_rule_and_validate_as_published() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️descriptor-open-targets/🔣️.json")).unwrap();
    let schema = "fixture.document@1";
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut descriptor = decode_package_descriptor(&descriptor_bytes("fixture-editor", "semio:fixture-editor", "1.0.0", Some(schema), None)).expect("fixture descriptor");
        descriptor.execution = serde_json::from_value(case["execution"].clone()).expect("execution mode");
        descriptor.manifest.artifact_kinds[0].id = case["kindId"].as_str().unwrap().to_string();
        if case["declaredOn"] == "editor" {
            let kind = descriptor.manifest.artifact_kinds.remove(0);
            descriptor.manifest.apps.iter_mut().find(|app| app.role == semio_framework::AppRole::Editor).expect("editor app").artifact_kinds.push(kind);
        }
        if case["declaredOn"] == "hosted" {
            let kind = descriptor.manifest.artifact_kinds.remove(0);
            descriptor.manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: kind.id, schema: kind.schema, owner: "fixture-owner".into() });
        }
        if case["declaredOn"] == "hosted-presented" {
            let kind = descriptor.manifest.artifact_kinds.remove(0);
            let editor = descriptor.manifest.apps.iter_mut().find(|app| app.role == semio_framework::AppRole::Editor).expect("editor app");
            editor.io.artifact.id = kind.id.clone();
            editor.io.artifact_schema = kind.schema.clone();
            descriptor.manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: kind.id, schema: kind.schema, owner: "fixture-owner".into() });
        }
        if case["declaredOn"] == "plugin-and-editor" {
            let kind = descriptor.manifest.artifact_kinds[0].clone();
            descriptor.manifest.apps.iter_mut().find(|app| app.role == semio_framework::AppRole::Editor).expect("editor app").artifact_kinds.push(kind);
        }
        let bytes = directory::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor));
        let answer: serde_json::Value = serde_json::from_slice(&descriptor_open_targets_answer(&bytes).expect("open-target answer")).expect("answer json");
        assert_eq!(answer["schema"], fixture["answerSchema"], "{name}");
        let targets = answer["targets"].as_array().unwrap();
        let expected = case["targets"].as_array().unwrap();
        assert_eq!(targets.len(), expected.len(), "{name}: {answer}");
        for (target, expected) in targets.iter().zip(expected) {
            assert_eq!(target["role"], expected["role"], "{name}");
            assert_eq!(target["surfaceId"], expected["surfaceId"], "{name}");
            assert_eq!(target["appId"], expected["surfaceId"], "{name}");
            assert_eq!(target["windowKindId"], fixture["windowKindId"], "{name}");
            assert_eq!(target["artifactKind"], case["kindId"], "{name}");
            assert_eq!(target["artifactSchema"], schema, "{name}");
            assert_eq!(target["rendererTarget"], "wasm", "{name}");
            assert_eq!(target["grant"], serde_json::json!({ "read": true, "write": expected["write"], "observe": true }), "{name}");
            let mut published = target.as_object().unwrap().clone();
            published.insert("packSchemaHash".into(), serde_json::json!("44".repeat(32)));
            let published: TrustedBundleOpenTargetV1 = serde_json::from_value(serde_json::Value::Object(published)).expect("bundle open target");
            assert_eq!(validate_descriptor_open_target(&descriptor, &published).expect("an answered target validates").artifact_kind, "s.fixture.document", "{name}");
        }
    }
    assert!(descriptor_open_targets_answer(&[]).is_err());
}

#[test]
fn bundle_rejects_incomplete_duplicate_conflicting_and_escaping_declarations() {
    let fixture = snapshot_json();
    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.packages.pop();
    assert!(validate_bundle(&bundle, "fixture").expect_err("incomplete closure").to_string().contains("incomplete"));

    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.packages[1].package_id = bundle.packages[0].package_id.clone();
    assert!(validate_bundle(&bundle, "fixture").expect_err("duplicate package identity").to_string().contains("duplicated"));

    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.profiles[0].selected_closure[0].version = "9.9.9".to_string();
    assert!(validate_bundle(&bundle, "fixture").expect_err("conflicting closure identity").to_string().contains("conflicts"));

    let mut bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    bundle.packages[0].component.path = "../escape.wasm".to_string();
    let error = TrustedCatalogRelativePathV1::parse(&bundle.packages[0].component.path).expect_err("escaping path");
    assert!(error.to_string().contains("relative path"));
}

#[test]
fn trusted_profile_generation_binds_zero_target_package_and_every_codec_row() {
    let dependency_fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
    for row in dependency_fixture["encodingCases"].as_array().unwrap() {
        let identities: Vec<TrustedBundleIdentityV1> = serde_json::from_value(row["identities"].clone()).unwrap();
        let mut encoded = Vec::new();
        append_trusted_profile_dependencies(&mut encoded, &identities).unwrap();
        assert_eq!(hex_lower(&encoded), row["hex"].as_str().unwrap(), "{}", row["name"]);
    }
    let fixture = snapshot_json();
    let bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    let original = trusted_profile_generation(&bundle, &bundle.profiles[0]).expect("generation");

    let mut component: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("component bundle");
    component.packages[1].component.sha256 = "31".repeat(32);
    assert_ne!(trusted_profile_generation(&component, &component.profiles[0]).expect("component generation"), original, "zero-target component SHA-256 must rotate the profile");

    let mut descriptor: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("descriptor bundle");
    descriptor.packages[1].descriptor.sha256 = "32".repeat(32);
    assert_ne!(trusted_profile_generation(&descriptor, &descriptor.profiles[0]).expect("descriptor generation"), original, "zero-target descriptor SHA-256 must rotate the profile");

    let mut codec: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("codec bundle");
    codec.packages[0].native_codecs[0].pack_schema_hash = "33".repeat(32);
    assert_ne!(trusted_profile_generation(&codec, &codec.profiles[0]).expect("codec generation"), original, "every codec row must rotate the profile");
}

#[test]
fn local_stdio_gis_profile_is_exact_two_packages_thirty_one_codecs_and_opens_every_package_target() {
    let bundle = local_stdio_gis_profile_bundle();
    let selected = validate_bundle(&bundle, "local-stdio-gis-open-v1").expect("closed stdio+GIS profile");
    assert_eq!(selected.package_indices.len(), 2);
    assert_eq!(selected.package_indices, vec![1, 0]);
    assert_eq!(bundle.packages.iter().map(|package| package.native_codecs.len()).sum::<usize>(), 31);
    assert_eq!(bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>(), 2);
    let mut writable_viewer = local_stdio_gis_profile_bundle();
    writable_viewer.packages[0].open_targets[1].grant.write = true;
    writable_viewer.profiles[0].open_targets[1].target.grant.write = true;
    assert!(validate_bundle(&writable_viewer, "local-stdio-gis-open-v1").is_err(), "a viewer target never carries a write grant");
    let mut missing_dependency = local_stdio_gis_profile_bundle();
    missing_dependency.packages[0].dependencies.clear();
    assert!(validate_bundle(&missing_dependency, "local-stdio-gis-open-v1").expect_err("missing compiled Stdio dependency").to_string().contains("exact closed"));
    assert_ne!(trusted_profile_generation(&missing_dependency, &missing_dependency.profiles[0]).unwrap(), bundle.profiles[0].generation_id);

    let mut missing_terrain = local_stdio_gis_profile_bundle();
    missing_terrain.packages[0].native_codecs.pop();
    assert!(validate_bundle(&missing_terrain, "local-stdio-gis-open-v1").expect_err("missing Terrain").to_string().contains("exact closed"));

    let mut terrain_target = local_stdio_gis_profile_bundle();
    let mut target = terrain_target.packages[0].open_targets[0].clone();
    target.artifact_kind = "s.gis.gisterrain".into();
    target.artifact_schema = "gis.terrain".into();
    target.pack_schema_hash = "a2".repeat(32);
    target.surface_id = "s.gis.gisterrain@1/*#editor".into();
    target.app_id = target.surface_id.clone();
    target.parent_dialect.artifact_kind = target.artifact_kind.clone();
    terrain_target.packages[0].open_targets.push(target.clone());
    assert!(validate_bundle(&terrain_target, "local-stdio-gis-open-v1").expect_err("a package target the profile does not open").to_string().contains("exact closed"));
    let package = terrain_target.profiles[0].open_targets[0].package.clone();
    terrain_target.profiles[0].open_targets.push(TrustedBundleProfileOpenTargetV1 { package, target });
    terrain_target.profiles[0].generation_id = trusted_profile_generation(&terrain_target, &terrain_target.profiles[0]).expect("profile generation");
    assert!(validate_bundle(&terrain_target, "local-stdio-gis-open-v1").is_ok(), "the terrain target the one rule admits is opened by the profile");

    let mut reordered = local_stdio_gis_profile_bundle();
    reordered.profiles[0].selected_closure.reverse();
    assert!(validate_bundle(&reordered, "local-stdio-gis-open-v1").expect_err("noncanonical closure").to_string().contains("canonical"));
}

/// 🧭️ Editors of committed descriptors that author no persisted document of their own, each with the reason; every
/// other editor surface must open at least one kind through the one rule (coordinator decision, session 12).
const EDITORS_WITHOUT_A_DOCUMENT: [(&str, &str); 3] = [
    ("s.space.home@1/*#editor", "the launcher: its snapshot holds only the catalog generation it lists (`SHomeSnapshot { schema, catalog_generation }`), never user content"),
    ("s.space.studio@1/*#editor", "the `s` shell's studio: it edits the running shell's OS-owned workflow (`semio_framework_os::WorkflowSnapshot`) and has no artifact kind of its own"),
    ("s.playbook.procedural@1/*#editor", "a `playbook.blockKind` module: its snapshot is the host playbook block's render payload (foreign document codec), never a document of its own"),
];

/// 🔗️ The `(artifact kind, schema)` pairs each hub-LINKED package owns: the committed registries the hub's linked native
/// providers and the publisher's `linkedCodecRegistry` read (`🌎️hub/🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts`).
fn linked_codec_registries() -> std::collections::BTreeMap<&'static str, BTreeSet<(String, String)>> {
    let root = fixture_path!("../../../../../../../✏️s/🔌️plugins");
    let pairs = |path: &str, kind: &str, schema: &str| -> BTreeSet<(String, String)> {
        let registry: serde_json::Value = serde_json::from_slice(&std::fs::read(root.join(path)).expect("committed linked codec registry")).expect("linked codec registry JSON");
        registry["receipts"].as_array().expect("linked codec receipts").iter().map(|row| (row[kind].as_str().expect("receipt kind").to_owned(), row[schema].as_str().expect("receipt schema").to_owned())).collect()
    };
    std::collections::BTreeMap::from([("stdio", pairs("🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json", "artifact_kind", "artifact_schema")), ("gis", pairs("🌍️gis/📇️native-codecs/🔣️.json", "kind", "schema"))])
}

/// 🗺️ LAW (census): every editor surface of every committed, isolated package descriptor opens at least one artifact
/// kind through [`descriptor_open_targets`], and every kind a hub-LINKED package opens binds one of its linked native
/// codecs ([`linked_codec_registries`]), so every document kind with an editor is creatable and openable over the hub;
/// the only exceptions are [`EDITORS_WITHOUT_A_DOCUMENT`], and each of those must still exist.
#[test]
fn every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule() {
    let mut pending = vec![fixture_path!("../../../../../../../✏️s/🔌️plugins")];
    let mut descriptors = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).unwrap_or_else(|error| panic!("{}: {error}", directory.display())) {
            let path = entry.expect("plugin tree entry").path();
            if path.is_dir() && !matches!(path.file_name().and_then(|name| name.to_str()), Some("dist" | "target" | "node_modules")) {
                pending.push(path);
            } else if path.file_name().and_then(|name| name.to_str()) == Some("🛂️.descriptor.semio") {
                descriptors.push(path);
            }
        }
    }
    let linked = linked_codec_registries();
    let (mut editors, mut unopened, mut unlinked, mut shells) = (0usize, Vec::new(), Vec::new(), std::collections::BTreeSet::new());
    for path in &descriptors {
        let descriptor = decode_package_descriptor(&std::fs::read(path).expect("committed descriptor")).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        if descriptor.execution != semio_framework::ExecutionMode::Isolated {
            continue;
        }
        let targets = descriptor_open_targets(&descriptor);
        if let Some(owned) = linked.get(descriptor.manifest.plugin_id.as_str()) {
            unlinked.extend(targets.iter().filter(|target| !owned.contains(&(target.artifact_kind.clone(), target.artifact_schema.clone()))).map(|target| format!("{} {} ({})", target.surface_id, target.artifact_kind, path.display())));
        }
        let opened = targets.into_iter().map(|target| target.surface_id).collect::<std::collections::BTreeSet<_>>();
        for app in descriptor.manifest.apps.iter().filter(|app| app.role == semio_framework::AppRole::Editor && app.id == semio_framework::surface_app_id(&app.dialect, app.role)) {
            editors += 1;
            if opened.contains(&app.id) {
                continue;
            }
            match EDITORS_WITHOUT_A_DOCUMENT.iter().find(|(surface, _)| *surface == app.id) {
                Some((surface, _)) => {
                    shells.insert(*surface);
                }
                None => unopened.push(format!("{} ({})", app.id, path.display())),
            }
        }
    }
    assert!(editors > 0, "no committed editor surface found");
    assert!(unopened.is_empty(), "{} of {editors} editor surfaces open no kind: {unopened:#?}", unopened.len());
    assert!(unlinked.is_empty(), "{} open targets of linked packages bind no linked native codec: {unlinked:#?}", unlinked.len());
    assert_eq!(shells.len(), EDITORS_WITHOUT_A_DOCUMENT.len(), "every allow-listed shell still exists and still opens nothing: {shells:?}");
}

mod long {
    use super::*;
use std::sync::Arc;
use semio_framework::DslValue;
use semio_framework_hash::Sha256;
use semio_hub_trusted_catalog::TrustedCatalogLoader;
use semio_hub_trusted_catalog_descriptor::{VerifiedTrustedPackage, decode_package_descriptor};

    #[tokio::test]
    async fn gis_map_binding_constructs_from_loaded_catalog_and_refuses_tampered_retained_bytes() {
        let _registry = semio_hub_artifact_authority::REAL_LINKED_CODEC_REGISTRY.lock().await;
        for (viewer, foreign_service) in [(false, false), (true, false), (false, true)] {
            let fixture = prepared_gis_binding_fixture(viewer, foreign_service).await;
            let control = TestControl::new();
            let catalog = Arc::new(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "frozen-gis-test", &NativeCodecProviderSetV1::linked(), &control.context()).await.expect("catalog loaded with real GIS receipts"));
            let result = semio_hub_inference_catalog::verified_gis_map_binding(catalog.clone(), &semio_hub_gis::inference::GisMapInferenceBindingV1);
            if foreign_service {
                assert!(matches!(result, Err(semio_hub_inference::InferenceErrorV1::Denied)));
            } else if viewer {
                assert!(result.expect("viewer profile is admissible").is_none());
            } else {
                let binding = result.expect("verified editor binding").expect("GIS Map editor is bound");
                assert!(Arc::ptr_eq(binding.catalog(), &catalog));
                assert_eq!(binding.selection(), catalog.selected_document_open().expect("sole selection"));
                assert_eq!(binding.service().executable_identity(), semio_s_artifact_gis_gismap::gis_map_inference_service().executable_identity());
                let retained = catalog.packages().iter().find(|package| package.plugin_id() == "gis").expect("verified GIS package").component().read(&control.context()).await.expect("verified GIS component rereads").to_vec();
                let digest = binding.digest().to_owned();
                std::fs::write(fixture.component_path(0), b"tampered").expect("mutate fixture backing component");
                assert!(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "frozen-gis-test", &NativeCodecProviderSetV1::linked(), &control.context()).await.is_err());
                drop(catalog);
                let tampered = binding.catalog().packages().iter().find(|package| package.plugin_id() == "gis").expect("retained GIS package").component().read(&control.context()).await;
                assert!(tampered.is_err() && !retained.is_empty(), "a retained component whose backing file changed must be refused, never served");
                assert_eq!(binding.digest(), digest);
            }
        }
    }

    #[tokio::test]
    async fn linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure() {
        let _registry = semio_hub_artifact_authority::REAL_LINKED_CODEC_REGISTRY.lock().await;
        let corpus: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
        let mut fixture = prepared_gis_binding_fixture(false, false).await;
        for receipt in semio_hub_gis::native_codecs::native_codec_factory_receipts().unwrap() {
            let native = receipt.into_codec().unwrap();
            let declared = document_codec(&native.schema).await.unwrap().expect("actual assembled GIS declaration owns its codec");
            assert_eq!(native.schema, declared.schema);
            assert_eq!(native.extension, declared.extension);
            assert_eq!(native.pack_schema_hash, declared.pack_schema_hash);
            assert!(std::ptr::fn_addr_eq(native.compile_dsl, declared.compile_dsl));
            assert!(std::ptr::fn_addr_eq(native.print_mirror, declared.print_mirror));
            assert!(std::ptr::fn_addr_eq(native.edit_text_from_envelope, declared.edit_text_from_envelope));
            assert!(std::ptr::fn_addr_eq(native.apply_ops_binary, declared.apply_ops_binary));
        }
        let baseline = fixture.bundle.clone();
        let originals: Vec<_> = baseline["packages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|record| {
                let path = fixture.root.join(record["descriptor"]["path"].as_str().unwrap());
                (path.clone(), std::fs::read(path).expect("original full descriptor"))
            })
            .collect();
        let schemas: Vec<_> = baseline["packages"].as_array().unwrap().iter().flat_map(|record| record["nativeCodecs"].as_array().unwrap().iter().map(|codec| codec["artifactSchema"].as_str().unwrap().to_owned())).collect();
        assert_eq!(schemas.len(), 31);
        let mut before = Vec::with_capacity(schemas.len());
        for schema in &schemas {
            before.push(document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash)));
        }
        for row in corpus["atomicCases"].as_array().unwrap() {
            fixture.bundle = baseline.clone();
            for (index, (path, bytes)) in originals.iter().enumerate() {
                std::fs::write(path, bytes).expect("restore exact descriptor bytes");
                attach_fixture_plugin_module(&fixture.root, &mut fixture.bundle["packages"][index], bytes);
            }
            let change = row["change"].as_str().unwrap();
            let package = if change == "stdio-catalog" { "stdio" } else { "gis" };
            let index = fixture.bundle["packages"].as_array().unwrap().iter().position(|record| record["pluginId"] == package).unwrap();
            if !matches!(change, "exact" | "gis-provider") {
                let mut descriptor = decode_package_descriptor(&originals[index].1).expect("original authoritative descriptor");
                match change {
                    "stdio-catalog" | "gis-catalog" => descriptor.manifest.topic_contributions.retain(|entry| entry.topic != "stdio.artifact-catalog.v1"),
                    "gis-dependency" => descriptor.manifest.dependencies.clear(),
                    "gis-trailing-byte" | "gis-duplicate-field" => {}
                    _ => panic!("unknown atomic fixture change {change}"),
                }
                let mut value = semio_framework_value::ToValue::to_value(&descriptor);
                if change == "gis-duplicate-field" {
                    let DslValue::Object(fields) = &mut value else { panic!("full descriptor object") };
                    let duplicate = fields.iter().find(|(key, _)| key == "manifest").unwrap().clone();
                    fields.push(duplicate);
                }
                let mut bytes = directory::os_pack::pack_rt::encode_wire_value(&value);
                if change == "gis-trailing-byte" {
                    bytes.push(0);
                }
                let raw_invalid = matches!(change, "gis-trailing-byte" | "gis-duplicate-field");
                assert_eq!(decode_package_descriptor(&bytes).is_err(), raw_invalid, "full descriptor raw gate: {change}");
                let record = &mut fixture.bundle["packages"][index];
                record["descriptor"]["byteLength"] = bytes.len().into();
                record["descriptor"]["sha256"] = hex_lower(&Sha256::digest(&bytes)).into();
                if record["browserActor"]["kind"] == "closed-browser-actor" {
                    record["browserActor"]["sourceDescriptorByteSha256"] = record["descriptor"]["sha256"].clone();
                }
                attach_fixture_plugin_module(&fixture.root, record, &bytes);
                std::fs::write(&originals[index].0, bytes).expect("write resealed complete candidate descriptor");
            }
            fixture.refresh_profile_generation();
            fixture.persist_bundle();
            let providers = RecordingLinkedProvider { previews: Mutex::new(Vec::new()), fail_gis: change == "gis-provider" };
            let control = TestControl::new();
            let result = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "frozen-gis-test", &providers, &control.context()).await;
            assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "loader result: {change}: {:?}", result.as_ref().err());
            let previews: Vec<String> = serde_json::from_value(row["previews"].clone()).unwrap();
            assert_eq!(*providers.previews.lock().unwrap(), previews, "successful private preview frontier: {change}");
            if let Ok(catalog) = result {
                assert_eq!(catalog.codec_count(), 31);
                assert_eq!(catalog.packages().iter().map(VerifiedTrustedPackage::plugin_id).collect::<Vec<_>>(), vec!["stdio", "gis"]);
                assert_eq!(catalog.open_target_count(), 1);
                assert_eq!(catalog.selected_document_open().unwrap().package.plugin_id, "gis");
                for schema in &schemas {
                    assert!(document_codec(schema).await.unwrap().is_some(), "complete public codec: {schema}");
                }
            } else {
                for (schema, prior) in schemas.iter().zip(&before) {
                    assert_eq!(&document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash)), prior, "partial public codec after {change}: {schema}");
                }
            }
        }
    }
}
