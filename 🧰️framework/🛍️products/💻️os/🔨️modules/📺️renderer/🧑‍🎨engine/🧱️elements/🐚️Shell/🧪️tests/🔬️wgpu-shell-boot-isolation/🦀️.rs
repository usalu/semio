use super::*;
use semio_framework::{AppRole, ArtifactDialect, ModeDefinition, Modes, PluginManifest, WindowKindDefinition, WindowKinds};

/// 🧪️ The smallest app a boot selection can name — only `id` and the mandatory window kind matter here.
fn boot_app(app_id: &str) -> AppDefinition {
    AppDefinition {
        id: app_id.into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.boot".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data(app_id),
        breadcrumb: vec!["semio".into()],
        icon_id: None,
        controller_id: "boot".into(),
        modes: Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
        default_mode_id: "default".into(),
        window_kinds: WindowKinds::try_from(vec![WindowKindDefinition {
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
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
        .expect("non-empty window kinds"),
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

fn boot_manifest(plugin_id: &str, app_ids: &[&str]) -> PluginManifest {
    PluginManifest {
        plugin_id: plugin_id.into(),
        label: plugin_id.into(),
        version: "1".into(),
        apps: app_ids.iter().map(|app_id| boot_app(app_id)).collect(),
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![],
        artifact_kinds: vec![],
        dependencies: vec![],
        contributions: vec![],
    }
}

/// 🧪️ Language-agnostic table (`🧫️fixtures/🔬️wgpu-shell-boot-selection/🔣️.json`): the renderer receives
/// its boot plan in DEPENDENCY order, so `plugins.first()` is a dependency of the requested plugin —
/// taking it opened `flow`'s app for `?plugin=generation3d` and turned flow's first-step trap into the
/// shell's own boot failure (`📓️runtime-verification-2026-09-09.md`, wgpu boot #1).
#[test]
fn boot_selection_opens_the_requested_variant_across_the_fixture_table() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔬️wgpu-shell-boot-selection/🔣️.json")).expect("boot selection fixture parses");
    for case in fixture["cases"].as_array().expect("fixture cases") {
        let variant = case["variant"].as_str().expect("case variant");
        let manifests: Vec<(String, PluginManifest)> = case["programs"]
            .as_array()
            .expect("case programs")
            .iter()
            .map(|program| {
                let app_ids: Vec<&str> = program["appIds"].as_array().expect("program appIds").iter().map(|id| id.as_str().expect("app id")).collect();
                (program["pluginId"].as_str().expect("program pluginId").to_string(), boot_manifest(program["pluginId"].as_str().expect("program pluginId"), &app_ids))
            })
            .collect();
        let programs: Vec<(&str, &PluginManifest)> = manifests.iter().map(|(plugin_id, manifest)| (plugin_id.as_str(), manifest)).collect();
        let selected = select_boot_program(&programs, variant);
        match case["expected"].as_object() {
            None => assert!(selected.is_none(), "case {} must not open a foreign plugin's app", case["id"]),
            Some(expected) => {
                let (index, app) = selected.unwrap_or_else(|| panic!("case {} selects a program", case["id"]));
                assert_eq!(index as u64, expected["index"].as_u64().expect("expected index"), "case {}", case["id"]);
                assert_eq!(app.id, expected["appId"].as_str().expect("expected appId"), "case {}", case["id"]);
            }
        }
    }
    eprintln!("[DEBUG] wgpu shell boot selection honoured every fixture case");
}

/// 🧪️ A boot that cannot open its app is that plugin's fault, not the renderer's: `boot()` still
/// settles (chrome + first refresh) and reports the plugin by name instead of rejecting `bootShell`.
#[test]
fn boot_without_the_requested_plugin_settles_with_a_per_plugin_status() {
    let mut shell = ShellState::new(Vec::new(), "generation3d".into());
    semio_framework_async::block_on(shell.boot()).expect("a missing plugin never fails the shell boot");
    assert!(shell.session.is_none());
    assert_eq!(shell.plugin_faults.len(), 1);
    assert_eq!(shell.plugin_faults[0].plugin_id, "procedural");
    assert_eq!(shell.plugin_faults[0].app_id, "s.procedural.generation3d@1/*#editor");
    assert_eq!(shell.error, shell.plugin_fault_status());
    eprintln!("[DEBUG] wgpu shell boot isolated the missing plugin: {:?}", shell.error);
}

/// 🧪️ The per-plugin status renders in English and in German, with no default language, and always
/// carries the guest's own cause verbatim.
#[test]
fn plugin_fault_status_reads_in_both_languages() {
    let mut shell = ShellState::new(Vec::new(), "generation3d".into());
    assert!(shell.plugin_fault_status().is_none());
    shell.plugin_faults.push(ShellPluginFault {
        plugin_id: "flow".into(),
        app_id: "s.flow.flow@1/*#editor".into(),
        detail: "interactive-job.catalog-authority: tool 'addGeneration'".into(),
    });
    shell.locale_id = "en".into();
    let english = shell.plugin_fault_status().expect("english status");
    assert!(english.starts_with("Plugin unavailable: flow (s.flow.flow@1/*#editor)"), "{english}");
    assert!(english.contains("interactive-job.catalog-authority"), "{english}");
    shell.locale_id = "de".into();
    let german = shell.plugin_fault_status().expect("german status");
    assert!(german.starts_with("Plugin nicht verfügbar: flow (s.flow.flow@1/*#editor)"), "{german}");
    assert!(german.contains("interactive-job.catalog-authority"), "{german}");
}

/// 🧪️ A surface whose document cannot be read is that SURFACE's fault, not the renderer's: the shell
/// keeps every other surface, shows the failing one as a typed card, and `refresh_ui` still succeeds —
/// the `?` that used to sit here turned `wgpu-ui.intake-budget-exhausted` into `shell-boot: …`, then
/// into `worker-boot-failed`, which closed the Worker and its transferred `OffscreenCanvas`
/// (`📓️wgpu-intake-budget-2026-09-10.md`, wgpu boot #4).
#[test]
fn a_poisoned_surface_faults_alone() {
    let mut shell = ShellState::new(Vec::new(), "generation3d".into());
    shell.settle_surface_faults(vec![
        ("flow-window".into(), "flow.body".into(), "renderDocument promise failed: wgpu-ui.intake-budget-exhausted:intake:163840001".into()),
        ("preview".into(), "preview.body".into(), "wgpu-ui.surface-not-published:preview".into()),
    ]);
    assert_eq!(shell.surface_faults.len(), 2);
    assert_eq!(shell.surface_faults[0].surface_id, "flow-window");
    assert!(shell.plugin_faults.is_empty(), "a surface fault never blames a plugin's activation");
    assert_eq!(shell.error, shell.fault_status());

    shell.settle_surface_faults(vec![("preview".into(), "preview.body".into(), "wgpu-ui.surface-not-published:preview".into())]);
    assert_eq!(shell.surface_faults.len(), 1, "a surface that recovered stops showing a card");
    assert_eq!(shell.surface_faults[0].surface_id, "preview");

    shell.settle_surface_faults(Vec::new());
    assert!(shell.surface_faults.is_empty());
    assert!(shell.error.is_none(), "no fault leaves no card");
}

/// 🧪️ The per-surface status renders in English and in German, with no default language, carries the
/// cause verbatim, and never accumulates two cards for one surface.
#[test]
fn surface_fault_status_reads_in_both_languages_and_never_duplicates_a_surface() {
    let mut shell = ShellState::new(Vec::new(), "generation3d".into());
    assert!(shell.surface_fault_status().is_none());
    shell.record_surface_fault("flow-window", "flow.body", "wgpu-ui.intake-budget-exhausted:intake:163840001".into());
    shell.record_surface_fault("flow-window", "flow.body", "wgpu-ui.read-snapshot-missing".into());
    assert_eq!(shell.surface_faults.len(), 1, "one surface owns exactly one card");
    assert_eq!(shell.surface_faults[0].detail, "wgpu-ui.read-snapshot-missing");

    shell.locale_id = "en".into();
    let english = shell.surface_fault_status().expect("english status");
    assert!(english.starts_with("Surface unavailable: flow-window (flow.body)"), "{english}");
    assert!(english.contains("wgpu-ui.read-snapshot-missing"), "{english}");
    shell.locale_id = "de".into();
    let german = shell.surface_fault_status().expect("german status");
    assert!(german.starts_with("Fläche nicht verfügbar: flow-window (flow.body)"), "{german}");
    assert!(german.contains("wgpu-ui.read-snapshot-missing"), "{german}");

    shell.plugin_faults.push(ShellPluginFault { plugin_id: "flow".into(), app_id: "s.flow.flow@1/*#editor".into(), detail: "trapped".into() });
    let combined = shell.fault_status().expect("combined status");
    assert!(combined.contains("Plugin nicht verfügbar"), "{combined}");
    assert!(combined.contains("Fläche nicht verfügbar"), "{combined}");
}
