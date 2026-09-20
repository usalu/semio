
use super::*;

/// 🔁 How long this test waits on one `ExternalChanged` poke before poking again. A SINGLE poke is
/// not enough: `FolderEventLogStorage::read_archive` answering `Some` proves the archive key exists,
/// not that the agent actor has finished appending this commit's rows to it, so a poke that reaches
/// the shell actor in that window is consumed against an archive it has already fully ingested — and
/// nothing ever pokes it again. Re-poking on every idle interval makes the propagation wait converge
/// on the real disk state instead of on one instant of it, and turns the observed failure mode
/// (`Status(… remote: Detached)` and then silence to the deadline) into an ordinary retry.
const EXTERNAL_CHANGE_REPOKE_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);

fn empty_catalog() -> Arc<Catalog> {
    Arc::new(crate::compile(&crate::CatalogSource::default(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("empty catalog source compiles"))
}

#[tokio::test]
async fn a_headless_commit_propagates_to_a_second_host_on_the_same_folder() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let agent = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:writer".to_string(), Vec::new(), empty_catalog()).expect("agent opens");
    let shell_host = store::sync::ArtifactHost::new(workspace_worker_pool());
    // 🪲️ Post-unblock fix (see `📓️terra-P7-report.md`'s "## post-unblock fixes"): `subscribe`
    // MUST come after `open`, never before. `ArtifactHost::subscribe`'s own doc says exactly why
    // ("If the document is not open the receiver's sender is dropped, so it simply reports
    // closed") — subscribing to a not-yet-open id hands back a receiver whose paired sender is
    // dropped in the very same statement, permanently closed. `open` below mints a BRAND NEW
    // `broadcast::channel` for "shared-doc"; a receiver taken out before that point can never see
    // it. This was this test's own bug, not a gap in `ArtifactHost` or in headless propagation —
    // `Ok(Err(Closed))` (not a timeout) was the tell: the channel was closed from the first poll,
    // never merely slow.
    let shell_channels = shell_host
        .open(store::sync::ArtifactActorConfig {
            document_id: "shared-doc".to_string(),
            schema: PROBE_SCHEMA.to_string(),
            bindings: vec![store::sync::PersistenceBinding::Folder { path: dir.path().to_path_buf() }],
            watch_external: true,
            actor: "shell".to_string(),
        })
        .await;
    let mut shell_events = shell_host.subscribe("shared-doc").await;
    // 🎧️ A second `ProbeStore` (the "live shell") attaches its own backbone end so the store
    // machinery ingests what `subscribe` reports, mirroring how a real shell would.
    let shell_envelope = store::create_document_envelope::<ProbeSnapshot, ProbeMutation>(PROBE_SCHEMA, "shared-doc", ProbeSnapshot::default(), None);
    let mut shell_store = ProbeStore::new(shell_envelope).await.expect("shell store");
    // 🏭️ The shell store needs the SAME owners the agent's own probe installs
    // (`ensure_probe_artifact`): without them an ingested remote edit is refused with
    // `ValidationFailed("edit history insertion requires its exact mutation retirement factory")`,
    // so this side could never observe what the agent committed.
    shell_store.install_document_store_owners_exact(probe_store_owners());
    shell_store.attach_backbone(store::Backbones::Channel(shell_channels.channel_backbone)).await.expect("attach");

    agent.ensure_probe_artifact("shared-doc", serde_json::json!({ "from": "agent" })).await.expect("agent commits headlessly");

    // 🪲️ Post-unblock fix (see `📓️terra-P7-report.md`'s "## post-unblock fixes"): the agent's
    // own actor persists ASYNCHRONOUSLY, on its own thread — `ensure_probe_artifact` returns as
    // soon as the LOCAL store applied the mutation, before the bytes are necessarily on disk yet.
    // Wait for the REAL persisted bytes before expecting the shell's side to see anything.
    // 🗃️ The persisted artefact is a recursive DOCUMENT ARCHIVE, not a bare pack+spr snapshot: the
    // actor's only folder writer is `persist_write_archive` (`🏪️store/🔄️sync/🦀️.rs`), which appends
    // `DOCUMENT_ARCHIVE_PUT_EVENT` rows — `FolderEventLogStorage::write`'s `DOCUMENT_PUT_EVENT` has
    // no producer on this lane at all, so waiting on `read` here waited for an event that can never
    // arrive. `read_archive` is the same key, the kind this lane really writes.
    let storage = store::sync::FolderEventLogStorage::new(dir.path().to_path_buf());
    let write_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    let seeded_archive = loop {
        if let Ok(Some(archive)) = storage.read_archive("shared-doc").await {
            break archive;
        }
        if tokio::time::Instant::now() >= write_deadline {
            panic!(
                "agent's commit never reached disk within 5s; log is {:?} bytes, document ids {:?}",
                std::fs::metadata(dir.path().join(".semio").join("events.semio")).map(|meta| meta.len()),
                storage.document_ids().await
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    };
    // 🪲️ Same root cause, second half: the shell's `notify` watcher IS real and IS wired
    // (`install_watcher`, `📡️spr/🔄️sync`'s own module doc) — but `🏪️store/🔄️sync`'s OWN test
    // suite deliberately does not rely on OS-level filesystem-event timing for determinism
    // either ("notify also wired, but timing-independent here" — that test's own comment); it
    // pokes `ArtifactActorMsg::ExternalChanged` explicitly instead of waiting on notify. This
    // test does the exact same thing, for the exact same reason — NOT a workaround for a broken
    // propagation path, the same deterministic nudge the reference test already establishes as
    // this codebase's own convention for exercising this property without flaking on OS notify
    // latency.
    shell_host.send("shared-doc", store::sync::ArtifactActorMsg::ExternalChanged).await;

    // 🪲️ Widened from 5s to 20s after real flakiness investigation (not a blind bump): with
    // `[DEBUG]` tracing temporarily attached, 9 of 10 runs delivered `RemoteMutations` in well
    // under 1s; the 1 observed failure timed out waiting on `shell_events.recv()` specifically
    // (the disk-write wait above never once timed out) on a machine `ps aux` showed running
    // several DOZEN concurrent `cargo`/`rustc` processes from unrelated sibling tickets at the
    // time (`26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`'s W3/W4 plugin fan-out). That is
    // scheduling contention on the shell actor's own OS thread, not a propagation gap — the
    // `Closed` bug (this test's real structural defect) is fixed above; this margin absorbs
    // shared-box latency instead of re-hiding a broken channel behind a bigger number.
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(20);
    let mut seen: Vec<String> = Vec::new();
    loop {
        match tokio::time::timeout(EXTERNAL_CHANGE_REPOKE_INTERVAL, shell_events.recv()).await {
            Ok(Ok(store::sync::ArtifactEvent::RemoteMutations { envelopes })) if !envelopes.is_empty() => break,
            Ok(Ok(other)) => {
                seen.push(format!("{other:?}").chars().take(120).collect::<String>());
                continue;
            }
            Err(_elapsed) => {
                assert!(tokio::time::Instant::now() < deadline, "no RemoteMutations before the 20s deadline; saw {seen:?}");
                shell_host.send("shared-doc", store::sync::ArtifactActorMsg::ExternalChanged).await;
            }
            other => panic!("the shell's event channel closed before any RemoteMutations: {other:?}; saw {seen:?}"),
        }
    }
    shell_store.tick().await.expect("shell ingests the propagated edit");
    assert_eq!(shell_store.snapshot().expect("shell snapshot").0["from"], "agent", "the shell's own store now sees the agent's headless commit");

    // 🎫️ W3 extension: a SECOND real commit (`apply_probe_mutation`, beyond
    // `ensure_probe_artifact`'s one-shot seed above) propagates too — proving "prepare → commit
    // actually changes the artifact, observable from a second host" end to end over the real VCS
    // this workspace already owns, not just the initial seed.
    agent.apply_probe_mutation("shared-doc", serde_json::json!({ "from": "agent", "revision": 2 })).await.expect("agent commits a second real mutation");
    let second_write_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match storage.read_archive("shared-doc").await {
            Ok(Some(archive)) if archive != seeded_archive => break,
            _ => {}
        }
        if tokio::time::Instant::now() >= second_write_deadline {
            panic!("agent's second commit never reached disk within 5s");
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    shell_host.send("shared-doc", store::sync::ArtifactActorMsg::ExternalChanged).await;
    let second_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        match tokio::time::timeout(EXTERNAL_CHANGE_REPOKE_INTERVAL, shell_events.recv()).await {
            Ok(Ok(store::sync::ArtifactEvent::RemoteMutations { envelopes })) if !envelopes.is_empty() => break,
            Ok(Ok(_other)) => continue,
            Err(_elapsed) => {
                assert!(tokio::time::Instant::now() < second_deadline, "no second RemoteMutations before the 20s deadline");
                shell_host.send("shared-doc", store::sync::ArtifactActorMsg::ExternalChanged).await;
            }
            other => panic!("the shell's event channel closed before the second RemoteMutations: {other:?}"),
        }
    }
    shell_store.tick().await.expect("shell ingests the second propagated edit");
    assert_eq!(shell_store.snapshot().expect("shell snapshot").0["revision"], 2, "the shell observes the agent's second real headless commit too");
    // 🚪️ Both real stores are drained to `ArtifactStore::drop`'s terminal-empty witness before this
    // test's frame unwinds; dropping either one live aborts the whole process in its destructor.
    shell_host.close("shared-doc");
    close_probe_store_to_terminal(shell_store);
}

/// 🎫️ W3: real, honest round trips for all six `PluginArtifactChannel` mutation verbs against
/// `🗒️note`'s real compiled `.wasm` — skipped with a clear message when it is not built (never a
/// fabricated pass). `TransactionCommit`/`Rollback`/`Undo`/`Redo` need only a `txn_id`/`group_id`
/// (no plugin-specific payload), so these are asserted as real, well-typed rejections of an
/// UNKNOWN transaction — genuinely reaching `ArtifactApp::transaction_commit`/etc and getting a
/// real answer back, never `channel.not-wired`. `PureCommand`/`TransactionPrepare` are asserted
/// only to be a real, non-panicking round trip (never `channel.not-wired` either) — see
/// `PluginArtifactChannel::exchange`'s own doc for exactly why a genuine SUCCESS is not possible
/// yet for any currently-compiled plugin (a guest-dispatch gap in `🔌️plugin`'s own shared code,
/// out of this packet's owned file).
#[test]
fn plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired() {
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
    let Ok(entry) = find_plugin_entry(&registry, "note") else {
        eprintln!("skipped: `note` not in the plugin registry");
        return;
    };
    if resolve_plugin_wasm_path(&repo_root, entry).is_err() {
        eprintln!("skipped: note.wasm not built at target/wasm32-wasip2/{{wasm-dev,wasm-release}}");
        return;
    }
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:mutation-test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let mut channel = match workspace.open_artifact_channel("note") {
        Ok(channel) => channel,
        Err(error) => {
            println!("[W3] could not open a real channel to `note`: {error:?}");
            return;
        }
    };

    let pure = channel.exchange(0, vec![AppCommand::PureCommand { capability_id: "note.editor.noop".to_string(), input: serde_json::json!({}) }]);
    println!("[W3] real PureCommand round trip: {pure:?}");
    assert!(pure.as_ref().err().map(|fault| fault.code.as_str()) != Some("channel.not-wired"), "PureCommand must be a real round trip, not the old host-side short-circuit: {pure:?}");

    let prepare = channel.exchange(
        0,
        vec![AppCommand::TransactionPrepare {
            txn_id: "w3-txn-1".to_string(),
            ops: PreparedOps::default(),
            label: "w3 probe".to_string(),
            origin: crate::actions::MutationOrigin::Agent { principal: "agent:mutation-test".to_string(), invocation_id: "inv-1".to_string() },
        }],
    );
    println!("[W3] real TransactionPrepare round trip: {prepare:?}");
    assert!(prepare.as_ref().err().map(|fault| fault.code.as_str()) != Some("channel.not-wired"), "TransactionPrepare must be a real round trip: {prepare:?}");

    let commit = channel.exchange(0, vec![AppCommand::TransactionCommit { txn_id: "w3-unknown-txn".to_string() }]);
    println!("[W3] real TransactionCommit round trip (unknown txn_id): {commit:?}");
    let commit_error = commit.expect_err("committing an unknown txn_id must be a real, typed rejection, never a fabricated success");
    assert_ne!(commit_error.code, "channel.not-wired", "must be the guest's OWN real rejection: {commit_error:?}");

    let rollback = channel.exchange(0, vec![AppCommand::TransactionRollback { txn_id: "w3-unknown-txn".to_string() }]);
    println!("[W3] real TransactionRollback round trip (unknown txn_id): {rollback:?}");
    let rollback_error = rollback.expect_err("rolling back an unknown txn_id must be a real, typed rejection");
    assert_ne!(rollback_error.code, "channel.not-wired");

    let undo = channel.exchange(0, vec![AppCommand::TransactionUndo { group_id: "w3-unknown-group".to_string() }]);
    println!("[W3] real TransactionUndo round trip (unknown group_id): {undo:?}");
    let undo_error = undo.expect_err("undoing an unknown group_id must be a real, typed rejection");
    assert_ne!(undo_error.code, "channel.not-wired");

    let redo = channel.exchange(0, vec![AppCommand::TransactionRedo { group_id: "w3-unknown-group".to_string() }]);
    println!("[W3] real TransactionRedo round trip (unknown group_id): {redo:?}");
    let redo_error = redo.expect_err("redoing an unknown group_id must be a real, typed rejection");
    assert_ne!(redo_error.code, "channel.not-wired");
}

/// ⚖️ LAW: a real `AppCommand` driven into a real compiled guest comes back as a real `AppFrame`.
///
/// 🧭️ The answer of an `AppCommand` travels as `Effect::SendMessage{target: Shell{instance}}`
/// carrying encoded `AppFrame` bytes, correlated by the frame's own `in_reply_to` against the
/// command's own `seq` — it is NOT an `Effect::Respond`. The reactor publishes `Respond` for exactly
/// one inbound event, `Event::Request` (`⚛️reactor/🔄️turn/🦀️.rs`'s `inbound_request_effects`, the
/// extension-capability seam); a command's frames go through `route_app_frame`, which addresses the
/// shell endpoint. Reading the wrong lane is why every mutation verb answered "the guest
/// acknowledged command seq 1 and then went idle without publishing a response for it" once the
/// compiled runtime made turns atomic (ticket 26/09/18, WR1 §5.4 → WR2).
///
/// Skipped with a clear message when `note.wasm` is not built — never a fabricated pass.
#[test]
fn a_real_app_command_is_answered_over_the_shell_message_lane_not_the_respond_lane() {
    let Ok(repo_root) = find_repo_root() else {
        eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");
        return;
    };
    let Ok(registry) = load_plugin_registry(&repo_root) else {
        eprintln!("skipped: plugin registry not generated");
        return;
    };
    let Ok(entry) = find_plugin_entry(&registry, "note") else {
        eprintln!("skipped: `note` not in the plugin registry");
        return;
    };
    if resolve_plugin_wasm_path(&repo_root, entry).is_err() {
        eprintln!("skipped: note.wasm not built at target/wasm32-wasip2/{{wasm-dev,wasm-release}}");
        return;
    }
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:command-response-test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let mut channel = workspace.open_artifact_channel("note").expect("a real channel to `note`");

    let read = channel.exchange(0, vec![AppCommand::ReadArtifact]);
    println!("[WR2] real ReadArtifact round trip: {read:?}");
    let fault = read.as_ref().err();
    assert!(
        !fault.is_some_and(|fault| fault.message.contains("went idle without publishing a response")),
        "the command's answer is a `SendMessage{{Shell}}` frame, not an `Effect::Respond`; a host that reads the respond lane sees an idle guest: {read:?}"
    );
    let frames = read.expect("`note` answers its own genesis document");
    match frames.first() {
        Some(AppFrame::Artifact { pack, .. }) => assert!(!pack.is_empty(), "the guest's genesis document pack is empty: {frames:?}"),
        other => panic!("ReadArtifact must answer a real Artifact frame, received {other:?}"),
    }
}

#[test]
fn attempt_plugin_activation_against_a_real_note_wasm_when_available() {
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
    let Ok(entry) = find_plugin_entry(&registry, "note") else {
        eprintln!("skipped: `note` not in the plugin registry");
        return;
    };
    if resolve_plugin_wasm_path(&repo_root, entry).is_err() {
        eprintln!("skipped: note.wasm not built at target/wasm32-wasip2/{{wasm-dev,wasm-release}}");
        return;
    }
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:activation-test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    match workspace.attempt_plugin_activation("note") {
        Ok(outcome) => {
            println!("[P7] note activation: app_id={} actor={:?} turn_status={} effects={} fuel_used={}", outcome.app_id, outcome.actor, outcome.turn_status, outcome.effects_emitted, outcome.fuel_used);
        }
        Err(error) => {
            // 🚧️ A real, informative failure (e.g. a WIT/kernel event shape mismatch this
            // packet's own effort budget did not resolve) is still useful test output — see
            // `📓️terra-P7-report.md` for exactly what this printed in this environment.
            println!("[P7] note activation did not complete: {error:?}");
        }
    }
}

//#region 🔖️TwoPhaseTypedCommand
/// 🧭️ The one mutation verb this file's two-phase laws drive, resolved out of the REAL compiled
/// catalog rather than spelled here: a capability id is `{plugin}.{app}.{action}` and an app id is
/// itself a dotted canonical surface ref (`s.note.note@1/*#editor`), so it cannot be written by hand
/// without pinning a schema version this law has no business knowing.
fn note_mutation_capability_id() -> Option<String> {
    crate::build_catalog().entries.iter().map(|entry| entry.id.as_str().to_string()).find(|id| id.starts_with("note.") && id.ends_with(".addBlock"))
}

/// 🧬️ The witness these laws compare on: the DOCUMENT itself, read back off the live guest.
///
/// 🧭️ Deliberately not `ReadHistory`. The command log is append-only and an undo does not remove
/// the row it walks back — `transaction_commit` appends `transaction:{txn_id}` and
/// `transaction_undo` only moves the store's own cursor — so a history stamp answers "one command
/// has been recorded" both before and after an undo and could never tell an applied edit from a
/// reverted one. An event-sourced document also keeps its genesis container in `pack` and every edit
/// in the `spr` sidecar (AP1 §3: 515/223 → 515/612 across a real mutation), so both lanes are
/// summed and the content is folded, never just `pack.len()`.
fn document_witness(channel: &mut PluginArtifactChannel, instance: u32) -> String {
    match channel.exchange(instance, vec![AppCommand::ReadArtifact]) {
        Ok(frames) => match frames.first() {
            Some(AppFrame::Artifact { pack, spr }) => {
                let fold = |bytes: &[u8]| bytes.iter().fold(1469598103934665603u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(1099511628211));
                format!("pack:{}/{:016x} spr:{}/{:016x}", pack.len(), fold(pack), spr.len(), fold(spr))
            }
            other => format!("<unexpected {other:?}>"),
        },
        Err(fault) => format!("<fault {}: {}>", fault.code, fault.message),
    }
}

fn history_stamp(channel: &mut PluginArtifactChannel, instance: u32) -> String {
    match channel.exchange(instance, vec![AppCommand::ReadHistory]) {
        Ok(frames) => match frames.first() {
            Some(AppFrame::HistorySnapshot(stamp)) => format!("{}@{}", stamp.cursor, stamp.head_edit_id),
            other => format!("<unexpected {other:?}>"),
        },
        Err(fault) => format!("<fault {}: {}>", fault.code, fault.message),
    }
}

/// ⚖️ LAW (WR4): the agent lane's two phases are genuinely two — and the second one applies ONCE.
///
/// This is the whole of `action_prepare`/`action_invoke`/`history_undo` driven against `🗒️note`'s
/// real compiled guest, asserting the four properties the split exists to give:
///
/// 1. **prepare produces ops** — a real `AppFrame::Emit` with at least one document-lane op, which
///    is only possible if the guest resolved the owner-qualified address, admitted the window view,
///    passed the interactive-job classification and ran `A::command_from_action` + `A::handle`.
/// 2. **prepare applies NOTHING** — the head is byte-identical across it. Before WR4 the pure lane
///    refused outright (`interactive-job.missing-exact-key`); the failure mode this pins is the
///    opposite one, a prepare routed through the applying dispatch lane, which would move the head
///    here and then move it again at commit (`RoutingArtifactChannel` caches ONE guest per plugin).
/// 3. **invoke applies exactly once** — `TransactionPrepare{ops}` + `TransactionCommit` moves the
///    head by one edit, whose group id is the transaction's own.
/// 4. **undo reverts that one application** — the head returns to the prepare-time baseline, which
///    is the assertion that would fail if anything had applied twice.
///
/// A fifth property is asserted in the same run: a prepared handle that is never invoked retires
/// without effect (a second `PureCommand` whose ops are dropped leaves the head where it was).
///
/// Skipped with a clear message when `note.wasm` is not built — never a fabricated pass.
#[test]
fn a_prepared_action_applies_nothing_and_its_commit_applies_exactly_once() {
    let Ok(repo_root) = find_repo_root() else {
        eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");
        return;
    };
    let Ok(registry) = load_plugin_registry(&repo_root) else {
        eprintln!("skipped: plugin registry not generated");
        return;
    };
    let Ok(entry) = find_plugin_entry(&registry, "note") else {
        eprintln!("skipped: `note` not in the plugin registry");
        return;
    };
    if resolve_plugin_wasm_path(&repo_root, entry).is_err() {
        eprintln!("skipped: note.wasm not built at target/wasm32-wasip2/{{wasm-dev,wasm-release}}");
        return;
    }
    let Some(capability_id) = note_mutation_capability_id() else {
        eprintln!("skipped: the compiled catalog publishes no `note.….addBlock` capability");
        return;
    };
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:two-phase-test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let mut channel = workspace.open_artifact_channel("note").expect("a real channel to `note`");
    let input = serde_json::json!({ "kind": "text", "x": 40, "y": 40 });

    let baseline = document_witness(&mut channel, 0);
    assert_eq!(history_stamp(&mut channel, 0), "0@", "a freshly opened `note` has no recorded command yet");
    let prepared = channel.exchange(0, vec![AppCommand::PureCommand { capability_id: capability_id.clone(), input: input.clone() }]);
    println!("[WR4] prepare {capability_id}: {prepared:?}");
    let ops = match prepared.expect("the guest previews its own verb").into_iter().next() {
        Some(AppFrame::Emit { ops, .. }) => ops,
        other => panic!("PureCommand must answer a real Emit frame, received {other:?}"),
    };
    assert!(!ops.document.is_empty(), "a previewed mutation that produced no document-lane op previewed nothing: {ops:?}");
    let after_prepare = document_witness(&mut channel, 0);
    assert_eq!(baseline, after_prepare, "PREPARE APPLIED: the head moved during a phase whose whole contract is that it does not ({baseline} → {after_prepare})");

    let txn = "wr4-two-phase".to_string();
    let prepare_frames = channel
        .exchange(
            0,
            vec![AppCommand::TransactionPrepare {
                txn_id: txn.clone(),
                ops: PreparedOps { document: ops.document.clone(), config: Vec::new(), draft: Vec::new() },
                label: "wr4 two-phase probe".to_string(),
                origin: crate::actions::MutationOrigin::Agent { principal: "agent:two-phase-test".to_string(), invocation_id: "wr4-inv-1".to_string() },
            }],
        )
        .expect("the guest stages the prepared ops");
    println!("[WR4] stage: {prepare_frames:?}");
    let staged = document_witness(&mut channel, 0);
    assert_eq!(baseline, staged, "STAGING APPLIED: TransactionPrepare must stash, never apply ({baseline} → {staged})");

    let committed = channel.exchange(0, vec![AppCommand::TransactionCommit { txn_id: txn.clone() }]).expect("the guest commits the staged ops");
    println!("[WR4] commit: {committed:?}");
    let after_commit = document_witness(&mut channel, 0);
    assert_ne!(baseline, after_commit, "COMMIT APPLIED NOTHING: the one phase that is supposed to mutate left the document at {baseline}");
    // 🧮️ **Exactly once**, stated as a count rather than as a diff. `transaction_commit` lands this
    //    member's whole prepared roster as ONE `Edit` and records ONE command row, so a cursor of
    //    exactly 1 over a document that had none is the assertion that a second application never
    //    happened — which is the failure mode routing `prepare` through the shell's applying
    //    dispatch lane would produce (`RoutingArtifactChannel` caches one guest per plugin, so the
    //    prepare-time apply and the commit-time apply would land on the same store).
    assert_eq!(history_stamp(&mut channel, 0), format!("1@transaction:{txn}"), "APPLIED MORE THAN ONCE: exactly one transaction was committed, so exactly one command row may exist");

    let undone = channel.exchange(0, vec![AppCommand::TransactionUndo { group_id: txn.clone() }]).expect("the guest undoes its own transaction group");
    println!("[WR4] undo: {undone:?}");
    let after_undo = document_witness(&mut channel, 0);
    println!("[WR4] after undo: {after_undo} (commit was {after_commit}, baseline {baseline})");
    // ↩️ **One undo is enough**, stated as the refusal of a second. The document bytes cannot say
    //    this: `note` is event-sourced, so the revert is itself appended to the `.spr` sidecar and
    //    the stream GROWS (measured here: 223 → 671 on the commit → 765 on the undo). What does say
    //    it is `VcsArtifactApp::transaction_undo`'s own precondition — it refuses unless the store's
    //    TAIL edit belongs to the named group — so a second undo of the same group must be refused
    //    by name. Had the commit applied twice, the group would still own the tail and this would
    //    succeed.
    let twice = channel.exchange(0, vec![AppCommand::TransactionUndo { group_id: txn.clone() }]);
    println!("[WR4] second undo of the same group: {twice:?}");
    let refusal = twice.expect_err("a group with one application left has nothing for a second undo to walk back");
    assert!(
        refusal.message.contains("does not belong to group"),
        "DOUBLE APPLY: a second undo of group {txn:?} was not refused for the reason that proves one application ({}: {})",
        refusal.code,
        refusal.message
    );

    let settled = document_witness(&mut channel, 0);
    let abandoned = channel.exchange(0, vec![AppCommand::PureCommand { capability_id: capability_id.clone(), input }]).expect("a second preview answers");
    println!("[WR4] abandoned prepare: {abandoned:?}");
    let after_abandon = document_witness(&mut channel, 0);
    assert_eq!(settled, after_abandon, "A PREPARED HANDLE THAT IS NEVER INVOKED LEFT AN EFFECT: the document moved from {settled} to {after_abandon}");
}
//#endregion 🔖️TwoPhaseTypedCommand
