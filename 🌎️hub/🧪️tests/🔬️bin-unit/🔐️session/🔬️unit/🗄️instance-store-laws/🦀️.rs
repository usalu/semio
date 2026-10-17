/// 🗄️ Laws of instance stores: sessions across restart, principal revocation, the saga drain and the wire frontier names.

//#region 🗄️InstanceStores
/// ⚖️ LAW: the session hub records in the server-product instance is still there after the process
/// that wrote it is gone. This is what makes `HubInstance`'s stores a durable role rather than a
/// per-process cache, and it is the property the auth move depends on.
#[tokio::test]
async fn instance_sessions_survive_a_hub_restart() {
    let dir = tempdir("instance-restart");
    let record = SessionRecord {
        id: SessionId("session-durable".to_string()),
        principal: Principal::User { id: "user-ada".to_string() },
        device: Some(server::contract::DeviceId("device-1".to_string())),
        issued_at_millis: 1_700_000_000_000,
    };

    let first = instance_state(&dir).await.expect("open hub instance stores");
    first.sessions.lock().await.create(record.clone()).await.expect("record the session");
    assert!(first.sessions.lock().await.get(&SessionId("session-durable".to_string())).await.is_some());
    drop(first);

    let second = instance_state(&dir).await.expect("reopen hub instance stores");
    let reopened = second.sessions.lock().await.get(&SessionId("session-durable".to_string())).await.expect("the session survived the restart");
    assert_eq!(reopened, record);

    second.sessions.lock().await.delete(&SessionId("session-durable".to_string())).await.expect("forget the session");
    drop(second);
    let third = instance_state(&dir).await.expect("reopen after the deletion");
    assert!(third.sessions.lock().await.get(&SessionId("session-durable".to_string())).await.is_none(), "a deleted session leaves no replayable trace");
}

/// ⚖️ LAW: signing out everywhere on the directory clears the instance's live set for that principal
/// too, so the two halves of one session fact can never disagree after a credential change.
#[tokio::test]
async fn revoking_a_principal_clears_every_instance_session_it_holds() {
    let dir = tempdir("instance-revoke");
    let instance = instance_state(&dir).await.expect("open hub instance stores");
    for index in 0..3 {
        let record = SessionRecord {
            id: SessionId(format!("session-{index}")),
            principal: Principal::User { id: "user-ada".to_string() },
            device: None,
            issued_at_millis: 1_700_000_000_000 + index,
        };
        instance.sessions.lock().await.create(record).await.expect("record the session");
    }
    let other = SessionRecord { id: SessionId("session-other".to_string()), principal: Principal::User { id: "user-grace".to_string() }, device: None, issued_at_millis: 1_700_000_000_009 };
    instance.sessions.lock().await.create(other).await.expect("record the other principal's session");

    assert_eq!(instance.sessions.lock().await.revoke_principal(&Principal::User { id: "user-ada".to_string() }).await.expect("revoke"), 3);
    assert!(instance.sessions.lock().await.get(&SessionId("session-0".to_string())).await.is_none());
    assert!(instance.sessions.lock().await.get(&SessionId("session-other".to_string())).await.is_some(), "another principal's session is untouched");
}

/// ⚖️ LAW: the saga drain is silent when it moves nothing. A cadence that reported every empty tick
/// would emit two records a second for ever and bury the ones that matter.
#[tokio::test]
async fn a_drain_over_an_empty_outbox_moves_nothing_and_says_nothing() {
    let dir = tempdir("saga-drain");
    let instance = instance_state(&dir).await.expect("open hub instance stores");
    let (tracer, sink) = Tracer::capturing(TraceLevel::Debug);
    assert_eq!(drain_instance_sagas(&instance, &tracer).await, 0);
    assert_eq!(drain_instance_sagas(&instance, &tracer).await, 0);
    assert!(sink.records_for("server.saga.drain").is_empty(), "an empty drain is not news: {:?}", sink.records());
}

/// ⚖️ LAW: the supervisor stops on demand and takes one last pass on the way out — the requests that
/// committed after the final tick are exactly the ones an unfinished drain would strand.
#[tokio::test]
async fn the_saga_drain_supervisor_stops_and_drains_once_more_on_shutdown() {
    let dir = tempdir("saga-drain-shutdown");
    let instance = instance_state(&dir).await.expect("open hub instance stores");
    let (tracer, sink) = Tracer::capturing(TraceLevel::Debug);
    let supervisor = SagaDrainSupervisor::start(instance, tracer, std::time::Duration::from_millis(5));
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    supervisor.shutdown().await;
    assert!(sink.records_for("server.saga.drain").is_empty(), "nothing was queued, so nothing is reported");
}
//#endregion 🗄️InstanceStores

/// 🧭️ Every frontier the hub puts on the wire names the DOCUMENT the client opened, never
/// `db_artifact_id`'s internal `v1:<len>:<len>:<space><document>` key.
///
/// The db engine keys documents by that composite — the db and fanout catalogs are flat and a bare
/// document id is not unique across spaces — and stamps it into every `Frontier` it returns, which
/// the replication wire carried through unchanged into `Welcome`, `Ack` and `Commands`. It stayed
/// invisible only because the client assigned the welcome frontier without checking it; C7 §1.4
/// measured `server_frontier.document_id = v1:36:41:01a0c314-…artifact-0954e2…` against a store
/// worker (`validateArtifactBootstrapIdentity`) that refuses exactly that pair. The law pins the
/// whole round trip: both of a `Welcome`'s bootstrap frontiers as well as its own, the two frames a
/// live session repeats it on, an untouched frame with no frontier, and the ingress direction —
/// a client frontier is re-keyed onto the db id, and one naming another document is REFUSED rather
/// than quietly rewritten into the identity the hub wanted to see.
#[test]
fn every_wire_frontier_the_hub_sends_names_the_document_not_its_internal_db_key() {
    let scope = DocumentScope::new("01a0c314-e41f-780d-a980-3adda40ca9f7", "artifact-0954e2d10d8fff9605f101b0dba34f3b");
    let db_id = db_artifact_id(&scope);
    assert_ne!(db_id.0, scope.document_id, "the internal key is deliberately not the document id");
    let internal = |ordinal: u64| RuntimeFrontierSummary { document_id: db_id.clone(), head_edit_ordinal: ordinal, head_edit_id: format!("edit:{ordinal}"), last_commit_seq: ordinal, chain_hash: [ordinal.to_le_bytes()[0]; 32] };
    let named = |frontier: &RuntimeFrontierSummary| frontier.document_id.0.clone();

    let mut welcome = ServerFrame::Welcome {
        session_id: "session".into(),
        resume_token: "resume".into(),
        server_frontier: internal(9),
        bootstrap: Bootstrap::ArtifactBootstrap(Box::new(protocol::ArtifactBootstrap {
            format_version: protocol::ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
            descriptor_hash: [1; 32],
            artifact_schema: "s.gis.gismap@1/*".into(),
            artifact_kind: "gis.map".into(),
            pack_schema_hash: [2; 32],
            baseline_frontier: internal(0),
            pack_hash: [3; 32],
            spr_hash: [4; 32],
            pack_length: 81_038,
            spr_length: 237,
            chunk_count: 1,
            aggregate_hash: [5; 32],
            required_tail_frontier: internal(9),
            inline: None,
        })),
    };
    project_server_frame(&mut welcome, &scope.document_id);
    let ServerFrame::Welcome { server_frontier, bootstrap, .. } = &welcome else { panic!("welcome") };
    assert_eq!(named(server_frontier), scope.document_id);
    let Bootstrap::ArtifactBootstrap(artifact) = bootstrap else { panic!("artifact bootstrap") };
    assert_eq!(named(&artifact.baseline_frontier), scope.document_id);
    assert_eq!(named(&artifact.required_tail_frontier), scope.document_id);
    assert_eq!(server_frontier.head_edit_ordinal, 9, "only the identity is projected; the position is untouched");
    assert_eq!(server_frontier.chain_hash, [9u8; 32]);

    let mut ack = ServerFrame::Ack { batch_id: 1, stages: vec![AckStage::Received], frontier: internal(10) };
    project_server_frame(&mut ack, &scope.document_id);
    let ServerFrame::Ack { frontier, .. } = &ack else { panic!("ack") };
    assert_eq!(named(frontier), scope.document_id);

    let mut commands = ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor".into()), frontier: internal(11) };
    project_server_frame(&mut commands, &scope.document_id);
    let ServerFrame::Commands { frontier, .. } = &commands else { panic!("commands") };
    assert_eq!(named(frontier), scope.document_id);

    assert!(frame_carries_frontier(&welcome) && frame_carries_frontier(&ack) && frame_carries_frontier(&commands));
    let mut session = ServerFrame::Session { actor: "actor".into(), color: 3 };
    let untouched = session.clone();
    assert!(!frame_carries_frontier(&session));
    project_server_frame(&mut session, &scope.document_id);
    assert_eq!(session, untouched);

    let mut advertised = RuntimeFrontierSummary { document_id: WireArtifactId(scope.document_id.clone()), head_edit_ordinal: 4, head_edit_id: "edit:4".into(), last_commit_seq: 4, chain_hash: [4; 32] };
    assert!(wire_frontier_to_db(&mut advertised, &scope.document_id, &db_id));
    assert_eq!(advertised.document_id, db_id);
    assert_eq!(advertised.head_edit_ordinal, 4);

    let mut foreign = RuntimeFrontierSummary { document_id: WireArtifactId("artifact-somebody-elses".into()), head_edit_ordinal: 4, head_edit_id: "edit:4".into(), last_commit_seq: 4, chain_hash: [4; 32] };
    assert!(!wire_frontier_to_db(&mut foreign, &scope.document_id, &db_id));
    assert_eq!(foreign.document_id.0, "artifact-somebody-elses", "a refused frontier is left exactly as the client sent it");
}
