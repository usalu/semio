/// 🪟️ Window publication journals: published tree and canvas documents, scripted window body fixtures, transfer journal refusals and the standalone multi-app variants.

fn published_tree_pointer_document(shell: &mut ShellState, surface: &str) -> (UiDocumentLease, String) {
    let mut item = UiTreeItemNode::base("transfer", Label::data("Transfer"));
    item.action = Some(ActionDescriptor { controller_id: "s.test.tree".into(), action: "selectTreeItem".into(), args: None });
    item.draggable = Some(true);
    item.drag_data = Some(HashMap::from([("application/x-semio-window-template".into(), "{\"windowKindId\":\"world\",\"templateId\":\"default\"}".into())]));
    let node = UiNode::Tree(UiTreeNode {
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, id: "section".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item], window: None }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    });
    let records = panel_ui_records(surface, &node).expect("the authored Tree projects through the production panel assembler");
    let item_key = records.iter().find(|record| matches!(&record.component, ui_contract::Component::TreeItem(_))).map(|record| record.key.as_str().to_string()).expect("the projected Tree carries its transfer row");
    (shell.windows.publish_surface_records(surface, records).expect("tree pointer document publishes"), item_key)
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
    shell.windows.publish_surface_records(surface, records).expect("catalogue pointer document publishes")
}

fn paint_tree_pointer_document(shell: &mut ShellState, surface: &str, document: &UiDocumentLease, body: Rect) -> InputState<ActionDescriptor> {
    paint_tree_pointer_documents(shell, &[(surface, document, body)])
}

fn table_pointer_record(id: u64, key: &str, scene: &semio_framework_ui_scene_views::TableScene) -> ui_contract::UiNodeRecord {
    let surface = semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::Table, scene).expect("bounded Table scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn canvas_pointer_record(id: u64, key: &str, scene: &semio_framework_ui_scene_plane::Canvas2dScene) -> ui_contract::UiNodeRecord {
    let surface = semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::Canvas2d, scene).expect("bounded Canvas2d scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn runnable_window_body_fixture(_instance_id: u32, surface_id: &str, body_key: &str, _view_state: &ViewModel, _document_dsl: Option<&str>, _refresh_effects: Option<&mut Vec<semio_framework::kernel::Effect>>) -> Result<UiDocumentLease, String> {
    for section in [semio_framework::UiRefreshSection::Catalogue, semio_framework::UiRefreshSection::Engagements, semio_framework::UiRefreshSection::Measures, semio_framework::UiRefreshSection::Tools] {
        if body_key == section.body_key() {
            return Ok(semio_framework_os_renderer_wgpu_shell_windows::section_fixtures::section_document(&serde_json::json!({}), 1, section));
        }
    }
    let scene = semio_framework_ui_scene_world3d::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    let records = vec![world3d_pointer_record(1, "initial-world", &scene)];
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation: 1, revision: ui_contract::UiRevision(1), root: Some(ui_contract::UiNodeId(1)), layout_epoch: 0 };
    UiDocumentLease::try_publish(SurfaceId::try_from(surface_id).map_err(|_| "fixture surface exceeds the retained contract")?, identity, records).map_err(|error| semio_framework_os_renderer_wgpu_program_bridge::retained_publication_refusal(surface_id, error))
}

std::thread_local! {
    static WINDOW_JOURNAL_FIXTURE_REFUSALS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_BODY_FIXTURE_REFUSALS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_BODY_FIXTURE_ATTEMPTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_PUBLICATION_FIXTURE_EVENTS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn scripted_window_body_fixture(instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<semio_framework::kernel::Effect>>) -> Result<UiDocumentLease, String> {
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

fn refuse_window_body_fixture_action(_instance_id: u32, action_json: &str, _view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
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
    let mut shell = host_test_shell();
    shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::instance("main-2", "main", WindowStackCorner::TopLeft)], active: "main-2".into() };
    shell.dock.dock.active_window_id = Some("main-2".into());
    shell.windows.active_window_id = Some("main-2".into());
    shell.dock.persist_dock_layout();
    let program = shell.plugins.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the host fixture has its guest program");
    program.install_fixture_render(scripted_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_BODY_FIXTURE_REFUSALS.with(|remaining| remaining.set(refusals));
    WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(0));
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().clear());
    let controller_id = shell.docs.shell_command_controller_id(&shell.plugins).expect("the fixture session journals transfers");
    let note = note_shell_command_action(&controller_id, "shell.windowSplit", "Split Window", Some(serde_json::json!({ "windowKindId": "main", "instanceId": "main-2" })));
    let token = shell.windows.reserve_window_topology_action(note).expect("the fixture transfer journal is admitted");
    shell.windows.commit_window_topology_publication("main-2", "main.body", Some(token), &mut shell.settle);
    shell
}

fn retire_scripted_window_publication_documents(shell: &mut ShellState) {
    shell.docs.retire_documents_outside(&[], true, &mut shell.chrome).expect("scripted window documents retire");
    shell.docs.retire_documents_outside(&[], false, &mut shell.chrome).expect("scripted panel documents retire");
    let auxiliary =
        std::mem::take(&mut shell.docs.window_actions_documents).into_values().chain(std::mem::take(&mut shell.docs.window_search_documents).into_values()).chain(std::mem::take(&mut shell.docs.window_measures_documents).into_values()).collect::<Vec<_>>();
    for document in auxiliary {
        shell.docs.retire_one_surface_document(Some(document)).expect("scripted auxiliary document retires");
    }
    shell.docs.drain_retained_document_arenas();
}

#[test]
fn required_window_body_refusal_holds_the_transfer_journal_then_recovers_once() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json")).expect("window publication fixture");
    let recovery = fixture["publicationOutcomes"]["recovery"].as_array().expect("recovery sequence");
    let mut shell = shell_with_scripted_window_publication(1);

    assert_eq!(semio_framework_async::block_on(PointerCtx::of(&mut shell).settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), 1);
    assert!(!shell.docs.window_ui.contains_key("main-2"));
    assert!(shell.windows.window_topology_refresh_owed);
    assert_eq!(shell.windows.window_topology_actions.len(), 1);
    assert!(shell.settle.deferred_actions.is_empty());
    assert_eq!(shell.plugins.surface_faults.iter().any(|fault| fault.surface_id == "main-2"), recovery[0]["surfaceFault"].as_bool().unwrap());

    assert_eq!(semio_framework_async::block_on(PointerCtx::of(&mut shell).settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), 2);
    assert!(shell.docs.window_ui.contains_key("main-2"));
    assert_eq!(shell.windows.window_topology_refresh_owed, recovery[1]["topologyOwed"].as_bool().unwrap());
    assert!(shell.windows.window_topology_actions.is_empty());
    assert!(shell.windows.window_topology_publications.is_empty());
    assert_eq!(shell.settle.deferred_actions.iter().filter(|action| action.action == "noteShellCommand").count(), 1);
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0);

    assert_eq!(semio_framework_async::block_on(PointerCtx::of(&mut shell).settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 1, "recovery releases exactly one journal on the following settle step");
    assert!(shell.settle.deferred_actions.is_empty());
    retire_scripted_window_publication_documents(&mut shell);
}

#[test]
fn permanent_window_body_refusal_retires_the_journal_at_the_fixture_ceiling() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json")).expect("window publication fixture");
    let outcomes = &fixture["publicationOutcomes"];
    let retry_ceiling = outcomes["retryCeiling"].as_u64().expect("retry ceiling") as usize;
    assert_eq!(retry_ceiling, usize::from(WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS));
    let mut shell = shell_with_scripted_window_publication(retry_ceiling);

    for attempt in 0..retry_ceiling {
        assert_eq!(semio_framework_async::block_on(PointerCtx::of(&mut shell).settle_pump_step_inner()), ShellSettleStep::Drained);
        if attempt + 1 < retry_ceiling {
            assert!(shell.windows.window_topology_refresh_owed);
            assert_eq!(shell.windows.window_topology_actions.len(), 1);
        }
    }
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), retry_ceiling);
    assert!(!shell.windows.window_topology_refresh_owed);
    assert!(shell.windows.window_topology_actions.is_empty());
    assert!(shell.windows.window_topology_publications.is_empty());
    assert!(shell.settle.deferred_actions.is_empty());
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0, "a permanently refused body never emits a success-shaped journal");
    let fault = shell.plugins.surface_faults.iter().find(|fault| fault.surface_id == "main-2").expect("terminal publication fault");
    assert!(fault.detail.contains(&format!("after {retry_ceiling} attempts")));
    assert_eq!(outcomes["permanentFault"]["journal"], "terminal-refusal");
    assert_eq!(outcomes["permanentFault"]["surfaceFault"], true);
    retire_scripted_window_publication_documents(&mut shell);
}

#[test]
fn topology_journal_credit_refusal_is_returned_to_the_transfer_caller() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut shell = host_test_shell();
    for _ in 0..semio_framework_ui_action::ACTION_QUEUE_ITEM_CAPACITY {
        shell.windows.reserve_window_topology_action(ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: None }).expect("the occupied fixture journal publishes");
    }
    let error = shell.windows.reserve_window_topology_action(ActionDescriptor { controller_id: "test".into(), action: "noteShellCommand".into(), args: None }).expect_err("a full action queue returns its exact refusal");
    assert_eq!(error, semio_framework_ui_action::BoundedActionFault::ItemCredits);
    assert!(shell.windows.window_topology_publications.is_empty(), "refusal precedes every topology owner");
    assert!(shell.settle.deferred_actions.is_empty());
}

fn canvas_input_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../📐️Canvas2dHost/🧪️tests/🧫️fixtures/🖱️input-contract/🔣️.json")).expect("shared Canvas2d input fixture")
}

fn assert_json_number_eq(actual: &Value, expected: &Value, field: &str) {
    assert_eq!(actual.as_f64().unwrap_or_else(|| panic!("{field} actual number")), expected.as_f64().unwrap_or_else(|| panic!("{field} expected number")), "{field}");
}

fn canvas_action_args(action: &ActionDescriptor) -> Value {
    dsl_value_as_json(action.args.as_ref().expect("Canvas2d action args"))
}
#[test]
fn standalone_multi_app_variants_resolve_their_declared_app() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let corpus: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🔬️wgpu-shell-boot-selection/🔣️.json")).unwrap();
    for row in corpus["catalog"]["playgrounds"].as_array().unwrap() {
        let plugin = row["pluginId"].as_str().unwrap();
        let app = row["app"].as_str().unwrap();
        let manifest = boot_manifest(plugin, &[app]);
        let request = BootSelectionRequest { plugin_id: plugin, app_id: Some(app) };
        let (_, selected) = select_boot_program(&[(plugin, &manifest)], request, semio_framework::AppRole::Editor, None).expect("an injected standalone declaration resolves");
        assert_eq!(selected.id, app);
    }
}
