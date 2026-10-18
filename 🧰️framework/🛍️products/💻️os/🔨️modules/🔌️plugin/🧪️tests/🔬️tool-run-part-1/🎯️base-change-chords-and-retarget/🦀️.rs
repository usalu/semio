// 🎯️ Base change, chords and retarget: the base rebind, chord actions without identity, declared settings reads, in-place reconfigure retarget and the retained previous overlay.

//#region 🎯️RetargetAndSettings
/// 📡️ A remote peer sets the count to `count`; `probe` forwards its mutation batch and `app` ingests it.
async fn ingest_remote_count(app: &mut ToyApp, probe: &mut MemoryBackbone, channel: &str, count: u64, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    let mut remote = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest(), protocol::ActorId("remote".into()), caller.policy, &mut caller.identity).await;
    let mut remote_probe = attach_probe(&mut remote, channel).await;
    assert_eq!(remote.stores.store.local_actor_id(), &protocol::ActorId("remote".into()));
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: count as i32 }.into()], transaction: None }, &mut caller.identity).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.stores.store.generation();
    app.tick_backbone(&mut caller.identity).await.expect("ingest remote edit");
    assert_eq!(app.stores.store.generation(), generation + 1, "the remote edit is ingested");
    drop(remote_probe);
    close(&mut remote, caller);
}

/// ⚖️ LAW: a base change during a run hands every later tick the new identity — a plain job is rebuilt under it
/// from its checkpoint, a retargetable job is rebound in place — so the run keeps appending after the rebase
/// instead of producing ticks the ledger drops as stale.
#[semio_framework_async_macros::async_test]
async fn tool_run_base_change_rebinds_the_running_job_so_ticks_never_carry_a_stale_identity() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["rebind"];
    for (tool_id, rebuilt) in [(text(&fixture["toolId"]), true), (text(&fixture["retarget"]["toolId"]), false)] {
        let mut app = toy_app(number(&expected["target"]), &mut caller).await;
        let mut probe = attach_probe(&mut app, &format!("tool-run-rebind-{tool_id}")).await;
        start(&mut app, tool_id, &mut caller).await;
        pump_until(&mut app, "first checkpointed unit", |app| {
            app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeRebase"]) * number(&fixture["opsPerUnit"]) && app.tool_runs.checkpoint().is_some() && app.tool_runs.state() == Some(ToolRunState::Running)
        }, &mut caller)
        .await;
        ingest_remote_count(&mut app, &mut probe, &format!("tool-run-rebind-remote-{tool_id}"), number(&expected["remoteCount"]), &mut caller).await;
        pump_until(&mut app, "rebase observed", |app| app.tool_runs.slot().is_some_and(|slot| u64::from(slot.generation) == number(&expected["generationAfterRebase"])), &mut caller).await;
        let identity = app.tool_runs.identity().expect("identity");
        let after_rebase = app.tool_runs.provisional().len();
        pump_until(&mut app, "the run appends after the rebase", |app| app.tool_runs.provisional().len() > after_rebase + 2 * number(&fixture["opsPerUnit"]) as usize, &mut caller).await;
        assert_eq!(app.tool_runs.identity(), Some(identity), "{tool_id}: appending never moved the identity again");
        let resumed_from = app.tool_runs.progress().expect("progress").counters.first().map_or(0, |counter| counter.value);
        println!("[STATS] rebind {tool_id}: provisional {after_rebase} -> {} resumedFrom={resumed_from}", app.tool_runs.provisional().len());
        assert_eq!(resumed_from > 0, rebuilt, "{tool_id}: a plain job is rebuilt from its checkpoint, a retargetable job keeps running");
        drop(probe);
        abort_and_close(&mut app, &mut caller).await;
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a §2.5 action that names no identity (a keyboard chord) targets the instance's live run at its
/// generation of the dispatch; an identity argument that is stale still no-ops.
#[semio_framework_async_macros::async_test]
async fn tool_run_chord_actions_without_an_identity_resolve_the_live_run_and_stale_identities_still_no_op() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["chord"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "running", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    let output = app.handle_action("toolRunPause", None, &toy_meta(), &mut caller.identity).await.expect("chord pause").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"), "a chord without arguments pauses the live run");
    let window_only = vec![("windowId".to_string(), DslValue::String(text(&expected["windowArgument"]).into()))];
    assert_eq!(tool_run_action(&mut app, "toolRunResume", window_only, &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("schedule"), "arguments that name no identity resolve the live run too");
    let slot = app.tool_runs.slot().expect("slot");
    let stale = tool_run_action(&mut app, "toolRunPause", vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(number(&fixture["stale"]["wrongGeneration"]).to_string()))], &mut caller).await;
    assert_eq!(stale.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])), "a stale identity still no-ops");
    let generation_only = tool_run_action(&mut app, "toolRunPause", vec![("generation".into(), DslValue::String(slot.generation.to_string()))], &mut caller).await;
    assert_eq!(generation_only.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])), "a partial identity is checked as named, never completed from the live run");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running));
    let output = app.handle_action("toolRunAbort", None, &toy_meta(), &mut caller.identity).await.expect("chord abort").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("closeJob"), "a chord abort aborts the live run");
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let output = app.handle_action("toolRunDismiss", None, &toy_meta(), &mut caller.identity).await.expect("chord dismiss").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("clearTrace"));
    assert!(app.tool_runs.slot().is_none());
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: `settingsChanged` fires only when a value behind a declared settings pointer changes. Republishing the
/// same target and a window-config publication (a camera move) never reconfigure — the run keeps its generation
/// and its job — and a run that declares no settings reads is never reconfigured at all.
#[semio_framework_async_macros::async_test]
async fn tool_run_settings_changed_fires_only_for_the_declared_settings_reads() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["settingsReads"];
    for tool_id in [text(&fixture["toolId"]), text(&expected["undeclaredToolId"])] {
        let declared = tool_id == text(&fixture["toolId"]);
        let mut app = toy_app(number(&expected["target"]), &mut caller).await;
        start(&mut app, tool_id, &mut caller).await;
        pump_until(&mut app, "running with provisional units", |app| app.tool_runs.state() == Some(ToolRunState::Running) && !app.tool_runs.provisional().is_empty(), &mut caller).await;
        let config_generation = app.stores.config_store.generation();
        set_target(&mut app, number(&expected["target"]), &mut caller).await;
        assert!(app.stores.config_store.generation() > config_generation, "{tool_id}: the republished target is a real config publication");
        app.tool_runs.note_window_config_published();
        for _ in 0..8 {
            advance_toy(&mut app, &mut caller).await.expect("turn");
        }
        assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterUnrelatedPublications"]), "{tool_id}: unrelated publications never reconfigure");
        assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(0), "{tool_id}: the job was never rebuilt");
        set_target(&mut app, number(&expected["changedTarget"]), &mut caller).await;
        for _ in 0..8 {
            advance_toy(&mut app, &mut caller).await.expect("turn");
        }
        let generation = u64::from(app.tool_runs.slot().expect("slot").generation);
        println!("[STATS] settingsReads {tool_id}: declared={declared} generation after the changed target={generation}");
        assert_eq!(generation, if declared { number(&expected["generationAfterChangedTarget"]) } else { 0 }, "{tool_id}: only a declared read reconfigures");
        abort_and_close(&mut app, &mut caller).await;
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a retargetable job stays resident while its run is complete and `reconfigure: resume` retargets it in
/// place — a raise continues and a lower retracts without any rebuild — and a base change rebinds it in place.
#[semio_framework_async_macros::async_test]
async fn tool_run_reconfigure_resume_retargets_a_retargetable_job_in_place() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["retarget"];
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    let mut app = toy_app(number(&expected["initialTarget"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-retarget").await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "initial target completes and its resident job is no work", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let run = app.tool_runs.slot().expect("slot").run;
    set_target(&mut app, number(&expected["raisedTarget"]), &mut caller).await;
    pump_until(&mut app, "raised target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit, &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((slot.run, u64::from(slot.generation)), (run, number(&expected["generationAfterRaise"])));
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the raise retargeted the resident job, no rebuild replayed a checkpoint");
    set_target(&mut app, number(&expected["loweredTarget"]), &mut caller).await;
    pump_until(&mut app, "lowered target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["loweredTarget"]) * ops_per_unit && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterLower"]));
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={}", number(&expected["loweredTarget"]))), "the retracted tail leaves the overlay");
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])));
    ingest_remote_count(&mut app, &mut probe, "tool-run-retarget-remote", number(&expected["remoteCount"]), &mut caller).await;
    set_target(&mut app, number(&expected["raisedTarget"]), &mut caller).await;
    pump_until(&mut app, "rebased and raised", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the rebased run was rebound and retargeted in place");
    assert_eq!(app.snapshot().expect("committed").count, number(&expected["remoteCount"]) as i32, "reconfigure never commits");
    drop(probe);
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a retract followed by re-appends spread over several ticks keeps the previous overlay rendered until the
/// job reaches its checkpoint; only then does the refolded overlay replace it.
#[semio_framework_async_macros::async_test]
async fn tool_run_retract_keeps_the_previous_overlay_until_the_job_reaches_its_checkpoint() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["compact"];
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    let counts: Vec<u64> = expected["compactCounts"].as_array().expect("counts").iter().map(number).collect();
    for (index, _) in counts.iter().enumerate() {
        pump_until(&mut app, "a compaction page landed and was folded", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting) && app.tool_runs.provisional().len() == index + 1 && !app.tool_runs.has_pending_work(), &mut caller).await;
        let body = render_text(&mut app, "main", &mut caller).await;
        println!("[STATS] compact page {index}: rendered {body}");
        assert!(app.tool_runs.is_refolding(), "page {index}: the retract refold waits for its boundary");
        assert!(body.contains(text(&expected["renderedWhileCompacting"])), "page {index}: the previous overlay stays rendered: {body}");
        app.tool_runs.port().expect("the run's port").wake();
    }
    pump_until(&mut app, "the compaction checkpoint swaps the overlay", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let body = render_text(&mut app, "main", &mut caller).await;
    assert!(body.contains(text(&expected["renderedAfterCheckpoint"])), "the refolded overlay replaces the previous one at the boundary: {body}");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🎯️RetargetAndSettings
