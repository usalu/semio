// ✅️ Acceptance, blocked review and long history: editing through the selection, a warning that stays after finalize, a long remote history change replayed over turns and the undo of a finalize.

//#region 🧭️AcceptanceLaws
/// 🖱️ Selects `children` in the toy's `test.slot` domain at the `child` granularity, as an interaction verb would.
async fn select_children(app: &mut ToyApp, children: &[String], mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let selection = protocol::DomainSelection { granularity: "child".into(), ids: children.to_vec(), anchor_id: None };
    let state = protocol::InteractionState { selection: BTreeMap::from([("test.slot".to_string(), selection)]), ..protocol::InteractionState::default() };
    app.stores.interaction_store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![InteractionConfigMutation::set_state(state)], lane: HistoryLane::Interaction, transaction: None }, identity).await.expect("the selection lands");
}

/// ⚖️ LAW (design §16.4, R14, R31): a blocked review is resolved by editing the failing mutation's targets — Next problem
/// opens the first blocking mutation from the review; "Use selection" refuses while nothing is selected, then drafts the
/// selected ids; the chips read the app's entity labels (German shell: German labels), every id is a chip row of the
/// reference's tree window (gap N2), a chip's remove binding removes its item; Accept replays and the review is ready.
#[semio_framework_async_macros::async_test]
async fn editing_targets_through_the_selection_resolves_a_blocked_review() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 2 }), serde_json::json!({ "input": { "path": "/children", "value": ["not a uri"] } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "fatal" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let failing = seeded_mutation(&app, 2);
    let panel = app.time_travel.panel().expect("a blocked review");
    assert_eq!((panel.review, panel.next_problem.as_deref()), (Some(semio_framework_time_travel::TimeTravelReview::Blocked), Some(failing.as_str())), "Next problem names the first blocking mutation");
    let history = render_history(&mut app, Locale::En).await;
    let next = find_node(&history, "framework.history.timeTravel.nextProblem").unwrap_or_else(|| panic!("the Next problem button: {history}"));
    assert_eq!(next["bindings"][0]["action"]["name"].as_str(), Some("historyEditBegin"));
    let begun = app.handle_action("historyEditBegin", Some(&dsl(&next["bindings"][0]["args"])), &meta(&fixture), &mut identity).await.expect("Next problem dispatches");
    assert_eq!(rejected(&begun), None, "{:?}", begun.output);
    assert_eq!(app.time_travel.editor().map(|editor| editor.target.0.as_str()), Some(failing.as_str()), "Next problem opens the failing mutation");
    let path = || vec![("path".to_string(), DslValue::String("/children".into()))];
    assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditUseSelection", path(), mounted_policy, &mut identity).await), Some("timeTravel.no-selection"), "nothing is selected yet");
    let children: Vec<String> = (1..=10).map(|index| format!("child-{index}!s.test.child@native/*")).collect();
    select_children(&mut app, &children, mounted_policy, &mut identity).await;
    let drafted = verb(&mut app, &fixture, "historyEditUseSelection", path(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&drafted), None, "{:?}", drafted.output);
    let drafted_children = |app: &ToyApp| app.time_travel.editor().and_then(|editor| editor.value.get("children").cloned());
    assert_eq!(drafted_children(&app), Some(DslValue::Array(children.iter().map(|child| DslValue::String(child.clone())).collect())), "the selection is the draft");
    let history = render_history(&mut app, Locale::De).await;
    let first = find_node(&history, "framework.history.editor.input.children.chip.0.row").unwrap_or_else(|| panic!("the first chip: {history}"));
    assert_eq!(first["component"]["label"].as_str(), Some("Kind child-1"), "a chip reads the entity label in the shell's locale: {first}");
    let references = find_node(&history, "framework.history.editor.input.children.row").unwrap_or_else(|| panic!("the reference row: {history}"));
    assert!(references["component"]["window"]["total"].as_u64().is_some_and(|total| total >= 11), "Use selection and one chip per id, all ten reachable: {references}");
    assert!(find_node(&history, "framework.history.editor.input.children.chip.9.row").is_some_and(|chip| chip.to_string().contains("Kind child-10")), "the tenth chip is a row too: {history}");
    let chip = find_node(&history, "framework.history.editor.input.children.chip.0").unwrap_or_else(|| panic!("the first chip's Remove: {history}"));
    assert_eq!(chip["bindings"][0]["args"]["edit"].as_str(), Some("remove"), "a chip removes its item by index: {chip}");
    let removed = app.handle_action("historyEditInput", Some(&dsl(&chip["bindings"][0]["args"])), &meta(&fixture), &mut identity).await.expect("the chip removes");
    assert_eq!(rejected(&removed), None, "{:?}", removed.output);
    assert_eq!(drafted_children(&app), Some(DslValue::Array(children[1..].iter().map(|child| DslValue::String(child.clone())).collect())), "removing a chip drafts the list without it");
    for step in [serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let status = app.time_travel.status().expect("a review");
    assert_eq!((status.review, status.blocking, status.worst), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, None), "edited targets resolve the review: {status:?}");
    assert!(seeded_mutation_row(&mut app, 2).await.worst.is_none(), "the edited mutation applies cleanly");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §16.5, R20): an upstream edit that turns a downstream mutation into a no-op reviews `ready` (a Warning
/// never blocks) with that mutation's Warning marked as introduced by the edit; finalizing keeps the Warning on its
/// history row — on the authoring replica, after a text reload and after a pack reload, in English and German.
#[semio_framework_async_macros::async_test]
async fn a_warning_an_edit_introduces_stays_visible_after_finalize_and_reload() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone().into()), mounted_policy, &mut identity).await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(actor.clone().into()));
    for value in ["a", "b"] {
        app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetLabel { value: value.into() }.into()], transaction: None }, &mut identity).await.expect("a label edit applies");
    }
    app.render_ctx().refresh_cache().await.expect("the command log backfills");
    assert!(seeded_mutation_row(&mut app, 1).await.worst.is_none(), "before the edit the downstream label changes the document");
    for step in [serde_json::json!({ "begin": 0 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let status = app.time_travel.status().expect("a review");
    assert_eq!((status.review, status.blocking, status.worst), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, Some(semio_framework_diagnostic::Severity::Warning)), "a warning never blocks finalizing");
    let downstream = seeded_mutation_row(&mut app, 1).await;
    assert!(downstream.introduced && downstream.worst == Some(semio_framework_diagnostic::Severity::Warning) && downstream.messages.iter().any(|message| message.code == "mutation.no-op"), "the edit introduced the no-op: {downstream:?}");
    assert!(!seeded_mutation_row(&mut app, 0).await.introduced, "the edited mutation itself raises nothing new");
    let history = render_history(&mut app, Locale::En).await.to_string();
    assert!(history.contains("New since this edit") && history.contains("Warning: No change"), "{history}");
    for step in [serde_json::json!({ "finalize": null }), serde_json::json!({ "commit": { "choice": "overwrite" } })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    pump_until(&mut app, "the finalize retires", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone().into()), mounted_policy, &mut identity).await;
    assert_eq!(reloaded.stores.store.local_actor_id(), &protocol::ActorId(actor.clone().into()));
    artifact_app_laws::load_document_text(&mut reloaded, &app.document_text().await.expect("document text"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("text reload");
    reloaded.render_ctx().refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into()), mounted_policy, &mut identity).await;
    assert_eq!(repacked.stores.store.local_actor_id(), &protocol::ActorId("reader".into()));
    artifact_app_laws::load_document(&mut repacked, &app.document_pack().await.expect("document pack"), &mut identity).await.expect("pack reload");
    repacked.render_ctx().refresh_cache().await.expect("the repacked log backfills");
    for (who, app) in [("finalized", &mut app), ("text reload", &mut reloaded), ("pack reload", &mut repacked)] {
        let row = seeded_mutation_row(app, 1).await;
        assert!(row.worst == Some(semio_framework_diagnostic::Severity::Warning) && row.messages.iter().any(|message| message.code == "mutation.no-op") && !row.introduced, "{who}: the warning persists without a session: {row:?}");
        assert_eq!(head(app).1, "b", "{who}: the edited head");
        for (locale, line) in [(Locale::En, "Warning: No change"), (Locale::De, "Warnung: Keine Änderung")] {
            let history = render_history(app, locale).await.to_string();
            assert!(history.contains(line), "{who} {locale:?}: the row names the warning in words: {history}");
        }
    }
    for app in [&mut app, &mut reloaded, &mut repacked] {
        close(app,cx);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧱️ `replica` with one long edit after the seed, so a replay downstream of the seed exceeds one turn's budget.
async fn long_replica(fixture: &Value, uri: &str, actor: &str, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (ToyApp, MemoryBackbone) {
    let (mut app, probe) = replica(fixture, uri, actor, true, mounted_policy, identity).await;
    apply_long_edit(&mut app, 3_000, identity).await;
    app.render_ctx().refresh_cache().await.expect("the long edit is logged");
    (app, probe)
}

/// ⚖️ LAW (design §16.6, G9): a remote history change whose replay exceeds one turn's budget waits while the replica shows
/// the history before it, its progress rides the history wire (`reprojection`) and the body's reprojection section; Cancel replay
/// pauses it (driver turns leave it alone), Replay again resumes it, and the adoption equals the author's head. The remote
/// store replays one operation per turn, so the seed's downstream suffix already spans several turns.
#[semio_framework_async_macros::async_test]
async fn a_long_remote_history_change_replays_over_turns_and_pauses_on_cancel() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "remote-replay-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "remote-replay-remote", "remote", false, mounted_policy, &mut identity).await;
    remote.stores.store.defer_remote_replays(Some(store::os_store::store_schema::ReplayTurnBudget::operations(1)));
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), head(&local), "the remote replica holds the seed");
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let status = remote.render_ref().reprojection_status().expect("the remote history change waits for its replay");
    assert!(status.total > 0 && status.done < status.total && !status.paused && status.kind == semio_framework::kernel::HistoryReprojectionKind::Remote && status.fault.is_none(), "{status:?}");
    assert_eq!(head(&remote).1, "a", "the replica shows the history before the change");
    assert_eq!(remote.render_ctx().history_patch(true).await.expect("history patch").reprojection.as_ref().map(|remote| remote.total), Some(status.total), "the wire carries the remote replay");
    let body = render_history(&mut remote, Locale::En).await;
    assert!(find_node(&body, "framework.history.reprojection.status").is_some_and(|row| row.to_string().contains("Replaying a remote history change")), "{body}");
    let cancelled = verb(&mut remote, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&cancelled), None, "{:?}", cancelled.output);
    assert!(remote.time_travel.reprojection_paused(), "the remote replay is paused");
    for _ in 0..4 {
        remote.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
        collect_original_history_command_receipt(&mut remote);
        while remote.take_typed_operation_ui_progress().is_some() {}
    }
    assert!(remote.time_travel.reprojection_paused() && !remote.time_travel_ref().time_travel_has_pending_work(), "once its patch shipped, a paused remote replay is no driver work");
    assert_eq!(head(&remote).1, "a", "paused, the change is not adopted");
    let body = render_history(&mut remote, Locale::De).await;
    assert!(find_node(&body, "framework.history.reprojection.rerun").is_some() && body.to_string().contains("pausiert"), "{body}");
    let resumed = verb(&mut remote, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&resumed), None, "{:?}", resumed.output);
    assert_eq!(rejected(&verb(&mut remote, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await), Some("timeTravel.illegal"), "nothing is paused any more");
    pump_until(&mut remote, "the remote change is adopted", |app| app.stores.store.reprojection_progress().is_none(), mounted_policy, &mut identity,cx).await;
    assert_eq!(head(&remote), head(&local), "the adoption equals the author's head");
    assert!(remote.render_ref().reprojection_status().is_none(), "nothing waits any more");
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §16.6, G9): the undo of a finalize over a long downstream history is authored resumably — the verb answers
/// at once, history editing is busy while its replay runs, and the driver commits it: the original input is back and the
/// undo row reads like every other.
#[semio_framework_async_macros::async_test]
async fn the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = long_replica(&fixture, "long-undo", &actor, mounted_policy, &mut identity).await;
    commit_scenario(&mut app, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "b");
    history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
    if app.time_travel.authoring_pending() {
        let first = seeded_mutation(&app, 0);
        let busy = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))], mounted_policy, &mut identity).await;
        assert_eq!(rejected(&busy), Some("timeTravel.busy"), "history editing waits for the authoring");
    }
    pump_until(&mut app, "the undo lands", |app| !app.time_travel.authoring_pending(), mounted_policy, &mut identity,cx).await;
    app.render_ctx().refresh_cache().await.expect("history refresh");
    assert_eq!(head(&app).1, "a", "the undo restores the original input");
    assert_eq!(english(&history_edit_rows(&mut app).await), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🧭️AcceptanceLaws
