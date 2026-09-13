//! 🎛️ LAW: one gesture on a retained form control dispatches that control's OWN guest action, with
//! the gesture's payload named by its trigger and merged over the node's authored args — and
//! dispatches nothing at all when the node declares no binding for that trigger.
//!
//! The defect: the retained `EventRouter` named committing an edited value through `on_change` as
//! "a documented gap for a later milestone", and `Toggle`/`Slider`/`NumberStepper`/`Ring`/
//! `IconSelect` appeared only in `is_focusable`. The shell's separate immediate-mode commit system
//! (`commit_focused_input` + `stepper_metas`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) is keyed by the ids
//! the CHROME's immediate-mode widget walk mints and never sees a retained document's, so editing a
//! generation's parameters on wgpu moved a caret and a knob and reached no guest
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-wgpu-parity-2026-09-13.md` gaps #1/#6).
//!
//! This law drives the REAL router — `EventRouter::dispatch` over a real `UiTree`, through real
//! hit-testing, real capture and real focus — and reads the `UiCommand::App`s it produced.
//!
//! Oracle: `🖱️ui/🧫️fixtures/🎛️retained-control-commit/🔣️.json`; its TypeScript twin is
//! `📺️renderer/🧑‍🎨engine/🧪️tests/🎛️retained-control-commit/🟦️.ts`.

use super::{EventModifiers, EventRouter, PointerButton, UiCommand, UiEvent};
use crate::wgpu::arena::NodeId;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiIconSelectNode, UiInputNode, UiNode, UiNumberStepperNode, UiPresence, UiRingNode, UiSliderNode, UiStackNode, UiToggleNode};
use crate::wgpu::tree::{Node, NodeKey, UiTree, WidgetSpec};
use crate::wgpu::IconName;
use dsl::DslValue;
use serde_json::Value;

fn law() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎛️retained-control-commit/🔣️.json")).expect("retained control commit fixture")
}

/// 🎬️ The retained node's own `ActionDescriptor` for one fixture binding. An ABSENT binding becomes
/// the empty action name `reconcile::record_action_or_inert` writes — the retained carrier for
/// "this node declares no binding for that trigger", which the router must answer with silence.
fn descriptor(binding: &Value) -> ActionDescriptor {
    let Some(binding) = binding.as_object() else {
        return ActionDescriptor { controller_id: "ctrl".into(), action: String::new(), args: None };
    };
    let args = binding.get("args").and_then(Value::as_object).map(|entries| DslValue::Object(entries.iter().map(|(key, value)| (key.clone(), json_to_dsl(value))).collect()));
    ActionDescriptor { controller_id: "ctrl".into(), action: binding["action"].as_str().expect("binding action").to_string(), args }
}

fn json_to_dsl(value: &Value) -> DslValue {
    match value {
        Value::Null => DslValue::Null,
        Value::Bool(flag) => DslValue::Bool(*flag),
        Value::Number(number) => DslValue::float(number.as_f64().expect("fixture number")),
        Value::String(text) => DslValue::String(text.clone()),
        Value::Array(items) => DslValue::Array(items.iter().map(json_to_dsl).collect()),
        Value::Object(entries) => DslValue::Object(entries.iter().map(|(key, entry)| (key.clone(), json_to_dsl(entry))).collect()),
    }
}

fn number(value: &Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture number {key}"))
}

fn control_node(case: &Value) -> UiNode {
    let node = &case["node"];
    let on_change = descriptor(&case["binding"]);
    let presence = UiPresence::default();
    match node["kind"].as_str().expect("fixture node kind") {
        "input" => UiNode::Input(UiInputNode {
            id: node["id"].as_str().unwrap_or_default().to_string(),
            input_kind: node["inputKind"].as_str().unwrap_or("text").to_string(),
            value: node["value"].as_str().unwrap_or_default().to_string(),
            placeholder: None,
            commit: node["commit"].as_str().map(str::to_owned),
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change,
            presence,
            menu: None,
        }),
        "toggle" => {
            let mut presence = presence;
            presence.selected = node["on"].as_bool().unwrap_or(false);
            UiNode::Toggle(UiToggleNode { id: node["id"].as_str().unwrap_or_default().to_string(), icon_id: IconName::CircleDot, text: None, on_change, presence, menu: None })
        }
        "slider" => UiNode::Slider(UiSliderNode {
            id: node["id"].as_str().unwrap_or_default().to_string(),
            value: number(node, "value"),
            min: number(node, "min"),
            max: number(node, "max"),
            step: number(node, "step"),
            unit: None,
            on_change,
            presence,
            menu: None,
        }),
        "numberStepper" => UiNode::NumberStepper(UiNumberStepperNode {
            id: node["id"].as_str().unwrap_or_default().to_string(),
            value: number(node, "value"),
            step: number(node, "step"),
            uniform: node["uniform"].as_bool().unwrap_or(true),
            on_absolute: on_change,
            on_delta: descriptor(&case["deltaBinding"]),
            presence,
            menu: None,
        }),
        "ring" => UiNode::Ring(UiRingNode { id: node["id"].as_str().unwrap_or_default().to_string(), orb_id: node["orbId"].as_str().unwrap_or_default().to_string(), t: number(node, "t"), on_change, presence, menu: None }),
        "iconSelect" => UiNode::IconSelect(UiIconSelectNode {
            id: node["id"].as_str().unwrap_or_default().to_string(),
            value: node["value"].as_str().unwrap_or_default().to_string(),
            uniform: node["uniform"].as_bool().unwrap_or(true),
            classifier_kind: node["classifierKind"].as_str().unwrap_or("icon").to_string(),
            on_change,
            presence,
            menu: None,
        }),
        other => panic!("fixture node kind {other}"),
    }
}

fn place(tree: &mut UiTree, parent: Option<NodeId>, ordinal: u32, node: UiNode, rect: (f32, f32, f32, f32)) -> NodeId {
    let id = tree.insert_child(parent, Node::new(NodeKey::Positional(ordinal, ordinal), WidgetSpec(node)));
    let bucket = tree.node_mut(id).expect("placed node");
    bucket.layout.x = rect.0;
    bucket.layout.y = rect.1;
    bucket.layout.width = rect.2;
    bucket.layout.height = rect.3;
    id
}

fn root_stack() -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
}

/// 🎬️ Every `UiCommand::App` one fixture gesture produced, driven through the real router.
fn dispatched(case: &Value) -> Vec<ActionDescriptor> {
    let bounds = case["bounds"].as_array().expect("fixture bounds");
    let rect = (bounds[0].as_f64().expect("x") as f32, bounds[1].as_f64().expect("y") as f32, bounds[2].as_f64().expect("w") as f32, bounds[3].as_f64().expect("h") as f32);
    let mut tree = UiTree::new();
    // 🪟️ A container generous enough that a blur press lands inside the window but outside the
    // control — `hit_test` never answers a plain container, so that press clears focus.
    let root = place(&mut tree, None, 0, root_stack(), (0.0, 0.0, rect.0 + rect.2 + 400.0, rect.1 + rect.3 + 400.0));
    let control = place(&mut tree, Some(root), 1, control_node(case), rect);
    let mut router = EventRouter::new("main");

    let gesture = &case["gesture"];
    let at = gesture["at"].as_array().expect("fixture gesture point");
    let x = rect.0 + rect.2 * at[0].as_f64().expect("gesture x") as f32;
    let y = rect.1 + rect.3 * at[1].as_f64().expect("gesture y") as f32;
    let mut commands = Vec::new();
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerDown { x, y, button: PointerButton::Primary }));
    // 🎚️ A drag keeps the press's capture while the pointer leaves the control, exactly as a browser
    // does — the release still belongs to the node the gesture started on.
    if let Some(to) = gesture["to"].as_array() {
        let to_x = rect.0 + rect.2 * to[0].as_f64().expect("drag x") as f32;
        let to_y = rect.1 + rect.3 * to[1].as_f64().expect("drag y") as f32;
        commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: to_x, y: to_y }));
        commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: to_x, y: to_y, button: PointerButton::Primary }));
    } else {
        commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerUp { x, y, button: PointerButton::Primary }));
    }
    match gesture["kind"].as_str().expect("fixture gesture kind") {
        "press" | "drag" => {}
        kind => {
            let text = gesture["text"].as_str().expect("fixture gesture text");
            commands.extend(router.dispatch(&mut tree, root, &UiEvent::TextInput { text: text.to_string() }));
            match kind {
                "type" => {}
                "typeThenEnter" => commands.extend(router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Enter".into(), modifiers: EventModifiers::default() })),
                "typeThenBlur" => {
                    let away_x = rect.0 + rect.2 + 200.0;
                    let away_y = rect.1 + rect.3 + 200.0;
                    commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: away_x, y: away_y, button: PointerButton::Primary }));
                }
                other => panic!("fixture gesture kind {other}"),
            }
        }
    }
    let _ = control;
    commands
        .into_iter()
        .filter_map(|command| match command {
            UiCommand::App { action, .. } => Some(action),
            _ => None,
        })
        .collect()
}

fn expected_descriptor(expected: &Value) -> ActionDescriptor {
    let args = expected["args"].as_object().expect("fixture expected args");
    // 🔤️ Sorted, because `serde_json::Map`'s own iteration order is its (alphabetical) BTree order
    // while a merged `DslValue::Object` keeps the authored order with the payload appended. The LAW
    // is the SET of key/value pairs, not the order the two sides happen to carry them in.
    let mut entries: Vec<(String, DslValue)> = args.iter().map(|(key, value)| (key.clone(), json_to_dsl(value))).collect();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    ActionDescriptor { controller_id: "ctrl".into(), action: expected["action"].as_str().expect("fixture expected action").to_string(), args: Some(DslValue::Object(entries)) }
}

fn sorted(action: &ActionDescriptor) -> ActionDescriptor {
    let mut entries: Vec<(String, DslValue)> = match action.args.as_ref() {
        Some(DslValue::Object(entries)) => entries.clone(),
        _ => Vec::new(),
    };
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    ActionDescriptor { controller_id: action.controller_id.clone(), action: action.action.clone(), args: Some(DslValue::Object(entries)) }
}

#[test]
fn every_retained_control_commits_its_own_guest_action() {
    let law = law();
    let cases = law["cases"].as_array().expect("fixture cases");
    assert!(cases.len() >= 15, "the oracle must cover every value-carrying kind, got {}", cases.len());
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let actions = dispatched(case);
        if case["expected"].is_null() {
            assert!(actions.is_empty(), "{name}: this gesture must dispatch nothing, got {actions:?}");
            continue;
        }
        let expected = expected_descriptor(&case["expected"]);
        assert!(!actions.is_empty(), "{name}: expected {expected:?}, got no dispatch at all");
        for action in &actions {
            assert_eq!(sorted(action), expected, "{name}");
        }
    }
}

/// 🎚️ A `Slider`'s and a `Ring`'s value follows the pointer while it is down — every intermediate
/// value, the way React's own controls report them, not only the release.
#[test]
fn a_dragged_slider_reports_every_intermediate_value() {
    let law = law();
    let case = law["cases"].as_array().expect("cases").iter().find(|case| case["name"] == "slider-reads-its-own-track").expect("slider case");
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, root_stack(), (0.0, 0.0, 600.0, 200.0));
    let control = place(&mut tree, Some(root), 1, control_node(case), (20.0, 0.0, 100.0, 24.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 12.0, button: PointerButton::Primary });
    let moved = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 70.0, y: 12.0 });

    let value = moved
        .iter()
        .find_map(|command| match command {
            UiCommand::App { action, .. } => match action.args.as_ref() {
                Some(DslValue::Object(entries)) => entries.iter().find(|(key, _)| key == "value").map(|(_, value)| value.as_f64().expect("numeric slider value")),
                _ => None,
            },
            _ => None,
        })
        .expect("a captured slider must commit on pointer move, not only on release");
    assert!((value - 5.0).abs() < f64::EPSILON, "half way along a 0..10 track is 5, got {value}");
    let _ = control;
}
