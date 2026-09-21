//! 🎡⎋️ LAWS: a wheel over a scene surface, and an Escape under a dismissable layer — packet W12c of
//! ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, families C and D.
//!
//! Both laws close a LIVE difference the unit suites could not see, because both depended on
//! transient chrome state the running page reaches and a fixture never had:
//!
//! | step | React actions | wgpu (run 14) | why |
//! | --- | --- | --- | --- |
//! | `zoom-wheel` | `interactionHover`, `noteWorldNavigation`, `setCamera` | `interactionHover` | the notch resolved the window's own `ScrollRegion`, whose id names no `.pane`/`.map` surface, so `wheel_reaches_scene_surface` refused and the world never saw it (`🗑️generated/w11a-parity-run-14/wgpu/console.txt:19476`, `wheel apply … hit=Some(ScrollRegion) control=Some("") propagates=false`) |
//! | `example-picker-dismiss` | `engagementAbort` | (none) | the open picker made `idle` false, so the app-keybinding rung never ran, and the sync Escape arm closed the picker and returned |

use super::command_registry_tests::test_app;
use super::*;
use infinite_world::world::World3dState;
use semio_framework::manifest::Keybinding;

const WGPU_RENDERER_SOURCE: &str = include_str!("../../../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
const WORLD_JOURNAL_SEQUENCES: &str = include_str!("../../../🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json");

/// 🪟️ The pane geometry of the parity journey's own perspective window, and the point the probe aims
/// at it (`🐍️parity-interact-probe.mjs`'s `surfacePoint`: 70 % across, 45 % down).
const PANE: (f32, f32, f32, f32) = (534.4, 54.4, 1062.4, 913.6);
const AIM: (f32, f32) = (PANE.0 + PANE.2 * 0.7, PANE.1 + PANE.3 * 0.45);

fn shell_with_pane() -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let _ = shell.world3d_states.try_insert("puzzle3d-main-perspective".into(), World3dState::new("puzzle3d-main-perspective".into(), "app.controller".into()));
    if let Some(state) = shell.world3d_states.get_mut("puzzle3d-main-perspective") {
        state.bounds = Rect::new(PANE.0, PANE.1, PANE.2, PANE.3);
    }
    shell
}

fn hit(kind: HitKind, control_id: &str) -> HitTarget<ActionDescriptor> {
    HitTarget::<ActionDescriptor> { rect: Rect::new(PANE.0, PANE.1, PANE.2, PANE.3), event: None, control_id: Some(control_id.to_string()), kind, drag_axis: None, drag_data: None }
}

//#region 🎡WheelReachesTheScene

/// 🛡️ Bounds and a suggestive control name never replace published scene provenance.
#[test]
fn a_wheel_over_a_live_pane_requires_the_published_scene_hit() {
    let shell = shell_with_pane();
    let theme = Theme::default();
    for row in [hit(HitKind::ScrollRegion, ""), hit(HitKind::ScrollRegion, "puzzle3d-main-perspective.pane"), hit(HitKind::World3d, "puzzle3d-main-perspective"), hit(HitKind::Generic, "puzzle3d-main-perspective")] {
        let mut input = InputState::<ActionDescriptor>::default();
        input.register_hit(row.clone());
        input.publish_hits();
        assert!(!shell.wheel_reaches_scene_surface(AIM.0, AIM.1, &input, &theme), "{:?}/{:?} has no published scene identity", row.kind, row.control_id);
    }
}

/// ⚖️ LAW: chrome painted over the pane still takes the notch — the rule this packet must not widen
/// away. React delivers a wheel to the DOM element it is over, and a pane chip, a panel tab or an
/// open menu is an element ABOVE the canvas.
#[test]
fn a_wheel_over_chrome_painted_inside_the_pane_never_reaches_the_scene() {
    let shell = shell_with_pane();
    let theme = Theme::default();
    for row in
        [hit(HitKind::PanelTab, "framework.panel.toolRun"), hit(HitKind::Toggle, "framework.window.puzzle3dMainPerspective.engagement.toggle"), hit(HitKind::ContextMenu, "shell.context.menu.scroll"), hit(HitKind::Button, "ui.introduction.skip")]
    {
        let mut input = InputState::<ActionDescriptor>::default();
        input.register_hit(row.clone());
        input.publish_hits();
        assert!(!shell.wheel_reaches_scene_surface(AIM.0, AIM.1, &input, &theme), "🛑️ chrome owns the notch at {:?}/{:?}", row.kind, row.control_id);
    }
}

/// ⚖️ LAW: a point outside every scene surface's rect is never the scene's, so a window whose body is
/// retained content keeps scrolling itself.
#[test]
fn a_wheel_outside_every_scene_rect_stays_with_the_chrome_that_owns_it() {
    let shell = shell_with_pane();
    let theme = Theme::default();
    let mut input = InputState::<ActionDescriptor>::default();
    input.register_hit(HitTarget::<ActionDescriptor> { rect: Rect::new(0.0, 54.4, 400.0, 913.6), event: None, control_id: Some("framework.panel.artifact".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    input.publish_hits();
    assert!(!shell.wheel_reaches_scene_surface(60.0, 400.0, &input, &theme), "🛑️ a panel's own scroll region is not a scene");
    assert!(!shell.scene_surface_contains(60.0, 400.0), "🛑️ and no scene surface is painted there");
    assert!(shell.scene_surface_contains(AIM.0, AIM.1), "🎬️ while the pane's own aim point is");
}

//#endregion 🎡WheelReachesTheScene

//#region ⎋️DismissableLayers

fn shell_with_escape_binding() -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut app = test_app(Vec::new(), Vec::new());
    app.keybindings = vec![Keybinding { keys: "escape".into(), action: ActionDescriptor { controller_id: "app.controller".into(), action: "engagementAbort".into(), args: None } }];
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::default() });
    shell
}

/// ⚖️ LAW: Escape closes every open dismissable layer — a retained `Select` and the navbar's example
/// picker — and says so, so the caller can decide whether the chord carries on.
#[test]
fn escape_dismisses_every_open_dismissable_layer_at_once() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    assert!(!shell.dismiss_dismissable_layers(), "⎋️ nothing open, nothing dismissed");
    shell.overlay_state = OverlayState::Dropdown("example".into());
    shell.open_selects.insert("widget.select".into(), true);
    assert!(shell.dismiss_dismissable_layers(), "⎋️ both layers were open");
    assert_eq!(shell.overlay_state, OverlayState::None);
    assert!(shell.open_selects.values().all(|open| !open));
    assert!(shell.deferred_actions.is_empty(), "⎋️ dismissing chrome is state, never a dispatch");
}

/// ⚖️ LAW: the quick-search and find overlays are NOT dismissable layers. They are focus-trapping
/// dialogs with a focused query field — React's `handleAppKeydown` returns early on an editable
/// target — so Escape closes them through `handle_keyboard`'s own arm and never reaches the app.
#[test]
fn the_palette_and_find_overlays_are_not_dismissable_layers() {
    for overlay in [OverlayState::Search, OverlayState::Find] {
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.overlay_state = overlay.clone();
        assert!(!shell.dismiss_dismissable_layers(), "🧯️ {overlay:?} is a dialog, not a dismissable layer");
        assert_eq!(shell.overlay_state, overlay);
    }
}

/// ⚖️ LAW: with the example picker open, Escape still reaches the app-keybinding rung — React's
/// measured `example-picker-dismiss` journal is one `engagementAbort` — because the layer is
/// dismissed BEFORE `idle` is read and the chord carries on down the ladder. Before this packet the
/// open picker made `idle` false and the sync Escape arm closed it and returned, so the whole step
/// journalled nothing.
///
/// 🧾️ A fixture with no mounted dock window cannot complete the dispatch: `dispatch_app_keybinding`
/// answers a window-owned verb no mounted window owns with the `KEYBINDING_UNOWNED_CODE` banner
/// (React's own `showTransientNotice`). That banner is therefore the proof the rung RAN — the one
/// observable a registry-less shell has — and the law asserts it is absent while the picker still
/// swallows the chord.
#[test]
fn escape_under_an_open_picker_still_reaches_the_app_keybinding_rung() {
    let mut swallowed = shell_with_escape_binding();
    let mut input = InputState::<ActionDescriptor>::default();
    swallowed.overlay_state = OverlayState::Search;
    semio_framework_async::block_on(swallowed.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input)).expect("a chord never errors here");
    assert!(swallowed.transient_notice().is_none(), "🧯️ a focus-trapping dialog DOES swallow the chord, which is React's editable-target rule");

    let mut shell = shell_with_escape_binding();
    let mut input = InputState::<ActionDescriptor>::default();
    shell.overlay_state = OverlayState::Dropdown("example".into());
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input)).expect("a chord never errors here");
    assert_eq!(shell.overlay_state, OverlayState::None, "⎋️ the picker is dismissed by the same keydown");
    assert_eq!(shell.transient_notice().map(|notice| notice.code.clone()), Some(Some(KEYBINDING_UNOWNED_CODE.to_string())), "⎋️ and the app's own escape binding was still resolved and addressed");
}

//#endregion ⎋️DismissableLayers

/// ⚖️ LAW: an app binding whose verb NO mounted window owns does not end the chord ladder — the
/// framework-universal `mod+z`/`mod+shift+z` tail below it still runs, exactly as React's keybinding
/// loop `continue`s past an unresolved target and its own tail still fires.
///
/// 🩸️ Measured live: after the parity journey's `window-cap-close` steps the wgpu dock held ZERO
/// windows (`chord gate … window=None … docked=0`), puzzle3d's own `undo` binding matched, its target
/// window could not be resolved, the unowned-chord banner was raised and `handle_keyboard_async`
/// returned — so `chord-undo` and `chord-redo` journalled nothing at all where React journals `undo`
/// and `redo` (`🗑️generated/w12c-parity-run-17/`).
#[test]
fn an_unowned_app_chord_falls_through_to_the_framework_edit_verbs() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut app = test_app(Vec::new(), Vec::new());
    app.keybindings = vec![Keybinding { keys: "mod+z".into(), action: ActionDescriptor { controller_id: "app.controller".into(), action: "undo".into(), args: None } }];
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::default() });
    let mut input = InputState::<ActionDescriptor>::default();
    let modifiers = PointerModifiers { meta: true, ..PointerModifiers::default() };

    assert!(shell.match_app_keybinding(&ui_wgpu::wgpu::KeyAction::Char("z".into()), &modifiers).is_some(), "⌨️ the app claims the chord");
    let outcome = semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Char("z".into()), &modifiers, &mut input));
    assert_eq!(shell.transient_notice().map(|notice| notice.code.clone()), Some(Some(KEYBINDING_UNOWNED_CODE.to_string())), "⌨️ the app rung ran and found no window to address");
    assert_eq!(outcome.err().as_deref(), Some("action program missing"), "⌨️ and the framework's own undo still crossed the dispatch funnel");
}

//#region 🪟️WindowActivationOnBodyPress

/// ⚖️ LAW: a press anywhere inside a window's own box ACTIVATES that window, and a real in-session
/// change of the active window arms exactly one `shell.windowActivate` history note.
///
/// React's is a capture-phase handler on the window element itself — `🪟️Window/🟦️.tsx`'s
/// `onPointerDownCapture={(event) => { if (!isSurfaceActiveBackgroundPointer(event)) onActivate?.(); }}`
/// — so it fires for the window's chrome AND for the scene canvas filling its body, before any inner
/// handler runs. It reaches `Mode`'s `activateWindow` (`🎨️Canvas/🟦️.tsx:1446`), whose
/// `onActiveWindowChange` is the funnel `🏛️ShellHost/🟦️.tsx:10389` notes the command from.
///
/// 🩸️ This renderer had eleven sites writing `active_window_id` and not one of them was a press into
/// a window BODY, so orbiting a pane never activated it: React's `orbit-drag` step journals a
/// `noteShellCommand` for the press into the perspective pane and the wgpu pane journalled none
/// (`🗑️generated/w12c-parity-run-19/steps.json` step 19).
#[test]
fn a_press_inside_a_window_body_activates_that_window_and_notes_it() {
    let mut shell = shell_with_escape_binding();
    shell.dock_window_plan = vec![("puzzle3d-main-top".into(), Rect::new(3.2, 54.4, 531.2, 913.6)), ("puzzle3d-main-perspective".into(), Rect::new(PANE.0, PANE.1, PANE.2, PANE.3))];
    shell.active_window_id = Some("puzzle3d-main-top".into());
    let mut input = InputState::<ActionDescriptor>::default();
    shell.publish_retained_input_for_test(&mut input, &Theme::light());
    shell.arm_window_activation_note();
    shell.deferred_actions.clear();

    assert!(shell.activate_window_under_pointer(AIM.0, AIM.1, &Theme::light()), "🪟️ the probe's aim point is inside the perspective pane's window box");
    assert_eq!(shell.active_window_id.as_deref(), Some("puzzle3d-main-perspective"));
    shell.arm_window_activation_note();
    let notes = shell.deferred_actions.iter().filter(|action| action.action == "noteShellCommand").count();
    assert_eq!(notes, 1, "🪟️ one real activation arms exactly one history note");

    shell.deferred_actions.clear();
    assert!(!shell.activate_window_under_pointer(AIM.0 + 1.0, AIM.1 + 1.0, &Theme::light()), "🪟️ a second press in the SAME window changes nothing");
    shell.arm_window_activation_note();
    assert!(shell.deferred_actions.is_empty(), "🪟️ so it notes nothing — React's `activateWindow` returns early on an unchanged id");
}

/// ⚖️ LAW: a modal layer owns the press and activates no window — React's menus, dropdowns, dialogs
/// and tour are portaled OUTSIDE the window element, so no capture handler of theirs ever fires. The
/// empty-id row the dock plans after a cap close names no window either.
#[test]
fn a_modal_layer_or_an_unnamed_dock_row_activates_no_window() {
    let mut shell = shell_with_escape_binding();
    shell.dock_window_plan = vec![("puzzle3d-main-perspective".into(), Rect::new(PANE.0, PANE.1, PANE.2, PANE.3))];
    shell.overlay_state = OverlayState::Dropdown("example".into());
    shell.open_selects.insert("example".into(), true);
    let mut input = InputState::<ActionDescriptor>::default();
    shell.publish_retained_input_for_test(&mut input, &Theme::light());
    assert!(shell.pointer_input_is_modal());
    assert!(!shell.activate_window_under_pointer(AIM.0, AIM.1, &Theme::light()), "🚧️ a modal layer owns every pointer, wherever it lands");
    assert!(shell.active_window_id.is_none());

    let mut shell = shell_with_escape_binding();
    shell.dock_window_plan = vec![(String::new(), Rect::new(PANE.0, PANE.1, PANE.2, PANE.3))];
    shell.publish_retained_input_for_test(&mut input, &Theme::light());
    assert!(!shell.activate_window_under_pointer(AIM.0, AIM.1, &Theme::light()), "🕳️ the dock's unnamed window row is not a window");
    assert!(shell.active_window_id.is_none());
}

//#endregion 🪟️WindowActivationOnBodyPress

//#region 🧊️IngressAnchors

/// 🖱️ A real secondary press opens the menu and reaches only its captured World.
#[test]
fn a_plain_secondary_press_over_a_pane_opens_the_menu_and_still_presses_the_surface() {
    super::shell_input_tests::retained_world_sequence_probe("secondary");
}

/// ⚖️ LAW (source anchor + oracle): a modal layer OPENING hands every surface that still publishes a
/// hover exactly one leave, in the frame the layer opened — React's overlays take pointer events the
/// instant they mount, so r3f raises `onPointerOut` there and then.
///
/// 🩸️ This renderer only handed a surface its leave on the next MOVE, so the clear rode along to the
/// next step that moved the pointer at all: the stray `interactionHover targets:[]` in
/// `example-picker-open`, ten steps and six keyboard chords after the menu that caused it
/// (`🗑️generated/w12c-parity-run-19/steps.json` step 33).
#[test]
fn a_modal_layer_opening_clears_every_published_hover_in_that_frame() {
    let anchor = "if modal && world3d_hover_clear_is_owed(state) && enqueue_world3d_event(state, WorldInteractionIntent::pointer_leave(";
    assert!(WGPU_RENDERER_SOURCE.contains(anchor), "🚧️ the world authority phase clears a published hover while a modal layer is up");
    assert!(
        !WGPU_RENDERER_SOURCE.contains("if modal && world3d_hover_is_published(state)"),
        "🚧️ and it asks the OWED predicate, never the published one — a per-FRAME caller that asks `is_published` re-enqueues one leave per frame until the bounded queue saturates and the frame faults"
    );
    assert!(WGPU_RENDERER_SOURCE.contains("let modal = shell.pointer_input_is_modal();"), "🚧️ and it asks the shell's own modal predicate, the one W12a's overlay laws already pin");

    let fixture: serde_json::Value = serde_json::from_str(WORLD_JOURNAL_SEQUENCES).expect("🧫️ the world journal oracle parses");
    let case = fixture["steps"].as_array().expect("steps").iter().find(|case| case["id"] == "modal-open").expect("🧫️ the oracle carries the modal-open step");
    assert_eq!(case["react"], case["wgpu"], "🧫️ and both renderers publish the SAME single hover clear for it");
}

//#endregion 🧊️IngressAnchors
