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
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
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
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let mut input = InputState::default();
    register_fixture_hits(&fixture, &mut input);
    shell.publish_retained_hit_registry(&mut input);
    assert_eq!(shell.keyboard_focus_ring_rect(&input), None, "nothing is keyboard-focused yet");
    shell.accessibility_focused_control_id = Some("ui.find.toggle".into());
    register_fixture_hits(&fixture, &mut input);
    assert_eq!(shell.keyboard_focus_ring_rect(&input), Some(Rect::new(40.0, 4.0, 24.0, 24.0)), "the ring is drawn around this frame's hit rect of the focused control");
}

/// ♿️ The platform tree sanity of a REAL painted chrome: the navbar walks to completion, the frame is presented, and the
/// native accessibility publication that frame hands the platform adapter announces every navbar control with a name and a
/// role the platform knows — the gaps a screen reader cannot recover from (ticket 26/09/23 slice WG10).
#[test]
fn a_painted_navbar_hands_the_platform_a_named_tree() {
    let theme = Theme::light();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.screen_w = 1440.0;
    shell.sync_dock_tabs();
    for (id, label) in [("fixture.leading.first", "First"), ("fixture.leading.second", "Second"), ("fixture.leading.third", "Third")] {
        shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft).push(DockTabNode::leaf(id, label, "box", 0));
    }
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let walked = (0..16_384).take_while(|_| !shell.render_navbar_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, 1440.0)).count();
    assert!(walked < 16_384 && shell.error.is_none(), "the navbar walk completes: {:?}", shell.error);
    let painted = input.staged_hits().iter().filter(|hit| hit.control_id.is_some()).count();
    shell.publish_retained_hit_registry(&mut input);
    let publication = crate::native_accessibility::published_native_accessibility();
    let chrome = publication.windows.iter().find(|window| window.window_id == crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID).expect("the chrome is announced to the platform");
    let census = crate::native_accessibility::native_accessibility_census(&publication);
    eprintln!("native accessibility census: painted controls {painted}, chrome nodes {}, {census:?}", chrome.nodes.len());
    let announced: Vec<&str> = chrome.nodes.iter().map(|node| node.key.as_str()).collect();
    for tab in ["fixture.leading.first", "fixture.leading.second", "fixture.leading.third"] {
        assert!(announced.contains(&tab), "the painted panel tab {tab} reaches the platform tree: {announced:?}");
    }
    assert!(census.nodes >= chrome.nodes.len() && census.focusable >= 3, "the platform can focus every painted panel tab: {census:?}");
    assert_eq!(census.unnamed_actionable, Vec::<String>::new(), "every control a reader can act on has a name");
    assert_eq!(census.unknown_roles, Vec::<String>::new(), "every node has a role the platform knows");
}

