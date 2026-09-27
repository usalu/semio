//! ⌨️ The native keyboard ring, replayed from the language-agnostic fixture `🧑‍🎨engine/🧫️fixtures/⌨️native-keyboard-ring`:
//! Tab / Shift+Tab walk the chrome's own accessibility publication and wrap, Enter presses the focused control through the
//! accessibility activation path, and the focus ring is drawn around the focused control's hit rect (ticket 26/09/23 slice WG10).

use super::*;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/⌨️native-keyboard-ring/🔣️.json")).expect("keyboard ring fixture")
}

/// 🖼️ One frame's chrome hits: the previous frame's registry retired first, exactly as `FrameBuildPhase::InputFrame` does.
fn register_fixture_hits(fixture: &Value, input: &mut InputState<ActionDescriptor>) {
    while input.retire_hit_step() {}
    for control in fixture["controls"].as_array().expect("controls") {
        let rect = control["rect"].as_array().expect("rect").iter().map(|value| value.as_f64().expect("rect value") as f32).collect::<Vec<_>>();
        let kind = if control["kind"] == "toggle" { HitKind::Toggle } else { HitKind::Button };
        input.register_hit(HitTarget { rect: Rect::new(rect[0], rect[1], rect[2], rect[3]), event: None, control_id: control["controlId"].as_str().map(str::to_string), kind, drag_axis: None, drag_data: None });
    }
}

#[test]
fn tab_walks_the_chrome_ring_and_enter_presses_the_focused_control() {
    let fixture = fixture();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::default();
    register_fixture_hits(&fixture, &mut input);
    shell.publish_retained_hit_registry(&mut input);
    let published: Vec<String> = fixture["controls"].as_array().unwrap().iter().map(|control| control["controlId"].as_str().unwrap().to_string()).collect();
    let ring: Vec<String> = shell.keyboard_ring().into_iter().filter_map(|stop| match stop { ShellKeyboardStop::Chrome(key) => Some(key), ShellKeyboardStop::Window(..) => None }).collect();
    assert_eq!(ring, published, "the ring is the chrome's focusable controls in publication order");
    for step in fixture["steps"].as_array().unwrap() {
        let (action, shift) = match step["key"].as_str().unwrap() {
            "tab" => (ui_wgpu::wgpu::KeyAction::Tab, false),
            "shift+tab" => (ui_wgpu::wgpu::KeyAction::Tab, true),
            _ => (ui_wgpu::wgpu::KeyAction::Enter, false),
        };
        let modifiers = PointerModifiers { shift, ..PointerModifiers::default() };
        semio_framework_async::block_on(shell.handle_keyboard_async(action, &modifiers, &mut input)).expect("keyboard step");
        assert_eq!(shell.accessibility_focused_control_id.as_deref(), step["focus"].as_str(), "{step}");
        if let Some(open) = step["searchOpen"].as_bool() {
            assert_eq!(shell.search_open, open, "{step}: Enter pressed the focused control once");
        }
        if published.iter().any(|control| Some(control.as_str()) == step["focus"].as_str()) {
            register_fixture_hits(&fixture, &mut input);
            shell.publish_retained_hit_registry(&mut input);
            let focused: Vec<&str> = shell.presented_chrome_accessibility.iter().filter(|node| node.focused).map(|node| node.key.as_str()).collect();
            assert_eq!(focused, vec![step["focus"].as_str().unwrap()], "{step}: the next presented frame announces the ring's focus to assistive technology");
        }
    }
}

#[test]
fn the_focus_ring_surrounds_the_focused_controls_hit_rect() {
    let fixture = fixture();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::default();
    register_fixture_hits(&fixture, &mut input);
    shell.publish_retained_hit_registry(&mut input);
    assert_eq!(shell.keyboard_focus_ring_rect(&input), None, "nothing is keyboard-focused yet");
    shell.accessibility_focused_control_id = Some("ui.find.toggle".into());
    register_fixture_hits(&fixture, &mut input);
    assert_eq!(shell.keyboard_focus_ring_rect(&input), Some(Rect::new(40.0, 4.0, 24.0, 24.0)), "the ring is drawn around this frame's hit rect of the focused control");
}
