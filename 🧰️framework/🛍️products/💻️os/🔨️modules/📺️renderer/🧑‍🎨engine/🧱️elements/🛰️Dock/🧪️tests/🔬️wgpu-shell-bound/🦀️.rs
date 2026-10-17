//! 🛰️ Dock laws that read shell state: the unit laws of `🔬️wgpu-unit` that need the shell crates, mounted by the shell test bin.

use super::*;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use {semio_framework::AppDefinition,semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect,semio_framework::ModeDefinition,semio_framework::PanelGroup,semio_framework::PanelTabDefinition,semio_framework::PanelTabKind,semio_framework::WindowKindDefinition};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_layout::layout::{create_default_layout, WindowOptions};
//#region WindowActionsAndUtilitiesTests
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, UtilityDefinition, UtilityRef};
use ui_wgpu::wgpu::{KeyAction, PointerModifiers};

fn tab(id: &str) -> DockStackTab {
    DockStackTab::new(id)
}

fn tabs(ids: &[&str]) -> Vec<DockStackTab> {
    ids.iter().map(|id| DockStackTab::new(*id)).collect()
}

fn stack_tabs(ids: &[&str], active: &str) -> DockNode {
    DockNode::Stack { windows: tabs(ids), active: active.into() }
}

fn sample_app(window_ids: &[&str], layout: Option<WindowLayout>) -> AppDefinition {
    AppDefinition {
        id: "test".into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.dock".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Test"),
        breadcrumb: vec!["semio".into(), "test".into()],
        icon_id: None,
        controller_id: "test".into(),
        modes: semio_framework::Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
        default_mode_id: "default".into(),
        window_kinds: semio_framework::WindowKinds::try_from(
            window_ids
                .iter()
                .map(|id| WindowKindDefinition {
                    initial_utility_id: None,
                    id: (*id).into(),
                    label: LocalizedLabel::data(*id),
                    body_key: format!("{id}.body"),
                    surface_kind: semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
                    icon_id: "app-window".into(),
                    options: WindowOptions::default(),
                    actions: vec![],
                    interactions: vec![],
                    utilities: vec![],
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                })
                .collect::<Vec<_>>(),
        )
        .expect("sample_app tests always pass at least one window id"),
        panel_tabs: vec![PanelTabDefinition { kind: PanelTabKind::App("tab".into()), label: LocalizedLabel::data("Tab"), group: PanelGroup::Workbench, body_key: Some("tab.body".into()), children: vec![] }],
        keybindings: vec![],
        actions: vec![],
        interactions: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        named_layouts: vec![],
        default_layout: layout,
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

#[test]
fn unregistered_world_and_pane_identifiers_cannot_claim_scene_wheel() {
    let shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let theme = Theme::default();
    for (kind, id) in [(HitKind::ScrollRegion, "panel.left.lowpoly"), (HitKind::World3d, "world-surface"), (HitKind::ScrollRegion, "graph-surface.pane"), (HitKind::ScrollRegion, "map-surface.map")] {
        let mut input = InputState::<ActionDescriptor>::default();
        input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 800.0, 600.0), event: None, control_id: Some(id.into()), kind, drag_axis: None, drag_data: None });
        input.publish_hits();
        assert!(!shell.input.wheel_reaches_scene_surface(400.0, 300.0, &input, &theme), "{id} has no published scene provenance");
    }
}

fn stack_with(id: &str) -> DockNode {
    DockNode::Stack { windows: vec![tab(id)], active: id.into() }
}

#[test]
fn map_marquee_mode_matches_ui_react() {
        use semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_marquee_mode;
    assert_eq!(map_marquee_mode(false, false), "replace");
    assert_eq!(map_marquee_mode(true, false), "additive");
    assert_eq!(map_marquee_mode(false, true), "subtractive");
    assert_eq!(map_marquee_mode(true, true), "invertive");
}

fn mods(meta: bool, ctrl: bool, shift: bool, alt: bool) -> PointerModifiers {
    PointerModifiers { meta, ctrl, shift, alt }
}

/// 🧰️ Builds a two-window app: window `main` scopes `utility.a`, window `aux` scopes nothing; `utility.b`
/// is an orphan (no window references it). Actions: `zeroArg` (no args) + `withArgs` (required text +
/// defaulted toggle) scoped to `main`.
///
/// Scope utility.a + both actions to `main`; leave utility.b an orphan referenced by no window.
fn actions_utilities_app() -> AppDefinition {
    let mut app = sample_app(&["main", "aux"], None);
    app.controller_id = "ctrl".into();
    app.utilities = vec![UtilityDefinition::new("utility.a", LocalizedLabel::data("Utility A"), "circle"), UtilityDefinition { allows_actions_while_active: true, ..UtilityDefinition::new("utility.b", LocalizedLabel::data("Utility B"), "square") }];
    let actions = vec![
        ActionDefinition::bounded_catalog("zeroArg", LocalizedLabel::data("Zero Arg"), ActionKind::View),
        ActionDefinition {
            args: vec![ActionArgDef::text("name", LocalizedLabel::data("Name")).required(), ActionArgDef { default: Some(semio_framework_value::ToValue::to_value(&true)), ..ActionArgDef::toggle("flag", LocalizedLabel::data("Flag")) }],
            keys: Some("mod+e".into()),
            ..ActionDefinition::bounded_catalog("withArgs", LocalizedLabel::data("With Args"), ActionKind::View)
        },
    ];
    for kind in app.window_kinds.iter_mut() {
        if kind.id == "main" {
            kind.utilities = vec![UtilityRef::new("utility.a")];
            kind.actions = actions.clone();
        }
    }
    app
}

fn shell() -> ShellState {
    ShellState::new(vec![], "test".into(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
}

/// 🗃️ `main` gets its explicit utility.a first, then the orphan utility.b; `aux` only sees the orphan.
#[test]
fn resolve_window_utilities_scopes_explicit_and_orphans() {
    let app = actions_utilities_app();
    let main = app.window_kinds.iter().find(|k| k.id == "main").unwrap();
    let aux = app.window_kinds.iter().find(|k| k.id == "aux").unwrap();
    let main_ids: Vec<&str> = semio_framework_os_renderer_wgpu_shell_dock::resolve_window_utilities(&app, main).iter().map(|t| t.id.as_str()).collect();
    let aux_ids: Vec<&str> = semio_framework_os_renderer_wgpu_shell_dock::resolve_window_utilities(&app, aux).iter().map(|t| t.id.as_str()).collect();
    assert_eq!(main_ids, vec!["utility.a", "utility.b"]);
    assert_eq!(aux_ids, vec!["utility.b"]);
}

/// ⌨️ Shift held but not declared → no match; declared shift required.
///
/// plain key must not fire while the accelerator is held.
#[test]
fn key_event_matches_chord_respects_modifiers() {
        use semio_framework_os_renderer_wgpu_shell_docs::key_event_matches_chord;
    let z = KeyAction::Char("z".into());
    assert!(key_event_matches_chord(&z, &mods(true, false, false, false), "mod+z"));
    assert!(key_event_matches_chord(&z, &mods(false, true, false, false), "mod+z"));
    assert!(!key_event_matches_chord(&z, &mods(true, false, true, false), "mod+z"));
    assert!(key_event_matches_chord(&z, &mods(true, false, true, false), "mod+shift+z"));
    let k = KeyAction::Char("k".into());
    assert!(key_event_matches_chord(&k, &mods(false, false, false, false), "k"));
    assert!(!key_event_matches_chord(&k, &mods(true, false, false, false), "k"));
    assert!(key_event_matches_chord(&KeyAction::Escape, &mods(false, false, false, false), "escape"));
}

/// 🚦️ Nothing staged → required `name` missing → no executable args (P2 gate).
///
/// Stage the required arg → executes, merging the defaulted `flag`.
#[test]
fn required_arg_gates_execution_and_merges_defaults() {
    let app = actions_utilities_app();
    let defs = &app.window_kinds.iter().find(|kind| kind.id == "main").unwrap().actions.iter().find(|action| action.id == "withArgs").unwrap().args;
    assert!(resolved_execute_args(defs, &serde_json::Map::new()).is_none());
    let mut staged = serde_json::Map::new();
    staged.insert("name".into(), serde_json::json!("hello"));
    let merged = resolved_execute_args(defs, &staged).expect("executable");
    assert_eq!(merged.get("name"), Some(&serde_json::json!("hello")));
    assert_eq!(merged.get("flag"), Some(&serde_json::json!(true)));
}

#[test]
fn staging_stage_and_reset_roundtrip() {
    let mut shell = shell();
    shell.commands.stage_arg("main", "withArgs", "name", serde_json::json!("x"));
    shell.commands.stage_arg("main", "withArgs", "flag", serde_json::json!(false));
    let staged = shell.commands.staged_map_for("main", "withArgs");
    assert_eq!(staged.get("name"), Some(&serde_json::json!("x")));
    assert_eq!(staged.get("flag"), Some(&serde_json::json!(false)));
    shell.commands.reset_staged_args("main", "withArgs");
    assert!(shell.commands.staged_map_for("main", "withArgs").is_empty());
}

/// 🔀️ Re-selecting the active utility deactivates it (the same update a re-click / Escape performs).
///
/// Switching to a different utility activates it.
#[test]
fn utility_activation_toggles_and_switches() {
    let mut shell = shell();
    shell.windows.toggle_active_utility("main", "utility.a", &mut shell.chrome, &mut shell.docs);
    assert_eq!(shell.windows.active_utility_for_window("main"), Some("utility.a"));
    shell.windows.toggle_active_utility("main", "utility.a", &mut shell.chrome, &mut shell.docs);
    assert_eq!(shell.windows.active_utility_for_window("main"), None);
    shell.windows.toggle_active_utility("main", "utility.a", &mut shell.chrome, &mut shell.docs);
    shell.windows.toggle_active_utility("main", "utility.b", &mut shell.chrome, &mut shell.docs);
    assert_eq!(shell.windows.active_utility_for_window("main"), Some("utility.b"));
}

/// 🛂️ No active utility → actions enabled.
///
/// utility.a defaults to `allows_actions_while_active = false` → actions gated.
///
/// utility.b sets the flag true → actions stay enabled.
#[test]
fn active_utility_gates_actions_unless_allowed() {
    let app = actions_utilities_app();
    let mut shell = shell();
    assert!(shell.windows.actions_enabled_for_window(&app, "main"));
    shell.windows.toggle_active_utility("main", "utility.a", &mut shell.chrome, &mut shell.docs);
    assert!(!shell.windows.actions_enabled_for_window(&app, "main"));
    shell.windows.toggle_active_utility("main", "utility.a", &mut shell.chrome, &mut shell.docs);
    shell.windows.toggle_active_utility("main", "utility.b", &mut shell.chrome, &mut shell.docs);
    assert!(shell.windows.actions_enabled_for_window(&app, "main"));
}

/// 🪶️ Closing a window is ONE lane: the layout collapses by React's rule, focus moves to the first
/// remaining window, the layout is persisted, and whatever the window hosted is torn down —
/// `closeWindow` + `onWindowClose` (`🎨️Canvas/🟦️.tsx:1475-1485`, `🏛️ShellHost/🟦️.tsx:10533-10555`).
#[test]
fn closing_a_window_collapses_persists_and_refocuses_in_one_lane() {
    let mut shell = shell();
    shell.dock.dock.root = DockNode::Row(vec![(stack_with("plug-1"), 0.5), (stack_tabs(&["main", "side"], "main"), 0.5)]);
    shell.dock.dock.sync_active_window("plug-1");
    shell.windows.active_window_id = Some("plug-1".into());
    assert!(SessionCtx::of(&mut shell).close_dock_window("plug-1"), "close is never a no-operation");
    assert_eq!(shell.dock.dock.root, stack_tabs(&["main", "side"], "main"), "the emptied slot is pruned and its sibling hoisted");
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("main"), "React refocuses children[0]");
    assert!(shell.dock.layout_override.is_some(), "every close persists the collapsed layout");
    assert!(!SessionCtx::of(&mut shell).close_dock_window("plug-1"), "a window that is already gone refuses");
}

/// 🪶️ The spawned-app teardown React's `onWindowClose` performs: the closed window's entry leaves the
/// panel and `activeSpawnedId` re-points at whatever is left, so the plugin instance behind it can be
/// destroyed instead of running forever behind a window that no longer exists.
#[test]
fn closing_a_spawned_window_takes_its_panel_entry_and_repoints_the_active_one() {
    let entry = |id: &str, instance: u32| semio_framework_os_renderer_wgpu_shell_docs::SpawnedAppEntry { id: id.into(), plugin_id: "plug".into(), instance_id: instance, app_id: "app".into(), label: "Plug".into(), breadcrumb: vec!["plug".into()] };
    let mut panel = semio_framework_os_renderer_wgpu_shell_docs::SpacePanelState { active_panel_tab: "workbench".into(), spawned_apps: vec![entry("plug-1", 7), entry("plug-2", 8)], active_spawned_id: Some("plug-1".into()) };
    let closed = take_spawned_entry(&mut panel, "plug-1").expect("the closed window owned a spawned app");
    assert_eq!(closed.instance_id, 7, "the instance id the guest must be told to destroy");
    assert_eq!(panel.spawned_apps.iter().map(|entry| entry.id.as_str()).collect::<Vec<_>>(), vec!["plug-2"]);
    assert_eq!(panel.active_spawned_id.as_deref(), Some("plug-2"));
    assert!(take_spawned_entry(&mut panel, "main").is_none(), "a plain window hosts nothing to tear down");
    let last = take_spawned_entry(&mut panel, "plug-2").expect("last spawned entry");
    assert_eq!(last.instance_id, 8);
    assert_eq!(panel.active_spawned_id, None);
}
