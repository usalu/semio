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
    assert!(!semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&map_id, bounds, down[0].0, down[0].1, down[0].2));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&map_frame.owner, "touch.map", bounds, down[0].1, down[0].2, 0, false, false, "rectangle", &mut InputState::default()), Ok(true));
    assert!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_drag_active(&map_id));
    assert!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&map_id, bounds, down[1].0, down[1].1, down[1].2));
    assert!(!semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_drag_active(&map_id), "map transfer drops marquee without a synthetic pointer-up");
    assert!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(&map_id, bounds, moves[0].0, moves[0].1, moves[0].2));
    assert!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(&map_id, bounds, moves[1].0, moves[1].1, moves[1].2));
    let map_camera = tiled_map_camera(&map_id).expect("map camera");
    close_camera(map_camera, [seeded[0], seeded[1], seeded[2] * zoom_ratio]);
    let mut map_input = InputState::default();
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(&map_id, "touch.map", down[0].0, &mut map_input), Ok(true));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut map_input).is_empty(), "the first lift is silent");
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(&map_id, "touch.map", down[1].0, &mut map_input), Ok(true));
    let map_actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut map_input);
    assert_eq!(map_actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
    assert!(!semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(&map_id, bounds, ui_render::PointerId(3), 400.0, 300.0), "the successor touch is a fresh single-pointer lane");

    let mut board_scene = board2d_scene("touch-board-host", &fixture);
    board_scene.controller_id = "touch.board".into();
    let board_frame = paint_retained_surface_scene(&board_scene, bounds);
    let board_id = board_frame.owner.host_id.clone();
    let board_window = board_frame.owner.window_id.clone();
    assert!(board2d_set_camera_silent(&board_id, [0.0, 0.0, 1.0]));
    let mut board_input = InputState::default();
    assert_eq!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_down_into(&board_id, bounds, down[0].0, down[0].1, down[0].2, &mut board_input), Ok(false));
    semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_pointer_down_into(&board_id, bounds, down[0].1, down[0].2, 0, false, false, &mut board_input).expect("an idle board takes the press");
    assert!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_drag_active(&board_id));
    assert_eq!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_down_into(&board_id, bounds, down[1].0, down[1].1, down[1].2, &mut board_input), Ok(true));
    assert!(!semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_drag_active(&board_id), "board transfer cancels the area-select lane and synthesizes its one pointer-up");
    assert!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_move(&board_id, bounds, moves[0].0, moves[0].1, moves[0].2));
    assert!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_move(&board_id, bounds, moves[1].0, moves[1].1, moves[1].2));
    close_camera(board2d_camera(&board_id).expect("board camera"), [0.0, 0.0, 2.0]);
    assert_eq!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_up_into(&board_id, "touch.board", down[0].0, &mut board_input), Ok(true));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut board_input).is_empty(), "the first board lift is silent");
    assert_eq!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_up_into(&board_id, "touch.board", down[1].0, &mut board_input), Ok(true));
    let board_actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut board_input);
    assert_eq!(board_actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setCamera"]);
    assert_eq!(semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_down_into(&board_id, bounds, ui_render::PointerId(3), 400.0, 300.0, &mut board_input), Ok(false), "the successor board touch is fresh");

    close_retained_surface_fixture(&map_window);
    close_retained_surface_fixture(&board_window);
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
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_move_into(&owner, controller_id, bounds, point.0, point.1, false, &mut input), Ok(true));
    let entered = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(entered.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["interactionHover"]);
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_leave_into(surface_id, surface_id, controller_id, &mut input), Ok(true));
    let left = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(left.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["interactionHover"]);
    let args: Value = serde_json::from_str(&semio_framework_pack_json::from_dsl_value(left[0].args.as_ref().expect("hover clear args")).to_string()).expect("hover clear args decode");
    assert_eq!(args["targets"], contract["tiledMap"]["hoverLeave"]["emptyTargets"]);
    assert_eq!(semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_leave_into(surface_id, surface_id, controller_id, &mut input), Ok(false));
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "a null witness cannot publish a second leave");
    assert!(!semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_drag_active(surface_id), "hover retirement cannot manufacture a gesture");
    drop_engine_surface(surface_id);
}
