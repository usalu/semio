use super::*;

fn shell() -> ShellState {
    ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
}

fn tutorial_runtime(duration_ms: u64, mode: TutorialMode, playhead_ms: f64, rate: f32, last_tick_wall_ms: f64) -> TutorialRuntime {
    TutorialRuntime {
        definition: semio_framework::onboarding::TutorialDefinition {
            id: "clock".into(),
            title: LocalizedLabel::data("Clock"),
            description: None,
            duration_ms,
            chapters: Vec::new(),
            base: semio_framework::onboarding::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::onboarding::TutorialUiSnapshot::default(), cameras: Vec::new() },
            tracks: semio_framework::onboarding::TutorialTracks::default(),
            recorded_at: None,
        },
        mode,
        playhead_ms,
        rate,
        applied_ms: playhead_ms,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        last_tick_wall_ms,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: last_tick_wall_ms,
    }
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
    let orbit = semio_framework_ui_scene_camera::OrbitController { target: ui_wgpu::wgpu::Vec3::new(1.0, 2.0, 3.0), distance: 10.0, yaw: 0.4, pitch: 0.2, fov_y: 45.0_f32.to_radians(), ..Default::default() };
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
    assert!(tutorial_camera_to_orbit(&semio_framework::onboarding::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }).is_none());
}

#[test]
fn camera_pose_close_only_compares_matching_kinds() {
    let orbit_a = semio_framework::onboarding::TutorialCameraState::Orbit { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(45.0) };
    let orbit_b = semio_framework::onboarding::TutorialCameraState::Orbit { position: [0.0001, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(45.0) };
    let canvas = semio_framework::onboarding::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 };
    assert!(tutorial_camera_pose_close(&orbit_a, &orbit_b, 0.01));
    assert!(!tutorial_camera_pose_close(&orbit_a, &canvas, 0.01));
}
//#endregion CameraConversionTests

//#region UiSnapshotTests
#[test]
fn ui_snapshot_round_trips_panel_tabs_and_focus() {
    let mut state = shell();
    state.windows.active_window_id = Some("window-a".into());
    for anchor in PanelAnchor::ALL {
        let tab = format!("panel.{}", anchor.as_str());
        *state.dock.dock_tabs.tabs_mut(anchor) = vec![DockTabNode::leaf(&tab, &tab, "circle-dot", 0)];
        state.dock.toggle_anchor_tab(anchor, &tab);
    }
    state.input.interaction_selection.insert("world".into(), semio_framework::DomainSelection { granularity: "object".into(), ids: vec!["solid-a".into(), "solid-b".into()], anchor_id: Some("solid-a".into()) });
    state.input.interaction_selection.insert("graph".into(), semio_framework::DomainSelection { granularity: "node".into(), ids: vec!["node-a".into()], anchor_id: None });
    state.input.set_canonical_tree_open("catalogue.solids", true);
    state.input.set_canonical_tree_open("settings.appearance", true);
    state.commands.set_search_open(false, &mut state.input);

    let snapshot = tutorial_capture_ui_snapshot(&WindowActionsCtx::of(&mut state));
    assert_eq!(snapshot.focused_window_id.as_deref(), Some("window-a"));
    assert_eq!(snapshot.active_panel_tab_by_group.len(), PanelAnchor::ALL.len());
    assert_eq!(snapshot.interaction_selection.len(), 2);
    assert_eq!(snapshot.expanded_tree_ids, vec!["catalogue.solids".to_string(), "settings.appearance".to_string()]);
    assert!(!snapshot.command_panel_open);

    let mut fresh = shell();
    for anchor in PanelAnchor::ALL {
        let tab = format!("panel.{}", anchor.as_str());
        *fresh.dock.dock_tabs.tabs_mut(anchor) = vec![DockTabNode::leaf(&tab, &tab, "circle-dot", 0)];
    }
    tutorial_apply_ui_snapshot(&mut WindowActionsCtx::of(&mut fresh), &snapshot);
    assert_eq!(fresh.windows.active_window_id.as_deref(), Some("window-a"));
    for anchor in PanelAnchor::ALL {
        let tab = format!("panel.{}", anchor.as_str());
        assert!(fresh.dock.anchor_open(anchor), "{} is restored", anchor.as_str());
        assert_eq!(fresh.dock.anchor_state(anchor).active_tab(), Some(tab.as_str()));
    }
    assert_eq!(fresh.input.interaction_selection, snapshot.interaction_selection);
    assert!(fresh.input.canonical_tree_open("catalogue.solids", false));
    assert!(fresh.input.canonical_tree_open("settings.appearance", false));
}

#[test]
fn ui_snapshot_absent_panel_tabs_close_the_panel() {
    let mut state = shell();
    *state.dock.dock_tabs.tabs_mut(PanelAnchor::BottomLeft) = vec![DockTabNode::leaf("tab-x", "Tab X", "circle-dot", 0)];
    *state.dock.dock_tabs.tabs_mut(PanelAnchor::BottomRight) = vec![DockTabNode::leaf("tab-y", "Tab Y", "circle-dot", 0)];
    state.dock.toggle_anchor_tab(PanelAnchor::BottomLeft, "tab-x");
    state.dock.toggle_anchor_tab(PanelAnchor::BottomRight, "tab-y");
    let empty_snapshot = semio_framework::onboarding::TutorialUiSnapshot::default();
    tutorial_apply_ui_snapshot(&mut WindowActionsCtx::of(&mut state), &empty_snapshot);
    assert!(!state.dock.left_panel_open());
    assert!(!state.dock.right_panel_open());
}

#[test]
fn ui_change_applies_live_against_shell_state() {
    let mut state = shell();
    let change = semio_framework::onboarding::TutorialUiChange::ActiveUtility { window_id: "window-a".into(), utility_id: Some("select".into()) };
    tutorial_apply_ui_change_to_shell(&mut WindowActionsCtx::of(&mut state), &change);
    assert_eq!(state.windows.active_utility_by_window.get("window-a").map(String::as_str), Some("select"));
}

#[test]
fn ui_deltas_close_selection_tree_panel_and_command_state() {
    let mut state = shell();
    *state.dock.dock_tabs.tabs_mut(PanelAnchor::BottomMiddle) = vec![DockTabNode::leaf("panel.bottom-middle", "Command", "circle-dot", 0)];
    state.dock.toggle_anchor_tab(PanelAnchor::BottomMiddle, "panel.bottom-middle");
    state.input.interaction_selection.insert("world".into(), semio_framework::DomainSelection { granularity: "object".into(), ids: vec!["solid-a".into()], anchor_id: None });
    state.input.set_canonical_tree_open("catalogue.solids", true);
    state.commands.set_search_open(true, &mut state.input);
    for change in [
        semio_framework::onboarding::TutorialUiChange::Selection { domain_id: "world".into(), granularity: "object".into(), ids: Vec::new() },
        semio_framework::onboarding::TutorialUiChange::TreeExpansion { id: "catalogue.solids".into(), expanded: false },
        semio_framework::onboarding::TutorialUiChange::PanelTab { group: "bottom-middle".into(), tab_id: None },
        semio_framework::onboarding::TutorialUiChange::CommandPanel { open: false },
    ] {
        tutorial_apply_ui_change_to_shell(&mut WindowActionsCtx::of(&mut state), &change);
    }
    assert!(!state.input.interaction_selection.contains_key("world"));
    assert!(!state.input.canonical_tree_open("catalogue.solids", true));
    assert!(!state.dock.anchor_open(PanelAnchor::BottomMiddle));
    assert!(!state.commands.search_open);
    assert_eq!(state.input.overlay_state, OverlayState::None);
}

#[test]
fn normal_interaction_completion_updates_typed_selection_projection() {
    let mut state = shell();
    state.input.observe_interaction_output(&DslValue::Object(vec![(
        "interactionView".into(),
        DslValue::Object(vec![(
            "selection".into(),
            DslValue::Object(vec![
                (
                    "world".into(),
                    DslValue::Object(vec![
                        ("granularity".into(), DslValue::String("object".into())),
                        ("ids".into(), DslValue::Array(vec![DslValue::String("solid-a".into()), DslValue::String("solid-b".into())])),
                        ("anchorId".into(), DslValue::String("solid-a".into())),
                    ]),
                ),
                ("graph".into(), DslValue::Object(vec![("granularity".into(), DslValue::String("node".into())), ("ids".into(), DslValue::Array(vec![DslValue::String("node-a".into())]))])),
            ]),
        )]),
    )]));
    assert_eq!(state.input.interaction_selection.len(), 2);
    assert_eq!(state.input.interaction_selection["world"].ids, vec!["solid-a".to_string(), "solid-b".to_string()]);
    assert_eq!(state.input.interaction_selection["graph"].granularity, "node");
}
//#endregion UiSnapshotTests

//#region GesturePointTests
#[test]
fn gesture_point_resolves_screen_and_normalized() {
    let mut state = shell();
    state.chrome.screen_w = 1000.0;
    state.chrome.screen_h = 500.0;
    assert_eq!(tutorial_resolve_gesture_point(&state.chrome, &state.surfaces, &state.dock, &semio_framework::onboarding::IntroductionPoint::Screen { x: 42.0, y: 7.0 }), Some((42.0, 7.0)));
    assert_eq!(tutorial_resolve_gesture_point(&state.chrome, &state.surfaces, &state.dock, &semio_framework::onboarding::IntroductionPoint::ScreenNormalized { x: 0.5, y: 0.25 }), Some((500.0, 125.0)));
}

#[test]
fn gesture_point_resolves_window_local() {
    let mut state = shell();
    state.dock.window_content_rects.insert("window-a".into(), Rect::new(100.0, 50.0, 400.0, 300.0));
    assert_eq!(tutorial_resolve_gesture_point(&state.chrome, &state.surfaces, &state.dock, &semio_framework::onboarding::IntroductionPoint::Window { id: "window-a".into(), x: 10.0, y: 20.0 }), Some((110.0, 70.0)));
    assert_eq!(tutorial_resolve_gesture_point(&state.chrome, &state.surfaces, &state.dock, &semio_framework::onboarding::IntroductionPoint::WindowNormalized { id: "window-a".into(), x: 0.5, y: 0.5 }), Some((300.0, 200.0)));
    assert_eq!(tutorial_resolve_gesture_point(&state.chrome, &state.surfaces, &state.dock, &semio_framework::onboarding::IntroductionPoint::Window { id: "missing".into(), x: 0.0, y: 0.0 }), None);
}

#[test]
fn gesture_point_rejects_detached_semantic_surface() {
    let state = shell();
    assert_eq!(tutorial_resolve_gesture_point(&state.chrome, &state.surfaces, &state.dock, &semio_framework::onboarding::IntroductionPoint::Scene { id: "detached".into(), position: [0.0, 0.0, 0.0] }), None);
}
//#endregion GesturePointTests

//#region LifecycleTests
#[test]
fn tutorial_deadline_self_schedules_playback_recording_convergence_and_stops_at_terminal_states() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🎥️tutorial-bridge/🔣️.json")).expect("tutorial fixture");
    let clock = &fixture["clock"];
    for row in clock["deadlines"].as_array().expect("deadline rows") {
        let mode = match row["mode"].as_str().expect("mode") {
            "playing" => TutorialMode::Playing,
            "paused" => TutorialMode::Paused,
            "recording" => TutorialMode::Recording,
            "deviated" => TutorialMode::Deviated,
            value => panic!("unknown tutorial mode {value}"),
        };
        let mut state = shell();
        let mut runtime = tutorial_runtime(row["durationMs"].as_u64().expect("duration"), mode, row["playheadMs"].as_f64().expect("playhead"), row["rate"].as_f64().expect("rate") as f32, row["lastTickWallMs"].as_f64().expect("last tick"));
        if let Some(converge_end) = row["convergeEndWallMs"].as_f64() {
            let camera = semio_framework::onboarding::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 };
            runtime.converge.insert("canvas".into(), TutorialCameraConverge { from: camera.clone(), to: camera, started_wall_ms: converge_end - semio_framework::onboarding::TUTORIAL_CONVERGE_MS as f64 });
        }
        state.tutorial.tutorial = Some(runtime);
        let actual = state.tutorial.next_tutorial_deadline_ms(1_000.0);
        match row["expectedWallMs"].as_f64() {
            Some(expected) => assert!((actual.expect("deadline") - expected).abs() < 0.000_001, "{}", row["id"]),
            None => assert!(actual.is_none(), "{}", row["id"]),
        }
    }

    let mut playing = shell();
    playing.tutorial.tutorial = Some(tutorial_runtime(clock["durationMs"].as_u64().expect("clock duration"), TutorialMode::Playing, 0.0, clock["rate"].as_f64().expect("clock rate") as f32, clock["wallFramesMs"][0].as_f64().expect("first frame")));
    for (index, wall_ms) in clock["wallFramesMs"].as_array().expect("wall frames").iter().enumerate() {
        WindowActionsCtx::of(&mut playing).tutorial_tick(wall_ms.as_f64().expect("wall frame"));
        let runtime = playing.tutorial.tutorial.as_ref().expect("playing runtime");
        assert!((runtime.playhead_ms - clock["expectedPlayheadMs"][index].as_f64().expect("expected playhead")).abs() < 0.000_001);
        let expected_playing = clock["expectedPlaying"][index].as_bool().expect("expected playing");
        assert_eq!(runtime.mode == TutorialMode::Playing, expected_playing);
        assert_eq!(playing.tutorial.next_tutorial_deadline_ms(wall_ms.as_f64().expect("wall frame")).is_some(), expected_playing, "a live tick rearms independently of frame acceptance");
    }
    assert!(playing.tutorial.next_tutorial_deadline_ms(clock["wallFramesMs"][2].as_f64().expect("terminal frame")).is_none());

    let mut recording = shell();
    recording.tutorial.tutorial = Some(tutorial_runtime(10, TutorialMode::Recording, 10.0, 5.0, 1_000.0));
    WindowActionsCtx::of(&mut recording).tutorial_tick(1_000.0 + clock["frameIntervalMs"].as_f64().expect("frame interval"));
    let recorded = recording.tutorial.tutorial.as_ref().expect("recording runtime");
    assert!((recorded.playhead_ms - (10.0 + clock["frameIntervalMs"].as_f64().expect("frame interval"))).abs() < 0.000_001);
    assert!(recording.tutorial.next_tutorial_deadline_ms(recorded.last_tick_wall_ms).is_some());

    let converge_row = clock["deadlines"].as_array().expect("deadline rows").iter().find(|row| row["id"] == "converging-paused").expect("convergence row");
    let converge_end = converge_row["convergeEndWallMs"].as_f64().expect("convergence end");
    let mut converging = shell();
    let mut runtime = tutorial_runtime(50, TutorialMode::Paused, 10.0, 1.0, 1_000.0);
    let camera = semio_framework::onboarding::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 };
    runtime.converge.insert("canvas".into(), TutorialCameraConverge { from: camera.clone(), to: camera, started_wall_ms: converge_end - semio_framework::onboarding::TUTORIAL_CONVERGE_MS as f64 });
    converging.tutorial.tutorial = Some(runtime);
    WindowActionsCtx::of(&mut converging).tutorial_tick(converge_end);
    assert!(converging.tutorial.tutorial.as_ref().expect("converging runtime").converge.is_empty());
    assert!(converging.tutorial.next_tutorial_deadline_ms(converge_end).is_none());

    let merge = &clock["merge"];
    let mut merged = shell();
    merged.tutorial.tutorial = Some(tutorial_runtime(50, TutorialMode::Playing, 10.0, 2.0, merge["wallNowMs"].as_f64().expect("wall now")));
    merged.chrome.chrome_build.tooltip_hover = Some(ChromeTooltipHover { control_id: "tutorial-clock".into(), anchor_x: 0.0, anchor_y: 0.0, started_ms: merge["chromeWallMs"].as_f64().expect("chrome deadline") - CHROME_TOOLTIP_DELAY_MS });
    let merged_deadline = merged.surfaces.next_chrome_deadline(merge["monotonicNowSeconds"].as_f64().expect("monotonic now"), merge["wallNowMs"].as_f64().expect("wall now"), &merged.chrome, &merged.tutorial).expect("merged deadline");
    assert!((merged_deadline - merge["expectedMonotonicSeconds"].as_f64().expect("expected monotonic deadline")).abs() < 0.000_001);

    merged.tutorial.tutorial.as_mut().expect("tutorial").mode = TutorialMode::Paused;
    merged.chrome.chrome_build.tooltip_hover = None;
    assert!(merged.surfaces.next_chrome_deadline(40.0, 1_000.0, &merged.chrome, &merged.tutorial).is_none());
    WindowActionsCtx::of(&mut merged).tutorial_stop();
    assert!(merged.tutorial.tutorial.is_none());
    assert!(merged.tutorial.next_tutorial_deadline_ms(1_000.0).is_none());
}

#[test]
fn seek_clamps_to_duration_and_updates_playhead() {
    let mut state = shell();
    let definition = semio_framework::onboarding::TutorialDefinition {
        id: "t1".into(),
        title: LocalizedLabel::data("Test"),
        description: None,
        duration_ms: 1000,
        chapters: Vec::new(),
        base: semio_framework::onboarding::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::onboarding::TutorialUiSnapshot::default(), cameras: Vec::new() },
        tracks: semio_framework::onboarding::TutorialTracks::default(),
        recorded_at: None,
    };
    state.tutorial.tutorial = Some(TutorialRuntime {
        definition,
        mode: TutorialMode::Paused,
        playhead_ms: 0.0,
        rate: 1.0,
        applied_ms: 0.0,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        last_tick_wall_ms: 0.0,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: 0.0,
    });
    WindowActionsCtx::of(&mut state).tutorial_seek(5000.0);
    assert_eq!(state.tutorial.tutorial.as_ref().unwrap().playhead_ms, 1000.0);
}

#[test]
fn note_real_dispatch_deviates_a_playing_tutorial() {
    let mut state = shell();
    let definition = semio_framework::onboarding::TutorialDefinition {
        id: "t1".into(),
        title: LocalizedLabel::data("Test"),
        description: None,
        duration_ms: 1000,
        chapters: Vec::new(),
        base: semio_framework::onboarding::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::onboarding::TutorialUiSnapshot::default(), cameras: Vec::new() },
        tracks: semio_framework::onboarding::TutorialTracks::default(),
        recorded_at: None,
    };
    state.tutorial.tutorial = Some(TutorialRuntime {
        definition,
        mode: TutorialMode::Playing,
        playhead_ms: 0.0,
        rate: 1.0,
        applied_ms: 0.0,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        last_tick_wall_ms: 0.0,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: 0.0,
    });
    state.tutorial.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "someAction".into(), args: None });
    assert_eq!(state.tutorial.tutorial.as_ref().unwrap().mode, TutorialMode::Deviated);
}

#[test]
fn recorder_records_annotational_events_but_skips_set_camera() {
    let mut state = shell();
    let definition = semio_framework::onboarding::TutorialDefinition {
        id: "rec".into(),
        title: LocalizedLabel::data("Recording"),
        description: None,
        duration_ms: 0,
        chapters: Vec::new(),
        base: semio_framework::onboarding::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::onboarding::TutorialUiSnapshot::default(), cameras: Vec::new() },
        tracks: semio_framework::onboarding::TutorialTracks::default(),
        recorded_at: None,
    };
    state.tutorial.tutorial = Some(TutorialRuntime {
        definition,
        mode: TutorialMode::Recording,
        playhead_ms: 250.0,
        rate: 1.0,
        applied_ms: 0.0,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        last_tick_wall_ms: 0.0,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: 0.0,
    });
    state.tutorial.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "setCamera".into(), args: None });
    state.tutorial.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "doSomething".into(), args: None });
    let events = &state.tutorial.tutorial.as_ref().unwrap().definition.tracks.events;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].at, 250);
}
//#endregion LifecycleTests
