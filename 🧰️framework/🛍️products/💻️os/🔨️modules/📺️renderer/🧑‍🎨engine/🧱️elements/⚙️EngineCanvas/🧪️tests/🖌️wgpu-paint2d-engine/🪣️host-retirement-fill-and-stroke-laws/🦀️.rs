/// 🪣️ Host retirement, fill and stroke laws: phase four binding retirement, the bucket fill region click and the long and mask stroke publications.

/// 🪟️ The retained paint binds exact editor tokens, and document retirement preserves sibling hosts.
#[test]
fn text_editor_phase_four_binding_retires_the_closed_host_and_preserves_its_sibling() {
    let _serialized = engine_surface_law_guard();
    let text = |id: &str| ENGINE_SURFACES.with(|cell| cell.borrow().get(id).and_then(|entry| entry.editor.as_ref()).map(|host| host.text().to_owned()));
    use semio_framework_os_renderer_wgpu_ui_engine_tests::surface_fixtures::{close_retained_surface_fixture, paint_retained_surface_in_window};
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/✍️editor/🧪️tests/🧫️fixtures/⌨️text-input/🔣️.json")).unwrap();
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
    let fixture:Value=serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🧫️fixtures/🎭️mask-stroke/🔣️.json")).unwrap();
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
