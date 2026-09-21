use super::*;
use ui_wgpu::wgpu::UiPresence;

fn sample_block(id: &str, x: f64, y: f64, w: f64, h: f64) -> Value {
    json!({
        "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": w, "height": h,
        "rotation": 0.0, "visible": true, "locked": false,
        "paragraphs": [], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
    })
}

#[test]
fn hit_test_prefers_topmost_block() {
    let blocks = vec![sample_block("a", 0.0, 0.0, 100.0, 100.0), sample_block("b", 20.0, 20.0, 100.0, 100.0)];
    let overrides = BTreeMap::new();
    let hits = ink_items_at_point(&blocks, &overrides, 50.0, 50.0);
    assert_eq!(ink_item_id(hits[0]), "b");
}

#[test]
fn hit_test_misses_outside_bounds() {
    let blocks = vec![sample_block("a", 0.0, 0.0, 10.0, 10.0)];
    let overrides = BTreeMap::new();
    assert!(ink_items_at_point(&blocks, &overrides, 50.0, 50.0).is_empty());
}

#[test]
fn resize_bounds_east_handle_grows_width_only() {
    let from = InkBoundsF { x: 0.0, y: 0.0, w: 100.0, h: 50.0 };
    let to = ink_resize_bounds(from, "e", 20.0, 0.0, 8.0);
    assert_eq!(to, InkBoundsF { x: 0.0, y: 0.0, w: 120.0, h: 50.0 });
}

#[test]
fn resize_bounds_northwest_handle_moves_origin() {
    let from = InkBoundsF { x: 10.0, y: 10.0, w: 100.0, h: 100.0 };
    let to = ink_resize_bounds(from, "nw", -10.0, -10.0, 8.0);
    assert_eq!(to, InkBoundsF { x: 0.0, y: 0.0, w: 110.0, h: 110.0 });
}

#[test]
fn resize_bounds_respects_minimum_size() {
    let from = InkBoundsF { x: 0.0, y: 0.0, w: 20.0, h: 20.0 };
    let to = ink_resize_bounds(from, "e", -100.0, 0.0, 8.0);
    assert_eq!(to.w, 8.0);
}

#[test]
fn screen_world_roundtrip() {
    let camera = InkCameraF { x: 12.0, y: -8.0, zoom: 1.5 };
    let inner = Rect::new(100.0, 40.0, 400.0, 300.0);
    let (wx, wy) = ink_screen_to_world(camera, inner, 250.0, 150.0);
    let (sx, sy) = ink_world_to_screen(camera, inner, wx, wy);
    assert!((sx - 250.0).abs() < 0.01);
    assert!((sy - 150.0).abs() < 0.01);
}

#[test]
fn snap_rounds_to_nearest_grid_cell() {
    assert_eq!(ink_snap_coordinate(13.0, 8.0), 16.0);
    assert_eq!(ink_snap_coordinate(3.0, 8.0), 0.0);
}

fn clipboard_scene(fixture: &Value, selection: &Value) -> UiComponentSceneNode {
    UiComponentSceneNode {
        host_id: "ink.clipboard.fixture".into(),
        surface_id: "ink.clipboard.fixture".into(),
        controller_id: "ink-clipboard-controller".into(),
        component_kind: SurfaceKind::InkCanvas,
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
        ink_canvas: Some(ui_wgpu::wgpu::InkCanvasScene {
            document_json: serde_json::to_string(&fixture["document"]).unwrap(),
            selection_json: serde_json::to_string(selection).unwrap(),
            hovered_id: None,
            active_utility: "selectDirect".into(),
            view_mode: "edit".into(),
            interactive: true,
            interaction_domain: None,
        }),
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}

fn domain_scene(fixture: &Value, utility: &str) -> UiComponentSceneNode {
    let mut scene = clipboard_scene(fixture, &fixture["scene"]["selectionJson"].as_str().and_then(|value| serde_json::from_str::<Value>(value).ok()).unwrap_or_else(|| json!([])));
    scene.surface_id = fixture["scene"]["surfaceId"].as_str().expect("surface id").into();
    scene.controller_id = fixture["scene"]["controllerId"].as_str().expect("controller id").into();
    let mut document = fixture["document"].clone();
    document["activeUtility"] = Value::String(utility.into());
    let ink = scene.ink_canvas.as_mut().expect("Ink scene");
    ink.document_json = serde_json::to_string(&document).expect("document JSON");
    ink.selection_json = fixture["scene"]["selectionJson"].as_str().expect("selection JSON").into();
    ink.hovered_id = fixture["scene"]["hoveredId"].as_str().map(str::to_owned);
    ink.interaction_domain = Some(ui_wgpu::wgpu::InkCanvasInteractionDomain {
        id: fixture["scene"]["interactionDomain"]["id"].as_str().expect("interaction domain id").into(),
        granularity_id: fixture["scene"]["interactionDomain"]["granularityId"].as_str().expect("interaction domain granularity").into(),
    });
    scene
}

fn run_domain_interaction(scene: &UiComponentSceneNode, event: InkInteractionEvent) -> Option<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut job = InkInteractionJob::new(1, 1, Some(ui_render::PointerId(7)), scene, event).expect("bounded interaction").expect("interactive scene");
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    for _ in 0..32 {
        if matches!(job.step(1, scene, bounds, &mut input).expect("interaction step"), InkInteractionStep::Complete) {
            break;
        }
    }
    input.take_action_step().expect("action queue remains valid").map(|action| action.into_descriptor().expect("action descriptor"))
}

fn assert_domain_action(action: ActionDescriptor, expected_action: &str, expected_args: &Value) {
    assert_eq!(action.action, expected_action);
    let args = Value::from(action.args.as_ref().expect("action args"));
    for key in ["domainId", "channel", "targets", "merge", "method"] {
        if let Some(expected) = expected_args.get(key) {
            assert_eq!(&args[key], expected, "{key}");
        }
    }
}

#[test]
fn ink_canvas_domain_hover_uses_scoped_topology_ids() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🖋️ink-canvas-domain-interaction/🔣️.json")).expect("domain fixture");
    let scene = domain_scene(&fixture, "selectDirect");
    let hover = run_domain_interaction(&scene, InkInteractionEvent::PointerMove { x: 40.0, y: 40.0 }).expect("hover action");
    assert_domain_action(hover, "interactionHover", &fixture["cases"][0]["args"]);
    let clear = run_domain_interaction(&scene, InkInteractionEvent::PointerMove { x: 300.0, y: 200.0 }).expect("hover clear");
    assert_domain_action(clear, "interactionHover", &fixture["cases"][1]["args"]);
}

#[test]
fn ink_canvas_domain_picks_use_scoped_topology_ids() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🖋️ink-canvas-domain-interaction/🔣️.json")).expect("domain fixture");
    let scene = domain_scene(&fixture, "selectDirect");
    let replace = run_domain_interaction(&scene, InkInteractionEvent::PointerDown { x: 40.0, y: 40.0, button: 0, shift: false }).expect("replace pick");
    assert_domain_action(replace, "interactionSelect", &fixture["cases"][2]["args"]);
    clear_ink_pointer_state(&scene.surface_id);
    let additive = run_domain_interaction(&scene, InkInteractionEvent::PointerDown { x: 180.0, y: 40.0, button: 0, shift: true }).expect("additive pick");
    assert_domain_action(additive, "interactionSelect", &fixture["cases"][3]["args"]);
    clear_ink_pointer_state(&scene.surface_id);
    let clear = run_domain_interaction(&scene, InkInteractionEvent::PointerDown { x: 300.0, y: 200.0, button: 0, shift: false }).expect("empty pick");
    assert_domain_action(clear, "interactionSelect", &fixture["cases"][4]["args"]);
    clear_ink_pointer_state(&scene.surface_id);
}

#[test]
fn ink_canvas_domain_marquee_uses_scoped_topology_ids() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🖋️ink-canvas-domain-interaction/🔣️.json")).expect("domain fixture");
    let scene = domain_scene(&fixture, "selectMarquee");
    assert!(run_domain_interaction(&scene, InkInteractionEvent::PointerDown { x: 10.0, y: 10.0, button: 0, shift: false }).is_none());
    assert!(run_domain_interaction(&scene, InkInteractionEvent::PointerMove { x: 270.0, y: 90.0 }).is_none());
    let marquee = run_domain_interaction(&scene, InkInteractionEvent::PointerUp { x: 270.0, y: 90.0 }).expect("rectangle selection");
    assert_domain_action(marquee, "interactionSelect", &fixture["cases"][5]["args"]);
    clear_ink_pointer_state(&scene.surface_id);
}

#[test]
fn shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-clipboard/🔣️.json")).expect("shared clipboard fixture");
    let scene = clipboard_scene(&fixture, &fixture["copy"]["selectedIds"]);
    let copied = ink_clipboard_copy_text(&scene).expect("bounded copy").expect("selected copy");
    let copied: Value = serde_json::from_str(&copied).unwrap();
    assert_eq!(copied["schema"], fixture["payloadSchema"]);
    assert_eq!(copied["blocks"].as_array().unwrap().iter().map(ink_item_id).collect::<Vec<_>>(), vec!["group-a", "image-a", "text-a"]);
    assert!(copied.get("assets").is_none());

    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let target = (fixture["paste"]["target"]["x"].as_f64().unwrap(), fixture["paste"]["target"]["y"].as_f64().unwrap());
    let payload = serde_json::to_string(&fixture["paste"]["blocks"]["payload"]).unwrap();
    assert!(ink_clipboard_paste_into(&scene, target, InkClipboardContentKind::Text, &payload, &mut input).unwrap());
    let action = input.take_action_step().unwrap().unwrap().into_descriptor().unwrap();
    let args = Value::from(action.args.as_ref().unwrap());
    let events: Value = serde_json::from_str(args["eventsJson"].as_str().unwrap()).unwrap();
    assert_eq!(events.as_array().unwrap().len(), 2);
    assert_eq!(events[0]["block"]["x"].as_f64(), fixture["paste"]["blocks"]["expectedTopLevelPoints"][0]["x"].as_f64());
    assert_eq!(events[0]["block"]["children"][0]["name"], fixture["paste"]["blocks"]["payload"]["blocks"][0]["children"][0]["name"]);
    assert_ne!(events[0]["block"]["children"][0]["id"], fixture["paste"]["blocks"]["payload"]["blocks"][0]["children"][0]["id"]);

    assert!(ink_clipboard_paste_into(&scene, target, InkClipboardContentKind::Text, fixture["paste"]["plainText"]["clipboard"].as_str().unwrap(), &mut input).unwrap());
    let plain = input.take_action_step().unwrap().unwrap().into_descriptor().unwrap();
    let plain_args = Value::from(plain.args.as_ref().unwrap());
    let plain_events: Value = serde_json::from_str(plain_args["eventsJson"].as_str().unwrap()).unwrap();
    assert_eq!(plain_events[0]["block"]["paragraphs"][0]["runs"][0]["text"], "Hello");
    assert_eq!(plain_events[0]["block"]["paragraphs"][1]["runs"][0]["text"], "Welt");

    assert!(ink_clipboard_paste_into(&scene, target, InkClipboardContentKind::Text, fixture["paste"]["svg"]["clipboard"].as_str().unwrap(), &mut input).unwrap());
    let svg = input.take_action_step().unwrap().unwrap().into_descriptor().unwrap();
    let svg_args = Value::from(svg.args.as_ref().unwrap());
    let svg_events: Value = serde_json::from_str(svg_args["eventsJson"].as_str().unwrap()).unwrap();
    assert_eq!(svg_events[0]["operation"], "putAsset");
    assert_eq!(svg_events[1]["block"]["kind"], "image");

    assert!(ink_clipboard_paste_into(&scene, target, InkClipboardContentKind::ImageDataUrl, fixture["paste"]["raster"]["dataUrl"].as_str().unwrap(), &mut input).unwrap());
    let raster = input.take_action_step().unwrap().unwrap().into_descriptor().unwrap();
    let raster_args = Value::from(raster.args.as_ref().unwrap());
    let raster_events: Value = serde_json::from_str(raster_args["eventsJson"].as_str().unwrap()).unwrap();
    assert_eq!(raster_events[0]["asset"]["mime"], fixture["paste"]["raster"]["mime"]);
    assert_eq!(raster_events[0]["asset"]["data"], fixture["paste"]["raster"]["dataUrl"]);
    for axis in ["width", "height"] {
        assert_eq!(raster_events[1]["block"][axis].as_f64(), fixture["paste"]["raster"]["size"][axis].as_f64(), "JSON numeric spelling cannot change the natural raster {axis}",);
    }
}

#[test]
fn ink_cancel_requires_the_exact_surface_generation_and_pointer_owner() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-clipboard/🔣️.json")).expect("shared clipboard fixture");
    let scene = clipboard_scene(&fixture, &fixture["copy"]["selectedIds"]);
    let pointer = ui_render::PointerId(7);
    let job = InkInteractionJob::new(3, 11, Some(pointer), &scene, InkInteractionEvent::PointerMove { x: 4.0, y: 5.0 }).expect("bounded Ink interaction").expect("interactive Ink surface");
    assert!(job.matches_pointer_owner(11, pointer));
    assert!(!job.matches_pointer_owner(12, pointer), "a reused surface generation cannot retire the old job");
    assert!(!job.matches_pointer_owner(11, ui_render::PointerId(8)), "a sibling pointer cannot retire the job");

    mutate_scene_state(&scene.surface_id, |state| {
        state.pointer_was_down = true;
        state.ink_marquee_points.extend([(1.0, 2.0), (3.0, 4.0)]);
        state.ink_overrides.insert("draft".into(), json!({ "id": "draft" }));
        state.drag = Some(SceneDrag { mode: SceneDragMode::InkPan { start_x: 1.0, start_y: 2.0, camera_x: 3.0, camera_y: 4.0, zoom: 1.0 } });
    });
    ink_pointer_cancel_into(&scene.surface_id);
    let state = scene_state(&scene.surface_id);
    assert!(!state.pointer_was_down);
    assert!(state.drag.is_none());
    assert!(state.ink_marquee_points.is_empty());
    assert!(state.ink_overrides.is_empty());
}

#[test]
fn ink_block_bounds_from_points() {
    let block = json!({
        "id": "i1", "kind": "stroke", "x": 10.0, "y": 10.0, "width": 1.0, "height": 1.0,
        "points": [[0.0, 0.0], [5.0, 10.0], [-5.0, 2.0]], "strokeWidth": 3.0, "color": [0, 0, 0, 1],
    });
    let bounds = ink_item_bounds(&block);
    assert_eq!(bounds.x, 5.0);
    assert_eq!(bounds.y, 10.0);
    assert_eq!(bounds.w, 10.0);
    assert_eq!(bounds.h, 10.0);
}

//#region InkCanvasPaintTests
#[test]
fn item_card_background_uses_the_background_token_not_panel() {
    let block = json!({
        "id": "i1", "kind": "text", "x": 0.0, "y": 0.0, "width": 100.0, "height": 40.0,
        "paragraphs": [], "fontSize": 16.0,
    });
    let doc: InkDocumentJson = serde_json::from_str("{}").unwrap();
    let scene = UiComponentSceneNode {
        host_id: "ink-paint-test".into(),
        surface_id: "ink-paint-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::InkCanvas,
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
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    };
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        let camera = InkCameraF { x: 0.0, y: 0.0, zoom: 1.0 };
        let inner = Rect::new(0.0, 0.0, 400.0, 300.0);
        draw_ink_item(&mut ctx, &scene, &block, camera, inner, &doc, false, false);
    }
    // 🎨️ `bg-background/90` in `ink-canvas-host.tsx` — the card's fill must resolve to
    // `theme.background`, not `theme.panel` (the app-chrome surface token).
    let expected = theme.background.with_alpha(0.9);
    let colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
    assert!(colors.contains(&[expected.r, expected.g, expected.b, expected.a]), "expected an item card fill at theme.background@0.9, got {colors:?}");
    let stale = theme.panel.with_alpha(0.92);
    assert!(!colors.contains(&[stale.r, stale.g, stale.b, stale.a]), "the item card must no longer fill with the stale theme.panel@0.92 token");
}

/// ♿️ An active editor is painted last and registered as a real retained Input hit, which is the
/// renderer's accessibility source for a textbox role and also keeps pointer resolution on the edit
/// control instead of the generic InkCanvas surface below it.
#[test]
fn active_ink_editor_registers_a_retained_accessible_input_over_the_surface() {
    let law: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-canvas-editing/🔣️.json")).expect("shared InkCanvas editing law parses");
    let surface_id = law["scene"]["surfaceId"].as_str().expect("surface id");
    let scene = UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: law["scene"]["controllerId"].as_str().expect("controller id").into(),
        component_kind: SurfaceKind::InkCanvas,
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
        ink_canvas: Some(ui_wgpu::wgpu::InkCanvasScene {
            document_json: serde_json::to_string(&law["document"]).expect("document serializes"),
            selection_json: "[]".into(),
            hovered_id: None,
            active_utility: "selectDirect".into(),
            view_mode: "edit".into(),
            interactive: true,
            interaction_domain: None,
        }),
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    };
    let block = &law["document"]["blocks"][0];
    let (_, mut edit) = ink_edit_for_point(block, law["gestures"]["text"]["x"].as_f64().unwrap(), law["gestures"]["text"]["y"].as_f64().unwrap()).expect("text block edits");
    let bounds = Rect::new(0.0, 0.0, law["viewport"]["width"].as_f64().unwrap() as f32, law["viewport"]["height"].as_f64().unwrap() as f32);
    edit.screen_rect = ink_edit_screen_rect(&edit, InkCameraF { x: 0.0, y: 0.0, zoom: 1.0 }, bounds);
    mutate_scene_state(surface_id, |state| state.ink_edit = Some(edit));

    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_ink_canvas(&scene, bounds, &mut ctx);
    }

    let control_id = "ink.edit.fixture.ink.text.text-a.input";
    let editor = input.staged_hits().iter().find(|hit| hit.control_id.as_deref() == Some(control_id)).expect("active editor registers a retained hit");
    assert_eq!(editor.kind, HitKind::Input, "Shell accessibility projects Input hits as textboxes");
    assert!(editor.rect.contains(law["gestures"]["text"]["x"].as_f64().unwrap() as f32, law["gestures"]["text"]["y"].as_f64().unwrap() as f32));
    input.publish_hits();
    assert_eq!(input.hit_at(law["gestures"]["text"]["x"].as_f64().unwrap() as f32, law["gestures"]["text"]["y"].as_f64().unwrap() as f32).and_then(|hit| hit.control_id.as_deref()), Some(control_id));
    cancel_ink_edit(surface_id, &mut input);
}
//#endregion InkCanvasPaintTests
