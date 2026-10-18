/// 🎬️ Fixture scenarios, sessions and drafts: every fixture scenario status, the open session that freezes document verbs and the draft kept across cancel and a remote edit.

/// ⚖️ LAW: every fixture scenario ends in its status and renders its body; accepted verbs never error, the committed
/// document only changes through a commit, and pending/superseded/withdrawn rows are exactly the fixture's.
#[semio_framework_async_macros::async_test]
async fn every_fixture_scenario_reaches_its_status_body_and_rows() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    for scenario in fixture["scenarios"].as_array().expect("scenarios") {
        let id = text(&scenario["id"]);
        let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
        let generation = app.stores.store.generation();
        let mut commits = false;
        for step in scenario["steps"].as_array().expect("steps") {
            commits |= step.get("commit").is_some();
            if let Some(result) = run_step(&mut app, &fixture, step, mounted_policy, &mut identity,cx).await {
                assert_eq!(rejected(&result), None, "{id}: step {step} was refused: {:?}", result.output);
            }
        }
        let status = app.time_travel.status();
        match scenario["status"].as_object() {
            None => assert!(status.is_none(), "{id}: the session closed"),
            Some(expected) => {
                let status = status.unwrap_or_else(|| panic!("{id}: a session is open"));
                assert_eq!(serde_json::to_value(status.stage).expect("stage"), expected["stage"], "{id}: stage");
                assert_eq!(status.blocking, expected["blocking"].as_bool().expect("blocking"), "{id}: blocking");
                assert_eq!(u64::from(status.accepted_count), expected["acceptedCount"].as_u64().expect("accepted"), "{id}: accepted drafts");
                let problem = expected.get("nextProblem").and_then(Value::as_u64).map(|index| seeded_mutation(&app, index as usize));
                assert_eq!(status.next_problem.as_ref().map(|problem| (problem.mutation_id.clone(), problem.store.clone())), problem.map(|mutation| (mutation, None)), "{id}: the next problem");
                assert_eq!(status.next_problem.is_some(), status.blocking, "{id}: a review names its next problem exactly while it blocks");
            }
        }
        if let Some(refusal) = scenario.get("finalizeRefusal") {
            assert_eq!(app.time_travel.session().finalize_refusal().map(|refusal| refusal.code()), refusal.as_str(), "{id}: finalize refusal");
        }
        assert_eq!(render_body(&mut app).await.contains(text(&scenario["body"])), true, "{id}: body {}", render_body(&mut app).await);
        if !commits {
            assert_eq!(app.stores.store.generation(), generation, "{id}: no commit, no store change");
        }
        let patch = app.render_ctx().history_patch(true).await.expect("history patch");
        let rows: Vec<&semio_framework::kernel::HistoryMutationEntry> = patch.upserts.iter().rev().flat_map(|entry| entry.mutations.iter()).collect();
        let mutations: Vec<String> = (0..fixture["seed"].as_array().expect("seed").len()).map(|index| seeded_mutation(&app, index)).collect();
        for (key, flag) in [("pending", 0usize), ("superseded", 1), ("withdrawn", 2)] {
            let Some(expected) = scenario.get(key).and_then(Value::as_array) else { continue };
            let expected: Vec<&str> = expected.iter().map(|index| mutations[index.as_u64().expect("index") as usize].as_str()).collect();
            let actual: Vec<&str> = rows.iter().filter(|row| [row.pending, row.superseded, row.withdrawn][flag]).map(|row| row.mutation_id.as_str()).collect();
            assert_eq!(actual, expected, "{id}: {key} rows");
        }
        if let Some(name) = scenario["alternative"].as_str() {
            let envelope = app.stores.store.envelope();
            let alternative = envelope.vcs.alternatives.iter().find(|alternative| alternative.name == name).unwrap_or_else(|| panic!("{id}: alternative {name} exists"));
            assert_eq!(envelope.active_alternative_id.as_deref(), Some(alternative.id.as_str()), "{id}: the new alternative is active");
            assert!(app.stores.store.supersessions().values().all(|supersession| supersession.scope.as_deref() == Some(alternative.id.as_str())), "{id}: the supersession is scoped to the new alternative");
        }
        if scenario["superseded"].as_array().is_some_and(|superseded| !superseded.is_empty()) && scenario["alternative"].is_null() {
            assert!(app.stores.store.supersessions().values().all(|supersession| supersession.scope.is_none()), "{id}: overwrite is unscoped");
        }
        pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
        close(&mut app,cx);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: while a history edit is open, artifact emits, child emits and history-lane verbs are refused with
/// `timeTravel.frozen`; view verbs keep working; a stale generation and an invalid input value are silent refusals
/// that leave the draft untouched.
#[semio_framework_async_macros::async_test]
async fn an_open_session_freezes_document_verbs_and_refuses_stale_or_invalid_drafts() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let refusals = &fixture["refusals"];
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let emitted = app.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { artifact_mutations: vec![SetCount { value: 3 }.into()], ..Default::default() }, &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    assert_eq!(emitted.err().map(|fault| fault.code), Some(FaultCode::new(text(&refusals["frozen"]))), "an artifact emit is frozen");
    for action in refusals["frozenVerbs"].as_array().expect("frozen verbs") {
        let refused = app.handle_action(text(action), None, &meta(&fixture), &mut identity).await;
        assert_eq!(refused.err().map(|fault| fault.code), Some(FaultCode::new(text(&refusals["frozen"]))), "{action} is frozen");
    }
    let filtered = app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&DslValue::object([("value".to_string(), DslValue::String("all".into()))])), &meta(&fixture), &mut identity).await;
    assert!(filtered.is_ok(), "a view verb keeps working: {filtered:?}");
    let generation = app.time_travel.session().generation;
    let stale = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(9)), ("generation".into(), DslValue::uint(u64::from(generation) + 1))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&stale), Some(text(&refusals["stale"])));
    let invalid = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), dsl(&refusals["invalidInputValue"]))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&invalid), Some(text(&refusals["invalidInput"])));
    assert!(app.time_travel.editor().is_some_and(|editor| editor.refused.is_some()), "the refused value names its reason on the editor");
    assert_eq!(app.time_travel.session().generation, generation, "refusals leave the session untouched");
    let history = render_history(&mut app, Locale::De).await;
    assert!(find_node(&history, "framework.history.timeTravel").is_some(), "the band leads the history body");
    assert!(find_node(&history, "framework.history.editor").is_some(), "the draft editor is shown while editing");
    assert!(find_node(&history, "framework.history.editor.input.value").is_some(), "the input's control is derived from its descriptor");
    assert!(history.to_string().contains("Verlaufsbearbeitung"), "German copy in a German shell");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    assert!(app.handle_action("undo", None, &meta(&fixture), &mut identity).await.is_ok(), "exit lifts the freeze");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: cancelling a running replay keeps the drafts and reviews without a report; a remote edit arriving while
/// reviewing moves the base and replays again, and the reviewed head includes the remote edit.
#[semio_framework_async_macros::async_test]
async fn cancel_keeps_drafts_and_a_remote_edit_replays_again() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let (backbone, mut probe) = MemoryBackbone::pair("time-travel-remote", "time-travel-remote").await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("attach probe backbone");
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&cancelled), None);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    assert!(app.time_travel.session().report.is_none() && app.time_travel.session().accepted.len() == 1, "the draft stays, no report");
    let status = app.time_travel.status().expect("a reviewing session");
    assert_eq!(
        (status.review, status.rerunnable, status.fault.as_deref()),
        (Some(semio_framework::kernel::HistoryTimeTravelReview::NeedsReplay), true, Some(semio_framework_time_travel::TIME_TRAVEL_CANCELLED_CODE)),
        "a cancelled replay needs a rerun, never reads as no changes"
    );
    let history = render_history(&mut app, Locale::En).await;
    assert!(find_node(&history, "framework.history.timeTravel.rerun").is_some_and(|rerun| rerun["disabled"] != Value::Bool(true)), "the band offers Replay again");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&rerun), None);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying, "rerun replays the kept drafts");
    run_step(&mut app, &fixture, &serde_json::json!({ "replay": "clean" }), mounted_policy, &mut identity,cx).await;
    assert_eq!(app.time_travel.status().and_then(|status| status.review), Some(semio_framework::kernel::HistoryTimeTravelReview::Ready));
    let mut remote = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("remote".into()), mounted_policy, &mut identity).await;
    let (remote_backbone, mut remote_probe) = MemoryBackbone::pair("time-travel-remote-peer", "time-travel-remote-peer").await;
    remote.attach_backbone(store::os_store::backbone::Backbones::Memory(remote_backbone)).await.expect("attach remote probe");
    assert_eq!(remote.stores.store.local_actor_id(), &protocol::ActorId("remote".into()));
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }, &mut identity).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.stores.store.generation();
    app.tick_backbone(&mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest remote edit");
    assert!(app.stores.store.generation() > generation, "the remote edit is ingested while the session reviews");
    pump_until(&mut app, "the moved base replays again", |app| app.time_travel.session().stage == TimeTravelStage::Reviewing && app.time_travel.session().report.is_some() && app.time_travel.session().base.content_revision == app.stores.store.content_revision(), mounted_policy, &mut identity,cx)
        .await;
    assert!(render_body(&mut app).await.contains("label=b"), "the replayed head keeps the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    drop((probe, remote_probe));
    close(&mut app,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
