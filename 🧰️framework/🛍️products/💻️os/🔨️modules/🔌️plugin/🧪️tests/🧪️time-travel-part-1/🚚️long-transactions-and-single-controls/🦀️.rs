/// 🚚️ Long transactions and single controls: the history window render, reachable mutations of a long transaction, the streamed tool transaction row and the band and editor single controls.

/// 🪟️ The history body as a host shows it with history row `seq`'s mutation window open on `rows` rows from `offset`.
async fn render_history_window(app: &mut ToyApp, seq: u64, offset: u32, rows: u32) -> Value {
    let request = semio_framework::TreeWindowRequest { body_key: FRAMEWORK_HISTORY_BODY_KEY.into(), node_key: time_travel::history_row_window_path(seq), open: Some(true), offset, rows };
    let view = ViewModel { tree_windows: vec![request], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let tree = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &view).await.unwrap_or_else(|fault| panic!("history render: {fault:?}"));
    serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).expect("history projection")).expect("history projection parses")
}

/// 🧒️ The mutation ids of the mutation rows a rendered history row materialised, in row order.
fn mutation_row_ids(row: &Value) -> Vec<String> {
    row["children"].as_array().into_iter().flatten().filter_map(|child| child["key"].as_str()?.strip_prefix("framework.history.mutation.").map(str::to_string)).collect()
}

/// ⚖️ LAW (gaps N1, N15): every mutation of a long transaction is reachable from its history row — the row is a tree window
/// over all of them (the flagged one first, then the projected ones, then the rest of its edit read on demand) while the
/// wire stays bounded; a row holding a warning opens by default with the warning first, a row without one stays closed,
/// and a window opened past the projection shows the 33rd…40th mutations with their Edit action, which opens the editor on
/// them. The Edit action is disabled, naming why and activating nothing, exactly where `Begin` is refused: a changed draft
/// still open, a running replay; it is enabled again in the review.
#[semio_framework_async_macros::async_test]
async fn every_mutation_of_a_long_transaction_is_reachable_and_edit_follows_the_begin_law() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let mut ops: Vec<TestMutation> = (1..40).map(|value| SetCount { value }.into()).collect();
    ops.push(SetLabel { value: head(&app).1 }.into());
    let transaction = protocol::TransactionRef { id: "tx-0000000000000040".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), ops), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the long transaction publishes");
    let row = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().find(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == transaction.id)).expect("the transaction row");
    let ids: Vec<String> = app.stores.store.mutation_ops().expect("applied operations").iter().filter(|op| row.edit_id.as_deref() == Some(op.edit_id)).map(|op| op.mutation_id.0.clone()).collect();
    assert_eq!((row.op_count, ids.len()), (40, 40), "one edit of forty operations");
    assert!(
        row.mutations.len() < 40 && row.mutations.iter().any(|mutation| mutation.mutation_id == ids[39] && mutation.worst == Some(semio_framework_diagnostic::Severity::Warning)),
        "the wire stays bounded and keeps the flagged no-op: {}",
        row.mutations.len()
    );
    let key = format!("framework.history.entry.{}", row.seq);

    let unrequested = render_history(&mut app, Locale::En).await;
    let opened = find_node(&unrequested, &key).unwrap_or_else(|| panic!("the transaction row: {unrequested}"));
    assert_eq!((opened["component"]["window"]["total"].as_u64(), mutation_row_ids(opened).first()), (Some(40), Some(&ids[39])), "a row holding a warning opens by default, the warning first, announcing all forty: {opened}");
    let seed_row = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().find(|entry| entry.edit_id.is_some() && entry.transaction.is_none() && !entry.mutations.is_empty()).expect("a seed row");
    assert!(mutation_row_ids(find_node(&unrequested, &format!("framework.history.entry.{}", seed_row.seq)).expect("the seed row")).is_empty(), "a row without an outcome stays closed");
    let first = render_history_window(&mut app, row.seq, 0, 8).await;
    assert_eq!(mutation_row_ids(find_node(&first, &key).expect("the open row")), [&ids[39..], &ids[..7]].concat(), "the flagged mutation leads, then op order");
    let last = render_history_window(&mut app, row.seq, 32, 8).await;
    let tail = find_node(&last, &key).expect("the row opened past its projection");
    assert_eq!(mutation_row_ids(tail), ids[31..39].to_vec(), "the 33rd to 40th rows: the last projected one, then the rest of the edit");
    let edited = find_node(tail, &format!("framework.history.mutation.{}", ids[38])).expect("the 39th mutation row");
    assert_eq!(
        (edited["component"]["rowActions"][0]["verb"].as_str(), edited["component"]["target"]["args"]["mutationId"].as_str(), edited["component"]["target"]["activation"].as_str()),
        (Some("historyEditBegin"), Some(ids[38].as_str()), Some("historyEditBegin")),
        "{edited}"
    );

    let begun = app.handle_action("historyEditBegin", Some(&dsl(&edited["component"]["target"]["args"])), &meta(&fixture), &mut identity).await.expect("Edit dispatches");
    assert_eq!(rejected(&begun), None, "{:?}", begun.output);
    assert_eq!(app.time_travel.editor().map(|editor| editor.target.0.clone()), Some(ids[38].clone()), "Edit opens the 39th mutation");
    let edit_action = |body: &Value, id: &str| {
        find_node(body, &format!("framework.history.mutation.{id}")).map(|row| (row["component"]["rowActions"][0].clone(), row["component"]["target"]["activation"].clone())).unwrap_or_else(|| panic!("the row of {id}: {body}"))
    };
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] != Value::Bool(true) && activation.as_str() == Some("historyEditBegin"), "an unchanged draft lets Edit switch targets: {action}");
    verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(77))], mounted_policy, &mut identity).await;
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(
        action["disabled"] == Value::Bool(true) && activation.is_null() && action["label"].as_str() == Some("Edit") && action["reason"].as_str().is_some_and(|reason| reason.starts_with("Blocked")),
        "a changed draft blocks Edit, naming why: {action}"
    );
    verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] == Value::Bool(true) && activation.is_null() && action["label"].as_str() == Some("Edit") && action["reason"].as_str() == Some("Not possible right now"), "a running replay disables Edit, naming why: {action}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] != Value::Bool(true) && activation.as_str() == Some("historyEditBegin") && action["label"].as_str() == Some("Edit"), "the review edits again: {action}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🌊️ LAW (design §15, transaction-scoped amend): a streamed tool transaction's ticks grow ONE edit and ONE history row
/// carrying its ref; while it is open a plain edit is refused `toolTransaction.open` and time travel `timeTravel.busy`; an
/// empty commit closes it with every op stamped; an aborted one leaves no edit, no row and the revision of before; streams
/// naming no transaction and aborts carrying mutations break the shape rule.
#[semio_framework_async_macros::async_test]
async fn a_streamed_tool_transaction_is_one_row_and_its_abort_leaves_none() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    type ToyEmit = Emit<TestMutation, TestConfigMutation, NoDraftMutation>;
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let edits = app.stores.store.envelope().vcs.edits.len();
    let streamed = protocol::TransactionRef { id: "tx-00000000000000aa".into(), tool: format!("{}#import", ToyHistoryApp::APP_ID) };
    for value in [2, 3, 4] {
        app.dispatch_emit("select", ToyEmit::stream_transaction(streamed.clone(), vec![SetCount { value }.into()]), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("a tick streams");
    }
    assert_eq!((app.stores.store.envelope().vcs.edits.len(), app.stores.store.open_transaction().map(|open| open.transaction.clone())), (edits + 1, Some(streamed.clone())), "the ticks grow one open edit");
    let plain = app.dispatch_emit("select", ToyEmit::mutations(vec![SetCount { value: 9 }.into()]), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    assert_eq!(plain.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.open")), "a plain edit waits for the open transaction");
    let first = seeded_mutation(&app, 0);
    let busy = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&busy), Some("timeTravel.busy"), "time travel waits for the open transaction");
    app.dispatch_emit("select", ToyEmit::commit_transaction(streamed.clone(), Vec::new()), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("an empty commit closes the edit");
    assert!(app.stores.store.open_transaction().is_none());
    let tail = app.stores.store.envelope().vcs.edits.last().expect("the committed edit");
    assert!(tail.forwards.len() == 3 && tail.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&streamed)), "one edit, every op stamped");
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    let rows: Vec<_> = patch.upserts.iter().filter(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == streamed.id)).collect();
    assert_eq!((rows.len(), rows.first().map(|row| row.mutations.len())), (1, Some(3)), "one row with one mutation row per op");
    let (edits, revision) = (app.stores.store.envelope().vcs.edits.len(), app.stores.store.content_revision_now());
    let aborted = protocol::TransactionRef { id: "tx-00000000000000bb".into(), tool: streamed.tool.clone() };
    for value in [5, 6] {
        app.dispatch_emit("select", ToyEmit::stream_transaction(aborted.clone(), vec![SetCount { value }.into()]), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("a tick streams");
    }
    app.dispatch_emit("select", ToyEmit::abort_transaction(aborted.clone()), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the abort reverts the open edit");
    assert_eq!((app.stores.store.envelope().vcs.edits.len(), app.stores.store.content_revision_now(), app.stores.store.open_transaction().is_none()), (edits, revision, true), "zero trace");
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    let held: HashSet<&str> = app.stores.store.envelope().vcs.edits.iter().map(|edit| edit.id.as_str()).collect();
    assert!(patch.upserts.iter().all(|entry| entry.transaction.as_ref().is_none_or(|reference| reference.id != aborted.id) && entry.edit_id.as_deref().is_none_or(|edit_id| held.contains(edit_id))), "no row of the aborted transaction remains");
    let unnamed = ToyEmit { transaction: None, transaction_phase: TransactionPhase::Stream, ..ToyEmit::mutations(vec![SetCount { value: 7 }.into()]) };
    assert_eq!(app.dispatch_emit("select", unnamed, &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.shape")));
    let carrying = ToyEmit { artifact_mutations: vec![SetCount { value: 8 }.into()], ..ToyEmit::abort_transaction(aborted) };
    assert_eq!(app.dispatch_emit("select", carrying, &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.shape")));
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🌲️ LAW: the band and the draft editor speak the one tree vocabulary both hosts mount — tree sections of rows, each row
/// holding at most one child that is not a row, and that child one single control (the kinds a wgpu row mounts as its
/// control; React mounts any): no text, progress bar or group is a row's child, and a second control is a nested row.
#[semio_framework_async_macros::async_test]
async fn the_band_and_the_editor_rows_hold_at_most_one_single_control() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    const SINGLE_CONTROLS: [&str; 9] = ["input", "select", "toggle", "button", "keyValueList", "slider", "numberStepper", "ring", "iconSelect"];
    fn violations(node: &Value, found: &mut Vec<String>) {
        let children = node["children"].as_array().map(Vec::as_slice).unwrap_or_default();
        if node["component"]["type"].as_str() == Some("treeItem") {
            let controls: Vec<&str> = children.iter().filter_map(|child| child["component"]["type"].as_str()).filter(|kind| *kind != "treeItem").collect();
            if controls.len() > 1 || controls.iter().any(|kind| !SINGLE_CONTROLS.contains(kind)) {
                found.push(format!("{}: {controls:?}", node["key"]));
            }
        }
        for child in children {
            violations(child, found);
        }
    }
    fn sections(history: &Value) -> Vec<String> {
        let mut found = Vec::new();
        for key in ["framework.history.timeTravel", "framework.history.editor", "framework.history.editor.inputs"] {
            if let Some(section) = find_node(history, key) {
                assert_eq!(section["component"]["type"].as_str(), Some("treeSection"), "{key} is a tree section");
                violations(section, &mut found);
            }
        }
        found
    }
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs =
        vec![semio_framework::ActionArgDef { nullable: true, ..semio_framework::ActionArgDef::number("/scale", LocalizedLabel::native("Scale", "Maßstab")) }, semio_framework::ActionArgDef::number("/width", LocalizedLabel::native("Width", "Breite"))];
    editor.value = dsl(&serde_json::json!({ "scale": 2.0, "width": 1.0 }));
    let history = render_history(&mut app, Locale::En).await;
    assert!(find_node(&history, "framework.history.editor.input.scale.clear").is_some(), "the nullable input keeps its Clear: {history}");
    assert_eq!(sections(&history), Vec::<String>::new(), "editing rows");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let history = render_history(&mut app, Locale::De).await;
    assert!(find_node(&history, "framework.history.timeTravel.finalize").is_some(), "the review offers Finalize: {history}");
    assert_eq!(sections(&history), Vec::<String>::new(), "reviewing rows");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
