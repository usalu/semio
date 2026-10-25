// 🎭️ Scene host, overlay body and frame laws: the recording and backpressure scene hosts, the scene at queries, the open and modal overlay body placement and the atomic frame candidate and scene host routing laws.

//#region 🔖️SceneHostTests
fn component_scene_ui(surface_id: &str) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "ctrl".into(),
        component_kind: SurfaceKind::World3d,
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

/// 🎬️ A bare-bones `SceneHost` recording every call it receives, so tests can assert `Ui::frame`
/// actually reaches the host (once per slot, with the right payload) instead of just trusting the
/// wiring compiles. Paints a single filled rect per slot so a hosted frame's `DrawList` is
/// distinguishable from an unpainted one.
struct RecordingSceneHost {
    paint_calls: usize,
    last_surface_id: Option<String>,
}

impl SceneHost for RecordingSceneHost {
    fn paint_slot_step(&mut self, slot: &SceneSlot<'_>, cursor: &mut ScenePaintCursor, draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
        match cursor.bind(slot.node) {
            Ok(true) => {}
            Ok(false) => return ScenePaintStep::Pending,
            Err(_) => return ScenePaintStep::Fault,
        }
        self.paint_calls += 1;
        self.last_surface_id = slot.surface().map(|(surface_id, _)| surface_id.to_string());
        draw.push_rounded([slot.rect.x, slot.rect.y, slot.rect.w, slot.rect.h], Theme::default().accent, 0.0);
        cursor.finish()
    }
}

struct BackpressureSceneHost {
    calls: usize,
    released: bool,
    external_completed: u64,
}

impl SceneHost for BackpressureSceneHost {
    fn paint_slot_step(&mut self, slot: &SceneSlot<'_>, cursor: &mut ScenePaintCursor, _draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
        self.calls += 1;
        match cursor.bind(slot.node) {
            Ok(true) => {}
            Ok(false) => return ScenePaintStep::Pending,
            Err(_) => return ScenePaintStep::Fault,
        }
        if !self.released {
            return ScenePaintStep::Pending;
        }
        if self.external_completed == 0 {
            self.external_completed = 1;
            return if cursor.observe_external_progress(1).is_ok() { ScenePaintStep::Pending } else { ScenePaintStep::Fault };
        }
        if cursor.advance_phase().is_err() {
            return ScenePaintStep::Fault;
        }
        cursor.finish()
    }
}

//#region 🖱️SceneAtTests
/// 📍️ `Ui::scene_at` answers the `ComponentScene` leaf under a window-local point with its identity
/// AND the absolute rect it was laid out at — the two things a right-click needs before any event is
/// dispatched (see [`Ui::scene_at`]).
#[test]
fn scene_at_answers_the_component_scene_leaf_under_the_point() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.at")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let hit = ui.scene_at("w", 200.0, 200.0).expect("a point inside the scene resolves it");
    assert_eq!(hit.surface_id, "surface.at");
    assert_eq!(hit.controller_id, "ctrl");
    assert_eq!(hit.kind, SurfaceKind::World3d);
    assert!(hit.rect.contains(200.0, 200.0), "the reported rect must be the one the point fell in, got {:?}", hit.rect);
}

/// 🖱️ A point OUTSIDE every scene answers `None`, which is what sends a right-click to the shell's
/// own `"window"` menu instead of inventing a surface for it.
#[test]
fn scene_at_answers_none_outside_every_scene() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![button_ui("b", "Button")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    assert!(ui.scene_at("w", 10.0, 10.0).is_none());
    assert!(ui.scene_at("missing-window", 10.0, 10.0).is_none());
}

/// 👁️ The query must NOT move focus, arm a press capture or emit a command: a right-click resolving
/// its surface is not a press, and React's `onContextMenu` calls `preventDefault()` rather than
/// letting one through. A real `PointerDown` at the same point DOES emit one — that contrast is the
/// whole reason `scene_at` exists instead of dispatching an event to find out.
#[test]
fn scene_at_is_read_only_where_a_press_is_not() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.readonly")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let _ = ui.scene_at("w", 200.0, 200.0);
    assert!(ui.dispatch_event("w", UiEvent::PointerMove { x: 380.0, y: 380.0, modifiers: Default::default() }).iter().all(|command| !matches!(command, UiCommand::FocusChanged { .. })));
    let pressed = ui.dispatch_event("w", UiEvent::PointerDown { x: 200.0, y: 200.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(pressed.iter().any(|command| matches!(command, UiCommand::Scene { .. })), "a real press reaches the scene lane, got {pressed:?}");
}
//#endregion 🖱️SceneAtTests

//#region 🪟️OverlayBodyTests
/// 🪟️ The first `ComponentScene` leaf's arena id — overlay tests need a real content root and the
/// façade exposes node ids only through the tree itself.
fn first_component_scene_node(ui: &Ui, window_id: &str) -> semio_framework_ui_arena::NodeId {
    let tree = ui.tree(window_id).expect("window has a tree");
    let root = tree.root.expect("tree has a root");
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if matches!(tree.node(id).map(|node| &node.spec.0), Some(UiNode::ComponentScene(_))) {
            return id;
        }
        stack.extend(tree.children(id));
    }
    panic!("no ComponentScene leaf in {window_id}");
}

/// ⚖️ f32 round-trip tolerance: the walk origin is `placement - layout`, and the walk adds `layout`
/// back, so the recovered coordinate is the placement to within one f32 rounding step.
fn assert_close(left: (f32, f32), right: (f32, f32), what: &str) {
    assert!((left.0 - right.0).abs() < 0.01 && (left.1 - right.1).abs() < 0.01, "{what}: {left:?} != {right:?}");
}

/** 🔓️ An OPEN floating overlay's BODY is positioned at its resolved placement, not at the
 * in-flow slot it was authored in — and the body's own content keeps painting through the normal node
 * pipeline, which is what `scene_at` resolving the scene INSIDE the overlay proves (the walk that
 * answers it is the paint walk's own geometry). React gets the same from a portal plus
 * `PopoverContent`'s fixed positioning. */
#[test]
fn an_open_overlays_body_is_positioned_at_its_resolved_placement() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.popover")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let content = first_component_scene_node(&ui, "w");
    let in_flow = ui.scene_at("w", 200.0, 200.0).expect("the scene resolves in flow").rect;

    ui.open_overlay("w", content, OverlayKind::Popover, OverlayAnchor::Point { x: 40.0, y: 30.0 });
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);
    let placement = *ui.overlay_placements("w").first().expect("one open overlay");
    assert_eq!(placement.root, content);

    let placed = ui.scene_at("w", placement.x + 2.0, placement.y + 2.0).expect("the overlay body resolves at its placement");
    assert_eq!(placed.surface_id, "surface.popover");
    assert_close((placed.rect.x, placed.rect.y), (placement.x, placement.y), "the overlay body must sit at its placement");
    assert!((in_flow.y - placement.y).abs() > 0.5, "this fixture must actually move the body, in-flow {in_flow:?} vs placement {placement:?}");
}

/// 📪️ Closing the overlay returns its body to the in-flow position — the origin override is per
/// frame, never a stored offset, so nothing has to be unwound.
#[test]
fn closing_an_overlay_returns_its_body_to_the_in_flow_position() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.popover")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let content = first_component_scene_node(&ui, "w");
    let in_flow = ui.scene_at("w", 200.0, 200.0).expect("in flow").rect;

    ui.open_overlay("w", content, OverlayKind::Popover, OverlayAnchor::Point { x: 40.0, y: 30.0 });
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);
    let _ = ui.close_overlay("w", content);
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);

    let restored = ui.scene_at("w", 200.0, 200.0).expect("in flow again").rect;
    assert_close((restored.x, restored.y), (in_flow.x, in_flow.y), "a closed overlay is back in flow");
}

/// 🎯️ A `Dialog`/`CommandPalette` is CENTERED rather than anchored, and its body follows the same
/// one origin rule — `OverlayKind::default_placement`'s `Centered` branch.
#[test]
fn a_modal_overlays_body_is_centered_in_the_viewport() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.dialog")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let content = first_component_scene_node(&ui, "w");
    ui.open_overlay("w", content, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });
    let _ = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None);
    let placement = *ui.overlay_placements("w").first().expect("one open overlay");
    assert!(placement.backdrop, "a Dialog draws a scrim");
    let placed = ui.scene_at("w", placement.x + 2.0, placement.y + 2.0).expect("the modal body resolves at its placement");
    assert_close((placed.rect.x, placed.rect.y), (placement.x, placement.y), "a centered modal body sits at its placement");
}
/// 🪜️ W15a item 5. `paint_overlay_backdrop`/`paint_overlay_surface` existed but were reachable
/// only through the `🪟️OverlayApi` façade, which no host ever called — an open `Dialog`/`Popover`
/// therefore painted its content with NO surface under it on every frame a host did not hand-pump
/// (W1n gap 3). This law drives the production ladder (`frame_into_step`) and pins three things:
/// the chrome is painted by the ladder itself; it lands in the OVERLAY bucket, which composites
/// above every panel; and the overlay's own content lands there too, AFTER the chrome, so the
/// surface can never hide the content it exists to carry.
#[test]
fn the_frame_ladder_paints_an_open_overlays_own_surface_chrome_under_its_content() {
    fn settle(ui: &mut Ui, atlas: &mut FontAtlas, body: Rect) -> DrawList {
        let mut draw = DrawList::default();
        for _ in 0..262_144 {
            match ui.frame_into_step::<RecordingSceneHost>("w", body, atlas, None, None, &mut draw) {
                UiFrameStep::Pending => {}
                UiFrameStep::Ready => return draw,
                step => panic!("retained paint answered {step:?} (phase {:?})", ui.paint_frame_phase("w")),
            }
        }
        panic!("retained paint never completed: {}", ui.paint_stall_census("w"));
    }
    let overlay_instances = |draw: &DrawList| -> usize { draw.layers.iter().map(|layer| layer.overlay_ui_instances.len()).sum() };
    let body = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.popover")]));
    drive_layout(&mut ui, "w", body.w, body.h, &mut atlas);
    let closed = settle(&mut ui, &mut atlas, body);
    assert_eq!(overlay_instances(&closed), 0, "a surface with no open overlay puts nothing in the overlay bucket");

    let content = first_component_scene_node(&ui, "w");
    ui.open_overlay("w", content, OverlayKind::Popover, OverlayAnchor::Point { x: 40.0, y: 30.0 });
    drive_layout(&mut ui, "w", body.w, body.h, &mut atlas);
    let opened = settle(&mut ui, &mut atlas, body);
    assert!(overlay_instances(&opened) > 0, "the ladder itself must paint the open Popover's surface chrome — no host pumps `overlay_placements`");
    assert!(!opened.overlay_routed(), "every overlay-routed region the ladder opened is closed again by the time the frame publishes");

    let placement = *ui.overlay_placements("w").first().expect("one open overlay");
    let bucket: Vec<[f32; 4]> = opened.layers.iter().flat_map(|layer| layer.overlay_ui_instances.iter()).map(|instance| instance.rect).collect();
    let surface_at = bucket.iter().position(|rect| (rect[0] - placement.x).abs() < 0.01 && (rect[1] - placement.y).abs() < 0.01 && (rect[2] - placement.width).abs() < 0.01);
    let surface_at = surface_at.expect("the overlay surface is painted at the placement layout resolved");
    assert!(bucket.len() > surface_at + 1, "the overlay's own content paints into the SAME bucket, after the chrome — otherwise the surface covers it");

    let _ = ui.close_overlay("w", content);
    ui.open_overlay("w", content, OverlayKind::SelectPopup, OverlayAnchor::Node(content));
    drive_layout(&mut ui, "w", body.w, body.h, &mut atlas);
    let popup = settle(&mut ui, &mut atlas, body);
    assert_eq!(overlay_instances(&popup), 0, "a SelectPopup owns its own surface — the ladder must not paint a second one over it");
}

#[test]
fn frame_into_step_publishes_one_atomic_candidate_only_when_ready() {
    let body = Rect::new(12.0, 18.0, 240.0, 160.0);
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("atomic-frame", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("Accepted label"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]));
    drive_layout(&mut ui, "atomic-frame", body.w, body.h, &mut atlas);
    let mut target = DrawList::default();
    target.push_solid([1.0, 2.0, 3.0, 4.0], semio_framework_ui_theme::Rgba::new(0.1, 0.2, 0.3, 1.0));
    let baseline_layers = target.layers.len();
    let baseline_quads = target.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum::<usize>();
    let mut prior_progress = None;
    let mut progress_changes = 0;
    for _ in 0..262_144 {
        match ui.frame_into_step::<RecordingSceneHost>("atomic-frame", body, &mut atlas, None, None, &mut target) {
            UiFrameStep::Pending => {
                let progress = ui.paint_frame_progress("atomic-frame").expect("pending frame progress");
                progress_changes += usize::from(prior_progress.is_none_or(|prior| progress != prior));
                prior_progress = Some(progress);
                assert_eq!(target.layers.len(), baseline_layers, "an in-flight retained frame cannot publish layers");
                assert_eq!(target.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum::<usize>(), baseline_quads, "an in-flight retained frame cannot publish quads");
                assert!(ui.window_hit_targets("atomic-frame").is_empty(), "an in-flight retained frame cannot publish hit targets");
            }
            UiFrameStep::Ready => {
                assert!(target.layers.len() > baseline_layers);
                assert!(target.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum::<usize>() > baseline_quads);
                assert!(ui.paint_census("atomic-frame").is_some_and(|census| census.glyphs > 0));
                assert!(progress_changes > 0, "owned cursor work must move the retained progress witness before publication");
                return;
            }
            step => panic!("atomic retained frame answered {step:?}: {}", ui.paint_stall_census("atomic-frame")),
        }
    }
    panic!("atomic retained frame never completed: {}", ui.paint_stall_census("atomic-frame"));
}

#[test]
fn retained_frame_progress_ignores_unchanged_scene_backpressure_and_observes_external_work() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🧱️retained-frame-progress/🔣️.json")).expect("retained frame fixture");
    assert_eq!(fixture["sceneBackpressure"]["externalCompleted"], serde_json::json!([0, 0, 1]));
    let body = Rect::new(0.0, 0.0, 240.0, 160.0);
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    let mut host = BackpressureSceneHost { calls: 0, released: false, external_completed: 0 };
    ui.apply_tree("scene-backpressure", &stack_ui(vec![component_scene_ui("surface.backpressure")]));
    drive_layout(&mut ui, "scene-backpressure", body.w, body.h, &mut atlas);
    let mut target = DrawList::default();
    for _ in 0..262_144 {
        assert_eq!(ui.frame_into_step("scene-backpressure", body, &mut atlas, None, Some(&mut host), &mut target), UiFrameStep::Pending);
        if host.calls == 1 {
            break;
        }
    }
    assert_eq!(host.calls, 1, "the scene cursor reaches its bound backpressure phase");
    let after_bind = ui.paint_frame_progress("scene-backpressure").expect("bound scene progress");
    assert_eq!(ui.frame_into_step("scene-backpressure", body, &mut atlas, None, Some(&mut host), &mut target), UiFrameStep::Pending);
    assert_eq!(host.calls, 2);
    assert_eq!(ui.paint_frame_progress("scene-backpressure"), Some(after_bind), "Pending with the same scene and external cursor is not work");
    host.released = true;
    assert_eq!(ui.frame_into_step("scene-backpressure", body, &mut atlas, None, Some(&mut host), &mut target), UiFrameStep::Pending);
    assert_ne!(ui.paint_frame_progress("scene-backpressure"), Some(after_bind), "the producer's completed-work cursor is retained progress");
    for _ in 0..262_144 {
        match ui.frame_into_step("scene-backpressure", body, &mut atlas, None, Some(&mut host), &mut target) {
            UiFrameStep::Pending => {}
            UiFrameStep::Ready => return,
            step => panic!("backpressure frame answered {step:?}"),
        }
    }
    panic!("released scene frame never completed");
}

#[test]
fn same_size_moved_viewport_discards_the_old_origin_candidate_before_atomic_publication() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🧱️retained-frame-progress/🔣️.json")).expect("retained frame fixture");
    let rect = |value: &serde_json::Value| {
        let values = value.as_array().expect("fixture rect");
        Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32)
    };
    let initial = rect(&fixture["sameSizeOriginMove"]["initial"]);
    let moved = rect(&fixture["sameSizeOriginMove"]["moved"]);
    assert_eq!((initial.w, initial.h), (moved.w, moved.h));
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("moved-origin", &stack_ui(vec![button_ui("target", "Target")]));
    drive_layout(&mut ui, "moved-origin", initial.w, initial.h, &mut atlas);
    let mut target = DrawList::default();
    let baseline = UiFramePaintCensus::of(&target);
    for _ in 0..262_144 {
        assert_eq!(ui.frame_into_step::<RecordingSceneHost>("moved-origin", initial, &mut atlas, None, None, &mut target), UiFrameStep::Pending);
        let candidate_items = ui.windows.get("moved-origin").and_then(|window| window.paint_frame.as_ref()).map_or(0, |frame| UiFramePaintCensus::of(&frame.candidate).quads);
        if candidate_items > 0 {
            break;
        }
    }
    assert_eq!(UiFramePaintCensus::of(&target), baseline, "the first-origin candidate remains unpublished");
    for _ in 0..262_144 {
        match ui.frame_into_step::<RecordingSceneHost>("moved-origin", moved, &mut atlas, None, None, &mut target) {
            UiFrameStep::Pending => assert_eq!(UiFramePaintCensus::of(&target), baseline, "discard and repaint stay atomic"),
            UiFrameStep::Ready => break,
            step => panic!("moved-origin frame answered {step:?}"),
        }
    }
    let instances: Vec<_> = target.layers.iter().flat_map(|layer| layer.ui_instances.iter().chain(layer.overlay_ui_instances.iter())).collect();
    assert!(!instances.is_empty());
    assert!(instances.iter().all(|instance| instance.rect[0] >= moved.x && instance.rect[1] >= moved.y), "no geometry from the discarded origin may publish");
    assert!(ui.window_hit_targets("moved-origin").iter().all(|hit| hit.rect.x >= moved.x && hit.rect.y >= moved.y), "hit publication shares the accepted viewport origin");
}
//#endregion 🪟️OverlayBodyTests

#[test]
fn frame_with_no_scene_host_falls_back_to_the_placeholder_chrome() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.no-host")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);
    let draw = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list");
    let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    assert!(instances > 0, "with no scene host registered, paint_component_scene's placeholder chrome should still paint");
}

#[test]
fn frame_with_a_scene_host_routes_the_component_scene_leaf_through_it() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.host-test")]));
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);

    let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
    let draw = ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list even with a scene host registered");
    let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

    assert_eq!(host.paint_calls, 1, "the host should be invoked exactly once for the single ComponentScene leaf");
    assert_eq!(host.last_surface_id.as_deref(), Some("surface.host-test"));
    assert!(instances > 0, "the host's own draw call should still land in the frame's DrawList");
}

#[test]
fn frame_with_a_scene_host_still_paints_ancestor_chrome_around_the_hosted_slot() {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    let group_node = UiNode::Group(UiGroupNode {
        id: "group".into(),
        label: Label::data("Group"),
        default_open: None,
        presence: UiPresence::default(),
        children: vec![UiNode::Text(UiTextNode { value: Label::data("label"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), component_scene_ui("surface.nested")],
        menu: None,
    });
    ui.apply_tree("w", &group_node);
    drive_layout(&mut ui, "w", 400.0, 400.0, &mut atlas);

    let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
    ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list");

    assert_eq!(host.paint_calls, 1);
    assert_eq!(host.last_surface_id.as_deref(), Some("surface.nested"));
}
//#endregion 🔖️SceneHostTests
