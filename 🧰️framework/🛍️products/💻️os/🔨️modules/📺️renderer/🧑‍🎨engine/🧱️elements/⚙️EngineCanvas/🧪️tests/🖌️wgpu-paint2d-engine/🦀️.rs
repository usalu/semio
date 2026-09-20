//! 🖌️ Wgpu paint-2d and text-editor attach — the production path that constructs a `RasterHost` /
//! `EditorHost` behind a `SurfaceKind::Paint2d` / `SurfaceKind::TextEditor` window, feeds it the
//! scene, paints it through the host's own vector renderer, and routes pointer/wheel/keyboard input.
//!
//! The action payloads are pinned against React's own hosts — the other implementation of these
//! surfaces (`🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `world3dHoverActionArgs`/`world3dSelectionActionArgs`
//! under the `"layers"` domain, `dispatch("setCamera", { camera })`; `🧱️elements/✏️TextEditor/🟦️.tsx` —
//! the `textEdit`/`textSelect` pair every keystroke commits).

use super::node_graph_attach_tests::{action_fields, drop_engine_surface};
use super::*;
use ui_wgpu::wgpu::{InputState, Paint2dScene, SurfaceKind, TextEditorScene, UiPresence};

fn empty_scene(surface_id: &str, controller_id: &str, kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        menu: None,
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
    }
}

/// 🖼️ A two-pixel-layer raster document in the wire shape `RasterHost::sync_document_json` reads —
/// the same `RasterSnapshot.layers` projection the raster plugin publishes.
///
/// 🧾️ `schema` and per-layer `mask` are REQUIRED keys of that wire, not omittable ones:
/// `parse_document` refuses anything whose `schema` is not `"raster.document"`, and
/// `LayerNodeJson::Pixel` declares `mask` as an `Option<MaskJson>` with no serde default. The attach
/// path swallows the refusal (`let _ = host.sync_document_json(…)`,
/// `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2801`), so a fixture missing either key leaves the host with
/// NO layers — which reads downstream as "nothing is pickable anywhere in the viewport" rather than
/// as a parse fault. The raster plugin's own snapshots carry both
/// (`🖨️raster/…/📸️snapshot/⬅️before/🔣️.json`); this fixture carried neither
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
fn raster_document_json() -> String {
    json!({
        "schema": "raster.document",
        "layers": [
            { "kind": "pixel", "id": "base", "visible": true, "opacity": 1.0, "blendMode": "normal", "transform": { "x": 0.0, "y": 0.0, "scaleX": 1.0, "scaleY": 1.0, "rotation": 0.0 }, "mask": null, "width": 64, "height": 64 },
            { "kind": "pixel", "id": "overlay", "visible": true, "opacity": 0.5, "blendMode": "normal", "transform": { "x": 0.0, "y": 0.0, "scaleX": 1.0, "scaleY": 1.0, "rotation": 0.0 }, "mask": null, "width": 64, "height": 64 }
        ]
    })
    .to_string()
}

fn paint2d_scene(surface_id: &str, active_utility: &str, selection: &[&str]) -> UiComponentSceneNode {
    let mut scene = empty_scene(surface_id, "raster", SurfaceKind::Paint2d);
    scene.paint_2d = Some(Paint2dScene {
        document_sync_json: raster_document_json(),
        assets_json: "{}".into(),
        camera_json: json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 }).to_string(),
        selection_json: serde_json::to_string(selection).expect("selection encodes"),
        hovered_id: None,
        active_utility: active_utility.into(),
        brush_size: 12.0,
        brush_opacity: 0.8,
        view_mode: "composite".into(),
        composite_viewport_json: None,
        lanes: Vec::new(),
    });
    scene
}

/// ✏️ A text-editor surface whose declared caret sits at the END of `buffer`, the way a host that has
/// just loaded a document and focused it publishes one. The caret is part of the SCENE
/// (`selectionJson`), not an editor default — React's `TextEditorHost` restores it from the same
/// field — so a law about typing has to declare where the caret is instead of assuming zero
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration: the fixture said `{0,0}` while
/// its own assertions read `"alpha!"` and a caret of 6).
fn text_editor_scene(surface_id: &str, buffer: &str) -> UiComponentSceneNode {
    let mut scene = empty_scene(surface_id, "note", SurfaceKind::TextEditor);
    let caret = buffer.chars().count();
    scene.text_editor = Some(TextEditorScene::base(buffer.into(), Some("markdown".into()), Some(json!({ "start": caret, "end": caret }).to_string())));
    scene
}

/// 🎯️ A surface-local point the attached `RasterHost` reports a pixel layer under — found by
/// scanning, because the host owns the camera and this test pins the ACTION payload, not the
/// projection.
fn layer_hit_point(surface_id: &str, bounds: Rect) -> Option<(f32, f32)> {
    let mut y = 0.0f32;
    while y < bounds.h {
        let mut x = 0.0f32;
        while x < bounds.w {
            if paint2d_pick_layer(surface_id, f64::from(x), f64::from(y)).is_some() {
                return Some((bounds.x + x, bounds.y + y));
            }
            x += 8.0;
        }
        y += 8.0;
    }
    None
}

fn drain(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    crate::collect_fixture_actions(input)
}

#[test]
fn paint2d_window_attaches_the_raster_host_and_paints_a_non_empty_scene() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-attach-draw";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "brush", &[]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };

    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "the production dispatcher attaches a paint-2d surface");
    let layers = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| host.pick_targets_at_screen_json(0.0, 0.0))
    });
    assert!(layers.is_some(), "attach constructed the RasterHost the React session wraps");
    assert!(stage_engine_scene_paint(&scene, bounds, Theme::default().canvas_clear), "the raster host stages a composited engine packet");

    drop_engine_surface(surface_id);
}

#[test]
fn paint2d_selection_utility_publishes_the_layers_domain_interaction_react_dispatches() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-select";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "selectMarquee", &[]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    let (x, y) = layer_hit_point(surface_id, bounds).expect("the composited document has a pickable pixel layer somewhere in the viewport");
    assert!(paint2d_pointer_button_into(&scene, bounds, x, y, false, 0, false, false, &mut input).expect("bounded publish"));
    let actions = drain(&mut input);

    let select = actions.iter().find(|action| action.action == "interactionSelect").expect("a release under a selection utility publishes interactionSelect");
    assert_eq!(select.controller_id, "raster");
    let fields = action_fields(select);
    assert_eq!(fields.iter().find(|(key, _)| key == "domainId").map(|(_, value)| value.as_str()), Some("layers"), "raster's framework-owned interaction domain, exactly as React's PAINT2D_INTERACTION_DOMAIN");
    assert_eq!(fields.iter().find(|(key, _)| key == "merge").map(|(_, value)| value.as_str()), Some("replace"), "no modifier is a replace, exactly as marqueeModeFromModifiers resolves it");
    assert_eq!(fields.iter().find(|(key, _)| key == "method").map(|(_, value)| value.as_str()), Some("pick"));
    let targets = fields.iter().find(|(key, _)| key == "targets").map(|(_, value)| value.clone()).expect("targets");
    let parsed: Vec<Value> = serde_json::from_str(&targets).expect("targets is a JSON array, byte-identical to React's JSON.stringify(targets)");
    for target in &parsed {
        assert_eq!(target.get("granularity").and_then(Value::as_str), Some("layer"), "raster declares one granularity");
        assert!(target.get("id").and_then(Value::as_str).is_some());
    }

    drop_engine_surface(surface_id);
}

#[test]
fn paint2d_hover_under_a_selection_utility_publishes_the_pointer_channel_hover() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-hover";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "selectWand", &[]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    let (x, y) = layer_hit_point(surface_id, bounds).expect("the composited document has a pickable pixel layer somewhere in the viewport");
    assert!(paint2d_pointer_move_into(&scene, bounds, x, y, &mut input).expect("bounded publish"));
    let actions = drain(&mut input);

    let hover = actions.iter().find(|action| action.action == "interactionHover").expect("a move under a selection utility publishes interactionHover");
    let fields = action_fields(&hover.clone());
    assert_eq!(fields.iter().find(|(key, _)| key == "domainId").map(|(_, value)| value.as_str()), Some("layers"));
    assert_eq!(fields.iter().find(|(key, _)| key == "channel").map(|(_, value)| value.as_str()), Some("pointer"), "byte-identical to React's world3dHoverActionArgs channel");

    drop_engine_surface(surface_id);
}

#[test]
fn paint2d_wheel_republishes_the_host_camera_as_set_camera() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-wheel";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "brush", &[]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    assert!(paint2d_wheel_into(&scene, bounds, 320.0, 240.0, -120.0, &mut input).expect("bounded publish"));
    let actions = drain(&mut input);

    let camera = actions.iter().find(|action| action.action == "setCamera").expect("a wheel notch republishes the host camera");
    let args = camera.args.as_ref().and_then(dsl::DslValue::as_object).expect("setCamera args are an object");
    assert!(args.iter().any(|(key, _)| key == "surfaceId"), "the dispatch helper merges surfaceId into every paint-2d action");
    let nested = args.iter().find(|(key, _)| key == "camera").map(|(_, value)| value.clone()).expect("setCamera carries a nested camera object, matching dispatch(\"setCamera\", { camera: next })");
    let camera_fields = nested.as_object().expect("camera is an object");
    for key in ["x", "y", "zoom"] {
        assert!(camera_fields.iter().any(|(name, _)| name == key), "the camera carries {key}");
    }

    drop_engine_surface(surface_id);
}

#[test]
fn paint2d_navigator_view_mode_refuses_every_pointer_route() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-navigator";
    drop_engine_surface(surface_id);
    let mut scene = paint2d_scene(surface_id, "brush", &[]);
    if let Some(paint) = scene.paint_2d.as_mut() {
        paint.view_mode = "navigator".into();
    }
    let bounds = Rect { x: 0.0, y: 0.0, w: 320.0, h: 240.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    assert!(!paint2d_pointer_button_into(&scene, bounds, 100.0, 100.0, true, 0, false, false, &mut input).expect("bounded"));
    assert!(!paint2d_pointer_move_into(&scene, bounds, 100.0, 100.0, &mut input).expect("bounded"));
    assert!(!paint2d_wheel_into(&scene, bounds, 100.0, 100.0, -120.0, &mut input).expect("bounded"));
    assert!(drain(&mut input).is_empty(), "the navigator pane is a read-only overview, exactly as React's `if (isNavigator || !session) return`");

    drop_engine_surface(surface_id);
}

#[test]
fn text_editor_window_attaches_the_editor_host_and_a_key_commits_the_react_action_pair() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "text-editor-attach";
    drop_engine_surface(surface_id);
    let scene = text_editor_scene(surface_id, "alpha");
    let bounds = Rect { x: 0.0, y: 0.0, w: 480.0, h: 320.0 };

    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "the production dispatcher attaches a text-editor surface");
    let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).and_then(|entry| entry.editor.as_ref()).map(|host| host.text().to_owned()));
    assert_eq!(text.as_deref(), Some("alpha"), "the scene buffer reaches the EditorHost whole");
    assert!(stage_engine_scene_paint(&scene, bounds, Theme::default().panel), "the editor host stages a composited engine packet");

    let mut input = InputState::<ActionDescriptor>::default();
    let modifiers = PointerModifiers::default();
    assert!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &modifiers, &mut input).expect("bounded publish"), "a printable key is consumed by the focused editor");
    let actions = drain(&mut input);

    let names: Vec<&str> = actions.iter().map(|action| action.action.as_str()).collect();
    assert_eq!(names, vec!["textSelect", "textEdit"], "every keystroke commits the SAME ordered pair React's TextEditor dispatches");
    let edit = actions.iter().find(|action| action.action == "textEdit").expect("textEdit");
    let fields = action_fields(edit);
    assert_eq!(fields.iter().find(|(key, _)| key == "document").map(|(_, value)| value.as_str()), Some("alpha!"), "the edit carries the whole projected document, not a delta");
    assert_eq!(fields.iter().find(|(key, _)| key == "surfaceId").map(|(_, value)| value.as_str()), Some(surface_id));
    let select = actions.iter().find(|action| action.action == "textSelect").expect("textSelect");
    let select_fields = action_fields(select);
    assert_eq!(select_fields.iter().find(|(key, _)| key == "selectionJson").map(|(_, value)| value.as_str()), Some(r#"{"start":6,"end":6}"#), "the caret lands past the inserted glyph");

    drop_engine_surface(surface_id);
}

#[test]
fn text_editor_refuses_the_keys_the_shell_owns() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "text-editor-chords";
    drop_engine_surface(surface_id);
    let scene = text_editor_scene(surface_id, "alpha");
    let bounds = Rect { x: 0.0, y: 0.0, w: 480.0, h: 320.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    let mut modifiers = PointerModifiers::default();
    modifiers.meta = true;
    assert!(!text_editor_apply_key_into(&scene, &KeyAction::Char("k".into()), &modifiers, &mut input).expect("bounded"), "cmd+k stays a shell chord");
    assert!(!text_editor_apply_key_into(&scene, &KeyAction::Escape, &PointerModifiers::default(), &mut input).expect("bounded"), "Escape stays a shell chord");
    assert!(drain(&mut input).is_empty());

    drop_engine_surface(surface_id);
}

//#region Paint2dMarqueeAndNavigatorTests
/// 🖱️ React `🖌️Paint2dHost/🟦️.tsx` `onPointerDown`/`onPointerMove`/`onPointerUp`: a selection press
/// that TRAVELS past `PAINT_2D_MARQUEE_THRESHOLD_PX` becomes a marquee, and the release commits
/// `session.marqueeHitsJson(...)` through `commitMarqueeSelection` instead of the single-point pick.
#[test]
fn paint2d_marquee_drag_commits_the_hosts_marquee_hits_not_a_point_pick() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-marquee";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "selectMarquee", &[]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    assert!(!paint2d_pointer_button_into(&scene, bounds, 4.0, 4.0, true, 0, false, false, &mut input).expect("bounded"), "the press itself publishes nothing — it only arms the gesture");
    assert!(paint2d_marquee_overlay(surface_id, "selectMarquee").is_none(), "an armed but unmoved gesture paints no overlay");
    let _ = paint2d_pointer_move_into(&scene, bounds, 600.0, 440.0, &mut input);
    let (points, _crossing, lasso) = paint2d_marquee_overlay(surface_id, "selectMarquee").expect("a travelled gesture paints a marquee overlay");
    assert!(!lasso, "selectMarquee is the rectangle method");
    assert_eq!(points.len(), 2, "a rectangle marquee is its two corners");
    let _ = drain(&mut input);

    assert!(paint2d_pointer_button_into(&scene, bounds, 600.0, 440.0, false, 0, false, false, &mut input).expect("bounded"), "the release commits the marquee");
    let actions = drain(&mut input);
    let select = actions.iter().find(|action| action.action == "interactionSelect").expect("the marquee release publishes interactionSelect");
    let fields = action_fields(select);
    assert_eq!(fields.iter().find(|(key, _)| key == "domainId").map(|(_, value)| value.as_str()), Some("layers"));
    let targets = fields.iter().find(|(key, _)| key == "targets").map(|(_, value)| value.clone()).expect("targets");
    let parsed: Vec<Value> = serde_json::from_str(&targets).expect("targets is a JSON array");
    assert!(!parsed.is_empty(), "a marquee spanning the whole viewport covers the document's pixel layers");
    assert!(paint2d_marquee_overlay(surface_id, "selectMarquee").is_none(), "the release clears the overlay");

    drop_engine_surface(surface_id);
}

/// 🖱️ A selection press that does NOT travel stays a point pick — React only promotes `marqueeRef`
/// to `active` past the threshold, so a plain click still resolves one layer.
#[test]
fn paint2d_selection_press_without_travel_is_still_a_point_pick() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-marquee-click";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "selectMarquee", &[]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    let (x, y) = layer_hit_point(surface_id, bounds).expect("a pickable layer");
    let _ = paint2d_pointer_button_into(&scene, bounds, x, y, true, 0, false, false, &mut input);
    let _ = paint2d_pointer_move_into(&scene, bounds, x + 1.0, y + 1.0, &mut input);
    assert!(paint2d_marquee_overlay(surface_id, "selectMarquee").is_none(), "1 px of travel is under the 4 px threshold");
    let _ = drain(&mut input);
    assert!(paint2d_pointer_button_into(&scene, bounds, x + 1.0, y + 1.0, false, 0, false, false, &mut input).expect("bounded"));
    let actions = drain(&mut input);
    assert!(actions.iter().any(|action| action.action == "interactionSelect"), "the release still publishes a point pick");

    drop_engine_surface(surface_id);
}

/// 🧭️ React `🖌️Paint2dHost/🟦️.tsx:275`: a navigator surface carrying a `compositeViewportJson` draws
/// the content viewport's "you are here" rectangle, mapped into navigator screen space by
/// `navigatorViewportOverlayJson`. Without that field there is no rectangle at all.
#[test]
fn paint2d_navigator_publishes_the_you_are_here_viewport_rectangle() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-navigator";
    drop_engine_surface(surface_id);
    let mut scene = paint2d_scene(surface_id, "select", &[]);
    if let Some(paint) = scene.paint_2d.as_mut() {
        paint.view_mode = "navigator".into();
    }
    let bounds = Rect { x: 0.0, y: 0.0, w: 320.0, h: 240.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");
    assert!(paint2d_navigator_overlay_rect(&scene).is_none(), "no compositeViewportJson, no overlay — React renders null");

    if let Some(paint) = scene.paint_2d.as_mut() {
        paint.composite_viewport_json = Some(json!({ "width": 640.0, "height": 480.0 }).to_string());
    }
    let rect = paint2d_navigator_overlay_rect(&scene).expect("a navigator with a composite viewport draws the overlay");
    assert!(rect[2] > 0.0 && rect[3] > 0.0, "the overlay is a real rectangle, got {rect:?}");

    drop_engine_surface(surface_id);
}

/// 🧭️ React `🖌️Paint2dHost/🟦️.tsx:453-462`: a middle-button drag on the NAVIGATOR pans the CONTENT
/// camera — the screen delta divided by the content camera's zoom, dispatched as `setCamera`. The
/// navigator's own camera stays fit to the document and never moves.
#[test]
fn paint2d_navigator_middle_drag_pans_the_content_camera() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-navigator-pan";
    drop_engine_surface(surface_id);
    let mut scene = paint2d_scene(surface_id, "select", &[]);
    if let Some(paint) = scene.paint_2d.as_mut() {
        paint.view_mode = "navigator".into();
        paint.camera_json = json!({ "x": 10.0, "y": 20.0, "zoom": 2.0 }).to_string();
    }
    let bounds = Rect { x: 0.0, y: 0.0, w: 320.0, h: 240.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");

    let mut input = InputState::<ActionDescriptor>::default();
    let _ = paint2d_pointer_move_into(&scene, bounds, 100.0, 100.0, &mut input);
    assert!(drain(&mut input).is_empty(), "a navigator move with no armed pan publishes nothing");
    let _ = paint2d_pointer_button_into(&scene, bounds, 100.0, 100.0, true, 1, false, false, &mut input);
    assert!(paint2d_pointer_move_into(&scene, bounds, 120.0, 90.0, &mut input).expect("bounded"), "the armed pan consumes the move");
    let actions = drain(&mut input);
    let camera = actions.iter().find(|action| action.action == "setCamera").expect("a navigator pan publishes setCamera");
    let args = camera.args.as_ref().and_then(dsl::DslValue::as_object).expect("setCamera args are an object");
    let nested = args.iter().find(|(key, _)| key == "camera").map(|(_, value)| value.clone()).expect("setCamera carries a nested camera object");
    let camera_fields = nested.as_object().expect("camera is an object");
    let read = |key: &str| camera_fields.iter().find(|(name, _)| name == key).and_then(|(_, value)| value.as_f64()).unwrap_or_else(|| panic!("camera.{key}"));
    assert!((read("x") - (10.0 - 20.0 / 2.0)).abs() < 1e-6, "x travels by the screen delta divided by the CONTENT zoom, got {}", read("x"));
    assert!((read("y") - (20.0 - -10.0 / 2.0)).abs() < 1e-6, "y travels by the screen delta divided by the CONTENT zoom, got {}", read("y"));
    assert!((read("zoom") - 2.0).abs() < 1e-6, "a pan never changes the zoom");

    let _ = paint2d_pointer_button_into(&scene, bounds, 120.0, 90.0, false, 1, false, false, &mut input);
    let _ = drain(&mut input);
    let _ = paint2d_pointer_move_into(&scene, bounds, 200.0, 200.0, &mut input);
    assert!(drain(&mut input).is_empty(), "the release disarms the pan");

    drop_engine_surface(surface_id);
}
//#endregion Paint2dMarqueeAndNavigatorTests
