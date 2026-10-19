/// 🖥️ Display and table transfer: published tree handle routing, the display transfer host fixture and laws and table surface transfers through the shell host.

/// ⚖️ LAW: the production Shell host routes BOTH semantic targets of a published handle-driven
/// Tree through the retained router. The trailing handle arms and promotes the retained drag across
/// the real down/move/up ingress without firing the row selection, while the remaining label band
/// still commits that row's published `Activate` binding on a later click.
#[test]
fn published_tree_handle_routes_through_shell_and_preserves_label_selection() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "tree-pointer-host-boundary";
    let body = Rect::new(17.0, 31.0, 360.0, 240.0);
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let (mut document, item_key) = published_tree_pointer_document(&mut shell, surface);
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    assert_eq!(semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::accessibility_visible_window_ids(), vec![surface.to_string()], "the same completed retained-body publication owns the unnamed accessibility surface set");

    let handle_id = format!("tree.drag.transfer.{item_key}");
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(handle_id.as_str())).expect("the published Handle driver exposes the transfer affordance").rect;
    let row_id = format!("tree.label.{item_key}");
    let row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(row_id.as_str())).expect("the same published row keeps its label target").rect;
    let (handle_x, handle_y) = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(handle_x, handle_y, true, 0, &mut input, &theme)).expect("Shell routes handle down");
    PointerCtx::of(&mut shell).handle_pointer_move(handle_x - 8.0, handle_y, true, &mut input, &theme);
    let sessions = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::active_retained_drag_sessions();
    assert_eq!(sessions.len(), 1, "the Shell move promotes the retained Tree press to one drag session");
    assert_eq!(sessions[0].0, surface);
    assert_eq!(sessions[0].3.get("application/x-semio-window-template").map(String::as_str), Some("{\"windowKindId\":\"world\",\"templateId\":\"default\"}"));
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(handle_x - 8.0, handle_y, false, 0, &mut input, &theme)).expect("Shell routes handle up");
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::active_retained_drag_sessions().is_empty(), "release retires the retained drag session");
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "a handle gesture never aliases the row-label selection");

    let (label_x, label_y) = (row.x + theme.gap_standard.max(1.0), row.y + row.h * 0.5);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(label_x, label_y, true, 0, &mut input, &theme)).expect("Shell routes label down");
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(label_x, label_y, false, 0, &mut input, &theme)).expect("Shell routes label up");
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1, "the label click still commits exactly one selection action");
    assert_eq!(actions[0].controller_id, "s.test.tree");
    assert_eq!(actions[0].action, "selectTreeItem");
    let args = dsl_value_as_json(actions[0].args.as_ref().expect("the retained host scopes the selection"));
    assert_eq!(args["windowId"], surface);
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::publish_accessibility_visible_documents();
    assert!(semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::accessibility_visible_window_ids().is_empty(), "a later complete walk with no retained body removes the live document from production accessibility without retiring it");
    while !document.close_step() {}
}

struct DisplayTransferHostFixture {
    shell: ShellState,
    document: UiDocumentLease,
    input: InputState<ActionDescriptor>,
    theme: Theme,
    source: (f32, f32),
}

fn display_transfer_host_fixture() -> DisplayTransferHostFixture {
    let surface = "display-transfer-negative-boundary";
    let body = Rect::new(19.0, 37.0, 420.0, 300.0);
    let theme = Theme::default();
    let mut shell = host_test_shell();
    let mut node = shell.docs.build_display_windows_ui(&shell.prefs);
    let UiNode::Stack(stack) = &mut node else { panic!("Display Windows publishes a stack") };
    let Some(UiNode::Tree(tree)) = stack.children.first_mut() else { panic!("Display Windows stack publishes a tree") };
    tree.sections.iter_mut().find(|section| section.id == "framework.display.windows.main").expect("the real app's main window-kind section").default_open = Some(true);
    let records = panel_ui_records(surface, &node).expect("the real Display producer projects");
    let projected_item_key = records
        .iter()
        .find(|record| record.key.as_str().ends_with("framework.display.windows.main.kind") && matches!(&record.component, ui_contract::Component::TreeItem(_)))
        .map(|record| record.key.as_str().to_string())
        .expect("the real Display producer projects its transfer row");
    shell.dock.dock = DockState::default();
    shell.dock.dock_canvas_bounds = Rect::new(500.0, 0.0, 400.0, 600.0);
    shell.dock.dock_drop_tab_bars.clear();
    shell.dock.dock_drop_bodies.clear();
    *shell.dock.dock_tabs.tabs_mut(PanelAnchor::TopLeft) = vec![DockTabNode::leaf(surface, "Windows", "panels-top-left", 0)];
    shell.dock.anchor_state_mut(PanelAnchor::TopLeft).visible = true;
    shell.dock.anchor_state_mut(PanelAnchor::TopLeft).path = vec![surface.to_string()];
    shell.windows.active_window_id = Some("previous-app-window".into());
    let mut document = shell.windows.publish_surface_records(surface, records).expect("the projected Display document acquires a retained lease");
    let input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    let transfer_control_id = format!("tree.drag.transfer.{projected_item_key}");
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(transfer_control_id.as_str())).expect("the real Display producer paints a transfer handle");
    let source = (handle.rect.x + handle.rect.w * 0.5, handle.rect.y + handle.rect.h * 0.5);
    DisplayTransferHostFixture { shell, document, input, theme, source }
}

fn perform_display_transfer(fixture: &mut DisplayTransferHostFixture, drop: (f32, f32)) -> Result<(), String> {
    let (source_x, source_y) = fixture.source;
    semio_framework_async::block_on(ChromeViewCtx::of(&mut fixture.shell).handle_pointer_button(source_x, source_y, true, 0, &mut fixture.input, &fixture.theme))?;
    PointerCtx::of(&mut fixture.shell).handle_pointer_move(drop.0, drop.1, true, &mut fixture.input, &fixture.theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut fixture.shell).handle_pointer_button(drop.0, drop.1, false, 0, &mut fixture.input, &fixture.theme))
}

fn close_display_transfer_host_fixture(mut fixture: DisplayTransferHostFixture) {
    while !fixture.document.close_step() {}
}

#[test]
fn cancelled_display_transfer_retires_capture_without_creating_a_window() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut fixture = display_transfer_host_fixture();
    let before = fixture.shell.dock.dock.window_instances().len();
    let (x, y) = fixture.source;
    semio_framework_async::block_on(ChromeViewCtx::of(&mut fixture.shell).handle_pointer_button(x, y, true, 0, &mut fixture.input, &fixture.theme)).expect("Display transfer press");
    PointerCtx::of(&mut fixture.shell).handle_pointer_move(800.0, 500.0, true, &mut fixture.input, &fixture.theme);
    PointerCtx::of(&mut fixture.shell).handle_pointer_cancel(&mut fixture.input);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(fixture.shell.dock.dock_drag.is_none() && fixture.shell.dock.pending_dock_drag.is_none());
    assert!(!fixture.input.pointer_down && !fixture.input.drag.active);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut fixture.shell).handle_pointer_button(800.0, 500.0, false, 0, &mut fixture.input, &fixture.theme)).expect("late release stays inert");
    assert_eq!(fixture.shell.dock.dock.window_instances().len(), before);
    assert!(fixture.shell.windows.window_topology_actions.is_empty());
    close_display_transfer_host_fixture(fixture);
}

#[test]
fn refused_window_publication_does_not_block_a_ready_display_peer() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json")).expect("window publication fixture");
    let cohort = &contract["publicationOutcomes"]["cohort"];
    let mut fixture = display_transfer_host_fixture();
    let program = fixture.shell.plugins.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the actual Display fixture has its guest ProgramBridge");
    program.install_fixture_render(scripted_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_BODY_FIXTURE_REFUSALS.with(|remaining| remaining.set(1));
    WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(0));
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().clear());

    perform_display_transfer(&mut fixture, (800.0, 500.0)).expect("the first actual Display transfer commits");
    perform_display_transfer(&mut fixture, (750.0, 300.0)).expect("the second actual Display transfer commits");
    assert_eq!(fixture.shell.dock.dock.window_instances().len(), 2);
    assert_eq!(fixture.shell.windows.window_topology_actions.len(), 2);
    assert_eq!(fixture.shell.windows.window_topology_publications.len(), 2);

    assert_eq!(semio_framework_async::block_on(PointerCtx::of(&mut fixture.shell).settle_pump_step_inner()), ShellSettleStep::Drained);
    assert!(!fixture.shell.docs.window_ui.contains_key(cohort["refused"].as_str().unwrap()));
    assert!(fixture.shell.docs.window_ui.contains_key(cohort["ready"].as_str().unwrap()));
    assert_eq!(fixture.shell.windows.window_topology_actions.len(), 1, "the ready peer releases its token during the first cohort refresh");
    assert_eq!(fixture.shell.settle.deferred_actions.len(), 1);
    let ready_journal = dsl_value_as_json(fixture.shell.settle.deferred_actions[0].args.as_ref().expect("the ready journal carries its instance"));
    assert_eq!(ready_journal["detail"]["instanceId"], cohort["ready"]);

    let dispatched = |instance: &str| WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().iter().any(|event| event.as_str() == format!("journal:{instance}")));
    for _ in 0..WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS * 4 {
        if dispatched("main-2") && dispatched("main-3") {
            break;
        }
        assert_eq!(semio_framework_async::block_on(PointerCtx::of(&mut fixture.shell).settle_pump_step_inner()), ShellSettleStep::Drained, "the settle lane owns the retry and every journal");
    }
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());
    let ready_dispatch = events.iter().position(|event| event == "journal:main-3").expect("the ready peer dispatches");
    let refused_retry = events.iter().enumerate().filter(|(_, event)| event.as_str() == "render:main-2").nth(1).map(|(index, _)| index).expect("the refused peer retries");
    assert!(ready_dispatch < refused_retry, "the ready journal dispatches before the unrelated retry");
    for instance in ["main-2", "main-3"] {
        assert_eq!(events.iter().filter(|event| event.as_str() == format!("journal:{instance}")).count(), 1, "{instance} dispatches exactly once");
        assert_eq!(contract["publicationOutcomes"]["cohort"]["afterRetryDispatches"][instance], 1);
    }
    assert!(fixture.shell.windows.window_topology_publications.is_empty());
    assert!(fixture.shell.windows.window_topology_actions.is_empty());
    assert!(fixture.shell.windows.window_topology_action_tokens.is_empty());
    retire_scripted_window_publication_documents(&mut fixture.shell);
    close_display_transfer_host_fixture(fixture);
}

#[test]
fn actual_display_release_refuses_item_and_byte_credit_before_dock_mutation() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json")).expect("window publication fixture");
    let refusals = contract["publicationOutcomes"]["atomicCreditRefusals"].as_array().expect("credit vectors");

    let mut item = display_transfer_host_fixture();
    let item_root = item.shell.dock.dock.root.clone();
    for _ in 0..semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY {
        item.shell.windows.reserve_window_topology_action(ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: None }).expect("item-credit fixture admission");
    }
    let item_error = perform_display_transfer(&mut item, (800.0, 500.0)).expect_err("the physical release observes item refusal");
    assert!(item_error.contains("ItemCredits"));
    assert_eq!(item.shell.dock.dock.root, item_root);
    assert!(item.shell.windows.window_topology_publications.is_empty());
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none(), "a refused physical release still retires capture");
    assert!(item.shell.dock.pending_dock_drag.is_none() && item.shell.dock.dock_drag.is_none());
    assert_eq!(refusals[0]["dockMutation"], "none");
    close_display_transfer_host_fixture(item);

    let mut bytes = display_transfer_host_fixture();
    let byte_root = bytes.shell.dock.dock.root.clone();
    let chunk = "x".repeat(3_900);
    let mut admitted = 0usize;
    loop {
        let action = ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "a": chunk.clone(), "b": chunk.clone(), "c": chunk.clone(), "d": chunk.clone() }) };
        match bytes.shell.windows.reserve_window_topology_action(action) {
            Ok(_) => admitted += 1,
            Err(semio_framework_ui_action::BoundedActionFault::ByteCredits) => break,
            Err(error) => panic!("byte fixture reached the wrong admission fault: {error:?}"),
        }
    }
    assert!(admitted < semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY, "byte credits exhaust before item credits");
    let byte_error = perform_display_transfer(&mut bytes, (800.0, 500.0)).expect_err("the physical release observes byte refusal");
    assert!(byte_error.contains("ByteCredits"));
    assert_eq!(bytes.shell.dock.dock.root, byte_root);
    assert!(bytes.shell.windows.window_topology_publications.is_empty());
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(bytes.shell.dock.pending_dock_drag.is_none() && bytes.shell.dock.dock_drag.is_none());
    assert_eq!(refusals[1]["dockMutation"], "none");
    close_display_transfer_host_fixture(bytes);
}

/// ⚖️ LAW: Display's real window-kind producer keeps its transfer payload through panel projection,
/// retained reconciliation and paint, so the published Shell ingress starts a NewWindow dock drag.
#[test]
fn display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "display-transfer-host-boundary";
    let body = Rect::new(19.0, 37.0, 420.0, 300.0);
    let theme = Theme::default();
    let mut shell = host_test_shell();
    let mut node = shell.docs.build_display_windows_ui(&shell.prefs);
    let UiNode::Stack(stack) = &mut node else { panic!("Display Windows publishes a stack") };
    let Some(UiNode::Tree(tree)) = stack.children.first_mut() else { panic!("Display Windows stack publishes a tree") };
    let section = tree.sections.iter_mut().find(|section| section.id == "framework.display.windows.main").expect("the real app's main window-kind section");
    section.default_open = Some(true);

    let records = panel_ui_records(surface, &node).expect("the real Display producer projects");
    let projected_item_key = records
        .iter()
        .find(|record| record.key.as_str().ends_with("framework.display.windows.main.kind") && matches!(&record.component, ui_contract::Component::TreeItem(_)))
        .map(|record| record.key.as_str().to_string())
        .expect("the real Display producer projects its transfer row");
    assert!(projected_item_key.ends_with("framework.display.windows.main.kind"));
    let transfer_control_id = format!("tree.drag.transfer.{projected_item_key}");
    let sort_control_id = format!("tree.drag.sort.{projected_item_key}");
    shell.dock.dock = DockState::default();
    shell.dock.dock_canvas_bounds = Rect::new(500.0, 0.0, 400.0, 600.0);
    shell.dock.dock_drop_tab_bars.clear();
    shell.dock.dock_drop_bodies.clear();
    *shell.dock.dock_tabs.tabs_mut(PanelAnchor::TopLeft) = vec![DockTabNode::leaf(surface, "Windows", "panels-top-left", 0)];
    shell.dock.anchor_state_mut(PanelAnchor::TopLeft).visible = true;
    shell.dock.anchor_state_mut(PanelAnchor::TopLeft).path = vec![surface.to_string()];
    shell.windows.active_window_id = Some("previous-app-window".into());
    let mut document = shell.windows.publish_surface_records(surface, records).expect("the projected Display document acquires a retained lease");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(transfer_control_id.as_str())).expect("the reconciled and painted window kind publishes its transfer handle");
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref() == Some(sort_control_id.as_str())), "a payload-bearing row never degrades to a sort handle");
    let drag_data = handle.drag_data.as_ref().expect("the transfer handle owns the producer's payload");
    let payload = decode_window_template_drag(drag_data).expect("the retained MIME payload stays decodable");
    assert_eq!(payload.window_kind_id, "main");
    assert_eq!(payload.template_id, None);

    let (x, y) = (handle.rect.x + handle.rect.w * 0.5, handle.rect.y + handle.rect.h * 0.5);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Shell routes the published transfer handle");
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("previous-app-window"), "a retained panel press never invents the panel as an application window");
    let (pending, _) = shell.dock.pending_dock_drag.as_ref().expect("the host ingress arms a dock drag");
    assert_eq!(pending.kind, DockDragKind::NewWindow);
    assert_eq!(pending.window_kind_id, "main");
    assert_eq!(pending.template_id, None);

    PointerCtx::of(&mut shell).handle_pointer_move(800.0, 500.0, true, &mut input, &theme);
    assert!(shell.dock.pending_dock_drag.is_none());
    assert!(shell.dock.dock_drag.is_some(), "the captured retained move promotes the transfer into the dock authority");
    assert!(input.hit_at(800.0, 500.0).is_none(), "the runtime failure's release point owns no current retained hit");
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("a hitless release commits through retained capture");
    assert_eq!(shell.dock.dock.window_instances(), vec![("main-2".to_string(), "main".to_string())], "the empty dock receives exactly one new window");
    assert!(shell.windows.window_topology_refresh_owed, "the successful topology mutation owes the new roster's first Full refresh");
    assert!(matches!(shell.settle.owed_refresh_scope, UiDirtyScope::Full));
    assert_eq!(shell.windows.window_topology_actions.len(), 1, "the gesture receives exactly one bounded journal admission");
    assert_eq!(shell.windows.window_topology_publications.len(), 1);
    assert!(shell.windows.window_topology_publications[0].journal_token.is_some());
    assert!(shell.settle.deferred_actions.is_empty(), "the guest journal cannot run before the new window body is published");
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none(), "the hitless release retires retained capture");
    assert!(shell.dock.pending_dock_drag.is_none() && shell.dock.dock_drag.is_none());
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("a duplicate release is inert");
    assert_eq!(shell.dock.dock.window_instances().len(), 1, "one gesture cannot create a second window");

    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("a second transfer can arm after capture retirement");
    PointerCtx::of(&mut shell).handle_pointer_move(1000.0, 700.0, true, &mut input, &theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(1000.0, 700.0, false, 0, &mut input, &theme)).expect("an outside release cancels cleanly");
    assert_eq!(shell.dock.dock.window_instances().len(), 1, "an outside drop does not create a window");
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(shell.dock.pending_dock_drag.is_none() && shell.dock.dock_drag.is_none());

    let scene = semio_framework_ui_scene_world3d::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    let body_document = shell.windows.publish_surface_records("main-2", vec![world3d_pointer_record(1, "initial-world", &scene)]).expect("the guest producer publishes the created instance's retained body");
    shell.docs.window_ui.insert("main-2".into(), body_document);
    SessionCtx::of(&mut shell).complete_window_topology_refresh();
    assert!(!shell.windows.window_topology_refresh_owed);
    assert!(shell.windows.window_topology_actions.is_empty());
    assert!(shell.windows.window_topology_publications.is_empty());
    assert_eq!(shell.settle.deferred_actions.len(), 1, "one admitted topology journal is released after body publication");
    let journal = &shell.settle.deferred_actions[0];
    assert_eq!(journal.action, "noteShellCommand");
    let journal_args = dsl_value_as_json(journal.args.as_ref().expect("the topology journal carries its command"));
    assert_eq!(journal_args["commandId"], "shell.windowSplit");
    assert_eq!(journal_args["detail"]["windowKindId"], "main");
    assert_eq!(journal_args["detail"]["instanceId"], "main-2");
    SessionCtx::of(&mut shell).complete_window_topology_refresh();
    assert_eq!(shell.settle.deferred_actions.len(), 1, "completion acknowledgement cannot duplicate the journal");

    let bounds = Rect::new(0.0, 0.0, 900.0, 600.0);
    shell.chrome.screen_w = bounds.w;
    shell.chrome.screen_h = bounds.h;
    let mut draw = DrawList::default();
    draw.set_screen_height(bounds.h);
    let mut overlay_draw = DrawList::default();
    overlay_draw.set_screen_height(bounds.h);
    let mut overlay = Some(&mut overlay_draw);
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut body_input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    let painted = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut body_input, &theme, bounds, &mut world_resources, cx)));
    assert!(painted, "the canonical Shell window walk consumes the created instance's initial body");
    assert!(body_input.staged_hits().iter().any(|hit| hit.kind == HitKind::World3d), "the canonical window walk stages the new instance's physical World3d hit target");
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut body_input);
    SessionCtx::of(&mut shell).sync_engine_surface_states();
    let world_host = shell.surfaces.world3d_host_id_for_window("main-2").expect("the accepted body registers its World3d host").to_string();
    assert!(shell.surfaces.world3d_states.contains_key(&world_host), "the published body registers a live World3d owner before any example switch");
    assert!(body_input.hits().iter().any(|hit| hit.kind == HitKind::World3d), "the new instance publishes a physical World3d hit target");
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🛟️panel-window-reservation/🔣️.json")).unwrap();
    let world_hit = body_input.hits().iter().find(|hit| hit.kind == HitKind::World3d).unwrap();
    assert!(shell.input.retained_hit_window(world_hit).is_some(), "the actual retained publication registers this scene's body ownership");
    let point = (world_hit.rect.x + world_hit.rect.w * 0.5, world_hit.rect.y + world_hit.rect.h * 0.5);
    let owner = shell.input.pointer_owner_at(point.0, point.1, &body_input, &theme);
    assert_eq!(if owner == PointerHitOwner::Surface { "surface" } else { "chrome" }, fixture["retainedWorld"]["owner"].as_str().unwrap());
    assert_eq!(shell.input.wheel_reaches_scene_surface(point.0, point.1, &body_input, &theme), fixture["retainedWorld"]["wheel"].as_bool().unwrap());

    let world_token = shell.surfaces.world3d_states.token(&world_host).expect("the live World3d owner has generation identity");
    assert!(SessionCtx::of(&mut shell).close_dock_window("main-2"));
    ChromeViewCtx::of(&mut shell).plan_dock_windows(bounds, &theme, &mut atlas);
    SessionCtx::of(&mut shell).sync_engine_surface_states();
    assert!(!shell.surfaces.world3d_states.contains_key(&world_host));
    assert!(shell.surfaces.world3d_states.get_token(world_token).is_none(), "the closed instance's generation cannot be recovered");
    assert_eq!(shell.surfaces.retired_world3d_states.len(), 1, "the closed owner enters bounded progressive retirement");
    shell.docs.retire_documents_outside(&[], true, &mut shell.chrome).expect("the closed retained body enters document retirement");
    assert!(!shell.docs.window_ui.contains_key("main-2"));
    shell.settle.deferred_actions.clear();
    shell.windows.window_topology_journal_dispatch_owed = false;
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.surfaces.retired_world3d_states.is_empty() {
            break;
        }
        shell.surfaces.advance_world3d_retirement_step();
    }
    assert!(shell.surfaces.retired_world3d_states.is_empty(), "the retired World3d owner reaches terminal empty under maintenance");
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for("main-2") {
            break;
        }
        assert!(caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn"));
        shell = semio_framework_os_renderer_wgpu_host_winit::os_host::finish_world_fixture_component_close(shell);
    }
    assert!(!semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for("main-2"), "the presenter maintenance lane returns the closed retained surface before its identity is reused");

    let program = shell.plugins.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the host fixture has its guest program");
    program.install_fixture_render(runnable_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("the real Display producer arms a fresh transfer");
    PointerCtx::of(&mut shell).handle_pointer_move(800.0, 500.0, true, &mut input, &theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("the real Display producer creates the fresh instance");
    assert!(shell.windows.window_topology_refresh_owed);
    assert!(!shell.docs.window_ui.contains_key("main-2"), "the new topology has no body before the settle lane runs");
    let settle = semio_framework_async::block_on(PointerCtx::of(&mut shell).settle_pump_step_inner());
    assert_eq!(settle, ShellSettleStep::Drained, "the topology debt consumes one bounded settle step");
    assert!(!shell.windows.window_topology_refresh_owed);
    assert!(shell.docs.window_ui.contains_key("main-2"), "refresh_ui fetches the new roster's body through the runnable ProgramBridge fixture");
    assert_eq!(shell.settle.deferred_actions.iter().filter(|action| action.action == "noteShellCommand").count(), 1, "the first body refresh releases exactly one admitted topology journal");
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0, "the journal cannot overtake initial body publication");
    assert_eq!(semio_framework_async::block_on(PointerCtx::of(&mut shell).settle_pump_step_inner()), ShellSettleStep::Drained, "the following bounded step owns guest dispatch");
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 1, "the topology journal is refused exactly once");
    assert!(shell.settle.deferred_actions.is_empty(), "the refused journal retains no stale action owner");

    ChromeViewCtx::of(&mut shell).plan_dock_windows(bounds, &theme, &mut atlas);
    let mut refreshed_draw = DrawList::default();
    refreshed_draw.set_screen_height(bounds.h);
    let mut refreshed_overlay_draw = DrawList::default();
    refreshed_overlay_draw.set_screen_height(bounds.h);
    let mut refreshed_overlay = Some(&mut refreshed_overlay_draw);
    let mut refreshed_input = InputState::<ActionDescriptor>::default();
    let mut refreshed_cursor = ShellChromeChildCursor::default();
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    let refreshed =
        (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut refreshed_cursor, &mut refreshed_draw, &mut refreshed_overlay, &mut atlas, &icons, &mut refreshed_input, &theme, bounds, &mut world_resources, cx)));
    assert!(refreshed, "the body returned by refresh_ui enters the canonical Shell paint walk");
    assert!(refreshed_input.staged_hits().iter().any(|hit| hit.kind == HitKind::World3d));
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut refreshed_input);
    assert!(refreshed_input.hits().iter().any(|hit| hit.kind == HitKind::World3d), "the actual refreshed body publishes a physical World3d hit before any example switch");

    assert!(SessionCtx::of(&mut shell).close_dock_window("main-2"));
    ChromeViewCtx::of(&mut shell).plan_dock_windows(bounds, &theme, &mut atlas);
    SessionCtx::of(&mut shell).sync_engine_surface_states();
    shell.docs.retire_documents_outside(&[], true, &mut shell.chrome).expect("the refreshed window body retires");
    shell.docs.retire_documents_outside(&[], false, &mut shell.chrome).expect("the runnable fixture's panel bodies retire");
    let auxiliary_documents =
        std::mem::take(&mut shell.docs.window_actions_documents).into_values().chain(std::mem::take(&mut shell.docs.window_search_documents).into_values()).chain(std::mem::take(&mut shell.docs.window_measures_documents).into_values()).collect::<Vec<_>>();
    for auxiliary in auxiliary_documents {
        shell.docs.retire_one_surface_document(Some(auxiliary)).expect("the refreshed auxiliary body retires");
    }
    shell.docs.drain_retained_document_arenas();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.surfaces.retired_world3d_states.is_empty() {
            break;
        }
        shell.surfaces.advance_world3d_retirement_step();
    }
    assert!(shell.surfaces.retired_world3d_states.is_empty(), "the refreshed owner also reaches terminal retirement");
    caller.retire_world(&mut world_resources);
    caller.finish();
    while !document.close_step() {}
}

/// ⚖️ LAW: two real `Component::Surface(Table)` documents publish into one Shell registry,
/// and the host's retained down/move/up ingress preserves the generation-owned transfer between them.
#[test]
fn two_published_table_surfaces_transfer_through_the_shell_host() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    table_transfer_through_the_shell_host(false);
}

#[test]
fn a_published_table_transfer_survives_a_same_host_document_refresh() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    table_transfer_through_the_shell_host(true);
}

fn table_transfer_through_the_shell_host(refresh: bool) {
    semio_framework_os_renderer_wgpu_scene_render::cancel_scene_list_transfer();
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🔀️scene-list-transfer/🔣️.json")).expect("shared transfer fixture");
    let source_fixture = &fixture["table"]["source"];
    let destination_fixture = &fixture["table"]["destination"];
    let source_id = "table-shell-source";
    let destination_id = "table-shell-destination";
    let columns = serde_json::json!([{ "id": "name", "label": "Name", "sortable": false }]).to_string();
    let mut source_scene = semio_framework_ui_scene_views::TableScene::base(
        columns.clone(),
        serde_json::json!([{
            "id": source_fixture["rowId"],
            "name": "Asset",
            "_drag": source_fixture["payload"]
        }])
        .to_string(),
    );
    source_scene.row_drag_mime = source_fixture["mime"].as_str().map(str::to_string);
    let mut destination_scene = semio_framework_ui_scene_views::TableScene::base(columns, serde_json::json!([{ "id": "destination", "name": "Destination" }]).to_string());
    destination_scene.drop_action_json = Some(destination_fixture["dropAction"].to_string());

    let mut shell = host_test_shell();
    let mut source_document = shell.windows.publish_surface_records(source_id, vec![table_pointer_record(1, "source-table", &source_scene)]).expect("source Table document publishes");
    let mut destination_document = shell.windows.publish_surface_records(destination_id, vec![table_pointer_record(1, "destination-table", &destination_scene)]).expect("destination Table document publishes");
    let source_body = Rect::new(11.0, 17.0, 360.0, 240.0);
    let destination_body = Rect::new(411.0, 17.0, 360.0, 240.0);
    let mut input = paint_component_pointer_documents(
        &mut shell,
        &[(source_id, "controller.table-a", &source_document, source_body), (destination_id, destination_fixture["dropAction"]["controllerId"].as_str().expect("destination controller"), &destination_document, destination_body)],
    );
    let source_owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(source_id, source_body.w * 0.5, source_body.h * 0.5).expect("source component host");
    let destination_owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(destination_id, destination_body.w * 0.5, destination_body.h * 0.5).expect("destination component host");
    let source_control = format!("{}.row.asset-7", source_owner.host_id);
    let destination_control = format!("{}.row.destination", destination_owner.host_id);
    let source_row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(source_control.as_str())).expect("source row paints from the decoded Component::Surface").rect;
    let destination_row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(destination_control.as_str())).expect("destination row paints from the decoded Component::Surface").rect;
    let theme = Theme::default();
    let handle_size = theme.control_height_small.min(source_row.h).min(source_row.w.max(0.0));
    let source_point = (source_row.x + theme.padding_standard + handle_size * 0.5, source_row.y + source_row.h * 0.5);
    let destination_point = (destination_row.x + destination_row.w * 0.5, destination_row.y + destination_row.h * 0.5);
    assert_eq!(input.hit_at(source_point.0, source_point.1).map(|hit| hit.kind), Some(HitKind::ComponentScene), "the published semantic surface owns the physical handle point");
    assert_eq!(input.hit_at(destination_point.0, destination_point.1).map(|hit| hit.kind), Some(HitKind::ComponentScene), "the published semantic surface owns the physical drop point");

    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("Shell routes Table source down");
    let mut refreshed_document = if refresh {
        let document = shell.windows.publish_surface_records(source_id, vec![table_pointer_record(1, "source-table", &source_scene)]).expect("the source refresh publishes");
        input =
            paint_component_pointer_documents(&mut shell, &[(source_id, "controller.table-a", &document, source_body), (destination_id, destination_fixture["dropAction"]["controllerId"].as_str().unwrap(), &destination_document, destination_body)]);
        let refreshed_owner = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(source_id, source_body.w * 0.5, source_body.h * 0.5).expect("refreshed component host");
        assert!(source_owner.same_component_host(&refreshed_owner), "ordinary publication preserves the mounted Table owner");
        Some(document)
    } else {
        None
    };
    PointerCtx::of(&mut shell).handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("Shell routes Table destination up");
    let actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1, "one physical transfer emits exactly one destination action");
    assert_eq!(serde_json::to_value(&actions[0]).expect("action wire"), fixture["table"]["expectedAction"]);
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(ui_render::PointerId(1)).is_none(), "release retires retained capture");
    assert!(!semio_framework_os_renderer_wgpu_scene_render::cancel_scene_list_transfer(), "release retires the scene transfer authority");
    while !source_document.close_step() {}
    while !destination_document.close_step() {}
    if let Some(document) = refreshed_document.as_mut() {
        while !document.close_step() {}
    }
}
