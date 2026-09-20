
use super::*;
use crate::dock::DockNode;

#[test]
fn retained_key_mapper_separates_select_typeahead_and_space_from_input_text() {
    let modifiers = PointerModifiers::default();
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("d".into()), &modifiers, true), Some(ui_wgpu::wgpu::UiEvent::KeyDown { key, .. }) if key == "d"));
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(true), &modifiers, true), Some(ui_wgpu::wgpu::UiEvent::KeyDown { key, .. }) if key == " "));
    assert!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(false), &modifiers, true).is_none());
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("d".into()), &modifiers, false), Some(ui_wgpu::wgpu::UiEvent::TextInput { text }) if text == "d"));
}

#[test]
fn accessibility_focus_arms_shell_select_space_and_typeahead_routing() {
    let surface = "control-focus-select";
    let records: Vec<ui_contract::UiNodeRecord> = serde_json::from_value(serde_json::json!([
        {
            "id": 1,
            "key": "framework.settings.appearance",
            "component": { "type": "select", "value": "system", "items": [{ "value": "system", "label": "System" }, { "value": "dark", "label": "Dark" }] },
            "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
            "style": {}, "activity": "idle", "accessibility": { "label": "Appearance" },
            "bindings": [{ "trigger": "change", "action": { "scope": "framework", "name": "setAppearance", "version": 1 } }]
        }
    ])).unwrap();
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell.publish_surface_records(surface, records).expect("Select surface publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, Rect::new(0.0, 0.0, 320.0, 120.0));
    shell.active_window_id = Some(surface.to_string());
    let target = ui_render::AccessibilityTarget { window_id: surface.to_string(), window_generation: 1, node_id: 1, node_key: "framework.settings.appearance".into() };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(shell.chrome_build.content_has_focus(surface));
    assert!(shell.retained_select_owns_keyboard());

    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "opening with Space commits nothing");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    let space = crate::collect_fixture_actions(&mut input);
    assert_eq!(space.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);

    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Char("d".into()), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "typeahead opens and highlights before commit");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Enter, &PointerModifiers::default(), &mut input)).unwrap();
    let typed = crate::collect_fixture_actions(&mut input);
    assert_eq!(typed.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);
    assert_eq!(typed[0].args.as_ref().and_then(|args| args.get("value")).and_then(DslValue::as_str), Some("dark"));
    while !document.close_step() {}
}

fn tree_pointer_record(
    id: u64,
    key: &str,
    component: ui_contract::Component,
    children: &[u64],
    action: Option<&str>,
) -> ui_contract::UiNodeRecord {
    let mut child_ids = ui_contract::UiNodeChildren::default();
    for child in children {
        child_ids.try_push(ui_contract::UiNodeId(*child)).expect("tree pointer fixture child");
    }
    let mut bindings = ui_contract::UiNodeBindings::default();
    if let Some(action) = action {
        bindings
            .try_push(ui_contract::ActionBinding {
                trigger: ui_contract::Trigger::Activate,
                action: ui_contract::ActionId::try_v1("s.test.tree", action).expect("bounded tree pointer action"),
                args: None,
                capability: None,
            })
            .expect("tree pointer fixture binding");
    }
    ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str(key).expect("bounded tree pointer key"),
        component,
        layout: PanelProjection::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings,
        menu: None,
        children: child_ids,
    }
}

fn published_tree_pointer_document(shell: &mut ShellState, surface: &str) -> (UiDocumentLease, String) {
    let mut item = UiTreeItemNode::base("transfer", Label::data("Transfer"));
    item.action = Some(ActionDescriptor { controller_id: "s.test.tree".into(), action: "selectTreeItem".into(), args: None });
    item.draggable = Some(true);
    item.drag_data = Some(HashMap::from([(
        "application/x-semio-window-template".into(),
        "{\"windowKindId\":\"world\",\"templateId\":\"default\"}".into(),
    )]));
    let node = UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "section".into(),
            label: None,
            default_open: Some(true),
            presence: UiPresence::default(),
            items: vec![item],
            window: None,
        }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    });
    let records = panel_ui_records(surface, &node).expect("the authored Tree projects through the production panel assembler");
    let item_key = records
        .iter()
        .find(|record| matches!(&record.component, ui_contract::Component::TreeItem(_)))
        .map(|record| record.key.as_str().to_string())
        .expect("the projected Tree carries its transfer row");
    (shell.publish_surface_records(surface, records).expect("tree pointer document publishes"), item_key)
}

fn published_canvas_catalogue_document(shell: &mut ShellState, surface: &str, raw_payload: &str) -> UiDocumentLease {
    let component = |wire: Value| serde_json::from_value(wire).expect("catalogue pointer component wire");
    let records = vec![
        tree_pointer_record(1, "root", component(serde_json::json!({ "type": "tree" })), &[2], None),
        tree_pointer_record(2, "section", component(serde_json::json!({ "type": "treeSection", "defaultOpen": true })), &[3], None),
        tree_pointer_record(
            3,
            "catalogue",
            component(serde_json::json!({
                "type": "treeItem",
                "label": "Catalogue item",
                "draggable": true,
                "dragData": {
                    "application/x-semio-catalogue-item": raw_payload,
                    "text/plain": "Catalogue item"
                }
            })),
            &[],
            None,
        ),
    ];
    shell.publish_surface_records(surface, records).expect("catalogue pointer document publishes")
}

fn paint_tree_pointer_document(shell: &mut ShellState, surface: &str, document: &UiDocumentLease, body: Rect) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut cursor = UiDocumentFrameCursor::default();
    let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
        let mut hosts = crate::scenes::SceneEngineHosts { world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
        let done = render_ui_document_step(&mut cursor, document, body, &mut ctx, surface, "s.test.tree", ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts);
        assert!(done || !cursor.terminal_is_fault(), "the tree pointer document faulted in phase {}", cursor.phase_name());
        done
    });
    assert!(complete, "the tree pointer document painted within its opportunity ceiling");
    crate::interpreter::begin_accessibility_visible_documents();
    shell.register_retained_body_hits(surface, body, &mut input);
    shell.publish_retained_hit_registry(&mut input);
    input
}

fn table_pointer_record(id: u64, key: &str, scene: &ui_wgpu::wgpu::TableScene) -> ui_contract::UiNodeRecord {
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::Table, scene).expect("bounded Table scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn canvas_pointer_record(id: u64, key: &str, scene: &ui_wgpu::wgpu::Canvas2dScene) -> ui_contract::UiNodeRecord {
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::Canvas2d, scene).expect("bounded Canvas2d scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn world3d_pointer_record(id: u64, key: &str, scene: &ui_wgpu::wgpu::World3dScene) -> ui_contract::UiNodeRecord {
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::World3d, scene).expect("bounded World3d scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn runnable_window_body_fixture(
    _instance_id: u32,
    surface_id: &str,
    body_key: &str,
    _view_state: &ViewModel,
    _document_dsl: Option<&str>,
    _refresh_effects: Option<&mut Vec<semio_framework::kernel::Effect>>,
) -> Result<UiDocumentLease, String> {
    for section in [
        semio_framework::UiRefreshSection::Catalogue,
        semio_framework::UiRefreshSection::Engagements,
        semio_framework::UiRefreshSection::Measures,
        semio_framework::UiRefreshSection::Tools,
    ] {
        if body_key == section.body_key() {
            return Ok(crate::program_bridge::window_measures_section_tests::section_document(&serde_json::json!({}), 1, section));
        }
    }
    let scene = ui_wgpu::wgpu::World3dScene::base(
        serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(),
        "[]".into(),
        "[]".into(),
        "{}".into(),
    );
    let records = vec![world3d_pointer_record(1, "initial-world", &scene)];
    let identity = ui_contract::UiDocumentAssemblyIdentity {
        generation: 1,
        revision: ui_contract::UiRevision(1),
        root: Some(ui_contract::UiNodeId(1)),
        layout_epoch: 0,
    };
    UiDocumentLease::try_publish(SurfaceId::try_from(surface_id).map_err(|_| "fixture surface exceeds the retained contract")?, identity, records)
        .map_err(|error| crate::program_bridge::retained_publication_refusal(surface_id, error))
}

std::thread_local! {
    static WINDOW_JOURNAL_FIXTURE_REFUSALS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_BODY_FIXTURE_REFUSALS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_BODY_FIXTURE_ATTEMPTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_PUBLICATION_FIXTURE_EVENTS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn scripted_window_body_fixture(
    instance_id: u32,
    surface_id: &str,
    body_key: &str,
    view_state: &ViewModel,
    document_dsl: Option<&str>,
    refresh_effects: Option<&mut Vec<semio_framework::kernel::Effect>>,
) -> Result<UiDocumentLease, String> {
    if surface_id == "main-2" {
        WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(attempts.get() + 1));
        WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().push("render:main-2".into()));
        let refused = WINDOW_BODY_FIXTURE_REFUSALS.with(|refusals| {
            let remaining = refusals.get();
            if remaining > 0 {
                refusals.set(remaining - 1);
                true
            } else {
                false
            }
        });
        if refused {
            return Err("fixture refuses the required initial window body".to_string());
        }
    } else if surface_id == "main-3" {
        WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().push("render:main-3".into()));
    }
    runnable_window_body_fixture(instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects)
}

fn refuse_window_body_fixture_action(
    _instance_id: u32,
    action_json: &str,
    _view_state: &ViewModel,
) -> Result<semio_framework::kernel::InvocationResult, String> {
    if action_json.contains("shell.windowSplit") || action_json.contains("shell.windowMove") {
        WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(count.get() + 1));
        let instance = if action_json.contains("main-2") {
            "main-2"
        } else if action_json.contains("main-3") {
            "main-3"
        } else {
            "unknown"
        };
        WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().push(format!("journal:{instance}")));
    }
    Err("fixture guest refuses the journal action".into())
}

fn shell_with_scripted_window_publication(refusals: usize) -> ShellState {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.dock.root = DockNode::Stack {
        windows: vec![DockStackTab::instance("main-2", "main", WindowStackCorner::TopLeft)],
        active: "main-2".into(),
    };
    shell.dock.active_window_id = Some("main-2".into());
    shell.active_window_id = Some("main-2".into());
    shell.persist_dock_layout();
    let program = shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the host fixture has its guest program");
    program.install_fixture_render(scripted_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_BODY_FIXTURE_REFUSALS.with(|remaining| remaining.set(refusals));
    WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(0));
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().clear());
    let controller_id = shell.shell_command_controller_id().expect("the fixture session journals transfers");
    let note = ShellState::note_shell_command_action(
        &controller_id,
        "shell.windowSplit",
        "Split Window",
        Some(serde_json::json!({ "windowKindId": "main", "instanceId": "main-2" })),
    );
    let token = shell.reserve_window_topology_action(note).expect("the fixture transfer journal is admitted");
    shell.commit_window_topology_publication("main-2", "main.body", Some(token));
    shell
}

fn retire_scripted_window_publication_documents(shell: &mut ShellState) {
    shell.retire_documents_outside(&[], true).expect("scripted window documents retire");
    shell.retire_documents_outside(&[], false).expect("scripted panel documents retire");
    let auxiliary = std::mem::take(&mut shell.window_actions_documents)
        .into_values()
        .chain(std::mem::take(&mut shell.window_search_documents).into_values())
        .chain(std::mem::take(&mut shell.window_measures_documents).into_values())
        .collect::<Vec<_>>();
    for document in auxiliary {
        shell.retire_one_surface_document(Some(document)).expect("scripted auxiliary document retires");
    }
    shell.drain_retained_document_arenas();
}

fn paint_component_pointer_documents(shell: &mut ShellState, documents: &[(&str, &str, &UiDocumentLease, Rect)]) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    crate::interpreter::begin_accessibility_visible_documents();
    for (window_id, controller_id, document, body) in documents {
        let mut cursor = UiDocumentFrameCursor::default();
        let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
            let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
            let mut hosts = crate::scenes::SceneEngineHosts { world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id };
            let done = render_ui_document_step(&mut cursor, document, *body, &mut ctx, window_id, controller_id, ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts);
            assert!(done || !cursor.terminal_is_fault(), "the component pointer document faulted in phase {}", cursor.phase_name());
            done
        });
        assert!(complete, "the component pointer document painted within its opportunity ceiling");
        shell.register_retained_body_hits(window_id, *body, &mut input);
    }
    shell.publish_retained_hit_registry(&mut input);
    input
}

#[test]
fn required_window_body_refusal_holds_the_transfer_journal_then_recovers_once() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"
    )))
    .expect("window publication fixture");
    let recovery = fixture["publicationOutcomes"]["recovery"].as_array().expect("recovery sequence");
    let mut shell = shell_with_scripted_window_publication(1);

    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), 1);
    assert!(!shell.window_ui.contains_key("main-2"));
    assert!(shell.window_topology_refresh_owed);
    assert_eq!(shell.window_topology_actions.len(), 1);
    assert!(shell.deferred_actions.is_empty());
    assert_eq!(shell.surface_faults.iter().any(|fault| fault.surface_id == "main-2"), recovery[0]["surfaceFault"].as_bool().unwrap());

    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), 2);
    assert!(shell.window_ui.contains_key("main-2"));
    assert_eq!(shell.window_topology_refresh_owed, recovery[1]["topologyOwed"].as_bool().unwrap());
    assert!(shell.window_topology_actions.is_empty());
    assert!(shell.window_topology_publications.is_empty());
    assert_eq!(shell.deferred_actions.iter().filter(|action| action.action == "noteShellCommand").count(), 1);
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0);

    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 1, "recovery releases exactly one journal on the following settle step");
    assert!(shell.deferred_actions.is_empty());
    retire_scripted_window_publication_documents(&mut shell);
}

#[test]
fn permanent_window_body_refusal_retires_the_journal_at_the_fixture_ceiling() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"
    )))
    .expect("window publication fixture");
    let outcomes = &fixture["publicationOutcomes"];
    let retry_ceiling = outcomes["retryCeiling"].as_u64().expect("retry ceiling") as usize;
    assert_eq!(retry_ceiling, usize::from(WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS));
    let mut shell = shell_with_scripted_window_publication(retry_ceiling);

    for attempt in 0..retry_ceiling {
        assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
        if attempt + 1 < retry_ceiling {
            assert!(shell.window_topology_refresh_owed);
            assert_eq!(shell.window_topology_actions.len(), 1);
        }
    }
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), retry_ceiling);
    assert!(!shell.window_topology_refresh_owed);
    assert!(shell.window_topology_actions.is_empty());
    assert!(shell.window_topology_publications.is_empty());
    assert!(shell.deferred_actions.is_empty());
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0, "a permanently refused body never emits a success-shaped journal");
    let fault = shell.surface_faults.iter().find(|fault| fault.surface_id == "main-2").expect("terminal publication fault");
    assert!(fault.detail.contains(&format!("after {retry_ceiling} attempts")));
    assert_eq!(outcomes["permanentFault"]["journal"], "terminal-refusal");
    assert_eq!(outcomes["permanentFault"]["surfaceFault"], true);
    retire_scripted_window_publication_documents(&mut shell);
}

#[test]
fn topology_journal_credit_refusal_is_returned_to_the_transfer_caller() {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    for _ in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        shell
            .reserve_window_topology_action(ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: None })
            .expect("the occupied fixture journal publishes");
    }
    let error = shell
        .reserve_window_topology_action(ActionDescriptor { controller_id: "test".into(), action: "noteShellCommand".into(), args: None })
        .expect_err("a full action queue returns its exact refusal");
    assert_eq!(error, ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    assert!(shell.window_topology_publications.is_empty(), "refusal precedes every topology owner");
    assert!(shell.deferred_actions.is_empty());
}

fn canvas_input_fixture() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../🧱️elements/📐️Canvas2dHost/🧫️fixtures/🖱️input-contract/🔣️.json"
    )))
    .expect("shared Canvas2d input fixture")
}

fn assert_json_number_eq(actual: &Value, expected: &Value, field: &str) {
    assert_eq!(actual.as_f64().unwrap_or_else(|| panic!("{field} actual number")), expected.as_f64().unwrap_or_else(|| panic!("{field} expected number")), "{field}");
}

fn canvas_action_args(action: &ActionDescriptor) -> Value {
    dsl_value_as_json(action.args.as_ref().expect("Canvas2d action args"))
}
#[test]
fn standalone_multi_app_variants_resolve_their_declared_app() {
    assert_eq!(resolve_playground_app_id("puzzle2d"), Some("s.puzzle.puzzle2d@1/*#editor"));
    assert_eq!(resolve_playground_app_id("puzzle3d"), Some("s.puzzle.puzzle3d@1/*#editor"));
    assert_eq!(resolve_playground_app_id("3d"), Some("s.puzzle.puzzle3d@1/*#editor"));
    assert_eq!(resolve_playground_app_id("puzzle5d"), Some("s.puzzle.puzzle5d@1/*#editor"));
    assert_eq!(resolve_registry_plugin_id("generation3d"), "procedural");
    assert_eq!(resolve_playground_app_id("generation3d"), Some("s.procedural.generation3d@1/*#editor"));
}

//#region SilhouetteContentTests

#[test]
fn silhouette_hit_intersections_leave_the_cap_gap_empty() {
    let silhouette = WindowSilhouette::from_measured_top(Rect::new(0.0, 0.0, 300.0, 200.0), 80.0, 60.0, 32.0);
    let hits: Vec<Rect> = silhouette.content_clip_rects().iter().filter_map(|clip| ShellState::intersect_content_rect(*clip, silhouette.bounds)).collect();
    assert_eq!(hits.len(), 3);
    assert!(!hits.iter().any(|rect| rect.contains(150.0, 16.0)));
    assert!(hits.iter().any(|rect| rect.contains(150.0, 100.0)));
}

//#endregion SilhouetteContentTests

/// 🧪️ `dock_window_order` is a `Self`-less associated fn, so it's callable without constructing a
/// full `ShellState` fixture (impractically large: 90+ fields, several without `Default`).
fn window_ids(node: &crate::dock::DockNode) -> Vec<String> {
    let mut out = Vec::new();
    ShellState::dock_window_order(node, &mut Vec::new(), &mut out);
    out.into_iter().map(|(_, window_id)| window_id).collect()
}

#[test]
fn dock_window_order_flattens_a_single_stack_in_tab_order() {
    let node = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() };
    assert_eq!(window_ids(&node), vec!["a", "b", "c"]);
}

#[test]
fn dock_window_order_walks_row_and_column_children_depth_first() {
    let left = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("left")], active: "left".into() };
    let right_top = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("top")], active: "top".into() };
    let right_bottom = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("bottom")], active: "bottom".into() };
    let right = crate::dock::DockNode::Column(vec![(right_top, 0.5), (right_bottom, 0.5)]);
    let root = crate::dock::DockNode::Row(vec![(left, 0.5), (right, 0.5)]);
    assert_eq!(window_ids(&root), vec!["left", "top", "bottom"]);
}

#[test]
fn dock_window_order_pairs_each_window_with_its_own_stack_path() {
    let a = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a")], active: "a".into() };
    let b = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("b")], active: "b".into() };
    let root = crate::dock::DockNode::Row(vec![(a, 0.5), (b, 0.5)]);
    let mut out = Vec::new();
    ShellState::dock_window_order(&root, &mut Vec::new(), &mut out);
    assert_eq!(out, vec![(vec![0], "a".to_string()), (vec![1], "b".to_string())]);
}

// 🎯️🕹️ w2-input-wiring: key mapping is pure and focus tracking is explicit owned build state,
// so both remain testable without a full `ShellState` fixture.

#[test]
fn ui_event_from_key_action_maps_plain_char_to_text_input() {
    let modifiers = PointerModifiers::default();
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("a".into()), &modifiers, false);
    assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::TextInput { text: "a".into() }));
}

#[test]
fn ui_event_from_key_action_routes_ctrl_char_as_key_down_for_clipboard_chords() {
    let modifiers = PointerModifiers { ctrl: true, ..Default::default() };
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("c".into()), &modifiers, false);
    assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "c".into(), modifiers: ui_wgpu::wgpu::EventModifiers { shift: false, ctrl: true, alt: false, meta: false } }));
}

#[test]
fn ui_event_from_key_action_maps_editing_and_tab_keys_to_matching_key_down_strings() {
    let modifiers = PointerModifiers::default();
    let cases = [
        (ui_wgpu::wgpu::KeyAction::Backspace, "Backspace"),
        (ui_wgpu::wgpu::KeyAction::Delete, "Delete"),
        (ui_wgpu::wgpu::KeyAction::Enter, "Enter"),
        (ui_wgpu::wgpu::KeyAction::Escape, "Escape"),
        (ui_wgpu::wgpu::KeyAction::ArrowLeft, "ArrowLeft"),
        (ui_wgpu::wgpu::KeyAction::ArrowRight, "ArrowRight"),
        (ui_wgpu::wgpu::KeyAction::ArrowUp, "ArrowUp"),
        (ui_wgpu::wgpu::KeyAction::ArrowDown, "ArrowDown"),
        (ui_wgpu::wgpu::KeyAction::Tab, "Tab"),
    ];
    for (action, key) in cases {
        let event = ui_event_from_key_action(&action, &modifiers, false);
        assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: key.into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }), "KeyAction {action:?} should map to KeyDown{{{key}}}");
    }
}

#[test]
fn ui_event_from_key_action_has_no_mapping_for_space() {
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), false);
    assert_eq!(event, None);
}

#[test]
fn content_focus_tracker_defaults_unfocused_and_tracks_focus_changed_commands() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-a";
    assert!(!chrome.content_has_focus(window_id));
    let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
    let node_id = arena.insert(());
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.to_string(), node: Some(node_id) }]);
    assert!(chrome.content_has_focus(window_id));
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.to_string(), node: None }]);
    assert!(!chrome.content_has_focus(window_id));
}

#[test]
fn content_focus_tracker_ignores_commands_for_other_windows() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-b";
    let other_window_id = "w2-input-wiring-test-window-c";
    let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
    let node_id = arena.insert(());
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: other_window_id.to_string(), node: Some(node_id) }]);
    assert!(!chrome.content_has_focus(window_id));
    assert!(chrome.content_has_focus(other_window_id));
}

#[test]
fn content_focus_tracker_ignores_non_focus_commands() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-d";
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::ClipboardPasteRequested { window_id: window_id.to_string() }]);
    assert!(!chrome.content_has_focus(window_id));
}

/// 🕒️ `finish_dock_drag`'s successful-drop branch persists the new layout and clears the drag —
/// unchanged behavior this ticket's `noteShellCommand` dispatch is appended after, not instead of.
/// No host app is configured (`ShellState::new`'s bare fixture, same "impractically large" 90+-field
/// constraint as `dock_window_order`'s own fixture note above), so `host_controller_id()` is `None`
/// and the new dispatch is a documented no-op here — this pins the "must still complete without a
/// host to log against" half of that behavior; `note_shell_command_action`'s own shape (the other
/// half) is covered directly in `command_registry_tests`.
///
/// 🎬️ A live drag never edits the committed tree, so the drop zone is a path into
/// `DockState::render_view`'s derivation — lifting `a` out of the lone axis child hoists the stack
/// to the ROOT, exactly as React's `collapseLayout` does.
#[test]
fn finish_dock_drag_persists_layout_and_clears_drag_state_on_successful_drop() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock.root = crate::dock::DockNode::Row(vec![(crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() }, 1.0)]);
    let payload = DockDragPayload { kind: DockDragKind::Tab, window_id: "a".into(), window_kind_id: "a".into(), template_id: None, source_path: vec![0], tab_index: 0, ghost_label: "a".into() };
    let zone = DockDropZone::Tab { stack_path: vec![], corner: WindowStackCorner::TopLeft, index: 2 };
    shell.dock_drag = Some(DockDragState { payload, x: 10.0, y: 10.0, drop_zone: Some(zone) });
    assert!(shell.layout_override.is_none(), "sanity: nothing persisted yet");
    let input = InputState::<ActionDescriptor>::default();
    let result = semio_framework_async::block_on(shell.finish_dock_drag(10.0, 10.0, &input));
    assert!(result.is_ok(), "finish_dock_drag must not error even without a host app to log a shell.windowMove against");
    assert!(shell.layout_override.is_some(), "a successful drop persists the new dock layout");
    assert!(shell.dock_drag.is_none(), "the transient drag state is always taken");
}

#[test]
fn context_menu_point_resolves_the_exact_concrete_window_instance() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock_drop_bodies = vec![(Vec::new(), Rect::new(0.0, 0.0, 100.0, 100.0), "canvas".into()), (vec![1], Rect::new(100.0, 0.0, 100.0, 100.0), "canvas-copy".into())];
    assert_eq!(shell.context_window_instance_id(25.0, 25.0), Some("canvas"));
    assert_eq!(shell.context_window_instance_id(125.0, 25.0), Some("canvas-copy"));
    assert_eq!(shell.context_window_instance_id(250.0, 25.0), None);
    println!("[DEBUG] native context menu resolved its exact concrete window and preserved panel scope outside dock bodies");
}

/// ⚖️ LAW: pressing a window's retained BODY activates that window, so the keyboard follows the
/// window the user actually clicked into rather than whichever one the session opened with.
///
/// 🩸️ `handle_keyboard_async` routes real keys into retained content on exactly one predicate —
/// `content_has_focus(active_window_id)` — and a retained body press was the one way into a window
/// that never set `active_window_id`. Measured on 6118: pressing the Generations window's inline
/// rename editor logged `content focus window=generation3d-generations node=Some(..)` and then every
/// keystroke logged `key routing window=procedural-main contentFocus=false`, so the editor opened,
/// took focus, and could not be typed into (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-generation-publication-2026-09-13.md`).
#[test]
fn a_retained_body_press_activates_its_own_window_so_the_keyboard_follows_it() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let opened_with = "procedural-main";
    let pressed = "generation3d-generations";
    shell.active_window_id = Some(opened_with.into());
    let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
    let node_id = arena.insert(());
    shell.chrome_build.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: pressed.to_string(), node: Some(node_id) }]);
    assert!(!shell.chrome_build.content_has_focus(opened_with), "sanity: the window the keyboard used to follow never had content focus");

    let mut input = InputState::<ActionDescriptor>::default();
    let body = Rect::new(3.0, 54.0, 315.0, 814.0);
    semio_framework_async::block_on(shell.route_retained_pointer_press(pressed, body, 255.0, 90.0, true, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).expect("a retained body press routes");

    assert_eq!(shell.active_window_id.as_deref(), Some(pressed), "the pressed window is the active one");
    assert!(shell.chrome_build.content_has_focus(shell.active_window_id.as_deref().expect("an active window")), "…so the keyboard's own predicate now answers for the window whose content holds focus");
    println!("[DEBUG] retained body press moved the active window {opened_with} -> {pressed}");
}

/// ⚖️ LAW: the RELEASE half of the same click changes nothing about activation — a click activates on
/// the press, the way every window manager and every browser does, and the action fires on release
/// (`route_retained_pointer_press`'s own `else if !down` arm).
#[test]
fn only_the_press_half_of_a_body_click_moves_the_active_window() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.active_window_id = Some("procedural-main".into());
    let mut input = InputState::<ActionDescriptor>::default();
    let body = Rect::new(3.0, 54.0, 315.0, 814.0);
    semio_framework_async::block_on(shell.route_retained_pointer_press("generation3d-generations", body, 255.0, 90.0, false, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).expect("a retained body release routes");
    assert_eq!(shell.active_window_id.as_deref(), Some("procedural-main"), "a release alone never activates — the press already did, or nothing did");
}

/// ⚖️ LAW: the production Shell host routes BOTH semantic targets of a published handle-driven
/// Tree through the retained router. The trailing handle arms and promotes the retained drag across
/// the real down/move/up ingress without firing the row selection, while the remaining label band
/// still commits that row's published `Activate` binding on a later click.
#[test]
fn published_tree_handle_routes_through_shell_and_preserves_label_selection() {
    let surface = "tree-pointer-host-boundary";
    let body = Rect::new(17.0, 31.0, 360.0, 240.0);
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let (mut document, item_key) = published_tree_pointer_document(&mut shell, surface);
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    assert_eq!(crate::interpreter::accessibility_visible_window_ids(), vec![surface.to_string()], "the same completed retained-body publication owns the unnamed accessibility surface set");

    let handle_id = format!("tree.drag.transfer.{item_key}");
    let handle = input
        .hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some(handle_id.as_str()))
        .expect("the published Handle driver exposes the transfer affordance")
        .rect;
    let row_id = format!("tree.label.{item_key}");
    let row = input
        .hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some(row_id.as_str()))
        .expect("the same published row keeps its label target")
        .rect;
    let (handle_x, handle_y) = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(handle_x, handle_y, true, 0, &mut input, &theme)).expect("Shell routes handle down");
    shell.handle_pointer_move(handle_x - 8.0, handle_y, true, &mut input, &theme);
    let sessions = crate::interpreter::active_retained_drag_sessions();
    assert_eq!(sessions.len(), 1, "the Shell move promotes the retained Tree press to one drag session");
    assert_eq!(sessions[0].0, surface);
    assert_eq!(sessions[0].3.get("application/x-semio-window-template").map(String::as_str), Some("{\"windowKindId\":\"world\",\"templateId\":\"default\"}"));
    semio_framework_async::block_on(shell.handle_pointer_button(handle_x - 8.0, handle_y, false, 0, &mut input, &theme)).expect("Shell routes handle up");
    assert!(crate::interpreter::active_retained_drag_sessions().is_empty(), "release retires the retained drag session");
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a handle gesture never aliases the row-label selection");

    let (label_x, label_y) = (row.x + theme.gap_standard.max(1.0), row.y + row.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(label_x, label_y, true, 0, &mut input, &theme)).expect("Shell routes label down");
    semio_framework_async::block_on(shell.handle_pointer_button(label_x, label_y, false, 0, &mut input, &theme)).expect("Shell routes label up");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1, "the label click still commits exactly one selection action");
    assert_eq!(actions[0].controller_id, "s.test.tree");
    assert_eq!(actions[0].action, "selectTreeItem");
    let args = dsl_value_as_json(actions[0].args.as_ref().expect("the retained host scopes the selection"));
    assert_eq!(args["windowId"], surface);
    crate::interpreter::begin_accessibility_visible_documents();
    crate::interpreter::publish_accessibility_visible_documents();
    assert!(crate::interpreter::accessibility_visible_window_ids().is_empty(), "a later complete walk with no retained body removes the live document from production accessibility without retiring it");
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
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut node = shell.build_display_windows_ui();
    let UiNode::Stack(stack) = &mut node else { panic!("Display Windows publishes a stack") };
    let Some(UiNode::Tree(tree)) = stack.children.first_mut() else { panic!("Display Windows stack publishes a tree") };
    tree.sections
        .iter_mut()
        .find(|section| section.id == "framework.display.windows.main")
        .expect("the real app's main window-kind section")
        .default_open = Some(true);
    let records = panel_ui_records(surface, &node).expect("the real Display producer projects");
    let projected_item_key = records
        .iter()
        .find(|record| record.key.as_str().ends_with("framework.display.windows.main.kind") && matches!(&record.component, ui_contract::Component::TreeItem(_)))
        .map(|record| record.key.as_str().to_string())
        .expect("the real Display producer projects its transfer row");
    let mut document = shell.publish_surface_records(surface, records).expect("the projected Display document acquires a retained lease");
    let input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    let transfer_control_id = format!("tree.drag.transfer.{projected_item_key}");
    let handle = input
        .hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some(transfer_control_id.as_str()))
        .expect("the real Display producer paints a transfer handle");
    let source = (handle.rect.x + handle.rect.w * 0.5, handle.rect.y + handle.rect.h * 0.5);
    shell.dock = DockState::default();
    shell.dock_canvas_bounds = Rect::new(500.0, 0.0, 400.0, 600.0);
    shell.dock_drop_tab_bars.clear();
    shell.dock_drop_bodies.clear();
    *shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft) = vec![DockTabNode::leaf(surface, "Windows", "panels-top-left", 0)];
    shell.anchor_state_mut(PanelAnchor::TopLeft).visible = true;
    shell.anchor_state_mut(PanelAnchor::TopLeft).path = vec![surface.to_string()];
    shell.active_window_id = Some("previous-app-window".into());
    DisplayTransferHostFixture { shell, document, input, theme, source }
}

fn perform_display_transfer(fixture: &mut DisplayTransferHostFixture, drop: (f32, f32)) -> Result<(), String> {
    let (source_x, source_y) = fixture.source;
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(source_x, source_y, true, 0, &mut fixture.input, &fixture.theme))?;
    fixture.shell.handle_pointer_move(drop.0, drop.1, true, &mut fixture.input, &fixture.theme);
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(drop.0, drop.1, false, 0, &mut fixture.input, &fixture.theme))
}

fn close_display_transfer_host_fixture(mut fixture: DisplayTransferHostFixture) {
    while !fixture.document.close_step() {}
}

#[test]
fn cancelled_display_transfer_retires_capture_without_creating_a_window() {
    let mut fixture = display_transfer_host_fixture();
    let before = fixture.shell.dock.window_instances().len();
    let (x, y) = fixture.source;
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(x, y, true, 0, &mut fixture.input, &fixture.theme)).expect("Display transfer press");
    fixture.shell.handle_pointer_move(800.0, 500.0, true, &mut fixture.input, &fixture.theme);
    fixture.shell.handle_pointer_cancel(&mut fixture.input);
    assert!(crate::interpreter::retained_pointer_capture_window().is_none());
    assert!(fixture.shell.dock_drag.is_none() && fixture.shell.pending_dock_drag.is_none());
    assert!(!fixture.input.pointer_down && !fixture.input.drag.active);
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(800.0, 500.0, false, 0, &mut fixture.input, &fixture.theme)).expect("late release stays inert");
    assert_eq!(fixture.shell.dock.window_instances().len(), before);
    assert!(fixture.shell.window_topology_actions.is_empty());
    close_display_transfer_host_fixture(fixture);
}

#[test]
fn refused_window_publication_does_not_block_a_ready_display_peer() {
    let contract: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"
    )))
    .expect("window publication fixture");
    let cohort = &contract["publicationOutcomes"]["cohort"];
    let mut fixture = display_transfer_host_fixture();
    let program = fixture.shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the actual Display fixture has its guest ProgramBridge");
    program.install_fixture_render(scripted_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_BODY_FIXTURE_REFUSALS.with(|remaining| remaining.set(1));
    WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(0));
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().clear());

    perform_display_transfer(&mut fixture, (800.0, 500.0)).expect("the first actual Display transfer commits");
    perform_display_transfer(&mut fixture, (750.0, 300.0)).expect("the second actual Display transfer commits");
    assert_eq!(fixture.shell.dock.window_instances().len(), 2);
    assert_eq!(fixture.shell.window_topology_actions.len(), 2);
    assert_eq!(fixture.shell.window_topology_publications.len(), 2);

    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert!(!fixture.shell.window_ui.contains_key(cohort["refused"].as_str().unwrap()));
    assert!(fixture.shell.window_ui.contains_key(cohort["ready"].as_str().unwrap()));
    assert_eq!(fixture.shell.window_topology_actions.len(), 1, "the ready peer releases its token during the first cohort refresh");
    assert_eq!(fixture.shell.deferred_actions.len(), 1);
    let ready_journal = dsl_value_as_json(fixture.shell.deferred_actions[0].args.as_ref().expect("the ready journal carries its instance"));
    assert_eq!(ready_journal["detail"]["instanceId"], cohort["ready"]);

    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());
    let ready_dispatch = events.iter().position(|event| event == "journal:main-3").expect("the ready peer dispatches");
    let refused_retry = events
        .iter()
        .enumerate()
        .filter(|(_, event)| event.as_str() == "render:main-2")
        .nth(1)
        .map(|(index, _)| index)
        .expect("the refused peer retries");
    assert!(ready_dispatch < refused_retry, "the ready journal dispatches before the unrelated retry");
    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());
    for instance in ["main-2", "main-3"] {
        assert_eq!(events.iter().filter(|event| event.as_str() == format!("journal:{instance}")).count(), 1, "{instance} dispatches exactly once");
        assert_eq!(contract["publicationOutcomes"]["cohort"]["afterRetryDispatches"][instance], 1);
    }
    assert!(fixture.shell.window_topology_publications.is_empty());
    assert!(fixture.shell.window_topology_actions.is_empty());
    assert!(fixture.shell.window_topology_action_tokens.is_empty());
    retire_scripted_window_publication_documents(&mut fixture.shell);
    close_display_transfer_host_fixture(fixture);
}

#[test]
fn actual_display_release_refuses_item_and_byte_credit_before_dock_mutation() {
    let contract: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"
    )))
    .expect("window publication fixture");
    let refusals = contract["publicationOutcomes"]["atomicCreditRefusals"].as_array().expect("credit vectors");

    let mut item = display_transfer_host_fixture();
    let item_root = item.shell.dock.root.clone();
    for _ in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        item.shell
            .reserve_window_topology_action(ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: None })
            .expect("item-credit fixture admission");
    }
    let item_error = perform_display_transfer(&mut item, (800.0, 500.0)).expect_err("the physical release observes item refusal");
    assert!(item_error.contains("ItemCredits"));
    assert_eq!(item.shell.dock.root, item_root);
    assert!(item.shell.window_topology_publications.is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window().is_none(), "a refused physical release still retires capture");
    assert!(item.shell.pending_dock_drag.is_none() && item.shell.dock_drag.is_none());
    assert_eq!(refusals[0]["dockMutation"], "none");
    close_display_transfer_host_fixture(item);

    let mut bytes = display_transfer_host_fixture();
    let byte_root = bytes.shell.dock.root.clone();
    let chunk = "x".repeat(3_900);
    let mut admitted = 0usize;
    loop {
        let action = ActionDescriptor {
            controller_id: "fixture".into(),
            action: "occupied".into(),
            args: crate::action_args_json!({ "a": chunk.clone(), "b": chunk.clone(), "c": chunk.clone(), "d": chunk.clone() }),
        };
        match bytes.shell.reserve_window_topology_action(action) {
            Ok(_) => admitted += 1,
            Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => break,
            Err(error) => panic!("byte fixture reached the wrong admission fault: {error:?}"),
        }
    }
    assert!(admitted < ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY, "byte credits exhaust before item credits");
    let byte_error = perform_display_transfer(&mut bytes, (800.0, 500.0)).expect_err("the physical release observes byte refusal");
    assert!(byte_error.contains("ByteCredits"));
    assert_eq!(bytes.shell.dock.root, byte_root);
    assert!(bytes.shell.window_topology_publications.is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window().is_none());
    assert!(bytes.shell.pending_dock_drag.is_none() && bytes.shell.dock_drag.is_none());
    assert_eq!(refusals[1]["dockMutation"], "none");
    close_display_transfer_host_fixture(bytes);
}

/// ⚖️ LAW: Display's real window-kind producer keeps its transfer payload through panel projection,
/// retained reconciliation and paint, so the published Shell ingress starts a NewWindow dock drag.
#[test]
fn display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag() {
    let surface = "display-transfer-host-boundary";
    let body = Rect::new(19.0, 37.0, 420.0, 300.0);
    let theme = Theme::default();
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut node = shell.build_display_windows_ui();
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
    let mut document = shell.publish_surface_records(surface, records).expect("the projected Display document acquires a retained lease");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    let handle = input
        .hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some(transfer_control_id.as_str()))
        .expect("the reconciled and painted window kind publishes its transfer handle");
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref() == Some(sort_control_id.as_str())), "a payload-bearing row never degrades to a sort handle");
    let drag_data = handle.drag_data.as_ref().expect("the transfer handle owns the producer's payload");
    let payload = decode_window_template_drag(drag_data).expect("the retained MIME payload stays decodable");
    assert_eq!(payload.window_kind_id, "main");
    assert_eq!(payload.template_id, None);

    shell.dock = DockState::default();
    shell.dock_canvas_bounds = Rect::new(500.0, 0.0, 400.0, 600.0);
    shell.dock_drop_tab_bars.clear();
    shell.dock_drop_bodies.clear();
    *shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft) = vec![DockTabNode::leaf(surface, "Windows", "panels-top-left", 0)];
    shell.anchor_state_mut(PanelAnchor::TopLeft).visible = true;
    shell.anchor_state_mut(PanelAnchor::TopLeft).path = vec![surface.to_string()];
    shell.active_window_id = Some("previous-app-window".into());
    let (x, y) = (handle.rect.x + handle.rect.w * 0.5, handle.rect.y + handle.rect.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Shell routes the published transfer handle");
    assert_eq!(shell.active_window_id.as_deref(), Some("previous-app-window"), "a retained panel press never invents the panel as an application window");
    let (pending, _) = shell.pending_dock_drag.as_ref().expect("the host ingress arms a dock drag");
    assert_eq!(pending.kind, DockDragKind::NewWindow);
    assert_eq!(pending.window_kind_id, "main");
    assert_eq!(pending.template_id, None);

    shell.handle_pointer_move(800.0, 500.0, true, &mut input, &theme);
    assert!(shell.pending_dock_drag.is_none());
    assert!(shell.dock_drag.is_some(), "the captured retained move promotes the transfer into the dock authority");
    assert!(input.hit_at(800.0, 500.0).is_none(), "the runtime failure's release point owns no current retained hit");
    semio_framework_async::block_on(shell.handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("a hitless release commits through retained capture");
    assert_eq!(shell.dock.window_instances(), vec![("main-2".to_string(), "main".to_string())], "the empty dock receives exactly one new window");
    assert!(shell.window_topology_refresh_owed, "the successful topology mutation owes the new roster's first Full refresh");
    assert!(matches!(shell.owed_refresh_scope, UiDirtyScope::Full));
    assert_eq!(shell.window_topology_actions.len(), 1, "the gesture receives exactly one bounded journal admission");
    assert_eq!(shell.window_topology_publications.len(), 1);
    assert!(shell.window_topology_publications[0].journal_token.is_some());
    assert!(shell.deferred_actions.is_empty(), "the guest journal cannot run before the new window body is published");
    assert!(crate::interpreter::retained_pointer_capture_window().is_none(), "the hitless release retires retained capture");
    assert!(shell.pending_dock_drag.is_none() && shell.dock_drag.is_none());
    semio_framework_async::block_on(shell.handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("a duplicate release is inert");
    assert_eq!(shell.dock.window_instances().len(), 1, "one gesture cannot create a second window");

    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("a second transfer can arm after capture retirement");
    shell.handle_pointer_move(1000.0, 700.0, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(1000.0, 700.0, false, 0, &mut input, &theme)).expect("an outside release cancels cleanly");
    assert_eq!(shell.dock.window_instances().len(), 1, "an outside drop does not create a window");
    assert!(crate::interpreter::retained_pointer_capture_window().is_none());
    assert!(shell.pending_dock_drag.is_none() && shell.dock_drag.is_none());

    let scene = ui_wgpu::wgpu::World3dScene::base(
        serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(),
        "[]".into(),
        "[]".into(),
        "{}".into(),
    );
    let body_document = shell
        .publish_surface_records("main-2", vec![world3d_pointer_record(1, "initial-world", &scene)])
        .expect("the guest producer publishes the created instance's retained body");
    shell.window_ui.insert("main-2".into(), body_document);
    shell.complete_window_topology_refresh();
    assert!(!shell.window_topology_refresh_owed);
    assert!(shell.window_topology_actions.is_empty());
    assert!(shell.window_topology_publications.is_empty());
    assert_eq!(shell.deferred_actions.len(), 1, "one admitted topology journal is released after body publication");
    let journal = &shell.deferred_actions[0];
    assert_eq!(journal.action, "noteShellCommand");
    let journal_args = dsl_value_as_json(journal.args.as_ref().expect("the topology journal carries its command"));
    assert_eq!(journal_args["commandId"], "shell.windowSplit");
    assert_eq!(journal_args["detail"]["windowKindId"], "main");
    assert_eq!(journal_args["detail"]["instanceId"], "main-2");
    shell.complete_window_topology_refresh();
    assert_eq!(shell.deferred_actions.len(), 1, "completion acknowledgement cannot duplicate the journal");

    let bounds = Rect::new(0.0, 0.0, 900.0, 600.0);
    shell.screen_w = bounds.w;
    shell.screen_h = bounds.h;
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
    let painted = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        shell.render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut body_input, &theme, bounds, &mut world_resources)
    });
    assert!(painted, "the canonical Shell window walk consumes the created instance's initial body");
    assert!(shell.world3d_states.contains_key("main-2"), "the published body registers a live World3d owner before any example switch");
    assert!(body_input.staged_hits().iter().any(|hit| hit.kind == HitKind::World3d), "the canonical window walk stages the new instance's physical World3d hit target");
    body_input.publish_hits();
    assert!(body_input.hits().iter().any(|hit| hit.kind == HitKind::World3d), "the new instance publishes a physical World3d hit target");

    let world_token = shell.world3d_states.token("main-2").expect("the live World3d owner has generation identity");
    assert!(shell.close_dock_window("main-2"));
    shell.plan_dock_windows(bounds, &theme, &mut atlas);
    shell.sync_engine_surface_states();
    assert!(!shell.world3d_states.contains_key("main-2"));
    assert!(shell.world3d_states.get_token(world_token).is_none(), "the closed instance's generation cannot be recovered");
    assert_eq!(shell.retired_world3d_states.len(), 1, "the closed owner enters bounded progressive retirement");
    shell.retire_documents_outside(&[], true).expect("the closed retained body enters document retirement");
    assert!(!shell.window_ui.contains_key("main-2"));
    shell.deferred_actions.clear();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.retired_world3d_states.is_empty() {
            break;
        }
        shell.advance_world3d_retirement_step();
    }
    assert!(shell.retired_world3d_states.is_empty(), "the retired World3d owner reaches terminal empty under maintenance");

    let program = shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the host fixture has its guest program");
    program.install_fixture_render(runnable_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("the real Display producer arms a fresh transfer");
    shell.handle_pointer_move(800.0, 500.0, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("the real Display producer creates the fresh instance");
    assert!(shell.window_topology_refresh_owed);
    assert!(!shell.window_ui.contains_key("main-2"), "the new topology has no body before the settle lane runs");
    let settle = semio_framework_async::block_on(shell.settle_pump_step_inner());
    assert_eq!(settle, ShellSettleStep::Drained, "the topology debt consumes one bounded settle step");
    assert!(!shell.window_topology_refresh_owed);
    assert!(shell.window_ui.contains_key("main-2"), "refresh_ui fetches the new roster's body through the runnable ProgramBridge fixture");
    assert_eq!(shell.deferred_actions.iter().filter(|action| action.action == "noteShellCommand").count(), 1, "the first body refresh releases exactly one admitted topology journal");
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0, "the journal cannot overtake initial body publication");
    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained, "the following bounded step owns guest dispatch");
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 1, "the topology journal is refused exactly once");
    assert!(shell.deferred_actions.is_empty(), "the refused journal retains no stale action owner");

    shell.plan_dock_windows(bounds, &theme, &mut atlas);
    let mut refreshed_draw = DrawList::default();
    refreshed_draw.set_screen_height(bounds.h);
    let mut refreshed_overlay_draw = DrawList::default();
    refreshed_overlay_draw.set_screen_height(bounds.h);
    let mut refreshed_overlay = Some(&mut refreshed_overlay_draw);
    let mut refreshed_input = InputState::<ActionDescriptor>::default();
    let mut refreshed_cursor = ShellChromeChildCursor::default();
    let refreshed = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        shell.render_main_window_step(
            &mut refreshed_cursor,
            &mut refreshed_draw,
            &mut refreshed_overlay,
            &mut atlas,
            &icons,
            &mut refreshed_input,
            &theme,
            bounds,
            &mut world_resources,
        )
    });
    assert!(refreshed, "the body returned by refresh_ui enters the canonical Shell paint walk");
    assert!(refreshed_input.staged_hits().iter().any(|hit| hit.kind == HitKind::World3d));
    refreshed_input.publish_hits();
    assert!(refreshed_input.hits().iter().any(|hit| hit.kind == HitKind::World3d), "the actual refreshed body publishes a physical World3d hit before any example switch");

    assert!(shell.close_dock_window("main-2"));
    shell.plan_dock_windows(bounds, &theme, &mut atlas);
    shell.sync_engine_surface_states();
    shell.retire_documents_outside(&[], true).expect("the refreshed window body retires");
    shell.retire_documents_outside(&[], false).expect("the runnable fixture's panel bodies retire");
    let auxiliary_documents = std::mem::take(&mut shell.window_actions_documents)
        .into_values()
        .chain(std::mem::take(&mut shell.window_search_documents).into_values())
        .chain(std::mem::take(&mut shell.window_measures_documents).into_values())
        .collect::<Vec<_>>();
    for auxiliary in auxiliary_documents {
        shell.retire_one_surface_document(Some(auxiliary)).expect("the refreshed auxiliary body retires");
    }
    shell.drain_retained_document_arenas();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.retired_world3d_states.is_empty() {
            break;
        }
        shell.advance_world3d_retirement_step();
    }
    assert!(shell.retired_world3d_states.is_empty(), "the refreshed owner also reaches terminal retirement");
    while !document.close_step() {}
}

/// ⚖️ LAW: two real `Component::Surface(Table)` documents publish into one Shell registry,
/// and the host's retained down/move/up ingress preserves the generation-owned transfer between them.
#[test]
fn two_published_table_surfaces_transfer_through_the_shell_host() {
    crate::scenes::cancel_scene_list_transfer();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🔀️scene-list-transfer/🔣️.json"))).expect("shared transfer fixture");
    let source_fixture = &fixture["table"]["source"];
    let destination_fixture = &fixture["table"]["destination"];
    let source_id = "table-shell-source";
    let destination_id = "table-shell-destination";
    let columns = serde_json::json!([{ "id": "name", "label": "Name", "sortable": false }]).to_string();
    let mut source_scene = ui_wgpu::wgpu::TableScene::base(
        columns.clone(),
        serde_json::json!([{
            "id": source_fixture["rowId"],
            "name": "Asset",
            "_drag": source_fixture["payload"]
        }])
        .to_string(),
    );
    source_scene.row_drag_mime = source_fixture["mime"].as_str().map(str::to_string);
    let mut destination_scene = ui_wgpu::wgpu::TableScene::base(columns, serde_json::json!([{ "id": "destination", "name": "Destination" }]).to_string());
    destination_scene.drop_action_json = Some(destination_fixture["dropAction"].to_string());

    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut source_document = shell
        .publish_surface_records(source_id, vec![table_pointer_record(1, "source-table", &source_scene)])
        .expect("source Table document publishes");
    let mut destination_document = shell
        .publish_surface_records(destination_id, vec![table_pointer_record(1, "destination-table", &destination_scene)])
        .expect("destination Table document publishes");
    let source_body = Rect::new(11.0, 17.0, 360.0, 240.0);
    let destination_body = Rect::new(411.0, 17.0, 360.0, 240.0);
    let mut input = paint_component_pointer_documents(
        &mut shell,
        &[
            (source_id, "controller.table-a", &source_document, source_body),
            (destination_id, destination_fixture["dropAction"]["controllerId"].as_str().expect("destination controller"), &destination_document, destination_body),
        ],
    );
    let source_row = input
        .hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some("table-shell-source.row.asset-7"))
        .expect("source row paints from the decoded Component::Surface")
        .rect;
    let destination_row = input
        .hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some("table-shell-destination.row.destination"))
        .expect("destination row paints from the decoded Component::Surface")
        .rect;
    let theme = Theme::default();
    let handle_size = theme.control_height_small.min(source_row.h).min(source_row.w.max(0.0));
    let source_point = (source_row.x + theme.padding_standard + handle_size * 0.5, source_row.y + source_row.h * 0.5);
    let destination_point = (destination_row.x + destination_row.w * 0.5, destination_row.y + destination_row.h * 0.5);
    assert_eq!(input.hit_at(source_point.0, source_point.1).map(|hit| hit.kind), Some(HitKind::ComponentScene), "the published semantic surface owns the physical handle point");
    assert_eq!(input.hit_at(destination_point.0, destination_point.1).map(|hit| hit.kind), Some(HitKind::ComponentScene), "the published semantic surface owns the physical drop point");

    semio_framework_async::block_on(shell.handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("Shell routes Table source down");
    shell.handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("Shell routes Table destination up");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1, "one physical transfer emits exactly one destination action");
    assert_eq!(serde_json::to_value(&actions[0]).expect("action wire"), fixture["table"]["expectedAction"]);
    assert!(crate::interpreter::retained_pointer_capture_window().is_none(), "release retires retained capture");
    assert!(!crate::scenes::cancel_scene_list_transfer(), "release retires the scene transfer authority");
    while !source_document.close_step() {}
    while !destination_document.close_step() {}
}

/// ⚖️ LAW: the physical Shell route preserves Canvas2d's full modifier chord and gives every
/// captured gesture exactly one cancelled terminal action on Escape or a hitless release.
#[test]
fn published_canvas_pointer_capture_preserves_modifiers_and_cancels_exactly_once() {
    crate::scenes::cancel_canvas_pointer_gesture(&mut InputState::default());
    let fixture = canvas_input_fixture();
    let surface = fixture["surface"]["id"].as_str().expect("surface id");
    let controller = fixture["surface"]["controllerId"].as_str().expect("controller id");
    let width = fixture["surface"]["width"].as_f64().expect("surface width") as f32;
    let height = fixture["surface"]["height"].as_f64().expect("surface height") as f32;
    let body = Rect::new(17.0, 31.0, width, height);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell
        .publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)])
        .expect("Canvas2d document publishes");
    let mut input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let down = &fixture["pointer"]["down"];
    let x = body.x + down["x"].as_f64().expect("pointer x") as f32;
    let y = body.y + down["y"].as_f64().expect("pointer y") as f32;
    input.modifiers = PointerModifiers { shift: true, ctrl: true, alt: true, meta: true };
    let theme = Theme::default();

    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Shell routes Canvas2d down");
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input);
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    let down_args = canvas_action_args(&actions[0]);
    assert_json_number_eq(&down_args["x"], &down["x"], "pointer down x");
    assert_json_number_eq(&down_args["y"], &down["y"], "pointer down y");
    for modifier in ["shift", "ctrl", "meta", "alt"] {
        assert_eq!(down_args[modifier], down["modifiers"][modifier]);
    }
    let cancelled = canvas_action_args(&actions[1]);
    assert_json_number_eq(&cancelled["x"], &fixture["pointer"]["cancel"]["x"], "cancel x");
    assert_json_number_eq(&cancelled["y"], &fixture["pointer"]["cancel"]["y"], "cancel y");
    assert_eq!(cancelled["cancelled"], true);
    for modifier in ["shift", "ctrl", "meta", "alt"] {
        assert_eq!(cancelled[modifier], false);
    }

    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("a fresh Canvas2d gesture arms");
    let outside = (body.x + body.w + 20.0, body.y + body.h + 20.0);
    semio_framework_async::block_on(shell.handle_pointer_button(outside.0, outside.1, false, 0, &mut input, &theme)).expect("captured outside release routes");
    let outside_actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(outside_actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    assert_eq!(canvas_action_args(&outside_actions[1])["cancelled"], true);
    assert!(crate::interpreter::retained_pointer_capture_window().is_none());
    semio_framework_async::block_on(shell.handle_pointer_button(outside.0, outside.1, false, 0, &mut input, &theme)).expect("duplicate release stays inert");
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a retired gesture cannot emit a second terminal action");
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("explicit cancellation gesture arms");
    shell.handle_pointer_cancel(&mut input);
    shell.handle_pointer_cancel(&mut input);
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    assert_eq!(canvas_action_args(&actions[1])["cancelled"], true);
    assert!(crate::interpreter::retained_pointer_capture_window().is_none());
    assert!(!input.pointer_down && !input.drag.active);
    while !document.close_step() {}
}

/// ⚖️ LAW: a real retained Tree drag reaches a real published Canvas2d leaf through Shell move/drop
/// ingress, preserves the raw catalogue MIME payload, and retires both drag authorities once.
#[test]
fn retained_catalogue_item_drags_and_drops_into_a_published_canvas() {
    let fixture = canvas_input_fixture();
    let catalogue = &fixture["catalogue"];
    let raw_payload = catalogue["rawPayload"].as_str().expect("raw catalogue payload");
    let source_id = "canvas-catalogue-source";
    let destination_id = fixture["surface"]["id"].as_str().expect("destination surface");
    let controller = fixture["surface"]["controllerId"].as_str().expect("destination controller");
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut source_document = published_canvas_catalogue_document(&mut shell, source_id, raw_payload);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut destination_document = shell
        .publish_surface_records(destination_id, vec![canvas_pointer_record(1, "canvas", &scene)])
        .expect("Canvas2d destination publishes");
    let source_body = Rect::new(11.0, 17.0, 320.0, 200.0);
    let destination_body = Rect::new(411.0, 17.0, 100.0, 100.0);
    let mut input = paint_component_pointer_documents(
        &mut shell,
        &[
            (source_id, "controller.catalogue", &source_document, source_body),
            (destination_id, controller, &destination_document, destination_body),
        ],
    );
    let handle = input
        .hits()
        .iter()
        .find(|hit| hit.control_id.as_deref() == Some("tree.drag.transfer.catalogue"))
        .expect("catalogue source publishes a transfer handle")
        .rect;
    let point = &catalogue["point"];
    let source_point = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    let destination_point = (
        destination_body.x + point["x"].as_f64().expect("drop x") as f32,
        destination_body.y + point["y"].as_f64().expect("drop y") as f32,
    );
    let theme = Theme::default();
    semio_framework_async::block_on(shell.handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("catalogue source down routes");
    shell.handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("Canvas2d drop routes");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasDragOver", "canvasDragLeave", "canvasDrop"]);
    let over = canvas_action_args(&actions[0]);
    assert_json_number_eq(&over["x"], &point["x"], "drag over x");
    assert_json_number_eq(&over["y"], &point["y"], "drag over y");
    assert_json_number_eq(&over["width"], &fixture["surface"]["width"], "drag over width");
    assert_json_number_eq(&over["height"], &fixture["surface"]["height"], "drag over height");
    assert_eq!(over["types"], catalogue["types"]);
    let leave = canvas_action_args(&actions[1]);
    assert_eq!(leave["surfaceId"], destination_id);
    let drop = canvas_action_args(&actions[2]);
    assert_json_number_eq(&drop["x"], &point["x"], "drop x");
    assert_json_number_eq(&drop["y"], &point["y"], "drop y");
    assert_json_number_eq(&drop["width"], &fixture["surface"]["width"], "drop width");
    assert_json_number_eq(&drop["height"], &fixture["surface"]["height"], "drop height");
    assert_eq!(drop["dragData"], raw_payload);
    assert!(crate::interpreter::active_retained_drag_sessions().is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window().is_none());
    semio_framework_async::block_on(shell.handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("duplicate drop release stays inert");
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a retired retained drag cannot drop twice");

    semio_framework_async::block_on(shell.handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("a second catalogue source down routes");
    shell.handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    shell.handle_pointer_move(destination_body.x + destination_body.w + 40.0, destination_body.y + destination_body.h + 40.0, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(destination_body.x + destination_body.w + 40.0, destination_body.y + destination_body.h + 40.0, false, 0, &mut input, &theme))
        .expect("outside catalogue release cancels cleanly");
    let cancelled = crate::collect_fixture_actions(&mut input);
    assert_eq!(cancelled.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasDragOver", "canvasDragLeave"]);
    assert!(crate::interpreter::active_retained_drag_sessions().is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window().is_none());
    while !source_document.close_step() {}
    while !destination_document.close_step() {}
}

/// ⚖️ LAW: two complete primary clicks on the published Canvas2d leaf emit one React-shaped
/// `canvasDoubleClick` at the shared fixture point, after both release phases.
#[test]
fn published_canvas_double_click_emits_once_at_surface_coordinates() {
    let fixture = canvas_input_fixture();
    let surface = "canvas-double-click-surface";
    let controller = fixture["surface"]["controllerId"].as_str().expect("controller id");
    let body = Rect::new(13.0, 29.0, 100.0, 100.0);
    let point = &fixture["doubleClick"];
    let x = body.x + point["x"].as_f64().expect("double click x") as f32;
    let y = body.y + point["y"].as_f64().expect("double click y") as f32;
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell
        .publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)])
        .expect("Canvas2d document publishes");
    let mut input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let theme = Theme::default();

    assert!(point["intervalMs"].as_f64().expect("double click interval") <= 400.0);
    for _ in 0..2 {
        semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Canvas2d click down routes");
        semio_framework_async::block_on(shell.handle_pointer_button(x, y, false, 0, &mut input, &theme)).expect("Canvas2d click up routes");
    }
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp", "canvasPointerDown", "canvasPointerUp", "canvasDoubleClick"]);
    let double_click = canvas_action_args(actions.last().expect("double click action"));
    assert_json_number_eq(&double_click["x"], &point["x"], "double click x");
    assert_json_number_eq(&double_click["y"], &point["y"], "double click y");
    assert_json_number_eq(&double_click["width"], &fixture["surface"]["width"], "double click width");
    assert_json_number_eq(&double_click["height"], &fixture["surface"]["height"], "double click height");
    while !document.close_step() {}
}

//#region 🔎️QuickSearchPaletteKeyboard
/// ⌨️ `mod+p`, as the browser delivers it.
fn palette_chord() -> (ui_wgpu::wgpu::KeyAction, PointerModifiers) {
    (ui_wgpu::wgpu::KeyAction::Char("p".into()), PointerModifiers { shift: false, ctrl: true, alt: false, meta: false })
}

/// ⚖️ LAW: the shell's own overlay query fields are CHROME, not content.
///
/// The gate every hardcoded shell chord sits behind used to be a bare `focused_id.is_some()`, and
/// the chord that opens the quick-search palette focuses the palette's own query field — so opening
/// the palette disabled the chord that closes it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_shells_own_overlay_fields_do_not_count_as_the_user_typing() {
    assert!(!ShellState::content_is_editing(None, false), "nothing focused is not editing");
    assert!(!ShellState::content_is_editing(Some("ui.search.input"), false), "the palette's own query field is chrome");
    assert!(!ShellState::content_is_editing(Some("ui.find.input"), false), "so is the find overlay's");
    assert!(ShellState::content_is_editing(Some("generation3d.height"), false), "an app content field IS the user typing");
    assert!(ShellState::content_is_editing(None, true), "and so is the sync-attach draft buffer");
}

/// ⚖️ LAW: `mod+p` TOGGLES the quick-search palette, and the palette owns every key while it is open.
///
/// Measured on 6118 before this fix: the second `mod+p` did not close the palette, it fell through
/// to the open palette's own `Char` arm and typed a literal `p` into the query — which then made
/// every later keystroke search for `p<whatever the user meant>` and `Enter` activate nothing at all.
#[test]
fn the_palette_chord_toggles_and_never_types_itself_into_the_query() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = palette_chord();

    shell.handle_keyboard(action.clone(), &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::Search, "the chord opens the palette");
    assert_eq!(input.focused_id.as_deref(), Some("ui.search.input"), "and focuses its query field");

    for key in ["D", "e"] {
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char(key.into()), &PointerModifiers::default(), &mut input);
    }
    assert_eq!(shell.search_query, "De", "plain keys type into the palette's query");

    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::None, "the same chord closes it");
    assert!(!shell.search_open);
    assert_eq!(input.focused_id, None, "and hands focus back to the canvas");
    assert_eq!(shell.search_query, "De", "unactivated dismissal preserves React's owned query without appending a literal `p`");
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char("x".into()), &PointerModifiers::default(), &mut input);
    assert_eq!(shell.search_query, "De", "closed palette state does not consume ordinary typing");
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char("p".into()), &PointerModifiers::default(), &mut input);
    assert_eq!(shell.search_query, "De", "the chord's character never leaks into a closed retained query");
    println!("[DEBUG] wgpu-shell palette chord: open -> typed \"De\" -> closed, query preserved, focus released");
}

/// ⚖️ LAW: Escape closes the palette. It used to be claimed by the focused-input commit first, which
/// left the palette with no keyboard route out at all once the toggle was broken too.
#[test]
fn escape_closes_the_palette_rather_than_committing_its_query_field() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = palette_chord();
    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::Search);

    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input)).expect("escape routes");
    assert_eq!(shell.overlay_state, OverlayState::None, "escape closes the topmost overlay");
    assert_eq!(input.focused_id, None);
}
//#endregion 🔎️QuickSearchPaletteKeyboard
