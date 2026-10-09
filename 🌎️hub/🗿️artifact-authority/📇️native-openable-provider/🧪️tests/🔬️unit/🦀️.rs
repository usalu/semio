
use super::*;

fn receipts() -> Vec<NativeCodecFactoryReceipt> {
    semio_hub_stdio::catalog::native_codec_factory_receipts().expect("verified stdio receipts")
}

#[test]
fn native_openable_provider_consumes_exact_complete_stdio_factory_closure() {
    let expected=receipts();
    let provider = NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), receipts()).expect("complete provider");
    let bindings = provider.into_bindings();
    assert_eq!(bindings.len(), semio_hub_stdio::catalog::live_native_codec_factory_receipts().unwrap().len());
    assert!(bindings.iter().all(|binding| binding.codec().pack_schema_hash != [0; 32]), "every headless Stdio codec carries its structural pack schema identity");
    for(binding,receipt)in bindings.iter().zip(&expected){assert_eq!(binding.factory_id(),Some(receipt.factory_id.as_str()));assert_eq!(serde_json::to_value(binding.factory_id()).unwrap(),serde_json::to_value(Some(&receipt.factory_id)).unwrap());assert_eq!(binding.plugin_id(),receipt.plugin_id);assert_eq!(binding.package_id(),receipt.package_id);assert_eq!(binding.artifact_kind(),receipt.artifact_kind);assert_eq!(binding.codec().schema,receipt.schema);}
    println!("[DEBUG] Original Stdio catalog binding factoryIds={} exactVerifiedReceipts=true independentSerde=true",bindings.len());
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌿️vcs-v1/🔣️.json")).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌎️hub/🧩️compositions/🌿️vcs/📇️native-codecs/🔣️.json")).unwrap();
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
            assert_eq!(bindings.len(), case["bindings"].as_u64().unwrap() as usize, "{}", case["name"]);
            assert_eq!(bindings.is_empty(), case["code"] == "unlinked-package", "{}", case["name"]);
            if bindings.is_empty() {
                continue;
            }
            accepted += 1;
            assert_eq!(bindings.len(), fixture["codecCount"].as_u64().unwrap() as usize);
            for (binding, row) in bindings.iter().zip(expected["receipts"].as_array().unwrap()) {
                assert_eq!(binding.plugin_id(), expected["pluginId"]);
                assert_eq!(binding.package_id(), expected["packageId"]);
                assert_eq!(binding.artifact_kind(), row["kind"]);
                assert_eq!(binding.codec().schema, row["schema"]);
                assert_eq!(binding.codec().extension, row["extension"]);
                let receipt = semio_hub_vcs::native_codecs::native_codec_factory_receipts().unwrap().into_iter().find(|receipt| receipt.identity().schema == binding.codec().schema).unwrap();
                assert_eq!(binding.factory_id(),Some(receipt.identity().factory_id));
                assert_eq!(hexadecimal(&receipt.identity().protocol_sha256), row["protocolSha256"]);
                assert_eq!(binding.codec().pack_schema_hash, receipt.into_codec().unwrap().pack_schema_hash);
                assert_ne!(binding.codec().pack_schema_hash, [0; 32]);
                assert_eq!(binding.artifact_kind(), row["kind"], "the exact VCS receipt binds its own artifact kind");
            }
        }
    }
    assert_eq!(accepted, 1);
}

mod quick {
    use super::*;

    #[tokio::test]
    async fn linked_consumer_descriptors_bind_their_actual_compiled_stdio_dependency_and_catalog() {
        let _registry = crate::artifact_authority::REAL_LINKED_CODEC_REGISTRY.lock().await;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
        let kind_json: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json")).unwrap();
        let kind: semio_framework::ArtifactKindSpec = semio_framework_value::FromValue::from_value(kind_json.clone().into()).unwrap();
        assert_eq!(serde_json::to_value(&kind).unwrap(), kind_json);
        let independent: semio_framework::ArtifactKindSpec = serde_json::from_value(kind_json.clone()).unwrap();
        assert_eq!(kind, independent);
        let projected = semio_framework_value::ToValue::to_value(&kind);
        assert_eq!(directory::os_store::pack_rt::encode_wire_value(&projected), directory::os_store::pack_rt::encode_wire_value(&kind_json.clone().into()));
        for field in ["exportStdioKinds", "importStdioKinds"] {
            for invalid in [serde_json::json!([1]), serde_json::json!("stdio.svg")] {
                let mut candidate = kind_json.clone();
                candidate[field] = invalid;
                assert!(<semio_framework::ArtifactKindSpec as semio_framework_value::FromValue>::from_value(candidate.clone().into()).is_err());
                assert!(serde_json::from_value::<semio_framework::ArtifactKindSpec>(candidate).is_err());
            }
        }
        let control = SelectionControl { cancelled: false, now_ms: 0 };
        let context = OperationContext::new(u64::MAX, crate::artifact_authority::AuthorityLimits::maximum(), &control);
        let providers = NativeCodecProviderSetV1::linked();
        for (plugin_id, package_id, schema, count) in [("gis", "semio:gis", "gis.map", 2), ("vcs", "semio:vcs", "vcs.vcs", 1)] {
            let emitted = if plugin_id == "gis" {
                let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
                semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_hub_gis::plugin().unwrap());
                let _foreign = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("foreign").label("Foreign").version("99.0.0").package_id("semio:foreign").try_build().unwrap();
                semio_framework_plugin::describe::describe_plugin(&runtime).await
            } else {
                let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
                semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_hub_vcs::plugin().unwrap());
                let _foreign = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("foreign").label("Foreign").version("99.0.0").package_id("semio:foreign").try_build().unwrap();
                semio_framework_plugin::describe::describe_plugin(&runtime).await
            };
            let descriptor: semio_framework::PackageDescriptor = semio_framework_value::FromValue::from_value(directory::os_store::pack_rt::decode_wire_value(&emitted).unwrap()).unwrap();
            assert_eq!(descriptor.package_id, package_id);
            assert_eq!(descriptor.manifest.plugin_id, plugin_id);
            assert_eq!(directory::os_store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor)), emitted);
            let selected = NativeCodecProviderPackageV1 { plugin_id, package_id, version: &descriptor.manifest.version };
            let before = directory::os_store::document_codec(schema).await.unwrap().map(|codec| (codec.schema, codec.extension, codec.pack_schema_hash));
            for row in fixture["nativeCases"].as_array().unwrap() {
                let accepted = match serde_json::from_value::<Vec<semio_framework::PluginDependency>>(row["dependencies"].clone()) {
                    Err(refusal) => {
                        assert!(refusal.to_string().contains("exact version"), "{plugin_id}: {}: a range pin is refused when the manifest decodes: {refusal}", row["id"]);
                        false
                    }
                    Ok(dependencies) => {
                        let mut candidate = descriptor.clone();
                        candidate.manifest.dependencies = dependencies;
                        let result = NativeCodecProviderSourceV1::preview(&providers, selected, &candidate, &context);
                        if let Ok(bindings) = &result {
                            assert_eq!(bindings.len(), count);
                        }
                        result.is_ok()
                    }
                };
                assert_eq!(accepted, row["accepted"].as_bool().unwrap(), "{plugin_id}: {}", row["id"]);
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

        let mut swapped_factory_ids = receipts();
        let first = swapped_factory_ids[0].factory_id.clone();
        swapped_factory_ids[0].factory_id = swapped_factory_ids[1].factory_id.clone();
        swapped_factory_ids[1].factory_id = first;
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), swapped_factory_ids).is_err());
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

        let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌎️hub/🧩️compositions/🗄️stdio/📇️catalog/🧫️fixtures/📇️native-catalog-surface/🧪️receipt-pairs.json")).unwrap();
        for row in corpus["cases"].as_array().unwrap() {
            let mut candidate = receipts();
            let pdf14 = candidate.iter().position(|receipt| receipt.artifact_kind == "s.stdio.pdf" && receipt.schema == "stdio.pdf").expect("PDF1.4 owner");
            let pdf17 = candidate.iter().position(|receipt| receipt.artifact_kind == "s.stdio.pdf" && receipt.schema == "stdio.pdf.1.7").expect("PDF1.7 owner");
            let json = candidate.iter().position(|receipt| receipt.artifact_kind == "s.stdio.json" && receipt.schema == "stdio.json").expect("JSON owner");
            match row["mutation"].as_str().unwrap() {
                "none" => {}
                "reverse" => candidate.reverse(),
                "missing-schema" => { candidate.remove(pdf17); }
                "missing-kind" => { candidate.remove(json); }
                "duplicate" => candidate[pdf17] = candidate[pdf14].clone(),
                "foreign-schema" => candidate[pdf17].schema = "foreign.schema".into(),
                "foreign-kind" => candidate[pdf17].artifact_kind = "foreign.artifact".into(),
                "extra" => {
                    let mut extra = candidate[pdf14].clone();
                    extra.schema = "foreign.schema".into();
                    candidate.push(extra);
                }
                mutation => panic!("unknown receipt pair mutation {mutation}"),
            }
            assert_eq!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), candidate).is_ok(), row["accepted"].as_bool().unwrap(), "actual pair roster {}", row["id"]);
        }
    }

    /// 🌱️ TC3b: a native binding carries NO creation authority any more — genesis is the
    /// component's, for every package alike, so there is nothing here to distinguish an
    /// editor receipt from an import/export one.
    #[test]
    fn linked_provider_set_previews_only_the_selected_packages_of_a_stdio_gis_or_stdio_gis_vcs_profile() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌿️vcs-v1/🔣️.json")).unwrap();
        let providers = NativeCodecProviderSetV1::linked();
        let control = SelectionControl { cancelled: false, now_ms: 0 };
        let context = OperationContext::new(u64::MAX, crate::artifact_authority::AuthorityLimits::maximum(), &control);
        let counts = [("stdio", "semio:stdio", semio_hub_stdio::catalog::live_native_codec_factory_receipts().unwrap().len()), ("gis", "semio:gis", 2), ("vcs", "semio:vcs", NATIVE_VCS_PROVIDER_RECEIPTS)];
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
                assert!(bindings.iter().all(|binding| binding.plugin_id() == plugin_id));
                for binding in &bindings{let expected=match plugin_id{"stdio"=>semio_hub_stdio::catalog::native_codec_factory_receipts().unwrap().into_iter().find(|receipt|receipt.schema==binding.codec().schema).unwrap().factory_id,"gis"=>semio_hub_gis::native_codecs::native_codec_factory_receipts().unwrap().into_iter().find(|receipt|receipt.identity().schema==binding.codec().schema).unwrap().identity().factory_id.to_owned(),"vcs"=>semio_hub_vcs::native_codecs::native_codec_factory_receipts().unwrap().into_iter().find(|receipt|receipt.identity().schema==binding.codec().schema).unwrap().identity().factory_id.to_owned(),_=>unreachable!()};assert_eq!(binding.factory_id(),Some(expected.as_str()));}
                requested.push(package_id);
                receipts += count;
            }
            assert_eq!(requested, previews, "{}", profile["name"]);
            assert_eq!(receipts, selected.iter().map(|package| counts.iter().find(|(_, id, _)| id == package).expect("compiled package").2).sum::<usize>());
        }
        assert_eq!(counts.iter().map(|(_, _, count)| count).sum::<usize>(), native_openable_provider_receipt_count().unwrap());
        assert!(providers.preview("note", "semio:note", env!("CARGO_PKG_VERSION"), &context).expect("an unlinked package is previewable").is_empty());
        assert!(providers.preview("note", "semio:gis", env!("CARGO_PKG_VERSION"), &context).is_err());
        assert!(providers.preview("gis", "semio:note", env!("CARGO_PKG_VERSION"), &context).is_err());
    }
}
