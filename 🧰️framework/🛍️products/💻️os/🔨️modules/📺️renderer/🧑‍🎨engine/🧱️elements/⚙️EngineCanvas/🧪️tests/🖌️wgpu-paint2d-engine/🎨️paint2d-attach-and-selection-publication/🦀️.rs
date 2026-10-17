/// 🎨️ Paint2d attach and selection publication: the raster host attach and paint, the layers domain selection interaction, hover, wheel camera and the navigator view mode.

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
