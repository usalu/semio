//! 🐚️ Shell fixtures of the test bins: the shells, apps and documents several laws build their worlds from.
//!
//! 🧭️ Moved out of the Shell law files that held them, so that laws in different bins name them through this crate.

extern crate infinite_canvas as infinite_world;
use infinite_world::world::{enqueue_world3d_event, WorldInteractionIntent};
use semio_framework::{
    ActionArgDef, ActionDefinition, ActionKind, AppDefinition, AppRole, CommandDefinition, ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, PluginManifest, UtilityDefinition, ViewModel, WindowKindDefinition, WindowKinds,
};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap;
use semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState;
use semio_framework_os_renderer_wgpu_program_bridge::{PluginHostConfig, ProgramBridgeEntry};
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels;
use semio_framework_os_renderer_wgpu_shell_dock::{PanelAnchorState, ShellDock};
use semio_framework_os_renderer_wgpu_shell_docs::{SpacePanelState, SpawnedAppEntry};
use semio_framework_os_renderer_wgpu_shell_input::{ContextMenuState, PointerCapture};
use semio_framework_os_renderer_wgpu_shell_prefs::ActiveSession;
use semio_framework_os_renderer_wgpu_shell_session::{panel_json, SessionCtx};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_view_paint::SHELL_WINDOW_PAINT_OPPORTUNITIES;
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_shell_windows::PanelProjection;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{render_ui_document_step, UiDocumentFrameCursor};
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_layout::layout::{WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use serde_json::Value;
use std::collections::HashMap;
use ui_contract::{UiDocumentLease, UiText};
use semio_framework_ui_declaration::layout::{ActionDescriptor, WindowEngagement};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::utilities::UtilityNode;

pub fn host_test_apps() -> (AppDefinition, AppDefinition) {
    let mut home = test_app(Vec::new(), Vec::new());
    home.id = "home".into();
    home.controller_id = "space.home".into();
    home.panel_tabs = vec![PanelTabDefinition { kind: PanelTabKind::App("home-library".into()), label: LocalizedLabel::data("Home Library"), group: PanelGroup::Display, body_key: Some("home.library".into()), children: vec![] }];
    let mut studio = test_app(Vec::new(), Vec::new());
    studio.id = "studio".into();
    studio.controller_id = "space.studio".into();
    studio.panel_tabs = vec![
        PanelTabDefinition { kind: PanelTabKind::App("s-play-catalogue".into()), label: LocalizedLabel::data("Catalogue"), group: PanelGroup::Workbench, body_key: Some("s.play.catalogue".into()), children: vec![] },
        PanelTabDefinition { kind: PanelTabKind::App("s-play-inspector".into()), label: LocalizedLabel::data("Inspector"), group: PanelGroup::Details, body_key: Some("s.play.inspector".into()), children: vec![] },
        PanelTabDefinition {
            kind: PanelTabKind::App("studio-settings".into()),
            label: LocalizedLabel::data("Studio Settings"),
            group: PanelGroup::Settings,
            body_key: None,
            children: vec![PanelTabDefinition { kind: PanelTabKind::App("studio-settings-leaf".into()), label: LocalizedLabel::data("Settings"), group: PanelGroup::Settings, body_key: Some("studio.settings".into()), children: vec![] }],
        },
    ];
    (home, studio)
}

pub fn host_test_shell() -> ShellState {
    let (home, studio) = host_test_apps();
    let manifest = semio_framework::PluginManifest {
        plugin_id: "space".into(),
        label: "Space".into(),
        version: "1".into(),
        apps: vec![home, studio.clone()],
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![],
        artifact_kinds: vec![],
        hosted_artifact_kinds: Vec::new(),
        dependencies: vec![],
        contributions: vec![],
    };
    let bridge = ProgramBridgeEntry::from_wasm("space".into(), None, None, std::path::PathBuf::from("missing-host-panel-guest.wasm"), manifest).expect("nonrunnable host bridge");
    let mut shell = ShellState::new(vec![bridge], "space".into(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    static HOST: PluginHostConfig = PluginHostConfig { plugin_id: "space", landing_app_id: "home", host_app_id: "studio" };
    shell.plugins.host_capability = Some(&HOST);
    shell.plugins.space_mode = true;
    let base = SpacePanelState {
        active_panel_tab: "s-play-catalogue".into(),
        spawned_apps: vec![
            SpawnedAppEntry { id: "spawned-model".into(), plugin_id: "model".into(), instance_id: 41, app_id: "model@1/any#editor".into(), label: "Model".into(), breadcrumb: vec!["Workspace".into(), "Model".into()] },
            SpawnedAppEntry { id: "spawned-note".into(), plugin_id: "note".into(), instance_id: 42, app_id: "note@1/any#editor".into(), label: "Note".into(), breadcrumb: vec!["Workspace".into(), "Note".into()] },
        ],
        active_spawned_id: Some("spawned-model".into()),
    };
    let mut view_state = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    view_state.active_mode_id = Some(studio.default_mode_id.clone());
    view_state.active_window_kind_id = Some(studio.window_kinds.first().id.clone());
    view_state.panel_json = Some(panel_json(&base).expect("strict base panel"));
    shell.docs.session = Some(ActiveSession { plugin_id: "space".into(), instance_id: 77, app: studio, view_state });
    shell
}

pub fn test_app(commands: Vec<CommandDefinition>, mode_commands: Vec<CommandDefinition>) -> AppDefinition {
    AppDefinition {
        id: "test-app".into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.app".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Test App"),
        breadcrumb: vec!["semio".into(), "test".into()],
        icon_id: None,
        controller_id: "test".into(),
        modes: Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: mode_commands }),
        default_mode_id: "default".into(),
        window_kinds: WindowKinds::try_from(vec![WindowKindDefinition {
            initial_utility_id: None,
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
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
        .expect("non-empty"),
        panel_tabs: vec![PanelTabDefinition { kind: PanelTabKind::App("tab".into()), label: LocalizedLabel::data("Tab"), group: PanelGroup::Workbench, body_key: Some("tab.body".into()), children: vec![] }],
        keybindings: vec![],
        actions: vec![],
        utilities: vec![],
        tools: vec![],
        commands,
        interactions: vec![],
        named_layouts: vec![],
        default_layout: None,
        terminologies: vec!["de".into()],
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

/// 🪟️ An app whose default layout is puzzle3d's own two-pane row split — one window KIND opened
/// twice, each pane carrying the authored instance title React reads its cap from.
pub fn split_pane_app() -> AppDefinition {
    let mut app = test_app(Vec::new(), Vec::new());
    app.id = "pane-fixture".into();
    app.controller_id = "pane.controller".into();
    app.utilities = vec![
        UtilityDefinition { id: "pane.transform".into(), label: LocalizedLabel::data("Transform"), icon_id: "move".into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: true, run: None },
        UtilityDefinition { id: "pane.brush".into(), label: LocalizedLabel::data("Brush"), icon_id: "brush".into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: true, run: None },
    ];
    let window = |instance: &str, title: &str, size: f64| {
        WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: Some(size),
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "main".into(), title: Some(title.into()), instance_id: Some(instance.into()), template_id: None, corner: None }],
        })
    };
    app.default_layout = Some(WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![window("pane-top", "Top", 33.333), window("pane-perspective", "Perspective", 66.667)] }) });
    app
}

pub fn split_pane_shell() -> ShellState {
    let app = split_pane_app();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.docs.session = Some(ActiveSession { plugin_id: "pane".into(), instance_id: 1, app, view_state: ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
    SessionCtx::of(&mut shell).sync_dock();
    shell.windows.active_window_id = Some("pane-top".into());
    shell
}

/// 🗣️ The split-pane fixture with ONE engagement payload on `pane-top` — a status line (React's
/// `engagement` prop) and a typed line (React's `search` prop), which is the minimum that mounts the
/// two top panes at all.
pub fn engaged_pane_shell() -> ShellState {
    let mut shell = split_pane_shell();
    shell.windows.window_engagements.insert(
        "pane-top".into(),
        WindowEngagement {
            session_active: None,
            options: None,
            input: Some(semio_framework_ui_layout::layout::WindowEngagementInput {
                id: Some("pane-engagement".into()),
                value: Some(String::new()),
                placeholder: None,
                disabled: None,
                on_change: None,
                on_submit: None,
                on_repeat_last: None,
                on_abort: None,
            }),
            control: None,
            controls: None,
            status: Some(vec![semio_framework_ui_layout::layout::WindowEngagementStatus { id: "pane.status".into(), text: "Ready".into() }]),
            possible_engagements: None,
        },
    );
    shell
}

/// 🎬️ puzzle3d's shape in miniature: one window kind opened as two instances, whose actions span
/// three categories and include one zero-arg verb, one arg-carrying verb and one framework-reserved
/// verb — the three rows every law below needs to tell apart.
pub fn actions_shell() -> ShellState {
    let mut shell = split_pane_shell();
    let session = shell.docs.session.as_mut().expect("the split-pane fixture carries a session");
    let kind = session.app.window_kinds.first_mut();
    kind.actions = vec![
        ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("addObjectKind", LocalizedLabel::data("Add Object Kind"), ActionKind::Mutation) },
        ActionDefinition {
            in_palette: true,
            category: Some("create".into()),
            args: vec![ActionArgDef { required: true, ..ActionArgDef::text("kind", LocalizedLabel::data("Kind")) }, ActionArgDef::number("count", LocalizedLabel::data("Count"))],
            ..ActionDefinition::new_catalog("openAddObjectDialog", LocalizedLabel::data("Add Object"), ActionKind::Mutation)
        },
        ActionDefinition { in_palette: true, category: Some("history".into()), ..ActionDefinition::new_catalog("undo", LocalizedLabel::data("Undo"), ActionKind::View) },
        ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("worldPointerDown", LocalizedLabel::data("Pointer"), ActionKind::Interaction) },
    ];
    shell
}

pub fn mouse_pointer(id: u64) -> ui_render::PointerInfo {
    ui_render::PointerInfo { id: ui_render::PointerId(id), kind: ui_render::PointerKind::Mouse, pressure: None, tilt: None }
}

pub fn pointer_interaction(shell: ShellState, input: InputState<ActionDescriptor>) -> semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState {
    semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState {
        shell,
        input,
        theme: Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: PointerCapture::default(),
        modifiers: PointerModifiers::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        #[cfg(not(target_arch = "wasm32"))]
        last_sync_pump_ms: 0.0,
    }
}

pub fn retained_world_sequence_probe(scenario: &str) {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.dock.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock.dock_tabs = ShellDock::default();
    let mut active_documents = Vec::new();
    let mut retired_documents = Vec::new();
    let mut row_sources = Vec::new();
    let scene = semio_framework_ui_scene_world3d::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    for row in fixture["surfaces"].as_array().unwrap() {
        let window = row["windowId"].as_str().unwrap();
        let surface = row["surfaceId"].as_str().unwrap();
        let bounds = row["bounds"].as_array().unwrap();
        let bounds = Rect::new(bounds[0].as_f64().unwrap() as f32, bounds[1].as_f64().unwrap() as f32, bounds[2].as_f64().unwrap() as f32, bounds[3].as_f64().unwrap() as f32);
        let document = shell.windows.publish_surface_records(window, vec![world3d_pointer_record(1, "world", &scene)]).unwrap();
        active_documents.push(document);
        row_sources.push((window.to_string(), surface.to_string(), bounds));
    }
    shell.dock.dock_window_plan = row_sources.iter().map(|(window, _, bounds)| (window.clone(), *bounds)).collect();
    let paint_rows: Vec<_> = row_sources.iter().zip(active_documents.iter()).map(|((window, _, bounds), document)| (window.as_str(), document, *bounds)).collect();
    let input = paint_tree_pointer_documents(&mut shell, &paint_rows);
    let mut rows = Vec::new();
    for (window, surface, bounds) in row_sources {
        let host_id = shell.surfaces.world3d_states.iter().find_map(|(host_id, state)| (state.surface_id == surface).then(|| host_id.clone())).expect("painted world admits one exact runtime host");
        let state = shell.surfaces.world3d_states.get_mut(&host_id).expect("painted world retains its exact runtime interaction owner");
        assert_eq!(state.surface_id, surface, "runtime host preserves the document wire surface");
        let before = infinite_world::world::enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_leave(-1.0, -1.0)).unwrap();
        rows.push((window, surface, host_id, bounds, before));
    }
    let mut interaction = pointer_interaction(shell, input);
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == scenario).unwrap();
    let point = |name: &str| (fixture[name][0].as_f64().unwrap() as f32, fixture[name][1].as_f64().unwrap() as f32);
    for step in case["steps"].as_array().unwrap() {
        match step.as_str().unwrap() {
            "press" | "release" | "secondaryPress" | "secondaryRelease" => {
                let down = matches!(step.as_str().unwrap(), "press" | "secondaryPress");
                let (x, y) = point(if down { "press" } else { "release" });
                let button = if step.as_str().unwrap().starts_with("secondary") { 2 } else { 0 };
                semio_framework_async::block_on(interaction.handle_pointer_button(mouse_pointer(77), x, y, down, button, PointerModifiers::default()));
            }
            "move" => {
                let (x, y) = point("release");
                semio_framework_async::block_on(interaction.handle_pointer_move(mouse_pointer(77), x, y, true, 0, PointerModifiers::default()));
            }
            "wheel" => {
                let (x, y) = point("press");
                let target = interaction.shell.input.scene_pointer_target_at(x, y, &interaction.input, &interaction.theme).unwrap();
                interaction.apply_scene_wheel(&target, x, y, 1.0, &PointerModifiers::default()).unwrap();
            }
            "chrome" => {
                let hits = interaction.input.hits().to_vec();
                for hit in hits {
                    interaction.input.register_hit(hit);
                }
                interaction.input.register_hit(HitTarget { rect: rows.last().unwrap().3, event: None, control_id: Some("shell.fixture.chrome".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                interaction.input.publish_hits();
            }
            "modal" => {
                interaction.shell.input.context_menu = Some(ContextMenuState::default());
                let paint_rows: Vec<_> = rows.iter().zip(active_documents.iter()).map(|((window, _, _, bounds, _), document)| (window.as_str(), document, *bounds)).collect();
                interaction.input = paint_tree_pointer_documents(&mut interaction.shell, &paint_rows);
            }
            "cancel" => interaction.handle_pointer_cancel(mouse_pointer(77)),
            "foreignPress" | "foreignRelease" => {
                let (x, y) = point("release");
                semio_framework_async::block_on(interaction.handle_pointer_button(mouse_pointer(78), x, y, step == "foreignPress", 0, PointerModifiers::default()));
            }
            "foreignCancel" => interaction.handle_pointer_cancel(mouse_pointer(78)),
            "refresh" | "replace" => {
                let window = rows.last().expect("the refreshed row remains mounted").0.clone();
                let key = if step == "replace" { "replacement-world" } else { "world" };
                let refreshed = semio_framework_ui_scene_world3d::World3dScene::base(serde_json::json!({ "position": [5.0, 5.0, 5.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
                let document = interaction.shell.windows.publish_surface_records(&window, vec![world3d_pointer_record(1, key, &refreshed)]).unwrap();
                retired_documents.push(std::mem::replace(active_documents.last_mut().expect("the refreshed row owns a document"), document));
                let paint_rows: Vec<_> = rows.iter().zip(active_documents.iter()).map(|((window, _, _, bounds, _), document)| (window.as_str(), document, *bounds)).collect();
                interaction.input = paint_tree_pointer_documents(&mut interaction.shell, &paint_rows);
            }
            _ => unreachable!(),
        }
    }
    let menu_open = interaction.shell.input.context_menu.is_some();
    let active_window = interaction.shell.windows.active_window_id.clone();
    let mut observed = Vec::new();
    for (_, surface, host_id, _, before) in &rows {
        let state = interaction.shell.surfaces.world3d_states.get_mut(host_id).expect("the exact pre-replacement runtime host remains available through captured release");
        let after = infinite_world::world::enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_leave(-1.0, -1.0)).unwrap();
        observed.push((surface.clone(), after - before - 1));
    }
    let fault = interaction.input.take_action_step().err();
    interaction.shell.dock.dock_window_plan.clear();
    SessionCtx::of(&mut interaction.shell).sync_engine_surface_states();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if interaction.shell.surfaces.retired_world3d_states.is_empty() {
            break;
        }
        interaction.shell.surfaces.advance_world3d_retirement_step();
    }
    for mut document in retired_documents.into_iter().chain(active_documents) {
        while !document.close_step() {}
    }
    assert!(fault.is_none(), "pointer admission fault: {fault:?}");
    for (surface, count) in observed {
        let expected = if Some(surface.as_str()) == fixture["expectedOwner"].as_str() { case["events"].as_array().unwrap().len() as u64 } else { 0 };
        assert_eq!(count, expected, "{surface}: exact retained owner receives the actual press and captured release");
    }
    assert_eq!(menu_open, case["menuOpen"].as_bool().unwrap(), "{scenario} context menu state");
    let expected_window = if case["steps"].as_array().unwrap().iter().any(|step| step == "modal") { None } else { fixture["expectedOwner"].as_str() };
    assert_eq!(active_window.as_deref(), expected_window, "{scenario} activates the exact receiving window");
    assert!(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::captured_scene_pointer(ui_render::PointerId(77)).is_none(), "{scenario} released its captured owner");
}

pub fn tree_pointer_record(id: u64, key: &str, component: ui_contract::Component, children: &[u64], action: Option<&str>) -> ui_contract::UiNodeRecord {
    let mut child_ids = ui_contract::UiNodeChildren::default();
    for child in children {
        child_ids.try_push(ui_contract::UiNodeId(*child)).expect("tree pointer fixture child");
    }
    let mut bindings = ui_contract::UiNodeBindings::default();
    if let Some(action) = action {
        bindings
            .try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1("s.test.tree", action).expect("bounded tree pointer action"), args: None, capability: None })
            .expect("tree pointer fixture binding");
    }
    ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str(key).expect("bounded tree pointer key"),
        component,
        layout: PanelProjection::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings,
        menu: None,
        children: child_ids,
    }
}

pub fn paint_tree_pointer_documents(shell: &mut ShellState, documents: &[(&str, &UiDocumentLease, Rect)]) -> InputState<ActionDescriptor> {
    let mut caller = semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = std::mem::take(&mut shell.surfaces.world3d_states);
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    for (surface, document, body) in documents {
        let mut cursor = UiDocumentFrameCursor::default();
        let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
            let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
            let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts {
                chrome_labels: semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels::english(),
                world3d_states: &mut world3d_states,
                world_resources: &mut world_resources,
                window_id: surface,
            };
            let done = caller.turn(|cx| render_ui_document_step(&mut cursor, document, *body, &mut ctx, surface, "s.test.tree", semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Handle, &mut hosts, cx));
            assert!(done || !cursor.terminal_is_fault(), "the tree pointer document faulted in phase {}", cursor.phase_name());
            done
        });
        assert!(complete, "the tree pointer document painted within its opportunity ceiling");
        shell.input.register_retained_body_hits(surface, *body, &mut input);
    }
    shell.surfaces.world3d_states = world3d_states;
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    caller.retire_world(&mut world_resources);
    caller.finish();
    input
}

pub fn world3d_pointer_record(id: u64, key: &str, scene: &semio_framework_ui_scene_world3d::World3dScene) -> ui_contract::UiNodeRecord {
    let surface = semio_framework_ui_scene_surface::encode(ui_contract::SurfaceKind::World3d, scene).expect("bounded World3d scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

pub fn paint_component_pointer_documents(shell: &mut ShellState, documents: &[(&str, &str, &UiDocumentLease, Rect)]) -> InputState<ActionDescriptor> {
    let mut caller = semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    for (window_id, controller_id, document, body) in documents {
        let mut cursor = UiDocumentFrameCursor::default();
        let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
            let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
            let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts {
                chrome_labels: semio_framework_os_renderer_wgpu_scene_state::SceneChromeLabels::english(),
                world3d_states: &mut world3d_states,
                world_resources: &mut world_resources,
                window_id,
            };
            let done = caller.turn(|cx| render_ui_document_step(&mut cursor, document, *body, &mut ctx, window_id, controller_id, semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Handle, &mut hosts, cx));
            assert!(done || !cursor.terminal_is_fault(), "the component pointer document faulted in phase {}", cursor.phase_name());
            done
        });
        assert!(complete, "the component pointer document painted within its opportunity ceiling");
        shell.input.register_retained_body_hits(window_id, *body, &mut input);
    }
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    caller.retire_world(&mut world_resources);
    caller.finish();
    input
}

/// 🧪️ The smallest app a boot selection can name — `id`, the `(dialect, role)` its id encodes, and
/// the mandatory window kind. The identity is PARSED out of the id rather than stamped, because
/// `select_boot_program`'s role projection resolves a sibling by `(dialect, role)` and a table of
/// apps that all claimed one synthetic dialect could never exercise it.
pub fn boot_app(app_id: &str) -> AppDefinition {
    let (dialect, role) = semio_framework::parse_surface_app_id(app_id).expect("the neutral boot corpus names an explicit dialect and role");
    AppDefinition {
        id: app_id.into(),
        role,
        dialect,
        label: LocalizedLabel::data(app_id),
        breadcrumb: vec!["semio".into()],
        icon_id: None,
        controller_id: "boot".into(),
        modes: Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
        default_mode_id: "default".into(),
        window_kinds: WindowKinds::try_from(vec![WindowKindDefinition {
            initial_utility_id: None,
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
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
        .expect("non-empty window kinds"),
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

pub fn boot_manifest(plugin_id: &str, app_ids: &[&str]) -> PluginManifest {
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
        hosted_artifact_kinds: Vec::new(),
        dependencies: vec![],
        contributions: vec![],
    }
}

pub fn tour_app(introduction: Option<semio_framework::onboarding::IntroductionDefinition>) -> AppDefinition {
    AppDefinition {
        id: "tour-app".into(),
        role: semio_framework::manifest::AppRole::Editor,
        dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.tour".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Tour App"),
        breadcrumb: vec!["semio".into(), "tour".into()],
        icon_id: None,
        controller_id: "tour".into(),
        modes: semio_framework::Modes::one(semio_framework::ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
        default_mode_id: "default".into(),
        window_kinds: semio_framework::WindowKinds::try_from(vec![semio_framework::WindowKindDefinition {
            initial_utility_id: None,
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
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
        .expect("non-empty"),
        panel_tabs: vec![],
        keybindings: vec![],
        actions: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        interactions: vec![],
        named_layouts: vec![],
        default_layout: None,
        terminologies: vec!["de".into()],
        terminology_breadcrumbs: HashMap::new(),
        introduction,
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

pub fn tour_introduction() -> semio_framework::onboarding::IntroductionDefinition {
    semio_framework::onboarding::IntroductionDefinition {
        title: LocalizedLabel::data("Welcome"),
        steps: vec![
            semio_framework::onboarding::IntroductionStepDefinition::new("one", LocalizedLabel::data("One"), LocalizedLabel::data("First")),
            semio_framework::onboarding::IntroductionStepDefinition::new("two", LocalizedLabel::data("Two"), LocalizedLabel::data("Second")),
        ],
    }
}

pub fn tour_shell(introduction: Option<semio_framework::onboarding::IntroductionDefinition>) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.docs.session = Some(ActiveSession { plugin_id: "tour".into(), instance_id: 1, app: tour_app(introduction), view_state: ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
    shell
}

/// 🧰️ Splits utility nodes into the four rails of the chrome by their category: selection, utilities, history and sync.
pub fn partition_utilities_by_category(utilities: &[UtilityNode]) -> [Vec<UtilityNode>; 4] {
    let mut buckets: [Vec<UtilityNode>; 4] = [vec![], vec![], vec![], vec![]];
    for utility in utilities {
        let idx = match utility.category() {
            semio_framework_ui_layout::utilities::UtilityCategory::Selection => 0,
            semio_framework_ui_layout::utilities::UtilityCategory::Utilities => 1,
            semio_framework_ui_layout::utilities::UtilityCategory::History => 2,
            semio_framework_ui_layout::utilities::UtilityCategory::Sync => 3,
        };
        buckets[idx].push(utility.clone());
    }
    buckets
}
