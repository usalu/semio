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
        host_id: surface_id.into(),
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
            { "kind": "pixel", "id": "base", "visible": true, "opacity": 1.0, "blendMode": "normal", "transform": {"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":-0.0,"d":1.0}, "mask": null, "width": 64, "height": 64 },
            { "kind": "pixel", "id": "overlay", "visible": true, "opacity": 0.5, "blendMode": "normal", "transform": {"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":-0.0,"d":1.0}, "mask": null, "width": 64, "height": 64 }
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
        brush_color: "#e07020".into(),
        brush_hardness: 0.25,
        paint_target:"pixels".into(),mask_value:255,fill_tolerance:24,pixel_selection_json:None,
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

fn drain_editor_actions_accepted(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let _ = drive_text_editor_outbox_step(input).expect("editor outbox drive");
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority") {
        let queued = action.into_envelope().expect("action envelope");
        if let Some(receipt) = queued.receipt {
            settle_text_editor_action_receipt(receipt, TextEditorActionOutcome::Accepted);
        }
        actions.push(queued.descriptor);
    }
    actions
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
    let args = camera.args.as_ref().and_then(semio_framework_value::DslValue::as_object).expect("setCamera args are an object");
    assert!(args.iter().any(|(key, _)| key == "surfaceId"), "the dispatch helper merges surfaceId into every paint-2d action");
    let nested = args.iter().find(|(key, _)| key == "camera").map(|(_, value)| value.clone()).expect("setCamera carries a nested camera object, matching dispatch(\"setCamera\", { camera: next })");
    let camera_fields = nested.as_object().expect("camera is an object");
    for key in ["x", "y", "zoom"] {
        assert!(camera_fields.iter().any(|(name, _)| name == key), "the camera carries {key}");
    }

    drop_engine_surface(surface_id);
}

#[test]
fn paint2d_navigator_view_mode_refuses_unarmed_editing_pointer_routes() {
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
    assert!(drain(&mut input).is_empty(), "Navigator editing is inert; camera gestures have their own routes");

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
    let actions = drain_editor_actions_accepted(&mut input);

    let names: Vec<&str> = actions.iter().map(|action| action.action.as_str()).collect();
    assert_eq!(names, vec!["textEdit", "textSelect"], "the edited document precedes its selection");
    let edit = actions.iter().find(|action| action.action == "textEdit").expect("textEdit");
    let fields = action_fields(edit);
    assert_eq!(fields.iter().find(|(key, _)| key == "text").map(|(_, value)| value.as_str()), Some("alpha!"), "the edit carries the current document in the authored action field");
    assert_eq!(fields.iter().find(|(key, _)| key == "surfaceId").map(|(_, value)| value.as_str()), Some(surface_id));
    assert_eq!(fields.iter().find(|(key, _)| key == ui_wgpu::wgpu::TEXT_EDITOR_TYPING_BUFFER_ARG).map(|(_, value)| value.as_str()), Some(surface_id), "a live typed edit names its buffer, so the window folds it into ONE typing run");
    let select = actions.iter().find(|action| action.action == "textSelect").expect("textSelect");
    let select_fields = serde_json::to_value(select.args.as_ref().unwrap()).expect("selection args");
    assert_eq!(select_fields, json!({ "surfaceId": surface_id, "start": 6, "end": 6 }));

    drop_engine_surface(surface_id);
}

#[test]
fn text_editor_explicit_draft_preserves_local_text_until_apply_or_discard() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/✏️TextEditor/🧫️fixtures/📝️explicit-draft/🔣️.json"))).expect("neutral explicit draft fixture");
    let law = &fixture["cases"][1];
    let id = "text-editor-explicit-draft";
    drop_engine_surface(id);
    let mut scene = text_editor_scene(id, law["initial"].as_str().unwrap());
    scene.text_editor.as_mut().unwrap().settings_json = Some(law["settings"].to_string());
    let bounds = Rect::new(0.0, 0.0, 480.0, 320.0);
    assert!(sync_engine_scene(&scene, "explicit-draft-law", bounds, &Theme::default()));

    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &PointerModifiers::default(), &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "typing stays local until Apply");
    assert!(text_editor_explicit_draft_is_dirty(&scene));
    assert!(text_editor_move_explicit_draft_history(&scene, false));
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some("Alpha"));
    assert!(text_editor_move_explicit_draft_history(&scene, true));
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some("Alpha!"));

    let mut unrelated = scene.clone();
    unrelated.text_editor.as_mut().unwrap().diagnostics_json = Some("[]".into());
    assert!(sync_engine_scene(&unrelated, "explicit-draft-law", bounds, &Theme::default()));
    assert_eq!(text_editor_accessibility_value(&unrelated).as_deref(), Some("Alpha!"), "unrelated scene refresh preserves the draft");

    assert_eq!(text_editor_apply_explicit_draft_into(&unrelated, &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "Apply uses the retained lane for every draft size");
    let action = loop {
        match drive_text_editor_retained_action_step().expect("retained draft page") {
            TextEditorRetainedActionStep::Pending => {}
            TextEditorRetainedActionStep::Ready(actions) => break actions.first,
            TextEditorRetainedActionStep::Idle => panic!("publication disappeared"),
        }
    };
    assert_eq!(action.descriptor.action, law["expected"]["action"].as_str().unwrap());
    assert_eq!(serde_json::to_value(action.descriptor.args.as_ref().unwrap()).unwrap(), json!({ "path": "/description", "value": "Alpha!" }));
    settle_text_editor_action_receipt(action.receipt.unwrap(), TextEditorActionOutcome::Accepted);
    assert!(text_editor_explicit_draft_is_dirty(&unrelated), "dispatch completion does not discard an unvalidated draft");

    let mut conflicted = unrelated.clone();
    conflicted.text_editor.as_mut().unwrap().buffer = fixture["preservation"]["collaboratorScene"].as_str().unwrap().into();
    assert!(sync_engine_scene(&conflicted, "explicit-draft-law", bounds, &Theme::default()));
    assert!(text_editor_explicit_draft_has_conflict(&conflicted));
    assert_eq!(text_editor_accessibility_value(&conflicted).as_deref(), Some("Alpha!"), "a collaborator refresh preserves the local draft");
    assert_eq!(text_editor_apply_explicit_draft_into(&conflicted, &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "a conflicted draft cannot overwrite collaborator data");

    assert!(text_editor_discard_explicit_draft(&conflicted));
    assert_eq!(text_editor_accessibility_value(&conflicted).as_deref(), fixture["preservation"]["collaboratorScene"].as_str());
    assert!(!text_editor_explicit_draft_is_dirty(&conflicted));
    assert!(!text_editor_explicit_draft_has_conflict(&conflicted));
    drop_engine_surface(id);
}

#[test]
fn text_editor_large_explicit_draft_pages_cancel_and_refusal_without_losing_text() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/✏️TextEditor/🧫️fixtures/📝️explicit-draft/🔣️.json"))).expect("neutral explicit draft fixture");
    let id = "text-editor-large-explicit-draft";
    drop_engine_surface(id);
    let initial = "a".repeat(65_535);
    let mut scene = text_editor_scene(id, &initial);
    scene.text_editor.as_mut().unwrap().settings_json = Some(fixture["cases"][0]["settings"].to_string());
    assert!(sync_engine_scene(&scene, "large-explicit-draft-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("🚀".into()), &PointerModifiers::default(), &mut input), Ok(true));
    let expected = format!("{initial}🚀");
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some(expected.as_str()));
    assert_eq!(text_editor_apply_explicit_draft_into(&scene, &mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "large draft never enters the inline bounded string queue");
    assert!(matches!(drive_text_editor_retained_action_step(), Ok(TextEditorRetainedActionStep::Pending)));
    assert!(text_editor_cancel_explicit_publication(&scene));
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some(expected.as_str()), "cancellation keeps the local draft");

    assert_eq!(text_editor_apply_explicit_draft_into(&scene, &mut input), Ok(true));
    let ready = loop {
        match drive_text_editor_retained_action_step().expect("retained source page") {
            TextEditorRetainedActionStep::Pending => {}
            TextEditorRetainedActionStep::Ready(actions) => break actions.first,
            TextEditorRetainedActionStep::Idle => panic!("publication disappeared"),
        }
    };
    assert_eq!(ready.descriptor.args.as_ref().and_then(|args| args.get("value")).and_then(semio_framework::DslValue::as_str), Some(expected.as_str()));
    let oracle = serde_json::to_value(ready.descriptor.args.as_ref().unwrap()).unwrap();
    assert_eq!(oracle["value"].as_str(), Some(expected.as_str()), "serde_json independently observes the same atomic value");
    settle_text_editor_action_receipt(ready.receipt.unwrap(), TextEditorActionOutcome::Refused("invalid-source"));
    let status = text_editor_explicit_draft_status(&scene);
    assert_eq!(status.error.as_deref(), Some("invalid-source"));
    assert!(!status.pending);
    assert_eq!(text_editor_accessibility_value(&scene).as_deref(), Some(expected.as_str()), "a refused action keeps the draft available for correction");
    drop_engine_surface(id);
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

#[test]
fn text_editor_renderer_keys_match_the_actual_react_fixture() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json"))).expect("neutral keyboard fixture");
    for law in fixture["rendererKeys"].as_array().expect("renderer keys") {
        let id = law["id"].as_str().unwrap();
        drop_engine_surface(id);
        let mut scene = text_editor_scene(id, law["text"].as_str().unwrap());
        let editor = scene.text_editor.as_mut().unwrap();
        editor.selection_json = Some(json!({ "start": law["selection"][0], "end": law["selection"][1] }).to_string());
        editor.settings_json = Some(json!({ "tabSize": law["tabSize"].as_u64().unwrap_or(2) }).to_string());
        editor.newline_gates_json = law.get("newlineGates").map(Value::to_string);
        assert!(sync_engine_scene(&scene, "text-key-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let key = match law["key"].as_str().unwrap() {
            "ArrowLeft" => KeyAction::ArrowLeft,
            "ArrowDown" => KeyAction::ArrowDown,
            "Home" => KeyAction::Home,
            "End" => KeyAction::End,
            "Tab" => KeyAction::Tab,
            "Enter" => KeyAction::Enter,
            " " => KeyAction::Space(true),
            key => KeyAction::Char(key.into()),
        };
        let modifiers = PointerModifiers { shift: law["shift"].as_bool().unwrap_or(false), alt: law["alt"].as_bool().unwrap_or(false), ..Default::default() };
        let mut input = InputState::<ActionDescriptor>::default();
        assert_eq!(text_editor_apply_key_into(&scene, &key, &modifiers, &mut input).expect("bounded key"), !modifiers.alt, "{id}");
        ENGINE_SURFACES.with(|cell| {
            let map = cell.borrow();
            let host = map.get(id).unwrap().editor.as_ref().unwrap();
            assert_eq!(host.text(), law["expect"]["text"].as_str().unwrap(), "{id}");
            assert_eq!(json!([host.anchor(), host.caret()]), law["expect"]["selection"], "{id}");
        });
        let actions = if law["operation"].is_null() { drain(&mut input) } else { drain_editor_actions_accepted(&mut input) };
        let expected = match law["operation"].as_str() {
            Some("insertText") => vec!["textEdit", "textSelect"],
            Some(_) => vec!["textSelect"],
            None => vec![],
        };
        assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), expected, "{id}");
        if let Some(select) = actions.last() {
            assert_eq!(serde_json::to_value(select.args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "start": law["expect"]["selection"][0], "end": law["expect"]["selection"][1] }), "{id}");
        }
        drop_engine_surface(id);
    }
}

#[test]
fn text_editor_outbox_preserves_local_echo_during_temporary_action_credit_pressure() {
    let _serialized = engine_surface_law_guard();
    let id = "text-editor-action-credits";
    drop_engine_surface(id);
    let scene = text_editor_scene(id, "alpha");
    assert!(sync_engine_scene(&scene, "editor-credit-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    let mut input = InputState::<ActionDescriptor>::default();
    for _ in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY - 1 {
        input.publish_action("c", "a", 2, |_, _| Ok(())).unwrap();
    }
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &PointerModifiers::default(), &mut input), Ok(true));
    assert_eq!(drive_text_editor_outbox_step(&mut input), Ok(false));
    assert!(has_pending_text_editor_outbox());
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let host = map.get(id).unwrap().editor.as_ref().unwrap();
        assert_eq!((host.text(), host.anchor(), host.caret()), ("alpha!", 6, 6));
    });
    let _ = drain(&mut input);
    let actions = drain_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"]);
    assert!(has_pending_text_editor_outbox(), "the open typing run keeps the outbox driven until it ends");
    assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::ArrowLeft, &PointerModifiers::default(), &mut input), Ok(true));
    let actions = drain_editor_actions_accepted(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"], "a pure caret move ends the typing run with ONE commit signal before its selection");
    assert_eq!(serde_json::to_value(actions[0].args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "typing": id, "typingCommit": "selectionJump" }));
    let selection = actions.last().unwrap();
    assert_eq!(serde_json::to_value(selection.args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "start": 5, "end": 5 }));
    assert!(!has_pending_text_editor_outbox(), "the ended run leaves nothing to drive");
    drop_engine_surface(id);
}

/// ⚖️ LAW (text-splice corpus `hostSignals`, shared with the React host): every host signal the wgpu editor owns ends its open
/// typing run with the corpus `typingCommit` reason — a pure caret move, the editor losing keyboard focus, the page hidden — as
/// ONE commit signal sent once the run's last keystroke left, and nothing is left to drive afterwards. The idle bound is the
/// tool-machine owner's constant.
#[test]
fn text_editor_host_signals_end_the_typing_run_with_the_corpus_reasons() {
    let _serialized = engine_surface_law_guard();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/✂️text-splice/🔣️.json")).expect("text-splice corpus");
    for row in corpus["hostSignals"].as_array().expect("host signals") {
        let (signal, commit) = (row["signal"].as_str().expect("signal"), row["commit"].as_str().expect("commit"));
        if signal == "idle" {
            assert_eq!((commit, ui_wgpu::wgpu::TEXT_EDITOR_TYPING_IDLE_MS), ("idle", 750), "the idle bound ends the run as `idle`");
            continue;
        }
        let id = format!("text-editor-signal-{signal}");
        drop_engine_surface(&id);
        let scene = text_editor_scene(&id, "alpha");
        assert!(sync_engine_scene(&scene, "editor-signal-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let mut input = InputState::<ActionDescriptor>::default();
        assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::Char("!".into()), &PointerModifiers::default(), &mut input), Ok(true));
        match signal {
            "caretMove" => {}
            "blur" => assert!(end_text_editor_typing(&id, TextEditorTypingEnd::Blur), "a keystroke in flight is a run the blur ends"),
            "hidden" => assert!(end_every_text_editor_typing(TextEditorTypingEnd::Hidden) >= 1, "the hidden page ends the run"),
            other => panic!("unknown host signal {other:?}"),
        }
        let delivered = drain_editor_actions_accepted(&mut input);
        assert_eq!(delivered.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["textEdit", "textSelect"], "{signal}: the keystroke leaves before the run ends");
        if signal == "caretMove" {
            assert_eq!(text_editor_apply_key_into(&scene, &KeyAction::ArrowLeft, &PointerModifiers::default(), &mut input), Ok(true));
        }
        let ended = drain_editor_actions_accepted(&mut input);
        assert_eq!(serde_json::to_value(ended[0].args.as_ref().expect("commit args")).expect("args"), json!({ "surfaceId": id, "typing": id, "typingCommit": commit }), "{signal}");
        assert_eq!(ended.len(), if signal == "caretMove" { 2 } else { 1 }, "{signal}: ONE commit signal");
        assert!(drain_editor_actions_accepted(&mut input).is_empty() && !has_pending_text_editor_outbox(), "{signal}: the ended run leaves nothing to drive");
        assert!(!end_text_editor_typing(&id, TextEditorTypingEnd::Blur), "{signal}: no run is left to end");
        drop_engine_surface(&id);
    }
}

#[test]
fn text_editor_outbox_pages_a_large_document_into_one_ordered_edit_and_selection_pair() {
    let _serialized = engine_surface_law_guard();
    let id = "text-editor-large-outbox";
    drop_engine_surface(id);
    let scene = text_editor_scene(id, "seed");
    assert!(sync_engine_scene(&scene, "editor-large-outbox-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
    let expected = format!("{}🚀", "a".repeat(65_535));
    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_replace_all_into(&scene, &expected, &mut input), Ok(true));
    assert_eq!(drive_text_editor_outbox_step(&mut input), Ok(true));
    assert!(drain(&mut input).is_empty(), "a large document never enters the bounded string ring");
    let actions = loop {
        match drive_text_editor_retained_action_step().expect("retained text page") {
            TextEditorRetainedActionStep::Pending => {}
            TextEditorRetainedActionStep::Ready(actions) => break actions,
            TextEditorRetainedActionStep::Idle => panic!("publication disappeared"),
        }
    };
    assert_eq!(actions.first.descriptor.action, "textEdit");
    assert_eq!(actions.first.descriptor.args.as_ref().and_then(|args| args.get("text")).and_then(semio_framework::DslValue::as_str), Some(expected.as_str()));
    assert_eq!(serde_json::to_value(actions.first.descriptor.args.as_ref().unwrap()).unwrap()["text"], expected);
    let selection = actions.second.expect("ordered selection");
    assert_eq!(selection.descriptor.action, "textSelect");
    assert_eq!(serde_json::to_value(selection.descriptor.args.as_ref().unwrap()).unwrap(), json!({ "surfaceId": id, "start": expected.len(), "end": expected.len() }));
    settle_text_editor_action_receipt(actions.first.receipt.unwrap(), TextEditorActionOutcome::Accepted);
    settle_text_editor_action_receipt(selection.receipt.unwrap(), TextEditorActionOutcome::Accepted);
    assert!(!has_pending_text_editor_outbox());
    drop_engine_surface(id);
}

#[test]
fn text_editor_receipts_match_the_neutral_first_latest_refusal_and_read_only_laws() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/✏️TextEditor/🧫️fixtures/📮️delivery/🔣️.json"))).expect("neutral delivery fixture");
    for law in fixture["cases"].as_array().expect("delivery cases") {
        let id = law["id"].as_str().unwrap();
        drop_engine_surface(id);
        let scene = text_editor_scene(id, law["initial"].as_str().unwrap());
        assert!(sync_engine_scene(&scene, "editor-delivery-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let mut input = InputState::<ActionDescriptor>::default();
        let typed = law["typed"].as_array().unwrap();
        assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(typed[0].as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
        assert!(drive_text_editor_outbox_step(&mut input).unwrap());
        let first_edit = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
        let mut dispatched = vec![format!("textEdit:{}", serde_json::to_value(first_edit.descriptor.args.as_ref().unwrap()).unwrap()["text"].as_str().unwrap())];
        assert_eq!(dispatched, law["expected"][0].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_owned()).collect::<Vec<_>>(), "{id}: leading edit");
        for value in &typed[1..] {
            assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(value.as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
        }
        let after_new_owner = law["typedAfterNewOwner"].as_array().cloned().unwrap_or_default();
        if !after_new_owner.is_empty() {
            assert!(sync_engine_scene(&text_editor_scene(id, law["initial"].as_str().unwrap()), "editor-delivery-law", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()), "{id}: the same host re-renders mid-flight with its guest's stale buffer");
            for value in &after_new_owner {
                assert!(text_editor_apply_key_into(&scene, &KeyAction::Char(value.as_str().unwrap().into()), &PointerModifiers::default(), &mut input).unwrap());
            }
        }
        let first_receipt = first_edit.receipt.unwrap();
        let accepted = law["outcome"] == "accepted";
        settle_text_editor_action_receipt(first_receipt, if accepted { TextEditorActionOutcome::Accepted } else { TextEditorActionOutcome::Refused(law["outcome"].as_str().unwrap()) });
        let first_selection = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
        if accepted {
            let args = serde_json::to_value(first_selection.descriptor.args.as_ref().unwrap()).unwrap();
            dispatched.push(format!("textSelect:{}:{}", args["start"], args["end"]));
            settle_text_editor_action_receipt(first_selection.receipt.unwrap(), TextEditorActionOutcome::Accepted);
        } else {
            settle_text_editor_action_receipt(first_selection.receipt.unwrap(), TextEditorActionOutcome::Cancelled);
        }
        if drive_text_editor_outbox_step(&mut input).unwrap() {
            let latest_edit = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
            let args = serde_json::to_value(latest_edit.descriptor.args.as_ref().unwrap()).unwrap();
            dispatched.push(format!("textEdit:{}", args["text"].as_str().unwrap()));
            assert_eq!(dispatched, law["expected"][1].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_owned()).collect::<Vec<_>>(), "{id}: latest edit");
            settle_text_editor_action_receipt(latest_edit.receipt.unwrap(), TextEditorActionOutcome::Accepted);
            let latest_selection = input.take_action_step().unwrap().unwrap().into_envelope().unwrap();
            let args = serde_json::to_value(latest_selection.descriptor.args.as_ref().unwrap()).unwrap();
            dispatched.push(format!("textSelect:{}:{}", args["start"], args["end"]));
            settle_text_editor_action_receipt(latest_selection.receipt.unwrap(), TextEditorActionOutcome::Accepted);
        }
        assert_eq!(dispatched, law["expected"][2].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_owned()).collect::<Vec<_>>(), "{id}: settled pair");
        let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned());
        let expected_text = if accepted { format!("{}{}", law["initial"].as_str().unwrap(), typed.iter().chain(after_new_owner.iter()).map(|row| row.as_str().unwrap()).collect::<String>()) } else { law["initial"].as_str().unwrap().to_owned() };
        assert_eq!(text, expected_text, "{id}: local echo");
        assert_eq!(text_editor_is_read_only(&scene), law["readOnly"].as_bool().unwrap(), "{id}: refusal mode");
        drop_engine_surface(id);
    }
}

#[test]
fn text_editor_scene_echoes_never_overwrite_newer_unsent_local_text() {
    let _serialized = engine_surface_law_guard();
    let id = "text-editor-local-echo";
    drop_engine_surface(id);
    let mut scene = text_editor_scene(id, "a");
    let bounds = Rect::new(0.0, 0.0, 480.0, 320.0);
    assert!(sync_engine_scene(&scene, "editor-echo-law", bounds, &Theme::default()));
    let mut input = InputState::<ActionDescriptor>::default();
    assert!(text_editor_apply_key_into(&scene, &KeyAction::Char("b".into()), &PointerModifiers::default(), &mut input).unwrap());
    assert!(drive_text_editor_outbox_step(&mut input).unwrap());
    assert!(text_editor_apply_key_into(&scene, &KeyAction::Char("c".into()), &PointerModifiers::default(), &mut input).unwrap());
    let _ = sync_text_editor_scene(&scene, bounds, &Theme::default());
    assert_eq!(ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned()), "abc", "an identical stale scene cannot revert local text");
    scene.text_editor.as_mut().unwrap().buffer = "ab".into();
    assert!(sync_text_editor_scene(&scene, bounds, &Theme::default()));
    let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned());
    assert_eq!(text, "abc", "the first edit's echo acknowledges it without overwriting unsent latest text");
    scene.text_editor.as_mut().unwrap().buffer = "external".into();
    assert!(sync_text_editor_scene(&scene, bounds, &Theme::default()));
    let text = ENGINE_SURFACES.with(|cell| cell.borrow().get(id).unwrap().editor.as_ref().unwrap().text().to_owned());
    assert_eq!(text, "external", "an unrelated guest buffer remains an authoritative external edit");
    drop_engine_surface(id);
}

#[test]
fn text_editor_delivery_state_matches_the_neutral_local_echo_ledger() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/✏️TextEditor/🧫️fixtures/🔁️local-echo/🔣️.json"))).expect("neutral local echo fixture");
    for law in fixture["cases"].as_array().unwrap() {
        let id = law["id"].as_str().unwrap();
        let mut state = TextEditorDeliveryState::default();
        let mut shown = law["initial"].as_str().unwrap().to_owned();
        let mut guest = shown.clone();
        let mut sequence = 1_u64;
        assert!(state.reconcile_buffer(&shown));
        for event in law["events"].as_array().unwrap() {
            if let Some(text) = event["local"].as_str() {
                if !state.read_only {
                    assert!(!state.guest_has_text(text), "{id}: fixture local must require textEdit");
                    shown = text.to_owned();
                    state.offer(TextEditorDeliverySnapshot { controller_id: "writer".into(), surface_id: id.into(), text: shown.clone(), start: shown.len(), end: shown.len() });
                    let token = std::num::NonZeroU64::new(sequence).unwrap();
                    assert!(state.note_dispatched(EngineSurfaceToken { slot: 0, generation: 1 }, token, true, true));
                    state.active = None;
                    sequence += 1;
                }
            }
            if let Some(text) = event["typed"].as_str() {
                if !state.read_only {
                    shown = text.to_owned();
                    state.offer(TextEditorDeliverySnapshot { controller_id: "writer".into(), surface_id: id.into(), text: shown.clone(), start: shown.len(), end: shown.len() });
                }
            }
            if let Some(text) = event["echo"].as_str() {
                guest = text.to_owned();
                if state.reconcile_buffer(text) {
                    shown = text.to_owned();
                }
            }
            if let Some(text) = event["refuse"].as_str() {
                let reason = event["reason"].as_str().unwrap_or("dispatch-failed");
                if matches!(reason, "undeclared-action" | "viewer-read-only") {
                    state.read_only = true;
                }
                if state.remove_pending_echo(text) && state.pending_echoes.is_empty() {
                    shown = guest.clone();
                }
            }
            assert_eq!(shown, event["expect"].as_str().unwrap(), "{id}: visible text");
            assert_eq!(state.pending_echoes.iter().map(String::as_str).collect::<Vec<_>>(), event["pending"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>(), "{id}: pending echo ledger");
            if let Some(read_only) = event["readOnly"].as_bool() {
                assert_eq!(state.read_only, read_only, "{id}: refusal mode");
            }
        }
    }
}

#[test]
fn text_editor_production_key_route_replays_chromiums_neutral_typing_sequences() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json"))).unwrap();
    for law in fixture["sequences"].as_array().unwrap() {
        let id = law["id"].as_str().unwrap();
        drop_engine_surface(id);
        let mut scene = text_editor_scene(id, law["text"].as_str().unwrap());
        scene.text_editor.as_mut().unwrap().selection_json = Some(json!({ "start": law["selection"][0], "end": law["selection"][1] }).to_string());
        assert!(sync_engine_scene(&scene, "text-input-oracle", Rect::new(0.0, 0.0, 480.0, 320.0), &Theme::default()));
        let mut input = InputState::<ActionDescriptor>::default();
        for step in law["steps"].as_array().unwrap() {
            let keys = if let Some(text) = step["type"].as_str() {
                text.chars()
                    .map(|ch| match ch {
                        '\n' => KeyAction::Enter,
                        ' ' => KeyAction::Space(true),
                        ch => KeyAction::Char(ch.to_string()),
                    })
                    .collect::<Vec<_>>()
            } else if let Some(text) = step["paste"].as_str().or_else(|| step["compose"].as_str()) {
                vec![KeyAction::Char(text.into())]
            } else {
                vec![match step["key"].as_str().unwrap() {
                    "ArrowLeft" => KeyAction::ArrowLeft,
                    "ArrowRight" => KeyAction::ArrowRight,
                    "ArrowUp" => KeyAction::ArrowUp,
                    "ArrowDown" => KeyAction::ArrowDown,
                    "Home" => KeyAction::Home,
                    "End" => KeyAction::End,
                    "Backspace" => KeyAction::Backspace,
                    "Delete" => KeyAction::Delete,
                    key => panic!("unexpected key {key}"),
                }]
            };
            for key in keys {
                assert!(text_editor_apply_key_into(&scene, &key, &PointerModifiers::default(), &mut input).expect("bounded key"), "{id}");
                let _ = drain_editor_actions_accepted(&mut input);
            }
        }
        ENGINE_SURFACES.with(|cell| {
            let map = cell.borrow();
            let host = map.get(id).unwrap().editor.as_ref().unwrap();
            assert_eq!(host.text(), law["expect"]["text"].as_str().unwrap(), "{id}");
            assert_eq!(json!([host.text()[..host.anchor()].chars().count(), host.text()[..host.caret()].chars().count()]), law["expect"]["selection"], "{id}");
        });
        drop_engine_surface(id);
    }
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
    let args = camera.args.as_ref().and_then(semio_framework_value::DslValue::as_object).expect("setCamera args are an object");
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

#[test]
fn paint2d_navigator_wheel_matches_the_mounted_react_camera_contract() {
    let _serialized = engine_surface_law_guard();
    let fixture: Value = serde_json::from_str(include_str!("../../../🖌️Paint2dHost/🧫️fixtures/🧭️navigator-camera/🔣️.json")).expect("neutral Navigator camera fixture");
    for sample in fixture["cases"].as_array().expect("camera cases") {
        let surface_id = sample["id"].as_str().expect("case identity");
        drop_engine_surface(surface_id);
        let mut scene = paint2d_scene(surface_id, "brush", &[]);
        let paint = scene.paint_2d.as_mut().expect("Paint scene");
        paint.view_mode = "navigator".into();
        paint.camera_json = sample["camera"].to_string();
        paint.composite_viewport_json = (!sample["viewport"].is_null()).then(|| sample["viewport"].to_string());
        let dimension = |index: usize| sample["surface"][index].as_f64().expect("surface dimension") as f32;
        let bounds = Rect { x: dimension(0), y: dimension(1), w: dimension(2), h: dimension(3) };
        assert!(sync_engine_scene(&scene, "navigator-wheel", bounds, &Theme::default()));
        let fit_camera = with_raster_host_mut(surface_id, |host| host.camera_json()).expect("Navigator host");
        let mut input = InputState::<ActionDescriptor>::default();
        assert!(paint2d_wheel_into(&scene, bounds, sample["point"][0].as_f64().unwrap() as f32, sample["point"][1].as_f64().unwrap() as f32, sample["deltaY"].as_f64().unwrap() as f32, &mut input).expect("bounded camera publish"));
        let actions = drain(&mut input);
        assert_eq!(actions.len(), 1, "{surface_id}");
        let action = &actions[0];
        assert_eq!(action.action, "setCamera");
        assert_eq!(action.controller_id, "raster");
        let args = action.args.as_ref().and_then(semio_framework_value::DslValue::as_object).expect("camera action arguments");
        let camera = args.iter().find(|(key, _)| key == "camera").and_then(|(_, value)| value.as_object()).expect("nested camera");
        for key in ["x", "y", "zoom"] {
            let actual = camera.iter().find(|(name, _)| name == key).and_then(|(_, value)| value.as_f64()).expect("camera coordinate");
            assert!((actual - sample["expected"][key].as_f64().unwrap()).abs() < 1e-9, "{surface_id}: {key} = {actual}");
        }
        assert_eq!(with_raster_host_mut(surface_id, |host| host.camera_json()).unwrap(), fit_camera, "the Navigator retains its own document-fit camera");
        drop_engine_surface(surface_id);
    }
}
//#endregion Paint2dMarqueeAndNavigatorTests

/// 🪟️ The retained paint binds exact editor tokens, and document retirement preserves sibling hosts.
#[test]
fn text_editor_phase_four_binding_retires_the_closed_host_and_preserves_its_sibling() {
    let _serialized = engine_surface_law_guard();
    let text = |id: &str| ENGINE_SURFACES.with(|cell| cell.borrow().get(id).and_then(|entry| entry.editor.as_ref()).map(|host| host.text().to_owned()));
    use super::engine_surface_attach_tests::{close_retained_surface_fixture, paint_retained_surface_in_window};
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json"))).unwrap();
    let law = &fixture["hostLifecycle"];
    let window_a = law["closingWindow"].as_str().unwrap();
    let window_b = law["siblingWindow"].as_str().unwrap();
    let bounds = Rect::new(0.0, 0.0, 400.0, 240.0);
    let scene_a = text_editor_scene(law["surfaceId"].as_str().unwrap(), law["text"].as_str().unwrap());
    let scene_b = text_editor_scene(law["surfaceId"].as_str().unwrap(), law["text"].as_str().unwrap());
    let painted_a = paint_retained_surface_in_window(&scene_a, bounds, window_a);
    let painted_b = paint_retained_surface_in_window(&scene_b, bounds, window_b);
    let token_a = engine_surface_token(&painted_a.owner.host_id).expect("first editor was painted and bound");
    let token_b = engine_surface_token(&painted_b.owner.host_id).expect("sibling editor was painted and bound");
    assert_ne!(token_a, token_b);
    close_retained_surface_fixture(window_a);
    assert_eq!(engine_surface_token(&painted_a.owner.host_id), None);
    assert_eq!(engine_surface_token_at(usize::from(token_a.slot)), Ok(None));
    assert_eq!(engine_surface_token(&painted_b.owner.host_id), Some(token_b));
    assert_eq!(engine_surface_token_at(usize::from(token_b.slot)), Ok(Some(token_b)));
    assert_eq!(text(&painted_b.owner.host_id).as_deref(), law["expect"]["siblingText"].as_str());
    let successor = text_editor_scene(law["surfaceId"].as_str().unwrap(), law["expect"]["successorText"].as_str().unwrap());
    let painted_successor = paint_retained_surface_in_window(&successor, bounds, window_a);
    let successor_token = engine_surface_token(&painted_successor.owner.host_id).expect("the reopened window owns a new editor generation");
    assert_ne!(successor_token, token_a);
    assert_eq!(text(&painted_successor.owner.host_id).as_deref(), law["expect"]["successorText"].as_str());
    assert_eq!(text(&painted_b.owner.host_id).as_deref(), law["expect"]["siblingText"].as_str());
    close_retained_surface_fixture(window_a);
    close_retained_surface_fixture(window_b);
}

/// 🪣️ The wgpu bucket is React's: the press publishes ONE `fillRegion { surfaceId, layerId, x, y }` in the layer image's
/// pixels (no host flood, no stroke), and a press off the layer's pixel grid publishes nothing.
#[test]
fn paint2d_bucket_press_publishes_one_fill_region_click_in_layer_pixels() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-bucket";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "paintBucket", &["base"]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");
    let (x, y) = with_raster_host_mut(surface_id, |host| host.world_to_screen_point(0.0, 0.0)).unwrap();
    let mut input = InputState::<ActionDescriptor>::default();
    assert!(paint2d_pointer_button_into(&scene, bounds, x as f32, y as f32, true, 0, false, false, &mut input).unwrap());
    let actions = drain(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "fillRegion");
    let args: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(actions[0].args.as_ref().expect("the click carries arguments"))).unwrap();
    assert_eq!(args.as_object().map(|object| object.len()), Some(4), "exactly surfaceId, layerId, x and y: {args}");
    assert_eq!((args["surfaceId"].as_str(), args["layerId"].as_str(), args["x"].as_f64(), args["y"].as_f64()), (Some(surface_id), Some("base"), Some(32.0), Some(32.0)));
    assert!(with_raster_host_mut(surface_id, |host| host.paint_edit().is_none()).unwrap());
    assert!(drain(&mut input).is_empty());
    let (x, y) = with_raster_host_mut(surface_id, |host| host.world_to_screen_point(40.0, 0.0)).unwrap();
    assert!(paint2d_pointer_button_into(&scene, bounds, x as f32, y as f32, true, 0, false, false, &mut input).unwrap());
    assert!(drain(&mut input).iter().all(|action| action.action != "fillRegion"), "a press off the pixel grid fills nothing");
    drop_engine_surface(surface_id);
}

/// 🌊️ A long wgpu stroke streams like React's: `paintStroke{phase: stream, gesture}` ticks while the pointer moves, then
/// ONE `paintStroke{phase: commit, gesture}` with the rest on release — one press id, every sample once, in order.
#[test]
fn paint2d_long_stroke_streams_ticks_then_commits_under_one_press() {
    let _serialized = engine_surface_law_guard();
    let surface_id = "paint2d-stream";
    drop_engine_surface(surface_id);
    let scene = paint2d_scene(surface_id, "paintBrush", &["base"]);
    let bounds = Rect { x: 0.0, y: 0.0, w: 640.0, h: 480.0 };
    assert!(sync_engine_scene(&scene, "law-window", bounds, &Theme::default()), "attach");
    let (x, y) = with_raster_host_mut(surface_id, |host| host.world_to_screen_point(-20.0, 0.0)).unwrap();
    let mut input = InputState::<ActionDescriptor>::default();
    assert!(paint2d_pointer_button_into(&scene, bounds, x as f32, y as f32, true, 0, false, false, &mut input).unwrap());
    let mut actions = drain(&mut input);
    for step in 1..=40 {
        paint2d_pointer_move_into(&scene, bounds, x as f32 + step as f32, y as f32, &mut input).unwrap();
        actions.extend(drain(&mut input));
    }
    assert!(paint2d_pointer_button_into(&scene, bounds, x as f32 + 41.0, y as f32, false, 0, false, false, &mut input).unwrap());
    actions.extend(drain(&mut input));
    let strokes: Vec<Value> = actions.iter().filter(|action| action.action == "paintStroke").map(|action| serde_json::from_str(&semio_framework_pack_json::to_json_string(action.args.as_ref().expect("a stroke carries arguments"))).unwrap()).collect();
    assert_eq!(strokes.iter().map(|args| args["phase"].as_str()).collect::<Vec<_>>(), [Some("stream"), Some("stream"), Some("commit")]);
    assert!(strokes.iter().all(|args| args["gesture"].is_string() && args["gesture"] == strokes[0]["gesture"] && args["layerId"] == "base" && args["tool"] == "brush"));
    let xs: Vec<f64> = strokes.iter().flat_map(|args| args["xs"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect::<Vec<_>>()).collect();
    assert_eq!(xs.len(), 42, "every sample once");
    assert!(xs.windows(2).all(|pair| pair[1] > pair[0]), "in drawing order");
    drop_engine_surface(surface_id);
}

#[test]
fn paint2d_mask_stroke_publishes_one_paint_stroke_at_the_shared_target(){
    let _serialized=engine_surface_law_guard();
    let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🎭️mask-stroke/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case|case["assets"].as_object().unwrap().is_empty()){
        let surface_id=format!("paint2d-mask-{}",case["name"].as_str().unwrap());drop_engine_surface(&surface_id);
        let mut scene=paint2d_scene(&surface_id,"paintBrush",&["p"]);let paint=scene.paint_2d.as_mut().unwrap();paint.document_sync_json=case["document"].to_string();paint.paint_target="mask".into();paint.mask_value=96;
        let bounds=Rect{x:0.0,y:0.0,w:640.0,h:480.0};assert!(sync_engine_scene(&scene,"law-window",bounds,&Theme::default()));
        let (x,y)=with_raster_host_mut(&surface_id,|host|host.world_to_screen_point(case["worldPoint"][0].as_f64().unwrap(),case["worldPoint"][1].as_f64().unwrap())).unwrap();
        let mut input=InputState::<ActionDescriptor>::default();assert!(paint2d_pointer_button_into(&scene,bounds,x as f32,y as f32,true,0,false,false,&mut input).unwrap());assert!(paint2d_pointer_button_into(&scene,bounds,x as f32,y as f32,false,0,false,false,&mut input).unwrap());
        let actions=drain(&mut input);assert_eq!(actions.len(),1);assert_eq!(actions[0].action,"paintStroke");
        let args:Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(actions[0].args.as_ref().expect("the stroke carries arguments"))).unwrap();
        assert_eq!(args["layerId"],"p");assert_eq!(args["tool"],"brush");assert!(args.get("operation").is_none()&&args.get("expectedMask").is_none());
        for (index,axis) in ["xs","ys"].into_iter().enumerate(){assert_eq!(args[axis][0].as_f64(),case["pixelPoint"][index].as_f64());}assert!(with_raster_host_mut(&surface_id,|host|host.paint_edit().is_none()).unwrap());drop_engine_surface(&surface_id);
    }
}
