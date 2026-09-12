//! 🪟️ The wgpu half of the shared mode-layout law
//! (`🧫️fixtures/🪟️app-mode-layouts/🔣️.json`, TS twin
//! `🧑‍🎨engine/🧪️tests/🪟️app-mode-layouts/🟦️.ts`): for each authored app + active mode, the dock must
//! place EVERY window instance the layout names, at the authored fractions, with the authored
//! focus — and it must re-seed when the mode changes. The shell's chrome walk paints exactly the
//! rects this solves (`ShellState::plan_dock_windows`), so a mode whose layout the dock ignores is
//! a mode whose second window has no engine surface.

use super::*;
use semio_framework::{AppDefinition, AppRole, ArtifactDialect, ModeDefinition, Modes, WindowKindDefinition, WindowKinds};
use serde_json::Value;
use ui_wgpu::wgpu::{LocalizedLabel, NamedLayout, WindowOptions};

//#region 🧫️FixtureModel
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LayoutFixture {
    canvas: FixtureRect,
    cases: Vec<FixtureCase>,
}

#[derive(serde::Deserialize, Clone, Copy)]
struct FixtureRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureCase {
    id: String,
    active_mode_id: String,
    app: FixtureApp,
    expected: FixtureExpected,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureApp {
    default_mode_id: String,
    modes: Vec<FixtureMode>,
    window_kinds: Vec<FixtureWindowKind>,
    default_layout: Option<WindowLayout>,
    named_layouts: Vec<FixtureNamedLayout>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureMode {
    id: String,
    #[serde(default)]
    layout_id: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureWindowKind {
    id: String,
    label: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureNamedLayout {
    id: String,
    layout: WindowLayout,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureExpected {
    resolved_layout_id: Option<String>,
    axis: String,
    window_instances: Vec<FixtureWindowInstance>,
    active_window_id: String,
    stacks: Vec<FixtureStack>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureWindowInstance {
    id: String,
    window_kind_id: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureStack {
    path: String,
    windows: Vec<String>,
    active: String,
    fraction: f32,
    rect: [f32; 4],
}
//#endregion 🧫️FixtureModel

//#region 🧫️FixtureApp
fn fixture_window_kind(kind: &FixtureWindowKind) -> WindowKindDefinition {
    WindowKindDefinition {
        id: kind.id.clone(),
        label: LocalizedLabel::data(&kind.label),
        body_key: format!("{}.body", kind.id),
        surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
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
    }
}

fn fixture_app(app: &FixtureApp) -> AppDefinition {
    AppDefinition {
        id: "s.test.mode-layouts@1/*#editor".into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.mode-layouts".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Mode layouts"),
        breadcrumb: vec!["semio".into()],
        icon_id: None,
        controller_id: "mode-layouts".into(),
        modes: Modes::try_from(
            app.modes
                .iter()
                .map(|mode| ModeDefinition { id: mode.id.clone(), label: LocalizedLabel::data(&mode.id), icon_id: "pencil".into(), tools: vec![], layout_id: mode.layout_id.clone(), commands: vec![] })
                .collect::<Vec<_>>(),
        )
        .expect("every fixture app declares at least one mode"),
        default_mode_id: app.default_mode_id.clone(),
        window_kinds: WindowKinds::try_from(app.window_kinds.iter().map(fixture_window_kind).collect::<Vec<_>>()).expect("every fixture app declares at least one window kind"),
        panel_tabs: vec![],
        keybindings: vec![],
        interactions: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        named_layouts: app
            .named_layouts
            .iter()
            .map(|named| NamedLayout { id: named.id.clone(), label: named.id.clone(), icon_id: None, layout: named.layout.clone(), origin: "builtin".into(), group_path: None })
            .collect(),
        default_layout: app.default_layout.clone(),
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

fn fixture() -> LayoutFixture {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🪟️app-mode-layouts/🔣️.json")).expect("the shared mode-layout fixture parses")
}

fn axis_name(node: &DockNode) -> &'static str {
    match node {
        DockNode::Row(_) => "row",
        DockNode::Column(_) => "column",
        DockNode::Stack { .. } => "stack",
    }
}

fn child_fractions(node: &DockNode) -> Vec<f32> {
    let children = match node {
        DockNode::Row(children) | DockNode::Column(children) => children,
        DockNode::Stack { .. } => return vec![1.0],
    };
    let total: f32 = children.iter().map(|(_, size)| *size).sum::<f32>().max(0.001);
    children.iter().map(|(_, size)| size / total).collect()
}

fn assert_close(actual: f32, expected: f32, what: &str, case: &str) {
    assert!((actual - expected).abs() < 0.01, "{case}: {what} is {actual}, fixture declares {expected}");
}
//#endregion 🧫️FixtureApp

//#region 🧪️Laws
/// 🪟️ Every window instance the ACTIVE mode's layout names is placed, at the authored fractions, in
/// the authored tree — the law the wgpu chrome walk's `plan_dock_windows` reads straight off.
#[test]
fn every_window_instance_of_the_active_mode_is_laid_out_at_its_authored_fraction() {
    let fixture = fixture();
    let canvas = Rect::new(fixture.canvas.x, fixture.canvas.y, fixture.canvas.w, fixture.canvas.h);
    for case in &fixture.cases {
        let app = fixture_app(&case.app);
        let resolved = semio_framework::resolve_layout_for_mode(&app, &case.active_mode_id).expect("every fixture case resolves a layout");
        let named_match = case.app.named_layouts.iter().find(|named| named.layout == resolved).map(|named| named.id.clone());
        assert_eq!(named_match, case.expected.resolved_layout_id, "{}: resolved layout identity", case.id);

        let mut dock = DockState::from_app(&app, None);
        dock.apply_layout_diff(&resolved);
        assert_eq!(axis_name(&dock.root), case.expected.axis, "{}: root axis", case.id);
        assert_eq!(dock.active_window_id.as_deref(), Some(case.expected.active_window_id.as_str()), "{}: focused window", case.id);

        let instances = dock.window_instances();
        let expected_instances: Vec<(String, String)> = case.expected.window_instances.iter().map(|row| (row.id.clone(), row.window_kind_id.clone())).collect();
        assert_eq!(instances, expected_instances, "{}: window instances, in layout order", case.id);

        let fractions = child_fractions(&dock.root);
        assert_eq!(fractions.len(), case.expected.stacks.len(), "{}: one fraction per declared stack", case.id);
        for (fraction, stack) in fractions.iter().zip(case.expected.stacks.iter()) {
            assert_close(*fraction, stack.fraction, &format!("fraction of stack {:?}", stack.path), &case.id);
        }

        let frames = dock.stack_frame_rects(canvas);
        assert_eq!(frames.len(), case.expected.stacks.len(), "{}: one solved frame per declared stack", case.id);
        for ((path, rect, active), stack) in frames.iter().zip(case.expected.stacks.iter()) {
            assert_eq!(path_str(path), stack.path, "{}: stack path", case.id);
            assert_eq!(active, &stack.active, "{}: active window of stack {:?}", case.id, stack.path);
            assert_eq!(dock.stack_windows_at_path(path).unwrap_or_default(), stack.windows, "{}: windows of stack {:?}", case.id, stack.path);
            for (index, (actual, expected)) in [(rect.x, stack.rect[0]), (rect.y, stack.rect[1]), (rect.w, stack.rect[2]), (rect.h, stack.rect[3])].iter().enumerate() {
                assert_close(*actual, *expected, &format!("rect[{index}] of stack {:?}", stack.path), &case.id);
            }
        }
    }
    eprintln!("[DEBUG] wgpu dock honoured every mode-layout fixture case");
}

/// 🔁️ A mode change RE-SEEDS the layout: switching from generate back to edit must restore edit's
/// own two windows, not keep generate's three. This is the transition
/// `ShellState::handle_control_command`'s `playground.navbar.modes.*` arm drives.
#[test]
fn a_mode_change_reseeds_the_dock_with_the_new_modes_windows() {
    let fixture = fixture();
    let case = fixture.cases.iter().find(|case| case.id == "generation3d-edit-is-a-row-split").expect("the edit case");
    let generate = fixture.cases.iter().find(|case| case.id == "generation3d-generate-is-the-named-three-pane-row").expect("the generate case");
    let app = fixture_app(&case.app);

    let mut dock = DockState::from_app(&app, None);
    let edit_windows: Vec<String> = dock.window_instances().into_iter().map(|(id, _)| id).collect();
    assert_eq!(edit_windows, vec!["procedural-main".to_string(), "procedural-preview".to_string()]);

    let generate_layout = semio_framework::resolve_layout_for_mode(&app, "generate").expect("generate resolves its named layout");
    dock.apply_layout_diff(&generate_layout);
    let generate_windows: Vec<String> = dock.window_instances().into_iter().map(|(id, _)| id).collect();
    assert_eq!(generate_windows, generate.expected.window_instances.iter().map(|row| row.id.clone()).collect::<Vec<_>>(), "generate mode replaces edit's windows outright");
    assert_eq!(dock.active_window_id.as_deref(), Some(generate.expected.active_window_id.as_str()), "focus lands in the new mode");

    let edit_layout = semio_framework::resolve_layout_for_mode(&app, "edit").expect("edit resolves the default layout");
    dock.apply_layout_diff(&edit_layout);
    let back: Vec<String> = dock.window_instances().into_iter().map(|(id, _)| id).collect();
    assert_eq!(back, edit_windows, "switching back restores edit's own windows");
    eprintln!("[DEBUG] wgpu dock re-seeded across edit → generate → edit");
}

/// 📐️ `stack_body_rects` is `stack_frame_rects` minus the tab cap and nothing else — the separation
/// that lets the fixture state geometry without pinning a theme metric, and the reason the fixture's
/// rects are the frames the chrome walk then insets.
#[test]
fn every_painted_body_is_its_solved_frame_minus_the_tab_cap() {
    let fixture = fixture();
    let canvas = Rect::new(fixture.canvas.x, fixture.canvas.y, fixture.canvas.w, fixture.canvas.h);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    for case in &fixture.cases {
        let app = fixture_app(&case.app);
        let resolved = semio_framework::resolve_layout_for_mode(&app, &case.active_mode_id).expect("a layout");
        let mut dock = DockState::from_app(&app, None);
        dock.apply_layout_diff(&resolved);
        let labels: HashMap<String, String> = dock.window_instances().into_iter().map(|(id, kind)| (id, kind)).collect();
        let frames = dock.stack_frame_rects(canvas);
        let bodies = dock.stack_body_rects(canvas, &theme, &labels, &mut atlas);
        assert_eq!(frames.len(), bodies.len(), "{}: one body per frame", case.id);
        for ((frame_path, frame, _), (body_path, body, _)) in frames.iter().zip(bodies.iter()) {
            assert_eq!(frame_path, body_path, "{}: bodies and frames walk in the same order", case.id);
            assert!(body.w <= frame.w + 0.01 && body.h <= frame.h + 0.01, "{}: body {body:?} escapes its frame {frame:?}", case.id);
            assert!(body.x >= frame.x - 0.01 && body.y >= frame.y - 0.01, "{}: body {body:?} starts before its frame {frame:?}", case.id);
            assert!(body.w > 0.0 && body.h > 0.0, "{}: body {body:?} is empty — nothing would be laid out in it", case.id);
        }
    }
    eprintln!("[DEBUG] wgpu dock bodies stay inside their solved frames");
}

/// 🧫️ The fixture is the one both targets read, so its own arithmetic has to close: fractions sum to
/// one per case, the declared rects tile the canvas without gap or overlap along the root axis, and
/// every declared window instance names a declared window kind.
#[test]
fn the_shared_fixture_declares_a_closed_layout_for_every_case() {
    let raw: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪟️app-mode-layouts/🔣️.json")).expect("fixture parses as json");
    assert!(raw.get("provenance").is_some(), "the fixture names where each authored layout comes from");
    let fixture = fixture();
    let canvas = fixture.canvas;
    for case in &fixture.cases {
        let sum: f32 = case.expected.stacks.iter().map(|stack| stack.fraction).sum();
        assert_close(sum, 1.0, "declared fractions sum", &case.id);
        let declared_kinds: Vec<&str> = case.app.window_kinds.iter().map(|kind| kind.id.as_str()).collect();
        for instance in &case.expected.window_instances {
            assert!(declared_kinds.contains(&instance.window_kind_id.as_str()), "{}: instance {} names an undeclared window kind", case.id, instance.id);
        }
        let area: f32 = case.expected.stacks.iter().map(|stack| stack.rect[2] * stack.rect[3]).sum();
        assert_close(area, canvas.w * canvas.h, "declared rects tile the canvas", &case.id);
        assert!(
            case.expected.stacks.iter().any(|stack| stack.windows.contains(&case.expected.active_window_id)),
            "{}: the focused window is in one of the declared stacks",
            case.id
        );
    }
    eprintln!("[DEBUG] shared mode-layout fixture is internally closed across {} cases", fixture.cases.len());
}
//#endregion 🧪️Laws
