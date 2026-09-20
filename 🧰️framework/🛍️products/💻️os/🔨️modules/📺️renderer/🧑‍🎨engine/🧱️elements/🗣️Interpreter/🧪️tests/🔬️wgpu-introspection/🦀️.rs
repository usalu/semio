
use super::*;
use ui_wgpu::wgpu::{Label, LayoutBucket, Node, NodeFlags, NodeKey, Theme, UiPresence, UiStackNode, UiTextNode, WidgetSpec};

fn text_node(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn stack_node(id: Option<&str>, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: id.map(String::from), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

#[test]
fn path_segments_use_kind_index_and_declared_id() {
    let root = stack_node(Some("root"), vec![text_node("a"), stack_node(None, vec![])]);
    assert_eq!(ui_node_path_segment(&root, 0), "stack[0]#root");
    let UiNode::Stack(stack) = &root else { unreachable!() };
    assert_eq!(ui_node_path_segment(&stack.children[0], 0), "text[0]");
    assert_eq!(ui_node_path_segment(&stack.children[1], 1), "stack[1]");
}

#[test]
fn walk_dump_accumulates_absolute_rects_and_builds_full_paths() {
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let child_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Positional(1, 0), WidgetSpec(text_node("hi"))));
    tree.node_mut(root_id).unwrap().layout = LayoutBucket { x: 10.0, y: 20.0, width: 200.0, height: 100.0, ..Default::default() };
    tree.node_mut(child_id).unwrap().layout = LayoutBucket { x: 5.0, y: 6.0, width: 50.0, height: 12.0, ..Default::default() };

    let theme = Theme::default();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    walk_dump(&tree, root_id, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].path, "stack[0]#root");
    assert_eq!(nodes[0].rect, [10.0, 20.0, 200.0, 100.0]);
    assert_eq!(nodes[1].path, "stack[0]#root/text[0]");
    assert_eq!(nodes[1].rect, [15.0, 26.0, 50.0, 12.0], "child rect must be the root's absolute origin plus its own parent-relative offset");
}

#[test]
fn focus_path_is_recorded_for_the_focused_node() {
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let child_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Positional(1, 0), WidgetSpec(text_node("hi"))));
    tree.node_mut(child_id).unwrap().flags.set(NodeFlags::FOCUSED, true);

    let theme = Theme::default();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    walk_dump(&tree, root_id, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);

    assert_eq!(focus_path, Some("stack[0]#root/text[0]".to_string()));
}

#[test]
fn the_accessibility_dump_announces_visible_windows_and_keeps_named_diagnostics() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/♿️wgpu-accessibility-visibility/🔣️.json")).expect("visibility fixture parses");
    let mut engine = ui_wgpu::wgpu::Ui::new();
    for window_id in law["liveWindows"].as_array().expect("live windows") {
        engine.set_viewport(window_id.as_str().expect("window id"), 400.0, 300.0);
    }
    begin_accessibility_visible_documents();
    for window_id in law["visibleWindows"].as_array().expect("visible windows") {
        note_accessibility_visible_document(window_id.as_str().expect("visible window id"));
    }
    publish_accessibility_visible_documents();
    note_chrome_accessibility(vec![serde_json::from_value(serde_json::json!({ "nodeId": 1, "key": "shell", "role": "button", "depth": 0, "live": "off" })).expect("chrome accessibility row")]);

    let all = build_accessibility_dump(&engine, None);
    let announced: Vec<_> = all.windows.iter().map(|window| window.window_id.as_str()).collect();
    assert_eq!(announced, law["unnamedPublished"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>(), "an unnamed dump follows the last complete visible-document publication");
    assert_eq!(all.window_id, None, "no window was named, so none is echoed");

    let requested = law["requestedDiagnostic"]["requested"].as_str().unwrap();
    let named = build_accessibility_dump(&engine, Some(requested));
    assert_eq!(named.windows.iter().map(|window| window.window_id.as_str()).collect::<Vec<_>>(), law["requestedDiagnostic"]["published"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>(), "a named diagnostic remains available for an inactive retained document");
    assert_eq!(named.window_id.as_deref(), Some(requested));
    assert_eq!(named.window_ids.len(), 3, "and still names every window a caller could ask for instead");
    assert!(!accessibility_window_is_visible(law["hiddenEvent"]["windowId"].as_str().unwrap()), "the same publication rejects events for an inactive document");

    let absent = build_accessibility_dump(&engine, Some("never-mounted"));
    assert!(absent.windows.is_empty(), "a window that is not live announces nothing, so a reader can tell it from an empty one");
    begin_accessibility_visible_documents();
    publish_accessibility_visible_documents();
    note_chrome_accessibility(Vec::new());
}

#[test]
fn kind_tags_match_the_ui_node_wire_format_tag() {
    // 🔒️ Guards path-grammar drift against `UiNode`'s own `#[serde(tag = "type")]` wire format.
    let node = text_node("x");
    let json = serde_json::to_value(&node).unwrap();
    assert_eq!(json.get("type").and_then(|v| v.as_str()), Some(ui_node_kind_tag(&node)));
}
/// 🧊️ `dumpMeshStats` answers the PRODUCER's publication — one row per `meshes_json` entry, its own
/// role stamp, its array lengths and its bounds — and never the renderer's mesh store, which is what
/// made a live census incomparable with a committed `delivery.meshes`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn mesh_stats_publish_one_row_per_published_mesh_with_its_role_and_bounds() {
    let meshes_json = serde_json::json!([
        {"id": "eval-extrude@solid#0", "role": "solid", "data": {"positions": [0.0, 0.0, 0.0, 2.0, 2.0, 3.0], "indices": [0, 1, 2], "edgePositions": []}},
        {"id": "eval-profile@wire#0", "role": "wire", "data": {"positions": [], "indices": [], "edgePositions": [-1.0, 0.0, 0.0, 1.0, 0.5, 0.0]}},
        {"id": "eval-axis@vector#0", "data": {"positions": [0.0, 0.0, 0.0], "indices": [], "edgePositions": []}}
    ])
    .to_string();
    let instances_json = serde_json::json!([{"id": "extrude@solid#0", "meshId": "eval-extrude@solid#0", "interactionId": "extrude@solid"}]).to_string();
    let selection_json = serde_json::json!({"ids": ["extrude@solid#0"], "hoveredId": "extrude@solid#0"}).to_string();
    let node = ui_wgpu::wgpu::build_world_3d_scene("procedural-preview", "controller", ui_wgpu::wgpu::World3dScene::base("{\"position\": [4.0, 4.0, 4.0]}".into(), meshes_json, instances_json, selection_json));
    let UiNode::ComponentScene(scene) = &node else { unreachable!("build_world_3d_scene answers a component scene") };

    let surface = mesh_stats_for_scene(scene, [10.0, 20.0, 400.0, 300.0]).expect("a world3d scene publishes mesh stats");
    assert_eq!(surface.surface_id, "procedural-preview");
    assert_eq!(surface.rect, [10.0, 20.0, 400.0, 300.0], "the rect is the page rect a pointer probe aims with");
    assert_eq!(surface.meshes.len(), 3, "one row per published mesh, companions included");
    assert_eq!(surface.roles, [("solid".to_string(), 1), ("wire".to_string(), 1), ("(unstamped)".to_string(), 1)].into_iter().collect(), "an unstamped producer is reported as such rather than defaulted into a role");
    assert_eq!(surface.meshes[0].indices, 3);
    assert_eq!(surface.meshes[0].positions, 6);
    assert_eq!(surface.meshes[1].edge_positions, 6);
    assert_eq!(surface.meshes[1].bbox_min, Some([-1.0, 0.0, 0.0]), "a wire body with no vertices is bounded by its edge polyline");
    assert_eq!(surface.bbox_min, Some([-1.0, 0.0, 0.0]), "the surface bounds are the union a camera fit must frame");
    assert_eq!(surface.bbox_max, Some([2.0, 2.0, 3.0]));
    assert_eq!(surface.instances.iter().map(|instance| instance.interaction_id.as_str()).collect::<Vec<_>>(), vec!["extrude@solid"], "the topology target a hover/select observation is addressed by");
    assert_eq!(surface.selected, vec!["extrude@solid#0".to_string()]);
    assert_eq!(surface.hovered.as_deref(), Some("extrude@solid#0"));
}

/// 🚶️ The walk finds a preview scene wherever the dock nested it, and reports its ABSOLUTE page
/// rect — a rect reported parent-relative aims every pointer probe at the wrong pane.
#[test]
fn walk_mesh_stats_finds_nested_world3d_surfaces_at_their_absolute_rect() {
    let scene = ui_wgpu::wgpu::build_world_3d_scene("procedural-preview", "controller", ui_wgpu::wgpu::World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into()));
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let scene_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Explicit("procedural-preview".into()), WidgetSpec(scene)));
    tree.node_mut(root_id).unwrap().layout = LayoutBucket { x: 10.0, y: 20.0, width: 900.0, height: 800.0, ..Default::default() };
    tree.node_mut(scene_id).unwrap().layout = LayoutBucket { x: 5.0, y: 6.0, width: 400.0, height: 300.0, ..Default::default() };

    let mut surfaces = Vec::new();
    walk_mesh_stats(&tree, root_id, 0.0, 0.0, &mut surfaces);
    assert_eq!(surfaces.len(), 1, "the nested preview is found");
    assert_eq!(surfaces[0].rect, [15.0, 26.0, 400.0, 300.0]);
    assert!(surfaces[0].meshes.is_empty(), "an empty publication is an empty row list, never a missing surface");
}

/// 🎬️ A scene host for a tree that declares no engine surface — it can never be called, and saying
/// so is cheaper than pulling the real one into an introspection law.
struct NoSceneHost;

impl ui_wgpu::wgpu::SceneHost for NoSceneHost {
    fn paint_slot_step(
        &mut self,
        _slot: &ui_wgpu::wgpu::SceneSlot<'_>,
        _cursor: &mut ui_wgpu::wgpu::ScenePaintCursor,
        _draw: &mut ui_wgpu::wgpu::DrawList,
        _atlas: &mut ui_wgpu::wgpu::FontAtlas,
        _icons: Option<&ui_wgpu::wgpu::IconAtlas>,
    ) -> ui_wgpu::wgpu::ScenePaintStep {
        ui_wgpu::wgpu::ScenePaintStep::Fault
    }
}

/// 🖼️ LAW: a window that really painted chrome answers `drawCalls > 0` and `glyphCount > 0`.
///
/// 🩸️ `dumpFrameStats` is the ONLY oracle a headless probe has for "did anything reach the frame",
/// and the wgpu shell's black-canvas regression was read off it: `drawCalls:0, glyphCount:0` against
/// a canvas that had, in fact, built 121 frames. This law drives the real pipeline — `apply_tree`,
/// the real `MountedLayoutJob`, the real retained paint — so an oracle that stops counting a painted
/// window fails here instead of in a screenshot (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
#[test]
fn a_painted_chrome_window_reports_draw_calls_quads_and_glyphs() {
    let mut engine = ui_wgpu::wgpu::Ui::new();
    let window_id = "chrome-law";
    engine.apply_tree(window_id, &stack_node(Some("root"), vec![text_node("Puzzle 3D"), text_node("Catalogue")]));
    engine.set_viewport(window_id, 640.0, 360.0);

    let mut atlas = ui_wgpu::wgpu::FontAtlas::from_bytes(&[]).expect("the deterministic builtin bitmap atlas");
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    for _ in 0..64 {
        for _ in 0..16_384 {
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview_sequence);
            if matches!(engine.step_layouts(&pool, &mut atlas, &mut cx), ui_wgpu::wgpu::UiLayoutStep::Idle) {
                break;
            }
        }
        if !engine.layout_is_dirty(window_id) {
            break;
        }
        engine.request_layout(window_id);
    }
    assert!(!engine.layout_is_dirty(window_id), "layout never settled");

    let mut painted = false;
    for _ in 0..131_072 {
        match engine.frame_step::<NoSceneHost>(window_id, 640.0, 360.0, &mut atlas, None, None) {
            ui_wgpu::wgpu::UiFrameStep::Pending => {}
            ui_wgpu::wgpu::UiFrameStep::Ready => {
                painted = true;
                break;
            }
            step => panic!("retained paint answered {step:?}"),
        }
    }
    assert!(painted, "retained paint never completed");

    let stats = build_frame_stats(&engine, Some(window_id));
    assert_eq!(stats.window_id.as_deref(), Some(window_id));
    assert!(stats.draw_calls > 0, "a painted chrome window submits at least one non-empty layer, got {stats:?}", stats = (stats.draw_calls, stats.quad_count, stats.glyph_count));
    assert!(stats.quad_count > 0, "chrome quads reach the frame");
    assert!(stats.glyph_count > 0, "and so does every glyph of its labels");
    assert!(stats.glyph_count <= stats.quad_count, "a glyph is itself one quad, so it can never outnumber them");
}

//#region 🎯️ChromeLedgerLaws
fn chrome_hit(control_id: &str, kind: ui_wgpu::wgpu::HitKind, rect: Rect, event: Option<ActionDescriptor>) -> ui_wgpu::wgpu::HitTarget<ActionDescriptor> {
    ui_wgpu::wgpu::HitTarget { rect, event, control_id: Some(control_id.to_string()), kind, drag_axis: None, drag_data: None }
}

fn chrome_action(controller_id: &str, action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: semio_framework::optional_json_to_dsl(args) }
}

/// 🎯️ The registry row a probe aims with: the ABSOLUTE rect the chrome registered, the control id it
/// minted, the hit kind, and — only for a retained body row — the window that owns it. Chrome rows own
/// no window, which is the ONE signal that tells a navbar chip apart from a document row.
#[test]
fn chrome_hit_rows_carry_absolute_rect_kind_and_only_body_rows_name_a_window() {
    let owners = [("puzzle3d-tree.row.0".to_string(), ("puzzle3d-tree".to_string(), Rect::new(0.0, 0.0, 300.0, 600.0)))].into_iter().collect::<std::collections::HashMap<_, _>>();
    let chrome = chrome_hit("shell.panel.tab.artifact", ui_wgpu::wgpu::HitKind::PanelTab, Rect::new(12.0, 40.0, 96.0, 28.0), None);
    let body = chrome_hit("puzzle3d-tree.row.0", ui_wgpu::wgpu::HitKind::TreeItem, Rect::new(8.0, 120.0, 280.0, 24.0), Some(chrome_action("puzzle3d", "selectNode", Some(serde_json::json!({"nodeId": "n1"})))));

    let chrome_row = chrome_hit_row(&chrome, &owners);
    assert_eq!(chrome_row.control_id, "shell.panel.tab.artifact");
    assert_eq!(chrome_row.kind, "PanelTab");
    assert_eq!(chrome_row.rect, [12.0, 40.0, 96.0, 28.0], "the rect is the one the pointer resolves against, not a parent-relative one");
    assert_eq!(chrome_row.window_id, None, "a chrome control belongs to the shell, never to a window body");
    assert_eq!(chrome_row.action, None);

    let body_row = chrome_hit_row(&body, &owners);
    assert_eq!(body_row.window_id.as_deref(), Some("puzzle3d-tree"));
    assert_eq!(body_row.controller_id.as_deref(), Some("puzzle3d"));
    assert_eq!(body_row.action.as_deref(), Some("selectNode"), "a registered event publishes the action the row will dispatch");
}

/// 🎬️ Every dispatch is one entry with a monotonic `seq`, its `windowId` lifted out of the arguments,
/// its `origin` carried the way React's input ledger carries one, and the ledger never grows past
/// `CHROME_ACTION_CAPACITY` however long a session runs.
#[test]
fn chrome_action_ledger_mints_monotonic_seq_and_stays_bounded() {
    let mut ledger = ChromeLedger::default();
    ledger_push_action(&mut ledger, &chrome_action("framework", "togglePanel", Some(serde_json::json!({"panelId": "artifact", "windowId": "puzzle3d-scene"}))), "user");
    assert_eq!(ledger.actions.len(), 1);
    assert_eq!(ledger.actions[0].seq, 1);
    assert_eq!(ledger.actions[0].controller_id, "framework");
    assert_eq!(ledger.actions[0].action, "togglePanel");
    assert_eq!(ledger.actions[0].window_id.as_deref(), Some("puzzle3d-scene"), "the window an action addresses is lifted out of its arguments");
    assert_eq!(ledger.actions[0].origin, "user", "a control press is `user` provenance, exactly as React's input ledger records it");
    assert!(ledger.actions[0].args.as_deref().is_some_and(|args| args.contains("\"artifact\"")), "the arguments are published as JSON text, got {:?}", ledger.actions[0].args);

    ledger_push_action(&mut ledger, &chrome_action("puzzle3d", "setCamera", Some(serde_json::json!({"surfaceId": "puzzle3d-main-perspective"}))), "gesture");
    assert_eq!(ledger.actions[1].origin, "gesture", "a pointer/camera stream an engine surface derived is `gesture`, the one origin React never toasts");
    assert_eq!(ledger.actions[1].window_id.as_deref(), Some("puzzle3d-main-perspective"), "a surface-addressed row still names the window a probe scopes by");

    for _ in 0..CHROME_ACTION_CAPACITY + 16 {
        ledger_push_action(&mut ledger, &chrome_action("puzzle3d", "setCamera", None), "gesture");
    }
    assert_eq!(ledger.actions.len(), CHROME_ACTION_CAPACITY, "the ledger is a ring, not a leak");
    assert_eq!(ledger.actions.back().map(|entry| entry.seq), Some(CHROME_ACTION_CAPACITY as u64 + 18), "seq keeps counting past the trim so a probe can diff across steps");
    assert!(ledger.actions.front().map(|entry| entry.seq).is_some_and(|seq| seq > 1), "the oldest entries are the ones that fall off");
}

/// ✂️ An action carrying a large payload is remembered truncated — the ledger is a witness, never a
/// second copy of the wire.
#[test]
fn chrome_action_args_are_truncated_on_a_character_boundary() {
    let long = "ü".repeat(CHROME_ACTION_ARGS_BYTES);
    let args = chrome_action_args(&chrome_action("puzzle3d", "setDocument", Some(serde_json::json!({"document": long})))).expect("args are published");
    assert!(args.len() <= CHROME_ACTION_ARGS_BYTES, "got {} bytes", args.len());
    assert!(std::str::from_utf8(args.as_bytes()).is_ok(), "the truncation never splits a character");
    assert!(chrome_action_args(&chrome_action("puzzle3d", "undo", None)).is_none(), "an action with no arguments publishes none");
}

/// 🪟️ A named window keeps that window's rows PLUS the chrome (which owns no window and is what a probe
/// usually aims at); an unnamed read answers the whole registry, and `armed` reports the diagnostics gate
/// so a probe tells "switch off" from "nothing registered".
#[test]
fn chrome_dump_window_filter_keeps_chrome_and_reports_the_diagnostics_gate() {
    let owners = [
        ("a.row".to_string(), ("window-a".to_string(), Rect::new(0.0, 0.0, 10.0, 10.0))),
        ("b.row".to_string(), ("window-b".to_string(), Rect::new(0.0, 0.0, 10.0, 10.0))),
    ]
    .into_iter()
    .collect::<std::collections::HashMap<_, _>>();
    let hits = vec![
        chrome_hit("shell.navbar.artifact", ui_wgpu::wgpu::HitKind::NavbarItem, Rect::new(0.0, 0.0, 40.0, 40.0), None),
        chrome_hit("a.row", ui_wgpu::wgpu::HitKind::TreeItem, Rect::new(0.0, 40.0, 40.0, 20.0), None),
        chrome_hit("b.row", ui_wgpu::wgpu::HitKind::TreeItem, Rect::new(0.0, 60.0, 40.0, 20.0), None),
    ];
    let mut ledger = ChromeLedger::default();
    ledger_publish_hits(&mut ledger, &hits, &owners);
    assert_eq!(ledger.generation, 1, "each published walk moves the generation, so a stale snapshot is visible as one");

    let all = project_chrome_dump(&ledger, None, true);
    assert_eq!(all.hits.len(), 3);
    assert!(all.armed);
    let scoped = project_chrome_dump(&ledger, Some("window-a"), false);
    assert_eq!(scoped.hits.iter().map(|hit| hit.control_id.as_str()).collect::<Vec<_>>(), vec!["shell.navbar.artifact", "a.row"], "the other window's rows drop, the chrome stays");
    assert!(!scoped.armed, "an unarmed page says so rather than answering an empty registry");
    assert_eq!(project_chrome_dump(&ledger, Some(""), true).hits.len(), 3, "an empty window id is no filter at all");
}
/// 🪟️ The surface census is what makes two shells comparable: every row carries the LEVEL React's DOM
/// states and React's own element id for that surface. Rows are sorted and deduplicated, so a frame
/// that walks its chrome in another order never reads as a surface change, and a named read answers
/// only that surface.
#[test]
fn chrome_surface_census_publishes_react_levels_sorted_and_deduplicated() {
    let census = vec![
        ("puzzle3d-main-top".to_string(), "window", "puzzle3d-main-top".to_string()),
        ("framework.panel.artifact".to_string(), "panel", "framework.panelTab.framework.panel.artifact".to_string()),
        ("puzzle3d-main-top".to_string(), "window", "puzzle3d-main-top".to_string()),
        ("ui.introduction".to_string(), "dialog", "ui.introduction".to_string()),
    ];
    let mut ledger = ChromeLedger::default();
    ledger_publish_surfaces(&mut ledger, &census);
    assert_eq!(ledger.surfaces.iter().map(|surface| (surface.level.as_str(), surface.id.as_str())).collect::<Vec<_>>(), vec![("dialog", "ui.introduction"), ("panel", "framework.panel.artifact"), ("window", "puzzle3d-main-top")], "one row per surface, level first");
    assert_eq!(ledger.surfaces[1].element_id, "framework.panelTab.framework.panel.artifact", "a panel is named by React's `panelTabElementId`, never by the window instance the shell keeps it in");

    let dump = project_chrome_dump(&ledger, None, true);
    assert_eq!(dump.surfaces.len(), 3);
    let scoped = project_chrome_dump(&ledger, Some("framework.panel.artifact"), true);
    assert_eq!(scoped.surfaces.len(), 1, "a named read answers that surface alone");
    ledger_publish_surfaces(&mut ledger, &[]);
    assert!(ledger.surfaces.is_empty(), "a walk that carries no surface publishes none, rather than retaining the last frame's");
}

//#endregion 🎯️ChromeLedgerLaws
