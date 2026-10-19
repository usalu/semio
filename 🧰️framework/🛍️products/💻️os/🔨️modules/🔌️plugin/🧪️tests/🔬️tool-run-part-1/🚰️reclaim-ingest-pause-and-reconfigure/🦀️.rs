/// 🚰️ Reclaim, ingest, pause and reconfigure: large finalize root reclaim, remote ingest rebase, the canonical operation wire, stale generation no-ops, pause and step, reconfigure and the overlay append timing.

/// ⚖️ LAW: a large finalize never piles up returned document roots — after every driver turn the document Store
/// holds at most one returned-but-unreclaimed snapshot read, so the publication's memory is bounded by the staged
/// root and the base its current op reads instead of one whole document per op.
#[semio_framework_async_macros::async_test]
async fn tool_run_large_finalize_reclaims_each_folded_root_before_the_next_op() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let law = &fixture["largeFinalize"];
    let units = number(&law["units"]);
    let maximum = number(&law["maximumReturnedReads"]) as usize;
    let mut app = toy_app(units, &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let edits = app.stores.store.envelope().vcs.edits.len();
    let output = run_action(&mut app, "toolRunFinalize", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    let (mut turns, mut peak) = (0usize, 0usize);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while !(app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work()) {
        assert!(std::time::Instant::now() < deadline, "the large finalize never settled; state {:?}", app.tool_runs.state());
        advance_toy(&mut app, &mut caller).await.unwrap_or_else(|fault| panic!("large finalize driver turn faulted: {fault:?}"));
        turns += 1;
        peak = peak.max(app.stores.store.returned_snapshot_read_count());
        assert!(peak <= maximum, "turn {turns}: {peak} returned document roots are still held (at most {maximum})");
    }
    assert_eq!(app.stores.store.envelope().vcs.edits.len() - edits, 1, "still exactly one grouped edit");
    assert_eq!(app.stores.store.envelope().vcs.edits.last().expect("finalized edit").forwards.len() as u64, units * number(&fixture["opsPerUnit"]), "every provisional op landed");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_remote_ingest_rebases_and_a_revalidation_conflict_returns_to_complete_with_the_next_generation() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["conflict"];
    let mut app = toy_app(number(&fixture["overlay"]["units"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-conflict").await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let mut remote = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest(), protocol::ActorId("remote".into()), caller.policy, &mut caller.identity).await;
    let mut remote_probe = attach_probe(&mut remote, "tool-run-conflict-remote").await;
    assert_eq!(remote.stores.store.local_actor_id(), &protocol::ActorId("remote".into()));
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: number(&expected["remoteCount"]) as i32 }.into()], transaction: None }, &mut caller.identity).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.stores.store.generation();
    app.tick_backbone(&mut caller.identity).await.expect("ingest remote edit");
    assert_eq!(app.stores.store.generation(), generation + 1, "the remote edit is ingested");
    pump_until(&mut app, "rebase refold settles", |app| app.tool_runs.slot().is_some_and(|slot| u64::from(slot.generation) == number(&expected["generationAfterRebase"])) && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Complete));
    let store_generation = app.stores.store.generation();
    run_action(&mut app, "toolRunFinalize", &mut caller).await;
    pump_until(&mut app, "revalidation returns", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!(slot.state.as_str(), text(&expected["state"]));
    assert_eq!(u64::from(slot.generation), number(&expected["generationAfterConflict"]));
    assert_eq!(u64::from(app.tool_runs.conflicts()), number(&expected["retracted"]));
    assert_eq!(app.stores.store.generation(), store_generation, "a conflict return never publishes");
    assert!(app.tool_runs.provisional().is_empty(), "conflicting ops are retracted");
    assert!(app.tool_runs.steps().expect("steps").iter().any(|step| step.reason == semio_framework_tool_run::TOOL_RUN_REASON_CONFLICT), "a danger conflict step is shown");
    drop((probe, remote_probe));
    close(&mut app, &mut caller);
    close(&mut remote, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🪪️ Law: the runtime mints the document, config, draft and interaction stores each with its canonical operation wire, and each authors a row through it.
/// The fixture is deliberately leaked: its close ladder is the registered-fixture close law, not this one.
#[semio_framework_async_macros::async_test]
async fn the_runtime_installs_its_canonical_operation_wire_on_every_store_it_mints_and_each_authors_through_it() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let mut app = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest(), protocol::ActorId(text(&fixture()["actor"]).into()), caller.policy, &mut caller.identity).await;
    assert!(app.stores.store.operation_wire().is_some(), "the document store mints with its semantic wire");
    assert!(app.stores.config_store.operation_wire().is_some(), "the config store mints with its canonical wire");
    assert!(app.stores.draft_store.operation_wire().is_some(), "the draft store mints with its canonical wire");
    assert!(app.stores.interaction_store.operation_wire().is_some(), "the interaction store mints with its canonical wire");
    let config_mutation: <ToyRunApp as ArtifactApp>::ConfigMutation = ChangeTestConfigSelection { selected: Some("7".to_string()) }.into();
    let rows = app.stores.config_store.authored_operation_bytes(std::slice::from_ref(&config_mutation), &mut caller.identity).expect("the config mutation authors through the runtime's wire");
    assert_eq!(rows.len(), 1);
    assert!(rows[0].starts_with(b"op"), "{:?}", String::from_utf8_lossy(&rows[0]));
    let interaction = <semio_framework_plugin_interaction::mutations::InteractionConfigMutation>::set_state(protocol::InteractionState::default());
    let interaction_rows = app.stores.interaction_store.authored_operation_bytes(std::slice::from_ref(&interaction), &mut caller.identity).expect("the interaction mutation authors through the runtime's wire");
    let oracle = format!("op{}", serde_json::json!({ "setInteractionState": { "activeGranularity": {}, "activeMode": {}, "hover": {}, "selection": {} } }));
    assert_eq!(String::from_utf8(interaction_rows[0].clone()).expect("canonical JSON is text"), oracle, "the interaction wire is the independent serde_json document");
    std::mem::forget(app);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_stale_generation_and_run_actions_are_silent_no_ops_and_an_illegal_action_publishes_nothing() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(number(&fixture["abort"]["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run is running", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    let stale = tool_run_action(&mut app, "toolRunPause", vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(number(&fixture["stale"]["wrongGeneration"]).to_string()))], &mut caller).await;
    assert_eq!(stale.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])));
    let wrong_run = tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(number(&fixture["stale"]["wrongRun"]).to_string())), ("generation".into(), DslValue::String(slot.generation.to_string()))], &mut caller).await;
    assert_eq!(wrong_run.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])));
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running), "stale actions change nothing");
    let generation = app.stores.store.generation();
    let illegal = run_action(&mut app, "toolRunResume", &mut caller).await;
    assert_eq!(illegal.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["illegal"]["code"])));
    assert_eq!(app.stores.store.generation(), generation, "an illegal action publishes nothing");
    let busy = tool_run_action(&mut app, "toolRunStart", vec![("toolId".into(), DslValue::String(text(&fixture["toolId"]).into()))], &mut caller).await;
    assert_eq!(busy.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["busy"]["code"])));
    let panel: Value = serde_json::from_str(&render_text(&mut app, FRAMEWORK_TOOL_RUN_BODY_KEY, &mut caller).await).expect("panel parses");
    let finalize = find_node(&panel, &panel_id(&fixture["panel"]["finalize"]["id"], slot.run)).expect("finalize button");
    assert_eq!((finalize["disabled"].as_bool().unwrap_or(false), finalize["accessibility"].get("description")), (false, None), "a running run offers finalize for the partial result it holds");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_pause_then_step_drives_exactly_one_unit_per_step() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["pauseStep"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    assert_eq!(run_action(&mut app, "toolRunPause", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"));
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("paused turn");
    }
    let mut provisional = app.tool_runs.provisional().len() as u64;
    for _ in 0..number(&expected["steps"]) {
        assert_eq!(run_action(&mut app, "toolRunStep", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("driveOneUnit"));
        pump_until(&mut app, "single step settles", |app| !app.tool_runs.has_pending_work(), &mut caller).await;
        let after = app.tool_runs.provisional().len() as u64;
        assert_eq!(after - provisional, number(&expected["opsPerStep"]), "one step is exactly one algorithm unit");
        provisional = after;
        for _ in 0..8 {
            advance_toy(&mut app, &mut caller).await.expect("paused turn");
        }
        assert_eq!(app.tool_runs.provisional().len() as u64, provisional, "a paused run schedules nothing");
    }
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Paused));
    assert_eq!(run_action(&mut app, "toolRunPause", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("schedule"), "the shared mod+alt+enter chord binds toolRunPause, which resumes a paused run");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running));
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_reconfigure_resumes_from_its_checkpoint_and_a_lowered_target_retracts() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["reconfigure"];
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    let mut app = toy_app(number(&expected["initialTarget"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "initial target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let run = app.tool_runs.slot().expect("slot").run;
    set_target(&mut app, number(&expected["raisedTarget"]), &mut caller).await;
    pump_until(&mut app, "raised target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit, &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((slot.run, u64::from(slot.generation)), (run, number(&expected["generationAfterRaise"])), "reconfigure keeps the run id and bumps the generation");
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the job resumed from its checkpoint");
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={}", number(&expected["raisedTarget"]))));
    set_target(&mut app, number(&expected["loweredTarget"]), &mut caller).await;
    pump_until(&mut app, "lowered target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["loweredTarget"]) * ops_per_unit && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterLower"]));
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={}", number(&expected["loweredTarget"]))), "the retracted tail leaves the overlay");
    assert_eq!(app.snapshot().expect("committed").count, 0, "reconfigure never commits");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⏱️ Bench-style law: a Nakagin-sized fill (771 ticks, two ops each) appends to the overlay in O(k).
/// Machine-load note: tests share the host with concurrent cargo builds, so up to
/// `allowedPreemptedTicks` single ticks may exceed the budget through preemption alone; a real O(n) fold
/// would exceed it on every late tick.
#[semio_framework_async_macros::async_test]
async fn tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["bench"];
    let mut app = toy_app(number(&fixture["defaultTarget"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    let identity = app.tool_runs.identity().expect("identity");
    let label = "x".repeat(number(&expected["labelBytes"]) as usize);
    let ticks = number(&expected["ticks"]);
    let budget = std::time::Duration::from_micros(number(&expected["budgetUs"]));
    let mut over_budget = 0;
    let mut worst = std::time::Duration::ZERO;
    for index in 0..ticks {
        let tick = ToolRunTick {
            identity,
            sequence: index,
            progress: None,
            steps: Vec::new(),
            trace: Vec::new(),
            append_ops: vec![encoded(SetCount { value: index as i32 + 1 }.into()), encoded(SetLabel { value: label.clone() }.into())],
            append_entities: vec![index + 1],
            retract_to: None,
            payload: None,
            payload_update:None,
        };
        let started = std::time::Instant::now();
        let receipt = app.tool_runs.apply_tick(tick).expect("tick applies");
        let elapsed = started.elapsed();
        assert_eq!(receipt.appended, 2);
        worst = worst.max(elapsed);
        if elapsed >= budget {
            over_budget += 1;
        }
    }
    assert!(over_budget <= number(&expected["allowedPreemptedTicks"]), "{over_budget} of {ticks} overlay appends exceeded {budget:?} (worst {worst:?})");
    assert_eq!(app.tool_runs.provisional().len() as u64, ticks * 2);
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={ticks}")));
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
