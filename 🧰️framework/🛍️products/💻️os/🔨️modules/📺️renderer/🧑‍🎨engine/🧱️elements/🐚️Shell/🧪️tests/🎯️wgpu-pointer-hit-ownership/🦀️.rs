//! 🎯️ LAWS: who owns a pointer — packet W12a of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY.
//!
//! 🩸️ Eight steps of the parity journey pressed the shell's own chrome and ALSO reached the world
//! pane painted under it: `panel-tool-runs`, `panel-chat`, `panel-chat-close`,
//! `pane-chip-engagement-toggle`, `split-gutter-drag`, `window-cap-focus`, `window-cap-close` and
//! `example-switch` each journalled an `interactionHover`/`interactionSelect` the React reference
//! never emits, and the stray select carried EMPTY targets — so pressing a panel tab cleared the
//! world selection (`📓️w11a-prepared-world-mesh-missing.md` §6 family A,
//! `🗑️generated/w11a-parity-run-14/steps.json`). The cause is geometric: the shell paints its chrome
//! INSIDE a pane's rect (the top-right panel's tabs at `+1380,57.6`, the pane chips at `+6.4,57.6`,
//! both over `puzzle3d-main-*@…+3,54`), and the renderer routed a pointer by `bounds.contains`.
//!
//! ⚖️ React never has the question: its chrome is DOM elements ABOVE the `<canvas>`, so a click on
//! chrome never reaches the canvas, a hover over chrome clears the canvas's hover, and the element a
//! `pointerdown` resolved to owns the rest of the sequence. The wgpu target states that as
//! [`ShellState::pointer_owner_at`] (the layers) and [`PointerCapture`] (the sequence), and the laws
//! below drive the probe's exact gestures through both.

use super::*;

/// 🧾️ The eight leaking steps, each with the hit its press actually resolved to in run 14 — kind and
/// control id as the live hit ledger published them (`steps.json`'s `detail.id`, and for the caps and
/// the gutter the `os_host pointer hit` lines of `📓️w9c-behaviour-parity-run-2.md` §1.1 and §5).
const CHROME_PRESS_STEPS: &[(&str, HitKind, &str)] = &[
    ("panel-tool-runs", HitKind::PanelTab, "framework.panel.toolRun"),
    ("panel-chat", HitKind::PanelTab, "framework.chat"),
    ("panel-chat-close", HitKind::PanelTab, "framework.chat"),
    ("pane-chip-engagement-toggle", HitKind::Toggle, "framework.window.puzzle3dMainTop.engagement.toggle"),
    ("split-gutter-drag", HitKind::DockSplit, "dock.split..0"),
    ("window-cap-focus", HitKind::Button, "dock.tab.0.puzzle3d-main-top.focus"),
    ("window-cap-close", HitKind::Button, "dock.tab.0.puzzle3d-main-top.close"),
    ("example-switch", HitKind::DropdownItem, "playground.navbar.fixture.option.1"),
];

fn hit(kind: HitKind, control_id: &str) -> HitTarget<ActionDescriptor> {
    HitTarget::<ActionDescriptor> { rect: Rect::new(0.0, 54.0, 120.0, 22.4), event: None, control_id: Some(control_id.to_string()), kind, drag_axis: None, drag_data: None }
}

/// 🖱️ The probe's own gesture, phase by phase: one move onto the target, a press, eight interpolated
/// moves and a release (`🐍️parity-interact-probe.mjs`'s `drag`; its `click` is the same without the
/// eight). `under_pointer` is what the moves travel over — a drag leaves the chrome it started on,
/// which is exactly the case pointer capture exists for.
fn sequence_owners(at_press: PointerHitOwner, under_pointer: PointerHitOwner) -> Vec<PointerHitOwner> {
    let mut capture = PointerCapture::default();
    let pointer = ui_render::PointerId(1);
    let mut owners = vec![capture.owner_of_move(pointer, at_press)];
    owners.push(capture.press(pointer, at_press, 10.0, 20.0).expect("capture admitted"));
    for _ in 0..8 {
        owners.push(capture.owner_of_move(pointer, under_pointer));
    }
    owners.push(capture.release(pointer));
    owners
}

//#region 🎯️SequenceOwnership

/// 🎯️ **The sequence law.** For every one of the eight leaking steps the chrome owns the press — and
/// with it every move that follows and the release — so the world lane receives nothing at all. The
/// gutter drag is the reason capture and not a per-point test: it starts on `dock.split..0` and its
/// eight moves travel straight across the pane beside it.
#[test]
fn every_chrome_press_of_the_journey_owns_its_whole_pointer_sequence() {
    for (step, kind, control_id) in CHROME_PRESS_STEPS {
        let target = hit(*kind, control_id);
        assert_eq!(ShellState::pointer_hit_owner(Some(&target)), PointerHitOwner::Chrome, "🎯️ {step}: {control_id} is chrome, not the pane painted under it");
        let owners = sequence_owners(PointerHitOwner::Chrome, PointerHitOwner::Surface);
        assert_eq!(owners.len(), 11, "🎯️ the probe's gesture is a move, a press, eight moves and a release");
        assert!(owners.iter().all(|owner| *owner == PointerHitOwner::Chrome), "🎯️ {step}: every phase belongs to the chrome the press landed on: {owners:?}");
    }
}

/// 🎯️ …and the converse, which is the same DOM rule read the other way: a press that landed on the
/// SURFACE keeps the gesture there even where it sweeps over chrome, because three's `OrbitControls`
/// calls `setPointerCapture` on the canvas (`node_modules/three/examples/jsm/controls/OrbitControls.js`,
/// its `onPointerDown`). An orbit drag that passes under a panel must not stall.
#[test]
fn a_press_on_the_surface_keeps_the_gesture_on_it_and_releases_it_again() {
    let owners = sequence_owners(PointerHitOwner::Surface, PointerHitOwner::Chrome);
    assert!(owners.iter().all(|owner| *owner == PointerHitOwner::Surface), "🎯️ a captured surface gesture survives the chrome it travels over: {owners:?}");
    let mut capture = PointerCapture::default();
    let pointer = ui_render::PointerId(1);
    assert_eq!(capture.press(pointer, PointerHitOwner::Surface, 10.0, 20.0), Some(PointerHitOwner::Surface));
    assert_eq!(capture.release(pointer), PointerHitOwner::Surface);
    assert_eq!(capture.holder(pointer), None, "🎯️ the release ENDS the sequence");
    assert_eq!(capture.owner_of_move(pointer, PointerHitOwner::Chrome), PointerHitOwner::Chrome, "🎯️ and the next move is answered by what is under the pointer again");
    assert_eq!(PointerCapture::default().release(pointer), PointerHitOwner::Surface, "🎯️ a release with no press before it — the pointer entered already down — is the surface's, as it is in the DOM");
}

#[test]
fn fixed_pointer_captures_keep_independent_positions_and_release_only_their_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).expect("neutral scene owner fixture");
    let capacity = fixture["captureCapacity"].as_u64().expect("fixed capture grant") as usize;
    let press = &fixture["press"];
    let x = press[0].as_f64().unwrap() as f32;
    let y = press[1].as_f64().unwrap() as f32;
    let release = &fixture["release"];
    let end_x = release[0].as_f64().unwrap() as f32;
    let end_y = release[1].as_f64().unwrap() as f32;
    let expected_delta = (fixture["moveDelta"][0].as_f64().unwrap() as f32, fixture["moveDelta"][1].as_f64().unwrap() as f32);
    let mut capture = PointerCapture::default();
    for index in 0..capacity {
        assert_eq!(capture.press(ui_render::PointerId(index as u64), PointerHitOwner::Surface, x, y), Some(PointerHitOwner::Surface));
    }
    assert_eq!(capture.press(ui_render::PointerId(capacity as u64), PointerHitOwner::Chrome, 900.0, 900.0), None);
    assert_eq!(capture.advance(ui_render::PointerId(1), x + 25.0, y - 40.0), Some((25.0, -40.0)));
    assert_eq!(capture.release(ui_render::PointerId(1)), PointerHitOwner::Surface);
    assert!(capture.any_active());
    assert_eq!(capture.advance(ui_render::PointerId(0), end_x, end_y), Some(expected_delta));
    assert_eq!(capture.advance(ui_render::PointerId(0), end_x + 4.0, end_y - 3.0), Some((4.0, -3.0)));
    assert_eq!(capture.position(ui_render::PointerId(0)), Some([end_x + 4.0, end_y - 3.0]));
    assert_eq!(capture.position(ui_render::PointerId(1)), None);
    assert_eq!(capture.press(ui_render::PointerId(capacity as u64), PointerHitOwner::Chrome, 900.0, 900.0), Some(PointerHitOwner::Chrome));
    for index in 0..=capacity {
        capture.release(ui_render::PointerId(index as u64));
    }
    assert!(!capture.any_active());
}

/// 🌍️ The surface's own body, and a point that hits nothing, still belong to the scene: this is an
/// ownership model, not a claim on everything.
#[test]
fn the_surface_body_and_the_empty_canvas_are_never_chrome() {
    assert_eq!(ShellState::pointer_hit_owner(Some(&hit(HitKind::World3d, "puzzle3d-main-perspective"))), PointerHitOwner::Surface);
    assert_eq!(ShellState::pointer_hit_owner(Some(&hit(HitKind::Generic, "puzzle3d-main-perspective.node.7"))), PointerHitOwner::Surface, "🌍️ a scene's own per-element target is the scene's");
    assert_eq!(ShellState::pointer_hit_owner(None), PointerHitOwner::Surface);
    assert!(!ShellState::pointer_press_belongs_to_shell_chrome(None), "🌍️ the bool this replaces answers the same");
}

//#endregion 🎯️SequenceOwnership

//#region 🚧️OverlayLayers

/// 🚧️ **The modal law.** A menu, a dropdown, a dialog and the tour are React overlays with their own
/// backdrop: while one is open NO pointer reaches the canvas, wherever it lands — including the point
/// beside the menu, which registers no hit of its own and used to fall straight through to the world.
#[test]
fn an_open_overlay_owns_every_pointer_until_it_closes() {
    let mut shell = super::window_pane_chrome_tests::split_pane_shell();
    let theme = Theme::light();
    let mut input = InputState::<ActionDescriptor>::default();
    shell.publish_retained_input_for_test(&mut input, &theme);
    assert!(!shell.pointer_input_is_modal(), "🚧️ a shell with no overlay open is not modal");
    assert_eq!(shell.pointer_owner_at(400.0, 400.0, &input, &theme), PointerHitOwner::Chrome, "an empty registry publishes no scene owner");

    shell.context_menu = Some(ContextMenuState::default());
    shell.publish_retained_input_for_test(&mut input, &theme);
    assert!(shell.pointer_input_is_modal(), "🚧️ an open context menu is modal");
    assert_eq!(shell.pointer_owner_at(400.0, 400.0, &input, &theme), PointerHitOwner::Chrome, "🚧️ even where the menu paints nothing");
    assert!(!shell.wheel_reaches_scene_surface(400.0, 400.0, &input, &theme), "🚧️ and a wheel notch over it never zooms the world");
    shell.context_menu = None;

    shell.open_selects.insert("playground.navbar.fixture".into(), true);
    shell.publish_retained_input_for_test(&mut input, &theme);
    assert!(shell.pointer_input_is_modal(), "🚧️ an open dropdown is modal — `example-switch` presses its option list");
    shell.open_selects.clear();

    shell.chrome_build.start_introduction();
    shell.publish_retained_input_for_test(&mut input, &theme);
    assert!(shell.pointer_input_is_modal(), "🚧️ the tour owns every pointer it covers while it plays");
    shell.chrome_build.skip_introduction();
    shell.publish_retained_input_for_test(&mut input, &theme);
    assert!(!shell.pointer_input_is_modal());
}

/// 📑️ **The panel-box law.** An open anchored panel is an opaque LAYER, not a set of controls: its
/// padding, the gaps between its tree rows and the empty space under the last one register no hit at
/// all, and every one of those points is over the pane the panel floats on. React's panel is a DOM
/// element, so the canvas beneath it receives nothing anywhere inside its box.
#[test]
fn an_open_panels_whole_box_owns_the_pointer_over_the_pane_it_floats_on() {
    let mut shell = super::window_pane_chrome_tests::split_pane_shell();
    let theme = Theme::light();
    let mut input = InputState::<ActionDescriptor>::default();
    shell.screen_w = 1594.0;
    shell.screen_h = 936.0;
    let body = shell.body_rect(&theme);
    let centre = [body.x + body.w / 2.0, body.y + body.h / 2.0];
    shell.publish_retained_input_for_test(&mut input, &theme);
    assert_eq!(shell.pointer_owner_at(centre[0], centre[1], &input, &theme), PointerHitOwner::Chrome, "folded anchors do not mint an unregistered scene");

    *shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft) = vec![DockTabNode::leaf("framework.panel.artifact", "Artifact", "circle-dot", 0)];
    shell.anchor_state_mut(PanelAnchor::TopLeft).visible = true;
    assert!(shell.anchor_open(PanelAnchor::TopLeft), "📑️ the anchor paints a panel now");
    shell.publish_retained_input_for_test(&mut input, &theme);
    let panel = shell.anchor_rect(PanelAnchor::TopLeft, body, &theme);
    let inside = [panel.x + panel.w / 2.0, panel.y + panel.h / 2.0];
    assert!(shell.pointer_is_over_open_panel(inside[0], inside[1], &theme));
    assert_eq!(shell.pointer_owner_at(inside[0], inside[1], &input, &theme), PointerHitOwner::Chrome, "📑️ a point inside the panel's box is the panel's, hit row or no hit row");
    assert!(!shell.wheel_reaches_scene_surface(inside[0], inside[1], &input, &theme), "📑️ and the wheel scrolls the panel, never the world under it");
    let outside = [body.x + body.w - 4.0, body.y + body.h - 4.0];
    assert!(!shell.pointer_is_over_open_panel(outside[0], outside[1], &theme), "📑️ the pane beside the panel still owns its own points");
    assert_eq!(shell.pointer_owner_at(outside[0], outside[1], &input, &theme), PointerHitOwner::Chrome, "outside a panel still requires published scene provenance");
}

//#endregion 🚧️OverlayLayers

//#region 🧊️RendererIngress

/// 🧊️ **The wiring law.** One predicate, asked by the renderer's own ingress — a routing rule nothing
/// calls is exactly the shape every defect in this family had (`📓️w9b` §1: the predicate existed and
/// the press path never reached it; `📓️w9c` §5: the gutter was hit-tested and the shell never saw the
/// press). The press resolves the capture, the move asks it before it may enqueue a world intent, and
/// a move the chrome owns hands the surface a LEAVE instead.
#[test]
fn the_renderer_ingress_routes_every_pointer_phase_through_the_capture() {
    super::shell_input_tests::retained_world_sequence_probe("motion-wheel");
}

//#endregion 🧊️RendererIngress
