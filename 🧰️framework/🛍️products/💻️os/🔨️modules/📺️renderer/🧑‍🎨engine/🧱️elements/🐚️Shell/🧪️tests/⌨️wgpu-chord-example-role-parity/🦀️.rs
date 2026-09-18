//! ⌨️📚️👁️ Laws for the wgpu shell's chord dispatch, navbar example picker and role switch against the
//! React shell's OWN measured journals (packet W10b of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
//!
//! Every expectation here is a fixture read off `🗑️generated/parity-run-2/steps.json` — the React
//! renderer driven through `🐍️parity-interact-probe.mjs`'s journey, one row per step, reproduced
//! identically by runs 5, 7 and 8. A step matches when both renderers dispatch the same SET of action
//! ids and move the same surfaces, so these laws pin exactly those two things:
//!
//! | step | React actions | React surfaces |
//! | --- | --- | --- |
//! | `chord-command-palette` (`mod+k`) | (none) | unchanged |
//! | `chord-escape` (`escape`) | `engagementAbort` | unchanged |
//! | `chord-undo` (`mod+z`) | `undo`, guest `shell.windowActivate` | unchanged |
//! | `chord-redo` (`mod+shift+z`) | `redo`, guest `shell.windowActivate` | unchanged |
//! | `chord-fullscreen(-exit)` (`mod+shift+f`) | (none) | unchanged |
//! | `chord-panel-anchor-left/right` (`mod+alt+1`/`2`) | (none) | unchanged |
//! | `example-picker-open` | (none) | unchanged |
//! | `example-switch` | `setActiveExample` + guest `registerBrushMesh` | unchanged |
//! | `example-picker-dismiss` (`escape`) | `engagementAbort` | unchanged |
//! | `role-viewer` / `role-editor` | `setActiveExample` + guest `registerBrushMesh` | window/panel swap |

use super::command_registry_tests::test_app;
use super::*;
use semio_framework::manifest::Keybinding;

//#region ⌨️InertProbeChords

/// 🧪️ The key event a probe chord arrives as. `mod` is the platform accelerator the probe presses
/// (`Meta` on this host), which is exactly how `key_event_matches_chord` reads an accelerator.
fn probe_chord(chord: &str) -> (ui_wgpu::wgpu::KeyAction, PointerModifiers) {
    let mut modifiers = PointerModifiers::default();
    let mut key = "";
    for token in chord.split('+') {
        match token {
            "mod" => modifiers.meta = true,
            "shift" => modifiers.shift = true,
            "alt" => modifiers.alt = true,
            other => key = other,
        }
    }
    let action = match key {
        "escape" => ui_wgpu::wgpu::KeyAction::Escape,
        other => ui_wgpu::wgpu::KeyAction::Char(other.to_string()),
    };
    (action, modifiers)
}

/// ⚖️ LAW: the three probe chords React answers with NOTHING are chords this shell claims nothing for
/// either — `mod+k`, `mod+alt+1` and `mod+alt+2` name no row of [`SHELL_SHORTCUT_ROWS`] and reserve
/// nothing, so they fall through to the app exactly as they do in React.
///
/// 🩸️ The probe NAMES these steps `chord-command-palette` and `chord-panel-anchor-left/right`, which
/// reads as though React opened a palette and moved an anchor. It does neither: React's palette chord
/// is `mod+p` (`ui.search.toggle`) and its anchors are the `ctrl|meta+b`/`+m` family, so all three
/// probe chords are inert on React and parity means being inert here too — journaling a verb for any
/// of them would be the difference, not the fix.
#[test]
fn the_inert_probe_chords_claim_no_shell_verb_and_reserve_nothing() {
    for chord in ["mod+k", "mod+alt+1", "mod+alt+2"] {
        let (action, modifiers) = probe_chord(chord);
        assert_eq!(shell_shortcut_for(&action, &modifiers), None, "{chord} names no shell verb, exactly as React answers it with nothing");
        assert!(!is_reserved_shell_chord(&action, &modifiers), "{chord} is not the shell's to reserve");
    }
    let (palette, palette_modifiers) = probe_chord("mod+p");
    assert_eq!(shell_shortcut_for(&palette, &palette_modifiers), Some(ShellShortcut::ToggleSearch), "the palette chord React actually binds");
    println!("[DEBUG] inert probe chords mod+k / mod+alt+1 / mod+alt+2 claim nothing; mod+p still opens the palette");
}

/// ⚖️ LAW: `mod+shift+f` IS the full-screen row, and taking it journals NO action — React's
/// `chord-fullscreen`/`chord-fullscreen-exit` steps each journal an empty set and move no surface.
/// The verb is a pure chrome latch (`fullscreen_toggle_requested`), never a dispatch, and the surface
/// census (`chrome_surface_census`) has no fullscreen term at all.
#[test]
fn the_fullscreen_chord_flips_a_chrome_latch_and_journals_nothing() {
    let (action, modifiers) = probe_chord("mod+shift+f");
    assert_eq!(shell_shortcut_for(&action, &modifiers), Some(ShellShortcut::ToggleFullscreen));
    assert!(ShellShortcut::ToggleFullscreen.is_async(), "the full-screen verb takes the async funnel");

    let mut shell = ShellState::new(Vec::new(), String::new());
    assert!(!shell.fullscreen_toggle_requested, "sanity: nothing requested yet");
    semio_framework_async::block_on(shell.apply_os_command("os.toggleFullscreen", None)).expect("the os command runs");
    assert!(shell.fullscreen_toggle_requested, "the chord arms the host's own full-screen request");
    assert!(shell.deferred_actions.is_empty(), "…and arms no action, which is why React's journal for both fullscreen steps is empty");
}

//#endregion ⌨️InertProbeChords

//#region ⌨️AppChordDispatch

/// 🧪️ A session whose app declares `keybindings`, so the app-keybinding rung has something to match.
fn shell_with_keybindings(keybindings: Vec<Keybinding>) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut app = test_app(Vec::new(), Vec::new());
    app.keybindings = keybindings;
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::default() });
    shell
}

fn binding(keys: &str, action: &str) -> Keybinding {
    Keybinding { keys: keys.into(), action: ActionDescriptor { controller_id: "test".into(), action: action.into(), args: None } }
}

/// ⚖️ LAW: Escape resolves the app's OWN `escape` binding — which is what makes React's `chord-escape`
/// and `example-picker-dismiss` steps each journal exactly one `engagementAbort` (puzzle3d's editor
/// declares `escape → engagementAbort`), rather than a shell verb of this renderer's invention.
#[test]
fn escape_resolves_the_apps_own_engagement_abort_binding() {
    let shell = shell_with_keybindings(vec![binding("escape", "engagementAbort")]);
    let resolved = shell.match_app_keybinding(&ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default());
    assert_eq!(resolved.map(|descriptor| descriptor.action), Some("engagementAbort".to_string()));
}

/// ⚖️ LAW: a COMMA-SEPARATED chord list matches on every chord it declares — React splits `keys` with
/// `parseKeys` before matching, and puzzle2d's `deleteSelection` is declared `"delete,backspace"`.
///
/// 🩸️ This target used to hand the whole string to `key_event_matches_chord`, which splits on `+`
/// alone: `"delete,backspace"` resolved the key token `"backspace"` carrying a literal `delete,`
/// prefix and therefore matched NEITHER key, so every multi-chord binding in the repo was dead here.
#[test]
fn a_comma_separated_binding_matches_each_chord_it_declares() {
    let shell = shell_with_keybindings(vec![binding("delete,backspace", "deleteSelection")]);
    for action in [ui_wgpu::wgpu::KeyAction::Delete, ui_wgpu::wgpu::KeyAction::Backspace] {
        let resolved = shell.match_app_keybinding(&action, &PointerModifiers::default());
        assert_eq!(resolved.map(|descriptor| descriptor.action), Some("deleteSelection".to_string()), "{action:?} is one of the binding's own chords");
    }
}

/// ⚖️ LAW: the reserved check is PER CHORD, where React puts it (`if (reservedChords.has(chord))
/// continue`) — a binding that also claims a shell chord loses THAT chord and keeps the others,
/// instead of being struck out whole.
#[test]
fn a_binding_that_also_claims_a_shell_chord_keeps_its_other_chords() {
    let shell = shell_with_keybindings(vec![binding("mod+p,mod+d", "duplicateSelection")]);
    let (shell_chord, shell_modifiers) = probe_chord("mod+p");
    assert_eq!(shell.match_app_keybinding(&shell_chord, &shell_modifiers), None, "mod+p is the shell's palette chord and never an app's");
    let (app_chord, app_modifiers) = probe_chord("mod+d");
    assert_eq!(shell.match_app_keybinding(&app_chord, &app_modifiers).map(|descriptor| descriptor.action), Some("duplicateSelection".to_string()));
}

/// ⚖️ LAW: `mod+z` / `mod+shift+z` / `mod+y` resolve to the framework-universal undo/redo verbs an app
/// may shadow — the action ids React's `chord-undo` and `chord-redo` steps journal.
#[test]
fn the_undo_and_redo_chords_resolve_the_framework_edit_verbs() {
    for (chord, expected) in [("mod+z", ShellEditVerb::Undo), ("mod+shift+z", ShellEditVerb::Redo), ("mod+y", ShellEditVerb::Redo)] {
        let (action, modifiers) = probe_chord(chord);
        assert_eq!(shell_edit_verb_for(&action, &modifiers), Some(expected), "{chord}");
        assert!(!is_reserved_shell_chord(&action, &modifiers), "{chord}: an app that declares it shadows the framework verb, exactly as React orders its own tail");
    }
    assert_eq!(ShellEditVerb::Undo.action_id(), "undo");
    assert_eq!(ShellEditVerb::Redo.action_id(), "redo");
}

//#endregion ⌨️AppChordDispatch

//#region 🪟️WindowActivationNote

/// ⚖️ LAW: only a REAL activation inside one live instance is a `shell.windowActivate` note — the
/// guard behind React's `handleActiveWindowChange`. The first observation seeds the witness, an
/// unchanged window notes nothing, and a change that crosses a session INSTANCE is a mount.
#[test]
fn only_a_real_in_session_activation_is_a_window_activate_note() {
    assert!(!window_activation_is_user_v1(None, 1, "w-a"), "the first observation seeds the witness in silence");
    assert!(!window_activation_is_user_v1(Some(&(1, "w-a".into())), 1, "w-a"), "an unchanged active window notes nothing");
    assert!(window_activation_is_user_v1(Some(&(1, "w-a".into())), 1, "w-b"), "the user activated another window of the same instance");
    assert!(!window_activation_is_user_v1(Some(&(1, "w-a".into())), 2, "w-b"), "a successor instance's landing window is a mount, not an activation");
    assert!(!window_activation_is_user_v1(Some(&(1, "w-a".into())), 1, ""), "no window is no activation");
}

/// ⚖️ LAW: the note this shell arms IS a `noteShellCommand` carrying `shell.windowActivate` and the
/// window it activated — the command React's own `handleActiveWindowChange` pushes onto the guest's
/// history, and therefore the command the guest replays when `mod+z` pops it.
///
/// 🩸️ React's `chord-undo` journals TWO rows: `undo` (origin `user`) and `shell.windowActivate`
/// (origin `guest`, refused `undeclared-action`) — the guest's replay of the top of its own command
/// stack. This shell never noted an activation at all, so its stack top was some other command and
/// `mod+z` could not journal React's pair however faithfully the undo itself was dispatched.
#[test]
fn a_real_activation_arms_the_shell_window_activate_history_note() {
    let mut shell = shell_with_keybindings(Vec::new());
    shell.active_window_id = Some("main".into());
    shell.arm_window_activation_note();
    assert!(shell.deferred_actions.is_empty(), "the first observation seeds the witness without noting");

    shell.active_window_id = Some("second".into());
    shell.arm_window_activation_note();
    let armed = shell.deferred_actions.first().expect("a real activation arms exactly one note");
    assert_eq!(armed.action, "noteShellCommand");
    let args = armed.args.as_ref().expect("the note carries its command");
    assert_eq!(args.get("commandId").and_then(DslValue::as_str), Some("shell.windowActivate"));
    assert_eq!(args.get("detail").and_then(|detail| detail.get("windowId")).and_then(DslValue::as_str), Some("second"));
    assert_eq!(shell.deferred_actions.len(), 1, "one activation, one note");

    shell.arm_window_activation_note();
    assert_eq!(shell.deferred_actions.len(), 1, "…and re-observing the same active window notes nothing again");
    println!("[DEBUG] window activation note armed for {:?}", shell.active_window_id);
}

//#endregion 🪟️WindowActivationNote

//#region 📚️ExamplePickerAndRole

/// ⚖️ LAW: the picker's trigger is React's own `playground.navbar.fixture` and every row is
/// `shell.example.<id>` — the control ids the probe resolves on both renderers. React's own 13 added
/// controls for `example-picker-open` are Radix Select internals (`semio-select-…`, `select-viewport`,
/// `select-item-text`), which no canvas renderer publishes and which the verdict deliberately ignores.
#[test]
fn the_example_picker_publishes_reacts_trigger_and_row_control_ids() {
    let rows = vec![
        ShellExampleRow { control_id: "shell.example.forest".into(), label: "Concrete Forest".into(), selected: true },
        ShellExampleRow { control_id: "shell.example.nakagin".into(), label: "Nakagin Capsule Tower".into(), selected: false },
    ];
    let control = shell_example_control(&rows, true, false).expect("a dialect with examples renders the trigger");
    assert_eq!(control.control_id, "playground.navbar.fixture");
    assert_eq!(control.label, "Concrete Forest", "a select shows its value");
    assert!(control.active, "an open picker paints its trigger active");
    assert!(shell_example_control(&[], false, false).is_none(), "a dialect with no example renders no picker, exactly as React's `exampleOptions.length > 0` gate");
}

/// ⚖️ LAW: Escape closes an OPEN example dropdown before any other rung sees it — the topmost-overlay
/// rule this target already applies to retained selects and the context menu.
///
/// 🩸️ The navbar dropdown had NO keyboard route out: `open_selects` knows only the retained widget
/// selects and `palette_open` covers `Search`/`Find`, so an open picker swallowed every later chord.
/// React closes it through Radix's `DismissableLayer`, which is why its `example-picker-dismiss` step
/// can journal the app's plain `engagementAbort` instead.
#[test]
fn escape_closes_the_open_example_dropdown() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    shell.overlay_state = OverlayState::Dropdown("example".into());
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input);
    assert_eq!(shell.overlay_state, OverlayState::None, "the picker is dismissed");
    assert!(shell.deferred_actions.is_empty(), "dismissing an overlay is chrome state, never a dispatch");
}

/// 🧪️ The error a `dispatch_action` reaches on a fixture with no loaded plugin — proof the action
/// crossed the funnel (and therefore the chrome ledger's tap at its very top) rather than being
/// skipped, which is the only observable a registry-less shell can give.
const NO_PROGRAM: &str = "action program missing";

/// ⚖️ LAW: clicking a row selects that example AND dispatches `setActiveExample` — the one action
/// React's `example-switch` step journals beside the guest's own `registerBrushMesh` echoes.
#[test]
fn picking_an_example_row_selects_it_and_dispatches_set_active_example() {
    let mut shell = shell_with_keybindings(Vec::new());
    shell.overlay_state = OverlayState::Dropdown("example".into());
    let row = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some("shell.example.nakagin".into()), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None };
    let dispatched = semio_framework_async::block_on(shell.handle_shell_hit(&row));
    assert_eq!(dispatched.err().as_deref(), Some(NO_PROGRAM), "the row dispatched `setActiveExample` all the way to the guest hop this fixture has no program for");
    assert_eq!(shell.active_example_id.as_deref(), Some("nakagin"), "the picker's own selection follows the click");
    assert_eq!(shell.overlay_state, OverlayState::None, "…and the dropdown closes behind it");
}

/// ⚖️ LAW: a freshly mounted instance is TOLD its example — React re-runs its boot-example effect per
/// session instance (`noExampleResetInstanceIdRef`), which is why `role-viewer` and `role-editor` each
/// journal `setActiveExample` even though neither click touched the picker.
///
/// 🩸️ `apply_boot_example` covers the boot alone and only for `?example=`, so this shell's role switch
/// mounted its successor on the dialect's first document and journalled nothing.
#[test]
fn a_freshly_mounted_instance_is_announced_its_resolved_example() {
    let mut shell = shell_with_keybindings(Vec::new());
    shell.active_example_id = Some("nakagin".into());
    let announced = semio_framework_async::block_on(shell.announce_session_example());
    assert_eq!(announced.err().as_deref(), Some(NO_PROGRAM), "the mount dispatched `setActiveExample` down to the guest hop this fixture has no program for");

    let mut without = shell_with_keybindings(Vec::new());
    without.active_example_id = None;
    assert!(semio_framework_async::block_on(without.announce_session_example()).is_ok(), "a dialect with no example announces nothing at all");
    println!("[DEBUG] session example announcement dispatched for {:?}", shell.active_example_id);
}

/// ⚖️ LAW: the two role chips carry React's own control ids and each one's chord, so the pointer and
/// the keyboard reach the SAME verb — `role-viewer`/`role-editor` are the probe's click steps and
/// `mod+alt+v`/`mod+alt+e` the chords of the same two rows.
#[test]
fn the_role_chips_and_their_chords_name_the_same_two_verbs() {
    for (control_id, chord, role) in [
        ("playground.navbar.roles.editor", "mod+alt+e", semio_framework::manifest::AppRole::Editor),
        ("playground.navbar.roles.viewer", "mod+alt+v", semio_framework::manifest::AppRole::Viewer),
    ] {
        assert_eq!(shell_shortcut_for_control_id(control_id), Some(ShellShortcut::SurfaceRole(role)), "{control_id}");
        let (action, modifiers) = probe_chord(chord);
        assert_eq!(shell_shortcut_for(&action, &modifiers), Some(ShellShortcut::SurfaceRole(role)), "{chord}");
        assert!(ShellShortcut::SurfaceRole(role).is_async(), "a role switch is a transactional session switch");
    }
}

//#endregion 📚️ExamplePickerAndRole
