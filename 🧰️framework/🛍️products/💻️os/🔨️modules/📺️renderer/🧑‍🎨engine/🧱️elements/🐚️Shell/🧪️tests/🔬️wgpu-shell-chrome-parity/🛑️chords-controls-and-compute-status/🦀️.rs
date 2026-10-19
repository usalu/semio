/// 🛑️ Chords, controls and compute status: shared keybinding rows, the alt axis, the surface overlay controls and the compute status pill and pane.

//#region ⌨️Chords

fn chord_key_action(key: &str) -> semio_framework_ui_wgpu_input::KeyAction {
    match key {
        "ArrowRight" => semio_framework_ui_wgpu_input::KeyAction::ArrowRight,
        "ArrowLeft" => semio_framework_ui_wgpu_input::KeyAction::ArrowLeft,
        other => semio_framework_ui_wgpu_input::KeyAction::Char(other.to_string()),
    }
}

/// ⌨️ Both chord families now resolve through the ONE remappable table
/// (`SHELL_SHORTCUT_ROWS` → `shell_shortcut_for`), the Rust twin of React's
/// `SHELL_KEYBINDINGS`; the per-family helpers this law used to call are gone with the
/// open-coded ladder they read.
#[test]
fn every_shared_keybinding_row_routes_to_its_shell_verb_and_outranks_app_keybindings() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let apps = manifest_apps(&fixture, "procedural");
    let dialect = fixture_dialect(&fixture["dialects"]["generation3d"]);
    let mut rows = 0;
    for case in fixture["keybindings"].as_array().expect("fixture keybinding rows") {
        let event = &case["event"];
        let action = chord_key_action(event["key"].as_str().expect("fixture chord key"));
        let modifiers = PointerModifiers { shift: event["shiftKey"].as_bool().unwrap_or(false), ctrl: event["ctrlKey"].as_bool().unwrap_or(false), alt: event["altKey"].as_bool().unwrap_or(false), meta: event["metaKey"].as_bool().unwrap_or(false) };
        assert!(is_reserved_shell_chord(&action, &modifiers), "{}: a shell chord must outrank every app-declared keybinding", case["controlId"]);
        let control_id = case["controlId"].as_str().expect("fixture control id");
        if let Some(role_suffix) = control_id.strip_prefix("playground.navbar.roles.") {
            assert_eq!(shell_shortcut_for(&action, &modifiers), Some(ShellShortcut::SurfaceRole(fixture_role(role_suffix))), "{control_id}");
            let dispatches = &case["dispatches"];
            let target = role_switch_target(&apps, &dialect, fixture_role(dispatches["currentRole"].as_str().expect("fixture current role")), fixture_role(dispatches["requested"].as_str().expect("fixture requested role")));
            assert_eq!(target.map(|app| app.id.as_str()), dispatches["expectedAppId"].as_str(), "{control_id}: the chord opens the app the fixture declares");
        } else {
            let expected_step = match control_id {
                "ui.shell.mode.next" => 1,
                "ui.shell.mode.previous" => -1,
                other => panic!("fixture declares an unknown shell keybinding {other:?}"),
            };
            assert_eq!(shell_shortcut_for(&action, &modifiers), Some(ShellShortcut::ModeStep(expected_step)), "{control_id}");
        }
        rows += 1;
    }
    assert_eq!(rows, 4, "the shared fixture's four keybinding rows");
}

#[test]
fn the_alt_axis_never_swallows_a_neighbouring_chord() {
    let accelerator = PointerModifiers { shift: false, ctrl: false, alt: false, meta: true };
    let with_alt = PointerModifiers { alt: true, ..accelerator };
    let bare = PointerModifiers::default();
    assert!(!matches!(shell_shortcut_for(&semio_framework_ui_wgpu_input::KeyAction::Char("e".into()), &accelerator), Some(ShellShortcut::SurfaceRole(_))), "mod+e is not a role chord — mod+alt+e is");
    assert_eq!(shell_shortcut_for(&semio_framework_ui_wgpu_input::KeyAction::Char("v".into()), &bare), None, "a bare `v` is ordinary text");
    assert!(!matches!(shell_shortcut_for(&semio_framework_ui_wgpu_input::KeyAction::ArrowRight, &accelerator), Some(ShellShortcut::ModeStep(_))), "mod+arrowright is not a mode step — mod+alt+arrowright is");
    assert!(is_reserved_shell_chord(&semio_framework_ui_wgpu_input::KeyAction::Char("f".into()), &accelerator), "mod+f stays the find chord");
    assert!(!is_reserved_shell_chord(&semio_framework_ui_wgpu_input::KeyAction::Char("f".into()), &with_alt), "mod+alt+f belongs to whoever declares it, not to find");
    assert!(!is_reserved_shell_chord(&semio_framework_ui_wgpu_input::KeyAction::Char("f".into()), &bare), "a bare `f` is never a reserved shell accelerator — app actions such as zoomToFlow own it");
}

//#endregion ⌨️Chords

//#region 🛑️SurfaceControls

#[test]
fn the_cancel_contract_is_read_the_way_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_CONTROLS_FIXTURE).expect("🛑️ the surface-controls fixture parses");
    let mut rows = 0;
    for case in fixture["cancelContract"].as_array().expect("fixture cancel rows") {
        let affordance = world3d_cancel_affordance(case["statusJson"].as_str());
        assert_eq!(affordance.cancellable, case["expected"]["cancellable"].as_bool().expect("fixture cancellable"), "{}", case["id"]);
        assert_eq!(affordance.cancel_action, case["expected"]["cancelAction"].as_str().expect("fixture cancelAction"), "{}", case["id"]);
        assert_eq!(Value::Object(affordance.cancel_args), case["expected"]["cancelArgs"], "{}", case["id"]);
        rows += 1;
    }
    assert_eq!(rows, 11, "the fixture's eleven cancel-contract rows");
}

/// 🔤️ The fixture carries the shared control-height token so the TypeScript twin can apply the
/// very same size gate without re-deriving `UI_SPACING_COMPACT_PX × CONTROL_HEIGHT_UI_SPACING`.
#[test]
fn a_live_surface_offers_exactly_the_overlay_controls_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_CONTROLS_FIXTURE).expect("🛑️ fixture parses");
    let theme = semio_framework_os_renderer_wgpu_host_base::resolve_theme("light");
    assert!((f64::from(theme.control_height) - fixture["controlHeightPx"].as_f64().expect("fixture control height")).abs() < 1e-6, "the fixture's declared control height must stay the theme's own token");
    let mut rows = 0;
    for case in fixture["surfaceControls"].as_array().expect("fixture surface rows") {
        let bounds = |value: &Value| Rect::new(value[0].as_f64().expect("x") as f32, value[1].as_f64().expect("y") as f32, value[2].as_f64().expect("w") as f32, value[3].as_f64().expect("h") as f32);
        let graph_rows: Vec<(String, Rect)> = case["graphs"].as_array().expect("fixture graphs").iter().map(|row| (row["surfaceId"].as_str().expect("surface id").to_string(), bounds(&row["bounds"]))).collect();
        let world_rows: Vec<(String, Rect, Option<String>)> =
            case["worlds"].as_array().expect("fixture worlds").iter().map(|row| (row["surfaceId"].as_str().expect("surface id").to_string(), bounds(&row["bounds"]), row["statusJson"].as_str().map(str::to_string))).collect();
        let graphs: Vec<(&str, Rect)> = graph_rows.iter().map(|(id, bounds)| (id.as_str(), *bounds)).collect();
        let worlds: Vec<(&str, Rect, Option<&str>)> = world_rows.iter().map(|(id, bounds, status)| (id.as_str(), *bounds, status.as_deref())).collect();
        let controls = surface_overlay_controls_for(&graphs, &worlds, &[], &theme, false);
        let expected: Vec<&str> = case["expected"].as_array().expect("fixture expectation").iter().map(|id| id.as_str().expect("control id")).collect();
        assert_eq!(controls.iter().map(|(control, _)| control.control_id.as_str()).collect::<Vec<_>>(), expected, "{}", case["id"]);
        for (control, anchor) in &controls {
            let surface_id = control.control_id.rsplit("::").next().expect("surface suffix");
            let source = worlds.iter().find(|(id, _, _)| *id == surface_id).map(|(_, bounds, _)| *bounds).expect("control names a live surface");
            assert!(anchor[0] >= source.x && anchor[1] >= source.y, "{}: {} is anchored inside the surface it annotates", case["id"], control.control_id);
        }
        rows += 1;
    }
    assert_eq!(rows, 5, "the fixture's five surface-control rows");
}

#[test]
fn the_cancel_control_dispatches_only_the_action_the_surface_itself_published() {
    let published = world3d_cancel_affordance(Some("{\"computing\":true,\"cancellable\":true,\"cancelAction\":\"stopTheThing\"}"));
    assert_eq!(published.cancel_action, "stopTheThing", "the shell learns the verb from the surface's own status contract, never from code");
    assert!(published.cancellable);
    assert!(!WGPU_SHELL_SOURCE().contains("\"cancelPreviewEval\""), "no domain verb may be hardcoded in the shell's cancel path");
}

//#endregion 🛑️SurfaceControls

//#region ⏳️ComputeStatusPane

/// ⏳️ Every field of the status contract, read the way the shared fixture declares — the Rust half of
/// the law the TypeScript `world3dComputeStatusV1` twin answers over the very same `statusPane` rows.
/// This is what makes the wgpu World3d surface show progress at all: before it, the shell read only
/// `cancellable`/`cancelAction` and threw the rest of the producer's report away.
#[test]
fn the_compute_status_is_read_field_for_field_the_way_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_CONTROLS_FIXTURE).expect("⏳️ the surface-controls fixture parses");
    let mut rows = 0;
    for case in fixture["statusPane"].as_array().expect("fixture status-pane rows") {
        let id = case["id"].as_str().expect("fixture row id");
        let status = world3d_compute_status(case["statusJson"].as_str());
        let expected = &case["expected"];
        assert_eq!(status.computing, expected["computing"].as_bool().expect("fixture computing"), "{id}: computing");
        assert_eq!(status.phase, expected["phase"].as_str().expect("fixture phase"), "{id}: phase");
        let expected_label = expected["phaseLabel"].as_object().map(|label| (label["en"].as_str().expect("fixture en").to_string(), label["de"].as_str().expect("fixture de").to_string()));
        assert_eq!(status.phase_label, expected_label, "{id}: the producer's own en/de pair, honoured only when both tongues carry text");
        for (name, observed) in [("unitsDone", status.units_done), ("unitsTotal", status.units_total), ("facesDone", status.faces_done), ("facesTotal", status.faces_total), ("inFlight", status.in_flight), ("ratio", status.ratio)] {
            assert!((observed - expected[name].as_f64().unwrap_or_else(|| panic!("{id}: fixture {name}"))).abs() < 1e-9, "{id}: {name} is {observed}");
        }
        assert_eq!(world3d_status_is_visible(&status), expected["visible"].as_bool().expect("fixture visible"), "{id}: visibility");
        let pill = world3d_status_pill_for("window:procedural-view-preview", &status, false);
        match expected["visible"].as_bool() {
            Some(true) => {
                let pill = pill.expect("an unsettled status asks for a pill");
                assert_eq!(pill.progress_text.as_deref(), expected["progressText"].as_str(), "{id}: progress text");
                assert_eq!(pill.ratio.is_some(), expected["progressText"].is_string(), "{id}: an indeterminate producer paints a bare track, never a filled one");
                rows += 1;
            }
            _ => assert!(pill.is_none(), "{id}: a settled producer annotates nothing"),
        }
    }
    assert_eq!(fixture["statusPane"].as_array().map(Vec::len), Some(11), "the fixture's eleven status-pane rows");
}

/// 🌍️ The pill carries the PRODUCER's language pair, and falls back to the shell's own bilingual
/// `common.loading` when the producer published none — never to the other language.
#[test]
fn the_status_pill_speaks_the_readers_language_and_never_defaults_to_one() {
    let status = world3d_compute_status(Some("{\"computing\":true,\"phase\":\"meshingFaces\",\"phaseLabel\":{\"en\":\"Meshing faces\",\"de\":\"Flächen werden vernetzt\"},\"progress\":{\"unitsDone\":3,\"unitsTotal\":8}}"));
    let english = world3d_status_pill_for("window:preview", &status, false).expect("a computing status asks for a pill");
    let german = world3d_status_pill_for("window:preview", &status, true).expect("a computing status asks for a pill");
    assert_eq!(english.phase_text, "Meshing faces");
    assert_eq!(german.phase_text, "Flächen werden vernetzt");
    assert_eq!(english.label(), "Meshing faces · 3/8 (38%)");
    assert_eq!(german.label(), "Flächen werden vernetzt · 3/8 (38%)");
    let unlabelled = world3d_compute_status(Some("{\"computing\":true,\"phase\":\"meshingFaces\"}"));
    assert_eq!(world3d_status_pill_for("window:preview", &unlabelled, false).expect("pill").phase_text, shell_chrome_string("common.loading", false));
    assert_eq!(world3d_status_pill_for("window:preview", &unlabelled, true).expect("pill").phase_text, shell_chrome_string("common.loading", true));
    assert_ne!(shell_chrome_string("common.loading", false), shell_chrome_string("common.loading", true), "the fallback itself carries both tongues");
}

/// 🖼️ A pill leads its surface's overlay row and the cancel control follows it, so the two never
/// paint on top of each other — and a surface too small to carry chrome carries neither.
#[test]
fn the_status_pill_leads_the_row_the_cancel_control_follows() {
    let theme = semio_framework_os_renderer_wgpu_host_base::resolve_theme("light");
    let computing =
        "{\"computing\":true,\"phase\":\"meshingFaces\",\"phaseLabel\":{\"en\":\"Meshing faces\",\"de\":\"Flächen werden vernetzt\"},\"cancellable\":true,\"cancelAction\":\"cancelPreviewEval\",\"progress\":{\"unitsDone\":3,\"unitsTotal\":8}}";
    let bounds = Rect::new(480.0, 40.0, 480.0, 320.0);
    let worlds = [("window:procedural-view-preview", bounds, Some(computing))];
    let pills = surface_status_pills_for(&worlds, &[], &theme, false);
    assert_eq!(pills.len(), 1, "one live computing surface asks for one pill");
    let (pill, rect) = &pills[0];
    assert_eq!(pill.surface_id, "window:procedural-view-preview");
    assert!(rect.x >= bounds.x && rect.y >= bounds.y && rect.x + rect.w <= bounds.x + bounds.w, "the pill paints inside the surface it annotates");
    let controls = surface_overlay_controls_for(&[], &worlds, &[], &theme, false);
    assert_eq!(controls.len(), 1, "the same surface still offers its cancel");
    assert!(controls[0].1[0] >= rect.x + rect.w, "the cancel control starts after the pill, never over it");
    let tiny = [("window:procedural-view-preview", Rect::new(24.0, 40.0, 24.0, 12.0), Some(computing))];
    assert!(surface_status_pills_for(&tiny, &[], &theme, false).is_empty(), "a collapsed pane carries no pill either");
}

/// ⏳️⛓️ The TIMELINE law — the one no single row can state. A long evaluation must publish a RUN of
/// non-idle frames whose ratio never decreases and whose every frame both paints a pill and offers
/// the cancel its producer names, and must end settled. A producer that publishes `idle` throughout
/// passes every row assertion above and fails this one, which is exactly the defect it exists for:
/// 54 identical `phase:"idle" inFlight:0 ratio:1.0` publications across one 23 s evaluation, on both
/// renderers (`📓️wgpu-progress-visibility-2026-09-14.md`).
#[test]
fn a_long_evaluation_publishes_a_monotone_run_of_non_idle_frames_and_then_settles() {
    let fixture: Value = serde_json::from_str(SURFACE_CONTROLS_FIXTURE).expect("⏳️ the surface-controls fixture parses");
    let timeline = &fixture["progressTimeline"];
    let minimum = timeline["minimumNonIdleFrames"].as_u64().expect("fixture minimum non-idle frames") as usize;
    let theme = semio_framework_os_renderer_wgpu_host_base::resolve_theme("light");
    let bounds = Rect::new(480.0, 40.0, 480.0, 320.0);
    for lane in ["evaluation", "cancelled"] {
        let frames = timeline[lane].as_array().unwrap_or_else(|| panic!("fixture {lane} frames"));
        let mut non_idle = 0usize;
        let mut previous_ratio = f64::NEG_INFINITY;
        for frame in frames {
            let id = frame["id"].as_str().expect("frame id");
            let status = world3d_compute_status(frame["statusJson"].as_str());
            let expected = &frame["expected"];
            assert_eq!(status.phase, expected["phase"].as_str().expect("frame phase"), "{lane}/{id}: phase");
            assert!((status.ratio - expected["ratio"].as_f64().expect("frame ratio")).abs() < 1e-9, "{lane}/{id}: ratio is {}", status.ratio);
            assert_eq!(world3d_status_is_visible(&status), expected["visible"].as_bool().expect("frame visible"), "{lane}/{id}: visibility");
            let pills = surface_status_pills_for(&[("window:procedural-preview", bounds, frame["statusJson"].as_str())], &[], &theme, false);
            let controls = surface_overlay_controls_for(&[], &[("window:procedural-preview", bounds, frame["statusJson"].as_str())], &[], &theme, false);
            if !expected["visible"].as_bool().expect("frame visible") {
                assert!(pills.is_empty() && controls.is_empty(), "{lane}/{id}: a settled frame annotates nothing");
                continue;
            }
            assert_eq!(pills.len(), 1, "{lane}/{id}: an unsettled frame paints its pill");
            assert_eq!(pills[0].0.progress_text.as_deref(), expected["progressText"].as_str(), "{lane}/{id}: progress text");
            if status.phase != "cancelled" {
                non_idle += 1;
                assert_ne!(status.phase, "idle", "{lane}/{id}: a frame that annotates the viewport may not call itself idle");
                assert!(status.ratio >= previous_ratio - 1e-9, "{lane}/{id}: ratio went backwards, {previous_ratio} → {}", status.ratio);
                previous_ratio = status.ratio;
                assert_eq!(controls.len(), 1, "{lane}/{id}: work in flight offers a cancel");
            }
        }
        let last = world3d_compute_status(frames.last().expect("a timeline has frames")["statusJson"].as_str());
        assert!(!world3d_status_is_visible(&last), "{lane}: the timeline ends settled");
        if lane == "evaluation" {
            assert!(non_idle >= minimum, "{lane}: {non_idle} non-idle frames, the law needs at least {minimum}");
        }
    }
    assert!(timeline["cancelled"].as_array().is_some_and(|frames| frames.iter().any(|frame| frame["expected"]["phase"] == "cancelled")), "the cancelled lane settles on `cancelled`, not on `idle`");
}

/// 🛑️🖱️ LAW: a press on the overlay chrome painted OVER an engine surface belongs to the shell, and
/// the renderer's press path asks before letting the surface claim the point.
///
/// 🩸️ Without this the World3d cancel control was decorative: it paints inside the surface rect, the
/// hit test names it, and the renderer returned on `bounds.contains` before the shell could dispatch
/// it — while the RELEASE path, which reaches the shell, is the phase `handle_shell_hit` ignores.
#[test]
fn a_press_on_the_overlay_chrome_over_a_surface_belongs_to_the_shell() {
    let theme = semio_framework_os_renderer_wgpu_host_base::resolve_theme("light");
    let bounds = Rect::new(480.0, 40.0, 480.0, 320.0);
    let computing = "{\"computing\":true,\"phase\":\"meshingFaces\",\"phaseLabel\":{\"en\":\"Meshing faces\",\"de\":\"Flächen werden vernetzt\"},\"cancellable\":true,\"cancelAction\":\"toolRunAbort\",\"progress\":{\"unitsDone\":3,\"unitsTotal\":8}}";
    let controls = surface_overlay_controls_for(&[], &[("procedural-preview", bounds, Some(computing))], &[], &theme, false);
    let (control, anchor) = controls.first().expect("a computing surface offers its cancel");
    assert!(control.control_id.starts_with("shell.world3d.cancel::"));
    assert!(anchor[0] > bounds.x && anchor[0] < bounds.x + bounds.w && anchor[1] > bounds.y && anchor[1] < bounds.y + bounds.h, "the control is painted INSIDE the surface it annotates — which is why the press path must ask");

    let hit = |kind: HitKind, control_id: &str| HitTarget::<ActionDescriptor> { rect: bounds, event: None, control_id: Some(control_id.to_string()), kind, drag_axis: None, drag_data: None };
    let chrome = hit(HitKind::NavbarItem, &control.control_id);
    let surface = hit(HitKind::World3d, "procedural-preview");
    assert!(pointer_press_belongs_to_shell_chrome(Some(&chrome)), "a press on the overlay control is the shell's");
    assert!(!pointer_press_belongs_to_shell_chrome(Some(&surface)), "a press on the surface body still orbits the scene");
    assert!(!pointer_press_belongs_to_shell_chrome(None), "a press on nothing is nobody's chrome");

    retained_world_sequence_probe("chrome");
}

//#endregion ⏳️ComputeStatusPane
