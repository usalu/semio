//! 🛰️ LAWS: the dock's close/reopen lane and everything it journals — packet W13a of ticket
//! 26/09/17/WGPU-RENDERER-REACT-PARITY.
//!
//! The parity journey drives three chrome steps against the dock's own tab caps and then every
//! window-owned chord below them. In `🗑️generated/w12c-parity-run-19/steps.json` the wgpu dock ended
//! those three steps at ZERO windows while React's kept both, and with no window left every later
//! `escape`/`mod+z`/`mod+shift+z` resolved its verb and found nothing to address:
//!
//! | step | control the probe resolved | wgpu result (run 19) | React result (run 19) |
//! | --- | --- | --- | --- |
//! | `window-cap-focus` | `dock.tab.0.puzzle3d-main-top.focus` | `noteShellCommand shell.windowMaximize` | (none) |
//! | `window-cap-close` | `dock.tab.0.puzzle3d-main-top.close` | `noteShellCommand shell.windowClose`, one window closed | (none) |
//! | `window-reopen` | `dock.tab..puzzle3d-main-perspective.close` | the SECOND window closed, `windows=["", …]` | `mode-dock-tabbar`, nothing happened |
//!
//! Three separate defects produced that row, and each has its own law below: the tab published its
//! destructive chip before its own select target (so "find the dock tab" resolved `close`), the
//! emptied stack still minted a window with an EMPTY id, and the Focus chip journaled a
//! `shell.windowMaximize` command React declares nowhere.

use super::command_registry_tests::test_app;
use super::*;
use semio_framework::manifest::Keybinding;
use semio_framework::{ActionDefinition, ActionKind, AppDefinition, WindowKinds};
use ui_wgpu::wgpu::InputState;

/// 🪟️ The journey's own two world panes and the window kind both instantiate — a Row of two
/// single-tab stacks, which is what makes the caps read `dock.tab.0.…` and `dock.tab.1.…`.
const TOP: &str = "puzzle3d-main-top";
const PERSPECTIVE: &str = "puzzle3d-main-perspective";
const CANVAS: Rect = Rect::new(0.0, 0.0, 1600.0, 1000.0);

fn journey_dock() -> DockState {
    let stack = |window_id: &str| crate::dock::DockNode::Stack { windows: vec![DockStackTab::instance(window_id, "main", WindowStackCorner::TopLeft)], active: window_id.to_string() };
    let mut dock = DockState::default();
    dock.root = crate::dock::DockNode::Row(vec![(stack(TOP), 0.5), (stack(PERSPECTIVE), 0.5)]);
    dock.sync_active_window(TOP);
    dock
}

/// 🧪️ The shell the journey reaches at `window-cap-focus`: a live session whose single window kind
/// declares `engagementAbort` (puzzle3d's own `escape` verb) and a dock holding both panes.
fn journey_shell(keybindings: Vec<Keybinding>) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let app = test_app(Vec::new(), Vec::new());
    let mut kind = app.window_kinds.first().clone();
    kind.actions = vec![ActionDefinition::new("engagementAbort", LocalizedLabel::data("Abort"), ActionKind::Interaction, "eye")];
    let mut app = AppDefinition { window_kinds: WindowKinds::try_from(vec![kind]).expect("one window kind"), ..app };
    app.keybindings = keybindings;
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::default() });
    shell.dock = journey_dock();
    shell.active_window_id = Some(TOP.to_string());
    shell
}

fn cap(control_id: &str) -> HitTarget<ActionDescriptor> {
    HitTarget::<ActionDescriptor> { rect: Rect::new(46.0, 32.0, 17.2, 22.4), event: None, control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }
}

/// 📑️ The `dock.tab.…` rows the dock publishes for this frame, in publication order — the exact list
/// `semioWgpuIntrospection.dumpChrome().hits` hands the parity probe, and therefore the list its
/// `reopenWindow` scan reads top-down.
fn published_dock_tab_rows(shell: &ShellState) -> Vec<String> {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([(TOP.to_string(), "Top".to_string()), (PERSPECTIVE.to_string(), "Perspective".to_string())]);
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids };
    shell.dock.paint_chrome(&mut ctx, CANVAS, false);
    input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).filter(|id| id.starts_with("dock.tab.")).collect()
}

fn accepted_dock_input(shell: &mut ShellState) -> InputState<ActionDescriptor> {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([(TOP.to_string(), "Top".to_string()), (PERSPECTIVE.to_string(), "Perspective".to_string())]);
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids };
    shell.dock.paint_chrome(&mut ctx, CANVAS, false);
    shell.publish_retained_hit_registry(&mut input);
    input
}

fn plan(shell: &mut ShellState) {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    shell.plan_dock_windows(CANVAS, &theme, &mut atlas);
}

//#region 🪟️CloseAndReopen

/// ⚖️ LAW: the Close chip closes EXACTLY the window it names and focus moves to the first survivor —
/// React's `closeWindow(windowId)` (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1475-1485`: one
/// `removeWindowFromLayout`, then `onActiveWindowChange(remaining[0] ?? null)`).
///
/// 🧾️ The retained deferred action is the proof the close was JOURNALED: React's `onWindowClose`
/// notes `shell.windowClose` with the closed window (`🏛️ShellHost/🟦️.tsx:10530-10531`). Keeping that
/// guest round trip outside the pointer dispatch returns the exact interaction owner before the
/// next presented-input candidate needs it.
#[test]
fn the_close_cap_closes_exactly_the_clicked_window_and_refocuses_the_survivor() {
    let mut shell = journey_shell(Vec::new());
    let outcome = semio_framework_async::block_on(shell.handle_shell_hit(&cap(&format!("dock.tab.0.{TOP}.close")), &InputState::<ActionDescriptor>::default()));
    assert_eq!(shell.dock.collect_window_ids(), vec![PERSPECTIVE.to_string()], "🪟️ one window closed, the other stayed");
    assert_eq!(shell.active_window_id.as_deref(), Some(PERSPECTIVE), "🪟️ focus moved to the survivor, as React's `remaining[0]` does");
    assert!(outcome.is_ok(), "🕒️ the close returns its interaction owner without awaiting the guest journal");
    let note = shell.deferred_actions.last().expect("🕒️ the close arms the note React's onWindowClose uses");
    assert_eq!(note.action, "noteShellCommand");
    let args = note.args.as_ref().expect("the close note carries its command and window");
    assert_eq!(args.get("commandId").and_then(DslValue::as_str), Some("shell.windowClose"));
    assert_eq!(args.get("detail").and_then(|detail| detail.get("windowId")).and_then(DslValue::as_str), Some(TOP));
}

/// ⚖️ LAW: retiring a Window Options popup never consumes the next, independent close-cap
/// gesture. The popup dismissal and the cap press are two complete pointer sequences over the same
/// accepted frame, matching the rapid physical sequence from checkpoint 20. One cap press closes
/// exactly Top, focuses Perspective and journals exactly one close command.
#[test]
fn a_window_options_popup_dismissal_preserves_the_immediate_close_cap_gesture() {
    let fixture: serde_json::Value = serde_json::from_str(JOURNAL_SEQUENCES).expect("the rapid popup-close fixture parses");
    let law = &fixture["rapidPopupClose"];
    let mut shell = journey_shell(Vec::new());
    let mut input = accepted_dock_input(&mut shell);
    shell.open_selects.insert(law["popupOwner"].as_str().expect("popup owner").to_string(), true);

    let dismiss = law["dismissPoint"].as_array().expect("dismiss point");
    let (dismiss_x, dismiss_y) = (dismiss[0].as_f64().expect("dismiss x") as f32, dismiss[1].as_f64().expect("dismiss y") as f32);
    semio_framework_async::block_on(shell.handle_pointer_button(dismiss_x, dismiss_y, true, 0, &mut input, &Theme::default())).expect("the outside press dismisses the Window Options popup");
    semio_framework_async::block_on(shell.handle_pointer_button(dismiss_x, dismiss_y, false, 0, &mut input, &Theme::default())).expect("the popup dismissal completes");
    assert!(shell.open_selects.values().all(|open| !open), "🔽️ the popup is retired before the cap gesture starts");

    let close_id = law["closeControl"].as_str().expect("close control");
    let close = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(close_id)).expect("the accepted frame publishes Top's close cap").rect;
    let (x, y) = (close.x + close.w * 0.5, close.y + close.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &Theme::default())).expect("the immediate close press routes");
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, false, 0, &mut input, &Theme::default())).expect("the immediate close release routes");

    let expected_windows = law["remainingWindows"].as_array().expect("remaining windows").iter().map(|window| window.as_str().expect("window id").to_string()).collect::<Vec<_>>();
    assert_eq!(shell.dock.collect_window_ids(), expected_windows, "🪟️ one immediate cap gesture removes Top");
    assert_eq!(shell.active_window_id.as_deref(), law["focusedWindow"].as_str(), "🪟️ the survivor receives focus");
    assert!(shell.presented_input_candidate.is_none(), "🎯️ no stale input candidate remains after the accepted gestures");
    let close_notes = shell
        .deferred_actions
        .iter()
        .filter(|action| action.action == "noteShellCommand" && action.args.as_ref().and_then(|args| args.get("commandId")).and_then(DslValue::as_str) == law["closeCommand"].as_str())
        .count();
    assert_eq!(close_notes, law["closeCommandCount"].as_u64().expect("close command count") as usize, "🕒️ the one cap gesture journals one close command");
}

/// ⚖️ LAW: after that close, the FIRST `dock.tab.…` row the dock publishes is the survivor's SELECT
/// target — never its Close chip — and pressing it closes nothing and journals nothing.
///
/// 🩸️ This is the whole of the `window-reopen` divergence. The probe resolves "the dock tab" by
/// scanning the published rows (`🐍️parity-interact-probe.mjs`'s `reopenWindow`,
/// `/mode-dock-tab|dock\.tab|appSwitcher|window.*tab/i`): on React the scan lands on the
/// `mode-dock-tabbar` container that OWNS the tabs and the step does nothing
/// (`🗑️generated/w12c-parity-run-19/steps.json` step 18: `controlCount [335, 335]`), while here the
/// chips were published before the select target and the scan landed on
/// `dock.tab..puzzle3d-main-perspective.close`, closing the last world pane and leaving `docked=0`
/// for every chord below it.
#[test]
fn the_reopen_scan_after_a_close_lands_on_a_tab_select_that_closes_nothing() {
    let mut shell = journey_shell(Vec::new());
    let _ = semio_framework_async::block_on(shell.handle_shell_hit(&cap(&format!("dock.tab.0.{TOP}.close")), &InputState::<ActionDescriptor>::default()));
    let rows = published_dock_tab_rows(&shell);
    let scanned = rows.first().cloned().expect("🛰️ the survivor still publishes its tab");
    assert_eq!(scanned, format!("dock.tab..{PERSPECTIVE}"), "🛰️ the reopen scan resolves a SELECT target: {rows:?}");
    assert!(ShellState::shell_command_for_control(&scanned, false).is_none(), "🕒️ a tab select is no shell command — React's reopen step journals nothing");

    let before = shell.dock.collect_window_ids();
    shell.deferred_actions.clear();
    let outcome = semio_framework_async::block_on(shell.handle_shell_hit(&HitTarget::<ActionDescriptor> { rect: CANVAS, event: None, control_id: Some(scanned), kind: HitKind::Window, drag_axis: None, drag_data: None }, &InputState::<ActionDescriptor>::default()));
    assert!(outcome.is_ok(), "🛰️ the reopen step crosses no dispatch funnel");
    assert_eq!(shell.dock.collect_window_ids(), before, "🛰️ …and closes nothing: the dock survives the whole cap sequence");
    assert!(shell.deferred_actions.is_empty(), "🕒️ the survivor was already active, so not even an activation is owed");
}

/// ⚖️ LAW: a tab publishes its select target before its chips, and its chips left to right — React's
/// DOM order (`🎨️Canvas/🟦️.tsx:1034-1100`). Stated here as well as in `🛰️Dock`'s own suite because
/// this is the order the SHELL's published hit ledger hands every consumer.
#[test]
fn every_dock_tab_publishes_its_select_target_before_its_chips() {
    let shell = journey_shell(Vec::new());
    let rows = published_dock_tab_rows(&shell);
    for window_id in [TOP, PERSPECTIVE] {
        let position = |suffix: &str| rows.iter().position(|id| id.ends_with(&format!("{window_id}{suffix}"))).unwrap_or_else(|| panic!("🛰️ {window_id}{suffix} is published: {rows:?}"));
        assert!(position("") < position(".focus"), "🛰️ {window_id}: select precedes Focus");
        assert!(position(".focus") < position(".close"), "🛰️ {window_id}: Focus precedes Close");
        assert!(position(".close") < position(".drag"), "🛰️ {window_id}: Close precedes the grip");
    }
}

/// ⚖️ LAW: an emptied dock is still a DROP body and is no window — React's `WindowChrome` keeps
/// rendering for an empty stack while its `activeDescriptor` is `undefined` and its body renders
/// nothing (`🎨️Canvas/🟦️.tsx:1176`/`:1212`).
///
/// 🩸️ The plan carried the empty stack's `active` id through, so closing the last window minted a
/// window whose id was `""`: the engine-surface census reported `windows=["", "tool.fill", …]`, the
/// chrome census published a `window:` surface, and its body registered a full-bounds
/// `HitKind::ScrollRegion` with an EMPTY control id — the row that swallowed the journey's
/// `zoom-wheel` notch (`📓️w12c-chords-and-camera-live.md` §2.1/§4.3).
#[test]
fn an_emptied_dock_never_enters_the_window_plan() {
    let mut shell = journey_shell(Vec::new());
    assert!(shell.dock.close_window(TOP) && shell.dock.close_window(PERSPECTIVE), "🪟️ both panes close");
    plan(&mut shell);
    assert!(shell.dock_window_plan.is_empty(), "🈳️ an empty dock plans no window: {:?}", shell.dock_window_plan);
    assert!(!shell.window_content_rects.contains_key(""), "🈳️ and no content rect is keyed by an empty window id");
    assert!(shell.window_silhouettes.is_empty(), "🈳️ an empty stack owns no silhouette");
    assert!(!shell.dock_drop_bodies.is_empty(), "📥️ …while the emptied stack stays a drop body, so a dragged window can come back");
}

//#endregion 🪟️CloseAndReopen

//#region 🕒️CapJournals

/// ⚖️ LAW: the Focus chip journals NO shell command. React's chip is `dock.activateWindow(tab.id)`
/// plus `dock.toggleMaximize(stackPath)` (`🎨️Canvas/🟦️.tsx:1076-1086`): the activation is journaled by
/// `handleActiveWindowChange`'s `shell.windowActivate` note and the maximize is pure local state that
/// `🏛️ShellHost/🟦️.tsx` notes nowhere — it declares no `shell.windowMaximize` command at all.
///
/// 🩸️ This renderer noted one, so `window-cap-focus` journaled a `noteShellCommand` React never emits
/// (`🗑️generated/w12c-parity-run-19/steps.json` step 16).
#[test]
fn the_focus_cap_journals_no_shell_command_and_only_its_activation() {
    assert!(ShellState::shell_command_for_control(&format!("dock.tab.0.{TOP}.focus"), false).is_none(), "🕒️ no `shell.windowMaximize` exists to note");
    let mut shell = journey_shell(Vec::new());
    shell.active_window_id = Some(PERSPECTIVE.to_string());
    shell.arm_window_activation_note();
    shell.deferred_actions.clear();

    let outcome = semio_framework_async::block_on(shell.handle_shell_hit(&cap(&format!("dock.tab.0.{TOP}.focus")), &InputState::<ActionDescriptor>::default()));
    assert!(outcome.is_ok(), "🕒️ the Focus chip crosses no dispatch funnel of its own");
    assert!(shell.deferred_actions.is_empty(), "🕒️ …and arms nothing by itself");
    assert_eq!(shell.active_window_id.as_deref(), Some(TOP), "🪟️ it DID activate the tab's window, exactly as React's `activateWindow` does");

    shell.arm_window_activation_note();
    let armed = shell.deferred_actions.first().expect("🪟️ the activation is the one thing the chip journals");
    assert_eq!(armed.action, "noteShellCommand");
    let args = armed.args.as_ref().expect("the note carries its command");
    assert_eq!(args.get("commandId").and_then(DslValue::as_str), Some("shell.windowActivate"));
    assert_eq!(shell.deferred_actions.len(), 1, "🕒️ one activation, one note — never a second `shell.windowMaximize`");
}

/// ⚖️ LAW: a press inside a docked pane activates that pane's window, which is what arms the
/// `shell.windowActivate` note the guest replays on `mod+z` — React's `onPointerDownCapture` →
/// `onActivate` on the window element (`🪟️Window/🟦️.tsx:302`), then `handleActiveWindowChange`.
///
/// 🩸️ Measured live: React's `chord-undo` journals `undo` AND a guest `shell.windowActivate`
/// (origin `guest`, refused `undeclared-action`, `causedBy` the undo) — the guest's
/// `dispatch_chrome_history_action` popping the newest undoable SHELL command and answering with
/// `Effect::ReplayShellCommand` (`🔌️plugin/🦀️.rs:26127`). wgpu journaled `undo` alone because its dock
/// was empty from `window-reopen` on, so no press ever changed the active window and the guest's
/// command log carried no activation to pop. The second half of this law is that empty-dock case.
#[test]
fn a_press_in_a_docked_pane_arms_the_activation_the_undo_chord_replays() {
    let mut shell = journey_shell(Vec::new());
    plan(&mut shell);
    let mut input = InputState::<ActionDescriptor>::default();
    shell.publish_retained_input_for_test(&mut input, &Theme::default());
    shell.arm_window_activation_note();
    shell.deferred_actions.clear();
    let (_, body) = shell.dock_window_plan.iter().find(|(window_id, _)| window_id == PERSPECTIVE).cloned().expect("🪟️ the perspective pane is planned");

    assert!(shell.activate_window_under_pointer(body.x + body.w * 0.7, body.y + body.h * 0.45, &Theme::light()), "🖱️ the probe's own aim point activates the pane it lands in");
    assert_eq!(shell.active_window_id.as_deref(), Some(PERSPECTIVE));
    shell.arm_window_activation_note();
    let armed = shell.deferred_actions.first().expect("🪟️ a real activation arms exactly one note");
    let args = armed.args.as_ref().expect("the note carries its command");
    assert_eq!(args.get("commandId").and_then(DslValue::as_str), Some("shell.windowActivate"));
    assert_eq!(args.get("detail").and_then(|detail| detail.get("windowId")).and_then(DslValue::as_str), Some(PERSPECTIVE));

    let mut empty = journey_shell(Vec::new());
    assert!(empty.dock.close_window(TOP) && empty.dock.close_window(PERSPECTIVE));
    plan(&mut empty);
    empty.publish_retained_input_for_test(&mut input, &Theme::default());
    empty.deferred_actions.clear();
    assert!(!empty.activate_window_under_pointer(CANVAS.w * 0.7, CANVAS.h * 0.45, &Theme::light()), "🈳️ an empty dock has no pane to activate — the run-19 state in which `mod+z` carried no `shell.windowActivate`");
}

//#endregion 🕒️CapJournals

//#region ⎋️EngagementAbort

/// ⚖️ LAW: with the dock intact, Escape under the open example picker dismisses the picker AND
/// dispatches the app's own `engagementAbort` at the window that owns it — React's measured
/// `context-menu-dismiss`, `chord-escape` and `example-picker-dismiss` journals, one `engagementAbort`
/// each (`🗑️generated/w12c-parity-run-19/steps.json` steps 24, 26 and 35).
///
/// 🧾️ W12c could only prove the RUNG ran, because its fixture had no mounted window and the verb died
/// on the unowned-chord banner. With a docked owner the banner is absent and the verb reaches the
/// dispatch funnel instead — which is the half that was blocked on the dock, not on the keyboard.
#[test]
fn escape_with_a_docked_owner_dispatches_the_apps_engagement_abort() {
    let mut shell = journey_shell(vec![Keybinding { keys: "escape".into(), action: ActionDescriptor { controller_id: "test".into(), action: "engagementAbort".into(), args: None } }]);
    shell.overlay_state = OverlayState::Dropdown("example".into());
    let mut input = InputState::<ActionDescriptor>::default();
    let outcome = semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input));
    assert_eq!(shell.overlay_state, OverlayState::None, "⎋️ the same keydown dismisses the picker");
    assert!(shell.transient_notice().is_none(), "⎋️ a docked owner answers the verb — no unowned-chord banner");
    assert_eq!(outcome.err().as_deref(), Some("action program missing"), "⎋️ …and `engagementAbort` crossed the dispatch funnel");
}

/// ⚖️ LAW: and the converse, which is the run-19 state — with the dock emptied by the cap steps the
/// SAME chord raises the unowned-chord banner and dispatches nothing. This is why `chord-escape`,
/// `context-menu-dismiss` and `example-picker-dismiss` were silent on wgpu while React journaled
/// `engagementAbort` at all three: the keyboard ladder resolved the binding at every one of them.
#[test]
fn escape_with_an_empty_dock_is_the_hinted_no_op_run_19_measured() {
    let mut shell = journey_shell(vec![Keybinding { keys: "escape".into(), action: ActionDescriptor { controller_id: "test".into(), action: "engagementAbort".into(), args: None } }]);
    assert!(shell.dock.close_window(TOP) && shell.dock.close_window(PERSPECTIVE));
    let mut input = InputState::<ActionDescriptor>::default();
    let outcome = semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input));
    assert!(outcome.is_ok(), "⎋️ nothing is dispatched at all");
    assert_eq!(shell.transient_notice().map(|notice| notice.code.clone()), Some(Some(KEYBINDING_UNOWNED_CODE.to_string())), "⎋️ the verb resolved and found no window to address");
}

//#endregion ⎋️EngagementAbort

//#region 📜️JournalSequences

/// 📜️ The shell's own journal-sequence fixture — the dock/chord twin of
/// `🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json`, whose `react` column is the measured run-19
/// journal and whose `wgpu` column is this shell's contract.
const JOURNAL_SEQUENCES: &str = include_str!("../../🧫️fixtures/📜️journal-sequences/🔣️.json");

/// ⚖️ LAW: every cap row's `commands` list IS what [`ShellState::shell_command_for_control`] answers
/// for that row's control — the fixture and the mapping table cannot drift apart.
#[test]
fn the_cap_rows_of_the_journal_fixture_match_the_shell_command_table() {
    let fixture: serde_json::Value = serde_json::from_str(JOURNAL_SEQUENCES).expect("📜️ the journal-sequence fixture parses");
    let steps = fixture["steps"].as_array().expect("📜️ the fixture carries its steps");
    let mut checked = 0;
    for step in steps {
        let Some(control) = step["control"].as_str() else { continue };
        let expected: Vec<&str> = step["commands"].as_array().expect("📜️ a control row declares its commands").iter().map(|value| value.as_str().expect("a command id")).collect();
        let actual: Vec<&str> = ShellState::shell_command_for_control(control, false).map(|(command_id, _)| command_id).into_iter().collect();
        assert_eq!(actual, expected, "📜️ {}: `{control}` journals {expected:?}", step["id"].as_str().unwrap_or("?"));
        checked += 1;
    }
    assert_eq!(checked, 3, "📜️ all three cap rows carry a control and were checked");
}

/// ⚖️ LAW: a row whose two columns differ carries a `why`, and a row that matches does not need one —
/// the discipline `📜️journal-sequences.json` already holds on the world surface. A divergence without
/// a written reason is how a defect becomes a contract.
#[test]
fn every_diverging_journal_row_states_why() {
    let fixture: serde_json::Value = serde_json::from_str(JOURNAL_SEQUENCES).expect("📜️ the journal-sequence fixture parses");
    for step in fixture["steps"].as_array().expect("📜️ the fixture carries its steps") {
        let id = step["id"].as_str().expect("📜️ every row is named");
        let react = step["react"].as_array().expect("📜️ every row carries React's measured journal");
        let wgpu = step["wgpu"].as_array().expect("📜️ every row carries this shell's contract");
        let why = step["why"].as_str().expect("📜️ every row carries a `why` field, empty when the columns agree");
        if react != wgpu {
            assert!(!why.is_empty(), "📜️ {id}: the two columns differ and no reason is written — that is how a defect becomes a contract");
        }
        assert!(!react.is_empty() || !wgpu.is_empty() || !why.is_empty(), "📜️ {id}: a row where both renderers journal nothing still says so on purpose");
    }
}

//#endregion 📜️JournalSequences
