//! 🛟️ Shared React layout and physical DOM ownership laws for shell panels.

use super::*;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🛟️panel-window-reservation/🔣️.json")).expect("panel reservation fixture")
}

fn fixture_shell(row: &Value) -> ShellState {
    let mut shell = window_pane_chrome_tests::split_pane_shell();
    shell.chrome.screen_w = row["width"].as_f64().expect("viewport width") as f32;
    shell.chrome.screen_h = 1000.0;
    shell.dock.dock_tabs = ShellDock::default();
    shell.dock.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    for panel in row["panels"].as_array().expect("panels") {
        let anchor = PanelAnchor::from_str(panel["anchor"].as_str().expect("anchor")).expect("known anchor");
        let state = shell.dock.anchor_state_mut(anchor);
        state.visible = panel["visible"].as_bool().expect("visibility");
        state.size = panel["size"].as_f64().expect("size") as f32;
        if panel["tabs"].as_u64() == Some(1) {
            shell.dock.dock_tabs.tabs_mut(anchor).push(DockTabNode::leaf(anchor.as_str(), anchor.as_str(), "settings", 0));
        }
    }
    shell
}

fn plan_through_chrome(shell: &mut ShellState, theme: &Theme) {
    let mut caller=crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut cursor = ShellChromeChildCursor::default();
    let mut draw = DrawList::default();
    let mut overlay = None;
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::default();
    let mut world = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    for _ in 0..2 {
        caller.turn(|cx| {
            let body = SessionCtx::of(&mut shell).body_rect(theme);
            PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, theme, body, &mut world, cx)
        });
    }
    assert_eq!(cursor.phase, 2, "the real chrome phases published the dock plan");
    caller.retire_world(&mut world);
    caller.finish();
}

#[test]
fn panel_reservation_matches_react_for_every_window_and_drop_registry() {
    let fixture = fixture();
    let theme = Theme::light();
    assert!((theme.panel_inset - fixture["spacing"].as_f64().unwrap() as f32).abs() < 0.001);
    for row in fixture["cases"].as_array().unwrap() {
        let mut shell = fixture_shell(row);
        let body = SessionCtx::of(&mut shell).body_rect(&theme);
        let anchors_before: Vec<_> = PanelAnchor::ALL.into_iter().map(|anchor| ChromeViewCtx::of(&mut shell).anchor_rect(anchor, body, &theme)).collect();
        plan_through_chrome(&mut shell, &theme);
        let left = row["reserved"][0].as_f64().unwrap() as f32;
        let right = row["reserved"][1].as_f64().unwrap() as f32;
        let expected = Rect::new(body.x + left, body.y, body.w - left - right, body.h).inset(theme.panel_inset);
        let actual = shell.dock.dock_canvas_bounds;
        for (got, want) in [(actual.x, expected.x), (actual.y, expected.y), (actual.w, expected.w), (actual.h, expected.h)] {
            assert!((got - want).abs() < 0.001, "{} actual={actual:?} expected={expected:?}", row["id"]);
        }
        assert!(!shell.dock.dock_window_plan.is_empty());
        assert!(!shell.dock.dock_drop_bodies.is_empty());
        assert!(!shell.dock.dock_drop_tab_bars.is_empty());
        for (_, rect) in &shell.dock.dock_window_plan {
            assert!(rect.x >= expected.x && rect.x + rect.w <= expected.x + expected.w + 0.001);
        }
        for (_, rect, _) in &shell.dock.dock_drop_bodies {
            assert!(rect.x >= expected.x && rect.x + rect.w <= expected.x + expected.w + 0.001);
        }
        for (anchor, before) in PanelAnchor::ALL.into_iter().zip(anchors_before) {
            assert_eq!(ChromeViewCtx::of(&mut shell).anchor_rect(anchor, body, &theme), before, "an overlay panel stays on the body edge");
        }
        for anchor in PanelAnchor::ALL {
            shell.dock.anchor_state_mut(anchor).visible = false;
        }
        plan_through_chrome(&mut shell, &theme);
        assert_eq!(shell.dock.dock_canvas_bounds, body.inset(theme.panel_inset), "closing restores the complete canvas");
    }
}

#[test]
fn open_shell_panels_never_activate_the_window_underneath() {
    let fixture = fixture();
    let theme = Theme::light();
    for row in fixture["pointerCases"].as_array().unwrap() {
        let anchor = PanelAnchor::from_str(row["anchor"].as_str().unwrap()).unwrap();
        let setup = serde_json::json!({"width": 1600, "panels": [{"anchor": anchor.as_str(), "visible": row["visible"], "tabs": row["tabs"], "size": 300}]});
        let mut shell = fixture_shell(&setup);
        let body = SessionCtx::of(&mut shell).body_rect(&theme);
        let panel = ChromeViewCtx::of(&mut shell).anchor_rect(anchor, body, &theme);
        let point = (panel.x + panel.w * 0.5, panel.y + panel.h * 0.5);
        shell.dock.dock_window_plan = vec![("pane-perspective".into(), SessionCtx::of(&mut shell).body_rect(&theme))];
        shell.windows.active_window_id = Some("pane-top".into());
        let mut input = InputState::<ActionDescriptor>::default();
        PanelsViewCtx::of(&mut shell).publish_retained_input_for_test(&mut input, &theme);
        let expected = row["activates"].as_bool().unwrap();
        assert_eq!(shell.input.activate_window_under_pointer(point.0, point.1, &Theme::light(), &mut shell.windows, &mut shell.dock), expected, "{row}");
        assert_eq!(shell.windows.active_window_id.as_deref(), Some(if expected { "pane-perspective" } else { "pane-top" }));
    }
}
