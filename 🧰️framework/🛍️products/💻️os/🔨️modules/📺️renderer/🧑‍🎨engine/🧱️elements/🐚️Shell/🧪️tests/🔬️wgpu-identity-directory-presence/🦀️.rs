
use super::*;

struct LateDialProbe(std::sync::Arc<std::sync::atomic::AtomicUsize>);

impl DirectoryWsConnection for LateDialProbe {
    fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
        Ok(())
    }
    fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> {
        Ok(())
    }
    fn try_recv_text(&mut self) -> Result<semio_framework_os_kernel::os_directory::client::DirectoryWsPoll, TransportError> {
        Ok(semio_framework_os_kernel::os_directory::client::DirectoryWsPoll::Pending)
    }
    fn close(&mut self) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

#[test]
fn dropped_shell_runner_explicitly_closes_a_late_dial_result() {
    let closes = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    close_unowned_directory_dial(Ok(LateDialProbe(closes.clone())));
    assert_eq!(closes.load(std::sync::atomic::Ordering::SeqCst), 1);
}

fn sample_identity() -> Identity {
    Identity { user_id: "u-amara".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://hub.local".to_string(), issued_at_ms: 0 }
}

/// 🧪️ Verify item: "the actor id shape".
#[test]
fn shell_actor_uses_contract_grammar_when_identity_present_else_local_default() {
    assert_eq!(shell_actor(Some(&sample_identity()), "sess-1", 7), "user:u-amara#sess-1");
    assert_eq!(shell_actor(None, "sess-1", 7), "wgpu-7");
}

/// 🧪️ Verify item: "the default-bindings decision with and without identity".
#[test]
fn default_bindings_are_hub_plus_folder_with_identity_and_space() {
    let identity = sample_identity();
    let data_dir = std::path::Path::new("/tmp/semio-s-user1");
    let bindings = default_persistence_bindings(Some(&identity), Some("sp-1"), Some(data_dir), Some("s.space.space@1/*#editor"));
    assert_eq!(bindings.len(), 2, "hub first, folder second");
    match &bindings[0] {
        PersistenceBinding::Hub { base_url, space_id, surface } => {
            assert_eq!(base_url, "http://hub.local");
            assert_eq!(space_id, "sp-1");
            assert_eq!(surface.as_deref(), Some("s.space.space@1/*#editor"));
        }
        other => panic!("expected Hub binding first, got {other:?}"),
    }
    assert_eq!(bindings[1], PersistenceBinding::Folder { path: data_dir.join("spaces").join("sp-1") });
}

#[test]
fn default_bindings_are_folder_only_without_identity() {
    let data_dir = std::path::Path::new("/tmp/semio-s-user1");
    let bindings = default_persistence_bindings(None, Some("sp-1"), Some(data_dir), None);
    assert_eq!(bindings, vec![PersistenceBinding::Folder { path: data_dir.join("spaces").join("sp-1") }]);
}

#[test]
fn default_bindings_are_empty_without_space_or_data_dir() {
    assert!(default_persistence_bindings(None, None, None, None).is_empty());
    assert!(default_persistence_bindings(Some(&sample_identity()), None, None, None).is_empty(), "identity alone, no open space, binds nothing");
}

#[test]
fn directory_command_from_action_covers_every_frozen_verb() {
    assert_eq!(
        directory_command_from_action("os.directory.create-space", Some(&serde_json::json!({"name": "Atelier", "spaceKind": "atelier", "visibility": "private"}))),
        Some(DirectoryCommand::CreateSpace { name: "Atelier".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private })
    );
    assert_eq!(directory_command_from_action("os.directory.delete-space", Some(&serde_json::json!({"spaceId": "sp-1"}))), Some(DirectoryCommand::DeleteSpace { space_id: "sp-1".into() }));
    assert_eq!(
        directory_command_from_action("os.directory.share-link", Some(&serde_json::json!({"spaceId": "sp-1", "role": "author", "ttlSecs": 120}))),
        Some(DirectoryCommand::CreateInvite { space_id: "sp-1".into(), role: DirectorySpaceRole::Author, ttl_secs: 120 }),
        "share-link is client-side sugar for create-invite, contract §C1 has no schema command kind of its own for it"
    );
    assert_eq!(directory_command_from_action("os.unknown.verb", None), None);
}

/// 🧪️ Verify item: "the `os.open-artifact{documentId}` path".
#[test]
fn open_artifact_relay_target_parses_document_and_space_ids() {
    let target = open_artifact_relay_target("os.open-artifact", Some(&serde_json::json!({"artifactRef": "s.space.space@1/*", "documentId": "index", "spaceId": "sp-1", "schema": "s.space"}))).expect("complete target");
    assert_eq!(target.document_id.as_deref(), Some("index"));
    assert_eq!(target.space_id.as_deref(), Some("sp-1"));
    assert_eq!(target.schema.as_deref(), Some("s.space"));
    assert!(open_artifact_relay_target("os.open-artifact", Some(&serde_json::json!({"artifactRef": "s.space.space@1/*"}))).expect("app-only open").document_id.is_none());
}

#[test]
fn open_artifact_relay_vectors_match_the_typescript_contract() {
    let fixture: Value = serde_json::from_str(include_str!("../../../🛠️ShellHelpers/🧫️fixtures/🚪️open-artifact/🔣️.json")).expect("opening relay fixture");
    for vector in fixture["valid"].as_array().expect("valid vectors") {
        let target = open_artifact_relay_target(vector["actionId"].as_str().unwrap(), Some(&vector["args"])).unwrap_or_else(|error| panic!("{}: {error}", vector["id"]));
        let expected = &vector["expected"];
        assert_eq!(target.artifact_ref, expected["artifactRef"].as_str().unwrap(), "{}", vector["id"]);
        assert_eq!(target.dialect.to_coordinate(), expected["artifactRef"].as_str().unwrap(), "{}", vector["id"]);
        assert_eq!(target.role.as_str(), expected["role"].as_str().unwrap(), "{}", vector["id"]);
        assert_eq!(target.plugin_id.as_deref(), vector["args"].get("pluginId").and_then(Value::as_str), "{}", vector["id"]);
        assert_eq!(target.app_id.as_deref(), vector["args"].get("appId").and_then(Value::as_str), "{}", vector["id"]);
        assert_eq!(target.document_id.as_deref(), expected.get("documentId").and_then(Value::as_str), "{}", vector["id"]);
        assert_eq!(target.space_id.as_deref(), expected.get("spaceId").and_then(Value::as_str), "{}", vector["id"]);
        assert_eq!(target.schema.as_deref(), expected.get("schema").and_then(Value::as_str), "{}", vector["id"]);
    }
    for vector in fixture["invalid"].as_array().expect("invalid vectors") {
        let error = open_artifact_relay_target(vector["actionId"].as_str().unwrap(), Some(&vector["args"])).expect_err(vector["id"].as_str().unwrap());
        assert_eq!(error, vector["error"].as_str().unwrap(), "{}", vector["id"]);
    }
}

/// 🧪️ The native command transport's FIFO laws: a fixed capacity that answers the NEWEST intent
/// instead of discarding an older one, a byte-identical retained head across a transient fault,
/// a terminal auth/conflict failure that produces a result and lets the queue proceed, and a
/// bounded transient result slot that never retains a capability past its operation.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_directory_command_queue_retains_a_transient_head_and_proceeds_past_a_terminal_failure() {
    let request = |index: usize| DirectoryCommandRequestV1::new(format!("{index:032x}"), DirectoryCommand::RenameSpace { space_id: "space-a".into(), name: format!("Name {index}") });
    let mut queue = NativeDirectoryCommandQueueV1::default();
    for index in 0..MAX_PENDING_DIRECTORY_COMMANDS {
        queue.admit(request(index + 1));
    }
    assert_eq!(queue.pending(), MAX_PENDING_DIRECTORY_COMMANDS);
    queue.admit(request(9_999));
    assert_eq!(queue.pending(), MAX_PENDING_DIRECTORY_COMMANDS, "a full transport never silently discards an older intent");
    assert_eq!(queue.head().map(|head| head.request_id.clone()), Some(request(1).request_id));
    assert_eq!(queue.result(&request(9_999).request_id), Some(&NativeDirectoryCommandResultV1::Failed(DirectoryCommandErrorCodeV1::Capacity)));

    let mut malformed = NativeDirectoryCommandQueueV1::default();
    malformed.admit(DirectoryCommandRequestV1::new("not-hex", DirectoryCommand::ArchiveSpace { space_id: "space-a".into() }));
    assert_eq!(malformed.pending(), 0, "a malformed correlation is terminal, never queued");

    let mut fifo = NativeDirectoryCommandQueueV1::default();
    fifo.admit(request(1));
    fifo.admit(request(2));
    let head = fifo.head().cloned().expect("transient head");
    assert!(!fifo.settle(Err(DirectoryCommandErrorCodeV1::Overloaded)), "a transient fault stops the FIFO");
    assert_eq!(fifo.head().map(DirectoryCommandRequestV1::canonical_json), Some(head.canonical_json()), "the retained head re-sends byte-identical bytes");
    assert!(fifo.settle(Err(DirectoryCommandErrorCodeV1::Forbidden)), "a terminal denial produces a result and lets the queue proceed");
    assert_eq!(fifo.result(&head.request_id), Some(&NativeDirectoryCommandResultV1::Failed(DirectoryCommandErrorCodeV1::Forbidden)));
    assert_eq!(fifo.pending(), 1);

    let second = fifo.head().cloned().expect("second operation");
    let receipt = DirectoryCommandReceiptV1::seal(second.request_id.clone(), directory_command_sha256(&second.command), DirectoryCommandOutcomeV1::Accepted, Vec::new(), DirectoryCommandResultV1::Invite { invite_token: "invite.v1.one-shot".into() });
    assert!(fifo.settle(Ok(receipt.clone())));
    assert_eq!(fifo.pending(), 0);
    assert_eq!(fifo.result(&second.request_id), Some(&NativeDirectoryCommandResultV1::Receipt(receipt)));

    let mut bounded = NativeDirectoryCommandQueueV1::default();
    for index in 0..MAX_DIRECTORY_COMMAND_RESULTS + 8 {
        bounded.admit(request(index + 1));
        assert!(bounded.settle(Err(DirectoryCommandErrorCodeV1::Forbidden)));
    }
    assert_eq!(bounded.result(&request(1).request_id), None, "the transient result slot is bounded");
    assert_eq!(bounded.result(&request(MAX_DIRECTORY_COMMAND_RESULTS + 8).request_id), Some(&NativeDirectoryCommandResultV1::Failed(DirectoryCommandErrorCodeV1::Forbidden)));

    let mut ordered = NativeDirectoryCommandQueueV1::default();
    ordered.admit(request(1));
    ordered.admit_first(request(2));
    assert_eq!(ordered.head().map(|head| head.request_id.clone()), Some(request(2).request_id), "a freshly issued command is not stuck behind an offline backlog");
}

/// 🧪️ Hub-normalized peer surface and color survive the WGPU footer projection.
#[test]
fn presence_rows_require_each_normalized_surface_and_preserve_hub_color() {
    let peer = |actor: &str, surface: Option<&str>, color: u8| PresencePeer {
        actor: actor.into(),
        connected_at_ms: 1,
        label: Some(actor.into()),
        presence_pack: None,
        user_id: Some(format!("user:{actor}")),
        role: Some("owner".into()),
        drag_ghost_json: None,
        interaction: None,
        color: Some(color),
        surface: surface.map(str::to_owned),
        views: Vec::new(),
        ui: None,
    };
    let editor_surface = "s.space.space@1/*#editor";
    let viewer_surface = "s.space.space@1/*#viewer";
    let peers = vec![peer("a", Some(editor_surface), 7), peer("b", Some(viewer_surface), 8), peer("c", None, 9)];
    let rows = presence_peer_rows_for_surface(&peers, Some(editor_surface), editor_surface);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].actor, "a");
    assert_eq!(rows[0].role, Some(ui_wgpu::wgpu::PresenceRole::Author));
    assert_eq!(rows[0].color, Some(7));
    assert!(presence_peer_rows_for_surface(&peers, Some(editor_surface), viewer_surface).is_empty(), "a different attached surface sees no roster");
    let viewer_rows = presence_peer_rows_for_surface(&peers, Some(viewer_surface), viewer_surface);
    assert_eq!(viewer_rows.iter().map(|row| (row.actor.as_str(), row.color)).collect::<Vec<_>>(), vec![("b", Some(8))]);
    assert!(presence_peer_rows_for_surface(&peers, None, editor_surface).is_empty(), "no attached surface sees no roster");
}

/// 🧪️ Verify item: "the status pill renders each state" — mirrors the React twin's
/// `computeSyncPillState`/`syncPillText` test coverage (`📓️w3-a-report.md`).
#[test]
fn sync_pill_text_covers_persisted_pending_and_every_remote_state() {
    assert_eq!(ShellState::sync_pill_text(None, None), "Remote: detached");
    assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: true, pending_mutations: 0, remote: RemoteState::Live { peer_count: 1 } }), None), "Persisted");
    assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 3, remote: RemoteState::Live { peer_count: 1 } }), None), "Pending (3)");
    assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 0, remote: RemoteState::Connecting }), None), "Remote: connecting");
    assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 0, remote: RemoteState::Backoff { retry_in_ms: 500 } }), None), "Remote: backoff");
    assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 0, remote: RemoteState::Detached }), None), "Remote: detached");
    // 🎯️ A non-live remote takes priority over a nonzero pending count — the connection itself
    // being degraded is the more urgent fact, mirroring the React twin's own priority order.
    assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 9, remote: RemoteState::Backoff { retry_in_ms: 500 } }), None), "Remote: backoff");
    assert_eq!(ShellState::sync_pill_text(None, Some(&(4, 8, 1, 2))), "Recovering 4/8 bytes · 1/2 chunks");
}

//#region 🧪️CheckInTests
fn mutation_entry(seq: u64, applied: bool) -> semio_framework::kernel::HistoryEntry {
    semio_framework::kernel::HistoryEntry { seq, action_id: "apply".into(), label: "Apply".into(), kind: "mutation".into(), applied, ..Default::default() }
}

fn checkpoint_entry(seq: u64) -> semio_framework::kernel::HistoryEntry {
    semio_framework::kernel::HistoryEntry { seq, action_id: "commitCheckpoint".into(), label: "Checkpoint".into(), kind: "history".into(), applied: true, ..Default::default() }
}

/// 🧪️ Verify item: "the fold merges upserts and resets the uncommitted count on a checkpoint".
#[test]
fn fold_history_patch_merges_upserts_and_uncommitted_count_resets_on_checkpoint() {
    let mut entries = BTreeMap::new();
    let mut cursor = 0u64;
    let patch1 = semio_framework::kernel::HistoryPatch { cursor: 2, upserts: vec![mutation_entry(1, true), mutation_entry(2, true)], ..Default::default() };
    assert!(fold_history_patch(&mut entries, &mut cursor, &patch1, false));
    assert_eq!(uncommitted_edit_count(&entries), 2);
    // 🎯️ A stale/duplicate reply (cursor no newer than tracked) changes nothing.
    assert!(!fold_history_patch(&mut entries, &mut cursor, &patch1, false));
    let patch2 = semio_framework::kernel::HistoryPatch { cursor: 3, upserts: vec![checkpoint_entry(3)], current_checkpoint_id: Some("chk-1".into()), ..Default::default() };
    assert!(fold_history_patch(&mut entries, &mut cursor, &patch2, false));
    assert_eq!(uncommitted_edit_count(&entries), 0, "a commitCheckpoint history entry resets the count");
    // 🎯️ A fresh `ReadHistory` snapshot (`replace=true`) discards whatever was tracked before.
    let snapshot = semio_framework::kernel::HistoryPatch { cursor: 1, upserts: vec![mutation_entry(1, true)], ..Default::default() };
    assert!(fold_history_patch(&mut entries, &mut cursor, &snapshot, true));
    assert_eq!(entries.len(), 1);
    assert_eq!(uncommitted_edit_count(&entries), 1);
}

/// 🧪️ Verify item: "viewer guard" — `can_check_in` and its two downstream gates.
#[test]
fn viewers_never_check_in() {
    assert!(can_check_in(semio_framework::manifest::AppRole::Editor));
    assert!(!can_check_in(semio_framework::manifest::AppRole::Viewer));
    assert!(should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, true, 3));
    assert!(!should_checkpoint_before_detach(semio_framework::manifest::AppRole::Viewer, true, 3), "a viewer with pending edits and an attached document still never checkpoints");
}

/// 🧪️ Verify item: "checkpoint-on-close" — the pure gate `checkpoint_before_detach` evaluates.
#[test]
fn checkpoint_before_detach_fires_only_with_an_attached_document_and_pending_edits() {
    assert!(should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, true, 1));
    assert!(!should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, false, 1), "nothing attached, nothing to check in");
    assert!(!should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, true, 0), "nothing uncommitted, nothing to check in");
}

/// 🧪️ Verify item: "auto-fires once per idle period" and "volume trigger".
#[test]
fn auto_checkin_fires_on_idle_or_volume_and_never_twice_while_pending() {
    // 🎯️ No uncommitted edits: never fires, regardless of elapsed time.
    assert!(!auto_checkin_should_fire(0, false, Some(0), 100_000, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
    // 🎯️ Volume trigger — fires immediately once the threshold is reached, without waiting out
    // the idle window (the edit just landed this instant: `now_ms == last_edit_at_ms`).
    assert!(auto_checkin_should_fire(AUTO_CHECKIN_EDIT_THRESHOLD, false, Some(1_000), 1_000, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
    // 🎯️ Below threshold, not yet idle long enough: does not fire.
    assert!(!auto_checkin_should_fire(5, false, Some(1_000), 1_000 + AUTO_CHECKIN_IDLE_MS - 1, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
    // 🎯️ Below threshold, idle window elapsed: fires exactly once per idle period.
    assert!(auto_checkin_should_fire(5, false, Some(1_000), 1_000 + AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
    // 🎯️ The storm guard: already pending (a checkpoint for this idle period is in flight) never
    // fires again, even past the threshold, until the caller observes count return to 0.
    assert!(!auto_checkin_should_fire(AUTO_CHECKIN_EDIT_THRESHOLD, true, Some(1_000), 1_000 + AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
}

/// 🧪️ Verify item: "TouchArtifact follows a checkpoint" — the pure decision
/// `observe_invocation_history` uses to fire it.
#[test]
fn touch_artifact_follows_only_a_checkpoint_this_shell_itself_dispatched() {
    // 🎯️ We asked for a checkpoint, and a NEW checkpoint id landed — fire.
    assert!(checkpoint_landed(None, Some("chk-1"), true));
    assert!(checkpoint_landed(Some("chk-1"), Some("chk-2"), true));
    // 🎯️ We asked for a checkpoint, but nothing changed (same id, e.g. a no-op reply) — no fire.
    assert!(!checkpoint_landed(Some("chk-1"), Some("chk-1"), true));
    // 🎯️ A checkpoint id changed, but THIS shell never dispatched one (a remote peer's checkpoint,
    // or the session mounting with a pre-existing id) — must never fire our own TouchArtifact.
    assert!(!checkpoint_landed(None, Some("chk-1"), false));
    assert!(!checkpoint_landed(Some("chk-1"), Some("chk-2"), false));
    // 🎯️ No checkpoint id at all yet — nothing landed.
    assert!(!checkpoint_landed(None, None, true));
}
//#endregion 🧪️CheckInTests
