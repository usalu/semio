/// 🧲️ Marquee and navigator laws: the marquee drag, the point pick, the you are here viewport rectangle, middle drag panning and the navigator wheel.

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
    let fixture: Value = serde_json::from_str(fixture_text!("../../../../../🧱️elements/🖌️Paint2dHost/🧪️tests/🧫️fixtures/🧭️navigator-camera/🔣️.json")).expect("neutral Navigator camera fixture");
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
