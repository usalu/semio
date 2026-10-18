// 📖️ Reader windows and panel fixtures: reader window refresh, concurrent read-only runs in two windows and the running panel fixture with its trace key and start offers.

//#region 🎯️RetargetAndSettings
/// ⚖️ LAW: a tick dirties the bodies of the window kinds the run declares it renders in, next to the panel and the scene
/// windows, and those bodies read the run's progress, step ring and latest tick payload through `ArtifactView::tool_run()`.
#[semio_framework_async_macros::async_test]
async fn tool_run_reader_windows_refresh_every_tick_and_read_progress_steps_and_payload() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["readerWindows"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    render_world(&mut app, None, &mut caller).await;
    app.tool_run_ctx().flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let before = app.tool_runs.provisional().len();
    pump_until(&mut app, "a tick lands", |app| app.tool_runs.provisional().len() > before, &mut caller).await;
    let scope = app.take_typed_operation_ui_scope().expect("a tick owes a dirty scope");
    let strings = |value: &Value| value.as_array().expect("strings").iter().map(|item| text(item).to_string()).collect::<Vec<_>>();
    assert_eq!(
        scope,
        UiDirtyScope::Partial { window_bodies: strings(&expected["windowBodies"]), panel_bodies: strings(&expected["panelBodies"]), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        "the reader window refreshes with every tick, nothing unrelated does"
    );
    run_action(&mut app, "toolRunPause", &mut caller).await;
    pump_until(&mut app, "paused and settled", |app| app.tool_runs.state() == Some(ToolRunState::Paused) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let view = app.tool_runs.view().expect("run view");
    let payload = view.payload.as_deref().map(Vec::as_slice).and_then(|payload| payload.try_into().ok()).map(u32::from_le_bytes).expect("the latest tick payload");
    assert_eq!(u64::from(payload), view.progress.completed, "the payload is the latest tick's: the toy writes its completed units");
    assert!(view.progress.total.is_some_and(|total| total == number(&expected["target"])));
    let body = render_text(&mut app, "main", &mut caller).await;
    println!("[STATS] reader body {body}");
    assert!(body.contains(&format!("completed={} steps={} payload={payload}", view.progress.completed, view.progress.steps.len())), "the reader body renders the run state it reads: {body}");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ▶️ Starts `tool_id` from window `window_id` and answers the dispatch output.
async fn start_in_window(app: &mut ToyApp, tool_id: &str, window_id: &str, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> DslValue {
    let mut meta = toy_meta();
    meta.view_state = Some(window_view(window_id, None));
    app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(tool_id.into()))])), &meta, &mut caller.identity).await.unwrap_or_else(|fault| panic!("start {tool_id}: {fault:?}")).output
}

/// ⚖️ LAW: read-only runs run concurrently, one per (tool, window), each with its own run id, trace and panel group; a
/// second mutating start while a mutating run is non-terminal is `toolRun.busy`; actions address a run by its `runId`;
/// presence follows the mutating run.
#[semio_framework_async_macros::async_test]
async fn read_only_runs_in_two_windows_run_concurrently_and_a_second_mutating_start_is_busy() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["concurrentReadOnly"];
    let read_only = text(&expected["readOnlyToolId"]);
    let windows: Vec<&str> = expected["windows"].as_array().expect("windows").iter().map(text).collect();
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    for window in &windows {
        assert_eq!(start_in_window(&mut app, read_only, window, &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "{window}: a read-only run starts beside the other window's");
    }
    assert_eq!(start_in_window(&mut app, read_only, windows[0], &mut caller).await.get("rejected").and_then(DslValue::as_str), Some(text(&expected["busy"])), "one non-terminal read-only run per tool and window");
    pump_until(&mut app, "both read-only runs are running and waiting on their hop", |app| {
        let views = app.tool_runs.views();
        views.len() == 2 && views.iter().all(|view| view.state == ToolRunState::Running)
    }, &mut caller)
    .await;
    let views = app.tool_runs.views();
    assert_ne!(views[0].identity.id.run, views[1].identity.id.run, "each run has its own id");
    for (window, view) in windows.iter().zip(&views) {
        assert_eq!(app.tool_runs.view_for(Some(window)).map(|found| found.identity.id.run), Some(view.identity.id.run), "{window} renders its own run");
    }
    assert_eq!(start_in_window(&mut app, text(&expected["mutatingToolId"]), windows[0], &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "a mutating run starts beside read-only runs");
    assert_eq!(start_in_window(&mut app, text(&expected["secondMutatingToolId"]), windows[1], &mut caller).await.get("rejected").and_then(DslValue::as_str), Some(text(&expected["busy"])), "a second mutating start is busy");
    assert_eq!(app.tool_run_presence().map(|presence| presence.tool_id), Some(text(&expected["mutatingToolId"]).to_string()), "presence follows the mutating run");
    let panel: Value = serde_json::from_str(&render_text(&mut app, FRAMEWORK_TOOL_RUN_BODY_KEY, &mut caller).await).expect("panel parses");
    for view in app.tool_runs.views() {
        assert!(find_node(&panel, &panel_id(&fixture["panel"]["groupId"], view.identity.id.run)).is_some(), "run {} has its own panel group", view.identity.id.run);
        let delta = app
            .tool_runs
            .trace_delta(Some(ToolRunTraceCursor { run: view.identity.id.run, generation: view.identity.generation + 1, page: 0 }), usize::MAX)
            .map(|lane| ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(&lane).expect("base64url")).expect("delta"));
        assert_eq!(delta.map(|delta| delta.identity.id.run), Some(view.identity.id.run), "run {} has its own trace", view.identity.id.run);
    }
    let first = views[0].clone();
    let aborted = tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(first.identity.id.run.to_string())), ("generation".into(), DslValue::uint(u64::from(first.identity.generation)))], &mut caller).await;
    assert_eq!(aborted.get("toolRun").and_then(DslValue::as_str), Some("closeJob"), "an action addresses its run by runId");
    pump_until(&mut app, "the addressed run aborts alone", |app| app.tool_runs.views().iter().any(|view| view.identity.id.run == first.identity.id.run && view.state == ToolRunState::Aborted), &mut caller).await;
    let states: Vec<(u64, ToolRunState)> = app.tool_runs.views().iter().map(|view| (view.identity.id.run, view.state)).collect();
    println!("[STATS] concurrent runs {states:?}");
    assert!(states.iter().filter(|(run, _)| *run != first.identity.id.run).all(|(_, state)| !state.is_terminal()), "the other runs keep running: {states:?}");
    for view in app.tool_runs.views().into_iter().filter(|view| !view.state.is_terminal()) {
        tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(view.identity.id.run.to_string())), ("generation".into(), DslValue::uint(u64::from(view.identity.generation)))], &mut caller).await;
    }
    pump_until(&mut app, "every run settles", |app| app.tool_runs.views().iter().all(|view| view.state.is_terminal()) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧾️ A `BuiltNode` as the host wire JSON every renderer's `BuiltNode` reads — every field spelled out.
fn built_node_wire(node: &BuiltNode) -> Value {
    serde_json::json!({
        "key": node.key.as_str(),
        "component": serde_json::to_value(&node.component).expect("component"),
        "layout": serde_json::to_value(&node.layout).expect("layout"),
        "style": serde_json::to_value(node.style).expect("style"),
        "activity": serde_json::to_value(node.activity).expect("activity"),
        "disabled": node.disabled,
        "accessibility": serde_json::to_value(&node.accessibility).expect("accessibility"),
        "bindings": serde_json::to_value(&node.bindings).expect("bindings"),
        "menu": serde_json::to_value(&node.menu).expect("menu"),
        "children": node.children.iter().map(built_node_wire).collect::<Vec<_>>(),
    })
}

/// 🪧️ The language-neutral panel every shell target mounts: the framework ToolRun panel `BuiltNode` of a running run
/// (`🔌️plugin/🧫️fixtures/⏯️tool-run/🪧️panel-running.json`). React (`🛠️ShellHelpers` panel law) and wgpu (`🐚️Shell`
/// panel law) render it and dispatch its buttons from this exact document.
const TOOL_RUN_PANEL_RUNNING: &str = include_str!("../../🧫️fixtures/⏯️tool-run/🪧️panel-running.json");

/// ⚖️ LAW: the ToolRun panel of a running run is exactly the committed shell fixture — one run group with a polite status,
/// a progressbar, real buttons addressing the run by id (pause and abort enabled, step disabled, finalize disabled with
/// its description) and an empty step log and trace list. `SEMIO_TOOL_RUN_PANEL_OUT` rewrites the fixture.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_of_a_running_run_is_the_shell_fixture() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&fixture["port"]["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.state() == Some(ToolRunState::Running) && app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting), &mut caller).await;
    while app.take_typed_operation_effect().is_some() {}
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render the panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    if let Some(path) = std::env::var_os("SEMIO_TOOL_RUN_PANEL_OUT") {
        std::fs::write(path, format!("{}\n", serde_json::to_string_pretty(&panel).expect("pretty panel"))).expect("write the panel fixture");
    }
    assert_eq!(panel, serde_json::from_str::<Value>(TOOL_RUN_PANEL_RUNNING).expect("the panel fixture parses"), "the running panel is the shell fixture");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🔁️ Every sibling key list under `node` names each key once — the admission law a retained UI surface enforces.
fn assert_unique_sibling_keys(node: &Value) {
    let children = node["children"].as_array().map(Vec::as_slice).unwrap_or_default();
    let mut keys: Vec<&str> = children.iter().filter_map(|child| child["key"].as_str()).collect();
    keys.sort_unstable();
    let before = keys.len();
    keys.dedup();
    assert_eq!(keys.len(), before, "duplicate sibling keys under {}", node["key"]);
    children.iter().for_each(assert_unique_sibling_keys);
}

/// ⚖️ LAW: a trace key the run re-upserts with a new verdict (testing → success/danger, the shape of every
/// collision-tested candidate) is one attempt row, moved to the newest position — never a duplicate sibling that
/// makes the panel unadmittable.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_lists_a_re_upserted_trace_key_once_at_its_newest_position() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["verdictRevisions"];
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting), &mut caller).await;
    while app.take_typed_operation_effect().is_some() {}
    let identity = app.tool_runs.identity().expect("identity");
    let mut writer = ToolRunTickWriter::new(identity);
    for upsert in expected["upserts"].as_array().expect("upserts") {
        let key = number(&upsert[0]);
        writer.upsert(key, ToolRunVerdict::parse(text(&upsert[1])).expect("verdict"), 1, ToolRunTraceSubject::Entity { entity: key });
        app.tool_runs.apply_tick(writer.finish().expect("an upsert tick")).expect("the upsert tick applies");
    }
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render the panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert_unique_sibling_keys(&panel);
    let scope = semio_framework_tool_run::tool_run_panel_group_id(identity.id.run);
    let rows: Vec<&str> = find_node(&panel, &format!("{scope}.trace")).expect("the attempts tree")["children"].as_array().expect("rows").iter().filter_map(|row| row["key"].as_str()).collect();
    let expected_rows: Vec<String> = expected["traceRows"].as_array().expect("trace rows").iter().map(|key| format!("{scope}.trace.{}", number(key))).collect();
    assert_eq!(rows, expected_rows, "one attempt row per trace key, newest first");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: while the active tool declares a run and this instance holds none of it, the panel offers a ready group
/// whose enabled Start dispatches `toolRunStart` with the tool id; once the run exists its own group replaces it.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_offers_start_for_the_active_run_tool_until_its_run_exists() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["readyGroup"];
    let mut app = toy_app(1, &mut caller).await;
    let view = ViewModel { active_tool_id: Some(text(&expected["toolId"]).to_string()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &view).await.expect("render the idle panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert_unique_sibling_keys(&panel);
    let group = find_node(&panel, text(&expected["groupId"])).expect("the ready group");
    assert_eq!(group["children"][0]["component"]["value"], expected["status"], "the ready status");
    let start_button = find_node(group, text(&expected["startButtonId"])).expect("the ready Start button");
    assert_eq!(start_button["component"]["label"], expected["startLabel"]);
    assert_eq!(start_button["disabled"], false, "Start is enabled with no run");
    assert_eq!(start_button["bindings"][0]["action"]["name"], "toolRunStart");
    assert_eq!(start_button["bindings"][0]["args"]["toolId"], expected["toolId"], "Start names the active tool");
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting), &mut caller).await;
    while app.take_typed_operation_effect().is_some() {}
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &view).await.expect("render the running panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert!(find_node(&panel, text(&expected["groupId"])).is_none(), "the run's group replaces the ready group");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🎯️RetargetAndSettings
