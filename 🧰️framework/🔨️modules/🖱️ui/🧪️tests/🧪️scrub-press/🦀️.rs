//! 🎚️ LAW: every continuous control of the retained wgpu router speaks the scrub protocol React's continuous lane speaks
//! (`📓️api-scrub-machine.md`, design §13.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the ticks and the
//! release of one press carry one `gesture`, `commit` is `true` on the release only, the release goes out wherever the
//! pointer lets go, a lost capture or a cancel sends `{gesture, abort}` with no value, and a keyboard step or a stepper
//! click is a one-shot press. The argument names are the `🛠️tool-machine` scrub fixture's.

use super::*;
use crate::wgpu::component::ui::{UiInputNode, UiPresence, UiRingNode, UiStackNode};
use crate::wgpu::tree::WidgetSpec;

fn binding() -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: "setOpacity".into(), args: Some(DslValue::Object(vec![("id".into(), DslValue::String("layer-1".into()))])) }
}

fn place(tree: &mut UiTree, parent: Option<NodeId>, ordinal: u32, node: UiNode, rect: (f32, f32, f32, f32)) -> NodeId {
    let id = tree.insert_child(parent, Node::new(NodeKey::Positional(ordinal, ordinal), WidgetSpec(node)));
    let bucket = tree.node_mut(id).expect("placed node");
    (bucket.layout.x, bucket.layout.y, bucket.layout.width, bucket.layout.height) = rect;
    id
}

fn stack() -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
}

fn slider() -> UiNode {
    UiNode::Slider(UiSliderNode { id: "opacity".into(), value: 0.0, min: 0.0, max: 10.0, step: 1.0, unit: None, snaps: Vec::new(), on_change: binding(), presence: UiPresence::default(), menu: None, ..Default::default() })
}

fn number_input() -> UiNode {
    UiNode::Input(UiInputNode {
        id: "width".into(),
        input_kind: "number".into(),
        value: "1".into(),
        placeholder: None,
        accessibility_label: None,
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        precision: None,
        snaps: Vec::new(),
        on_change: binding(),
        on_submit: None,
        on_abort: None,
        on_repeat_last: None,
        presence: UiPresence::default(),
        menu: None,
        ..Default::default()
    })
}

fn stepper() -> UiNode {
    let unbound = ActionDescriptor { controller_id: "ctrl".into(), action: String::new(), args: None };
    UiNode::NumberStepper(UiNumberStepperNode { id: "count".into(), value: 2.0, step: 1.0, uniform: true, min: None, max: None, precision: None, on_absolute: binding(), on_delta: unbound, presence: UiPresence::default(), menu: None, ..Default::default() })
}

/// 🧾️ Every app dispatch as `(value, gesture, commit, abort)`, the authored args checked on the way.
fn presses(commands: &[UiCommand]) -> Vec<(Option<f64>, String, Option<bool>, Option<String>)> {
    commands
        .iter()
        .filter_map(|command| match command {
            UiCommand::App { intent, .. } => intent.descriptor().args,
            _ => None,
        })
        .map(|args| {
            let field = |key: &str| match &args {
                DslValue::Object(entries) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value.clone()),
                _ => None,
            };
            assert_eq!(field("id").and_then(|id| id.as_str().map(str::to_string)).as_deref(), Some("layer-1"), "the authored args ride every press dispatch");
            (field("value").and_then(|value| value.as_f64()), field(SCRUB_GESTURE_ARG).and_then(|gesture| gesture.as_str().map(str::to_string)).expect("every press dispatch names its gesture"), field(SCRUB_COMMIT_ARG).and_then(|commit| commit.as_bool()), field(SCRUB_ABORT_ARG).and_then(|reason| reason.as_str().map(str::to_string)))
        })
        .collect()
}

fn down(x: f32, y: f32) -> UiEvent {
    UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: EventModifiers::default() }
}

fn up(x: f32, y: f32) -> UiEvent {
    UiEvent::PointerUp { x, y, button: PointerButton::Primary, modifiers: EventModifiers::default() }
}

fn track() -> Rect {
    slider_control_presentation(Rect::new(20.0, 0.0, 100.0, 24.0), 0.0, 0.0, 10.0, None, crate::wgpu::theme::Theme::default().gap_standard, FlowInline::Ltr).slider.track_cell
}

#[test]
fn the_argument_names_are_the_tool_machine_scrub_fixtures() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🛠️tool-machine/🧫️fixtures/🧫️scrub-law/🔣️.json")).expect("scrub law");
    assert_eq!((law["args"]["gesture"].as_str(), law["args"]["commit"].as_str(), law["args"]["abort"].as_str()), (Some(SCRUB_GESTURE_ARG), Some(SCRUB_COMMIT_ARG), Some(SCRUB_ABORT_ARG)));
}

/// ⚖️ A slider drag is one press: ticks while the pointer moves, then ONE release on the last value — even when the
/// pointer lets go far off the control — all under one gesture; the next drag is a new gesture.
#[test]
fn a_slider_drag_is_one_press_released_wherever_the_pointer_lets_go() {
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, stack(), (0.0, 0.0, 600.0, 200.0));
    place(&mut tree, Some(root), 1, slider(), (20.0, 0.0, 100.0, 24.0));
    let mut router = EventRouter::new("main");
    let track = track();
    let mut commands = router.dispatch(&mut tree, root, &down(track.x + track.w * 0.2, 12.0));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: track.x + track.w * 0.5, y: 12.0, modifiers: EventModifiers::default() }));
    commands.extend(router.dispatch(&mut tree, root, &up(500.0, 150.0)));
    let presses = presses(&commands);
    assert_eq!(presses.iter().map(|press| (press.0, press.2, press.3.clone())).collect::<Vec<_>>(), vec![(Some(2.0), Some(false), None), (Some(5.0), Some(false), None), (Some(5.0), Some(true), None)]);
    assert!(presses.iter().all(|press| press.1 == presses[0].1), "one gesture: {presses:?}");
    let again = presses_of(&mut router, &mut tree, root, track);
    assert_ne!(again[0].1, presses[0].1, "the next drag is another press");
}

fn presses_of(router: &mut EventRouter, tree: &mut UiTree, root: NodeId, track: Rect) -> Vec<(Option<f64>, String, Option<bool>, Option<String>)> {
    let mut commands = router.dispatch(tree, root, &down(track.x + track.w * 0.8, 12.0));
    commands.extend(router.dispatch(tree, root, &up(track.x + track.w * 0.8, 12.0)));
    presses(&commands)
}

/// ⚖️ A lost pointer capture cancels the open press: `{gesture, abort: captureLost}`, no value, no release.
#[test]
fn a_lost_capture_cancels_the_slider_press() {
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, stack(), (0.0, 0.0, 600.0, 200.0));
    let control = place(&mut tree, Some(root), 1, slider(), (20.0, 0.0, 100.0, 24.0));
    let mut router = EventRouter::new("main");
    let track = track();
    let mut commands = router.dispatch(&mut tree, root, &down(track.x + track.w * 0.3, 12.0));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerCancel));
    let presses = presses(&commands);
    assert_eq!(presses.len(), 2);
    assert_eq!((presses[1].0, presses[1].2, presses[1].3.as_deref()), (None, None, Some("captureLost")));
    assert_eq!(presses[1].1, presses[0].1, "the cancel names the open press");
    assert!(tree.node(control).expect("slider").state.scrub_gesture.is_none(), "the press is closed");
}

/// ⚖️ A keyboard step and a stepper click are one-shot presses: ONE dispatch that is its own release.
#[test]
fn keyboard_steps_and_stepper_clicks_are_one_shot_presses() {
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, stack(), (0.0, 0.0, 600.0, 200.0));
    let control = place(&mut tree, Some(root), 1, slider(), (20.0, 0.0, 100.0, 24.0));
    let count = place(&mut tree, Some(root), 2, stepper(), (20.0, 40.0, 90.0, 24.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(control), true);
    let stepped = presses(&router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowRight".into(), modifiers: EventModifiers::default() }));
    assert_eq!(stepped.iter().map(|press| (press.0, press.2)).collect::<Vec<_>>(), vec![(Some(1.0), Some(true))]);
    let mut clicked = router.dispatch(&mut tree, root, &down(20.0 + 90.0 * 0.85, 52.0));
    clicked.extend(router.dispatch(&mut tree, root, &up(20.0 + 90.0 * 0.85, 52.0)));
    let clicked = presses(&clicked);
    assert_eq!(clicked.iter().map(|press| (press.0, press.2)).collect::<Vec<_>>(), vec![(Some(3.0), Some(true))]);
    assert_ne!(clicked[0].1, stepped[0].1);
    assert!(tree.node(count).expect("stepper").state.scrub_gesture.is_none(), "a one-shot press leaves nothing open");
}

/// ⚖️ A number field without a commit policy is a press while it is edited: every keystroke a tick of one gesture, its
/// blur the release of the final number.
#[test]
fn a_typed_number_field_is_one_press_released_on_blur() {
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, stack(), (0.0, 0.0, 600.0, 200.0));
    place(&mut tree, Some(root), 1, number_input(), (0.0, 0.0, 160.0, 24.0));
    let mut router = EventRouter::new("main");
    let mut commands = router.dispatch(&mut tree, root, &down(80.0, 12.0));
    commands.extend(router.dispatch(&mut tree, root, &up(80.0, 12.0)));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "2".into() }));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "5".into() }));
    commands.extend(router.dispatch(&mut tree, root, &down(500.0, 150.0)));
    let presses = presses(&commands);
    assert_eq!(presses.iter().map(|press| (press.0, press.2)).collect::<Vec<_>>(), vec![(Some(12.0), Some(false)), (Some(125.0), Some(false)), (Some(125.0), Some(true))]);
    assert!(presses.iter().all(|press| press.1 == presses[0].1), "one gesture: {presses:?}");
}

/// ⚖️ A typed draft refused at blur (crossing a hard bound) is never dispatched, so the blur releases the press on the value
/// it last ticked — the value React's lane releases on — and leaves no press open.
#[test]
fn a_refused_draft_releases_the_press_on_the_value_last_ticked() {
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, stack(), (0.0, 0.0, 600.0, 200.0));
    let UiNode::Input(bounded) = number_input() else { unreachable!() };
    let field = place(&mut tree, Some(root), 1, UiNode::Input(UiInputNode { max: Some(100.0), ..bounded }), (0.0, 0.0, 160.0, 24.0));
    let mut router = EventRouter::new("main");
    let mut commands = router.dispatch(&mut tree, root, &down(80.0, 12.0));
    commands.extend(router.dispatch(&mut tree, root, &up(80.0, 12.0)));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "2".into() }));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "5".into() }));
    commands.extend(router.dispatch(&mut tree, root, &down(500.0, 150.0)));
    let presses = presses(&commands);
    assert_eq!(presses.iter().map(|press| (press.0, press.2, press.3.clone())).collect::<Vec<_>>(), vec![(Some(12.0), Some(false), None), (Some(12.0), Some(true), None)]);
    assert!(presses.iter().all(|press| press.1 == presses[0].1), "one gesture: {presses:?}");
    let state = &tree.node(field).expect("field").state;
    assert_eq!((state.scrub_gesture.as_deref(), state.scrub_offered.as_ref()), (None, None), "the press is closed");
}

fn ring() -> UiNode {
    UiNode::Ring(UiRingNode { id: "orbit".into(), orb_id: "sun".into(), t: 0.0, on_change: binding(), presence: UiPresence::default(), menu: None })
}

/// ⚖️ A ring drag is one press like a slider's: ticks while the pointer moves, ONE release where the pointer lets go —
/// even off the control — under one gesture; a lost capture cancels it with no value.
#[test]
fn a_ring_drag_is_one_press_and_a_lost_capture_cancels_it() {
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, stack(), (0.0, 0.0, 600.0, 200.0));
    let orbit = place(&mut tree, Some(root), 1, ring(), (0.0, 0.0, 100.0, 100.0));
    let mut router = EventRouter::new("main");
    let mut commands = router.dispatch(&mut tree, root, &down(90.0, 50.0));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 50.0, y: 90.0, modifiers: EventModifiers::default() }));
    commands.extend(router.dispatch(&mut tree, root, &up(-250.0, 50.0)));
    let drag = presses(&commands);
    assert_eq!(drag.iter().map(|press| (press.0, press.2, press.3.clone())).collect::<Vec<_>>(), vec![(Some(0.0), Some(false), None), (Some(0.25), Some(false), None), (Some(0.5), Some(true), None)]);
    assert!(drag.iter().all(|press| press.1 == drag[0].1), "one gesture: {drag:?}");
    let mut cancelled = router.dispatch(&mut tree, root, &down(90.0, 50.0));
    cancelled.extend(router.dispatch(&mut tree, root, &UiEvent::PointerCancel));
    let cancelled = presses(&cancelled);
    assert_eq!(cancelled.iter().map(|press| (press.0, press.2, press.3.clone())).collect::<Vec<_>>(), vec![(Some(0.0), Some(false), None), (None, None, Some("captureLost".to_string()))]);
    assert_eq!(cancelled[1].1, cancelled[0].1, "the cancel names the open press");
    assert_ne!(cancelled[0].1, drag[0].1, "another drag is another press");
    assert!(tree.node(orbit).expect("ring").state.scrub_gesture.is_none(), "the press is closed");
}

/// ⚖️ A colour field without a commit policy is a press like a number field (React's colour `InputView` rides the same lane):
/// every keystroke a tick of one gesture carrying the typed text, its blur the release.
#[test]
fn a_typed_colour_field_is_one_press_released_on_blur() {
    let mut tree = UiTree::new();
    let root = place(&mut tree, None, 0, stack(), (0.0, 0.0, 600.0, 200.0));
    let UiNode::Input(field) = number_input() else { unreachable!() };
    place(&mut tree, Some(root), 1, UiNode::Input(UiInputNode { id: "tint".into(), input_kind: "color".into(), value: "#12345".into(), ..field }), (0.0, 0.0, 160.0, 24.0));
    let mut router = EventRouter::new("main");
    let mut commands = router.dispatch(&mut tree, root, &down(80.0, 12.0));
    commands.extend(router.dispatch(&mut tree, root, &up(80.0, 12.0)));
    commands.extend(router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "6".into() }));
    commands.extend(router.dispatch(&mut tree, root, &down(500.0, 150.0)));
    let texts: Vec<(Option<String>, Option<String>, Option<bool>)> = commands
        .iter()
        .filter_map(|command| match command {
            UiCommand::App { intent, .. } => intent.descriptor().args,
            _ => None,
        })
        .map(|args| {
            let field = |key: &str| match &args {
                DslValue::Object(entries) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value.clone()),
                _ => None,
            };
            (field("value").and_then(|value| value.as_str().map(str::to_string)), field(SCRUB_GESTURE_ARG).and_then(|gesture| gesture.as_str().map(str::to_string)), field(SCRUB_COMMIT_ARG).and_then(|commit| commit.as_bool()))
        })
        .collect();
    assert_eq!(texts.iter().map(|press| (press.0.clone(), press.2)).collect::<Vec<_>>(), vec![(Some("#123456".to_string()), Some(false)), (Some("#123456".to_string()), Some(true))]);
    assert!(texts.iter().all(|press| press.1.is_some() && press.1 == texts[0].1), "one gesture: {texts:?}");
}
