/// 🤏️ Two touch gestures and hover: the shared camera math with distinct transfer rules and the one empty owner transition on hover leave.

#[test]
fn board_and_map_two_touch_gestures_share_camera_math_but_keep_distinct_transfer_rules() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(SURFACE_PINCH_FIXTURE).expect("surface pinch contract");
    let fixture = fixture();
    let bounds = Rect {
        x: 0.0,
        y: 0.0,
        w: contract["viewport"]["width"].as_f64().expect("viewport width") as f32,
        h: contract["viewport"]["height"].as_f64().expect("viewport height") as f32,
    };
    let spread = &contract["gestures"]["spread"];
    let points = |key: &str| {
        spread[key]
            .as_array()
            .expect("gesture points")
            .iter()
            .map(|point| (ui_render::PointerId(point["pointerId"].as_u64().expect("pointer id")), point["x"].as_f64().expect("x") as f32, point["y"].as_f64().expect("y") as f32))
            .collect::<Vec<_>>()
    };
    let down = points("down");
    let moves = points("move");

    let mut map_scene = tiled_map_scene("touch-map-host", &fixture);
    map_scene.controller_id = "touch.map".into();
    let map_frame = paint_retained_surface_scene(&map_scene, bounds);
    let map_id = map_frame.owner.host_id.clone();
    let map_window = map_frame.owner.window_id.clone();
    assert!(tiled_map_set_camera_silent(&map_id, [0.0, 0.0, 10.0]));
    let seeded = tiled_map_camera(&map_id).expect("the seeded map camera, clamped to cover its viewport");
    let (initial, expected) = (&spread["initialCamera"], &spread["expectedCamera"]);
    assert_eq!((initial["x"].as_f64(), initial["y"].as_f64()), (expected["x"].as_f64(), expected["y"].as_f64()), "the spread gesture is centred: it scales about the viewport centre and pans nothing");
    let zoom_ratio = expected["zoom"].as_f64().expect("expected zoom") / initial["zoom"].as_f64().expect("initial zoom");
    let modifiers = Default::default();
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&map_frame.owner, bounds, down[0].0, down[0].1, down[0].2, &modifiers));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&map_id));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&map_frame.owner, bounds, down[1].0, down[1].1, down[1].2, &modifiers));
    assert!(!semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&map_id), "pinch takes ownership from the first contact");
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(&map_id, bounds, moves[0].0, moves[0].1, moves[0].2));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(&map_id, bounds, moves[1].0, moves[1].1, moves[1].2));
    let map_camera = tiled_map_camera(&map_id).expect("map camera");
    close_camera(map_camera, [seeded[0], seeded[1], seeded[2] * zoom_ratio]);
    let mut map_input = InputState::default();
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(&map_id, "touch.map", down[0].0, Some((bounds, moves[0].1, moves[0].2)), &mut map_input), Ok(true));
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut map_input).is_empty(), "the first lift is silent");
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(&map_id, "touch.map", down[1].0, Some((bounds, moves[1].1, moves[1].2)), &mut map_input), Ok(true));
    let map_actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut map_input);
    assert_eq!(map_actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&map_frame.owner, bounds, ui_render::PointerId(3), 400.0, 300.0, &modifiers));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&map_id), "the successor touch is a fresh single-pointer lane");
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(&map_id, "touch.map", ui_render::PointerId(3), None, &mut map_input), Ok(true));
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut map_input).is_empty());

    let mut board_scene = board2d_scene("touch-board-host", &fixture);
    board_scene.controller_id = "touch.board".into();
    let board_frame = paint_retained_surface_scene(&board_scene, bounds);
    let board_id = board_frame.owner.host_id.clone();
    let board_window = board_frame.owner.window_id.clone();
    assert!(board2d_set_camera_silent(&board_id, [0.0, 0.0, 1.0]));
    let mut board_input = InputState::default();
    assert_eq!(semio_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_down_into(&board_id, bounds, down[0].0, down[0].1, down[0].2, &mut board_input), Ok(false));
    semio_renderer_wgpu_scene_text_vfs::board2d_pointer_down_into(&board_id, bounds, down[0].1, down[0].2, 0, false, false, &mut board_input).expect("an idle board takes the press");
    assert!(semio_renderer_wgpu_scene_text_vfs::board2d_drag_active(&board_id));
    assert_eq!(semio_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_down_into(&board_id, bounds, down[1].0, down[1].1, down[1].2, &mut board_input), Ok(true));
    assert!(!semio_renderer_wgpu_scene_text_vfs::board2d_drag_active(&board_id), "board transfer cancels the area-select lane and synthesizes its one pointer-up");
    assert!(semio_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_move(&board_id, bounds, moves[0].0, moves[0].1, moves[0].2));
    assert!(semio_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_move(&board_id, bounds, moves[1].0, moves[1].1, moves[1].2));
    close_camera(board2d_camera(&board_id).expect("board camera"), [0.0, 0.0, 2.0]);
    assert_eq!(semio_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_up_into(&board_id, "touch.board", down[0].0, &mut board_input), Ok(true));
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut board_input).is_empty(), "the first board lift is silent");
    assert_eq!(semio_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_up_into(&board_id, "touch.board", down[1].0, &mut board_input), Ok(true));
    let board_actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut board_input);
    assert_eq!(board_actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
    assert_eq!(semio_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_down_into(&board_id, bounds, ui_render::PointerId(3), 400.0, 300.0, &mut board_input), Ok(false), "the successor board touch is fresh");

    close_retained_surface_fixture(&map_window);
    close_retained_surface_fixture(&board_window);
}

#[test]
fn tiled_map_single_touch_pans_without_selecting_and_settles_once() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/👆️tiled-map-touch/🔣️.json")).expect("single-touch fixture");
    let fixture = fixture();
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let controller = "touch.map";
    let frame = paint_retained_surface_scene(&tiled_map_scene("single-touch-map", &fixture), bounds);
    let owner = frame.owner;
    let host_id = &owner.host_id;
    let drag = &contract["drag"];
    let point = |p: &Value| (p["x"].as_f64().unwrap() as f32, p["y"].as_f64().unwrap() as f32);
    let camera = |pose: &Value| [pose["x"].as_f64().unwrap() / 1000.0, pose["y"].as_f64().unwrap() / 1000.0, pose["zoom"].as_f64().unwrap() * 1000.0];
    let pointer = ui_render::PointerId(1);
    let mut input = InputState::default();
    let modifiers = Default::default();
    let picked = (8..600).step_by(4).find_map(|y| (8..800).step_by(4).find_map(|x| {
        let hit = with_map_host(host_id, |host| host.hit_test_feature_json(f64::from(x), f64::from(y)))?;
        (hit != "null").then_some((x as f32, y as f32))
    })).expect("fixture feature has a physical hit point");
    let (start_x, start_y) = point(&contract["tap"]["start"]);
    let (end_x, end_y) = point(&contract["tap"]["end"]);
    let tap_camera = tiled_map_camera(host_id).unwrap();
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&owner, bounds, pointer, picked.0 + start_x - end_x, picked.1 + start_y - end_y, &modifiers));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(host_id, bounds, pointer, picked.0, picked.1));
    close_camera(tiled_map_camera(host_id).unwrap(), tap_camera);
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(host_id, controller, pointer, Some((bounds, picked.0, picked.1)), &mut input), Ok(true));
    let tap = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(tap.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["interactionSelect"]);

    for terminal in ["up", "cancel", "return"] {
        assert!(tiled_map_set_camera_silent(host_id, camera(&drag["initialCamera"])));
        let (x, y) = point(&drag["start"]);
        assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&owner, bounds, pointer, x, y, &modifiers));
        let (x, y) = point(&drag["move"]);
        assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(host_id, bounds, pointer, x, y));
        close_camera(tiled_map_camera(host_id).unwrap(), camera(&drag["moveCamera"]));
        assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
        let (x, y) = point(&drag[if terminal == "return" { "start" } else { "end" }]);
        let release = (terminal != "cancel").then_some((bounds, x, y));
        assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(host_id, controller, pointer, release, &mut input), Ok(true));
        let expected = match terminal {
            "up" => "finalCamera",
            "cancel" => "moveCamera",
            _ => "initialCamera",
        };
        close_camera(tiled_map_camera(host_id).unwrap(), camera(&drag[expected]));
        let actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
        assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
        assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(host_id, controller, pointer, release, &mut input), Ok(false));
    }

    let (x, y) = point(&drag["start"]);
    assert!(tiled_map_set_camera_silent(host_id, camera(&drag["initialCamera"])));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&owner, bounds, pointer, x, y, &modifiers));
    let (x, y) = point(&drag["move"]);
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(host_id, bounds, pointer, x, y));
    let second = ui_render::PointerId(2);
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&owner, bounds, second, x + 40.0, y, &modifiers));
    close_camera(tiled_map_camera(host_id).unwrap(), camera(&drag["moveCamera"]));
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(host_id, controller, pointer, Some((bounds, x, y)), &mut input), Ok(true));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(host_id, bounds, second, x + 80.0, y));
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(host_id, controller, second, None, &mut input), Ok(true));
    let pinch = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(pinch.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
    close_camera(tiled_map_camera(host_id).unwrap(), camera(&drag["moveCamera"]));

    let _ = take_engine_surface_registrations();
    close_retained_surface_fixture(&owner.window_id);
}

fn close_camera(actual: [f64; 3], expected: [f64; 3]) {
    for axis in 0..3 {
        assert!((actual[axis] - expected[axis]).abs() < 1.0e-8, "camera axis {axis}: expected {}, got {}", expected[axis], actual[axis]);
    }
}

#[test]
fn tiled_map_hover_leave_publishes_one_empty_owner_transition() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json")).expect("scene cancellation contract");
    let fixture = fixture();
    let surface_id = contract["tiledMap"]["owner"]["surfaceId"].as_str().expect("map surface id");
    let window_id = contract["tiledMap"]["owner"]["windowId"].as_str().expect("map window id");
    let controller_id = "map.hover.fixture";
    let mut scene = tiled_map_scene(surface_id, &fixture);
    scene.controller_id = controller_id.into();
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    drop_engine_surface(surface_id);
    assert!(sync_tiled_map_scene(&scene, window_id, bounds, &Theme::default()));
    let point = (0..600)
        .step_by(4)
        .find_map(|y| (0..800).step_by(4).find_map(|x| with_map_host(surface_id, |host| host.hit_test_feature_json(x as f64, y as f64)).filter(|hit| hit != "null").map(|_| (x as f32, y as f32))))
        .expect("fixture feature has a physical hit point");
    let mut input = InputState::<ActionDescriptor>::default();
    let owner = map_fixture_owner(window_id, surface_id);
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_move_into(&owner, controller_id, bounds, point.0, point.1, false, &mut input), Ok(true));
    let entered = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(entered.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["interactionHover"]);
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_leave_into(surface_id, surface_id, controller_id, &mut input), Ok(true));
    let left = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(left.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["interactionHover"]);
    let args: Value = serde_json::from_str(&semio_framework_pack_json::from_dsl_value(left[0].args.as_ref().expect("hover clear args")).to_string()).expect("hover clear args decode");
    assert_eq!(args["targets"], contract["tiledMap"]["hoverLeave"]["emptyTargets"]);
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_leave_into(surface_id, surface_id, controller_id, &mut input), Ok(false));
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "a null witness cannot publish a second leave");
    assert!(!semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(surface_id), "hover retirement cannot manufacture a gesture");
    drop_engine_surface(surface_id);
}
