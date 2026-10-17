/// 📦️ Laws of served plugin module files, hosted open targets, creation preference and the most general dialect.

#[tokio::test]
async fn a_loaded_catalog_indexes_and_serves_every_verified_plugin_module_file_and_refuses_a_tampered_one() {
    let fixture = prepared_fixture();
    let provider = FixtureProviderSource::new(vec![fixture.binding()]);
    let control = TestControl::new();
    let catalog = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &control.context()).await.expect("fixture catalog");
    let index = catalog.plugin_module_index();
    semio_hub_trusted_catalog_contract::plugin_module::validate_plugin_module_index(&index).expect("canonical index");
    assert_eq!(index.generation_id, catalog.generation_id());
    assert_eq!(index.modules.iter().map(|entry| entry.plugin_id.as_str()).collect::<Vec<_>>(), vec!["fixture-base", "fixture-editor"]);
    assert_eq!(index.modules[1].dependencies, vec!["fixture-base".to_owned()]);
    assert_eq!(index.modules[1].dialect_artifact_kinds, vec!["s.fixture.document".to_owned()]);
    assert!(index.modules[0].dialect_artifact_kinds.is_empty(), "a package without apps opens no dialect");
    assert!(index.modules.iter().all(|entry| entry.extends_plugin_id.is_none()), "plugin packages extend nothing");
    for entry in &index.modules {
        let module = catalog.plugin_module(&entry.bundle_sha256).expect("indexed module");
        assert_eq!(hex_lower(&Sha256::digest(module.manifest_bytes())), entry.bundle_sha256);
        assert_eq!(module.manifest_bytes().len() as u64, entry.bundle_byte_length);
        assert_eq!(module.bundle().entry, entry.entry);
        for file in &module.bundle().files {
            let bytes = module.file(&file.path).expect("listed file").read(&control.context()).await.expect("verified file");
            assert_eq!(hex_lower(&Sha256::digest(&bytes)), file.sha256);
        }
    }
    assert!(catalog.plugin_module(&"00".repeat(32)).is_none());
    let editor = catalog.plugin_module(&index.modules[1].bundle_sha256).expect("editor module");
    let bridge = editor.bundle().files.iter().find(|file| file.path.ends_with("🌉️bridge.js")).expect("editor entry");
    assert!(editor.file("fixture-editor/unlisted.js").is_none());
    std::fs::write(fixture.root.join(semio_hub_trusted_catalog_contract::plugin_module::plugin_module_blob_path(&bridge.sha256)), b"tampered").expect("tamper entry");
    assert!(editor.file(&bridge.path).expect("editor entry").read(&control.context()).await.is_err(), "a module file tampered after verification is never served");
    assert!(drain_stream(editor.file(&bridge.path).expect("editor entry")).await.1.is_some(), "a module file tampered after verification never completes its stream");
}

/// 🚰️ Drains one asset stream: the bytes it released and the error that ended it, if any.
async fn drain_stream(asset: &TrustedCatalogAsset) -> (Vec<u8>, Option<AuthorityError>) {
    let mut stream = match asset.stream() {
        Ok(stream) => stream,
        Err(error) => return (Vec::new(), Some(error)),
    };
    let mut released = Vec::new();
    while let Some(chunk) = stream.next_chunk().await {
        match chunk {
            Ok(bytes) => released.extend_from_slice(&bytes),
            Err(error) => return (released, Some(error)),
        }
    }
    (released, None)
}

/// ⚖️ LAW: a verified file is served as a chunked stream with NO request deadline that releases exactly the verified bytes,
/// and a file changed on disk after verification — same length, one byte flipped anywhere, or truncated — ends the stream in
/// an error before its final chunk, so a reader holding the declared length never completes it. The per-request reread
/// under the 8 s execution-target deadline refused every large core module with 500 once a burst loaded the debug hub
/// (S15, session 12: all 7 core modules ≥ 17 MB of catalog B answered 500 at 8.0–8.2 s, `wp-s15/s15-module-burst.ts`).
#[tokio::test]
async fn a_verified_file_streams_its_exact_bytes_without_a_deadline_and_withholds_its_last_chunk_once_tampered() {
    let fixture = prepared_fixture();
    let canonical_root = std::fs::canonicalize(&fixture.root).expect("canonical fixture-owned generation root");
    let root = Arc::new(TrustedCatalogGenerationRoot::open_fixture_owned(&canonical_root).expect("opened generation root"));
    let bytes: Vec<u8> = (0..(3 * TRUSTED_READ_CHUNK_BYTES + 17)).map(|index| (index % 251) as u8).collect();
    let retained = |name: &str| {
        std::fs::write(fixture.root.join(name), &bytes).expect("write multi-chunk module file");
        TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(&root), TrustedCatalogRelativePathV1::parse(name).expect("relative path"), bytes.len() as u64, Sha256::digest(&bytes), Some(*Hasher::new().update(&bytes).finalize().as_bytes())))
    };

    let intact = retained("intact.bin");
    assert_eq!(intact.stream().expect("stream").byte_length(), bytes.len() as u64);
    let (released, error) = drain_stream(&intact).await;
    assert!(error.is_none(), "an intact verified file streams to its end: {error:?}");
    assert_eq!(released, bytes, "the stream releases exactly the verified bytes");

    for (name, offset) in [("flip-first.bin", 0), ("flip-middle.bin", TRUSTED_READ_CHUNK_BYTES + 5), ("flip-last.bin", bytes.len() - 1)] {
        let asset = retained(name);
        let mut tampered = bytes.clone();
        tampered[offset] ^= 0xff;
        std::fs::write(fixture.root.join(name), &tampered).expect("tamper after verification");
        let (released, error) = drain_stream(&asset).await;
        assert!(error.is_some(), "{name}: a file changed after verification ends its stream in an error");
        assert!(released.len() < bytes.len(), "{name}: the final chunk is withheld, so the declared length is never completed ({} of {})", released.len(), bytes.len());
    }

    let truncated = retained("truncated.bin");
    std::fs::write(fixture.root.join("truncated.bin"), &bytes[..bytes.len() - 1]).expect("truncate after verification");
    assert!(drain_stream(&truncated).await.1.is_some(), "a truncated file is refused before it streams");

    let (released, error) = drain_stream(&TrustedCatalogAsset::resident(Arc::from(bytes.clone()))).await;
    assert!(error.is_none() && released == bytes, "a resident asset streams the bytes the catalog verified");
}

/// 🏠️ A hosted kind's document-open target binds the ONE declared dependency in the catalog that declares its exact native
/// codec — never a copied row, never an owner read off the kind id (demonstrator's `3d.cad` names no plugin): accepted for a
/// present declared dependency declaring the exact codec; refused by name when none does, when it is absent or undeclared, and
/// when two declared dependencies declare the same exact codec.
#[test]
fn hosted_open_targets_bind_the_owner_codec_or_are_refused_by_name() {
    let fixture = snapshot_json();
    let bundle: TrustedBundleV1 = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
    let mut host = bundle.packages[0].clone();
    let mut owner = bundle.packages[1].clone();
    let target = host.open_targets[0].clone();
    owner.native_codecs = std::mem::take(&mut host.native_codecs);
    host.dependencies[0].plugin_id = owner.plugin_id.clone();
    let packages = vec![host.clone(), owner.clone()];
    assert_eq!(open_target_codec_package(&packages, &packages[0], &target).expect("hosted target binds the owner codec").plugin_id, owner.plugin_id);
    let refusal = |packages: Vec<TrustedBundlePackageV1>| open_target_codec_package(&packages, &packages[0], &target).expect_err("refused").to_string();
    assert!(refusal(vec![host.clone()]).contains("binds no exact native codec"));
    let mut stranger = host.clone();
    stranger.dependencies.clear();
    assert!(refusal(vec![stranger, owner.clone()]).contains("binds no exact native codec"));
    let mut bare = owner.clone();
    bare.native_codecs.clear();
    assert!(refusal(vec![host.clone(), bare]).contains("binds no exact native codec"));
    let mut twin = owner.clone();
    twin.plugin_id = format!("{}-twin", owner.plugin_id);
    let mut both = host.clone();
    let mut twin_dependency = both.dependencies[0].clone();
    twin_dependency.plugin_id = twin.plugin_id.clone();
    both.dependencies.push(twin_dependency);
    assert!(refusal(vec![both, owner.clone(), twin]).contains("more than one declared dependency"));
    let mut own = host.clone();
    own.native_codecs = owner.native_codecs.clone();
    assert_eq!(open_target_codec_package(&[own.clone()], &own, &target).expect("own codec").plugin_id, own.plugin_id);
}

/// 🌱️ LAW: creation is owner-preferred — the owners' editors decide and a host's editor of the same kind never makes it
/// ambiguous (cad + demonstrator's embedded cad editor → cad, even against a host's more general editor); a kind only hosts
/// create falls to its hosts (a stdio family); within the deciding tier the most general dialect creates (stdio's json
/// `rfc8259/*` over `i-json`, the image family's jpg `jfif-1.01/*` over `baseline`); two owners, two hosts or two standards
/// alone stay ambiguous; nothing creates nothing.
#[test]
fn creation_prefers_the_owners_editor_over_a_hosts() {
    let pick = |owned: &[(&'static str, &str)], hosted: &[(&'static str, &str)]| {
        let tier = |entries: &[(&'static str, &str)]| entries.iter().map(|(name, coordinate)| (*name, semio_framework::ArtifactDialect::parse_coordinate(coordinate).expect("canonical coordinate"))).collect::<Vec<_>>();
        owner_preferred_creation(tier(owned), tier(hosted), |(_, dialect)| dialect).one().map(|(name, _)| name)
    };
    assert_eq!(pick(&[("cad", "s.cad.cad@1/*")], &[("demonstrator", "s.cad.cad@1/*")]), Some("cad"));
    assert_eq!(pick(&[("cad", "s.cad.cad@1/strict")], &[("demonstrator", "s.cad.cad@1/*")]), Some("cad"));
    assert_eq!(pick(&[], &[("stdio-pdf", "s.stdio.pdf@1.7/*")]), Some("stdio-pdf"));
    assert_eq!(pick(&[("stdio i-json", "s.stdio.json@rfc8259/i-json"), ("stdio json", "s.stdio.json@rfc8259/*")], &[]), Some("stdio json"));
    assert_eq!(pick(&[], &[("stdio-image baseline", "s.stdio.jpg@jfif-1.01/baseline"), ("stdio-image jfif", "s.stdio.jpg@jfif-1.01/*")]), Some("stdio-image jfif"));
    assert_eq!(pick(&[("cad", "s.cad.cad@1/*"), ("cad-twin", "s.cad.cad@1/*")], &[("demonstrator", "s.cad.cad@1/*")]), None);
    assert_eq!(pick(&[], &[("demonstrator", "s.cad.cad@1/*"), ("bundle", "s.cad.cad@1/*")]), None);
    assert_eq!(pick(&[], &[("stdio-cad ac1018", "s.stdio.dwg@ac1018/*"), ("stdio-cad ac1024", "s.stdio.dwg@ac1024/*")]), None);
    assert_eq!(pick(&[], &[]), None);
}

/// 🌳️ LAW: the hub's most-general-dialect rule answers every case of the ONE shared fixture — the rule the guest's
/// `artifact_codec_owner` answers `codec.*` by — for the candidates as listed and reversed (order never decides).
#[test]
fn the_most_general_dialect_rule_answers_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🧫️fixtures/🌳️most-general-dialect/🔣️.json")).expect("shared most-general-dialect fixture");
    let cases = fixture["cases"].as_array().expect("fixture cases");
    assert!(cases.len() >= 10, "the shared fixture keeps its cases");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let candidates = case["candidates"].as_array().expect("candidates").iter().enumerate().map(|(index, coordinate)| (index, semio_framework::ArtifactDialect::parse_coordinate(coordinate.as_str().expect("coordinate")).expect("canonical coordinate"))).collect::<Vec<_>>();
        let expected = match &case["mostGeneral"] {
            serde_json::Value::Number(index) => MostGeneralDialect::One(usize::try_from(index.as_u64().expect("index")).expect("index")),
            serde_json::Value::String(answer) if answer == "ambiguous" => MostGeneralDialect::Ambiguous,
            serde_json::Value::String(answer) if answer == "none" => MostGeneralDialect::Empty,
            other => panic!("{name}: unknown answer {other}"),
        };
        let answer = |candidates: Vec<(usize, semio_framework::ArtifactDialect)>| match most_general_dialect(candidates, |(_, dialect)| dialect) {
            MostGeneralDialect::One((index, _)) => MostGeneralDialect::One(index),
            MostGeneralDialect::Ambiguous => MostGeneralDialect::Ambiguous,
            MostGeneralDialect::Empty => MostGeneralDialect::Empty,
        };
        assert_eq!(answer(candidates.clone()), expected, "{name}");
        assert_eq!(answer(candidates.into_iter().rev().collect()), expected, "{name} (reversed)");
    }
}

/// 🏠️ LAW: a kind a package only HOSTS (a stdio family hosting `stdio`'s kinds) is created by the host's MOST GENERAL editor
/// and executed under the HOST's own identity — the owner's linked native codec when the hub links one, the HOST's component
/// for genesis and every guest call (the family ships the app; the owner's component need not) — and its document opens
/// without a requested surface in that same whole-standard editor, while a requested subset surface still opens exactly.
/// Unlinked, the hosted row is a verification row of the HOST's component. Live defect (ticket 26/09/23 H14, 2026-09-29):
/// every multi-subset kind stayed uncreatable, and a hosted creation resolved no codec for the host's identity.
#[tokio::test]
async fn a_hosted_multi_subset_kind_is_created_and_executed_by_its_hosts_most_general_editor() {
    for linked in [true, false] {
    let mut fixture = prepared_fixture();
    let schema = fixture.schema.clone();
    let codecs = fixture.bundle["packages"][0]["nativeCodecs"].take();
    fixture.bundle["packages"][0]["nativeCodecs"] = serde_json::json!([]);
    fixture.bundle["packages"][1]["nativeCodecs"] = codecs;
    let mut strict = fixture.bundle["packages"][0]["openTargets"][0].clone();
    strict["surfaceId"] = "s.fixture.document@1/strict#editor".into();
    strict["appId"] = "s.fixture.document@1/strict#editor".into();
    strict["parentDialect"]["subset"] = "strict".into();
    fixture.bundle["packages"][0]["openTargets"].as_array_mut().expect("host targets").insert(0, strict.clone());
    let mut selected = fixture.bundle["profiles"][0]["openTargets"][0].clone();
    selected["target"] = strict;
    fixture.bundle["profiles"][0]["openTargets"].as_array_mut().expect("profile targets").insert(0, selected);
    fixture.rewrite_descriptor(1, Some(&schema), None);
    fixture.replace_descriptor(0, hosting_descriptor_bytes(&schema));
    let host = |plugin_id: &str| plugin_id == "fixture-editor";
    let descriptor = DocumentDescriptor {
        space_id: "space".into(),
        document_id: "document".into(),
        artifact_kind: "s.fixture.document".into(),
        artifact_schema: schema.clone(),
        owner: directory::os_directory::DocumentOwner { plugin_id: "fixture-editor".into(), package_id: "semio:fixture-editor".into(), version: "1.2.3".into(), package_hash: component_sha256.clone() },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: "33".repeat(32),
    };
        let bindings = if linked { vec![NativeCodecBinding::new("fixture-base", "semio:fixture-base", "s.fixture.document", None, fixture_codec(&schema, [0x11; 32]))] } else { Vec::new() };
        let catalog = load_fixture(&fixture, &bindings, &TestControl::new().context()).await.expect("hosting catalog loads");
        let selection = catalog.artifact_creation_selection("s.fixture.document").expect("the hosted multi-subset kind is creatable");
        assert!(host(&selection.package.plugin_id), "the host creates: {:?}", selection.package);
        assert_eq!(selection.surface.surface_id, "s.fixture.document@1/*#editor", "the most general editor creates");
        let identity = TrustedArtifactIdentity {
            plugin_id: selection.package.plugin_id.clone(),
            package_id: selection.package.package_id.clone(),
            version: selection.package.version.clone(),
            package_hash: selection.package.component_sha256.clone(),
            artifact_kind: selection.artifact.kind.clone(),
            artifact_schema: selection.artifact.schema.clone(),
            pack_schema_hash: selection.artifact.pack_schema_hash.clone(),
        };
        assert_eq!(identity, TrustedArtifactIdentity::from_descriptor(&descriptor), "a created document names its host as owner");
        let codec = catalog.resolve(&identity).await.expect("the host's identity resolves a codec");
        assert_eq!(codec.guest.component.package.package.0, "semio:fixture-editor", "genesis and guest calls go to the host's component");
        assert_eq!(codec.codec.is_some(), linked, "the owner's linked native codec executes the hosted identity");
        let declarations=directory::io::artifact_catalog_bindings().unwrap().into_iter().filter(|row|row.artifact.schema==schema).collect::<Vec<_>>();
        assert_eq!(declarations.len(),catalog.open_targets.len()+1);
        for declaration in &declarations{assert_eq!(declaration.owner.plugin_id,"fixture-base");assert_eq!(declaration.owner.package_id,"semio:fixture-base");assert_eq!(matches!(declaration.capability,ArtifactCatalogCapability::Linked{..}),linked);if let ArtifactCatalogCapability::Linked{factory,..}=&declaration.capability{assert!(factory.is_none());}if let Some(target)=&declaration.target{assert!(host(&declaration.contributor.plugin_id));let original=catalog.open_targets.iter().find(|selection|selection.package==declaration.contributor&&selection.artifact==declaration.artifact&&selection.surface==target.surface&&selection.parent_dialect==target.parent_dialect).expect("exact immutable selected catalog target");assert_eq!(original.grant,target.grant);assert_eq!(original.browser_actor,target.browser_actor);}}
        assert_eq!(codec.owner_declaration.owner.plugin_id,"fixture-base");assert_eq!(codec.owner_declaration.contributor.plugin_id,"fixture-base");assert_eq!(serde_json::to_value(codec.owner_declaration.owner.plugin_id.as_str()).unwrap(),serde_json::json!("fixture-base"));
        let owner = TrustedArtifactIdentity { plugin_id: "fixture-base".into(), package_id: "semio:fixture-base".into(), version: "1.0.0".into(), ..identity.clone() };
        assert_eq!(catalog.resolve(&owner).await.expect("the owner's row stays its own").guest.component.package.package.0, "semio:fixture-base");
        assert_eq!(catalog.resolve_document_open(&descriptor, None, true).expect("default editor").surface.surface_id, "s.fixture.document@1/*#editor");
        assert_eq!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/strict#editor"), true).expect("requested subset editor").surface.surface_id, "s.fixture.document@1/strict#editor");
        let progress = catalog.load_progress();
        let hosting = progress.packages.iter().find(|package| host(&package.plugin_id)).expect("host progress");
        assert_eq!((hosting.rows, hosting.rows_pinned), (1, u64::from(linked)), "the hosted identity is a row of the host, pinned only when linked");
    }
}
