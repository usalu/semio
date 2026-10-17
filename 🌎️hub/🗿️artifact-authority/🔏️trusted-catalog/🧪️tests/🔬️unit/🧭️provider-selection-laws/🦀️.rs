/// 🧭️ Laws of native provider selection, descriptor verification, failure substitution and the neutral dependency-order fixture.

#[tokio::test]
async fn selected_native_providers_are_descriptor_verified_dependency_first_and_only_selected() {
    let mut single = prepared_fixture();
    single.bundle["packages"][0]["dependencies"] = serde_json::json!([]);
    single.bundle["profiles"][0]["selectedClosure"] = serde_json::json!([
        { "pluginId": "fixture-editor", "packageId": "semio:fixture-editor", "version": "1.2.3" }
    ]);
    single.rewrite_descriptor(0, Some(&single.schema.clone()), None);
    let unselected = NativeCodecBinding::new("unselected", "semio:unselected", "fixture.unselected", None, fixture_codec(&format!("{}.unselected", single.schema), [0x11; 32]));
    let mut source = FixtureProviderSource::new(vec![single.binding(), unselected]);
    source.exact_pool = false;
    let catalog = TrustedCatalogLoader::load_fixture(&single.bundle_path, "fixture", &source, &TestControl::new().context()).await.expect("selected-only catalog");
    assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-editor"]);
    assert_eq!(catalog.packages().len(), 1);
    assert_eq!(catalog.codec_count(), 1);
    assert!(document_codec(&format!("{}.unselected", single.schema)).await.expect("registry").is_none());

    let mut pair = prepared_fixture();
    let source = FixtureProviderSource::new(pair.make_two_codec_bindings());
    let catalog = TrustedCatalogLoader::load_fixture(&pair.bundle_path, "fixture", &source, &TestControl::new().context()).await.expect("complete selected closure");
    assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-base", "semio:fixture-editor"]);
    assert_eq!(catalog.packages().len(), 2);
    assert_eq!(catalog.codec_count(), 2);
    assert!(document_codec(&pair.schema).await.expect("registry").is_some());
    assert!(document_codec(&format!("{}.base", pair.schema)).await.expect("registry").is_some());
}

#[tokio::test]
async fn gis_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication() {
    struct SelectionControl {
        cancelled: bool,
        now_ms: u64,
    }
    impl AuthorityOperationControl for SelectionControl {
        fn now_ms(&self) -> u64 {
            self.now_ms
        }
        fn is_cancelled(&self) -> bool {
            self.cancelled
        }
        fn report(&self, _progress: AuthorityProgress) {}
    }
    let _registry = semio_hub_artifact_authority::REAL_LINKED_CODEC_REGISTRY.lock().await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️native-openable-provider/🧪️tests/🧫️fixtures/🌍️gis-v1/🔣️.json")).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🌎️hub/🧩️compositions/🌍️gis/📇️native-codecs/🔣️.json")).unwrap();
    assert_eq!(fixture["packageVersion"], expected["packageVersion"]);
    let providers = NativeCodecProviderSetV1::linked();
    let mut published = Vec::new();
    for row in expected["receipts"].as_array().unwrap() {
        published.push(document_codec(row["schema"].as_str().unwrap()).await.unwrap().map(|codec| codec.pack_schema_hash));
    }
    for case in fixture["cases"].as_array().unwrap() {
        let control = SelectionControl { cancelled: case["cancelled"].as_bool().unwrap(), now_ms: case["nowMs"].as_u64().unwrap() };
        let context = OperationContext::new(case["deadlineMs"].as_u64().unwrap(), AuthorityLimits::maximum(), &control);
        let result = providers.preview(case["pluginId"].as_str().unwrap(), case["packageId"].as_str().unwrap(), case["version"].as_str().unwrap(), &context);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
        if let Ok(bindings) = result {
            assert_eq!(bindings.len(), case["bindings"].as_u64().unwrap() as usize, "{}", case["name"]);
            assert_eq!(bindings.is_empty(), case["code"] == "unlinked-package", "{}", case["name"]);
            for (binding, row) in bindings.iter().zip(expected["receipts"].as_array().unwrap()) {
                assert_eq!(binding.plugin_id, expected["pluginId"]);
                assert_eq!(binding.package_id, expected["packageId"]);
                assert_eq!(binding.artifact_kind, row["kind"]);
                assert_eq!(binding.codec.schema, row["schema"]);
                assert_eq!(binding.codec.extension, row["extension"]);
                let receipt = semio_hub_gis::native_codecs::native_codec_factory_receipts().unwrap().into_iter().find(|receipt| receipt.identity().schema == binding.codec.schema).unwrap();
                assert_eq!(hex_lower(&receipt.identity().protocol_sha256), row["protocolSha256"]);
                assert_eq!(binding.codec.pack_schema_hash, receipt.into_codec().unwrap().pack_schema_hash);
            }
        }
        for (row, prior) in expected["receipts"].as_array().unwrap().iter().zip(&published) {
            assert_eq!(&document_codec(row["schema"].as_str().unwrap()).await.unwrap().map(|codec| codec.pack_schema_hash), prior, "{} must not publish even a partial codec closure", case["name"]);
        }
    }
}

/// 🧩️ A selected provider that fails, substitutes, conflicts or binds foreign rows publishes nothing; a provider that
/// binds NOTHING for a declared row is no trust failure — the row waits for its component's own answer (the catalog
/// verifies guest rows lazily) and is never served while its component does not give it.
#[tokio::test]
async fn selected_native_provider_failure_substitution_and_conflict_publish_no_partial_closure() {
    for hostile in ["foreign", "missing", "duplicate", "zero", "hash", "extra", "provider-failure", "registry-conflict"] {
        let mut fixture = prepared_fixture();
        let mut source = FixtureProviderSource::new(fixture.make_two_codec_bindings());
        source.hostile = Some(hostile);
        if hostile == "provider-failure" {
            source.failure = Some("semio:fixture-editor");
        }
        if hostile == "registry-conflict" {
            os_store::codec_registry::register_document_codec(fixture_codec(&fixture.schema, [0x22; 32])).expect("prior immutable owner");
        }
        if hostile == "missing" {
            let control = TestControl::new();
            let catalog = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &source, &control.context()).await.expect("a row no provider binds loads and waits for its component's answer");
            assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-base", "semio:fixture-editor"]);
            let pending = catalog.codecs.iter().find(|codec| codec.identity.artifact_schema == fixture.schema).expect("the unbound row's codec");
            assert!(pending.guest.genesis("fixture-document", &control.context()).await.is_err(), "the unbound row is never served: its component does not answer it");
            assert!(document_codec(&fixture.schema).await.expect("registry").is_none(), "no native codec is published for the unbound row");
            continue;
        }
        assert!(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &source, &TestControl::new().context()).await.is_err(), "{hostile}");
        assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-base", "semio:fixture-editor"], "{hostile}");
        assert!(document_codec(&format!("{}.base", fixture.schema)).await.expect("registry").is_none(), "{hostile} published first provider");
        let existing = document_codec(&fixture.schema).await.expect("registry");
        if hostile == "registry-conflict" {
            assert_eq!(existing.expect("prior owner retained").pack_schema_hash, [0x22; 32]);
        } else {
            assert!(existing.is_none(), "{hostile} published second provider");
        }
    }
    {
        let fixture = prepared_fixture();
        let control = TestControl::new();
        let catalog = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &NativeCodecProviderSetV1::linked(), &control.context()).await.expect("rows the linked providers do not bind wait for their components");
        let pending = catalog.codecs.iter().find(|codec| codec.identity.artifact_schema == fixture.schema).expect("the fixture row's codec");
        assert!(pending.guest.genesis("fixture-document", &control.context()).await.is_err(), "an unlinked row whose component does not answer it is never served");
        assert!(document_codec(&fixture.schema).await.expect("registry").is_none());
    }
}

#[tokio::test]
async fn selected_native_provider_descriptor_and_cancellation_fences_precede_publication() {
    let cases: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).unwrap();
    for case in cases["descriptorPreviewCases"].as_array().unwrap() {
        let mut invalid = prepared_fixture();
        let source = FixtureProviderSource::new(invalid.make_two_codec_bindings());
        let index = case["packageIndex"].as_u64().unwrap() as usize;
        let descriptor_path = invalid.root.join(invalid.bundle["packages"][index]["descriptor"]["path"].as_str().expect("descriptor path"));
        std::fs::write(descriptor_path, b"invalid descriptor").expect("hostile descriptor bytes");
        assert!(TrustedCatalogLoader::load_fixture(&invalid.bundle_path, "fixture", &source, &TestControl::new().context()).await.is_err());
        assert_eq!(serde_json::to_value(source.calls.lock().expect("calls").clone()).unwrap(), case["previews"], "{}", case["id"]);
        assert!(document_codec(&invalid.schema).await.expect("registry").is_none());
        assert!(document_codec(&format!("{}.base", invalid.schema)).await.expect("registry").is_none());
    }

    for before in [true, false] {
        let mut fixture = prepared_fixture();
        let control = TestControl::new();
        control.cancelled.store(before, Ordering::SeqCst);
        let mut source = FixtureProviderSource::new(fixture.make_two_codec_bindings());
        source.cancel_after_preview = Some(&control);
        let result = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &source, &control.context()).await;
        assert!(matches!(result, Err(AuthorityError::Cancelled)));
        let calls = source.calls.lock().expect("calls").clone();
        assert_eq!(calls, if before { Vec::<String>::new() } else { vec!["semio:fixture-base".into()] });
        assert!(document_codec(&fixture.schema).await.expect("registry").is_none());
        assert!(document_codec(&format!("{}.base", fixture.schema)).await.expect("registry").is_none());
    }
}

#[tokio::test]
async fn neutral_fixture_proves_dependency_order_hash_oracles_and_exact_limit_edges() {
    let fixture = snapshot_json();
    let bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle shape");
    assert_eq!(validate_bundle(&bundle, "fixture").expect("valid closure").package_indices, vec![1, 0]);
    let bytes = fixture_component_bytes(&fixture);
    let control = TestControl::new();
    let (sha, internal_blake3) = dual_hash(&bytes, &control.context()).await.expect("dual hash");
    assert_eq!(hex_lower(&sha), fixture["componentSha256"]);
    assert_eq!(hex_lower(&internal_blake3), fixture["componentBlake3"]);
    assert_eq!(blake3::hash(&bytes).to_hex().as_str(), fixture["componentBlake3"]);

    let limits = &fixture["limits"];
    assert_eq!(limits["componentBytesMax"], TRUSTED_COMPONENT_MAX_BYTES);
    assert_eq!(limits["componentBytesMaxPlusOne"], TRUSTED_COMPONENT_MAX_BYTES + 1);
    assert!(validate_file(&TrustedBundleFileV1 { path: "component.wasm".to_string(), byte_length: TRUSTED_COMPONENT_MAX_BYTES, sha256: "11".repeat(32) }, TRUSTED_COMPONENT_MAX_BYTES).is_ok());
    assert!(validate_file(&TrustedBundleFileV1 { path: "component.wasm".to_string(), byte_length: TRUSTED_COMPONENT_MAX_BYTES + 1, sha256: "11".repeat(32) }, TRUSTED_COMPONENT_MAX_BYTES).is_err());
    assert_eq!(limits["descriptorBytesMax"], TRUSTED_DESCRIPTOR_MAX_BYTES);
    assert!(validate_file(&TrustedBundleFileV1 { path: "descriptor.semio".to_string(), byte_length: TRUSTED_DESCRIPTOR_MAX_BYTES, sha256: "11".repeat(32) }, TRUSTED_DESCRIPTOR_MAX_BYTES).is_ok());
    assert!(validate_file(&TrustedBundleFileV1 { path: "descriptor.semio".to_string(), byte_length: TRUSTED_DESCRIPTOR_MAX_BYTES + 1, sha256: "11".repeat(32) }, TRUSTED_DESCRIPTOR_MAX_BYTES).is_err());
    assert!(valid_identity(&"a".repeat(TRUSTED_IDENTITY_MAX_BYTES)));
    assert!(!valid_identity(&"a".repeat(TRUSTED_IDENTITY_MAX_BYTES + 1)));
    assert!(ensure_count(TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_CODECS, "trusted codec count").is_ok());
    assert!(ensure_count(TRUSTED_CATALOG_MAX_CODECS + 1, TRUSTED_CATALOG_MAX_CODECS, "trusted codec count").is_err());
}
