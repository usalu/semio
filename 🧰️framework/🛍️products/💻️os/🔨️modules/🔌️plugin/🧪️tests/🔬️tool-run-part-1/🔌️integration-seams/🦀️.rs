// 🔌️ Integration seams: the scene render with the trace lane, the trace backlog, the tick dirty scope, presence and ephemeral snapshots, provisional entities and the job port.

//#region 🔌️IntegrationSeams
/// 🪟️ The per-window view state a renderer sends: both toy windows live, `window_id` targeted with its echoed cursor.
fn window_view(window_id: &str, cursor: Option<ToolRunTraceCursor>) -> ViewModel {
    let lane = &fixture()["traceLane"];
    let (world, other) = (text(&lane["windowId"]).to_string(), text(&lane["otherWindowId"]).to_string());
    ViewModel {
        window_id: Some(window_id.to_string()),
        window_instances: vec![ViewWindowInstance { id: world.clone(), window_kind_id: world }, ViewWindowInstance { id: other.clone(), window_kind_id: other }],
        tool_run_trace_cursor_by_window_id: cursor.map(|cursor| (window_id.to_string(), cursor)).into_iter().collect(),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

/// 🎬️ Renders the `world` body for `cursor` and returns its assembled scene plus the decoded trace delta.
async fn render_world(app: &mut ToyApp, cursor: Option<ToolRunTraceCursor>, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> (World3dScene, Option<ToolRunTraceDelta>) {
    let lane = &fixture()["traceLane"];
    let view=window_view(text(&lane["windowId"]),cursor);let mut tree=None;
    for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){tree=app.render(text(&lane["bodyKey"]),None,&view,caller.context).await.unwrap_or_else(|fault|panic!("render world: {fault:?}"));if tree.is_some(){break}}
    let tree=tree.expect("world render did not settle under the original fixture caller");
    let scene = artifact_app_laws::observe_and_retire_fixture_tree(tree, |root| {
        let surface = root.children.iter().find(|child| child.key.as_str() == text(&lane["surfaceId"])).expect("the world body carries its scene surface");
        artifact_app_laws::built_surface_scene::<World3dScene>(surface).expect("the scene surface assembles")
    });
    let delta = scene.tool_run_trace.as_deref().map(|lane| ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(lane).expect("the lane is base64url")).expect("the lane is one ToolRunTraceDelta"));
    (scene, delta)
}

fn cursor_after(delta: &ToolRunTraceDelta) -> ToolRunTraceCursor {
    ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next }
}

async fn abort_and_close(app: &mut ToyApp, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    run_action(app, "toolRunAbort", caller).await;
    pump_until(app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), caller).await;
    close(app, caller);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_scene_render_carries_the_trace_lane_and_honours_the_echoed_cursor() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["traceLane"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    let (idle, idle_delta) = render_world(&mut app, None, &mut caller).await;
    assert!(idle_delta.is_none() && idle.lanes.iter().all(|lane| lane.lane != text(&expected["laneName"])), "no run, no lane");
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "trace run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let store_pages = app.tool_runs.trace().expect("trace").next_page();
    assert!(store_pages > number(&expected["resumePage"]) as u32, "the run logged several trace pages ({store_pages})");
    let (scene, delta) = render_world(&mut app, None, &mut caller).await;
    let delta = delta.expect("a scene rendered during a run carries the toolRunTrace lane");
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((delta.identity.id.run, delta.identity.generation, delta.clear, delta.next), (slot.run, slot.generation, true, store_pages), "a renderer without a cursor gets clear plus the whole log");
    assert_eq!(delta.pages.iter().map(|page| page.ops.len()).sum::<usize>() as u64, number(&expected["units"]), "every tested unit reaches the renderer");
    let reference = scene.lanes.iter().find(|lane| lane.lane == text(&expected["laneName"])).expect("the spine names the injected lane");
    let lane_text = scene.tool_run_trace.as_deref().expect("lane text");
    assert_eq!((reference.bytes as usize, reference.hash.as_str()), (lane_text.len(), semio_framework_ui_scene::scene_lane_hash(lane_text).as_str()), "the lane ref matches its carrier exactly as split_lanes would emit it");
    let (_, caught_up) = render_world(&mut app, Some(cursor_after(&delta)), &mut caller).await;
    assert!(caught_up.is_none(), "a caught-up cursor costs nothing");
    let resume = number(&expected["resumePage"]) as u32;
    let (_, resumed) = render_world(&mut app, Some(ToolRunTraceCursor { page: resume, ..cursor_after(&delta) }), &mut caller).await;
    let resumed = resumed.expect("a lagging cursor gets the pages after it");
    assert!(!resumed.clear && resumed.pages.first().map(|page| page.page) == Some(resume) && resumed.next == store_pages, "resume continues from the echoed page");
    let (_, rebound) = render_world(&mut app, Some(ToolRunTraceCursor { generation: slot.generation + 1, page: resume, ..cursor_after(&delta) }), &mut caller).await;
    let rebound = rebound.expect("a generation mismatch resends");
    assert!(rebound.clear && rebound.pages.first().map(|page| page.page) == Some(0), "a run/generation mismatch sends clear and resends from page 0");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(run_action(&mut app, "toolRunDismiss", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("clearTrace"));
    assert!(app.tool_runs.slot().is_none(), "a dismissed run leaves no slot");
    let (_, cleared) = render_world(&mut app, Some(cursor_after(&delta)), &mut caller).await;
    let cleared = cleared.expect("a dismissed run clears a renderer still holding pages");
    assert!(cleared.clear && cleared.pages.is_empty() && cleared.next == 0);
    let (_, empty) = render_world(&mut app, Some(cursor_after(&cleared)), &mut caller).await;
    assert!(empty.is_none(), "a cleared renderer at page 0 gets nothing");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_trace_backlog_keeps_the_scene_dirty_until_the_echoed_cursor_stalls() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["traceLane"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "trace run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    while app.take_typed_operation_ui_scope().is_some() {}
    app.tool_run_ctx().flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let (window, body) = (text(&expected["windowId"]), text(&expected["bodyKey"]));
    let budget = number(&expected["pageByteBudget"]) as usize;
    let pages = app.tool_runs.trace().expect("trace").next_page();
    let mut cursor = None;
    for delivered in 1..=pages {
        let lane = app.tool_runs.trace_lane(window, body, cursor, budget).expect("one page per refresh under a one-byte budget");
        let delta = ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(&lane).expect("base64url")).expect("delta");
        assert_eq!((delta.pages.len(), delta.next), (1, delivered), "the byte budget still delivers one page per refresh");
        assert_eq!(app.tool_runs.is_ui_dirty(), delivered < pages, "a backlog keeps the window dirty; the last page does not");
        app.tool_run_ctx().flush_tool_run_ui_dirty();
        while app.take_typed_operation_ui_scope().is_some() {}
        cursor = Some(cursor_after(&delta));
    }
    let lagging = Some(ToolRunTraceCursor { page: 0, ..cursor.expect("cursor") });
    let mut dirty_refreshes = 0;
    for _ in 0..number(&expected["stallRefreshes"]) * 2 {
        app.tool_runs.trace_lane(window, body, lagging, budget).expect("the backlog is resent");
        dirty_refreshes += u64::from(app.tool_runs.is_ui_dirty());
        app.tool_run_ctx().flush_tool_run_ui_dirty();
        while app.take_typed_operation_ui_scope().is_some() {}
    }
    assert_eq!(dirty_refreshes, number(&expected["stallRefreshes"]), "a renderer that never echoes costs a bounded number of refreshes");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_tick_dirty_scope_is_the_panel_plus_the_scene_windows_and_excludes_unrelated_windows() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["dirtyScope"];
    let mut app = toy_app(number(&fixture["abort"]["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    render_world(&mut app, None, &mut caller).await;
    let other = text(&fixture["traceLane"]["otherWindowId"]);
    let tree = app.render(text(&expected["excludedWindowBody"]), None, &window_view(other, None)).await.expect("render the unrelated window");
    artifact_app_laws::project_and_retire_fixture_tree(tree).expect("project the unrelated window");
    app.tool_run_ctx().flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let before = app.tool_runs.provisional().len();
    pump_until(&mut app, "a tick lands", |app| app.tool_runs.provisional().len() > before, &mut caller).await;
    let scope = app.take_typed_operation_ui_scope().expect("a tick owes a dirty scope");
    let strings = |value: &Value| value.as_array().expect("strings").iter().map(|item| text(item).to_string()).collect::<Vec<_>>();
    assert_eq!(
        scope,
        UiDirtyScope::Partial { window_bodies: strings(&expected["windowBodies"]), panel_bodies: strings(&expected["panelBodies"]), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        "never the whole UI per tick"
    );
    let UiDirtyScope::Partial { window_bodies, .. } = &scope else { unreachable!() };
    assert!(!window_bodies.iter().any(|body| body == text(&expected["excludedWindowBody"])), "a window without a scene surface is not dirtied by a tick");
    let pause = run_action(&mut app, "toolRunPause", &mut caller).await;
    assert_eq!(pause.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_presence_and_the_ephemeral_snapshot_follow_the_run_with_completed_never_above_total() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["presence"];
    let reconfigure = &fixture["reconfigure"];
    let mut app = toy_app(number(&fixture["abort"]["target"]), &mut caller).await;
    assert!(app.tool_run_presence().is_none() && app.ephemeral_snapshot().await.tool_run.is_none(), "no run, no presence summary");
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "running", |app| app.tool_runs.state() == Some(ToolRunState::Running) && app.tool_runs.progress().is_some_and(|progress| progress.completed > 0), &mut caller).await;
    let running = app.tool_run_presence().expect("running presence");
    assert_eq!((running.tool_id.as_str(), running.state.wire_name()), (text(&fixture["toolId"]), text(&expected["running"])));
    assert!(running.total.is_none_or(|total| running.completed <= total));
    assert_eq!(app.ephemeral_snapshot().await.tool_run, Some(running), "the ephemeral snapshot carries the summary the heartbeat publishes");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(app.tool_run_presence().map(|presence| presence.state.wire_name()), Some(text(&expected["aborted"])));
    close(&mut app, &mut caller);
    let mut app = toy_app(number(&reconfigure["raisedTarget"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "raised run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    set_target(&mut app, number(&reconfigure["loweredTarget"]), &mut caller).await;
    let mut clamped = true;
    pump_until(&mut app, "lowered run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&reconfigure["loweredTarget"]) * number(&fixture["opsPerUnit"]), &mut caller).await;
    for _ in 0..4 {
        let presence = app.tool_run_presence().expect("presence");
        clamped &= presence.total.is_none_or(|total| presence.completed <= total);
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    let presence = app.tool_run_presence().expect("presence");
    assert!(clamped, "completed never exceeds total");
    assert_eq!((presence.state.wire_name(), presence.total), (text(&expected["complete"]), Some(number(&expected["loweredTotal"]))));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_provisional_entities_ride_the_instance_records_the_producer_stamps() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["provisional"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let (scene, _) = render_world(&mut app, None, &mut caller).await;
    let instances: Vec<Value> = serde_json::from_str(&scene.instances_json).expect("instances json");
    let flagged: Vec<&str> = instances.iter().filter(|instance| instance["provisional"] == Value::Bool(true)).map(|instance| text(&instance["id"])).collect();
    assert_eq!(flagged, expected["flaggedInstances"].as_array().expect("flagged").iter().map(text).collect::<Vec<_>>(), "ArtifactView::tool_run().provisional_entities drives the provisional instance flag");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
#[semio_framework_async_macros::async_test]
async fn tool_run_job_port_hands_effects_to_the_host_and_a_waiting_job_keeps_nothing_runnable_until_woken() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["port"];
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    for hop in 1..=number(&expected["hops"]) {
        pump_until(&mut app, "the hop is handed to the host", |app| {
            app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting) && !app.tool_runs.port().is_some_and(ToolRunJobPort::has_effects) && app.tool_runs.trace().is_some_and(|trace| trace.len() as u64 == hop - 1)
        }, &mut caller)
        .await;
        let mut effects = Vec::new();
        while let Some(effect) = app.take_typed_operation_effect() {
            effects.push(effect);
        }
        assert!(matches!(effects.as_slice(), [Effect::DispatchAction { req: RequestId(req), action, .. }] if *req == hop && action == text(&expected["hopAction"])), "hop {hop}: exactly the job's effect reaches the host outbox: {effects:?}");
        for _ in 0..2 {
            while app.take_typed_operation_ui_scope().is_some() {}
            app.tool_run_ctx().flush_tool_run_ui_dirty();
        }
        assert!(!app.tool_runs.has_pending_work() && !app.tool_run_ref().tool_run_has_pending_work(), "hop {hop}: a waiting job keeps its instance idle");
        advance_toy(&mut app, &mut caller).await.expect("an idle turn");
        assert!(app.take_typed_operation_effect().is_none(), "hop {hop}: an unwoken job is never stepped again");
        app.tool_runs.port().expect("the run's port").wake();
        assert!(app.tool_runs.has_pending_work(), "hop {hop}: the wake makes the run runnable again");
    }
    pump_until(&mut app, "every hop answered", |app| app.tool_runs.state().map(ToolRunState::as_str) == Some(text(&expected["state"])), &mut caller).await;
    assert_eq!(app.tool_runs.trace().expect("trace").len() as u64, number(&expected["traceRecords"]), "one trace record per answered hop");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🔌️IntegrationSeams
