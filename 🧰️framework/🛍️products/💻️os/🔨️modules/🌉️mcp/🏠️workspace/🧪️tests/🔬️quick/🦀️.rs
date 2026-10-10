
use super::*;

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
    let document = binding("no `store::ArtifactCodec` is registered for artifact schema `s.note.note`");
    let spectator = hub_edit_refusal(Ok(DirectorySpaceRole::Spectator), "note", Some(("hub-note", &document))).expect("a spectator never edits");
    assert_eq!(spectator.code, crate::actions::VIEWER_READ_ONLY_FAULT_CODE);
    let unready = hub_edit_refusal(Err("hub descriptor binding is refreshing".to_string()), "note", Some(("hub-note", &document))).expect("no role, no edit");
    assert!(unready.code == "plugin.unavailable" && unready.message.contains("refreshing"), "{unready:?}");
    let nothing_open = hub_edit_refusal(Ok(DirectorySpaceRole::Author), "note", None).expect("no hub document, no edit");
    assert!(nothing_open.code == crate::actions::HUB_EDIT_UNBOUND_FAULT_CODE && nothing_open.message.contains("`note`") && nothing_open.message.contains("artifact_open"), "{nothing_open:?}");
    let no_actor = hub_edit_refusal(Ok(DirectorySpaceRole::Author), "note", Some(("hub-note", &document))).expect("no document actor, no edit");
    assert!(no_actor.code == crate::actions::HUB_EDIT_UNBOUND_FAULT_CODE && no_actor.message.contains("hub-note") && no_actor.message.contains("ArtifactCodec"), "{no_actor:?}");
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
    note.descriptor = crate::source_builders::note_descriptor();
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
    let actual = store::os_pack::schema_hash(&probe_record_spec()).iter().map(|byte| format!("{byte:02x}")).collect::<String>();
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
    assert!(!include_str!("../../🦀️.rs").contains("probe_document_socket_surface"));
}

fn empty_catalog() -> Arc<Catalog> {
    Arc::new(crate::compile(&crate::CatalogSource::default(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native).expect("empty catalog source compiles"))
}

fn authenticated_hub_workspace_fixture() -> HeadlessWorkspace {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔗️remote/🧫️fixtures/🔣️authenticated-hub-descriptor-index.json")).unwrap();
    let ready = &fixture["cases"]["memberReady"];
    let page = semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationPageV1::parse_canonical_json(ready["responses"][1]["canonicalBody"].as_str().unwrap()).unwrap();
    let semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationPageV1::Author { space, members, documents, .. } = page else { panic!("author administration page fixture") };
    let members: Vec<semio_framework_os_kernel::os_directory::MemberView> =
        members.rows.into_iter().map(|row| semio_framework_os_kernel::os_directory::MemberView { user_id: row.user_id, email: row.email, display_name: row.display_name, role: row.role }).collect();
    let documents = documents.rows;
    let view = documents[0].clone();
    let scope = DocumentScope::new("space-a", "shared-doc");
    let digest = semio_framework_os_kernel::os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&view.descriptor).unwrap();
    let document = remote::AuthorizedDocumentView { scope: scope.clone(), descriptor_digest_v1: semio_framework_os_kernel::os_directory::io::binary::artifact_hash::hex_lower(digest.as_bytes()), view };
    let snapshot = AuthorizedDescriptorSnapshot { authenticated_user_id: "user-a".to_string(), session_expires_at_ms: i64::MAX, space, membership: members[0].clone(), observed_event_seq: 8, documents: HashMap::from([(scope, document)]) };
    let binding = Arc::new(HubRemoteBinding::new("https://hub.invalid", "space-a").unwrap());
    binding.install_snapshot_for_test(snapshot);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution-target corpus json");
    let mut lease: semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&corpus["manifest"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("manifest");
    lease.scope = DocumentScope::new("space-a", "shared-doc");
    let (descriptor, descriptor_bytes) = crate::source_builders::catalog_contract_descriptor();
    lease.package.plugin_id = descriptor.manifest.plugin_id.clone();
    lease.package.package_id = descriptor.package_id.clone();
    lease.package.version = descriptor.manifest.version.clone();
    lease.package.component_sha256 = descriptor.hashes.wasm_sha256.clone();
    lease.package.descriptor_byte_sha256 = framework_hash::sha256_hex(&descriptor_bytes);
    lease.descriptor.sha256 = lease.package.descriptor_byte_sha256.clone();
    lease.descriptor.byte_length = descriptor_bytes.len() as u64;
    binding.install_catalog_for_test(vec![remote::AuthorizedPackageSelection { scope: lease.scope.clone(), descriptor_digest_v1: lease.descriptor_digest_v1.clone(), lease, descriptor }]);
    let mut workspace = HeadlessWorkspace::new(WorkspaceOrigin::Hub { base_url: "https://hub.invalid".to_string(), space_id: "space-a".to_string() }, "forged-local-principal".to_string(), vec!["admin".to_string()], empty_catalog());
    workspace.catalog = Arc::new(WorkspaceCatalog::following(Arc::clone(&binding)).expect("the fixture's authority compiles"));
    workspace.hub_binding = Some(binding);
    workspace
}

#[test]
fn authenticated_hub_workspace_resources_are_snapshot_only_scoped_and_fail_closed_when_stale() {
    let workspace = authenticated_hub_workspace_fixture();
    let resources = workspace.list_resources().unwrap();
    let uris: Vec<_> = resources.iter().map(|resource| resource.uri.as_str()).collect();
    assert_eq!(uris, vec!["semio://workspace/scopes/space-a/shared-doc/descriptor", "semio://workspace/scopes/space-a/shared-doc/checkpoint",]);
    let workspace_body = workspace.read_resource("semio://workspace").unwrap()[0].text.clone().unwrap();
    assert!(workspace_body.contains("user-a"));
    assert!(!workspace_body.contains("forged-local-principal"));
    assert!(!workspace_body.contains("secret-never-rendered"));
    let descriptor_body = workspace.read_resource("semio://workspace/scopes/space-a/shared-doc/descriptor").unwrap()[0].text.clone().unwrap();
    assert!(descriptor_body.contains("\"spaceId\":\"space-a\""));
    assert!(descriptor_body.contains("\"documentId\":\"shared-doc\""));
    let artifacts_body = workspace.read_resource("semio://workspace/artifacts").unwrap()[0].text.clone().unwrap();
    assert!(artifacts_body.contains("\"checkpointResource\":\"semio://workspace/scopes/space-a/shared-doc/checkpoint\""));
    let checkpoint_error = workspace.read_resource("semio://workspace/scopes/space-b/shared-doc/checkpoint").unwrap_err();
    assert_eq!(checkpoint_error.code, GatewayErrorCode::NotFound);
    let schema_body = workspace.read_resource("semio://artifact/shared-doc/schema").unwrap()[0].text.clone().unwrap();
    let schema: serde_json::Value = serde_json::from_str(&schema_body).unwrap();
    assert_eq!(schema["artifactId"], "shared-doc");
    assert_eq!(schema["spaceId"], "space-a");
    assert!(schema["schema"].is_string(), "{schema_body}");
    let lease:serde_json::Value=serde_json::from_str(include_str!("../../../../📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).unwrap();
    assert_eq!(schema["artifactKind"], lease["manifest"]["parentDialect"]["artifactKind"], "the document kind follows its authenticated parent dialect: {schema_body}");
    for uri in ["semio://artifact/shared-doc", "semio://artifact/shared-doc/validation"] {
        let error = workspace.read_resource(uri).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }
    workspace.hub_binding.as_ref().unwrap().invalidate_stream();
    let error = workspace.list_resources().unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn authenticated_hub_discovery_uses_retained_selection_and_never_installed_fallback() {
    let workspace = Arc::new(authenticated_hub_workspace_fixture());
    let descriptors = workspace.discovery_descriptors().expect("ready selected descriptors");
    assert_eq!(descriptors.len(), 1);
    assert_eq!(descriptors[0].manifest.plugin_id, "contract-test");
    let roster = crate::inference::declared_inferences_for_workspace(&workspace).expect("selected inference roster");
    assert_eq!(roster.len(), 1);
    assert_eq!(roster[0].owner, "contract-test");
    let principal = AgentPrincipal::from_scope_names("agent:hub-test", "hub test", &[], None);
    let server = crate::build_server_with_workspace(principal, Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), workspace.clone(), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), crate::GatewayRuntime::default());
    let listed = server.tools.list();
    let inference_list = listed.iter().find(|tool| tool.name == "inference_list").expect("inference_list tool");
    let selected = inference_list.meta.as_ref().expect("ready tools/list selection metadata");
    assert_eq!(selected["semio"]["hubSelectedPackages"][0]["package"]["pluginId"], "contract-test");
    assert_ne!(selected["semio"]["hubSelectedPackages"][0]["package"]["componentSha256"], "");
    assert_eq!(selected["semio"]["hubSelectedPackages"][0]["package"]["executionProtocol"]["appChannelVersion"], semio_framework_os_kernel::os_spr::CHANNEL_VERSION);
    let inference_result = server.tools.call("inference_list", serde_json::json!({})).expect("inference_list registered");
    assert!(!inference_result.is_error);
    assert_eq!(inference_result.structured_content.as_ref().expect("inference roster")["declared"][0]["owner"], "contract-test");
    workspace.hub_binding.as_ref().unwrap().invalidate_stream();
    let descriptor_error = workspace.discovery_descriptors().expect_err("refreshing authority must remove selected descriptors");
    assert_eq!(descriptor_error.code, GatewayErrorCode::PluginUnavailable);
    assert!(descriptor_error.retryable);
    let inference_error = crate::inference::declared_inferences_for_workspace(&workspace).expect_err("inference discovery must not fall back to installed packages");
    assert_eq!(inference_error.code, GatewayErrorCode::PluginUnavailable);
    assert!(inference_error.retryable);
    let revoked_inference_list = server.tools.list().into_iter().find(|tool| tool.name == "inference_list").expect("revoked inference_list tool");
    assert!(revoked_inference_list.meta.is_none(), "refreshing tools/list must expose no stale or installed package identity");
    let revoked_result = server.tools.call("inference_list", serde_json::json!({})).expect("revoked inference_list registered");
    assert!(revoked_result.is_error);
    let error = revoked_result.structured_content.expect("revoked discovery error");
    assert_eq!(error["code"], "PLUGIN_UNAVAILABLE");
    assert_eq!(error["retryable"], true);
}

/// 🔗️ LAW (fixture `🧫️fixtures/🔗️hub-live-links.json`, Python oracle `wp-g12/g12-live-links-oracle.py` of ticket
/// 26/09/23): an agent session keeps one live hub link per app and at most [`HUB_SESSION_LIVE_LINK_LIMIT`] in all —
/// opening a document supersedes its app's older link, relinks its own expired one, and closes the least recently used
/// past the bound; an app's session document is the hub document opened or edited last, a folder's the single one.
#[test]
fn a_hub_session_keeps_one_live_link_per_app_within_its_bound_and_edits_the_document_opened_last() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️hub-live-links.json")).expect("hub-live-links fixture parses");
    assert_eq!(fixture["limit"].as_u64(), Some(HUB_SESSION_LIVE_LINK_LIMIT as u64), "the fixture pins the session bound");
    let text = |value: &serde_json::Value| value.as_str().expect("string").to_string();
    for case in fixture["close"].as_array().expect("close cases") {
        let live = case["live"].as_array().expect("live").iter().map(|row| (text(&row[0]), text(&row[1]), text(&row[2]), row[3].as_u64().expect("used"))).collect::<Vec<_>>();
        let limit = case["limit"].as_u64().map_or(HUB_SESSION_LIVE_LINK_LIMIT, |limit| limit as usize);
        let closes = hub_links_to_close(&live, case["opening"].as_str().expect("opening"), (case["route"][0].as_str().expect("plugin"), case["route"][1].as_str().expect("app")), limit)
            .into_iter()
            .map(|(artifact_id, close)| serde_json::json!([artifact_id, match close { HubLinkClose::Relinked => "relinked", HubLinkClose::Superseded => "superseded", HubLinkClose::Bounded => "bounded" }]))
            .collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(closes), case["expect"], "{}", case["name"]);
    }
    for case in fixture["session"].as_array().expect("session cases") {
        let bound = case["bound"].as_array().expect("bound").iter().map(|row| (row[0].as_str().expect("artifact"), row[1].as_bool().expect("hub"), row[2].as_u64().expect("used"))).collect::<Vec<_>>();
        assert_eq!(route_session_artifact(bound), case["expect"].as_str(), "{}", case["name"]);
    }
    let (first, second) = (hub_link_tick(), hub_link_tick());
    assert!(second > first, "the hub-link clock is monotonic");
}

/// 🪟️ LAW: a headless agent's transaction carries document operations and owned children only — a verb's config/draft view
/// state beside a document change is omitted with a preview warning, a verb whose whole effect is view state is refused
/// `interactive-job.agent-lane-uncarried` at prepare, and a verb with no view lane passes untouched.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_headless_agent_carries_document_operations_and_leaves_view_state_in_the_shell() {
    let ops = |document: usize, config: usize, draft: usize, children: &[u8]| PreparedOps { document: vec![vec![1]; document], config: vec![vec![2]; config], draft: vec![vec![3]; draft], children: children.to_vec() };
    assert_eq!(headless_agent_ops("p.editor.rename", ops(2, 0, 0, &[])).expect("document only"), (ops(2, 0, 0, &[]), Vec::new()));
    assert_eq!(headless_agent_ops("p.editor.addGeneration", ops(1, 1, 0, &[])).expect("document beside view state"), (ops(1, 0, 0, &[]), vec![VIEW_STATE_OMITTED_WARNING.to_string()]));
    assert_eq!(headless_agent_ops("p.editor.addWidget", ops(0, 0, 1, &[9])).expect("children beside view state"), (ops(0, 0, 0, &[9]), vec![VIEW_STATE_OMITTED_WARNING.to_string()]));
    let refused = headless_agent_ops("p.editor.setShowMode", ops(0, 1, 1, &[])).expect_err("view state alone");
    assert_eq!(refused.code, crate::actions::AGENT_LANE_UNCARRIED_FAULT_CODE);
    assert!(refused.message.contains("config and draft") && refused.message.contains("p.editor.setShowMode"), "{}", refused.message);
}

/// 🧹️ LAW: the exchange an abandoned command left on an instance (its wall budget ran out, or its turn faulted and the
/// instance was discarded) is closed before the next command on that instance is admitted — never met by the next
/// command's own turn, which asserted on the seq and took the whole gateway down (live hub coverage, 7800/p33); the
/// command's own exchange is never touched.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_abandoned_commands_exchange_is_closed_before_the_next_command_on_its_instance() {
    let driver = |seq: u64| {
        let mut owners = semio_framework::kernel::CommandEnvelopeSet::try_new().expect("envelope set");
        let command = ::semio_framework_async::poll::resolve_ready(store::encode_app_command(&store::AppCommand::ReadDocument { seq })).expect("the command encodes");
        assert!(owners.try_push(semio_framework::kernel::CommandEnvelope { instance: 3, seq, command }).is_ok());
        let Ok(batch) = semio_framework::kernel::CommandBatch::try_new(seq, owners) else { panic!("command batch") };
        semio_framework::kernel::CommandBatchDriver::new(seq, batch)
    };
    let mut exchanges = PendingExchangeRegistry::<1>::new();
    let mut closes = semio_framework::kernel::CommandDriverRegistry::<1>::new();
    closes.insert_admitted(3, 5, driver(5));
    exchanges.insert_admitted(PendingExchange { instance: 3, seq: 5, response: PendingResponsePage::Empty, ingress_fault: None });
    begin_closing_abandoned_exchange(&exchanges, &mut closes, 3, 5).expect("the command's own exchange");
    assert!(closes.is_active(3, 5), "a command's own exchange is never closed");
    begin_closing_abandoned_exchange(&exchanges, &mut closes, 3, 6).expect("an abandoned exchange");
    assert!(!closes.is_active(3, 5), "the abandoned exchange is closing");
    let mut retired = false;
    for _ in 0..64 {
        let (response_complete, _) = exchanges.close_response_step(3, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
        if !response_complete {
            continue;
        }
        let (complete, _, _) = closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
        if complete {
            exchanges.remove(3).expect("the abandoned exchange");
            retired = true;
            break;
        }
    }
    assert!(retired && exchanges.can_insert(3) && closes.can_insert(3), "the next command on the instance is admitted");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pending_response_close_releases_one_grant_at_a_time() {
    let mut response = PendingResponsePage::Empty;
    response.admit_frame(&vec![7; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]);
    assert_eq!(response.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1), (false, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1));
    assert_eq!(response.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (true, 1));
    assert!(response.terminal_is_empty());
}

/// ⚖️ LAW: an answer past the declared host-answer ceiling is a typed fault, a second answer for the
/// same sequence is a typed fault, and a command that settled silently answers a stamped `Done` —
/// the reply-stamp rule the React host applies (`commandIngressNeedsReplyStampV1`), without which
/// every verb whose guest has nothing to say would hang on an answer that is never coming.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pending_response_faults_oversize_and_duplicate_and_stamps_a_silent_settlement() {
    let mut oversized = PendingResponsePage::Empty;
    oversized.admit_frame(&vec![1; semio_framework::kernel::COMMAND_MAXIMUM_BYTES + 1]);
    assert_eq!(oversized.take(9).unwrap_err().code, "channel.not-wired");
    let mut duplicate = PendingResponsePage::Empty;
    duplicate.admit_frame(&[2]);
    duplicate.admit_frame(&[3]);
    assert!(duplicate.take(10).unwrap_err().message.contains("more than one frame"));
    let mut silent = PendingResponsePage::Empty;
    silent.stamp_settled();
    assert_eq!(silent.take(11).expect("a settled command answers"), store::AppFrame::Done { in_reply_to: 11 });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn guest_fault_wire_decodes_through_the_first_party_value_codec() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    let wire = semio_framework_value::DslValue::from(&fixture["guestFault"]["wire"]);
    let fault = decode_guest_fault(&store::pack_rt::encode_wire_value(&wire));
    assert_eq!(fault.code, fixture["guestFault"]["expected"]["code"].as_str().expect("fixture fault code"));
    assert_eq!(fault.message, fixture["guestFault"]["expected"]["message"].as_str().expect("fixture fault message"));
}

/// 🪲️ Regression law for the two-frame `to_value` → `to_dsl_value` → `to_value` recursion that
/// aborted every probe-committing test with `fatal runtime error: stack overflow`: reaching an
/// assertion at all proves the cycle is gone, and the fixture pins the exact wire shape so the
/// cure cannot silently change the encoding. `serde_json` is the independent third-party oracle
/// — the first-party `ToValue` tree must equal what it serializes for the same values.
#[test]
fn probe_codec_encodes_the_fixture_shape_and_agrees_with_the_third_party_serializer() {
    use semio_framework_value::{FromValue as _, ToValue as _};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    let probe = &fixture["probeCodec"];
    let value = probe["value"].clone();
    let snapshot = ProbeSnapshot(value.clone().into());
    let diff = ProbeDiff(value.clone().into());
    let mutation = ProbeMutation::SetValue(value.clone().into());

    assert_eq!(serde_json::Value::from(snapshot.to_value()), probe["snapshotEncoding"]);
    assert_eq!(serde_json::Value::from(diff.to_value()), probe["diffEncoding"]);
    assert_eq!(serde_json::Value::from(mutation.to_value()), probe["mutationEncoding"]);
    assert_eq!(serde_json::to_value(&snapshot).expect("serde oracle"), probe["snapshotEncoding"]);
    assert_eq!(serde_json::to_value(&diff).expect("serde oracle"), probe["diffEncoding"]);
    assert_eq!(serde_json::to_value(&mutation).expect("serde oracle"), probe["mutationEncoding"]);

    assert_eq!(ProbeSnapshot::from_value(snapshot.to_value()).expect("snapshot round trip"), snapshot);
    assert_eq!(ProbeDiff::from_value(diff.to_value()).expect("diff round trip"), diff);
    assert_eq!(ProbeMutation::from_value(mutation.to_value()).expect("mutation round trip"), mutation);

    for rejected in probe["rejectedMutationEncodings"].as_array().expect("fixture rejection cases") {
        assert!(ProbeMutation::from_value(semio_framework_value::DslValue::from(rejected)).is_err(), "must reject {rejected}");
    }
}

#[test]
fn open_folder_creates_the_directory_if_missing() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let target = dir.path().join("nested").join("space");
    let workspace = HeadlessWorkspace::open_folder(target.clone(), "agent:test".to_string(), vec!["workspace.read".to_string()], empty_catalog()).expect("opens and creates");
    assert!(target.is_dir());
    assert_eq!(workspace.origin().describe(), format!("folder://{}", target.display()));
}

#[test]
fn a_fresh_folder_workspace_lists_zero_artifacts() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    assert_eq!(workspace.workspace_artifact_ids().expect("list"), Vec::<String>::new());
}

#[tokio::test]
async fn ensure_probe_artifact_seeds_a_real_revision_and_is_idempotent() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let first = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "hello" }).into()).await.expect("seed");
    assert_eq!(first.artifact_id, "probe-a");
    assert!(!first.head_edit_id.is_empty(), "a genuinely applied edit has a real edit id");
    let second = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "should not apply" }).into()).await.expect("idempotent re-open");
    assert_eq!(first.head_edit_id, second.head_edit_id, "re-calling ensure_probe_artifact never double-commits");
}

#[tokio::test]
async fn resolve_context_reports_the_open_probe_artifact_as_active() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), vec!["workspace.read".to_string()], empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-b", serde_json::json!({ "n": 1 }).into()).await.expect("seed");
    let summary = workspace.resolve_context("agent:test").expect("resolve");
    assert_eq!(summary.principal, "agent:test");
    assert_eq!(summary.active_artifact_id.as_deref(), Some("probe-b"));
    assert!(!summary.session_id.is_empty());
}

#[tokio::test]
async fn read_resource_artifact_returns_real_bytes_after_a_commit() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-c", serde_json::json!({ "n": 42 }).into()).await.expect("seed");
    let contents = workspace.read_resource("semio://artifact/probe-c").expect("read");
    let body: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().expect("text body")).expect("json body");
    assert!(body["packBytes"].as_u64().unwrap_or(0) > 0, "a real committed edit persists non-empty pack bytes: {body}");
}

#[test]
fn read_resource_on_an_unknown_artifact_is_not_found_not_fabricated() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let error = workspace.read_resource("semio://artifact/does-not-exist").expect_err("must not fabricate");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

#[test]
fn prepare_action_on_an_unknown_capability_is_not_found_not_a_panic() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let prepare_error = workspace.prepare_action("cad.editor.translateSelection", serde_json::json!({}), None).expect_err("unknown capability in an empty catalog");
    assert_eq!(prepare_error.code, GatewayErrorCode::NotFound);
}

/// 🔀️ `action_adapter()` no longer needs to resolve ANY plugin just to be BUILT — routing is
/// now per-call (`RoutingArtifactChannel`), so this reaches the real `HandleTable` and gets a
/// real, typed `NOT_FOUND` for the bogus handle — never the old blanket `PLUGIN_UNAVAILABLE`
/// every mutation call used to short-circuit with before a plugin was even looked at (the exact
/// defect `📓️w8-capability-routing.md` fixes).
#[test]
fn invoke_action_on_an_unknown_handle_is_not_found_not_a_panic() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let invoke_error = workspace.invoke_action("prep_x", None).expect_err("unknown handle");
    assert_eq!(invoke_error.code, GatewayErrorCode::NotFound);
}

fn note_and_cad_catalog() -> Arc<Catalog> {
    Arc::new(crate::compile(&crate::source_builders::note_and_cad_source(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native).expect("note+cad fixture source compiles"))
}

#[test]
fn resolve_plugin_for_capability_is_not_found_for_an_unknown_id() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog()).expect("opens");
    let error = workspace.resolve_plugin_for_capability("totally.unknown.capability").expect_err("no such capability");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

#[test]
fn resolve_plugin_for_capability_is_plugin_unavailable_for_a_gateway_owned_id() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog()).expect("opens");
    let error = workspace.resolve_plugin_for_capability("capabilities.search").expect_err("gateway-owned, not a plugin");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn resolve_plugin_for_capability_routes_note_and_cad_to_different_plugins() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog()).expect("opens");
    assert_eq!(workspace.resolve_plugin_for_capability("note.editor.setGridVisible").expect("note-owned"), "note");
    assert_eq!(workspace.resolve_plugin_for_capability("cad.editor.addObject").expect("cad-owned"), "cad");
}

/// 🗿️ A routing channel with no artifact bound to any plugin — the shape every routing test below
/// asserts against, since none of them creates an artifact.
fn unbound_plugin_artifacts() -> Arc<Mutex<HashMap<String, PluginArtifactBinding>>> {
    Arc::new(Mutex::new(HashMap::new()))
}

/// 🗿️ The stamp a routed command answers names the ARTIFACT this workspace bound to the plugin, and
/// a channel with none bound says so in a form no client can mistake for an artifact id. Before
/// ticket 26/09/18 slice M8 it was the bare plugin id, so `artifact_snapshot(revision.artifactId)`
/// answered `no such artifact: note` (`📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §8 gap 4).
#[test]
fn a_routed_channel_resolves_the_artifact_its_plugin_session_document_is() {
    let bound = unbound_plugin_artifacts();
    let router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), Arc::clone(&bound), None);
    let note = AppRoute { plugin_id: "note".to_string(), app_id: Some("note.editor".to_string()) };
    let cad = AppRoute { plugin_id: "cad".to_string(), app_id: Some("cad.editor".to_string()) };
    assert_eq!(router.session_artifact_for(&note), None, "nothing bound yet");
    bound
        .lock()
        .expect("binding map")
        .insert("journey-note-typed".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, document: None, backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });
    assert_eq!(router.session_artifact_for(&note).as_deref(), Some("journey-note-typed"));
    assert_eq!(router.session_artifact_for(&cad), None, "a plugin with no bound artifact stays unnamed");
    assert_eq!(router.session_artifact_for(&AppRoute { plugin_id: "note".to_string(), app_id: Some("note.viewer".to_string()) }), None, "an artifact of one app is never another app's session document");
    bound
        .lock()
        .expect("binding map")
        .insert("journey-note-second".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, document: None, backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });
    assert_eq!(router.session_artifact_for(&note), None, "two artifacts on one app: no single stamp could name either truthfully");
}

/// 🪪️ A plugin with several apps (block 2d/3d/5d, wfc bitmap/grid*) routes every verb to an
/// instance of the verb's OWN app: one slot per app plus the plugin's default route, each decoding
/// back to exactly the route it encodes. Before this, every verb of a plugin reached the instance
/// of its first editor app, and `block3d`'s verbs answered `action app owner … does not match
/// s.block.block2d@1/*#editor` (measured 2026-09-26, `wp-g10` coverage run 2).
#[test]
fn every_app_of_a_multi_app_plugin_routes_to_its_own_instance_slot() {
    let capability = |id: &str, app_id: Option<&str>| crate::catalog::CapabilityDefinition {
        id: crate::catalog::CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: "block".to_string(), label: None, app_id: app_id.map(str::to_string), window_kind_id: None, mode_id: None },
        kind: crate::catalog::CapabilityKind::Mutation,
        audience: crate::catalog::CapabilityAudience::Agent,
        title: id.to_string(),
        description: "test".to_string(),
        artifact_kind: None,
        use_when: Vec::new(),
        input_schema: serde_json::json!({ "type": "object" }),
        output_schema: serde_json::json!({ "type": "object" }),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: crate::catalog::ToolExposure::CatalogOnly,
        presentation: crate::catalog::CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: crate::catalog::CapabilitySource::Gateway,
    };
    let mut entries = vec![
        capability("block.s.block.block2d@1/*#editor.addHandle", Some("s.block.block2d@1/*#editor")),
        capability("block.s.block.block3d@1/*#editor.addVortex", Some("s.block.block3d@1/*#editor")),
        capability("block.s.block.block5d@1/*#editor.addGrip", Some("s.block.block5d@1/*#editor")),
        capability("block.importLibrary", None),
    ];
    entries.sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
    let catalog = Catalog { hash: "multi-app".to_string(), entries };
    assert_eq!(distinct_routes(&catalog).len(), 4, "three apps plus the plugin's default route");
    let slots: Vec<u32> = ["block.s.block.block2d@1/*#editor.addHandle", "block.s.block.block3d@1/*#editor.addVortex", "block.s.block.block5d@1/*#editor.addGrip", "block.importLibrary"].iter().map(|id| capability_instance_slot(&catalog, id).expect("a plugin capability has a slot")).collect();
    assert_eq!(slots.iter().collect::<std::collections::BTreeSet<_>>().len(), 4, "every app and the plugin scope own a distinct slot: {slots:?}");
    assert_eq!(route_for_slot(&catalog, slots[1]), Some(AppRoute { plugin_id: "block".to_string(), app_id: Some("s.block.block3d@1/*#editor".to_string()) }));
    assert_eq!(route_for_slot(&catalog, slots[3]), Some(AppRoute { plugin_id: "block".to_string(), app_id: None }), "a plugin-scope verb runs on the plugin's default route");
    assert_eq!(plugin_instance_slot(&catalog, "block"), Some(slots[3]));
    assert_eq!(app_instance_slot(&catalog, "block", "s.block.block5d@1/*#editor"), Some(slots[2]), "a bound artifact's app finds the slot its verbs use");
}

#[test]
fn routing_artifact_channel_purecommand_unknown_capability_is_not_found_before_opening_any_channel() {
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), unbound_plugin_artifacts(), None);
    let fault = router.exchange(0, vec![AppCommand::PureCommand { capability_id: "totally.unknown.capability".to_string(), input: serde_json::json!({}) }]).expect_err("unknown capability must not route to any plugin");
    assert_eq!(fault.code, "capability.not-found");
}

#[test]
fn routing_artifact_channel_purecommand_gateway_owned_capability_is_plugin_unavailable() {
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), unbound_plugin_artifacts(), None);
    let fault = router.exchange(0, vec![AppCommand::PureCommand { capability_id: "capabilities.search".to_string(), input: serde_json::json!({}) }]).expect_err("a gateway-owned capability names no plugin channel");
    assert_eq!(fault.code, "plugin.unavailable");
}

#[test]
fn routing_artifact_channel_exchange_on_an_unrouted_instance_without_a_purecommand_is_plugin_unavailable() {
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(empty_catalog())), None, "agent:test#sess".to_string(), unbound_plugin_artifacts(), None);
    let fault = router.exchange(0, vec![AppCommand::ReadHistory]).expect_err("no known plugin for this instance and no PureCommand to derive one from");
    assert_eq!(fault.code, "plugin.unavailable");
}

/// 🎫️ W8: the actual defect this ticket fixes — a multi-plugin catalog (note+cad) must route two
/// different capability ids to two different real plugin channels, and open each plugin's channel
/// at most once across repeated commands (including a `ReadHistory` for `note` that carries no
/// capability id at all, decoded via `plugin_for_instance_slot` instead). Skipped with a clear
/// message when the note/cad `.wasm` fixtures are not built (never a fabricated pass) — same
/// convention as `plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired`.
///
/// 🔁️ Re-exchange against `note` via `ReadHistory`, which carries no capability id at all —
/// proving `route_for_slot` decodes the SAME app the earlier `PureCommand` did, and that this
/// second call reuses the cached channel rather than opening a second one.
#[test]
fn routing_artifact_channel_routes_two_capabilities_to_two_different_plugins_opening_each_once() {
    let repo_root = match find_repo_root() {
        Ok(root) => root,
        Err(_) => {
            eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");
            return;
        }
    };
    let registry = match load_plugin_registry(&repo_root) {
        Ok(registry) => registry,
        Err(error) => {
            eprintln!("skipped: plugin registry not generated: {error}");
            return;
        }
    };
    for plugin_id in ["note", "cad"] {
        let Ok(entry) = find_plugin_entry(&registry, plugin_id) else {
            eprintln!("skipped: `{plugin_id}` not in the plugin registry");
            return;
        };
        if resolve_plugin_wasm_path(&repo_root, entry).is_err() {
            eprintln!("skipped: {plugin_id}.wasm not built at target/wasm32-wasip2/{{wasm-dev,wasm-release}}");
            return;
        }
    }
    let descriptors = ["note", "cad"].map(|plugin_id| load_package_descriptor(&find_plugin_entry(&registry, plugin_id).expect("registered above").owner_root).expect("committed descriptor"));
    let catalog = Arc::new(crate::compile(&crate::CatalogSource { descriptors: descriptors.to_vec(), ..Default::default() }, semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native).expect("the real note+cad descriptors compile"));
    let first_app_verb = |plugin_id: &str| {
        catalog
            .entries
            .iter()
            .find(|entry| matches!(&entry.owner, CapabilityOwner::Plugin { plugin_id: owner, app_id: Some(_), .. } if owner == plugin_id) && entry.kind == crate::catalog::CapabilityKind::Mutation)
            .map(|entry| entry.id.as_str().to_string())
            .expect("the plugin declares an app mutation")
    };
    let (note_verb, cad_verb) = (first_app_verb("note"), first_app_verb("cad"));
    let note_instance = capability_instance_slot(&catalog, &note_verb).expect("note's verb has a route slot");
    let cad_instance = capability_instance_slot(&catalog, &cad_verb).expect("cad's verb has a route slot");
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(Arc::clone(&catalog))), Some(crate::workspace::PluginComponentSource::Repo(repo_root)), "agent:routing-test#sess".to_string(), unbound_plugin_artifacts(), None);

    let note_result = router.exchange(note_instance, vec![AppCommand::PureCommand { capability_id: note_verb.clone(), input: serde_json::json!({}) }]);
    assert_ne!(note_result.as_ref().err().map(|fault| fault.code.as_str()), Some("plugin.unavailable"), "{note_verb} must route to a real note channel: {note_result:?}");

    let cad_result = router.exchange(cad_instance, vec![AppCommand::PureCommand { capability_id: cad_verb.clone(), input: serde_json::json!({}) }]);
    assert_ne!(cad_result.as_ref().err().map(|fault| fault.code.as_str()), Some("plugin.unavailable"), "{cad_verb} must route to a real cad channel: {cad_result:?}");

    let _ = router.exchange(note_instance, vec![AppCommand::ReadHistory]);
    assert_eq!(router.channels.lock().expect("cache lock").len(), 2, "exactly one cached channel per distinct app routed to (note, cad), never one per call");
}

#[test]
fn read_artifact_resource_validation_is_plugin_unavailable_never_hardcoded_true() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let error = workspace.read_resource("semio://artifact/does-not-exist/validation").expect_err("must not fabricate `valid: true` for an artifact this workspace has never seen");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn read_artifact_resource_schema_is_real_for_an_open_probe_and_plugin_unavailable_otherwise() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let missing = workspace.read_resource("semio://artifact/does-not-exist/schema").expect_err("no schema query command exists on the real wire yet");
    assert_eq!(missing.code, GatewayErrorCode::PluginUnavailable);
}

#[tokio::test]
async fn read_artifact_resource_schema_is_real_for_an_open_probe() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-schema", serde_json::json!({}).into()).await.expect("seed");
    let contents = workspace.read_resource("semio://artifact/probe-schema/schema").expect("real answer for an open probe artifact");
    let body: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().expect("text body")).expect("json body");
    assert_eq!(body["schema"], PROBE_SCHEMA);
}

#[tokio::test]
async fn apply_probe_mutation_commits_a_real_second_edit_beyond_the_seed() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let seeded = workspace.ensure_probe_artifact("probe-mutate", serde_json::json!({ "n": 1 }).into()).await.expect("seed");
    let mutated = workspace.apply_probe_mutation("probe-mutate", serde_json::json!({ "n": 2 }).into()).await.expect("real second commit");
    assert_ne!(seeded.head_edit_id, mutated.head_edit_id, "a genuine second edit gets a genuinely different edit id");
    assert_ne!(seeded, mutated, "the revision stamp a real caller would compare as `expectedRevision` differs — this is exactly the predicate REVISION_CONFLICT is built on");
    let bytes = workspace.read_artifact_bytes("probe-mutate").expect("read").expect("artifact exists");
    assert!(bytes.0.len() > 0, "the mutated pack is real, non-empty bytes");
}

#[tokio::test]
async fn undo_then_redo_round_trips_a_real_probe_mutation() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-undo-redo", serde_json::json!({ "n": 1 }).into()).await.expect("seed");
    workspace.apply_probe_mutation("probe-undo-redo", serde_json::json!({ "n": 2 }).into()).await.expect("second commit");
    let undone = workspace.undo_probe_mutation("probe-undo-redo").await.expect("real undo");
    assert_eq!(serde_json::Value::from(undone), serde_json::json!({ "n": 1 }), "undo reverts to the seeded value for real");
    let redone = workspace.redo_probe_mutation("probe-undo-redo").await.expect("real redo");
    assert_eq!(serde_json::Value::from(redone), serde_json::json!({ "n": 2 }), "redo restores the undone edit for real — a genuine round trip");
}

#[test]
fn base64_encode_matches_a_known_vector() {
    assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
    assert_eq!(base64_encode(b""), "");
}

#[test]
fn find_plugin_entry_reports_a_typed_not_found_for_an_unknown_plugin() {
    let error = find_plugin_entry(&[], "does-not-exist").expect_err("must not fabricate an entry");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

//#region 🧵️DocumentBackboneEgress
/// 🧵️ ticket 26/09/18 slice M10: a guest's committed envelopes leave the process on the DOCUMENT
/// BACKBONE, and the only messages this channel may relay are the ones addressed at its own actor
/// uri — the same ownership fence the wgpu shell's `route_document_backbone_effects` applies. A
/// shell reply, a topic publish, and another actor's backbone are all three not this document's
/// egress, and confusing any of them for it would relay one document's bytes into another's socket.
#[test]
fn only_a_backbone_effect_addressed_at_this_channel_is_document_egress() {
    let actor = "agent:test#sess";
    let backbone = |uri: &str| semio_framework::kernel::Effect::SendMessage {
        target: semio_framework::kernel::MessageEndpoint::Backbone { uri: uri.to_string() },
        payload: vec![7, 8, 9],
    };
    assert_eq!(document_backbone_payload(actor, &backbone(actor)), Some([7u8, 8, 9].as_slice()));
    assert_eq!(document_backbone_payload(actor, &backbone("agent:other#sess")), None, "another actor's backbone is another document's egress");
    let shell = semio_framework::kernel::Effect::SendMessage {
        target: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId("0".to_string()) },
        payload: vec![7, 8, 9],
    };
    assert_eq!(document_backbone_payload(actor, &shell), None, "the shell reply lane is an answer, never document egress");
    let topic = semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Topic { name: actor.to_string() }, payload: vec![7, 8, 9] };
    assert_eq!(document_backbone_payload(actor, &topic), None, "a topic that happens to be named like the actor is not the backbone");
}

/// 🚧️ …and a committed message that cannot reach the hub is a typed, named fault, never a silent
/// drop. The guest has already committed by the time these bytes exist, so "the edit went nowhere"
/// has to be something the agent is told — with the REASON the document actor is missing, which for
/// a hub document is today the unregistered artifact codec.
#[test]
fn a_committed_backbone_message_with_no_document_actor_faults_with_its_reason() {
    let bound = unbound_plugin_artifacts();
    let router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), Arc::clone(&bound), None);
    let note = AppRoute { plugin_id: "note".to_string(), app_id: Some("note.editor".to_string()) };
    let unbound = router.relay_backbone_egress(&note, vec![vec![1, 2, 3]]).expect_err("no bound document at all");
    assert_eq!(unbound.code, "channel.not-wired");
    assert!(unbound.message.contains("has bound no document to it"), "{}", unbound.message);
    bound.lock().expect("binding map").insert(
        "hub-note".to_string(),
        PluginArtifactBinding {
            schema: "s.note.note".to_string(),
            plugin_id: "note".to_string(),
            app_id: "note.editor".to_string(),
            surface_id: Some("s.note.note@1/*#editor".to_string()),
            document: None,
            backbone: None,
            backbone_blocked_by: Some("no `store::ArtifactCodec` is registered for artifact schema `s.note.note`".to_string()),
            relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            relay: Arc::default(),
            used: 0,
        },
    );
    let blocked = router.relay_backbone_egress(&note, vec![vec![1, 2, 3]]).expect_err("bound, but no document actor");
    assert_eq!(blocked.code, "channel.not-wired");
    assert!(blocked.message.contains("hub-note") && blocked.message.contains("no `store::ArtifactCodec` is registered"), "the fault carries the binding's own recorded reason: {}", blocked.message);
}
//#endregion 🧵️DocumentBackboneEgress

//#region 🗂️GuestDocumentCodec
/// 🗂️ ticket 26/09/18 slice M10: `store::ArtifactCodec`'s four operations are bare, non-capturing
/// `fn` pointers, so a guest-backed codec's thunks resolve their route out of a process-global
/// table. With NO route registered every one of them must refuse in a typed, countable way — a
/// thunk that answered anything at all with no component behind it would be fabricating a document.
///
/// 🧭️ Reads the live table rather than clearing it: registration is process-global and another
/// test in this binary may legitimately hold a route. The assertion is on the SHAPE of the
/// refusal and on the count it reports, both of which hold for any candidate list that cannot
/// print these bytes — and `b"not-a-pack"` is a pair no real component prints.
#[tokio::test]
async fn a_guest_backed_codec_with_no_registered_route_refuses_and_counts_its_candidates() {
    let refused = guest_print_mirror(b"not-a-pack", b"not-an-spr").await.expect_err("no component prints bytes that are not a pack");
    let store::VcsError::Deserialize(message) = &refused else { panic!("print-mirror must refuse by decode, got {refused:?}") };
    assert!(message.contains("no registered guest codec prints this pair") && message.contains("candidate(s)"), "{message}");

    let applied = guest_apply_ops_binary(b"not-a-pack", b"not-an-spr", b"").await.expect_err("no component applies a batch onto bytes that are not a pair");
    let store::VcsError::Deserialize(message) = &applied else { panic!("apply-ops must refuse by decode, got {applied:?}") };
    assert!(message.contains("no registered guest codec applies this batch") && message.contains("candidate(s)"), "{message}");
}

/// 🚫️ …and the two operations `interface codec` does NOT export refuse by naming exactly that,
/// rather than by inventing text no component ever produced. Both belong to the folder `.dsl`/`.ops`
/// lane, which a guest-backed codec exists precisely because a hub binding does not have.
#[tokio::test]
async fn a_guest_backed_codec_refuses_the_two_operations_the_wit_does_not_export() {
    let compiled = guest_compile_dsl("", "").await.expect_err("there is no codec.compile-dsl");
    let store::VcsError::Deserialize(message) = &compiled else { panic!("compile-dsl must refuse by decode, got {compiled:?}") };
    assert!(message.contains("has no `compile-dsl`") && message.contains("pack-schema-hash, genesis, print-mirror, apply-ops and replay-envelopes"), "{message}");

    let envelope = store::os_spr::MutationEnvelope {
        mutation_id: store::os_spr::MutationId("m1".to_string()),
        document_id: store::os_spr::ArtifactId("doc".to_string()),
        actor: store::os_spr::ActorId("agent:test#sess".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: store::os_spr::ArtifactDiff { schema: store::os_spr::SchemaId("gis.map".to_string()), payload: Vec::new() },
        inverse: store::os_spr::InverseMutation { schema: store::os_spr::SchemaId("gis.map".to_string()), payload: Vec::new() },
        timestamp: store::os_spr::HybridLogicalTimestamp::new(1, 1),
        transaction: None, verb: None, line: None,
    };
    let printed = guest_edit_text_from_envelope(&envelope).await.expect_err("there is no per-envelope codec printer");
    let store::VcsError::Deserialize(message) = &printed else { panic!("edit-text must refuse by decode, got {printed:?}") };
    assert!(message.contains("prints whole pairs, not single edits"), "{message}");
}
//#endregion 🗂️GuestDocumentCodec

//#region 🔗️InferenceArtifactBinding
/// 🧾️ One declared inference row carrying a published contract, for the binding laws below. The
/// shape is exactly what a re-described `🀄️wfc` commits under
/// `contributions.inferenceServices[].payload`.
fn bound_inference_row(required: bool, encoding: &str) -> semio_framework::ContributedInferenceMetadata {
    semio_framework::ContributedInferenceMetadata {
        owner: "wfc".into(),
        artifact_kind: "s.wfc.bitmap".into(),
        artifact_schema: "s.wfc.bitmap".into(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.bitmap.solve".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: "wfc".into(),
        depends_on: Vec::new(),
        payload: Some(semio_framework::InferencePayloadContract {
            payload_schema_id: "s.wfc.bitmap.inference.request.v1".into(),
            input_schema: "{\"type\":\"object\"}".into(),
            output_schema: "{\"type\":\"object\"}".into(),
            progress_unit: "cells".into(),
            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".into(), encoding: encoding.into(), required }),
            commit: None,
        }),
    }
}

fn bound_inference_command(document: Option<crate::actions::ArtifactDocumentBinding>, payload: &[u8]) -> crate::actions::InferCommand {
    crate::actions::InferCommand { plugin_id: "wfc".into(), artifact_kind: "s.wfc.bitmap".into(), inference_schema: "s.wfc.bitmap.solve".into(), turn:semio_framework_actor::RetainedTurnInput{operation:71,generation:3,epoch:19,grant:semio_framework_value::RetainedCloneGrant{maximum_items:7,maximum_copy_bytes:3,maximum_capacity_bytes:129,maximum_release_bytes:4096,maximum_depth:2}},revision:7,cancellation_id:"original binding cancellation".into(),work_units:11,maximum_elapsed_milliseconds:13, canonical_payload: payload.to_vec(),artifact_id:String::new(), artifact_document: document,cancel:crate::actions::InferenceCancel::default() }
}

/// 🔗️ The artifact the caller named lands under the field the PLUGIN declared, base64, and nowhere
/// else — the fix for the gap `📓️pz2-puzzle-describe-under-budget.md` §5.2 named (`InferCommand`
/// carried no artifact binding at all, so the guest had no document to read a snapshot from).
///
/// 🔡️ …and what the host wrote is what a guest decodes back, byte for byte — the two halves of
///    `artifact-pack-base64` meet here and nowhere else.
#[test]
fn a_named_artifact_is_bound_under_the_field_the_contract_declares() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(crate::actions::ArtifactDocumentBinding { pack: b"PACK-BYTES".to_vec(), spr: b"SPR".to_vec() }), b"{}");
    let bound: serde_json::Value = serde_json::from_slice(&bind_inference_document(&declared, &command).expect("a named artifact binds")).expect("the bound body is JSON");
    assert_eq!(bound["document"]["pack"], serde_json::Value::String(crate::shell_channel::encode_base64(b"PACK-BYTES")));
    assert_eq!(bound["document"]["spr"], serde_json::Value::String(crate::shell_channel::encode_base64(b"SPR")));
    assert_eq!(crate::shell_channel::decode_base64(bound["document"]["pack"].as_str().expect("a base64 string")), Some(b"PACK-BYTES".to_vec()));
}

/// ✍️ A caller who authored the bound field keeps it: the binding fills a GAP, it never overwrites a
/// body the caller wrote.
#[test]
fn a_caller_authored_binding_field_is_never_overwritten() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(crate::actions::ArtifactDocumentBinding { pack: b"HOST".to_vec(), spr: b"HOST".to_vec() }), br#"{"document":{"pack":"mine","spr":"mine"}}"#);
    let bound: serde_json::Value = serde_json::from_slice(&bind_inference_document(&declared, &command).expect("the caller's body survives")).expect("JSON");
    assert_eq!(bound["document"]["pack"], serde_json::Value::String("mine".to_string()));
}

/// 🚧️ A REQUIRED binding with no artifact named never reaches a guest: the refusal says which tool
/// argument fixes it, in milliseconds, instead of a 240 s dispatch that ends in a decode fault.
#[test]
fn a_required_binding_without_an_artifact_refuses_by_name() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let fault = bind_inference_document(&declared, &bound_inference_command(None, b"{}")).expect_err("a required binding with no artifact is refused");
    assert!(fault.message.contains("artifactId") && fault.message.contains("payload.document"), "{}", fault.message);
}

/// 🚧️ An encoding this gateway has no writer for is named, never silently written in the one shape
/// this host happens to know.
#[test]
fn an_unknown_binding_encoding_is_refused_rather_than_guessed() {
    let declared = bound_inference_row(true, "artifact-dsl-text");
    let fault = bind_inference_document(&declared, &bound_inference_command(Some(crate::actions::ArtifactDocumentBinding::default()), b"{}")).expect_err("an unwritable encoding is refused");
    assert!(fault.message.contains("artifact-dsl-text") && fault.message.contains(semio_framework::INFERENCE_ARTIFACT_PACK_BASE64), "{}", fault.message);
}

/// 🫙 An inference that publishes NO contract keeps the host-opaque behaviour it always had: the
/// caller's own body travels verbatim.
#[test]
fn an_inference_without_a_published_contract_passes_its_payload_through() {
    let mut declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    declared.payload = None;
    let bound = bind_inference_document(&declared, &bound_inference_command(None, br#"{"seed":7}"#)).expect("no contract, no binding");
    assert_eq!(bound, br#"{"seed":7}"#.to_vec());
}
//#endregion 🔗️InferenceArtifactBinding

/// 🪆️ LAW (design §20.15, W-b): the `ChildHeads` a session guest answers become the inference request's dependencies one per owned
/// child, keyed `child:<slot>/<childId>` — the exact key the guest's request validation parses back — with the child's head pack.
#[test]
fn a_composed_documents_child_heads_become_child_keyed_inference_dependencies() {
    let entries = vec![
        store::ChildHeadPackEntry { slot: "content".into(), child_id: "flow-1".into(), dialect: "s.flow@1/*".into(), head_pack: vec![1, 2, 3], owner: String::new() },
        store::ChildHeadPackEntry { slot: "content".into(), child_id: "flow-2".into(), dialect: "s.flow@1/*".into(), head_pack: vec![4], owner: String::new() },
    ];
    let dependencies = child_head_dependencies(entries);
    assert_eq!(dependencies, vec![("child:content/flow-1".to_string(), vec![1, 2, 3]), ("child:content/flow-2".to_string(), vec![4])]);
    assert!(dependencies.iter().all(|(key, _)| semio_framework_plugin::inference_child_dependency_parts(key).is_some()), "the guest parses every key back");
    assert!(child_head_dependencies(Vec::new()).is_empty(), "a document without children sends no dependencies");
}

/// 🔡️ The bound document survives the GUEST's own JSON decoder, byte for byte. The gateway writes
/// the pair with `serde_json`; the guest reads it with the kernel's DSL JSON parser
/// (`protocol::json::from_json_str`) and then base64-decodes it. Two different parsers on one
/// string is exactly the seam where a document silently becomes garbage — and a garbage pack does
/// not fail cleanly, it traps the guest inside the pack inflater (measured 2026-09-22 21:1x).
#[test]
fn a_bound_document_round_trips_through_the_guest_json_decoder() {
    let pack: Vec<u8> = (0..=255u8).cycle().take(405).collect();
    let spr: Vec<u8> = (0..=255u8).rev().cycle().take(211).collect();
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(crate::actions::ArtifactDocumentBinding { pack: pack.clone(), spr: spr.clone() }), b"{}");
    let bound = bind_inference_document(&declared, &command).expect("a named artifact binds");
    let text = std::str::from_utf8(&bound).expect("the bound body is UTF-8");
    let value: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the guest's own JSON decoder reads what serde_json wrote");
    let document = value.get("document").expect("the declared field survives the decoder");
    let pack_text = document.get("pack").and_then(semio_framework_value::DslValue::as_str).expect("pack is a string");
    let spr_text = document.get("spr").and_then(semio_framework_value::DslValue::as_str).expect("spr is a string");
    assert_eq!(crate::shell_channel::decode_base64(pack_text), Some(pack), "the guest decodes the EXACT pack bytes the host bound");
    assert_eq!(crate::shell_channel::decode_base64(spr_text), Some(spr), "…and the exact spr bytes");
}

/// 🧊️ The isolated compile worker of a unit-test process (see [`ISOLATED_COMPILE_TEST_ENTRY`]): a
/// no-op in an ordinary test run, the whole worker when its two inputs are set.
#[test]
fn isolated_compile_worker_entry() {
    use std::io::Read as _;
    let (Some(engine), Some(out)) = (std::env::var_os(ISOLATED_COMPILE_TEST_ENGINE), std::env::var_os(ISOLATED_COMPILE_TEST_OUT)) else { return };
    let mut bytes = Vec::new();
    std::io::stdin().read_to_end(&mut bytes).expect("component on stdin");
    let result = semio_framework_plugin_host::compile_component_isolated(&engine.to_string_lossy(), &bytes, Path::new(&out));
    if let Err(error) = &result {
        eprintln!("{error}");
    }
    std::process::exit(i32::from(result.is_err()));
}

/// 📮️ A hub commit is acknowledged only by a status the document actor reports AFTER the relay: a live
/// link, nothing pending and an acknowledged head. A stale acknowledgement from before the relay, a
/// link in backoff and a link that expired all answer `acknowledged: false` with the link state and
/// the last coded fault, so `action_invoke` reverts the edit and answers `hub.relay-unacknowledged`, never an unqualified success.
/// Measured 2026-09-26 on hub 7800: 1 of 3 agent commits answered SUCCEEDED while the hub head never moved.
#[test]
fn a_hub_commit_is_acknowledged_only_by_a_live_status_reported_after_its_relay() {
    let head = semio_framework_os_kernel::os_directory::EditedArtifactFrontierV1 { document_id: "doc".to_string(), head_edit_ordinal: 1, head_edit_id: "edit".to_string(), last_commit_seq: 1, chain_sha256: "0".repeat(64) };
    let live = |pending: usize, acknowledged: bool| store::sync::ArtifactSyncStatus { persisted: true, pending_mutations: pending, remote: store::sync::RemoteState::Live { peer_count: 1 }, acknowledged_head: acknowledged.then(|| head.clone()) };
    let relay = HubRelay::default();
    relay.record(Some(live(0, true)), None);
    let since = relay.version();
    let stale = relay.await_acknowledged(since, 20);
    assert!(!stale.acknowledged, "an acknowledgement from before the relay does not cover it: {stale:?}");
    relay.record(Some(live(1, false)), None);
    assert!(!relay.await_acknowledged(since, 20).acknowledged, "a pending mutation is not acknowledged");
    relay.record(Some(store::sync::ArtifactSyncStatus { persisted: true, pending_mutations: 1, remote: store::sync::RemoteState::Backoff { retry_in_ms: 4_000 }, acknowledged_head: None }), Some("document.link.expired: the link to the hub expired".to_string()));
    let backoff = relay.await_acknowledged(since, 20);
    assert!(!backoff.acknowledged && backoff.detail.contains("backoff") && backoff.detail.contains("document.link.expired"), "{backoff:?}");
    let waiter = std::thread::scope(|scope| {
        let handle = scope.spawn(|| relay.await_acknowledged(since, 5_000));
        std::thread::sleep(std::time::Duration::from_millis(50));
        relay.record(Some(live(0, true)), None);
        handle.join().expect("waiter")
    });
    assert!(waiter.acknowledged, "a later live, fully acknowledged status wakes the waiting commit: {waiter:?}");
    assert_eq!(relay.report()["remote"], "live");
    assert_eq!(relay.report()["lastFault"], "document.link.expired: the link to the hub expired");
}


#[test]
fn original_probe_pack_receiving_retains_every_cancelled_owner_and_exact_allocations(){
 use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,retirement::ControlledRetirement};
 use std::cell::Cell;
 let vectors:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🧫️fixtures/📦️receiving.json")).unwrap();
 let policy=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
 let mut successes=0;
 for row in vectors["cases"].as_array().unwrap(){for cutoff in (0..=16).chain(std::iter::once(usize::MAX)){
  let input=row["text"].as_str().unwrap().as_bytes();let pointer=input.as_ptr();let allowance=Cell::new(cutoff);let events=Cell::new(0);
  let mut original=|_|{let next=events.get()+1;events.set(next);next<=allowance.get()};
  let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut original);native.install_retirement_recipient(&mut recipient).unwrap();
  let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,policy);
  let(result,born,released)=crate::test_allocation::observe_backing(||<ProbeSnapshot as store::ArtifactPackReceiving>::receive_pack(input,&mut owner));let accepted=owner.progress();assert!(accepted.fits(policy));assert_eq!(born,accepted.retained_capacity_bytes);assert_eq!(released,accepted.released_bytes);drop(owner);assert_eq!(input.as_ptr(),pointer);allowance.set(usize::MAX);
  let mut terminal_born=0;let mut terminal_released=0;
  if let Ok(snapshot)=result{successes+=1;assert_eq!(serde_json::to_value(&snapshot.0).unwrap(),row["expected"]);let mut retained=ControlledRetirement::new(snapshot).ok().unwrap();let(_,born,released)=crate::test_allocation::observe_backing(||retained.step(RetainedCloneGrant::default()).unwrap());assert_eq!((born,released),(0,0));for _ in 0..4096{if retained.terminal_is_empty(){break}let(step,born,released)=crate::test_allocation::observe_backing(||retained.step(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((born,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));terminal_born+=born;terminal_released+=released;}assert!(retained.terminal_is_empty());}
  for _ in 0..4096{if !native.has_retirement_owner(){break}let(step,born,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((born,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));terminal_born+=born;terminal_released+=released;}assert!(!native.has_retirement_owner());assert_eq!(accepted.released_bytes+terminal_released,accepted.retained_capacity_bytes+terminal_born);
 }}
 assert!(successes>=vectors["cases"].as_array().unwrap().len());
 for axis in ["items","capacity","depth"]{let mut denied=policy;match axis{"items"=>denied.maximum_items=0,"capacity"=>denied.maximum_capacity_bytes=0,_=>denied.maximum_depth=0};let mut yes=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,denied);let(result,born,released)=crate::test_allocation::observe_backing(||<ProbeSnapshot as store::ArtifactPackReceiving>::receive_pack(b"null",&mut owner));assert!(result.is_err());assert_eq!((born,released,owner.progress()),(0,0,RetainedCloneProgress::default()));drop(owner);assert!(!native.has_retirement_owner());}
 eprintln!("[DEBUG] Original Probe Pack receiving fixed five-axis policy conserved every System allocation/release across17 original callback cuts, same borrowed pointer and prebirth denied axes");
}
