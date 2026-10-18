/// 🛰️ Startup progress, check in and payload laws of the hub boot: the synthetic directory event source, the launcher level catalog progress, the check in process fixture with its gis ledger edit and the language neutral mutation payload.

struct SyntheticDirectoryEventSource {
    head: u64,
    requests: std::sync::Mutex<Vec<(u64, usize)>>,
}

impl DirectoryEventPageSource for SyntheticDirectoryEventSource {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError> {
        Ok(self.head)
    }

    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError> {
        self.requests.lock().expect("request lock").push((since, limit));
        let limit = u64::try_from(limit).map_err(|error| DirectoryError::Backend(error.to_string()))?;
        let end = since.saturating_add(limit).min(self.head);
        Ok(((since + 1)..=end)
            .map(|seq| DirectoryEvent {
                seq,
                id: format!("event-{seq}"),
                hlc: os_directory::Hlc { physical_ms: i64::try_from(seq).expect("bounded synthetic sequence"), logical: 0 },
                actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:paged-read-law".into() },
                space_id: Some("default".into()),
                user_id: None,
                body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: "default".into(), name: format!("paged-{seq}") },
                recorded_at_ms: 0,
            })
            .collect())
    }
}

/// 📡️ A catalog load that is still working says so at the level a launcher runs the hub at: an
/// in-flight unit (a guest codec's fuel progress) is an `info` record, rate-limited per second, and
/// the terminal unit an `info` `ok`. A launcher's readiness waiter reads exactly this output, so
/// a load whose progress only reached `debug` looked wedged and was killed while it still worked.
#[test]
fn startup_catalog_progress_is_visible_at_the_launcher_level() {
    let (tracer, sink) = Tracer::capturing(TraceLevel::Info);
    let control = StartupCatalogControl::new(tracer, StartupCancellationV1::default(), StartupProgressCellV1::default());
    control.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: 25_000_000, total_units: 4_000_000_000 });
    control.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: 50_000_000, total_units: 4_000_000_000 });
    control.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: 4_000_000_000, total_units: 4_000_000_000 });
    let records = sink.records_for("server.catalog.publication");
    assert_eq!(records.iter().map(|record| (record.level, record.outcome)).collect::<Vec<_>>(), vec![(TraceLevel::Info, TraceOutcome::Started), (TraceLevel::Info, TraceOutcome::Ok)], "one in-flight record per gap, then the terminal one: {records:?}");
    assert!(records[0].detail.as_deref().is_some_and(|detail| detail.contains("GuestCodecExecuting 25000000/4000000000")), "{records:?}");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn check_in_process_fixture_emits_verified_gis_ledger_and_catalog() {
    use semio_hub_trusted_catalog_fixtures as trusted_catalog_fixture;

    let artifact_root = semio_hub_testing::test_artifact_root();
    let destination = artifact_root.join("check-in-process-fixture");
    let stage = artifact_root.join(format!(".check-in-process-fixture-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&stage);
    let _ = std::fs::remove_dir_all(&destination);
    std::fs::create_dir_all(&stage).expect("create process fixture stage");

    let profile = trusted_catalog_fixture::verified_gis_map_integration_profile(&trusted_catalog_fixture::unique_profile_root("checkpoint-process")).await.expect("verified GIS Map process profile");
    let selection = profile.binding().selection();
    assert_eq!((selection.artifact.kind.as_str(), selection.artifact.schema.as_str()), ("s.gis.gismap", "gis.map"));
    let source = profile.bundle_path().parent().expect("profile bundle parent");
    let bundle_bytes = std::fs::read(profile.bundle_path()).expect("read verified profile bundle");
    let bundle: serde_json::Value = serde_json::from_slice(&bundle_bytes).expect("decode verified profile bundle");
    let generation_id = bundle["profiles"][0]["generationId"].as_str().expect("verified generation id");
    let generation = stage.join("data/trusted-catalog/generations").join(generation_id);
    std::fs::create_dir_all(&generation).expect("create trusted generation");
    for name in ["component.wasm", "descriptor.semio", "closed-actor.mjs", "plugin-module-gis.json", "stdio-component.wasm", "stdio-descriptor.semio", "plugin-module-stdio.json", "trusted-catalog.json"] {
        std::fs::copy(source.join(name), generation.join(name)).unwrap_or_else(|error| panic!("copy verified {name}: {error}"));
    }
    for directory in ["plugin-modules"] {
        std::fs::create_dir_all(generation.join(directory)).expect("create verified plugin module store");
        for entry in std::fs::read_dir(source.join(directory)).expect("verified plugin module store") {
            let entry = entry.expect("verified plugin module file");
            std::fs::copy(entry.path(), generation.join(directory).join(entry.file_name())).expect("copy verified plugin module file");
        }
    }
    let bundle_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&bundle_bytes));
    std::fs::create_dir_all(stage.join("data/trusted-catalog")).expect("create trusted current owner");
    std::fs::write(
        stage.join("data/trusted-catalog/current.json"),
        format!(
            r#"{{"profileId":"{}","generationId":"{generation_id}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#,
            trusted_catalog_fixture::GIS_MAP_INTEGRATION_PROFILE_ID
        ),
    )
    .expect("write trusted current pointer");

    let envelopes = check_in_fixture_gis_ledger_edit(&selection.artifact.schema).await;
    let hex = |bytes: &[u8]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let ledger: Vec<serde_json::Value> = envelopes
        .iter()
        .map(|envelope| {
            serde_json::json!({
                "mutationId": envelope.mutation_id.0,
                "dependencies": envelope.dependencies.iter().map(|dependency| dependency.0.clone()).collect::<Vec<_>>(),
                "diffSchema": envelope.diff.schema.0,
                "diff": hex(&envelope.diff.payload),
                "inverseSchema": envelope.inverse.schema.0,
                "inverse": hex(&envelope.inverse.payload),
            })
        })
        .collect();
    let ledger_bytes = serde_json::to_vec_pretty(&ledger).expect("encode process ledger");
    let payload = stage.join("payload");
    std::fs::create_dir_all(&payload).expect("create process payload owner");
    std::fs::write(payload.join("envelopes.json"), &ledger_bytes).expect("write process ledger");
    let fixture = serde_json::json!({
        "schema": "semio.hub.check-in-process-seed/v1",
        "profileId": trusted_catalog_fixture::GIS_MAP_INTEGRATION_PROFILE_ID,
        "generationId": generation_id,
        "documentId": "mcp-cold-gis-map",
        "package": {
            "pluginId": selection.package.plugin_id.as_str(),
            "packageId": selection.package.package_id.as_str(),
            "version": selection.package.version.as_str(),
            "componentSha256": selection.package.component_sha256.as_str()
        },
        "artifact": {
            "kind": selection.artifact.kind.as_str(),
            "schema": selection.artifact.schema.as_str(),
            "packSchemaHash": selection.artifact.pack_schema_hash.as_str()
        },
        "surfaceId": selection.surface.surface_id.as_str(),
        "payload": { "envelopes": { "path": "payload/envelopes.json", "count": envelopes.len(), "sha256": os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&ledger_bytes)) } }
    });
    std::fs::write(stage.join("fixture.json"), serde_json::to_vec_pretty(&fixture).expect("encode process fixture")).expect("write process fixture receipt");
    std::fs::rename(&stage, &destination).expect("publish process fixture atomically");
    let dependency_fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).expect("compiled dependency fixture");
    let expected_files: std::collections::BTreeSet<_> = ["publicationFiles", "publicationDirectories"].iter().flat_map(|key| dependency_fixture[*key].as_array().unwrap().iter()).map(|name| name.as_str().unwrap().to_owned()).collect();
    let retained_generation = destination.join("data/trusted-catalog/generations").join(generation_id);
    let actual_files: std::collections::BTreeSet<_> = std::fs::read_dir(&retained_generation).unwrap().map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect();
    assert_eq!(actual_files, expected_files);
    let control = StartupCatalogControl::silent();
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let relocated = TrustedCatalogLoader::load_current(&destination.join("data"), &NativeCodecProviderSetV1::linked(), &context).await.expect("relocated process catalog dependency closure").expect("relocated current");
    assert_eq!(relocated.codec_count(), 31);
    assert_eq!(relocated.packages().len(), 2);
    assert_eq!(relocated.generation_id(), generation_id);
    assert!(destination.join("data/trusted-catalog/current.json").is_file());
    assert_eq!(std::fs::read(destination.join("payload/envelopes.json")).expect("read retained GIS ledger"), ledger_bytes);
}

/// 🗺️ The ledger a real GIS Map editor emits for one edit that places `gis_map_test_snapshot()`'s
/// position and route onto an empty map: its store's own event log, exactly what a client sends.
#[cfg(feature = "integration-fixtures")]
async fn check_in_fixture_gis_ledger_edit(schema: &str) -> Vec<MutationEnvelope> {
    use directory::os_store::{store_schema::ArtifactCommand, ArtifactStore};
    use semio_s_artifact_gis_gismap::mutations::{create_position::CreatePosition, create_route::CreateRoute};
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot};
    let target = gis_map_test_snapshot();
    let mut store = ArtifactStore::<GisMapSnapshot, GisMapMutation>::new(directory::os_store::document_materialize::create_document_envelope(schema, "check-in-fixture", GisMapSnapshot::default(), None), directory::ActorId(directory::os_spr::LOCAL_ACTOR_ID.into())).await.expect("fixture GIS store");
    store.install_document_store_owners_exact(directory::os_store::document_store_owners::bounded_artifact_store_owners());
    let applied = store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![GisMapMutation::CreatePosition(CreatePosition { index: 0, item: target.positions[0].clone() }), GisMapMutation::CreateRoute(CreateRoute { index: 0, item: target.routes[0].clone() })],
            transaction: None,
        })
        .await;
    let events = store.event_log();
    while !store.close_owned_terminal_is_empty() {
        let demand = store.close_owned_demands(0).expect("the store quotes its next close turn");
        let grant = directory::os_store::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        store.close_owned_step(grant).expect("fixture GIS store closes");
    }
    assert!(store.close_owned_terminal_is_empty());
    drop(store);
    applied.expect("fixture GIS edit applies");
    events.expect("fixture GIS ledger")
}

#[test]
fn mutation_message_payload_matches_language_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
    let messages = vec![protocol::MutationMessage::warning("mutation.clamped", "height clamped").at(["node", "height"]).at_op(2), protocol::MutationMessage::info("mutation.cascade", "dependent value updated")];
    let encoded = encode_messages(&messages);
    let parsed: serde_json::Value = serde_json::from_slice(&encoded).expect("first-party message bytes are valid JSON");
    assert_eq!(parsed, fixture["mutationMessages"]);
    assert_eq!(<Vec<protocol::MutationMessage> as FromValue>::from_value(DslValue::from(parsed)).expect("first-party message decode"), messages);
}
