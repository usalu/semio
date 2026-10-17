/// 🧪️ Scene fixtures and focus rebase builders: the component scene leaf, the identity and clipboard input documents, the seeded scene windows, the focus rebase document builders and the ink intent builders.

//#region 🔖️SceneCommandTests
/// 🎬️ A minimal `ComponentScene` leaf — every optional per-`SurfaceKind` payload left `None`,
/// matching `ui_wgpu::wgpu::events::tests::component_scene_ui`'s own fixture shape (that one is private
/// to the sibling `ui_wgpu` crate, so this is a separate copy for this crate's own tests).
fn component_scene_ui(surface_id: &str, kind: semio_framework_ui_declaration::ui::SurfaceKind) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "ctrl".into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    })
}

/// 🌱️ `apply_tree`s a single-child `Stack(scene_node)` into `window_id` and returns the scene
/// leaf's own `NodeId` — every scene-command test below needs a real, live tree node since
/// `apply_scene_ui_command` re-fetches it by `(window_id, node)` from `UI_ENGINE`.
fn scene_surface_props(scene: &UiComponentSceneNode) -> ui_contract::SurfaceProps {
    let kind = match scene.component_kind {
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d => ui_contract::SurfaceKind::Canvas2d,
        semio_framework_ui_declaration::ui::SurfaceKind::World3d => ui_contract::SurfaceKind::World3d,
        semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph => ui_contract::SurfaceKind::NodeGraph,
        semio_framework_ui_declaration::ui::SurfaceKind::TextEditor => ui_contract::SurfaceKind::TextEditor,
        semio_framework_ui_declaration::ui::SurfaceKind::Table => ui_contract::SurfaceKind::Table,
        semio_framework_ui_declaration::ui::SurfaceKind::Paint2d => ui_contract::SurfaceKind::Paint2d,
        semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem => ui_contract::SurfaceKind::VirtualFileSystem,
        semio_framework_ui_declaration::ui::SurfaceKind::TiledMap => ui_contract::SurfaceKind::TiledMap,
        semio_framework_ui_declaration::ui::SurfaceKind::Board2d => ui_contract::SurfaceKind::Board2d,
        semio_framework_ui_declaration::ui::SurfaceKind::IconRender => ui_contract::SurfaceKind::IconRender,
        semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas => ui_contract::SurfaceKind::InkCanvas,
        semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline => ui_contract::SurfaceKind::GraphTimeline,
        semio_framework_ui_declaration::ui::SurfaceKind::BlockList => ui_contract::SurfaceKind::BlockList,
        semio_framework_ui_declaration::ui::SurfaceKind::DiffView => ui_contract::SurfaceKind::DiffView,
        semio_framework_ui_declaration::ui::SurfaceKind::EventFeed => ui_contract::SurfaceKind::EventFeed,
    };
    let encoded = match scene.component_kind {
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d => scene.canvas_2d.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::World3d => scene.world_3d.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph => scene.node_graph.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::TextEditor => scene.text_editor.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::Table => scene.table.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::Paint2d => scene.paint_2d.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem => scene.virtual_file_system.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::TiledMap => scene.tiled_map.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::Board2d => scene.board2d.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::IconRender => scene.icon_render.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas => scene.ink_canvas.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline => scene.graph_timeline.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::BlockList => scene.block_list.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::DiffView => scene.diff_view.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::EventFeed => scene.event_feed.as_ref().map(|value| semio_framework_ui_scene_surface::encode(kind, value)),
    };
    encoded.transpose().expect("the bounded scene fixture encodes").unwrap_or_else(|| ui_contract::SurfaceProps {
        kind,
        doc_schema: ui_contract::UiText::try_from_str(&format!("{}@1", scene.component_kind.as_str())).expect("bounded fixture schema"),
        doc: Default::default(),
        bindings: Default::default(),
    })
}

fn scene_identity_document(window_id: &str, scene: &UiComponentSceneNode) -> semio_framework_ui_retained_tree::tree::UiDocumentTree {
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from(window_id).unwrap(), revision: ui_contract::UiRevision(1), root: ui_contract::UiNodeId(0), layout_epoch: 1, node_count: 2 };
    let mut document = semio_framework_ui_retained_tree::tree::UiDocumentTree::new(header).unwrap();
    let mut children = ui_contract::UiFixedList::default();
    children.try_push(ui_contract::UiNodeId(1)).unwrap();
    let root = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(0),
        key: ui_contract::UiText::try_from_str("seed/root").unwrap(),
        component: ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        layout: ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() }),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children,
    };
    let scene = ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(1),
        key: ui_contract::UiText::try_from_str("seed/scene").unwrap(),
        component: ui_contract::Component::Surface(scene_surface_props(scene)),
        layout: ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() }),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    };
    document.try_upsert_record(root).unwrap();
    document.try_upsert_record(scene).unwrap();
    document
}

fn clipboard_input_document(window_id: &str) -> semio_framework_ui_retained_tree::tree::UiDocumentTree {
    let records = serde_json::json!([
        {
            "id": 0,
            "key": "clipboard/root",
            "component": { "type": "container" },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": true, "wrap": false },
            "style": {}, "activity": "idle", "accessibility": {}, "children": [1]
        },
        {
            "id": 1,
            "key": "clipboard/name",
            "component": { "type": "input", "kind": "text", "value": "" },
            "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
            "style": {}, "activity": "idle", "accessibility": { "label": "Clipboard input" },
            "bindings": [{ "trigger": "change", "action": { "scope": "clipboard", "name": "onChange", "version": 1 } }]
        }
    ]);
    let rows = records.as_array().unwrap();
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from(window_id).unwrap(), revision: ui_contract::UiRevision(1), root: ui_contract::UiNodeId(0), layout_epoch: 1, node_count: rows.len() };
    let mut document = semio_framework_ui_retained_tree::tree::UiDocumentTree::new(header).unwrap();
    for row in rows {
        document.try_upsert_record(serde_json::from_value(row.clone()).unwrap()).unwrap();
    }
    document
}

fn publish_presented_document(window_id: &str, generation: u64, witness: u64, controller_id: &str, document: semio_framework_ui_retained_tree::tree::UiDocumentTree) {
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, document)));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    let complete = (0..4096).any(|_| {
        let mut step_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut step = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(64, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut step_retained);
        match UI_ENGINE.with(|cell| cell.borrow_mut().step_document_reconcile(window_id, controller_id, &mut step)) {
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Pending => false,
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Complete => true,
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Fault(fault) => panic!("presented document reconcile fault: {fault:?}"),
        }
    });
    assert!(complete, "presented fixture reconcile reaches terminal within its fixed opportunity ceiling");
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    assert!(acknowledge_presented_input(witness));
}

fn drive_scene_seed_reconcile(window_id: &str, controller_id: &str) {
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    for _ in 0..4096 {
        let mut step_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut step = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(64, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut step_retained);
        match UI_ENGINE.with(|cell| cell.borrow_mut().step_document_reconcile(window_id, controller_id, &mut step)) {
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Pending => {}
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Complete => return,
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Fault(fault) => panic!("scene seed reconcile fault: {fault:?}"),
        }
    }
    panic!("scene seed reconcile exceeded its fixed opportunity ceiling");
}

fn seed_scene_window_with(window_id: &str, scene_node: UiNode) -> NodeId {
    let UiNode::ComponentScene(scene) = scene_node else { panic!("the seed scene fixture requires ComponentScene") };
    let controller_id = scene.controller_id.clone();
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, scene_identity_document(window_id, &scene))));
    drive_scene_seed_reconcile(window_id, &controller_id);
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let tree = engine.tree(window_id).expect("the retained scene window exists");
        let node = tree.children(tree.root.expect("the scene document has a root")).next().expect("the scene document has a child");
        assert!(tree.node(node).is_some_and(|retained| matches!(retained.spec.0, UiNode::ComponentScene(_))));
        node
    })
}

/// 🎞️ The presenter witness a seeded scene window's first presentation is accepted under.
const SEEDED_SCENE_PRESENTATION_WITNESS: u64 = 0x5EED_5CE7;

/// 🎞️ Presents a seeded window exactly as a frame presents it (seal + accepted pixels): since 09-27 every input, caret and
/// accessibility address resolves in the PRESENTED tree, so a law driving one needs the window presented.
fn present_seeded_scene_window(window_id: &str) {
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(SEEDED_SCENE_PRESENTATION_WITNESS), "the seeded scene window seals its presentation");
    assert!(acknowledge_presented_input(SEEDED_SCENE_PRESENTATION_WITNESS), "and its pixels are accepted");
}

/// 🪪️ The ComponentScene a seeded window mounted. Its host id is engine-minted (`reconcile::component_scene_host_id`, 09-21),
/// so every scene-scoped key, staged control and focus address reads it from here, never from the authored fixture scene.
fn retained_scene(window_id: &str, node: NodeId) -> UiComponentSceneNode {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window_id).and_then(|tree| tree.node(node)).expect("the retained scene node remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("the retained node is a ComponentScene") };
        scene.clone()
    })
}

/// 🔑️ A fixture key authored under the fixture's host id, rebased onto the engine-minted host id its retained scene carries.
fn rebased_scene_key(key: &str, authored_host: &str, retained_host: &str) -> String {
    format!("{retained_host}{}", key.strip_prefix(authored_host).expect("the fixture key is scoped by its authored host"))
}

/// ♿️ Stages, seals and accepts the editable-text cells a seeded Table window's paint publishes (09-27: only accepted cells
/// reach the accessibility tree).
fn accept_table_editable_text_cells(window_id: &str, node: NodeId, epoch: u64) {
    let retained = retained_scene(window_id, node);
    semio_framework_os_renderer_wgpu_scene_state::remember_scene_theme(&Theme::default());
    let cells = semio_framework_os_renderer_wgpu_scene_tables::table_editable_text_accessibility_cells(&retained, Rect::new(0.0, 0.0, 200.0, 200.0), ui_wgpu::wgpu::UiDriverDrag::Handle);
    assert!(!cells.is_empty(), "the seeded Table paints its editable cells");
    semio_framework_os_renderer_wgpu_scene_feeds::stage_table_editable_text_accessibility_cells(&retained.host_id, cells);
    semio_framework_os_renderer_wgpu_scene_feeds::seal_table_editable_text_accessibility_candidates(epoch);
    semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_table_editable_text_accessibility_candidates(epoch);
}

fn seed_scene_window(window_id: &str, surface_id: &str, kind: semio_framework_ui_declaration::ui::SurfaceKind) -> NodeId {
    seed_scene_window_with(window_id, component_scene_ui(surface_id, kind))
}

fn retained_scene_host_id(window_id: &str, node: NodeId) -> String {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window_id).and_then(|tree| tree.node(node)).expect("the retained scene node remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("the retained node is a ComponentScene") };
        scene.host_id.clone()
    })
}

fn retained_scene_surface_id(window_id: &str, node: NodeId) -> String {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let retained = engine.tree(window_id).and_then(|tree| tree.node(node)).expect("the retained scene node remains mounted");
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { panic!("the retained node is a ComponentScene") };
        scene.surface_id.clone()
    })
}

fn focus_rebase_document_with_ink(window_id: &str, generation: u64, kind: semio_framework_ui_declaration::ui::SurfaceKind, children: &[u64], ink: Option<&semio_framework_ui_scene_plane::InkCanvasScene>) -> semio_framework_ui_retained_tree::tree::UiDocumentTree {
    let mut root: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "focus-rebase/root",
        "component": { "type": "container" },
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "style": {}, "activity": "idle", "accessibility": {}, "children": children
    }))
    .unwrap();
    root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
    let mut document = semio_framework_ui_retained_tree::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation,
        surface: ui_contract::SurfaceId::try_from(window_id).unwrap(),
        revision: ui_contract::UiRevision(generation),
        root: ui_contract::UiNodeId(1),
        layout_epoch: generation,
        node_count: children.len() + 1,
    })
    .unwrap();
    document.try_upsert_record(root).unwrap();
    for id in children {
        let mut record: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
            "id": id,
            "key": format!("focus-rebase/{id}"),
            "component": { "type": "container" },
            "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
            "style": {}, "activity": "idle", "accessibility": {}, "children": []
        }))
        .unwrap();
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
        if *id == 4 {
            let surface = match kind {
                semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d => semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::Canvas2d, &semio_framework_ui_scene_plane::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into())).unwrap(),
                semio_framework_ui_declaration::ui::SurfaceKind::TextEditor => semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::TextEditor, &semio_framework_ui_scene_views::TextEditorScene::base("focus".into(), None, None)).unwrap(),
                semio_framework_ui_declaration::ui::SurfaceKind::Table => semio_framework_ui_scene_surface::encode(
                    ui_contract::SurfaceKind::Table,
                    &semio_framework_ui_scene_views::TableScene::base(
                        serde_json::json!([{ "id": "count", "label": "Count" }]).to_string(),
                        serde_json::json!([{ "id": "r1", "count": { "kind": "stepper", "value": 2.0, "min": 0.0, "max": 5.0, "step": 0.5, "action": { "controllerId": "focus-rebase", "action": "setCount", "args": { "objectId": "r1" } } } }])
                            .to_string(),
                    ),
                )
                .unwrap(),
                semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas => {
                    semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::InkCanvas, &ink.cloned().unwrap_or_else(|| semio_framework_ui_scene_plane::InkCanvasScene::base("[]".into(), "pen".into(), "document".into(), true))).unwrap()
                }
                _ => panic!("focus rebase fixture supports Canvas, Table and editor surfaces"),
            };
            record.component = ui_contract::Component::Surface(surface);
        }
        document.try_upsert_record(record).unwrap();
    }
    document
}

fn focus_rebase_document(window_id: &str, generation: u64, kind: semio_framework_ui_declaration::ui::SurfaceKind, children: &[u64]) -> semio_framework_ui_retained_tree::tree::UiDocumentTree {
    focus_rebase_document_with_ink(window_id, generation, kind, children, None)
}

fn drive_focus_rebase_reconcile(window_id: &str, generation: u64) {
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    for _ in 0..4096 {
        let mut step_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut step = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(64, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut step_retained);
        match UI_ENGINE.with(|cell| cell.borrow_mut().step_document_reconcile(window_id, "focus-rebase", &mut step)) {
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Pending => {}
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Complete => return,
            semio_framework_ui_retained_tree::reconcile::UiDocumentReconcileStep::Fault(fault) => panic!("focus rebase reconcile fault: {fault:?}"),
        }
    }
    panic!("focus rebase reconcile exceeded its fixed opportunity ceiling");
}

fn focus_rebase_node(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Option<NodeId> {
    tree.children(tree.root?).find(|node| tree.node(*node).is_some_and(|node| node.key == semio_framework_ui_retained_tree::tree::NodeKey::Explicit("focus-rebase/4".into())))
}

fn publish_new_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: semio_framework_ui_declaration::ui::SurfaceKind, children: &[u64]) -> Option<NodeId> {
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    assert!(acknowledge_presented_input(witness));
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(focus_rebase_node))
}

fn publish_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: semio_framework_ui_declaration::ui::SurfaceKind, children: &[u64]) -> Option<NodeId> {
    if generation > 1 {
        drive_focus_rebase_reconcile(window_id, generation - 1);
    }
    publish_new_focus_rebase_document(window_id, generation, witness, kind, children)
}

fn reconcile_focus_rebase_document(window_id: &str, generation: u64, kind: semio_framework_ui_declaration::ui::SurfaceKind, children: &[u64]) {
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
}

fn stage_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: semio_framework_ui_declaration::ui::SurfaceKind, children: &[u64]) -> NodeId {
    drive_focus_rebase_reconcile(window_id, generation - 1);
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let presented = engine.tree(window_id).and_then(focus_rebase_node).expect("presented editor node");
        let candidate = engine.candidate_tree(window_id).and_then(focus_rebase_node).expect("candidate editor node");
        assert_eq!(engine.candidate_scene_node_for_presented_node(window_id, witness, presented), Some(candidate));
        candidate
    })
}

fn stage_new_focus_rebase_document(window_id: &str, generation: u64, witness: u64, kind: semio_framework_ui_declaration::ui::SurfaceKind, children: &[u64]) -> NodeId {
    drive_focus_rebase_reconcile(window_id, generation - 1);
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document(window_id, generation, kind, children))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    UI_ENGINE.with(|cell| cell.borrow().candidate_tree(window_id).and_then(focus_rebase_node).expect("candidate editor node"))
}

fn ink_intent_scene(law: &Value, utility: &str, document_id: &str) -> semio_framework_ui_scene_plane::InkCanvasScene {
    let mut document = law["document"].clone();
    document["id"] = Value::String(document_id.into());
    document["activeUtility"] = Value::String(utility.into());
    let mut scene = semio_framework_ui_scene_plane::InkCanvasScene::base(serde_json::to_string(&document).unwrap(), utility.into(), "edit".into(), true);
    scene.selection_json = law["scene"]["inkCanvas"]["selectionJson"].as_str().unwrap().into();
    scene
}

fn publish_ink_intent_document(window_id: &str, generation: u64, witness: u64, children: &[u64], ink: &semio_framework_ui_scene_plane::InkCanvasScene) -> NodeId {
    publish_presented_document(window_id, generation, witness, "focus-rebase", focus_rebase_document_with_ink(window_id, generation, semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas, children, Some(ink)));
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(focus_rebase_node).expect("presented Ink intent receiver"))
}

fn stage_ink_intent_document(window_id: &str, generation: u64, witness: u64, children: &[u64], ink: &semio_framework_ui_scene_plane::InkCanvasScene) -> NodeId {
    drive_focus_rebase_reconcile(window_id, generation - 1);
    assert!(UI_ENGINE.with(|cell| cell.borrow_mut().publish_document(window_id, focus_rebase_document_with_ink(window_id, generation, semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas, children, Some(ink)))));
    drive_focus_rebase_reconcile(window_id, generation);
    begin_accessibility_visible_documents();
    note_accessibility_visible_document(window_id);
    assert!(seal_presented_input_candidate(witness));
    UI_ENGINE.with(|cell| cell.borrow().candidate_tree(window_id).and_then(focus_rebase_node).expect("candidate Ink intent receiver"))
}

fn begin_pending_ink_intent(window_id: &str, node: NodeId, pointer: ui_render::PointerId, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    apply_scene_ui_command(
        window_id,
        node,
        semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas,
        Rect::new(0.0, 0.0, 320.0, 240.0),
        &ui_wgpu::wgpu::UiEvent::PointerDown { x: 220.0, y: 190.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        Some(pointer),
        input,
    );
    assert!(drive_scene_interaction_step(input));
    assert!(SCENE_INTENTS.with(|cell| cell.borrow().slots.iter().flatten().any(|intent| intent.pointer_id == Some(pointer) && intent.ink_job.is_some() && !intent.retiring)));
}
//#endregion 🔖️SceneCommandTests
