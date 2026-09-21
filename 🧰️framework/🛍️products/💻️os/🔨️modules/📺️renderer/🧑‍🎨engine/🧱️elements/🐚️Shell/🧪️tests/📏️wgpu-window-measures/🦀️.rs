//! 📏️ LAW: the measures a React user sees in a window's Measures overlay are visible and operable on
//! wgpu. The guest's `framework.section.measures` surface is read through the bridge, projected into
//! `ui_contract` records, published as a right-sized retained document and painted by the SAME stepped
//! retained paint every window body uses; keyboard and pointer gestures on it dispatch the measure's
//! `onChange` with React's payload, addressed to the owning window instance.
//!
//! Oracle: `🧑‍🎨engine/🧫️fixtures/📏️window-measures/🔣️.json` (its `expectedOverlay` outlines are the
//! contract wire JSON, derived independently of this implementation).

use super::*;
use crate::program_bridge::window_measures_section_tests::{fixture, measures_section_document};

fn tree_parity_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🌳️window-measures-tree-parity/🔣️.json")).expect("window measure tree parity fixture")
}

#[test]
fn window_options_publish_compact_tree_rows_and_checkbox_controls() {
    let law = tree_parity_fixture();
    let window = law["windowId"].as_str().unwrap();
    let measures: Vec<WindowMeasure> = serde_json::from_value(law["measures"].clone()).expect("neutral measure contract");
    let records = window_measures_overlay_records(window, &measures, None).unwrap().unwrap();
    assert!(matches!(records[0].component, ui_contract::Component::Tree(_)), "Window Options publishes a retained Tree root");
    assert_eq!(serde_json::to_value(&records[0].component).unwrap()["presentation"], "compact");
    for label in law["expected"]["labels"].as_array().unwrap() {
        assert!(records.iter().any(|record| matches!(&record.component, ui_contract::Component::TreeItem(props) if props.label.0.as_str() == label.as_str().unwrap())), "each authored measure owns a TreeItem row: {label}");
    }
    let checkbox = records.iter().find(|record| record.key.as_str() == format!("{window}/grid-visible")).expect("the original control identity survives tree projection");
    assert_eq!(serde_json::to_value(&checkbox.component).unwrap()["appearance"], law["expected"]["toggleRole"]);
}

#[test]
fn window_options_hug_their_tree_below_the_chip_and_retire_folded_children() {
    let law = tree_parity_fixture();
    let window = law["windowId"].as_str().unwrap();
    let measures: Vec<WindowMeasure> = serde_json::from_value(law["measures"].clone()).unwrap();
    let body = &law["body"];
    let body = Rect::new(body["x"].as_f64().unwrap() as f32, body["y"].as_f64().unwrap() as f32, body["width"].as_f64().unwrap() as f32, body["height"].as_f64().unwrap() as f32);
    let mut shell = ShellState::new(vec![], "test".into());
    shell.screen_w = 1_000.0;
    shell.screen_h = 900.0;
    shell.measures_folded.insert(window.into(), false);
    let surface = window_measures_surface_id(window);
    let document = shell.publish_window_measures(window, &surface, &measures).unwrap().unwrap();
    shell.window_measures_documents.insert(window.into(), document);
    let first = paint_overlay(&mut shell, window, body);
    assert!(!first.input.hits().iter().any(|hit| hit.control_id.as_deref() == Some(format!("{window}/sun-intensity").as_str())), "a folded measure group publishes no child input");
    let theme = Theme::default();
    let rect = shell.window_measures_rect(window, body, &theme);
    assert!(rect.y >= body.y + theme.control_height, "Window Options body begins below its chip");
    assert!(rect.h + (law["expected"]["minimumUnusedHeight"].as_f64().unwrap() as f32) < body.h, "Window Options hugs accepted content instead of consuming the viewport");
    for control in ["grid-visible", "grid-mode", "grid-spacing"] {
        let id = format!("{window}/{control}");
        let hit = first.input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(id.as_str())).expect("the expanded group publishes its control");
        assert!(hit.rect.h <= law["expected"]["maximumRowHeight"].as_f64().unwrap() as f32 + 0.02, "measure controls use the measured React compact row metrics");
        assert!(hit.rect.x + hit.rect.w * 0.5 < rect.x + rect.w * 0.5, "right-anchored measure values live left of their labels");
    }
    for (id, height) in law["expected"]["rowHeights"].as_object().unwrap() {
        let key = format!("tree.label.{window}/{id}.row");
        let hit = first.input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(key.as_str())).expect("each visible tree row publishes its own hit");
        assert!((hit.rect.h - height.as_f64().unwrap() as f32).abs() < 0.02, "{id}: the accepted row matches React's per-control height");
    }
}

fn outline(records: &[ui_contract::UiNodeRecord], id: ui_contract::UiNodeId) -> Value {
    let record = records.iter().find(|record| record.id == id).expect("outline child is published");
    serde_json::json!({
        "key": record.key.as_str(),
        "component": serde_json::to_value(&record.component).expect("component wire"),
        "activity": serde_json::to_value(record.activity).expect("activity wire"),
        "disabled": record.disabled,
        "label": record.accessibility.label.as_ref().map(|label| label.0.as_str()),
        "bindings": serde_json::to_value(&record.bindings).expect("bindings wire"),
        "children": record.children.iter().map(|child| outline(records, *child)).collect::<Vec<_>>(),
    })
}

fn section_measures() -> HashMap<String, Vec<WindowMeasure>> {
    let fixture = fixture();
    let mut document = measures_section_document(&fixture["section"], 21);
    let measures = crate::program_bridge::window_measures_from_section(&document).expect("measures section decodes");
    while !document.close_step() {}
    measures
}

#[test]
fn window_measures_project_into_the_contract_overlay_react_renders() {
    let fixture = fixture();
    let measures = section_measures();
    for window in fixture["windows"].as_array().expect("fixture windows") {
        let window_id = window["windowId"].as_str().expect("window id");
        let records = window_measures_overlay_records(window_id, &measures[window_id], window["activeUtilityId"].as_str()).expect("measures project");
        match (&records, &window["expectedOverlay"]) {
            (None, Value::Null) => {}
            (Some(records), expected) => {
                assert!(records.windows(2).all(|pair| pair[0].id.0 < pair[1].id.0), "{window_id}: records publish in id order");
                assert!(records.iter().all(|record| record.children.iter().all(|child| child.0 > record.id.0)), "{window_id}: every parent precedes its children");
                assert_eq!(&outline(records, ui_contract::UiNodeId(1)), expected, "{window_id}: projected overlay");
            }
            (None, expected) => panic!("{window_id}: expected an overlay {expected}"),
        }
    }
}

#[test]
fn window_measures_surface_names_its_owning_window_instance() {
    let surface = window_measures_surface_id("puzzle3d-main");
    assert_eq!(surface, "puzzle3d-main/framework.section.measures");
    assert_eq!(window_measures_owner(&surface), Some("puzzle3d-main"));
    assert_eq!(window_measures_owner("puzzle3d-main"), None);
    assert_eq!(window_measures_owner("framework.section.measures"), None);
}

struct PaintedOverlay {
    input: InputState<ActionDescriptor>,
    instances: usize,
}

fn paint_overlay(shell: &mut ShellState, window_id: &str, rect: Rect) -> PaintedOverlay {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut cursor = ShellChromeChildCursor::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        let complete = shell.paint_window_measures_step(&mut cursor, &mut draw, &mut None, &mut atlas, &icons, &mut input, &theme, window_id, rect, &mut world_resources);
        assert!(!cursor.document.terminal_is_fault(), "the overlay paint faulted in phase {}", cursor.document.phase_name());
        complete
    });
    assert!(complete, "the overlay paint completed within its opportunity ceiling");
    let instances = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    shell.publish_retained_hit_registry(&mut input);
    PaintedOverlay { input, instances }
}

fn key_event(event: &Value) -> ui_wgpu::wgpu::UiEvent {
    match (event.get("key").and_then(Value::as_str), event.get("text").and_then(Value::as_str)) {
        (Some(key), _) => ui_wgpu::wgpu::UiEvent::KeyDown { key: key.to_string(), modifiers: ui_wgpu::wgpu::EventModifiers::default() },
        (None, Some(text)) => ui_wgpu::wgpu::UiEvent::TextInput { text: text.to_string() },
        _ => panic!("a gesture event is a key or a text input: {event}"),
    }
}

fn dispatched(input: &mut InputState<ActionDescriptor>, name: &str) -> Value {
    let mut actions = crate::collect_fixture_actions(input);
    let action = actions.iter_mut().rev().find(|action| action.action == name).unwrap_or_else(|| panic!("the gesture dispatched `{name}`"));
    let surface = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(DslValue::as_str).expect("retained actions carry their surface").to_string();
    scope_action_to_window(action, window_measures_owner(&surface).expect("a measures action names its overlay surface"));
    serde_json::json!({ "controllerId": action.controller_id, "action": action.action, "args": dsl_value_as_json(action.args.as_ref().expect("action args")) })
}

#[test]
fn window_measures_overlay_paints_and_dispatches_every_gesture_like_react() {
    let fixture = fixture();
    let measures = section_measures();
    let mut shell = ShellState::new(vec![], "test".into());
    shell.active_utility_by_window.insert("puzzle3d-main".into(), "fill".into());
    for gesture in fixture["gestures"].as_array().expect("fixture gestures") {
        let name = gesture["name"].as_str().expect("gesture name");
        let window_id = gesture["windowId"].as_str().expect("gesture window");
        let surface = window_measures_surface_id(window_id);
        let document = shell.publish_window_measures(window_id, &surface, &measures[window_id]).expect("overlay publishes").expect("the window has general measures");
        shell.window_measures_documents.insert(window_id.to_string(), document);
        shell.measures_folded.insert(window_id.to_string(), false);
        let rect = Rect::new(0.0, 0.0, 600.0, 720.0);
        let mut painted = paint_overlay(&mut shell, window_id, rect);
        assert!(painted.instances > 0, "{name}: the overlay painted instances");
        let targets: Vec<&str> = painted.input.hits().iter().filter_map(|hit| hit.control_id.as_deref()).collect();
        for measure in ["puzzle3d-fill-count", "grid-opacity", "projection", "grid-visible", "distribution.header-slider"] {
            assert!(targets.contains(&format!("{window_id}/{measure}").as_str()), "{name}: `{measure}` is painted and pointer-reachable: {targets:?}");
        }
        assert!(!targets.iter().any(|target| target.ends_with("brush-snap")), "{name}: a utility-scoped group stays out of the Measures overlay");
        if let Some(events) = gesture["events"].as_array() {
            for event in events {
                let commands = crate::interpreter::dispatch_ui_event(&surface, key_event(event), &mut painted.input);
                shell.chrome_build.note_content_focus_commands(&commands);
            }
            assert!(shell.chrome_build.content_has_focus(&surface), "{name}: keyboard focus lands in the overlay surface");
        }
        if let Some(control) = gesture["press"].as_str() {
            let hit = painted.input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(control)).unwrap_or_else(|| panic!("{name}: `{control}` registered a pointer target")).rect;
            let (x, y) = (hit.x + hit.w * 0.5, hit.y + hit.h * 0.5);
            semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut painted.input, &Theme::default())).expect("accepted Measures press");
            semio_framework_async::block_on(shell.handle_pointer_button(x, y, false, 0, &mut painted.input, &Theme::default())).expect("accepted Measures release");
        }
        let expected = &gesture["expectedAction"];
        assert_eq!(dispatched(&mut painted.input, expected["action"].as_str().expect("expected action")), *expected, "{name}");
        assert!(crate::interpreter::request_ui_document_close(&surface));
        for _ in 0..262_144 {
            if !crate::interpreter::ui_document_close_pending_for(&surface) {
                break;
            }
            assert!(crate::interpreter::close_ui_document_one());
        }
        assert!(!crate::interpreter::ui_document_close_pending_for(&surface));
        let mut document = shell.window_measures_documents.remove(window_id).expect("the accepted overlay retains its lease");
        while !document.close_step() {}
        println!("[DEBUG] accepted Measures gesture {name} dispatched its independent payload oracle");
    }
}
