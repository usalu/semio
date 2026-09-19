//! ⏯️ LAW: the framework ToolRun panel the plugin runtime renders for a running run is visible and operable on wgpu. The
//! panel document is published as the `framework.panel.toolRun` retained surface and painted by the SAME stepped retained
//! paint every panel tab uses; its enabled buttons are pointer- and keyboard-reachable and dispatch the run's own actions
//! addressed by `runId` and `generation`, its disabled buttons dispatch nothing, and a run the shell has not seen reveals
//! the panel exactly once.
//!
//! Oracle: `🔌️plugin/🧫️fixtures/⏯️tool-run/🪧️panel-running.json`, the golden panel the Rust plugin law
//! `tool_run_panel_of_a_running_run_is_the_shell_fixture` pins and the React `🛠️ShellHelpers/⏯️tool-run-panel` law renders.

use super::*;

const PANEL_RUNNING: &str = include_str!("../../../../../../🔌️plugin/🧫️fixtures/⏯️tool-run/🪧️panel-running.json");

/// 🧾️ The fixture `BuiltNode` wire flattened into pre-order retained records, parents before children.
fn panel_records(node: &Value, records: &mut Vec<ui_contract::UiNodeRecord>) -> ui_contract::UiNodeId {
    let id = records.len() as u64 + 1;
    let mut wire = node.clone();
    let object = wire.as_object_mut().expect("a panel node is an object");
    object.insert("id".into(), Value::from(id));
    object.insert("children".into(), Value::Array(Vec::new()));
    records.push(serde_json::from_value(wire).expect("the panel node is a retained record"));
    let children: Vec<Value> = node["children"].as_array().expect("panel children").iter().map(|child| Value::from(panel_records(child, records).0)).collect();
    let index = (id - 1) as usize;
    let mut own = serde_json::to_value(&records[index]).expect("record wire");
    own["children"] = Value::Array(children);
    records[index] = serde_json::from_value(own).expect("the panel node keeps its children");
    ui_contract::UiNodeId(id)
}

fn fixture_records() -> Vec<ui_contract::UiNodeRecord> {
    let mut records = Vec::new();
    panel_records(&serde_json::from_str(PANEL_RUNNING).expect("the panel fixture parses"), &mut records);
    records
}

fn publish_panel(surface: &str) -> UiDocumentLease {
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation: 1, revision: ui_contract::UiRevision(1), root: Some(ui_contract::UiNodeId(1)), layout_epoch: 0 };
    UiDocumentLease::try_publish(SurfaceId::try_from(surface).expect("panel surface"), identity, fixture_records()).expect("the panel publishes")
}

struct PaintedPanel {
    input: InputState<ActionDescriptor>,
    instances: usize,
}

fn paint_panel(surface: &str, document: &UiDocumentLease) -> PaintedPanel {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut cursor = UiDocumentFrameCursor::default();
    let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        let mut hosts = crate::scenes::SceneEngineHosts { world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
        let done = render_ui_document_step(&mut cursor, document, Rect::new(0.0, 0.0, 360.0, 720.0), &mut ctx, surface, "s.test.tool-run", &mut hosts);
        assert!(done || !cursor.terminal_is_fault(), "the panel paint faulted in phase {}", cursor.phase_name());
        done
    });
    assert!(complete, "the panel paint completed within its opportunity ceiling (parked in {})", cursor.phase_name());
    let instances = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    crate::interpreter::register_retained_hit_targets(surface, &mut input);
    PaintedPanel { input, instances }
}

fn dispatched(input: &mut InputState<ActionDescriptor>) -> Vec<Value> {
    crate::collect_fixture_actions(input).into_iter().map(|action| {
        let args = dsl_value_as_json(action.args.as_ref().expect("action args"));
        serde_json::json!({ "action": action.action, "runId": args["runId"], "generation": args["generation"] })
    }).collect()
}

fn press(painted: &mut PaintedPanel, control: &str) {
    let hit = painted.input.staged_hits().iter().find(|hit| hit.control_id.as_deref() == Some(control)).unwrap_or_else(|| panic!("`{control}` registered a pointer target")).rect;
    let (x, y) = (hit.x + hit.w * 0.5, hit.y + hit.h * 0.5);
    crate::interpreter::dispatch_ui_event(FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, ui_wgpu::wgpu::UiEvent::PointerDown { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }, &mut painted.input);
    crate::interpreter::dispatch_ui_event(FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, ui_wgpu::wgpu::UiEvent::PointerUp { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }, &mut painted.input);
}

#[test]
fn tool_run_panel_of_a_running_run_paints_and_its_buttons_dispatch_the_run() {
    let mut document = publish_panel(FRAMEWORK_PANEL_TAB_TOOL_RUN_ID);
    let mut painted = paint_panel(FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, &document);
    assert!(painted.instances > 0, "the panel painted instances");
    let targets: Vec<String> = painted.input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect();
    println!("[STATS] tool-run panel targets {targets:?}");
    let control = |action: &str| targets.iter().find(|target| target.ends_with(&format!("framework.toolRun.1.{action}"))).cloned().unwrap_or_else(|| panic!("`{action}` is painted and pointer-reachable: {targets:?}"));
    let (pause, abort, step, finalize) = (control("toolRunPause"), control("toolRunAbort"), control("toolRunStep"), control("toolRunFinalize"));
    press(&mut painted, &pause);
    press(&mut painted, &step);
    press(&mut painted, &finalize);
    press(&mut painted, &abort);
    assert_eq!(
        dispatched(&mut painted.input),
        vec![
            serde_json::json!({ "action": "toolRunPause", "runId": "1", "generation": 0.0 }),
            serde_json::json!({ "action": "toolRunFinalize", "runId": "1", "generation": 0.0 }),
            serde_json::json!({ "action": "toolRunAbort", "runId": "1", "generation": 0.0 }),
        ],
        "enabled buttons dispatch the run's own action and disabled ones dispatch nothing"
    );
    while !document.close_step() {}
}

/// 🔢️ The arena slot of a freshly reconciled retained node — its record's publication index (`NodeId` keeps its index
/// private; its `Debug` spelling is the only reader).
fn arena_index(node: &ui_wgpu::wgpu::NodeId) -> usize {
    let spelled = format!("{node:?}");
    spelled.split("index: ").nth(1).and_then(|rest| rest.split(',').next()).and_then(|digits| digits.trim().parse().ok()).unwrap_or_else(|| panic!("a node id spells its index: {spelled}"))
}

#[test]
fn tool_run_panel_buttons_are_keyboard_reachable() {
    let surface = "framework.panel.toolRun/keyboard";
    let mut document = publish_panel(surface);
    let mut painted = paint_panel(surface, &document);
    let records = fixture_records();
    let mut shell = ShellState::new(vec![], "test".into());
    let mut focused = Vec::new();
    for _ in 0..3 {
        let commands = crate::interpreter::dispatch_ui_event(surface, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }, &mut painted.input);
        shell.chrome_build.note_content_focus_commands(&commands);
        focused.extend(commands.iter().filter_map(|command| match command {
            ui_wgpu::wgpu::UiCommand::FocusChanged { node: Some(node), .. } => records.get(arena_index(node)).map(|record| record.key.as_str().to_string()),
            _ => None,
        }));
    }
    assert!(shell.chrome_build.content_has_focus(surface), "keyboard focus lands in the ToolRun panel");
    assert_eq!(focused[..2], ["framework.toolRun.1.toolRunPause".to_string(), "framework.toolRun.1.toolRunAbort".to_string()], "Tab walks the enabled run buttons in order and skips the disabled ones: {focused:?}");
    assert!(!focused.iter().any(|key| key.ends_with("toolRunStep")), "a disabled run button never takes focus: {focused:?}");
    let mut wrapped = None;
    for _ in 0..=focused.len() {
        let commands = crate::interpreter::dispatch_ui_event(surface, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }, &mut painted.input);
        wrapped = commands.iter().find_map(|command| match command {
            ui_wgpu::wgpu::UiCommand::FocusChanged { node: Some(node), .. } => records.get(arena_index(node)).map(|record| record.key.as_str().to_string()),
            _ => None,
        });
        if wrapped.as_deref().is_some_and(|key| key.ends_with("toolRunPause")) {
            break;
        }
    }
    assert_eq!(wrapped.as_deref(), Some("framework.toolRun.1.toolRunPause"), "Tab wraps past the last enabled button back onto the first");
    crate::interpreter::dispatch_ui_event(surface, ui_wgpu::wgpu::UiEvent::KeyDown { key: "Enter".into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }, &mut painted.input);
    assert_eq!(dispatched(&mut painted.input), vec![serde_json::json!({ "action": "toolRunPause", "runId": "1", "generation": 0.0 })], "Enter on the focused Pause button dispatches the run's pause");
    while !document.close_step() {}
}

#[test]
fn a_run_the_shell_has_not_seen_reveals_the_tool_run_panel_once() {
    let mut document = publish_panel("framework.panel.toolRun/reveal");
    let mut seen = std::collections::BTreeSet::new();
    assert_eq!(tool_run_panel_reveals(&mut seen, &document), Ok(true), "the first rendered run reveals the panel");
    assert_eq!(seen.iter().copied().collect::<Vec<_>>(), vec![1], "the shell remembers run 1");
    assert_eq!(tool_run_panel_reveals(&mut seen, &document), Ok(false), "a re-render of the same run does not reveal again");
    while !document.close_step() {}
}
