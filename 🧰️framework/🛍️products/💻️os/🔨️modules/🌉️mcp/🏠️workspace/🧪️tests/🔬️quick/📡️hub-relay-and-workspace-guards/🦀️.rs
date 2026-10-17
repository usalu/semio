/// 🔬️ Hub relay and workspace guards: 9 tests covering repository root is refused as a workspace folder; an agent session beats presence once per hub connection and claims no identity; a hub documents tail and baselines reach the gue…
use semio_framework_test_fixtures::fixture_text;

/// 📡️ Hub relay and workspace guards: the committed fixture reader, the repository root refusal, agent presence, hub document tails, relay queues, terminal links, hub session edit refusal, hub catalog authority and the probe transport.

/// 🧪️ Reads a committed fixture from the package directory at run time instead of embedding it in the test binary.


#[test]
fn repository_root_is_refused_as_a_workspace_folder() {
    let root = find_repo_root().expect("repo root");
    let error = HeadlessWorkspace::reject_repository_root_workspace(&root).expect_err("repository root");
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    assert!(error.message.contains("events.semio"), "{}", error.message);
    assert!(HeadlessWorkspace::reject_repository_root_workspace(&std::env::temp_dir()).is_ok());
}

/// 🤖️ An agent session beats presence on every hub connection and on nothing else, and its beat
/// claims no admitted identity: the hub stamps label, user and principal kind for the socket.
#[test]
fn an_agent_session_beats_presence_once_per_hub_connection_and_claims_no_identity() {
    use store::sync::{ArtifactEvent, ArtifactSyncStatus, RemoteState};
    let status = |remote| ArtifactEvent::Status(ArtifactSyncStatus { persisted: true, pending_mutations: 0, remote, acknowledged_head: None });
    assert!(agent_presence_moment(&ArtifactEvent::Session { actor: "hub.v1.agent".to_string(), color: 3 }));
    assert!(agent_presence_moment(&status(RemoteState::Live { peer_count: 1 })));
    for quiet in [status(RemoteState::Connecting), status(RemoteState::Detached), status(RemoteState::Backoff { retry_in_ms: 500 }), ArtifactEvent::Presence { peers: Vec::new() }, ArtifactEvent::DocumentBackbone { message: Vec::new() }] {
        assert!(!agent_presence_moment(&quiet), "{quiet:?}");
    }
    let peer = agent_presence_peer("agent:local#sess_1");
    assert_eq!(peer.actor, "agent:local#sess_1");
    assert_eq!((peer.label, peer.user_id, peer.role, peer.color, peer.principal_kind), (None, None, None, None, None));
    assert!(peer.presence_pack.is_none() && peer.interaction.is_none() && peer.views.is_empty() && peer.tool_run.is_none());
}

/// 📥️ What a hub document's actor delivers reaches the guest: batches after the seed in order, a
/// baseline byte-identical to the seed replaces nothing, a newer baseline reseeds the guest and supersedes
/// every batch delivered before it (the fresh-session id collision's root: the guest never saw the tail).
#[test]
fn a_hub_documents_tail_and_baselines_reach_the_guest_in_order_and_a_new_baseline_reseeds_it() {
    let seed = SessionDocumentPair { pack: b"pack-0".to_vec(), spr: b"spr-0".to_vec() };
    let archive = |pack: &[u8], spr: &[u8]| HubInbound::Archive(store::encode_document_archive_bytes(&store::DocumentArchivePack { parent_pack: pack.to_vec(), parent_spr: spr.to_vec(), members: Vec::new() }).expect("archive encodes"));
    let batch = |tag: &str| HubInbound::Backbone(tag.as_bytes().to_vec());
    let tail = plan_hub_inbound(vec![batch("a"), batch("b")], &seed).expect("plans");
    assert_eq!(tail, HubInboundPlan { reseed: None, backbone: vec![b"a".to_vec(), b"b".to_vec()] });
    let same = plan_hub_inbound(vec![archive(b"pack-0", b"spr-0"), batch("t1"), batch("t2")], &seed).expect("plans");
    assert_eq!(same, HubInboundPlan { reseed: None, backbone: vec![b"t1".to_vec(), b"t2".to_vec()] }, "the first bootstrap installs the checkpoint artifact_open read");
    let moved = plan_hub_inbound(vec![batch("stale"), archive(b"pack-0", b"spr-0"), batch("old"), archive(b"pack-1", b"spr-1"), batch("t3")], &seed).expect("plans");
    assert_eq!(moved, HubInboundPlan { reseed: Some(SessionDocumentPair { pack: b"pack-1".to_vec(), spr: b"spr-1".to_vec() }), backbone: vec![b"t3".to_vec()] });
    let garbage = plan_hub_inbound(vec![HubInbound::Archive(b"not an archive".to_vec())], &seed).expect_err("an undecodable baseline is refused by name");
    assert!(garbage.message.contains("undecodable baseline"), "{}", garbage.message);
}

/// 🧵️ A guest takes its hub document's deliveries before any command except the ones that finish a
/// transaction and the inference lane; the relay hands them out oldest first, puts refused ones back in
/// front, and reports live only once the actor said so.
#[test]
fn a_relay_queues_deliveries_in_order_and_withholds_them_while_a_transaction_finishes() {
    use store::sync::{ArtifactSyncStatus, RemoteState};
    assert!(carries_hub_inbound(&[AppCommand::ReadArtifact]));
    assert!(!carries_hub_inbound(&[AppCommand::TransactionCommit { txn_id: "t".into() }]));
    assert!(!carries_hub_inbound(&[AppCommand::TransactionRollback { txn_id: "t".into() }]));
    let relay = HubRelay::default();
    relay.deliver(HubInbound::Backbone(b"1".to_vec()));
    relay.deliver(HubInbound::Backbone(b"2".to_vec()));
    let taken = relay.take_inbound();
    assert_eq!(taken, vec![HubInbound::Backbone(b"1".to_vec()), HubInbound::Backbone(b"2".to_vec())]);
    relay.deliver(HubInbound::Backbone(b"3".to_vec()));
    relay.restore_inbound(vec![HubInbound::Backbone(b"2".to_vec())]);
    assert_eq!(relay.take_inbound(), vec![HubInbound::Backbone(b"2".to_vec()), HubInbound::Backbone(b"3".to_vec())]);
    assert!(!relay.await_live(0));
    relay.record(Some(ArtifactSyncStatus { persisted: true, pending_mutations: 0, remote: RemoteState::Live { peer_count: 1 }, acknowledged_head: None }), None);
    assert!(relay.await_live(0));
}

/// 🚫️ Once the actor reports its link terminal (the hub withdrew access), a commit waiting for the hub's
/// acknowledgement learns at once that the hub will never take the edit, and says why.
#[test]
fn a_terminal_link_answers_a_waiting_commit_at_once_with_the_refusal() {
    let relay = HubRelay::default();
    let since = relay.version();
    assert!(!document_link_terminal_code("reconnecting"));
    assert!(document_link_terminal_code(store::sync::DocumentLinkStatus::LinkExpired.code()));
    relay.record_terminal(store::sync::DocumentLinkStatus::AccessRevoked.code(), "access was withdrawn");
    let started = std::time::Instant::now();
    let outcome = relay.await_acknowledged(since, 10_000);
    assert!(started.elapsed() < std::time::Duration::from_secs(1), "no wait once the link is terminal");
    assert_eq!(outcome.refused.as_deref(), Some(store::sync::DocumentLinkStatus::AccessRevoked.code()));
    assert!(!outcome.acknowledged && outcome.detail.contains("access was withdrawn"), "{}", outcome.detail);
    assert_eq!(relay.terminal().as_deref(), Some("access-revoked"));
}

/// 🚦️ A hub session's edit is refused before its guest runs unless an author holds a live hub document of the verb's
/// plugin: a terminal link names its code, a spectator (a `read` delegation) is read-only, an authority that is not
/// ready refuses retryably, and no bound document — or one without a document actor — is unbound, never a local write
/// on the plugin's genesis document (measured 2026-09-28 by H13: a read agent's addBlock answered SUCCEEDED on
/// `plugin:note`, hub head 0).
#[test]
fn a_hub_session_edit_is_refused_before_its_guest_runs_unless_an_author_holds_a_live_document() {
    use semio_framework_os_kernel::os_directory::DirectorySpaceRole;
    let binding = |blocked: &str| PluginArtifactBinding {
        schema: "s.note.note".to_string(),
        plugin_id: "note".to_string(),
        app_id: "note.editor".to_string(),
        surface_id: Some("s.note.note@1/*#editor".to_string()),
        document: Some(Arc::new(SessionDocumentPair { pack: b"pack".to_vec(), spr: b"spr".to_vec() })),
        backbone: None,
        backbone_blocked_by: Some(blocked.to_string()),
        relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        relay: Arc::default(),
        used: 1,
    };
    let document = binding("no `store::os_store::serialization::ArtifactCodec` is registered for artifact schema `s.note.note`");
    let spectator = hub_edit_refusal(Ok(DirectorySpaceRole::Spectator), "note", Some(("hub-note", &document))).expect("a spectator never edits");
    assert_eq!(spectator.code, VIEWER_READ_ONLY_FAULT_CODE);
    let unready = hub_edit_refusal(Err("hub descriptor binding is refreshing".to_string()), "note", Some(("hub-note", &document))).expect("no role, no edit");
    assert!(unready.code == "plugin.unavailable" && unready.message.contains("refreshing"), "{unready:?}");
    let nothing_open = hub_edit_refusal(Ok(DirectorySpaceRole::Author), "note", None).expect("no hub document, no edit");
    assert!(nothing_open.code == HUB_EDIT_UNBOUND_FAULT_CODE && nothing_open.message.contains("`note`") && nothing_open.message.contains("artifact_open"), "{nothing_open:?}");
    let no_actor = hub_edit_refusal(Ok(DirectorySpaceRole::Author), "note", Some(("hub-note", &document))).expect("no document actor, no edit");
    assert!(no_actor.code == HUB_EDIT_UNBOUND_FAULT_CODE && no_actor.message.contains("hub-note") && no_actor.message.contains("ArtifactCodec"), "{no_actor:?}");
    document.relay.record_terminal(store::sync::DocumentLinkStatus::AccessRevoked.code(), "access was withdrawn");
    let revoked = hub_edit_refusal(Ok(DirectorySpaceRole::Author), "note", Some(("hub-note", &document))).expect("a terminal link takes no edit");
    assert_eq!(revoked.code, store::sync::DocumentLinkStatus::AccessRevoked.code());
    let committed = commits_with_nothing_relayed(vec![AppFrame::TransactionCommitted { txn_id: "t".into(), edit_id: "e".into(), relay: None }]);
    assert!(matches!(&committed[..], [AppFrame::TransactionCommitted { relay: Some(relay), .. }] if !relay.acknowledged && relay.refused.is_none()), "{committed:?}");
}

/// 🗂️ A hub workspace's catalog follows the hub's descriptor authority: a new selection set (a package installed on the
/// hub) is compiled in, an unchanged set reuses the compiled catalog, and while the authority refreshes discovery fails
/// closed but routing keeps the last compiled catalog (measured 2026-09-28 by H13: a gateway opened on 4 packages never
/// saw the space's 5th).
#[test]
fn a_hub_workspace_catalog_follows_a_new_descriptor_authority_generation() {
    let workspace = authenticated_hub_workspace_fixture();
    let binding = Arc::clone(workspace.hub_binding.as_ref().expect("hub fixture"));
    let catalog = WorkspaceCatalog::following(Arc::clone(&binding)).expect("a ready authority compiles");
    let first = catalog.current();
    assert_eq!(distinct_plugin_ids(&first), vec!["contract-test".to_string()]);
    assert!(Arc::ptr_eq(&first, &catalog.current()), "an unchanged selection set is never recompiled");
    let mut selections = binding.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX)).expect("ready").selections.clone();
    let mut note = selections[0].clone();
    note.lease.package.plugin_id = "note".to_string();
    note.lease.package.descriptor_byte_sha256 = "note-descriptor".to_string();
    note.descriptor = note_descriptor();
    selections.push(note);
    binding.install_catalog_for_test(selections);
    let grown = catalog.current();
    assert_eq!(distinct_plugin_ids(&grown), vec!["contract-test".to_string(), "note".to_string()], "the package the hub selected since is routable");
    assert!(Arc::ptr_eq(&grown, &catalog.authoritative().expect("ready")), "discovery reads the same compiled catalog");
    binding.invalidate_stream();
    assert!(catalog.authoritative().is_err(), "a refreshing authority lists nothing");
    assert!(Arc::ptr_eq(&grown, &catalog.current()), "routing keeps the last compiled catalog while the authority refreshes");
}

#[test]
fn probe_pack_schema_hash_matches_the_cross_process_descriptor_contract() {
    let actual = pack::record::schema_hash(&probe_record_spec()).iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(actual, PROBE_PACK_SCHEMA_HASH);
}

/// 🪪️ The probe verifies no execution-target bytes, so it claims no lease and binds only its
/// requested surface id; a forgeable local target is no longer an admission input anywhere.
#[test]
fn mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority() {
    let origin = WorkspaceOrigin::Hub { base_url: "https://hub.example".into(), space_id: "space-a".into() };
    assert_eq!(origin.artifact_document_key("shared-document"), store::sync::ArtifactDocumentKey::hub("space-a", "shared-document"));
    assert_ne!(origin.artifact_document_key("shared-document"), store::sync::ArtifactDocumentKey::hub("space-b", "shared-document"));
    match origin.persistence_binding() {
        store::sync::PersistenceBinding::Hub { surface, .. } => assert_eq!(surface.as_deref(), Some(PROBE_SURFACE_ID)),
        store::sync::PersistenceBinding::Folder { .. } => panic!("hub origin must bind a hub persistence binding"),
    }
    assert!(!fixture_text("../../🏠️workspace/🦀️.rs").contains("probe_document_socket_surface"));
}
