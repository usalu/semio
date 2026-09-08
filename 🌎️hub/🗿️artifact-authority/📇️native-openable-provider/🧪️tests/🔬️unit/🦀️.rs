
use super::*;

fn receipts() -> Vec<NativeCodecFactoryReceipt> {
    semio_s_plugin_stdio::registry::native_codec_factory_receipts().expect("verified stdio receipts")
}

#[test]
fn native_openable_provider_consumes_exact_complete_stdio_factory_closure() {
    let provider = NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), receipts()).expect("complete provider");
    let bindings = provider.into_bindings();
    assert_eq!(bindings.len(), NATIVE_STDIO_PROVIDER_RECEIPTS);
    assert!(bindings.iter().all(|binding| !binding.has_genesis()), "headless Stdio codecs must not imply an editor genesis capability");
}

#[test]
fn native_openable_provider_rejects_missing_extra_and_duplicate_receipts_without_publication() {
    let mut missing = receipts();
    missing.pop();
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), missing).is_err());

    let mut extra = receipts();
    extra.push(extra[0].clone());
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), extra).is_err());

    let mut duplicate = receipts();
    duplicate[1].factory_id = duplicate[0].factory_id.clone();
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), duplicate).is_err());
}

#[test]
fn native_openable_provider_rejects_identity_hash_schema_and_factory_substitution() {
    let mut wrong_version = receipts();
    wrong_version[0].package_version = "9.9.9";
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), wrong_version).is_err());

    let mut wrong_plugin = receipts();
    wrong_plugin[0].plugin_id = "foreign";
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), wrong_plugin).is_err());

    let mut zero_hash = receipts();
    zero_hash[0].pack_schema_hash = [0; 32];
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), zero_hash).is_err());

    let mut wrong_schema = receipts();
    wrong_schema[0].schema = "stdio.foreign".into();
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), wrong_schema).is_err());

    let mut substituted_factory = receipts();
    substituted_factory[0].factory = substituted_factory[1].factory;
    assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), substituted_factory).is_err());
}

struct SelectionControl {
    cancelled: bool,
    now_ms: u64,
}

impl super::super::AuthorityOperationControl for SelectionControl {
    fn now_ms(&self) -> u64 {
        self.now_ms
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    fn report(&self, _progress: super::super::AuthorityProgress) {}
}

fn hexadecimal(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn vcs_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🌿️vcs-v1/🔣️.json")).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🔣️.json")).unwrap();
    assert_eq!(fixture["packageVersion"], expected["packageVersion"]);
    assert_eq!(fixture["codecCount"].as_u64().unwrap() as usize, expected["receipts"].as_array().unwrap().len());
    let providers = NativeCodecProviderSetV1::linked();
    let mut accepted = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let control = SelectionControl { cancelled: case["cancelled"].as_bool().unwrap(), now_ms: case["nowMs"].as_u64().unwrap() };
        let context = OperationContext::new(case["deadlineMs"].as_u64().unwrap(), super::super::AuthorityLimits::maximum(), &control);
        let result = providers.preview(case["pluginId"].as_str().unwrap(), case["packageId"].as_str().unwrap(), case["version"].as_str().unwrap(), &context);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
        if let Ok(bindings) = result {
            accepted += 1;
            assert_eq!(bindings.len(), fixture["codecCount"].as_u64().unwrap() as usize);
            for (binding, row) in bindings.iter().zip(expected["receipts"].as_array().unwrap()) {
                assert_eq!(binding.plugin_id(), expected["pluginId"]);
                assert_eq!(binding.package_id(), expected["packageId"]);
                assert_eq!(binding.artifact_kind(), row["kind"]);
                assert_eq!(binding.codec().schema, row["schema"]);
                assert_eq!(binding.codec().extension, row["extension"]);
                let receipt = semio_s_plugin_vcs::native_codecs::native_codec_factory_receipts().unwrap().into_iter().find(|receipt| receipt.identity().schema == binding.codec().schema).unwrap();
                assert_eq!(hexadecimal(&receipt.identity().protocol_sha256), row["protocolSha256"]);
                assert_eq!(binding.codec().pack_schema_hash, receipt.into_codec().unwrap().pack_schema_hash);
                assert_ne!(binding.codec().pack_schema_hash, [0; 32]);
                assert!(binding.has_genesis(), "the exact VCS editor receipt must carry its package-owned genesis factory");
            }
        }
    }
    assert_eq!(accepted, 1);
}

#[test]
fn linked_provider_set_previews_only_the_selected_packages_of_a_stdio_gis_or_stdio_gis_vcs_profile() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🌿️vcs-v1/🔣️.json")).unwrap();
    let providers = NativeCodecProviderSetV1::linked();
    let control = SelectionControl { cancelled: false, now_ms: 0 };
    let context = OperationContext::new(u64::MAX, super::super::AuthorityLimits::maximum(), &control);
    let counts = [("stdio", "semio:stdio", NATIVE_STDIO_PROVIDER_RECEIPTS), ("gis", "semio:gis", 2), ("vcs", "semio:vcs", NATIVE_VCS_PROVIDER_RECEIPTS)];
    for profile in fixture["unconsumedProfiles"].as_array().unwrap() {
        let selected = profile["selected"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
        let previews = profile["previews"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(selected, previews, "{}", profile["name"]);
        let mut requested = Vec::new();
        let mut receipts = 0;
        for (plugin_id, package_id, count) in counts {
            if !selected.contains(&package_id) {
                continue;
            }
            let bindings = providers.preview(plugin_id, package_id, env!("CARGO_PKG_VERSION"), &context).expect("selected compiled provider");
            assert_eq!(bindings.len(), count, "{}", profile["name"]);
            assert!(bindings.iter().all(|binding| binding.package_id() == package_id));
            assert!(bindings.iter().all(|binding| binding.has_genesis() == (plugin_id != "stdio")), "only exact package editor receipts carry genesis authority");
            requested.push(package_id);
            receipts += count;
        }
        assert_eq!(requested, previews, "{}", profile["name"]);
        assert_eq!(receipts, selected.iter().map(|package| counts.iter().find(|(_, id, _)| id == package).expect("compiled package").2).sum::<usize>());
    }
    assert_eq!(counts.iter().map(|(_, _, count)| count).sum::<usize>(), NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS);
    assert!(providers.preview("note", "semio:note", env!("CARGO_PKG_VERSION"), &context).is_err());
}

#[tokio::test]
async fn linked_consumer_descriptors_bind_their_actual_compiled_stdio_dependency_and_catalog() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔏️trusted-catalog/🧪️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
    let kind_json: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🗄️artifact-kind-formats.json")).unwrap();
    let kind: semio_framework::ArtifactKindSpec = semio_framework::from_dsl_value(kind_json.clone().into()).unwrap();
    assert_eq!(serde_json::to_value(&kind).unwrap(), kind_json);
    let independent: semio_framework::ArtifactKindSpec = serde_json::from_value(kind_json.clone()).unwrap();
    assert_eq!(kind, independent);
    let projected = semio_framework::to_dsl_value(&kind).unwrap();
    assert_eq!(directory::os_store::pack_rt::encode_wire_value(&projected), directory::os_store::pack_rt::encode_wire_value(&kind_json.clone().into()));
    for field in ["exportStdioKinds", "importStdioKinds"] {
        for invalid in [serde_json::json!([1]), serde_json::json!("stdio.svg")] {
            let mut candidate = kind_json.clone();
            candidate[field] = invalid;
            assert!(semio_framework::from_dsl_value::<semio_framework::ArtifactKindSpec>(candidate.clone().into()).is_err());
            assert!(serde_json::from_value::<semio_framework::ArtifactKindSpec>(candidate).is_err());
        }
    }
    let control = SelectionControl { cancelled: false, now_ms: 0 };
    let context = OperationContext::new(u64::MAX, super::super::AuthorityLimits::maximum(), &control);
    let providers = NativeCodecProviderSetV1::linked();
    for (plugin_id, package_id, schema, count) in [("gis", "semio:gis", "gis.map", 2), ("vcs", "semio:vcs", "vcs.vcs", 1)] {
        let emitted = if plugin_id == "gis" {
            let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new();
            semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_s_plugin_gis::plugin().unwrap());
            let _foreign = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("foreign").label("Foreign").version("99.0.0").package_id("semio:foreign").try_build().unwrap();
            semio_framework_plugin::describe::describe_plugin(&runtime).await
        } else {
            let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new();
            semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_s_plugin_vcs::plugin().unwrap());
            let _foreign = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("foreign").label("Foreign").version("99.0.0").package_id("semio:foreign").try_build().unwrap();
            semio_framework_plugin::describe::describe_plugin(&runtime).await
        };
        let descriptor: semio_framework::PackageDescriptor = semio_framework::from_dsl_value(directory::os_store::pack_rt::decode_wire_value(&emitted).unwrap()).unwrap();
        assert_eq!(descriptor.package_id, package_id);
        assert_eq!(descriptor.manifest.plugin_id, plugin_id);
        assert_eq!(directory::os_store::pack_rt::encode_wire_value(&semio_framework::to_dsl_value(&descriptor).unwrap()), emitted);
        let selected = NativeCodecProviderPackageV1 { plugin_id, package_id, version: &descriptor.manifest.version };
        let before = directory::os_store::document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash));
        for row in fixture["nativeCases"].as_array().unwrap() {
            let mut candidate = descriptor.clone();
            candidate.manifest.dependencies = serde_json::from_value(row["dependencies"].clone()).unwrap();
            let result = NativeCodecProviderSourceV1::preview(&providers, selected, &candidate, &context);
            assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{plugin_id}: {}: {:?}", row["id"], result.as_ref().err());
            if let Ok(bindings) = result {
                assert_eq!(bindings.len(), count);
            }
            assert_eq!(directory::os_store::document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash)), before);
        }
        for row in fixture["consumerCatalogCases"].as_array().unwrap() {
            let mut candidate = descriptor.clone();
            let topic = candidate.manifest.topic_contributions.iter().position(|entry| entry.topic == "stdio.artifact-catalog.v1").expect("exact compiled catalog topic");
            match row["change"].as_str().unwrap() {
                "exact" => {}
                "missing" => {
                    candidate.manifest.topic_contributions.remove(topic);
                }
                "duplicate" => candidate.manifest.topic_contributions.push(candidate.manifest.topic_contributions[topic].clone()),
                "foreign-topic-version" => candidate.manifest.topic_contributions[topic].topic = "stdio.artifact-catalog.v2".into(),
                "wrong-version" => {
                    let semio_framework::DslValue::Object(fields) = &mut candidate.manifest.topic_contributions[topic].payload else { panic!("catalog object") };
                    fields.iter_mut().find(|(key, _)| key == "packageVersion").unwrap().1 = semio_framework::DslValue::String("99.0.0".into());
                }
                change => panic!("unknown catalog case {change}"),
            }
            let result = NativeCodecProviderSourceV1::preview(&providers, selected, &candidate, &context);
            assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{plugin_id}: {}: {:?}", row["change"], result.as_ref().err());
            assert_eq!(directory::os_store::document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash)), before);
        }
        assert!(NativeCodecProviderSourceV1::preview(&providers, NativeCodecProviderPackageV1 { plugin_id, package_id: "semio:foreign", version: selected.version }, &descriptor, &context).is_err());
    }
}
