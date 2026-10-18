    // 🧰️ Wait helpers and folder external edit: the demo envelope, the bounded waits for predicates and events and the folder external edit that delivers remote operations.

//#region 🧪️Actor
    async fn demo_envelope(document_id: &str) -> semio_framework_os_store_space_schema::ArtifactEnvelope<DemoSnapshot, DemoMutation> {
        create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)
    }

    async fn wait_until<Fut>(label: &str, mut predicate: impl FnMut() -> Fut)
    where
        Fut: std::future::Future<Output = bool>,
    {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        while !predicate().await {
            if tokio::time::Instant::now() >= deadline {
                panic!("{label} not satisfied before 5s deadline");
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn wait_until_value<T, Fut>(label: &str, mut predicate: impl FnMut() -> Fut) -> T
    where
        Fut: std::future::Future<Output = Option<T>>,
    {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(value) = predicate().await {
                return value;
            }
            if tokio::time::Instant::now() >= deadline {
                panic!("{label} not satisfied before 5s deadline");
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn wait_for_event(events: &mut broadcast::Receiver<ArtifactEvent>, predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        wait_for_named_event("event", events, predicate).await
    }

    async fn wait_for_named_event(label: &str, events: &mut broadcast::Receiver<ArtifactEvent>, mut predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, events.recv()).await {
                Ok(Ok(event)) => {
                    if predicate(&event) {
                        return event;
                    }
                }
                Ok(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                other => panic!("no matching {label} before deadline: {other:?}"),
            }
        }
    }

    async fn wait_for_mock_hub_event(label: &str, hub: &MockHub, events: &mut broadcast::Receiver<ArtifactEvent>, mut predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        let mut observed = Vec::new();
        loop {
            match tokio::time::timeout_at(deadline, events.recv()).await {
                Ok(Ok(event)) => {
                    if predicate(&event) {
                        return event;
                    }
                    observed.push(format!("{event:?}"));
                }
                Ok(Err(broadcast::error::RecvError::Lagged(count))) => observed.push(format!("lagged:{count}")),
                other => panic!("no matching {label} before deadline: {other:?}; actor-events={observed:?}; mock-hub={:?}", hub.trace()),
            }
        }
    }

    // 🔬️ External folder edit → RemoteMutations event + the store timeline grows on tick().
    /// 📁️ A local apply establishes a persisted edit on disk.
    ///
    /// Wait until the actor has persisted the local edit to the folder db as real pack+spr bytes.
    ///
    /// Out-of-band: append a foreign edit directly to the spr bytes (real binary op
    /// payloads, no codec, no JSON) before writing pack+spr back.
    ///
    /// Deterministically poke the actor to re-read (notify also wired, but timing-independent here).
    ///
    /// The store ingests the pushed operation on tick(); the timeline grows and snapshot updates.
    #[tokio::test]
    async fn folder_external_edit_delivers_remote_operations() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        ensure_demo_codec_registered().await;
        let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
        let host = ArtifactHost::new(test_pool());
        let channels = host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "doc-a".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() }).await;
        let mut events = host.subscribe("doc-a").await;
        let mut store = semio_framework_os_store_test_support::plain_test_store(demo_envelope("doc-a").await).await;
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], transaction: None }, &mut identity).await.expect("apply");
        channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        let mut archive = wait_until_value("persisted edit on disk", || async {
            let bytes = storage.read_archive("doc-a").await.expect("read archive")?;
            let archive = semio_framework_os_spr::decode_document_archive_bytes(&bytes).await.ok()?;
            if spr_op_ids(&archive.parent_spr).await.ok()?.is_empty() {
                None
            } else {
                Some(archive)
            }
        })
        .await;

        let external_edit = semio_framework_os_spr::HistoryEdit { line: None,
            id: "external-1".into(),
            actor: Some("peer".into()),
            started_at: "0".into(),
            finished_at: None, verb: None,
            ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
            inverse: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 1 }.encode_op().expect("encode")) }],
            meta: None, lane: None,
        };
        archive.parent_spr = semio_framework_os_store_text_io::text_format::append_history_events_to_spr(&archive.parent_spr, &[external_edit], &[]).await.expect("append external edit");
        let bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode externally changed archive");
        storage.write_archive("doc-a", "demo/v1", &bytes).await.expect("out-of-band archive write");

        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");

        let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
        match event {
            ArtifactEvent::RemoteMutations { envelopes } => {
                assert_eq!(envelopes.len(), 1);
                assert_eq!(envelopes[0].mutation_id.0, "external-1#0", "single-op edit -> mutation_id is edit.id#0 (crate::os_spr::mutation_envelope_from_edit's ordinal-suffix convention)");
            }
            other => panic!("expected RemoteMutations, got {other:?}"),
        }

        store.tick().await.expect("tick");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "external edit joined the timeline");
        assert_eq!(store.snapshot().expect("snapshot").n, 42);
        host.close("doc-a");
        semio_framework_os_store_test_support::close_plain_test_store(&mut store);
    }
//#endregion 🧪️Actor
