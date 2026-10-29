// 🪣️ Shared schemas, filters and backbone: shared framework schema inputs, history filter options, long option rows, backbone attach and detach while editing and mutation refusal reasons.

//#region 🪣️SharedSchemaDocuments
/// 🪣️ A payload schema whose one input references the framework value schema (its numeric transport).
const SHARED_REFERENCE_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["opacity"],"properties":{"opacity":{"$ref":"https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport","x-semio-ui":{"label":{"en":"Opacity","de":"Deckkraft"}}}}}"#;

/// ⚖️ LAW (design §23): the framework's own shared schema documents are in every instance's input-schema resolver — a
/// registryless instance, for which no plugin assembly published anything, holds the framework value schema, the io
/// vocabulary and the store's document model (child, owner, link, blob); an input that references the value schema reads
/// as one input and the draft editor renders its control.
#[semio_framework_async_macros::async_test]
async fn an_input_that_references_a_shared_framework_schema_resolves_on_any_instance() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for id in ["framework/value/schema.json", "framework/io/schema.json", "framework/schema/artifact-reference/child/schema.json", "os/store/child/owner/schema.json", "framework/schema/artifact-reference/link/schema.json", "framework/schema/artifact-reference/blob/schema.json"] {
        let id = format!("https://json.schemas.assets.semio-tech.com/{id}");
        assert!(semio_framework::registered_input_schema_document(&id).is_some(), "{id} is always in the runtime resolver");
    }
    let inputs = semio_framework::mutation_input_defs(SHARED_REFERENCE_SCHEMA, &semio_framework::registered_input_schema_document).unwrap_or_else(|error| panic!("the reference resolves: {error:?}"));
    assert_eq!(inputs.len(), 1, "the referenced value is one input: {inputs:?}");
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = inputs;
    editor.value = dsl(&serde_json::json!({ "opacity": 0.5 }));
    let history = render_history(&mut app, Locale::De).await;
    let row = find_node(&history, "framework.history.editor.input.opacity").unwrap_or_else(|| panic!("the input's control: {history}"));
    assert!(history.to_string().contains("Deckkraft"), "the control carries the input's own label: {row}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🪣️SharedSchemaDocuments

//#region 🎚️HistoryFilter
/// ⚖️ LAW (schema-first, one vocabulary): every option `setHistoryCommandFilter` declares is a value its dispatch accepts and
/// the history wire echoes as `commandFilter`, the history body's select offers exactly those values, and an undeclared value
/// is refused instead of silently showing every row.
#[semio_framework_async_macros::async_test]
async fn every_declared_history_filter_option_is_dispatched_and_echoed() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let definition = semio_framework::set_history_command_filter_action_definition();
    let declared: Vec<String> = match &definition.args[0].schema {
        semio_framework::ArgSchema::String { options, .. } => options.iter().map(|option| option.value.clone()).collect(),
        other => panic!("the filter is a choice: {other:?}"),
    };
    assert_eq!(declared, HistoryCommandFilter::ALL.map(|filter| filter.value().to_string()), "the declared options are the runtime's filters, in order");
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(&actor) };
    for value in &declared {
        let args = DslValue::Object(vec![("value".into(), DslValue::String(value.clone()))]);
        let admitted = app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&args), &meta, &mut identity).await.unwrap_or_else(|fault| panic!("{value}: {fault:?}"));
        semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut app, admitted, cx).await.unwrap_or_else(|fault| panic!("{value}: {fault:?}"));
        assert_eq!(&app.render_ctx().history_patch(true).await.expect("history patch").command_filter, value, "the wire echoes the declared option");
        let body = render_history(&mut app, Locale::En).await;
        let select = find_node(&body, "framework.history.filter.control").expect("the filter select");
        for option in &declared {
            assert!(select.to_string().contains(&format!("\"{option}\"")), "the select offers {option}: {select}");
        }
    }
    let undeclared = DslValue::Object(vec![("value".into(), DslValue::String("withoutOperations".into()))]);
    let refused = match app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&undeclared), &meta, &mut identity).await {
        Ok(admitted) => semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut app, admitted, cx).await.err(),
        Err(fault) => Some(fault),
    };
    assert!(refused.is_some(), "an undeclared filter value is refused");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🎚️HistoryFilter


/// ☑️ LAW (audit W1E-2, conformance `💬️row-semantics` `selectedRows`): a choice with more options than one fixed list holds is
/// windowed option rows, each a tree row holding its choose button and stating its choice as `selected` — the chosen option
/// `Some(true)`, every other `Some(false)` — so both renderers announce (`aria-selected`, the mirror's `selected`) and paint it,
/// never by its icon alone.
#[test]
fn long_option_rows_state_their_choice_as_selected() {
    let option = |index: usize| semio_framework::ActionArgOption { value: format!("o{index}"), label: LocalizedLabel::native(&format!("Option {index}"), &format!("Option {index}")) };
    let mode = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: (0..40).map(option).collect(), option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/mode", LocalizedLabel::native("Mode", "Modus"))
    };
    let value = dsl(&serde_json::json!({ "mode": "o3" }));
    let editor = TimeTravelEditorPanel { target: "m-1".into(), label: LocalizedLabel::native("Set mode", "Modus setzen"), rows: time_travel::time_travel_input_rows(&[mode], &value), editable: true, inputs_refused: None, withdrawn: false, outcome: Vec::new(), refused: None, changed: false, reference_labels: Default::default() };
    let panel = TimeTravelPanel { status: Default::default(), store: None, stage: TimeTravelStage::Editing, pending_after: None, finalize_refusal: None, review: None, rerun_refusal: None, begin_refusal: None, next_problem: None, editor: Some(editor.clone()), outcomes: Default::default(), edited: Default::default(), accepted: Default::default() };
    let sections = time_travel::time_travel_editor_sections(&panel, &editor, &TreeWindows::unhosted(), "toy", Locale::En).expect("the editor builds");
    fn option_rows(node: &BuiltNode, rows: &mut Vec<(String, Option<bool>)>) {
        if let semio_framework_ui_contract::Component::TreeItem(props) = &node.component {
            if node.key.as_str().contains(".option.") && node.key.as_str().ends_with(".row") {
                rows.push((node.key.as_str().to_string(), props.selected));
            }
        }
        node.children.iter().for_each(|child| option_rows(child, rows));
    }
    let mut rows = Vec::new();
    sections.iter().for_each(|section| option_rows(section, &mut rows));
    assert!(rows.len() >= 4 && rows.iter().any(|(_, selected)| *selected == Some(true)), "the first paint holds the chosen option among several: {rows:?}");
    for (key, selected) in &rows {
        assert_eq!(*selected, Some(key == "framework.history.editor.input.mode.option.3.row"), "{key}");
    }
}

/// ⚖️ LAW (design §22.24, live fault F4): the base a session watches is the content revision of its store, never the
/// store's local generation. A backbone attaching and detaching while a draft is edited moves that generation and no
/// event: driver turns deliver no base move — the stage, the session generation, the base and the draft are what they
/// were — and an input stamped with the generation from before is accepted, not stale.
#[semio_framework_async_macros::async_test]
async fn a_backbone_attach_and_detach_while_editing_is_no_base_move() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let before = app.time_travel.session().clone();
    let (store_generation, revision) = (app.stores.store.generation(), app.stores.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-editing", "attach-while-editing").await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.stores.store.generation() > store_generation && app.stores.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("a driver turn");
        collect_original_history_command_receipt(&mut app);
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the stage, the generation, the base and the draft are what they were");
    let typed = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(before.generation)))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&typed), None, "an input stamped before the port change is accepted");
    assert!(render_body(&mut app).await.contains("count=1 label=c"), "the preview follows the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §22.24, store wave RB): a replay finished before a port change still commits. A backbone attaching and
/// detaching while the session reviews its finished replay moves the store's local generation and no event: driver turns
/// deliver no base move and start no second replay, and Finalize → Overwrite commits the head reviewed — no
/// `timeTravel.stale`, the session closes with its commit.
#[semio_framework_async_macros::async_test]
async fn a_replay_finished_before_a_backbone_attach_and_detach_still_commits() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        if let Some(result) = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await {
            assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
        }
    }
    let before = app.time_travel.session().clone();
    assert_eq!(before.stage, TimeTravelStage::Reviewing, "the replay finished");
    let (store_generation, revision) = (app.stores.store.generation(), app.stores.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-reviewing", "attach-while-reviewing").await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.stores.store.generation() > store_generation && app.stores.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("a driver turn");
        collect_original_history_command_receipt(&mut app);
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the review stands and no second replay started");
    for step in [serde_json::json!({ "finalize": null }), serde_json::json!({ "commit": { "choice": "overwrite" } })] {
        let result = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: the replay finished before the port change still commits: {:?}", result.output);
    }
    pump_until(&mut app, "the finalize retires", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    assert!(app.time_travel.status().is_none(), "the session closed with its commit: {:?}", app.time_travel.session().stage);
    assert_eq!(head(&app), (5, "b".to_string()), "the committed document is the head reviewed");
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "every window shows the committed edit");
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §22.1, §22.20): a mutation row always names Edit and, unless it is withdrawn, Withdraw — or Restore on a
/// mutation holding an accepted draft. A mutation without editable inputs keeps Withdraw and reads why Edit is refused, a
/// mutation the store's supersede law lets nobody withdraw reads that, a viewer reads why both are refused, in both
/// locales; only an offered Edit is the row's activation.
#[test]
fn a_mutation_row_names_why_edit_or_withdraw_is_refused() {
    let entry = |editable: bool, withdrawable: bool, withdrawn: bool| semio_framework::kernel::HistoryMutationEntry {
        mutation_id: "e-1#0".into(),
        position: 0,
        op_index: 0,
        label: LocalizedLabel::native("Set label", "Beschriftung setzen"),
        worst: None,
        messages: Vec::new(),
        superseded: withdrawn,
        withdrawn,
        editable,
        withdrawable,
        pending: false,
        edited: false,
        introduced: false,
        store: None,
    };
    let row = |entry: &semio_framework::kernel::HistoryMutationEntry, locale: Locale, read_only: bool, refusals: time_travel::MutationRowRefusals| {
        let row = history_panel_mutation_row(entry, "toy", locale, read_only, refusals).expect("the row builds");
        let semio_framework_ui_contract::Component::TreeItem(props) = &row.component else { panic!("a mutation row is a tree row") };
        let actions: Vec<(String, Option<String>)> = props.row_actions.iter().map(|action| (action.verb.as_str().to_string(), action.reason.as_ref().map(|reason| reason.0.as_str().to_string()))).collect();
        assert!(props.row_actions.iter().all(|action| action.disabled == action.reason.is_some()), "a disabled action names why: {actions:?}");
        (actions, props.target.as_ref().and_then(|target| target.activation.as_ref().map(|verb| verb.as_str().to_string())))
    };
    let offered = |verb: &str| (verb.to_string(), None);
    let refused = |verb: &str, reason: &str| (verb.to_string(), Some(reason.to_string()));
    let idle = time_travel::MutationRowRefusals::default();
    assert_eq!(row(&entry(true, true, false), Locale::En, false, idle), (vec![offered("historyEditBegin"), offered("historyEditWithdraw")], Some("historyEditBegin".to_string())), "an editable mutation");
    assert_eq!(
        row(&entry(false, true, false), Locale::En, false, idle),
        (vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditWithdraw")], None),
        "a mutation without editable inputs keeps Withdraw and activates nothing"
    );
    assert_eq!(
        row(&entry(false, true, false), Locale::De, false, idle).0,
        vec![refused("historyEditBegin", "Die Eingaben dieser Mutation können nicht bearbeitet werden"), offered("historyEditWithdraw")],
        "the reason is the reader's language"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, false, idle).0,
        vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), refused("historyEditWithdraw", "This mutation cannot be withdrawn here")],
        "the store's supersede law refuses the withdrawal"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, true, idle),
        (vec![refused("historyEditBegin", "History cannot be edited in a read-only view"), refused("historyEditWithdraw", "History cannot be edited in a read-only view")], None),
        "a viewer reads why"
    );
    assert_eq!(row(&entry(true, false, true), Locale::En, false, idle).0, vec![offered("historyEditBegin")], "a withdrawn mutation offers no second withdrawal");
    let accepted = time_travel::MutationRowRefusals { accepted: true, ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, accepted).0, vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditRestore")], "an accepted draft is restored from its row");
    let replaying = time_travel::MutationRowRefusals { accepted: true, restore: Some(semio_framework_time_travel::TimeTravelLabel::RefusalIllegal), ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, replaying).0[1], refused("historyEditRestore", "Not possible right now"), "outside the review Restore names why");
}
