
use super::*;

fn shell() -> ShellState {
    ShellState::new(Vec::new(), String::new())
}

//#region PureMathTests
#[test]
fn advance_playhead_scales_by_rate() {
    assert_eq!(tutorial_advance_playhead(1000.0, 500.0, 2.0), 2000.0);
    assert_eq!(tutorial_advance_playhead(1000.0, 500.0, 0.5), 1250.0);
    assert_eq!(tutorial_advance_playhead(1000.0, 0.0, 1.0), 1000.0);
}

#[test]
fn scrub_progress_clamps_and_avoids_divide_by_zero() {
    assert_eq!(tutorial_scrub_progress(0.0, 0), 0.0);
    assert_eq!(tutorial_scrub_progress(500.0, 1000), 0.5);
    assert_eq!(tutorial_scrub_progress(5000.0, 1000), 1.0);
    assert_eq!(tutorial_scrub_progress(-5.0, 1000), 0.0);
}

#[test]
fn scrub_target_ms_maps_pointer_position_to_playhead() {
    let track = Rect::new(100.0, 0.0, 200.0, 20.0);
    assert_eq!(tutorial_scrub_target_ms(100.0, track, 1000), 0.0);
    assert_eq!(tutorial_scrub_target_ms(200.0, track, 1000), 500.0);
    assert_eq!(tutorial_scrub_target_ms(300.0, track, 1000), 1000.0);
    assert_eq!(tutorial_scrub_target_ms(50.0, track, 1000), 0.0);
    assert_eq!(tutorial_scrub_target_ms(9999.0, track, 1000), 1000.0);
}
//#endregion PureMathTests

//#region CameraConversionTests
#[test]
fn orbit_camera_round_trips_through_tutorial_camera_state() {
    let orbit = ui_wgpu::wgpu::OrbitController { target: ui_wgpu::wgpu::Vec3::new(1.0, 2.0, 3.0), distance: 10.0, yaw: 0.4, pitch: 0.2, fov_y: 45.0_f32.to_radians() };
    let tutorial_camera = orbit_to_tutorial_camera(&orbit);
    let round_tripped = tutorial_camera_to_orbit(&tutorial_camera).expect("orbit camera state converts back");
    let original_pose = orbit.to_camera();
    let round_tripped_pose = round_tripped.to_camera();
    assert!((original_pose.position.x - round_tripped_pose.position.x).abs() < 0.01);
    assert!((original_pose.position.y - round_tripped_pose.position.y).abs() < 0.01);
    assert!((original_pose.position.z - round_tripped_pose.position.z).abs() < 0.01);
    assert!((original_pose.fov_y - round_tripped_pose.fov_y).abs() < 0.001);
}

#[test]
fn canvas_camera_state_has_no_orbit_equivalent() {
    assert!(tutorial_camera_to_orbit(&semio_framework::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }).is_none());
}

#[test]
fn camera_pose_close_only_compares_matching_kinds() {
    let orbit_a = semio_framework::TutorialCameraState::Orbit { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(45.0) };
    let orbit_b = semio_framework::TutorialCameraState::Orbit { position: [0.0001, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(45.0) };
    let canvas = semio_framework::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 };
    assert!(tutorial_camera_pose_close(&orbit_a, &orbit_b, 0.01));
    assert!(!tutorial_camera_pose_close(&orbit_a, &canvas, 0.01));
}
//#endregion CameraConversionTests

//#region UiSnapshotTests
#[test]
fn ui_snapshot_round_trips_panel_tabs_and_focus() {
    let mut state = shell();
    state.active_window_id = Some("window-a".into());
    state.left_panel_open = true;
    state.active_left_kind = LeftPanelKind::Display;
    state.active_left_tab = Some("tab-x".into());
    state.right_panel_open = true;
    state.active_right_kind = RightPanelKind::Settings;
    state.active_right_tab = Some("tab-y".into());

    let snapshot = tutorial_capture_ui_snapshot(&state);
    assert_eq!(snapshot.focused_window_id.as_deref(), Some("window-a"));
    assert_eq!(snapshot.active_panel_tab_by_group.get("display").map(String::as_str), Some("tab-x"));
    assert_eq!(snapshot.active_panel_tab_by_group.get("settings").map(String::as_str), Some("tab-y"));

    let mut fresh = shell();
    tutorial_apply_ui_snapshot(&mut fresh, &snapshot);
    assert_eq!(fresh.active_window_id.as_deref(), Some("window-a"));
    assert!(fresh.left_panel_open);
    assert_eq!(fresh.active_left_kind, LeftPanelKind::Display);
    assert_eq!(fresh.active_left_tab.as_deref(), Some("tab-x"));
    assert!(fresh.right_panel_open);
    assert_eq!(fresh.active_right_kind, RightPanelKind::Settings);
    assert_eq!(fresh.active_right_tab.as_deref(), Some("tab-y"));
}

#[test]
fn ui_snapshot_absent_panel_tabs_close_the_panel() {
    let mut state = shell();
    state.left_panel_open = true;
    state.active_left_tab = Some("tab-x".into());
    state.right_panel_open = true;
    state.active_right_tab = Some("tab-y".into());
    let empty_snapshot = semio_framework::TutorialUiSnapshot::default();
    tutorial_apply_ui_snapshot(&mut state, &empty_snapshot);
    assert!(!state.left_panel_open);
    assert!(!state.right_panel_open);
}

#[test]
fn ui_change_applies_live_against_shell_state() {
    let mut state = shell();
    let change = semio_framework::TutorialUiChange::ActiveUtility { window_id: "window-a".into(), utility_id: Some("select".into()) };
    tutorial_apply_ui_change_to_shell(&mut state, &change);
    assert_eq!(state.active_utility_by_window.get("window-a").map(String::as_str), Some("select"));
}
//#endregion UiSnapshotTests

//#region GesturePointTests
#[test]
fn gesture_point_resolves_screen_and_normalized() {
    let mut state = shell();
    state.screen_w = 1000.0;
    state.screen_h = 500.0;
    assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Screen { x: 42.0, y: 7.0 }), Some((42.0, 7.0)));
    assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::ScreenNormalized { x: 0.5, y: 0.25 }), Some((500.0, 125.0)));
}

#[test]
fn gesture_point_resolves_window_local() {
    let mut state = shell();
    state.window_content_rects.insert("window-a".into(), Rect::new(100.0, 50.0, 400.0, 300.0));
    assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Window { id: "window-a".into(), x: 10.0, y: 20.0 }), Some((110.0, 70.0)));
    assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::WindowNormalized { id: "window-a".into(), x: 0.5, y: 0.5 }), Some((300.0, 200.0)));
    assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Window { id: "missing".into(), x: 0.0, y: 0.0 }), None);
}

#[test]
fn gesture_point_scopes_out_scene_and_entity_kinds() {
    let state = shell();
    assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Scene { id: "w".into(), position: [0.0, 0.0, 0.0] }), None);
    let point = semio_framework_async::block_on(semio_framework::IntroductionPoint::any_entity("w", "vortex"));
    assert_eq!(tutorial_resolve_gesture_point(&state, &point), None);
}
//#endregion GesturePointTests

//#region LifecycleTests
#[test]
fn seek_clamps_to_duration_and_updates_playhead() {
    let mut state = shell();
    let definition = semio_framework::TutorialDefinition {
        id: "t1".into(),
        title: LocalizedLabel::data("Test"),
        description: None,
        duration_ms: 1000,
        chapters: Vec::new(),
        base: semio_framework::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::TutorialUiSnapshot::default(), cameras: Vec::new() },
        tracks: semio_framework::TutorialTracks::default(),
        recorded_at: None,
    };
    state.tutorial = Some(TutorialRuntime {
        definition,
        mode: TutorialMode::Paused,
        playhead_ms: 0.0,
        rate: 1.0,
        applied_ms: 0.0,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::TutorialUiSnapshot::default(),
        last_tick_wall_ms: 0.0,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: 0.0,
    });
    state.tutorial_seek(5000.0);
    assert_eq!(state.tutorial.as_ref().unwrap().playhead_ms, 1000.0);
}

#[test]
fn note_real_dispatch_deviates_a_playing_tutorial() {
    let mut state = shell();
    let definition = semio_framework::TutorialDefinition {
        id: "t1".into(),
        title: LocalizedLabel::data("Test"),
        description: None,
        duration_ms: 1000,
        chapters: Vec::new(),
        base: semio_framework::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::TutorialUiSnapshot::default(), cameras: Vec::new() },
        tracks: semio_framework::TutorialTracks::default(),
        recorded_at: None,
    };
    state.tutorial = Some(TutorialRuntime {
        definition,
        mode: TutorialMode::Playing,
        playhead_ms: 0.0,
        rate: 1.0,
        applied_ms: 0.0,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::TutorialUiSnapshot::default(),
        last_tick_wall_ms: 0.0,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: 0.0,
    });
    state.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "someAction".into(), args: None });
    assert_eq!(state.tutorial.as_ref().unwrap().mode, TutorialMode::Deviated);
}

#[test]
fn recorder_records_annotational_events_but_skips_set_camera() {
    let mut state = shell();
    let definition = semio_framework::TutorialDefinition {
        id: "rec".into(),
        title: LocalizedLabel::data("Recording"),
        description: None,
        duration_ms: 0,
        chapters: Vec::new(),
        base: semio_framework::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::TutorialUiSnapshot::default(), cameras: Vec::new() },
        tracks: semio_framework::TutorialTracks::default(),
        recorded_at: None,
    };
    state.tutorial = Some(TutorialRuntime {
        definition,
        mode: TutorialMode::Recording,
        playhead_ms: 250.0,
        rate: 1.0,
        applied_ms: 0.0,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::TutorialUiSnapshot::default(),
        last_tick_wall_ms: 0.0,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: 0.0,
    });
    state.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "setCamera".into(), args: None });
    state.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "doSomething".into(), args: None });
    let events = &state.tutorial.as_ref().unwrap().definition.tracks.events;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].at, 250);
}
//#endregion LifecycleTests
