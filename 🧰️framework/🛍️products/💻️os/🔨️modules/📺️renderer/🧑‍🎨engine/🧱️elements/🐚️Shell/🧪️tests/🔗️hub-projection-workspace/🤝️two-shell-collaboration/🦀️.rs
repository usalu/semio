/// 🤝️ Two-shell collaboration: two live native shells collaborating on one hub document.

/// 🤝️ Two native wgpu shells, two hub users, one hub document — the whole collaboration journey
/// through each shell's own lanes and state, no browser and no stub: both sign in, the first
/// creates a space and seats the second as an author, both open the space and the same block2d
/// document through the `os.open-artifact` relay (the guest runs natively on the kernel thread,
/// the document actor dials the hub's document socket), presence must show both, each actor's
/// edit must reach the other's ledger, undo is per actor, and severing the second actor's network
/// must neither freeze its shell nor lose its offline edit. "Not frozen" is measured against the same
/// shell's own online edit (at most twice its latency), so the bound holds for a debug interpreter too.
///
/// 📒️ Every step is recorded (PASS/FAIL + what the shells showed) before the law asserts the
/// whole ledger, so one run is the per-step table even when an early step fails.
///
/// 🔌️ `#[ignore]`d: needs a live credential-sign-in hub at `SEMIO_HUB_LIVE_ORIGIN` with the two
/// principals `SEMIO_HUB_LIVE_EMAIL`/`_PASSWORD` and `SEMIO_HUB_LIVE_PEER_EMAIL`/`_PASSWORD`, plus
/// a native runtime staged for `block2d` at `SEMIO_PLUGIN_MODULES`; the renderer package's
/// `hub-live-collaboration-check` verb provides all of it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a live hub, two principals and a staged native block2d runtime; see this test's own doc comment"]
fn two_live_wgpu_shells_collaborate_on_one_hub_document() {
    let journey = native_guest_journey();
    let origin = live_env("SEMIO_HUB_LIVE_ORIGIN");
    let (a_email, a_password) = (live_env("SEMIO_HUB_LIVE_EMAIL"), live_env("SEMIO_HUB_LIVE_PASSWORD"));
    let (b_email, b_password) = (live_env("SEMIO_HUB_LIVE_PEER_EMAIL"), live_env("SEMIO_HUB_LIVE_PEER_PASSWORD"));
    let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
    let variant = live_env("SEMIO_PLUGIN");
    let relay = SeverableRelay::start(&origin);
    let mut ledger = CollaborationLedger::default();
    let plugins = drive(semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
    assert!(plugins.iter().any(|entry| entry.plugin_id == journey.plugin_id.as_str()), "the staged runtime carries {}", journey.plugin_id);
    let mut a = ShellState::new(plugins.clone(), variant.clone(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let mut b = ShellState::new(plugins, variant, semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);

    let a_retry = sign_in_live(&mut a, &origin, &a_email, &a_password);
    let b_retry = sign_in_live(&mut b, &relay.origin, &b_email, &b_password);
    let (a_user, b_user) = (a.directory.hub_workspace.session.user_id.clone(), b.directory.hub_workspace.session.user_id.clone());
    ledger.record(
        "1-sign-in",
        a.directory.hub_workspace.session.phase == HubSessionPhase::SignedIn && b.directory.hub_workspace.session.phase == HubSessionPhase::SignedIn && a_user.is_some() && a_user != b_user,
        format!(
            "A={} {:?} {:?} error={:?} retried-after={a_retry:?} B={} {:?} {:?} error={:?} retried-after={b_retry:?} (B via relay {})",
            a.directory.hub_workspace.session.phase.as_str(),
            a_user,
            a.directory.hub_workspace.display_name,
            a.directory.hub_workspace.session.error,
            b.directory.hub_workspace.session.phase.as_str(),
            b_user,
            b.directory.hub_workspace.display_name,
            b.directory.hub_workspace.session.error,
            relay.origin
        ),
    );

    let space_name = format!("g7w collaboration {}", chrome_now_ms() as u64);
    hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
    hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CREATE_SPACE, &[]);
    let space_id = a.directory.hub_workspace.rows.iter().find(|row| row.name == space_name).map(|row| row.id.clone()).unwrap_or_default();
    shell_command(&mut a, "os.directory.upsert-member", &[("spaceId", space_id.as_str()), ("email", b_email.as_str()), ("role", "author")]);
    hub_verb(&mut b, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::REFRESH_SPACES, &[]);
    let b_row = b.directory.hub_workspace.rows.iter().find(|row| row.id == space_id).map(|row| (row.name.clone(), row.role));
    ledger.record("2-same-space", !space_id.is_empty() && b_row.as_ref().is_some_and(|(_, role)| *role == Some(DirectorySpaceRole::Author)), format!("space={space_id} B sees {b_row:?}"));

    hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
    hub_verb(&mut b, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
    let (a_roster, b_roster) = (online_members(&mut a, &space_id), online_members(&mut b, &space_id));
    ledger.record("3-roster", a_roster.len() == 2 && b_roster.len() == 2, format!("A roster={a_roster:?} B roster={b_roster:?}"));

    let kind_id = a.directory.hub_workspace.creation.catalog.as_ref().and_then(|catalog| catalog.kinds.iter().find(|kind| kind.schema == journey.schema.as_str())).map(|kind| kind.kind_id.clone()).unwrap_or_default();
    hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SELECT_ARTIFACT_KIND, &[("kindId", kind_id.as_str())]);
    hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_ARTIFACT_NAME, &[("value", "Shared board")]);
    hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CREATE_ARTIFACT, &[]);
    let creation_started = std::time::Instant::now();
    let mut creation_trail = Vec::new();
    while creation_started.elapsed() < std::time::Duration::from_secs(150) {
        let observed = a.directory.hub_workspace.creation.operation.as_ref().map(|operation| (semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::hub_artifact_creation_phase_str(operation.phase), operation.opening));
        if creation_trail.last() != Some(&observed) {
            creation_trail.push(observed);
        }
        if a.directory.hub_workspace.creation.operation.as_ref().is_some_and(|operation| operation.opening != HubArtifactOpening::Idle && operation.opening != HubArtifactOpening::Opening) {
            break;
        }
        let _ = frame_pump(&mut a);
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    let a_open = settle_document_opening(&mut a);
    println!("g7w-live A open frames={a_open:?} phase={:?}", a.opening.document_opening.as_ref().map(|opening| &opening.phase));
    let created = a.directory.hub_workspace.creation.operation.as_ref().and_then(|operation| operation.ready.clone());
    ledger.record(
        "4a-create-through-the-door",
        created.is_some() && a.directory.hub_workspace.creation.operation.as_ref().is_some_and(|operation| operation.opening == HubArtifactOpening::Opened),
        format!("catalog={} kind={kind_id:?} trail={creation_trail:?} created={created:?} after={:?}", a.directory.hub_workspace.creation.catalog_phase.as_str(), creation_started.elapsed()),
    );
    let document_id = created.as_ref().map(|ready| ready.artifact_id.clone()).unwrap_or_default();
    let open_args =
        [("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id.as_str()), ("schema", journey.schema.as_str()), ("spaceId", space_id.as_str())];
    shell_command(&mut b, "os.open-artifact", &open_args);
    let b_open = settle_document_opening(&mut b);
    println!("g7w-live B open frames={b_open:?}");
    let session_of = |shell: &ShellState| shell.docs.session.as_ref().map(|session| (session.plugin_id.clone(), session.app.id.clone(), session.instance_id));
    ledger.record(
        "4-guest-mounted",
        [&a, &b].iter().all(|shell| shell.docs.session.as_ref().is_some_and(|session| session.plugin_id == journey.plugin_id.as_str() && session.app.id == journey.app_id.as_str())),
        format!("A session={:?} error={:?} B session={:?} error={:?}", session_of(&a), a.chrome.error, session_of(&b), b.chrome.error),
    );
    ledger.record(
        "5-hub-binding",
        a.sync.sync_channel.is_some() && b.sync.sync_channel.is_some() && a.directory.presence_surface.is_some() && b.directory.presence_surface.is_some(),
        format!("A uri={:?} surface={:?} B uri={:?} surface={:?}", a.sync.sync_backbone_uri, a.directory.presence_surface, b.sync.sync_backbone_uri, b.directory.presence_surface),
    );

    let mut trail = Vec::new();
    let live_after = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(30), &mut trail, |a, b| is_live(a) && is_live(b));
    let codec = match drive(store_sync::os_store::codec_registry::document_kind_codec(journey.schema.as_str())) {
        Ok(Some(codec)) => drive(codec.pack_schema_hash()).ok(),
        _ => None,
    };
    let socket_live = ledger.record("6-document-socket-live", live_after.is_some(), format!("after={live_after:?} trail={trail:?} native codec for {}={codec:?} hub_documents A={:?} B={:?}", journey.schema, a.sync.hub_documents, b.sync.hub_documents));

    let presence_after = socket_live.then(|| pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |a, b| a.directory.presence_peers.len() >= 2 && b.directory.presence_peers.len() >= 2)).flatten();
    let (a_online, b_online) = (online_members(&mut a, &space_id), online_members(&mut b, &space_id));
    ledger.record(
        "7-presence-both",
        presence_after.is_some() && a_online.iter().all(|(_, online)| *online) && b_online.iter().all(|(_, online)| *online),
        format!("after={presence_after:?} A peers={} roster={a_online:?} B peers={} roster={b_online:?}", a.directory.presence_peers.len(), b.directory.presence_peers.len()),
    );

    let a_before = applied_edits(&mut a).len();
    let (a_edit, a_latency) = author_edit(&mut a, journey.verb.as_str());
    let a_ledger = applied_edits(&mut a);
    ledger.record("8-a-authors", a_edit.is_ok() && a_ledger.len() == a_before + 1, format!("outcome={a_edit:?} latency={a_latency:?} A ledger {a_before}->{:?}", a_ledger));

    let b_before = applied_edits(&mut b).len();
    let b_ingest = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let b_seen = applied_edits(&mut b);
    ledger.record("9-b-ingests", socket_live && b_seen.len() > b_before, format!("pumped={b_ingest:?} B ledger {b_before}->{b_seen:?} B remote={}", remote_of(&b)));

    let a_seen_before = applied_edits(&mut a).len();
    let (b_edit, b_latency) = author_edit(&mut b, journey.verb.as_str());
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let a_seen = applied_edits(&mut a);
    ledger.record("10-b-authors-a-ingests", b_edit.is_ok() && socket_live && a_seen.len() > a_seen_before, format!("B outcome={b_edit:?} latency={b_latency:?} A ledger {a_seen_before}->{a_seen:?}"));

    let (undo, _) = author_edit(&mut a, journey.undo.as_str());
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let (a_after_undo, b_after_undo) = (applied_edits(&mut a), applied_edits(&mut b));
    let a_own_reverted = a_after_undo.iter().filter(|(action, applied)| action == journey.verb.as_str() && !applied).count() == 1;
    ledger.record("11-per-actor-undo", undo.is_ok() && a_own_reverted && socket_live, format!("undo={undo:?} A ledger={a_after_undo:?} B ledger={b_after_undo:?}"));

    let severed = relay.sever();
    let (offline_edit, offline_latency) = author_edit(&mut b, journey.verb.as_str());
    let offline_pump = std::time::Instant::now();
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(3), &mut trail, |_, _| false);
    let pump_elapsed = offline_pump.elapsed();
    hub_verb(&mut b, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::REFRESH_SPACES, &[]);
    let offline_phase = b.directory.hub_workspace.phase.as_str();
    let offline_remote = remote_of(&b);
    relay.heal();
    hub_verb(&mut b, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::REFRESH_SPACES, &[]);
    let healed_phase = b.directory.hub_workspace.phase.as_str();
    let a_before_heal = applied_edits(&mut a).len();
    let relive = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(40), &mut trail, |a, b| is_live(a) && is_live(b));
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let a_after_heal = applied_edits(&mut a);
    ledger.record(
        "12-connection-loss",
        offline_edit.is_ok() && offline_latency <= b_latency.saturating_mul(2) && pump_elapsed < std::time::Duration::from_secs(5) && offline_phase == semio_framework_os_renderer_wgpu_hub_contracts::space_browser::SpaceBrowserPhase::Stale.as_str() && healed_phase == semio_framework_os_renderer_wgpu_hub_contracts::space_browser::SpaceBrowserPhase::Ready.as_str() && relive.is_some() && a_after_heal.len() > a_before_heal,
        format!("severed={severed} B offline edit={offline_edit:?} latency={offline_latency:?} pump={pump_elapsed:?} B spaces offline={offline_phase} healed={healed_phase} B remote offline={offline_remote} relive={relive:?} A ledger {a_before_heal}->{a_after_heal:?}"),
    );

    println!("g7w-live trail={trail:?}");
    let failed: Vec<&str> = ledger.steps.iter().filter(|(_, passed, _)| !passed).map(|(step, _, _)| *step).collect();
    hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SIGN_OUT, &[]);
    hub_verb(&mut b, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SIGN_OUT, &[]);
    assert!(failed.is_empty(), "two-user collaboration steps failed: {failed:?}");
}
