// 🪟️ Trace requests and window config: the request trace key and entity marks, the board scene lane, window settings reads and the published window config close and refresh.

//#region 🎯️RetargetAndSettings
/// ⚖️ LAW: the run's trace key allocator hands out keys above every key its ticks upserted and every key allocated
/// before, the request carries the entity marks, and the request names the window the run was started from.
#[semio_framework_async_macros::async_test]
async fn tool_run_requests_carry_trace_key_allocation_entity_marks_and_the_starting_window() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["traceKeys"];
    let window = &fixture["startWindow"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().clear());
    let mut meta = toy_meta();
    meta.view_state = Some(window_view(text(&window["windowId"]), None));
    let output = app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(text(&fixture["toolId"]).into()))])), &meta, &mut caller.identity).await.expect("start").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    assert_eq!(app.tool_runs.window(), Some((text(&window["windowId"]), text(&window["windowKindId"]))));
    let requests = TOY_REQUEST_WINDOWS.with(|windows| windows.borrow().clone());
    assert!(requests.iter().any(|(tool, window_id, window_config)| tool == text(&fixture["toolId"]) && window_id.as_deref() == Some(text(&window["windowId"])) && !window_config), "the job request names the starting window: {requests:?}");
    let keys = app.tool_runs.trace_keys().expect("trace keys");
    assert_eq!(keys.next(), number(&expected["nextAfterRun"]));
    let allocated = keys.allocate(number(&expected["allocate"]));
    assert_eq!([allocated.start, allocated.end], [number(&expected["allocated"][0]), number(&expected["allocated"][1])]);
    assert_eq!(keys.next(), number(&expected["nextAfterAllocation"]));
    let marks = app.tool_runs.entity_marks();
    assert_eq!(marks.iter().map(|(_, entity)| *entity).collect::<Vec<_>>(), (1..=number(&expected["units"])).collect::<Vec<_>>());
    assert!(marks.iter().all(|(end, _)| *end as usize <= app.tool_runs.provisional().len()));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a board-2d scene surface carries the `toolRunTrace` lane exactly like a world-3d one — the spine names
/// the lane with its byte length and hash, and the carrier holds the whole trace delta of the run.
#[semio_framework_async_macros::async_test]
async fn tool_run_board_scene_render_carries_the_trace_lane() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["boardLane"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let mut view = window_view(text(&expected["windowId"]), None);
    view.window_instances.push(ViewWindowInstance { id: text(&expected["windowId"]).to_string(), window_kind_id: text(&expected["windowId"]).to_string() });
    let tree = app.render(text(&expected["bodyKey"]), None, &view).await.unwrap_or_else(|fault| panic!("render board: {fault:?}"));
    let scene = artifact_app_laws::observe_and_retire_fixture_tree(tree, |root| {
        let surface = root.children.iter().find(|child| child.key.as_str() == text(&expected["surfaceId"])).expect("the board body carries its scene surface");
        artifact_app_laws::built_surface_scene::<Board2dScene>(surface).expect("the board scene surface assembles")
    });
    let lane = scene.tool_run_trace.as_deref().expect("a board rendered during a run carries the toolRunTrace lane");
    let reference = scene.lanes.iter().find(|reference| reference.lane == text(&expected["laneName"])).expect("the spine names the injected lane");
    assert_eq!((reference.bytes as usize, reference.hash.as_str()), (lane.len(), semio_framework_ui_scene::scene_lane_hash(lane).as_str()));
    let delta = ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(lane).expect("base64url")).expect("one ToolRunTraceDelta");
    assert!(delta.clear, "a renderer without a cursor gets clear plus the whole log");
    assert_eq!(delta.pages.iter().map(|page| page.ops.len()).sum::<usize>() as u64, number(&expected["units"]));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🪟️ Publishes `selected` into the world window config of `window_id` through the production emit path.
async fn publish_window_selection(app: &mut ToyApp, window_id: &str, selected: u64, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    let fixture = fixture();
    let mut view = window_view(window_id, None);
    view.window_instances.push(ViewWindowInstance { id: text(&fixture["windowSettingsReads"]["otherWindowId"]).to_string(), window_kind_id: text(&fixture["windowSettingsReads"]["windowKindId"]).to_string() });
    let mut meta = toy_meta();
    meta.view_state = Some(view);
    let mutation = WindowConfigMutation::of::<ToyWorldWindowConfig>(window_id, ChangeTestConfigSelection { selected: Some(selected.to_string()) }.into());
    app.dispatch_emit("setWorldSelection", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { window_config_mutations: vec![mutation], ..Default::default() }, &meta, &mut caller.identity)
        .await
        .unwrap_or_else(|fault| panic!("window config publication: {fault:?}"));
}

/// ⚖️ LAW: a run started from a window reads the window-config fields it declares for that window's kind from the
/// starting window only — its job request carries that window's config snapshot, a publication on another window of the
/// same kind or of an unchanged value never reconfigures, and a changed value on the starting window does.
#[semio_framework_async_macros::async_test]
async fn tool_run_window_settings_reads_follow_the_starting_window_only() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["windowSettingsReads"];
    let (start_window, other_window) = (text(&expected["startWindowId"]), text(&expected["otherWindowId"]));
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    publish_window_selection(&mut app, start_window, 1, &mut caller).await;
    TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().clear());
    let mut meta = toy_meta();
    let mut view = window_view(start_window, None);
    view.window_instances.push(ViewWindowInstance { id: other_window.to_string(), window_kind_id: text(&expected["windowKindId"]).to_string() });
    meta.view_state = Some(view);
    let output = app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(text(&expected["toolId"]).into()))])), &meta, &mut caller.identity).await.expect("start").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
    pump_until(&mut app, "running with provisional units", |app| app.tool_runs.state() == Some(ToolRunState::Running) && !app.tool_runs.provisional().is_empty(), &mut caller).await;
    let requests = TOY_REQUEST_WINDOWS.with(|windows| windows.borrow().clone());
    assert!(requests.iter().any(|(tool, window_id, window_config)| tool == text(&expected["toolId"]) && window_id.as_deref() == Some(start_window) && *window_config), "the job request carries the starting window's config snapshot: {requests:?}");
    publish_window_selection(&mut app, other_window, 7, &mut caller).await;
    publish_window_selection(&mut app, start_window, 1, &mut caller).await;
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterOtherWindow"]), "another window and an unchanged value never reconfigure");
    publish_window_selection(&mut app, start_window, 2, &mut caller).await;
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterStartWindow"]), "a changed value on the starting window reconfigures");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a window config partition closes through the physical release its own store publishes — its resident backings (the 1 024-slot
/// snapshot read registry alone is far over one ordinary close page) are priced by the app's close demand, so a published window config
/// reaches the exact terminal-empty witness instead of answering `Pending { 0, 0 }` against an ordinary-page grant for ever.
#[semio_framework_async_macros::async_test]
async fn a_published_window_config_closes_through_its_own_published_physical_demand() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(1, &mut caller).await;
    publish_window_selection(&mut app, text(&fixture["windowSettingsReads"]["startWindowId"]), 1, &mut caller).await;
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a MOUNTED typed operation that publishes a window config keeps its published receipt while the window authority refresh
/// is still yielding for the displaced snapshot's custody, resumes that refresh on its next unit and queues the page of exactly the
/// retained receipt — it never advances the already published store again ("awaits an ACK without its retained host page").
#[semio_framework_async_macros::async_test]
async fn a_mounted_window_config_publication_resumes_its_refresh_and_queues_the_retained_receipt() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let window_id = text(&fixture["windowSettingsReads"]["startWindowId"]);
    let mut app = toy_app(1, &mut caller).await;
    let view = window_view(window_id, None);
    let authority = app.stores.window_config_store.capture(Some(&view)).await.expect("window authority capture").expect("the world window owns a config partition");
    let generation_before = authority.snapshot.generation();
    let revision = app.stores.store.content_revision_now();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(app.stores.store.generation_now()), 17);
    let lease = app.tools.tool_cancellations.begin(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation }).expect("cancellation lease");
    let mutation = WindowConfigMutation::of::<ToyWorldWindowConfig>(window_id, ChangeTestConfigSelection { selected: Some("5".into()) }.into());
    let mut mounted = test_blank_mounted_operation::<ToyRunApp>();
    mounted.verb = "setWorldSelection".into();
    mounted.meta = ActionMeta { actor: text(&fixture["actor"]).into(), instance_id: 7, view_state: Some(view) };
    mounted.operation = operation;
    mounted.canonical_revision = revision;
    mounted.artifact_generation = operation.generation.0;
    mounted.window_config_authority = Some(authority);
    mounted.publication_lanes = &[ArtifactToolPublicationLane::WindowConfig];
    mounted.cancellation_lease = Some(lease);
    mounted.terminal_seen = true;
    mounted.publication = Some(ArtifactToolCompletionValue::Emit(Ok(Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { window_config_mutations: vec![mutation], ..Default::default() }), EphemeralEmit::default()));
    let grant = mounted_policy.maintenance;
    let mut yielded_with_retained_receipt = 0;
    let mut page = None;
    for _ in 0..4096 {
        app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, grant).unwrap_or_else(|fault| panic!("every unit of a window config publication resumes: {fault:?}"));
        mounted.publication_ownership_progress.take();
        if mounted.pending_window_config_receipt.is_some() {
            yielded_with_retained_receipt += 1;
        }
        if let Some(found) = mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation) {
            page = Some(found);
            break;
        }
    }
    let mut page = page.expect("the retained window config receipt reaches its result page");
    assert_eq!(page.lane, TypedOperationResultLane::WindowConfig);
    assert!(yielded_with_retained_receipt > 0, "the first refresh unit yields while the displaced snapshot awaits custody");
    assert!(mounted.pending_window_config_receipt.is_none() && mounted.published_window_config);
    assert!(mounted.window_config_authority.as_ref().is_some_and(|authority| authority.snapshot.generation() > generation_before), "the captured authority was refreshed to the published generation");
    assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).expect("exact ACK"));
mounted.close_fixture_original_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
    let mut terminal = false;
    for _ in 0..4096 {
        app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, grant).unwrap_or_else(|fault| panic!("every unit after the window config ACK resumes: {fault:?}"));
        mounted.publication_ownership_progress.take();
        if mounted.granted_retirement_ready() && mounted.granted_retirement_pending() {
            let _demand = mounted.granted_retirement_demands(grant.maximum_copy_bytes).expect("quoted granted retirement");
            let quoted = grant;
            mounted.granted_retirement_step(quoted).expect("granted retirement of the acknowledged publication");
        }
        if let Some(mut page) = mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation) {
            terminal = page.lane == TypedOperationResultLane::Terminal;
            assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).expect("exact ACK"));
mounted.close_fixture_original_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
            break;
        }
    }
    assert!(terminal, "the acknowledged window config operation ends with its terminal page");
    for _ in 0..4096 {
        if mounted.terminal_is_empty() {
            break;
        }
        if mounted.lease_retirement_ready() {
            let _demand = mounted.lease_retirement_demands(&app.tools.tool_cancellations, grant.maximum_copy_bytes).expect("quoted cancellation lease retirement");
            let quoted = grant;
            mounted.lease_retirement_step(&app.tools.tool_cancellations, quoted).expect("cancellation lease retirement");
            continue;
        }
        let _demand = mounted.retirement_demands(grant.maximum_copy_bytes).expect("quoted mounted retirement");
            let quoted = grant;
        match mounted.retirement_step(quoted).expect("mounted retirement step") {
            PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => assert!(progress.fits(quoted)),
            PluginLifecycleStep::AwaitingInput { reason } | PluginLifecycleStep::Blocked { reason } => panic!("the published window config operation retires: {reason}"),
        }
    }
    assert!(mounted.terminal_is_empty(), "the published window config operation retires every owner");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🎯️RetargetAndSettings
