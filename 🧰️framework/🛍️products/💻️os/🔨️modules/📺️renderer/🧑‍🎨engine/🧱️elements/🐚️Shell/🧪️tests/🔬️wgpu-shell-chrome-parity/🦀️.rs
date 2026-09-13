//! 🔀️ Laws for the wgpu shell's chrome parity with the React shell: the example picker, the mode and
//! surface-role navbar groups and their chords, and the two per-surface overlay controls (the World3d
//! compute cancel and the node-graph `Fit graph`).
//!
//! Every law is answered from a SHARED, language-neutral fixture — `📚️example-picker.json` (the
//! dialect-keyed picker, also answered by `manifest::examples_for_app`'s own Rust law and by the
//! TypeScript `examplesForApp` twin), `🔀️surface-switch/🔣️.json` (the React surface-switch lane's
//! own corpus) and this element's `🛑️surface-controls/🔣️.json` (also answered by the TypeScript
//! `world3dComputeStatusV1` twin). Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

use super::*;
use semio_framework::{AppDefinition, AppRole, ArtifactDialect, ExampleDefinition, ModeDefinition, Modes, WindowKindDefinition, WindowKinds};

const EXAMPLE_PICKER_FIXTURE: &str = include_str!("../../../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/📚️example-picker.json");
const SURFACE_SWITCH_FIXTURE: &str = include_str!("../../../🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json");
const SURFACE_CONTROLS_FIXTURE: &str = include_str!("../../🧫️fixtures/🛑️surface-controls/🔣️.json");
const WGPU_SHELL_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");

/// 🪪️ A dialect with no coordinate at all — how this target spells the fixture's `"dialect": null`
/// rows (`AppDefinition.dialect` is not optional in Rust, and an empty coordinate matches nothing,
/// which is exactly what a dialectless app must do to the roles group).
fn dialectless() -> ArtifactDialect {
    ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() }
}

fn fixture_dialect(value: &Value) -> ArtifactDialect {
    ArtifactDialect {
        artifact_kind: value["artifactKind"].as_str().expect("fixture dialect artifactKind").to_string(),
        standard: value["standard"].as_str().expect("fixture dialect standard").to_string(),
        subset: value["subset"].as_str().expect("fixture dialect subset").to_string(),
    }
}

fn fixture_role(value: &str) -> AppRole {
    match value {
        "editor" => AppRole::Editor,
        "viewer" => AppRole::Viewer,
        other => panic!("fixture declares an unknown surface role {other:?}"),
    }
}

fn parity_window_kinds() -> WindowKinds {
    WindowKinds::try_from(vec![WindowKindDefinition {
        id: "main".into(),
        label: LocalizedLabel::native("Main", "Haupt"),
        body_key: "main.body".into(),
        surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
        icon_id: "app-window".into(),
        options: Default::default(),
        actions: vec![],
        utilities: vec![],
        interactions: vec![],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: vec![],
    }])
    .expect("one window kind is non-empty")
}

fn parity_modes(mode_ids: &[&str]) -> Modes {
    Modes::try_from(mode_ids.iter().map(|id| ModeDefinition { id: (*id).into(), label: LocalizedLabel::native(id, id), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }).collect::<Vec<_>>())
        .expect("a parity app declares at least one mode")
}

fn parity_app(id: &str, role: AppRole, dialect: ArtifactDialect, label_en: &str, label_de: &str, mode_ids: &[&str]) -> AppDefinition {
    AppDefinition {
        id: id.into(),
        role,
        dialect,
        label: LocalizedLabel::native(label_en, label_de),
        breadcrumb: vec![],
        icon_id: None,
        controller_id: "parity".into(),
        modes: parity_modes(mode_ids),
        default_mode_id: mode_ids.first().copied().unwrap_or("edit").into(),
        window_kinds: parity_window_kinds(),
        panel_tabs: vec![],
        keybindings: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        interactions: vec![],
        named_layouts: vec![],
        default_layout: None,
        terminologies: vec![],
        terminology_breadcrumbs: HashMap::new(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: semio_framework_async::block_on(semio_framework::ConfigSpec::empty()),
        command_grammar: semio_framework_async::block_on(semio_framework::CommandGrammar::empty()),
        io: semio_framework::AppIo::default(),
    }
}

/// 🪪️ Every app of one fixture manifest, each carrying the label its own role reads in the navbar.
fn manifest_apps(fixture: &Value, manifest_id: &str) -> Vec<AppDefinition> {
    fixture["manifests"][manifest_id]
        .as_array()
        .unwrap_or_else(|| panic!("fixture declares manifest {manifest_id:?}"))
        .iter()
        .map(|row| {
            let role = fixture_role(row["role"].as_str().expect("fixture app role"));
            let dialect = row["dialect"].as_str().map_or_else(dialectless, |key| fixture_dialect(&fixture["dialects"][key]));
            let (en, de) = match role {
                AppRole::Editor => ("Editor", "Editor"),
                AppRole::Viewer => ("Viewer", "Betrachter"),
            };
            parity_app(row["id"].as_str().expect("fixture app id"), role, dialect, en, de, &["edit", "generate"])
        })
        .collect()
}

//#region 📚️ExamplePicker

#[test]
fn the_example_picker_offers_every_example_of_the_open_dialect_and_nothing_else() {
    let fixture: Value = serde_json::from_str(EXAMPLE_PICKER_FIXTURE).expect("📚️ the shared example-picker fixture parses");
    let examples: Vec<ExampleDefinition> = fixture["examples"]
        .as_array()
        .expect("fixture examples")
        .iter()
        .map(|row| ExampleDefinition {
            id: row["id"].as_str().expect("fixture example id").to_string(),
            label: LocalizedLabel::native(row["id"].as_str().expect("fixture example id"), row["id"].as_str().expect("fixture example id")),
            icon_id: "file".into(),
            artifact_json: "{}".into(),
            dialect: fixture_dialect(&row["dialect"]),
        })
        .collect();
    let mut cases = 0;
    for case in fixture["cases"].as_array().expect("fixture cases") {
        let app = parity_app(
            case["app"]["id"].as_str().expect("fixture app id"),
            fixture_role(case["app"]["role"].as_str().expect("fixture app role")),
            fixture_dialect(&case["app"]["dialect"]),
            "Surface",
            "Fläche",
            &["edit"],
        );
        let expected: Vec<String> = case["expected"].as_array().expect("fixture expectation").iter().map(|id| id.as_str().expect("fixture example id").to_string()).collect();
        let rows = shell_example_rows(&examples, &app, None, Terminology::default(), Locale::default());
        let control_ids: Vec<String> = rows.iter().map(|row| row.control_id.clone()).collect();
        let expected_control_ids: Vec<String> = expected.iter().map(|id| format!("shell.example.{id}")).collect();
        assert_eq!(control_ids, expected_control_ids, "{}: the picker's rows carry the React `shell.example.<id>` control ids, in manifest order", case["name"]);
        assert_eq!(
            shell_example_control(&rows, false, false).map(|control| control.control_id),
            (!expected.is_empty()).then(|| "playground.navbar.fixture".to_string()),
            "{}: the navbar hit target exists exactly when the open dialect authored an example",
            case["name"]
        );
        cases += 1;
    }
    assert!(cases >= 5, "the shared fixture is expected to carry every surface/dialect case");
    eprintln!("[DEBUG] wgpu example picker: {cases} dialect cases answered by the shared 📚️example-picker fixture");
}

#[test]
fn the_example_picker_trigger_shows_the_picked_row_and_the_open_dropdown_is_its_pressed_state() {
    let dialect = ArtifactDialect { artifact_kind: "s.procedural.generation3d".into(), standard: "1".into(), subset: "*".into() };
    let app = parity_app("s.procedural.generation3d@1/*#editor", AppRole::Editor, dialect.clone(), "Editor", "Editor", &["edit"]);
    let examples = vec![
        ExampleDefinition { id: "hex".into(), label: LocalizedLabel::native("Hexagonal Column", "Sechseckige Säule"), icon_id: "file".into(), artifact_json: "{}".into(), dialect: dialect.clone() },
        ExampleDefinition { id: "box".into(), label: LocalizedLabel::native("Box Shell", "Boxschale"), icon_id: "file".into(), artifact_json: "{}".into(), dialect },
    ];
    let rows = shell_example_rows(&examples, &app, Some("box"), Terminology::default(), Locale::De);
    assert_eq!(rows.iter().filter(|row| row.selected).map(|row| row.control_id.as_str()).collect::<Vec<_>>(), vec!["shell.example.box"]);
    assert_eq!(rows[0].label, "Sechseckige Säule", "row labels are the example's OWN localized label — no shell dictionary, no default language");
    let control = shell_example_control(&rows, true, true).expect("two examples offer a picker");
    assert_eq!(control.label, "Boxschale", "the trigger shows the picked row, the way a select shows its value");
    assert!(control.active, "an open dropdown is the trigger's pressed state");
    assert!(!shell_example_control(&rows, false, true).expect("picker").active);
    let empty = shell_example_rows(&[], &app, None, Terminology::default(), Locale::default());
    assert!(shell_example_control(&empty, false, false).is_none(), "a dialect with no authored example paints no trigger at all");
    eprintln!("[DEBUG] wgpu example picker trigger: localized label, selected row and pressed state all hold");
}

//#endregion 📚️ExamplePicker

//#region 🎛️ModeAndRoleGroups

#[test]
fn the_roles_group_exists_exactly_when_the_plugin_declares_both_surfaces_of_the_open_dialect() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ the shared surface-switch fixture parses");
    let mut rows = 0;
    for case in fixture["group"].as_array().expect("fixture group rows") {
        let apps = manifest_apps(&fixture, case["manifest"].as_str().expect("fixture manifest id"));
        let dialect = case["dialect"].as_str().map_or_else(dialectless, |key| fixture_dialect(&fixture["dialects"][key]));
        let open = parity_app("open", AppRole::Editor, dialect.clone(), "Editor", "Editor", &["edit"]);
        let controls = shell_role_controls(&apps, &open, Terminology::default(), Locale::default());
        match case["expected"].as_object() {
            Some(expected) => {
                assert_eq!(
                    controls.iter().map(|control| control.control_id.as_str()).collect::<Vec<_>>(),
                    vec!["playground.navbar.roles.editor", "playground.navbar.roles.viewer"],
                    "{}: the group renders both React control ids in editor → viewer focus order",
                    case["id"]
                );
                let resolved = surface_role_apps(&apps, &dialect).expect("both surfaces resolve");
                assert_eq!(resolved.0.id, expected["editor"].as_str().expect("fixture editor id"));
                assert_eq!(resolved.1.id, expected["viewer"].as_str().expect("fixture viewer id"));
                assert!(controls[0].active, "the open editor surface is the pressed button");
                assert!(!controls[1].active);
            }
            None => {
                assert!(controls.is_empty(), "{}: no sibling surface means no group at all", case["id"]);
                assert!(surface_role_apps(&apps, &dialect).is_none());
            }
        }
        rows += 1;
    }
    assert_eq!(rows, 4, "the shared fixture's four group rows");
    eprintln!("[DEBUG] wgpu roles group: {rows} shared fixture rows answered identically to React");
}

#[test]
fn a_role_switch_resolves_the_app_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let mut rows = 0;
    for case in fixture["roleTargets"].as_array().expect("fixture roleTargets rows") {
        let apps = manifest_apps(&fixture, case["manifest"].as_str().expect("fixture manifest id"));
        let dialect = fixture_dialect(&fixture["dialects"][case["dialect"].as_str().expect("fixture dialect key")]);
        let target = role_switch_target(&apps, &dialect, fixture_role(case["currentRole"].as_str().expect("fixture current role")), fixture_role(case["requested"].as_str().expect("fixture requested role")));
        assert_eq!(target.map(|app| app.id.as_str()), case["expected"].as_str(), "{}", case["id"]);
        rows += 1;
    }
    assert_eq!(rows, 4, "the shared fixture's four role-target rows");
    eprintln!("[DEBUG] wgpu role switch targets: {rows} shared fixture rows answered identically to React");
}

#[test]
fn the_mode_group_renders_one_pressed_button_per_declared_mode_and_none_for_a_single_mode_surface() {
    let dialect = ArtifactDialect { artifact_kind: "s.procedural.generation3d".into(), standard: "1".into(), subset: "*".into() };
    let editor = parity_app("s.procedural.generation3d@1/*#editor", AppRole::Editor, dialect.clone(), "Editor", "Editor", &["edit", "generate"]);
    let controls = shell_mode_controls(&editor, Some("generate"), Terminology::default(), Locale::default());
    assert_eq!(controls.iter().map(|control| control.control_id.as_str()).collect::<Vec<_>>(), vec!["playground.navbar.modes.edit", "playground.navbar.modes.generate"]);
    assert_eq!(controls.iter().map(|control| control.active).collect::<Vec<_>>(), vec![false, true]);
    let viewer = parity_app("s.procedural.generation3d@1/*#viewer", AppRole::Viewer, dialect, "Viewer", "Betrachter", &["view"]);
    assert!(shell_mode_controls(&viewer, Some("view"), Terminology::default(), Locale::default()).is_empty(), "a one-mode surface renders no mode switcher, exactly as React renders none");
    eprintln!("[DEBUG] wgpu mode group: two-mode editor paints both ids, one-mode viewer paints none");
}

#[test]
fn stepping_the_mode_wraps_the_way_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let mut rows = 0;
    for case in fixture["modeSteps"].as_array().expect("fixture modeSteps rows") {
        let mode_ids: Vec<String> = case["modeIds"].as_array().expect("fixture mode ids").iter().map(|id| id.as_str().expect("fixture mode id").to_string()).collect();
        let step = i32::try_from(case["step"].as_i64().expect("fixture step")).expect("fixture step fits");
        assert_eq!(step_mode_id(&mode_ids, case["activeModeId"].as_str(), step).as_deref(), case["expected"].as_str(), "{}", case["id"]);
        rows += 1;
    }
    assert_eq!(rows, 6, "the shared fixture's six mode-step rows");
    eprintln!("[DEBUG] wgpu mode step: {rows} shared fixture rows answered identically to React");
}

//#endregion 🎛️ModeAndRoleGroups

//#region ⌨️Chords

fn chord_key_action(key: &str) -> ui_wgpu::wgpu::KeyAction {
    match key {
        "ArrowRight" => ui_wgpu::wgpu::KeyAction::ArrowRight,
        "ArrowLeft" => ui_wgpu::wgpu::KeyAction::ArrowLeft,
        other => ui_wgpu::wgpu::KeyAction::Char(other.to_string()),
    }
}

#[test]
fn every_shared_keybinding_row_routes_to_its_shell_verb_and_outranks_app_keybindings() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let apps = manifest_apps(&fixture, "procedural");
    let dialect = fixture_dialect(&fixture["dialects"]["generation3d"]);
    let mut rows = 0;
    for case in fixture["keybindings"].as_array().expect("fixture keybinding rows") {
        let event = &case["event"];
        let action = chord_key_action(event["key"].as_str().expect("fixture chord key"));
        let modifiers = PointerModifiers {
            shift: event["shiftKey"].as_bool().unwrap_or(false),
            ctrl: event["ctrlKey"].as_bool().unwrap_or(false),
            alt: event["altKey"].as_bool().unwrap_or(false),
            meta: event["metaKey"].as_bool().unwrap_or(false),
        };
        assert!(is_reserved_shell_chord(&action, &modifiers), "{}: a shell chord must outrank every app-declared keybinding", case["controlId"]);
        let control_id = case["controlId"].as_str().expect("fixture control id");
        if let Some(role_suffix) = control_id.strip_prefix("playground.navbar.roles.") {
            assert_eq!(shell_role_chord(&action, &modifiers), Some(fixture_role(role_suffix)), "{control_id}");
            assert_eq!(shell_mode_step_chord(&action, &modifiers), None, "{control_id}: a role chord is not a mode chord");
            let dispatches = &case["dispatches"];
            let target = role_switch_target(&apps, &dialect, fixture_role(dispatches["currentRole"].as_str().expect("fixture current role")), fixture_role(dispatches["requested"].as_str().expect("fixture requested role")));
            assert_eq!(target.map(|app| app.id.as_str()), dispatches["expectedAppId"].as_str(), "{control_id}: the chord opens the app the fixture declares");
        } else {
            let expected_step = match control_id {
                "ui.shell.mode.next" => 1,
                "ui.shell.mode.previous" => -1,
                other => panic!("fixture declares an unknown shell keybinding {other:?}"),
            };
            assert_eq!(shell_mode_step_chord(&action, &modifiers), Some(expected_step), "{control_id}");
            assert_eq!(shell_role_chord(&action, &modifiers), None, "{control_id}: a mode chord is not a role chord");
        }
        rows += 1;
    }
    assert_eq!(rows, 4, "the shared fixture's four keybinding rows");
    eprintln!("[DEBUG] wgpu shell chords: {rows} shared fixture rows route to the same verb React's SHELL_KEYBINDINGS declare");
}

#[test]
fn the_alt_axis_never_swallows_a_neighbouring_chord() {
    let accelerator = PointerModifiers { shift: false, ctrl: false, alt: false, meta: true };
    let with_alt = PointerModifiers { alt: true, ..accelerator };
    let bare = PointerModifiers::default();
    assert_eq!(shell_role_chord(&ui_wgpu::wgpu::KeyAction::Char("e".into()), &accelerator), None, "mod+e is not the role chord — mod+alt+e is");
    assert_eq!(shell_role_chord(&ui_wgpu::wgpu::KeyAction::Char("v".into()), &bare), None, "a bare `v` is ordinary text");
    assert_eq!(shell_mode_step_chord(&ui_wgpu::wgpu::KeyAction::ArrowRight, &accelerator), None);
    assert!(is_reserved_shell_chord(&ui_wgpu::wgpu::KeyAction::Char("f".into()), &accelerator), "mod+f stays the find chord");
    assert!(!is_reserved_shell_chord(&ui_wgpu::wgpu::KeyAction::Char("f".into()), &with_alt), "mod+alt+f belongs to whoever declares it, not to find");
    assert!(!is_reserved_shell_chord(&ui_wgpu::wgpu::KeyAction::Char("f".into()), &bare), "a bare `f` is the `Fit graph` shortcut, never a reserved accelerator");
    eprintln!("[DEBUG] wgpu shell chords: the alt axis is disjoint from the palette/find/panel accelerators");
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
        rows += 1;
    }
    assert_eq!(rows, 9, "the fixture's nine cancel-contract rows");
    eprintln!("[DEBUG] wgpu world3d cancel contract: {rows} fixture rows, hostile payloads included, degrade to no affordance");
}

#[test]
fn a_live_surface_offers_exactly_the_overlay_controls_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_CONTROLS_FIXTURE).expect("🛑️ fixture parses");
    let theme = crate::resolve_theme("light");
    // 🔤️ The fixture carries the shared control-height token so the TypeScript twin can apply the
    // very same size gate without re-deriving `UI_SPACING_COMPACT_PX × CONTROL_HEIGHT_UI_SPACING`.
    assert!(
        (f64::from(theme.control_height) - fixture["controlHeightPx"].as_f64().expect("fixture control height")).abs() < 1e-6,
        "the fixture's declared control height must stay the theme's own token"
    );
    let mut rows = 0;
    for case in fixture["surfaceControls"].as_array().expect("fixture surface rows") {
        let bounds = |value: &Value| Rect::new(value[0].as_f64().expect("x") as f32, value[1].as_f64().expect("y") as f32, value[2].as_f64().expect("w") as f32, value[3].as_f64().expect("h") as f32);
        let graph_rows: Vec<(String, Rect)> = case["graphs"].as_array().expect("fixture graphs").iter().map(|row| (row["surfaceId"].as_str().expect("surface id").to_string(), bounds(&row["bounds"]))).collect();
        let world_rows: Vec<(String, Rect, Option<String>)> = case["worlds"]
            .as_array()
            .expect("fixture worlds")
            .iter()
            .map(|row| (row["surfaceId"].as_str().expect("surface id").to_string(), bounds(&row["bounds"]), row["statusJson"].as_str().map(str::to_string)))
            .collect();
        let graphs: Vec<(&str, Rect)> = graph_rows.iter().map(|(id, bounds)| (id.as_str(), *bounds)).collect();
        let worlds: Vec<(&str, Rect, Option<&str>)> = world_rows.iter().map(|(id, bounds, status)| (id.as_str(), *bounds, status.as_deref())).collect();
        let controls = surface_overlay_controls_for(&graphs, &worlds, &theme, false);
        let expected: Vec<&str> = case["expected"].as_array().expect("fixture expectation").iter().map(|id| id.as_str().expect("control id")).collect();
        assert_eq!(controls.iter().map(|(control, _)| control.control_id.as_str()).collect::<Vec<_>>(), expected, "{}", case["id"]);
        for ((control, anchor), source) in controls.iter().zip(graphs.iter().map(|(_, bounds)| *bounds).chain(worlds.iter().map(|(_, bounds, _)| *bounds))) {
            assert!(anchor[0] >= source.x && anchor[1] >= source.y, "{}: {} is anchored inside the surface it annotates", case["id"], control.control_id);
        }
        rows += 1;
    }
    assert_eq!(rows, 5, "the fixture's five surface-control rows");
    eprintln!("[DEBUG] wgpu surface controls: {rows} fixture rows offer the Fit graph and cancel hit targets exactly when the surface declares them");
}

#[test]
fn the_cancel_control_dispatches_only_the_action_the_surface_itself_published() {
    let published = world3d_cancel_affordance(Some("{\"computing\":true,\"cancellable\":true,\"cancelAction\":\"stopTheThing\"}"));
    assert_eq!(published.cancel_action, "stopTheThing", "the shell learns the verb from the surface's own status contract, never from code");
    assert!(published.cancellable);
    assert!(!WGPU_SHELL_SOURCE.contains("\"cancelPreviewEval\""), "no domain verb may be hardcoded in the shell's cancel path");
    eprintln!("[DEBUG] wgpu world3d cancel: the dispatched verb is the published one and no domain verb is compiled in");
}

//#endregion 🛑️SurfaceControls

//#region 🔀️SwitchOrder

#[test]
fn the_session_switch_creates_before_it_retires_and_leaks_no_instance() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let switched = fixture["switch"].as_array().expect("fixture switch rows").iter().find(|case| case["expected"]["status"] == "switched").expect("the fixture declares at least one completed switch");
    let declared: Vec<String> = switched["expected"]["steps"].as_array().expect("fixture steps").iter().map(|step| step.as_str().expect("fixture step").split(':').next().expect("step verb").to_string()).collect();
    let body_start = WGPU_SHELL_SOURCE.find("async fn run_session_app_switch").expect("the shell declares the ordered session switch");
    let body = &WGPU_SHELL_SOURCE[body_start..];
    let body = &body[..body.find("\n    /// 🔁️ Steps the active mode").expect("the switch body ends before the next item")];
    let at = |needle: &str| body.find(needle).unwrap_or_else(|| panic!("the ordered switch performs {needle}"));
    let (create, seal, retire, publish, refresh) = (at("create_app("), at("retire_documents_outside("), at("destroy_app("), at("push_contributions("), at("refresh_ui("));
    assert!(create < seal && seal < retire, "create → seal → retire: the successor exists before the predecessor's documents and instance go");
    assert!(retire < publish && publish < refresh, "retire → publish → refresh");
    assert_eq!(body.matches("create_app(").count(), 1, "created − retired === 0: one create per switch");
    assert_eq!(body.matches("destroy_app(").count(), 1, "created − retired === 0: one retire per switch");
    // 🔀️ The SHARED subsequence both shells hold: seal before retire, create before retire, retire
    // before publish, publish before refresh. React's `quiesce` and `seed` steps have no wgpu
    // counterpart yet — this target has no pending-guest-work tracker to quiesce and seeds its
    // landing layout inside the same `sync_dock` the publish step runs — so they are deliberately
    // NOT asserted against this body rather than faked.
    let declared_at = |verb: &str| declared.iter().position(|step| step == verb).unwrap_or_else(|| panic!("the shared fixture declares {verb}"));
    assert!(declared_at("seal") < declared_at("retire"));
    assert!(declared_at("create") < declared_at("retire"));
    assert!(declared_at("retire") < declared_at("publish"));
    assert!(declared_at("publish") < declared_at("refresh"));
    eprintln!("[DEBUG] wgpu session switch order: create → seal → retire → publish → refresh, one create and one retire; shared fixture declares {declared:?}");
}

#[test]
fn asking_for_the_role_already_mounted_creates_and_retires_nothing() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let dialect = ArtifactDialect { artifact_kind: "s.procedural.generation3d".into(), standard: "1".into(), subset: "*".into() };
    let app = parity_app("s.procedural.generation3d@1/*#editor", AppRole::Editor, dialect, "Editor", "Editor", &["edit", "generate"]);
    shell.session = Some(ActiveSession { plugin_id: "procedural".into(), instance_id: 7, app, view_state: ViewModel::default() });
    semio_framework_async::block_on(shell.switch_to_session_role(AppRole::Editor)).expect("a no-op switch never fails");
    let session = shell.session.as_ref().expect("the session survives a no-op switch");
    assert_eq!(session.instance_id, 7, "the mounted instance is untouched");
    assert_eq!(session.app.role, AppRole::Editor);
    eprintln!("[DEBUG] wgpu session switch: asking for the mounted role is a no-operation");
}

//#endregion 🔀️SwitchOrder
