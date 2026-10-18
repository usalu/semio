/// 🔁️ Unchanged input, stage and alternatives: the retained editor on an unchanged input, the stage change generation, the alternatives section and host events for history and remote edits.

/// 🧾️ LAW: unchanged Accept is refused through the app route, preserving the draft, preview and committed store.
#[semio_framework_async_macros::async_test]
async fn accepting_an_unchanged_input_retains_the_open_editor_and_preview() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let session = app.time_travel.session().clone();
    let pending = session.pending.as_ref().expect("the open draft");
    assert!(session.unchanged(pending), "opening preserves the draft baseline: {pending:?}");
    assert!(!app.time_travel.panel().expect("native history panel").editor.expect("native editor availability").changed, "unchanged availability comes from the authoritative draft baseline");
    let input = app.time_travel.editor().expect("the draft editor").value.clone();
    let body = render_body(&mut app).await;
    let generation = app.stores.store.generation();
    for (locale, reason) in [(Locale::En, "Change an input before accepting"), (Locale::De, "Vor dem Übernehmen eine Eingabe ändern")] {
        let history = render_history(&mut app, locale).await;
        let accept = find_node(&history, "framework.history.editor.accept").expect("the Accept control");
        let row = find_node(&history, "framework.history.editor.accept.row").expect("the Accept row");
        assert_eq!(accept["disabled"], Value::Bool(true), "{locale:?}: {accept}");
        assert!(row.to_string().contains(reason), "{locale:?}: {row}");
        assert_ne!(find_node(&history, "framework.history.editor.discard").expect("the Discard control")["disabled"], Value::Bool(true));
    }
    let refused = verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&refused), Some(text(&fixture["refusals"]["unchanged"])));
    assert_eq!(app.time_travel.session(), &session);
    assert_eq!(app.time_travel.editor().expect("the draft editor stays open").value, input);
    assert_eq!(render_body(&mut app).await, body);
    assert_eq!(app.stores.store.generation(), generation);
    for (value, changed) in [(7, true), (1, false)] {
        run_step(&mut app, &fixture, &serde_json::json!({ "input": { "path": "/value", "value": value } }), mounted_policy, &mut identity,cx).await;
        let history = render_history(&mut app, Locale::En).await;
        let accept = find_node(&history, "framework.history.editor.accept").expect("the Accept control");
        assert_eq!(accept["disabled"] == Value::Bool(true), !changed, "{accept}");
    }
    let discarded = verb(&mut app, &fixture, "historyEditDiscard", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&discarded), None);
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    assert!(app.time_travel.status().is_none());
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a stage change ships its session status in the turn it happens — Begin from a review bumps the generation,
/// and the editing stage with that generation rides both the command reply and the unsolicited UI progress frame of the
/// same turn (the one carrier of a retained-surface intent, whose reply has no patch), even while the review's snapshots
/// still retire; a verb stamped with the published generation applies, one stamped with the previous is stale.
#[semio_framework_async_macros::async_test]
async fn a_stage_change_ships_its_generation_in_the_same_turn_and_a_verb_stamped_with_it_applies() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    while app.take_typed_operation_ui_progress().is_some() {}
    let reviewed = app.time_travel.session().generation;
    let mutation = seeded_mutation(&app, 2);
    let begun = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))], mounted_policy, &mut identity).await;
    let generation = app.time_travel.session().generation;
    assert_ne!(generation, reviewed, "Begin from a review bumps the generation");
    let status = |patch: &semio_framework::kernel::HistoryPatch| patch.time_travel.as_ref().map(|status| (status.stage, status.generation));
    let editing = Some((semio_framework::kernel::HistoryTimeTravelStage::Editing, generation));
    assert_eq!(begun.history_patch.as_ref().and_then(status), editing, "the reply publishes the new generation with the stage");
    let progress = app.take_typed_operation_ui_progress().expect("the stage change owes this turn a UI progress frame");
    assert_eq!(progress.history_patch.as_ref().and_then(status), editing, "the progress frame publishes it before any driver turn");
    assert!(progress.history_patch.as_ref().is_some_and(|patch| !patch.upserts.is_empty()), "it carries the rows the session overlay moved");
    assert!(app.time_travel.has_pending_work(), "the review's snapshots still retire behind it");
    let generation_arg = |value: u32| vec![(semio_framework::history_edit::HISTORY_EDIT_ARG_GENERATION.to_string(), DslValue::uint(u64::from(value)))];
    let stale = verb(&mut app, &fixture, "historyEditDiscard", generation_arg(reviewed), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&stale), Some("timeTravel.stale"), "the previous generation is stale");
    let discarded = verb(&mut app, &fixture, "historyEditDiscard", generation_arg(generation), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&discarded), None, "the published generation applies: {:?}", discarded.output);
    let deadline=std::time::Instant::now()+std::time::Duration::from_secs(30);
    let mut reviewing_progress=false;
    while std::time::Instant::now()<deadline {
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("original Discard bounded publication");
        collect_original_history_command_receipt(&mut app);
        while let Some(progress)=app.take_typed_operation_ui_progress(){reviewing_progress|=progress.history_patch.as_ref().and_then(status)==Some((semio_framework::kernel::HistoryTimeTravelStage::Reviewing,app.time_travel.session().generation));}
        if !app.time_travel.has_pending_work(){break}
    }
    assert!(!app.time_travel.has_pending_work(),"original Discard effects reach terminal custody");
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    assert!(reviewing_progress,"the actual bounded return publishes its reviewing generation");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🌿️ LAW: the history body lists every line — the trunk (read "Main line", the original history) and the alternative a
/// finalize kept — with its current marker, author and branch time and whether it carries history edits, in English and
/// German; the other line offers Switch, whose rendered target dispatches `switchAlternative{alternativeId}`, and
/// switching reprojects: the edited alternative applies its history edit, the trunk shows the original.
#[semio_framework_async_macros::async_test]
async fn the_alternatives_section_lists_each_alternative_and_switching_reprojects_its_history_edits() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = replica(&fixture, "history-alternatives", &actor, true, mounted_policy, &mut identity).await;
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original", mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "b", "the edited alternative is current");
    let history = app.render_ctx().test_history().await;
    let listed: Vec<(&str, bool, bool)> = history.alternatives.iter().map(|alternative| (alternative.name.as_str(), alternative.current, alternative.edited)).collect();
    assert_eq!(listed, vec![("", false, false), ("Variant", true, true)], "the trunk first, then the kept edit");
    let (trunk, variant) = (history.alternatives[0].id.clone(), history.alternatives[1].id.clone());
    assert_eq!(history.alternatives[1].author.as_deref(), Some(actor.as_str()));
    assert!(history.alternatives[1].branched_at.as_deref().is_some_and(|at| at.ends_with('Z')), "{:?}", history.alternatives[1]);
    let row_key = |id: &str| format!("framework.history.alternative.{id}");
    for (locale, main_line, current, edited, switch) in [(Locale::En, "Main line", "Current", "Edited history", "Switch"), (Locale::De, "Hauptlinie", "Aktuell", "Bearbeiteter Verlauf", "Wechseln")] {
        let body = render_history(&mut app, locale).await;
        let section = find_node(&body, "framework.history.alternatives").unwrap_or_else(|| panic!("the alternatives section: {body}"));
        assert_eq!(section["component"]["type"].as_str(), Some("treeSection"));
        let trunk_row = find_node(section, &row_key(&trunk)).expect("the trunk row");
        assert!(trunk_row.to_string().contains(main_line), "{locale:?}: {trunk_row}");
        let action = &trunk_row["component"]["rowActions"][0];
        assert_eq!((action["verb"].as_str(), action["label"].as_str()), (Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID), Some(switch)), "{locale:?}: {trunk_row}");
        assert_eq!(trunk_row["component"]["target"]["activation"].as_str(), Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID), "Enter on the row switches");
        let variant_row = find_node(section, &row_key(&variant)).expect("the variant row");
        assert!(variant_row.to_string().contains(current) && variant_row.to_string().contains(edited) && variant_row.to_string().contains(&actor), "{locale:?}: {variant_row}");
        assert!(variant_row["component"]["rowActions"].as_array().is_none_or(Vec::is_empty), "the current line offers no switch");
    }
    let body = render_history(&mut app, Locale::En).await;
    let args = find_node(&body, &row_key(&trunk)).expect("the trunk row")["component"]["target"]["args"].clone();
    assert_eq!(args[semio_framework::SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID].as_str(), Some(trunk.as_str()));
    history_verb(&mut app, &actor, semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, Some(dsl(&args)), mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "a", "the trunk shows the original history");
    let history = app.render_ctx().test_history().await;
    assert_eq!(history.alternatives.iter().map(|alternative| alternative.current).collect::<Vec<_>>(), vec![true, false]);
    history_verb(&mut app, &actor, semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, Some(dsl(&serde_json::json!({ "alternativeId": variant }))), mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "b", "switching back applies the history edit again");
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 📨️ LAW: opening a history edit delivers `TimeTravelFrozen` to every window of the view once (retargeting the open
/// session delivers none), and a remote edit moving the base delivers `BaseMoved` to every window of the last roster.
#[semio_framework_async_macros::async_test]
async fn opening_a_history_edit_and_a_remote_edit_deliver_host_events_to_every_window() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "host-events-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "host-events-remote", "remote", false, mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let roster = vec![semio_framework::ViewWindowInstance { id: "pane-a".into(), window_kind_id: "main".into() }, semio_framework::ViewWindowInstance { id: "pane-b".into(), window_kind_id: "main".into() }];
    let under = ActionMeta { view_state: Some(ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }), ..artifact_app_laws::meta(&actor) };
    let every = |event: fn(String) -> HostEvent| vec![event("pane-a".into()), event("pane-b".into())];
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    for index in [0, 1] {
        let mutation = seeded_mutation(&local, index);
        let begun = local.handle_action("historyEditBegin", Some(&DslValue::Object(vec![("mutationId".into(), DslValue::String(mutation))])), &under, &mut identity).await.expect("begin");
        assert_eq!(rejected(&begun), None);
    }
    assert_eq!(TOY_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut())), every(|window_id| HostEvent::TimeTravelFrozen { window_id }), "only opening the session freezes the windows");
    local.handle_action("historyEditExit", None, &under, &mut identity).await.expect("exit");
    pump_until(&mut local, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }, &mut identity).await.expect("a remote edit");
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    relay(&mut remote_probe, &mut local_probe, &mut local, mounted_policy, &mut identity).await;
    assert_eq!(TOY_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut())), every(|window_id| HostEvent::BaseMoved { window_id }), "the remote edit moved the base");
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
