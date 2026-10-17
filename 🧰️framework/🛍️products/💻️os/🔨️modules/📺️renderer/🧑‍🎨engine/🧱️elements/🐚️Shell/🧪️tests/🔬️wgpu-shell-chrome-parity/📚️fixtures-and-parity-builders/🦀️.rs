/// 📚️ Fixtures and parity builders: the shared picker, switch, controls, safe-area, boot and icon fixtures and the parity app, mode and window-kind builders.

const EXAMPLE_PICKER_FIXTURE: &str = include_str!("../../../../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/📚️example-picker.json");
const SURFACE_SWITCH_FIXTURE: &str = include_str!("../../../../🏛️ShellHost/🧪️tests/🧫️fixtures/🔀️surface-switch/🔣️.json");
const SURFACE_CONTROLS_FIXTURE: &str = include_str!("../../🧫️fixtures/🛑️surface-controls/🔣️.json");
const CHROME_PANEL_SAFE_AREA_FIXTURE: &str = include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json");
const BOOT_EXAMPLE_FIXTURE: &str = include_str!("../../🧫️fixtures/📚️boot-example/🔣️.json");
const WINDOW_ICON_OVERRIDE_FIXTURE: &str = include_str!("../../🧫️fixtures/🪟️window-icon-overrides/🔣️.json");
#[allow(non_snake_case)]
fn WGPU_SHELL_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::shell_source()
}

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
        initial_utility_id: None,
        id: "main".into(),
        label: LocalizedLabel::native("Main", "Haupt"),
        body_key: "main.body".into(),
        surface_kind: semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
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
        actions: vec![],
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
        fault_notices: Vec::new(),
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
