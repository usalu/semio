/// 🛡️ Laws of exact byte retention, execution-target assets, document-open resolution, opened roots and trust failures before activation.

#[tokio::test]
async fn loader_retains_exact_bytes_and_independent_identities_before_atomic_codec_activation() {
    let fixture = prepared_fixture();
    let binding = fixture.binding();
    let control = TestControl::new();
    let catalog = load_fixture(&fixture, &[binding], &control.context()).await.expect("verified catalog");
    assert_eq!(catalog.packages().iter().map(VerifiedTrustedPackage::plugin_id).collect::<Vec<_>>(), vec!["fixture-base", "fixture-editor"]);
    let editor = &catalog.packages()[1];
    assert_eq!(editor.package_ref().package.0, "semio:fixture-editor");
    assert_eq!(hex_lower(&editor.package_ref().hash.0), snapshot_json()["componentBlake3"]);
    assert_ne!(editor.plugin_id(), editor.package_ref().package.0);
    assert_eq!(editor.component().byte_length(), 8);
    assert_eq!(&*editor.component().read(&control.context()).await.expect("retained component rereads"), fixture_component_bytes(&snapshot_json()).as_slice());
    assert_eq!(editor.receipt().wasm_sha256, hex_lower(editor.component_sha256()));
    assert_eq!(editor.receipt().semantic_descriptor_sha256, hex_lower(editor.descriptor_sha256()));
    assert_eq!(hex_lower(editor.component_sha256()), snapshot_json()["componentSha256"]);
    assert_eq!(editor.descriptor().manifest.plugin_id, editor.plugin_id());
    assert_eq!(editor.descriptor().manifest.version, editor.version());
    assert_eq!(Sha256::digest(editor.descriptor_bytes()), *editor.descriptor_sha256());
    assert_eq!(catalog.codec_count(), 1);
    assert_eq!(catalog.open_target_count(), 1);
    assert_eq!(catalog.generation_id().len(), 64);
    let descriptor = DocumentDescriptor {
        space_id: "space".into(),
        document_id: "document".into(),
        artifact_kind: "s.fixture.document".into(),
        artifact_schema: fixture.schema.clone(),
        owner: directory::os_directory::DocumentOwner {
            plugin_id: "fixture-editor".into(),
            package_id: "semio:fixture-editor".into(),
            version: "1.2.3".into(),
            package_hash: snapshot_json()["componentSha256"].as_str().expect("component sha256").into(),
        },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    };
    assert!(catalog.resolve(&TrustedArtifactIdentity::from_descriptor(&descriptor)).await.is_ok(), "the codec and open plan use the same descriptor SHA-256 owner identity");
    let editor = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("exact editor target");
    assert_eq!(editor.surface.role, DocumentOpenSurfaceRoleV1::Editor);
    assert!(editor.grant.write);
    assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), false).is_none());
    assert!(catalog.resolve_document_open(&descriptor, None, true).is_some());
    assert!(document_codec(&fixture.schema).await.expect("codec registry").is_some());
    let progress = control.progress.lock().expect("progress lock");
    assert_eq!(progress.first().map(|entry| entry.stage), Some(AuthorityProgressStage::Preflight));
    assert_eq!(progress.last().map(|entry| entry.stage), Some(AuthorityProgressStage::CatalogResolved));
    assert!(progress.iter().all(|entry| entry.completed_units <= entry.total_units));
}

/// 🧱 The exact-selection asset accessor is bound to the current generation and to every
/// selected digest: it answers only for the exact descriptor/role/surface the catalog itself
/// resolves, only while the caller-observed generation is still this catalog's own, and the
/// bytes it returns are the very bytes whose SHA-256/BLAKE3 the selection projects.
///
/// 🔁 A rotated (or merely guessed) generation is never served, and no role, surface or
/// descriptor substitution reaches bytes.
#[tokio::test]
async fn selected_execution_target_assets_are_generation_and_digest_bound() {
    let fixture = prepared_fixture();
    let control = TestControl::new();
    let catalog = load_fixture(&fixture, &[fixture.binding()], &control.context()).await.expect("verified catalog");
    let descriptor = DocumentDescriptor {
        space_id: "space".into(),
        document_id: "document".into(),
        artifact_kind: "s.fixture.document".into(),
        artifact_schema: fixture.schema.clone(),
        owner: directory::os_directory::DocumentOwner {
            plugin_id: "fixture-editor".into(),
            package_id: "semio:fixture-editor".into(),
            version: "1.2.3".into(),
            package_hash: snapshot_json()["componentSha256"].as_str().expect("component sha256").into(),
        },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    };
    let generation = catalog.generation_id().to_string();
    let assets = catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), true, &generation).expect("selected assets");
    let selection = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("selection");
    assert_eq!(assets.selection, selection);
    let component = assets.component.read(&control.context()).await.expect("selected component rereads");
    assert!(!component.is_empty() && !assets.descriptor.is_empty());
    assert_eq!(hex_lower(&Sha256::digest(&component)), assets.selection.package.component_sha256);
    assert_eq!(semio_framework_hash::hash_bytes(&component), assets.selection.package.component_blake3);
    assert_eq!(hex_lower(&Sha256::digest(&assets.descriptor)), assets.selection.package.descriptor_byte_sha256);
    let actor = assets.browser_actor.as_ref().expect("Wasm selection retains its actor").read(&control.context()).await.expect("selected actor rereads");
    let retained = catalog.packages.iter().find(|package| package.plugin_id == selection.package.plugin_id).unwrap();
    assert!(Arc::ptr_eq(&actor, &retained.browser_actor_asset.as_ref().unwrap().read(&control.context()).await.expect("resident actor")));
    assert_eq!(actor.as_ref(), b"abc");
    assert_eq!(assets.selection.browser_actor, retained.browser_actor);
    assert!(assets.component.byte_length() <= TRUSTED_COMPONENT_MAX_BYTES && assets.descriptor.len() as u64 <= TRUSTED_DESCRIPTOR_MAX_BYTES);
    assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), true, &"ab".repeat(32)).is_none());
    assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), false, &generation).is_none());
    assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#viewer"), true, &generation).is_none());
    assert!(catalog.assets_for_current_selection(&descriptor, Some("foreign"), true, &generation).is_none());
    for change in ["plugin", "package", "version", "hash", "kind", "schema", "pack"] {
        let mut candidate = descriptor.clone();
        match change {
            "plugin" => candidate.owner.plugin_id.push_str(".foreign"),
            "package" => candidate.owner.package_id.push_str(".foreign"),
            "version" => candidate.owner.version.push_str("-foreign"),
            "hash" => candidate.owner.package_hash = "ab".repeat(32),
            "kind" => candidate.artifact_kind.push_str(".foreign"),
            "schema" => candidate.artifact_schema.push_str("-foreign"),
            "pack" => candidate.pack_schema_hash = "ab".repeat(32),
            _ => unreachable!(),
        }
        assert!(catalog.assets_for_current_selection(&candidate, Some("s.fixture.document@1/*#editor"), true, &generation).is_none(), "descriptor {change} reached selected bytes");
    }
}

#[tokio::test]
async fn verified_trusted_catalog_document_open_generation_and_resolution_are_exact() {
    let fixture = prepared_fixture();
    let catalog = load_fixture(&fixture, &[fixture.binding()], &TestControl::new().context()).await.expect("verified catalog");
    let descriptor = DocumentDescriptor {
        space_id: "space".into(),
        document_id: "document".into(),
        artifact_kind: "s.fixture.document".into(),
        artifact_schema: fixture.schema.clone(),
        owner: directory::os_directory::DocumentOwner {
            plugin_id: "fixture-editor".into(),
            package_id: "semio:fixture-editor".into(),
            version: "1.2.3".into(),
            package_hash: snapshot_json()["componentSha256"].as_str().expect("component sha256").into(),
        },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    };
    assert_eq!(catalog.open_target_count(), 1);
    assert_eq!(catalog.generation_id().len(), 64);
    assert!(catalog.generation_id().bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')));
    let editor = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("editor");
    assert_eq!(editor.surface.role, DocumentOpenSurfaceRoleV1::Editor);
    assert_eq!(editor.grant, DocumentOpenGrantV1 { read: true, write: true, observe: true });
    assert_eq!(editor.parent_dialect, semio_framework::ArtifactDialect { artifact_kind: "s.fixture.document".into(), standard: "1".into(), subset: "*".into() });
    assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), false).is_none());
    for field in ["artifactKind", "standard", "subset"] {
        let mut changed: TrustedBundleV1 = serde_json::from_value(fixture.bundle.clone()).expect("bundle");
        let dialect = &mut changed.profiles[0].open_targets[0].target.parent_dialect;
        match field {
            "artifactKind" => dialect.artifact_kind.push_str(".foreign"),
            "standard" => dialect.standard.push_str("-foreign"),
            "subset" => dialect.subset.push_str("-foreign"),
            _ => unreachable!(),
        }
        let changed_generation = trusted_profile_generation(&changed, &changed.profiles[0]).unwrap();
        assert_ne!(changed_generation, catalog.generation_id(), "catalog binds parent {field}");
    }
    assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), true).is_none());
    assert!(catalog.resolve_document_open(&descriptor, Some("foreign"), false).is_none());
    let roles: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️identity-roles/🔣️.json")).unwrap();
    for case in roles["cases"].as_array().unwrap() {
        let mut candidate = descriptor.clone();
        let mut surface = "s.fixture.document@1/*#editor".to_owned();
        match case["change"].as_str().unwrap() {
            "none" => {}
            "blake3-owner" => candidate.owner.package_hash = snapshot_json()["componentBlake3"].as_str().unwrap().to_owned(),
            "descriptor-owner" => candidate.owner.package_hash = hex_lower(catalog.packages()[1].descriptor_sha256()),
            "zero-owner" => candidate.owner.package_hash = "00".repeat(32),
            "bare-kind" => candidate.artifact_kind = "fixture.document".to_owned(),
            "bare-surface" => surface = "fixture.document@1/*#editor".to_owned(),
            _ => panic!("unknown catalog identity role vector"),
        }
        assert_eq!(catalog.resolve(&TrustedArtifactIdentity::from_descriptor(&candidate)).await.is_ok(), case["codec"].as_bool().unwrap(), "codec {}", case["change"]);
        assert_eq!(catalog.resolve_document_open(&candidate, Some(&surface), true).is_some(), case["open"].as_bool().unwrap(), "open {}", case["change"]);
    }
}

#[tokio::test]
async fn trusted_browser_actor_loader_verifies_retains_and_cancels_before_publication() {
    struct ActorControl {
        control: TestControl,
        cancel_after_descriptor: bool,
    }
    impl AuthorityOperationControl for ActorControl {
        fn now_ms(&self) -> u64 {
            0
        }
        fn is_cancelled(&self) -> bool {
            self.control.is_cancelled()
        }
        fn report(&self, progress: AuthorityProgress) {
            if self.cancel_after_descriptor && progress.stage == AuthorityProgressStage::CatalogLoading && progress.completed_units == 6 {
                self.control.cancelled.store(true, Ordering::SeqCst);
            }
            self.control.report(progress);
        }
    }
    let corpus: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🌐️browser-actor/🔣️.json")).unwrap();
    for law in corpus["loadCases"].as_array().unwrap() {
        let mut fixture = prepared_fixture();
        let path = fixture.root.join(fixture.bundle["packages"][0]["browserActor"]["path"].as_str().unwrap());
        if let Some(hex) = law["bodyHex"].as_str() {
            let bytes: Vec<u8> = hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
            std::fs::write(&path, bytes).unwrap();
        } else {
            std::fs::remove_file(&path).unwrap();
        }
        fixture.bundle["packages"][0]["browserActor"]["byteLength"] = law["byteLength"].clone();
        fixture.refresh_profile_generation();
        fixture.persist_bundle();
        let control = ActorControl { control: TestControl::new(), cancel_after_descriptor: law["cancelAfterDescriptor"].as_bool().unwrap() };
        let provider = FixtureProviderSource::new(vec![fixture.binding()]);
        let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
        let result = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &context).await;
        let accepted = law["accepted"].as_bool().unwrap();
        assert_eq!(result.is_ok(), accepted, "{}", law["id"]);
        assert_eq!(document_codec(&fixture.schema).await.unwrap().is_some(), accepted, "{} codec publication", law["id"]);
        let calls = provider.calls.lock().unwrap();
        assert_eq!(calls.iter().any(|package| package == "semio:fixture-editor"), accepted, "{} provider preview", law["id"]);
        if let Ok(catalog) = result {
            let package = catalog.packages.iter().find(|package| package.plugin_id == "fixture-editor").unwrap();
            let asset = package.browser_actor_asset.as_ref().unwrap();
            let bytes = asset.read(&context).await.expect("verified actor rereads");
            assert_eq!(bytes.as_ref(), b"abc");
            let selected = catalog.selected_document_open().unwrap();
            assert_eq!(selected.browser_actor, package.browser_actor);
            std::fs::write(&path, b"untrusted replacement").unwrap();
            assert_eq!(bytes.as_ref(), b"abc");
            let DocumentOpenBrowserActorV1::ClosedBrowserActor { sha256, source_component_sha256, source_descriptor_byte_sha256, .. } = &selected.browser_actor else { panic!("required actor") };
            assert_eq!(*sha256, hex_lower(&Sha256::digest(&bytes)));
            assert_eq!(*source_component_sha256, selected.package.component_sha256);
            assert_eq!(*source_descriptor_byte_sha256, selected.package.descriptor_byte_sha256);
            assert!(Arc::ptr_eq(&bytes, &asset.read(&context).await.expect("held actor stays resident")));
            drop(bytes);
            assert!(asset.read(&context).await.is_err(), "a replaced actor file is refused once no reader holds the verified bytes");
        } else if control.cancel_after_descriptor {
            assert!(matches!(result, Err(AuthorityError::Cancelled)));
        }
    }
}

#[tokio::test]
async fn trusted_catalog_opened_root_rejects_linked_roots_leaves_intermediates_and_actors() {
    let leaf = prepared_fixture();
    let leaf_path = leaf.component_path(0);
    let leaf_target = leaf.root.join("components/retained-component.wasm");
    std::fs::write(&leaf_target, std::fs::read(&leaf_path).expect("read component")).expect("write retained component");
    std::fs::remove_file(&leaf_path).expect("remove component leaf");
    fixture_file_link(&leaf_target, &leaf_path);
    assert_linked_fixture_denied(&leaf).await;

    let intermediate = prepared_fixture();
    let components = intermediate.root.join("components");
    let retained_components = intermediate.root.join("retained-components");
    std::fs::rename(&components, &retained_components).expect("retain component directory");
    fixture_directory_link(&retained_components, &components);
    assert_linked_fixture_denied(&intermediate).await;

    let actor = prepared_fixture();
    let actor_path = actor.root.join("browser/closed-actor.mjs");
    let actor_target = actor.root.join("browser/retained-actor.mjs");
    std::fs::write(&actor_target, std::fs::read(&actor_path).expect("read actor")).expect("write retained actor");
    std::fs::remove_file(&actor_path).expect("remove actor leaf");
    fixture_file_link(&actor_target, &actor_path);
    assert_linked_fixture_denied(&actor).await;

    let owner_fixture = prepared_fixture();
    let actual_data = owner_fixture.root.join("owned-data");
    std::fs::create_dir(&actual_data).expect("create owned data root");
    let linked_data = owner_fixture.root.join("linked-data");
    fixture_directory_link(&actual_data, &linked_data);
    let canonical_owner_root = std::fs::canonicalize(&owner_fixture.root).expect("canonical fixture-owned parent root");
    assert!(TrustedCatalogDataRoot::open_server_owned(&canonical_owner_root.join("linked-data")).is_err(), "linked initial data root was admitted");

    let trusted = actual_data.join("trusted-catalog");
    std::fs::create_dir(&trusted).expect("create trusted root");
    let current_target = actual_data.join("retained-current.json");
    std::fs::write(&current_target, b"{}\n").expect("write current target");
    fixture_file_link(&current_target, &trusted.join("current.json"));
    let canonical_data = std::fs::canonicalize(&actual_data).expect("canonical fixture-owned data root");
    let data_root = TrustedCatalogDataRoot::open_server_owned(&canonical_data).expect("open fixture-owned data root");
    assert!(data_root.open_current().is_err(), "linked current pointer was admitted");

    let generations = trusted.join("generations");
    std::fs::create_dir(&generations).expect("create generations root");
    let retained_generation = trusted.join("retained-generation");
    std::fs::create_dir(&retained_generation).expect("create retained generation");
    let generation_id = "ab".repeat(32);
    fixture_directory_link(&retained_generation, &generations.join(&generation_id));
    assert!(data_root.open_generation(&generation_id).is_err(), "linked generation root was admitted");
}

#[tokio::test]
async fn trusted_catalog_opened_handle_is_swap_stable_bounded_and_cancel_safe() {
    let fixture = prepared_fixture();
    let canonical_root = std::fs::canonicalize(&fixture.root).expect("canonical fixture-owned generation root");
    let root = TrustedCatalogGenerationRoot::open_fixture_owned(&canonical_root).expect("opened generation root");
    let relative = TrustedCatalogRelativePathV1::parse(fixture.bundle["packages"][0]["component"]["path"].as_str().expect("component path")).expect("relative component path");
    let opened = root.open_regular(&relative).expect("open retained component handle");
    let component_path = fixture.component_path(0);
    std::fs::rename(&component_path, fixture.root.join("original-component.wasm")).expect("retain opened component inode");
    std::fs::write(&component_path, b"xyz").expect("write substituted component path");
    assert_eq!(opened.read_bounded(3, &TestControl::new().context()).await.expect("read retained handle"), b"abc");
    let provider = FixtureProviderSource::new(vec![fixture.binding()]);
    assert!(TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &TestControl::new().context()).await.is_err(), "substituted component digest loaded");
    assert!(provider.calls.lock().expect("provider calls").is_empty());
    assert!(document_codec(&fixture.schema).await.expect("codec registry").is_none());

    let zero_path = fixture.root.join("zero.bin");
    std::fs::write(&zero_path, []).expect("write zero file");
    let zero = root.open_regular(&TrustedCatalogRelativePathV1::parse("zero.bin").expect("zero path")).expect("open zero file");
    assert!(zero.read_bounded(1, &TestControl::new().context()).await.is_err());

    let over_path = fixture.root.join("over.bin");
    std::fs::write(&over_path, b"abcd").expect("write over-bound file");
    let over = root.open_regular(&TrustedCatalogRelativePathV1::parse("over.bin").expect("over path")).expect("open over-bound file");
    assert!(over.read_bounded(3, &TestControl::new().context()).await.is_err());

    let growth_path = fixture.root.join("growth.bin");
    std::fs::write(&growth_path, b"abc").expect("write growth file");
    let growth = root.open_regular(&TrustedCatalogRelativePathV1::parse("growth.bin").expect("growth path")).expect("open growth file");
    std::fs::OpenOptions::new().append(true).open(&growth_path).expect("open growth writer").write_all(b"d").expect("grow opened file");
    assert!(growth.read_bounded(4, &TestControl::new().context()).await.is_err(), "opened handle admitted growth within the caller maximum");

    #[cfg(unix)]
    {
        let fifo_path = fixture.root.join("special.fifo");
        create_fifo_fixture(&fifo_path).expect("create FIFO fixture");
        assert!(root.open_regular(&TrustedCatalogRelativePathV1::parse("special.fifo").expect("FIFO path")).is_err(), "opened root admitted a blocking special-file leaf");
    }

    struct CancelAfterTwoCheckpoints(AtomicUsize);
    impl AuthorityOperationControl for CancelAfterTwoCheckpoints {
        fn now_ms(&self) -> u64 {
            0
        }
        fn is_cancelled(&self) -> bool {
            self.0.fetch_add(1, Ordering::SeqCst) >= 2
        }
        fn report(&self, _progress: AuthorityProgress) {}
    }
    let large_path = fixture.root.join("large.bin");
    std::fs::write(&large_path, vec![7u8; 64 * 1024 + 1]).expect("write multi-chunk file");
    let large = root.open_regular(&TrustedCatalogRelativePathV1::parse("large.bin").expect("large path")).expect("open multi-chunk file");
    let cancel = CancelAfterTwoCheckpoints(AtomicUsize::new(0));
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &cancel);
    assert!(matches!(large.read_bounded(64 * 1024 + 1, &context).await, Err(AuthorityError::Cancelled)));
}

#[test]
fn trusted_catalog_relative_paths_match_the_neutral_no_link_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛡️opened-root/🔣️.json")).expect("opened-root corpus");
    let rows = corpus["relativePaths"].as_array().expect("relative path rows");
    assert_eq!(rows.len(), 15);
    for row in rows {
        let value = row["value"].as_str().expect("relative path value");
        assert_eq!(TrustedCatalogRelativePathV1::parse(value).is_ok(), row["accepted"].as_bool().expect("relative path acceptance"), "{}", row["id"]);
    }
}

#[test]
fn descriptor_projection_rejects_package_conflicts_unknown_fields_and_duplicate_fields() {
    let fixture = snapshot_json();
    let bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    let canonical = descriptor_bytes("fixture-editor", "semio:fixture-editor", "1.2.3", Some("fixture.document@1"), Some(("fixture-base", "1.0.0")));
    decode_package_descriptor(&canonical).unwrap_or_else(|error| panic!("canonical descriptor must decode: {error}"));

    let conflicting = decode_package_descriptor(&descriptor_bytes("fixture-editor", "semio:other-package", "1.2.3", Some("fixture.document@1"), Some(("fixture-base", "1.0.0")))).expect("structurally valid conflicting descriptor");
    let refusal = validate_descriptor(&bundle.packages[0], &conflicting, &bundle.packages).expect_err("package mismatch").to_string();
    assert!(refusal.contains("identity") && refusal.contains("semio:fixture-editor") && refusal.ends_with("packageId"), "{refusal}");

    let mut stale_record = bundle.packages[0].clone();
    let mut stale = decode_package_descriptor(&canonical).expect("canonical descriptor");
    stale_record.execution_protocol.app_channel_version = directory::os_spr::CHANNEL_VERSION - 1;
    stale.execution_protocol.app_channel_version = directory::os_spr::CHANNEL_VERSION - 1;
    let refusal = validate_descriptor(&stale_record, &stale, &bundle.packages).expect_err("a catalog of another app channel").to_string();
    assert!(refusal.contains("semio:fixture-editor") && refusal.contains(&format!("app channel {}", directory::os_spr::CHANNEL_VERSION - 1)) && refusal.contains(&format!("speaks app channel {}", directory::os_spr::CHANNEL_VERSION)), "{refusal}");

    let mut unknown = os_pack::pack_rt::decode_wire_value(&canonical).expect("canonical value");
    let DslValue::Object(fields) = &mut unknown else { panic!("descriptor object") };
    fields.push(("unexpected".to_owned(), DslValue::Null));
    let error = decode_package_descriptor(&os_pack::pack_rt::encode_wire_value(&unknown)).expect_err("unknown descriptor field");
    assert!(error.to_string().contains("unknown field") || error.to_string().contains("canonical schema projection"));

    let mut duplicate = os_pack::pack_rt::decode_wire_value(&canonical).expect("canonical value");
    let DslValue::Object(fields) = &mut duplicate else { panic!("descriptor object") };
    fields.push(("packageId".to_owned(), DslValue::String("semio:fixture-editor".to_owned())));
    assert!(decode_package_descriptor(&os_pack::pack_rt::encode_wire_value(&duplicate)).expect_err("duplicate descriptor field").to_string().contains("duplicate object field"));
    assert!(valid_package_id("semio:fixture-editor"));
    assert!(!valid_package_id("fixture.editor.native"));
    assert!(!valid_package_id("semio:Fixture"));
}

#[tokio::test]
async fn all_trust_failures_precede_activation_and_have_bounded_diagnostics() {
    let mut mutated = prepared_fixture();
    std::fs::write(mutated.component_path(0), b"abd").expect("mutate component");
    let control = TestControl::new();
    let error = expect_load_error(&mutated, &[mutated.binding()], &control).await;
    assert!(error.to_string().contains("digest"));
    assert!(document_codec(&mutated.schema).await.expect("codec registry").is_none());

    let mut descriptor_mutation = prepared_fixture();
    let descriptor_path = descriptor_mutation.root.join(descriptor_mutation.bundle["packages"][0]["descriptor"]["path"].as_str().expect("descriptor path"));
    let mut bytes = std::fs::read(&descriptor_path).expect("descriptor bytes");
    let last = bytes.last_mut().expect("descriptor byte");
    *last ^= 1;
    std::fs::write(descriptor_path, bytes).expect("mutate descriptor");
    let error = expect_load_error(&descriptor_mutation, &[descriptor_mutation.binding()], &control).await;
    assert!(error.to_string().contains("digest"));
    assert!(document_codec(&descriptor_mutation.schema).await.expect("codec registry").is_none());

    let missing = prepared_fixture();
    let context = control.context();
    let loaded = load_fixture(&missing, &[], &context).await.expect("a row no provider answers for loads and waits for its component's answer");
    let owner = loaded.codecs.iter().find(|codec| codec.identity.artifact_schema == missing.schema).expect("the pending row's codec");
    let staged = |catalog: &VerifiedTrustedCatalog| catalog.load_progress().packages[owner.guest.component.position].phase;
    assert_eq!(staged(&loaded), TrustedCatalogPackagePhaseV1::Staged, "nothing interpreted the component during the load");
    let error = owner.guest.genesis("fixture-document", &context).await.expect_err("no codec call runs before its rows are pinned");
    assert!(error.to_string().contains(&missing.schema), "the refusal names the row it was pinning: {error}");
    assert_eq!(staged(&loaded), TrustedCatalogPackagePhaseV1::Refused);
    loaded.verify_pending_guests(&context).await.expect("a refused package does not stop the background pass");
    assert_eq!(loaded.load_progress().packages_refused, 1);
    assert!(owner.guest.genesis("fixture-document", &context).await.expect_err("a refused package refuses every call").to_string().contains(&missing.schema));
    assert!(document_codec(&missing.schema).await.expect("codec registry").is_none());

    let wrong_package = prepared_fixture();
    let lossy = NativeCodecBinding::new("fixture-editor", "semio:fixture-wrong", "s.fixture.document", None, fixture_codec(&wrong_package.schema, [0x11; 32]));
    let error = expect_load_error(&wrong_package, &[lossy], &control).await;
    assert!(error.to_string().contains("no explicit native codec"));
    assert!(document_codec(&wrong_package.schema).await.expect("codec registry").is_none());

    let mut zero = prepared_fixture();
    zero.bundle["packages"][0]["nativeCodecs"][0]["packSchemaHash"] = "00".repeat(32).into();
    zero.persist_bundle();
    let error = expect_load_error(&zero, &[zero.binding()], &control).await;
    assert!(error.to_string().contains("zero"));
    assert!(document_codec(&zero.schema).await.expect("codec registry").is_none());

    let mut detached_open_target = prepared_fixture();
    detached_open_target.bundle["packages"][0]["openTargets"][0]["packSchemaHash"] = "12".repeat(32).into();
    detached_open_target.persist_bundle();
    let error = expect_load_error(&detached_open_target, &[detached_open_target.binding()], &control).await;
    assert!(error.to_string().contains("open target"));
    assert!(document_codec(&detached_open_target.schema).await.expect("codec registry").is_none());

    let mismatch = prepared_fixture();
    let binding = NativeCodecBinding::new("fixture-editor", "semio:fixture-editor", "s.fixture.document", None, fixture_codec(&mismatch.schema, [0x12; 32]));
    let error = expect_load_error(&mismatch, &[binding], &control).await;
    assert!(error.to_string().contains("mismatched"));
    assert!(document_codec(&mismatch.schema).await.expect("codec registry").is_none());

    let cancelled = prepared_fixture();
    let control = TestControl::new();
    control.cancelled.store(true, Ordering::SeqCst);
    assert_eq!(expect_load_error(&cancelled, &[cancelled.binding()], &control).await, AuthorityError::Cancelled);
    assert!(document_codec(&cancelled.schema).await.expect("codec registry").is_none());
    assert!(catalog_error("x".repeat(AUTHORITY_MAX_DIAGNOSTIC_BYTES * 2)).to_string().len() <= AUTHORITY_MAX_DIAGNOSTIC_BYTES + 40);
}
