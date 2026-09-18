//! ⌨️🔍️ Laws for the wgpu shell's shortcut table and command palette against the React shell
//! (packet W1c of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
//!
//! Two contracts are pinned here:
//! - `SHELL_SHORTCUT_ROWS` is a verbatim transcription of React's `SHELL_KEYBINDINGS`
//!   (`🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`), every row resolves to a verb that
//!   actually runs, and every row's chord is reserved against app keybindings — a row can never be
//!   reserved-but-dead or dispatched-but-shadowable.
//! - the palette lists and EXECUTES every in-palette command of every owner, and ranks its rows with the
//!   very algorithm React's `UISearch` uses (`🔨️modules/🖱️ui/🔨️modules/🔎️fuzzy-ranking/🟦️.ts`).

use super::command_registry_tests::test_app;
use super::*;
use semio_framework::{ActionArgDef, ActionArgOption, ActionKind, CommandDefinition, CommandOwnerAddress};

//#region ⌨️ChordTable

/// ⌨️ Parses one chord of the shared table into the key event this target would receive.
fn chord_event(chord: &str) -> (ui_wgpu::wgpu::KeyAction, PointerModifiers) {
    let mut modifiers = PointerModifiers::default();
    let mut key = "";
    for token in chord.split('+') {
        match token {
            "mod" | "meta" | "cmd" => modifiers.meta = true,
            "ctrl" => modifiers.ctrl = true,
            "shift" => modifiers.shift = true,
            "alt" | "option" => modifiers.alt = true,
            other => key = other,
        }
    }
    let action = match key {
        "enter" => ui_wgpu::wgpu::KeyAction::Enter,
        "escape" => ui_wgpu::wgpu::KeyAction::Escape,
        "tab" => ui_wgpu::wgpu::KeyAction::Tab,
        "up" | "arrowup" => ui_wgpu::wgpu::KeyAction::ArrowUp,
        "down" | "arrowdown" => ui_wgpu::wgpu::KeyAction::ArrowDown,
        "left" | "arrowleft" => ui_wgpu::wgpu::KeyAction::ArrowLeft,
        "right" | "arrowright" => ui_wgpu::wgpu::KeyAction::ArrowRight,
        other => ui_wgpu::wgpu::KeyAction::Char(other.to_string()),
    };
    (action, modifiers)
}

/// ⚖️ LAW: the wgpu table IS React's `SHELL_KEYBINDINGS` — same control ids, same chords, same count.
/// Transcribed rather than imported because the two renderers cannot share a literal; this law is what
/// keeps the transcription honest, and it is the row that catches a drifting chord (`os.toggleFullscreen`
/// was `f11`/`ctrl+meta+f`-only here while React has always ALSO bound `mod+shift+f`).
#[test]
fn the_shortcut_table_transcribes_every_react_shell_keybinding_row() {
    let expected: Vec<(&str, &str)> = vec![
        ("ui.search.toggle", "mod+p"),
        ("ui.find.toggle", "mod+f"),
        ("os.toggleFullscreen", "mod+shift+f"),
        ("ui.nav.back", "mod+["),
        ("ui.nav.forward", "mod+]"),
        ("ui.nav.up", "mod+up"),
        ("ui.shell.panelAnchor.topLeft", "ctrl+b,meta+b"),
        ("ui.shell.panelAnchor.topMiddle", "ctrl+m,meta+m"),
        ("ui.shell.panelAnchor.topRight", "ctrl+shift+b,meta+shift+b"),
        ("ui.shell.panelAnchor.rightMiddle", "ctrl+shift+m,meta+shift+m"),
        ("ui.shell.panelAnchor.bottomRight", "ctrl+alt+shift+b,meta+alt+shift+b"),
        ("ui.shell.panelAnchor.bottomMiddle", "ctrl+alt+m,meta+alt+m"),
        ("ui.shell.panelAnchor.bottomLeft", "ctrl+alt+b,meta+alt+b"),
        ("ui.shell.panelAnchor.leftMiddle", "ctrl+alt+shift+m,meta+alt+shift+m"),
        ("ui.window.close", "mod+shift+w"),
        ("ui.window.focus", "mod+shift+enter"),
        ("ui.window.newWindow", "mod+shift+n"),
        ("ui.shell.mode.next", "mod+alt+arrowright"),
        ("ui.shell.mode.previous", "mod+alt+arrowleft"),
        ("playground.navbar.roles.editor", "mod+alt+e"),
        ("playground.navbar.roles.viewer", "mod+alt+v"),
    ];
    assert_eq!(SHELL_SHORTCUT_ROWS.to_vec(), expected, "every accelerator row of React's SHELL_KEYBINDINGS, verbatim");
    eprintln!("[DEBUG] wgpu shell shortcut table: {} rows transcribed from React's SHELL_KEYBINDINGS", SHELL_SHORTCUT_ROWS.len());
}

/// ⚖️ LAW: every row resolves to a verb, every chord of every row resolves to that same verb, and every
/// chord is reserved against app-declared keybindings. No row may be listed without being dispatchable.
#[test]
fn every_row_dispatches_its_verb_and_outranks_app_keybindings() {
    let mut chords = 0;
    for (control_id, keys) in SHELL_SHORTCUT_ROWS {
        let expected = shell_shortcut_for_control_id(control_id).unwrap_or_else(|| panic!("{control_id} names no shell verb"));
        for chord in keys.split(',') {
            let (action, modifiers) = chord_event(chord);
            assert_eq!(shell_shortcut_for(&action, &modifiers), Some(expected), "{control_id} / {chord}");
            assert!(is_reserved_shell_chord(&action, &modifiers), "{control_id} / {chord}: a shell chord is never an app's to shadow");
            chords += 1;
        }
    }
    assert_eq!(chords, 29, "21 rows, eight of which spell both the ctrl and the meta accelerator");
    eprintln!("[DEBUG] wgpu shell shortcut table: {chords} chords dispatch and reserve their verb");
}

/// ⚖️ LAW: the two verbs with hand-written chord helpers (the surface-role and mode-cycle axes, kept as
/// named helpers because the chrome-parity fixture asserts them directly) agree with the table.
#[test]
fn the_role_and_mode_helpers_agree_with_the_table() {
    for (chord, expected) in [
        ("mod+alt+e", ShellShortcut::SurfaceRole(semio_framework::AppRole::Editor)),
        ("mod+alt+v", ShellShortcut::SurfaceRole(semio_framework::AppRole::Viewer)),
        ("mod+alt+arrowright", ShellShortcut::ModeStep(1)),
        ("mod+alt+arrowleft", ShellShortcut::ModeStep(-1)),
    ] {
        let (action, modifiers) = chord_event(chord);
        assert_eq!(shell_shortcut_for(&action, &modifiers), Some(expected), "{chord}");
    }
}

/// ⚖️ LAW: undo/redo are the framework's UNIVERSAL chords, never reserved ones — React dispatches them
/// after the app-keybinding loop precisely so an app that declares `mod+z` itself keeps it.
#[test]
fn the_edit_chords_are_shadowable_and_carry_the_react_redo_alias() {
    for (chord, expected) in [("mod+z", ShellEditVerb::Undo), ("mod+shift+z", ShellEditVerb::Redo), ("mod+y", ShellEditVerb::Redo)] {
        let (action, modifiers) = chord_event(chord);
        assert_eq!(shell_edit_verb_for(&action, &modifiers), Some(expected), "{chord}");
        assert!(!is_reserved_shell_chord(&action, &modifiers), "{chord}: an app keybinding may shadow the universal edit chords");
        assert_eq!(shell_shortcut_for(&action, &modifiers), None, "{chord} is not a shell-chrome row");
    }
    let (action, modifiers) = chord_event("mod+alt+z");
    assert_eq!(shell_edit_verb_for(&action, &modifiers), None, "mod+alt+z is nobody's undo — React's branch refuses the alt axis");
    assert_eq!(ShellEditVerb::Undo.action_id(), "undo");
    assert_eq!(ShellEditVerb::Redo.action_id(), "redo");
}

/// ⚖️ LAW: a chord only fires when the user is NOT typing, and a bare key is never a shell chord.
#[test]
fn a_focused_content_field_swallows_every_shell_chord() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    input.focused_id = Some("app.content.field".into());
    let (action, modifiers) = chord_event("meta+b");
    shell.handle_keyboard(action, &modifiers, &mut input);
    assert!(!shell.anchor_open(PanelAnchor::TopLeft), "a chord never fires while a content field has focus");
    assert_eq!(shell_shortcut_for(&ui_wgpu::wgpu::KeyAction::Char("b".into()), &PointerModifiers::default()), None, "a bare `b` is ordinary text");
}

//#endregion ⌨️ChordTable

//#region 🧭️PanelAndWindowChords

/// ⚖️ LAW: all EIGHT panel anchors have a chord, and each toggles ITS OWN anchor. Before this wave only
/// two of the eight rows existed (`mod+b`/`mod+shift+b`) and they flipped a whole side's visibility flag
/// rather than an anchor, so no chord could ever reach the Display, Settings or Command anchors.
/// An anchor carrying no tab is a witnessed no-op rather than an empty panel over the canvas.
#[test]
fn each_panel_anchor_chord_toggles_its_own_anchor() {
    let mut shell = palette_shell();
    shell.sync_dock_tabs();
    let mut input = InputState::<ActionDescriptor>::default();
    let press = |shell: &mut ShellState, chord: &str, input: &mut InputState<ActionDescriptor>| {
        let (action, modifiers) = chord_event(chord);
        shell.handle_keyboard(action, &modifiers, input);
    };

    press(&mut shell, "ctrl+b", &mut input);
    assert!(shell.anchor_open(PanelAnchor::TopLeft), "ctrl+b opens the top-left (Workbench) anchor");
    press(&mut shell, "ctrl+b", &mut input);
    assert!(!shell.anchor_open(PanelAnchor::TopLeft), "and the same chord folds it shut again");

    press(&mut shell, "meta+shift+b", &mut input);
    assert!(shell.anchor_open(PanelAnchor::TopRight), "mod+shift+b opens the top-right anchor");
    assert!(!shell.anchor_open(PanelAnchor::TopLeft), "without disturbing another anchor");

    // 🎛️ The bottom-middle Command anchor is React's own home for the command panel, and `ctrl+alt+m` is
    // React's own chord for it — neither existed on this renderer before this wave.
    press(&mut shell, "ctrl+alt+m", &mut input);
    assert!(shell.anchor_open(PanelAnchor::BottomMiddle), "ctrl+alt+m opens the bottom-middle Command anchor");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomMiddle).path.first().map(String::as_str), Some("framework.category.command"));

    // 🖥️ Bottom-left always carries the framework Display branch (React's `frameworkDisplayTabs` are
    // framework chrome, not an app contribution), so its chord always has something to open — even for
    // a fixture that declares no Display-group tab of its own.
    press(&mut shell, "ctrl+alt+b", &mut input);
    assert!(shell.anchor_open(PanelAnchor::BottomLeft), "ctrl+alt+b opens the bottom-left Display anchor");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomLeft).path.first().map(String::as_str), Some("framework.category.display"));

    // 🧭️ An anchor NO source assigns a tab to stays shut: left-middle carries none in either renderer.
    let before = shell.open_anchors();
    press(&mut shell, "ctrl+alt+shift+m", &mut input);
    assert_eq!(shell.open_anchors(), before, "a chord for an empty anchor opens nothing");
    eprintln!("[DEBUG] wgpu panel anchors: eight chords, each toggling its own anchor; empty anchors are witnessed no-ops");
}

/// ⚖️ LAW: the palette/find toggles still own their chords through the table, including the second press
/// that closes them (the regression `🔬️wgpu-shell-input` pins for `mod+p` — re-asserted here for `mod+f`).
#[test]
fn the_overlay_chords_toggle_through_the_same_table() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = chord_event("mod+f");
    shell.handle_keyboard(action.clone(), &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::Find);
    assert_eq!(input.focused_id.as_deref(), Some("shell.find.input"));
    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::None, "the same chord closes it");
    assert_eq!(input.focused_id, None);
}

fn two_window_shell() -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let app = test_app(Vec::new(), Vec::new());
    shell.dock.root = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("main"), DockStackTab::new("side")], active: "main".into() };
    shell.dock.active_window_id = Some("main".into());
    shell.active_window_id = Some("main".into());
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::default() });
    shell
}

/// ⚖️ LAW: the three window chords React binds — close, focus (maximize) and open-in-new-window — do on
/// wgpu exactly what the window's own chrome controls do. None of the three existed here before.
#[test]
fn the_window_chords_close_maximize_and_open_a_new_instance() {
    let mut input = InputState::<ActionDescriptor>::default();
    let press = |shell: &mut ShellState, chord: &str, input: &mut InputState<ActionDescriptor>| {
        let (action, modifiers) = chord_event(chord);
        shell.handle_keyboard(action, &modifiers, input);
    };

    let mut shell = two_window_shell();
    press(&mut shell, "mod+shift+enter", &mut input);
    assert_eq!(shell.dock.maximized_stack, Some(Vec::new()), "mod+shift+enter maximizes the active window's stack (the root stack, path `[]`)");
    press(&mut shell, "mod+shift+enter", &mut input);
    assert_eq!(shell.dock.maximized_stack, None, "and the same chord restores it");

    let mut shell = two_window_shell();
    press(&mut shell, "mod+shift+n", &mut input);
    let instances = shell.dock.window_instances();
    assert!(instances.iter().any(|(id, kind)| id == "main-2" && kind == "main"), "a second INSTANCE of the same kind, not a tab of an undeclared kind: {instances:?}");
    assert_eq!(shell.active_window_id.as_deref(), Some("main-2"), "the new instance takes focus, exactly as React's SET_ACTIVE_WINDOW_ID does");

    let mut shell = two_window_shell();
    press(&mut shell, "mod+shift+w", &mut input);
    assert_eq!(shell.dock.window_instances().into_iter().map(|(id, _)| id).collect::<Vec<_>>(), vec!["side".to_string()], "mod+shift+w closes the active window");
    press(&mut shell, "mod+shift+w", &mut input);
    assert!(shell.dock.window_instances().is_empty(), "and keeps closing: React's `closeWindow` has no last-window guard, so an empty mode is reachable by chord too");
    eprintln!("[DEBUG] wgpu window chords: close / maximize / new-instance all drive the dock");
}

//#endregion 🧭️PanelAndWindowChords

//#region 🔎️FuzzyRanking

fn ranked(items: &[&'static str], query: &str) -> Vec<&'static str> {
    rank_fuzzy_items(items.to_vec(), query, &[(|item: &&str| Some(*item), 1.0)], FUZZY_SEARCH_THRESHOLD, FUZZY_SEARCH_LIMIT)
}

/// ⚖️ LAW: the Rust ranker answers the TypeScript branches — exact, prefix, substring, gapped
/// subsequence and a bounded typo — with the same shape of score, LOWER being better.
#[test]
fn the_ranker_answers_the_same_branches_as_the_typescript_twin() {
    let threshold = FUZZY_SEARCH_THRESHOLD;
    assert_eq!(fuzzy_token_score("set locale", "set locale", threshold), Some(0.0), "an exact match scores zero");
    let prefix = fuzzy_token_score("set", "set locale", threshold).expect("prefix matches");
    let substring = fuzzy_token_score("locale", "set locale", threshold).expect("substring matches");
    let subsequence = fuzzy_token_score("stlc", "set locale", threshold).expect("the gapped subsequence still matches");
    assert!(prefix < substring, "a prefix beats a substring: {prefix} vs {substring}");
    assert!(substring < subsequence, "a substring beats a gapped subsequence: {substring} vs {subsequence}");
    assert!(fuzzy_token_score("locl", "set locale", threshold).is_some(), "a one-edit typo inside the threshold still matches");
    assert_eq!(fuzzy_token_score("zzzz", "set locale", threshold), None, "an unrelated query matches nothing");
}

/// ⚖️ LAW: an empty query keeps declaration order (React's `items.slice(0, limit)`), a no-match query
/// returns NOTHING — the former hand-rolled scorer had no threshold and so returned the whole palette —
/// and ties break on declaration order.
#[test]
fn the_palette_ranking_bounds_and_orders_the_way_react_does() {
    let items = ["Set Locale", "Set Appearance", "Reset Dock Layout"];
    assert_eq!(ranked(&items, ""), items.to_vec(), "a cold palette is the declared list");
    assert_eq!(ranked(&items, "   "), items.to_vec(), "whitespace is an empty query");
    assert!(ranked(&items, "zzzz").is_empty(), "a query matching nothing shows nothing");
    assert_eq!(ranked(&items, "set").first().copied(), Some("Set Locale"), "prefix matches rank first, ties on declaration order");
    assert_eq!(ranked(&items, "stlc").first().copied(), Some("Set Locale"), "a gapped subsequence still finds its command");
    assert_eq!(ranked(&items, "reset dock").first().copied(), Some("Reset Dock Layout"), "every token must match, and both do here");
    let many: Vec<String> = (0..40).map(|index| format!("Command {index}")).collect();
    assert_eq!(rank_fuzzy_items(many, "command", &[(|item: &String| Some(item.as_str()), 2.0)], FUZZY_SEARCH_THRESHOLD, FUZZY_SEARCH_LIMIT).len(), FUZZY_SEARCH_LIMIT, "React's limit of 20 is this target's limit too");
}

/// ⚖️ LAW: the weighted fields are React's — a hit in the label outranks the same hit in the group
/// heading, which is what makes typing a command's name beat typing its category.
#[test]
fn a_label_hit_outranks_the_same_hit_in_a_lower_weighted_field() {
    #[derive(Clone)]
    struct Row {
        label: &'static str,
        group: &'static str,
    }
    let rows = vec![Row { label: "Reset Dock Layout", group: "Layout" }, Row { label: "Set Appearance", group: "Appearance" }];
    let ranked = rank_fuzzy_items(rows, "appearance", &[(|row: &Row| Some(row.label), 2.0), (|row: &Row| Some(row.group), 0.5)], FUZZY_SEARCH_THRESHOLD, FUZZY_SEARCH_LIMIT);
    assert_eq!(ranked.first().map(|row| row.label), Some("Set Appearance"), "the label hit wins over the group hit");
}

//#endregion 🔎️FuzzyRanking

//#region 🎛️CommandPalette

fn select_arg(id: &str) -> ActionArgDef {
    ActionArgDef::select(id, LocalizedLabel::data("Choice"), vec![ActionArgOption { value: "one".into(), label: LocalizedLabel::data("One") }, ActionArgOption { value: "two".into(), label: LocalizedLabel::data("Two") }]).required()
}

fn palette_shell() -> ShellState {
    let commands = vec![
        CommandDefinition::new("app.fire", LocalizedLabel::data("Fire"), "app", "play", ActionKind::Shell),
        CommandDefinition::new("app.pick", LocalizedLabel::data("Pick"), "app", "play", ActionKind::Shell).with_args([select_arg("value")]),
        CommandDefinition::new("app.compose", LocalizedLabel::data("Compose"), "app", "play", ActionKind::Shell).with_args([select_arg("value"), ActionArgDef::text("note", LocalizedLabel::data("Note"))]),
        CommandDefinition::new("app.rename", LocalizedLabel::data("Rename"), "app", "play", ActionKind::Shell).with_args([ActionArgDef::text("name", LocalizedLabel::data("Name"))]),
    ];
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app: test_app(commands, Vec::new()), view_state: ViewModel::default() });
    shell
}

/// ⚖️ LAW: every in-palette command of every owner is REACHABLE. Arg-carrying App/Plugin/Mode-scope
/// commands used to be dropped on the (stale) premise that the plugin bridge had no `handle_command`
/// RPC — it has one on both backends, and `dispatch_command` has always driven it.
#[test]
fn the_palette_lists_and_executes_every_in_palette_command() {
    let shell = palette_shell();
    let items = shell.command_search_items();
    let by_id = |id: &str| items.iter().find(|item| item.id == id).unwrap_or_else(|| panic!("{id} is missing from the palette: {:?}", items.iter().map(|item| item.id.as_str()).collect::<Vec<_>>()));

    let fire = by_id("command.app:test:test-app:app.fire");
    assert_eq!(fire.label, "Fire", "a zero-arg command carries its plain label");
    assert!(fire.action.as_deref().is_some_and(|action| action.starts_with("command:")), "and fires across the command boundary: {:?}", fire.action);

    for option in ["one", "two"] {
        let row = by_id(&format!("command.app:test:test-app:app.pick.{option}"));
        assert_eq!(row.label, if option == "one" { "Pick: One" } else { "Pick: Two" }, "a single-select command expands one concrete row per option");
        let invocation: semio_framework::manifest::CommandInvocation = dsl::os_pack::json::from_json_str(row.action.as_deref().expect("row action").strip_prefix("command:").expect("command invocation")).expect("invocation parses");
        assert_eq!(invocation.arguments.len(), 1, "with exactly the one argument bound");
    }

    for (id, label) in [("command.app:test:test-app:app.compose", "Compose…"), ("command.app:test:test-app:app.rename", "Rename…")] {
        let row = by_id(id);
        assert_eq!(row.label, label, "a command whose args cannot be expanded carries React's `…` suffix");
        assert!(row.action.as_deref().is_some_and(|action| action.starts_with("command-form:")), "and redirects to its staged form rather than firing a guess: {:?}", row.action);
    }
    eprintln!("[DEBUG] wgpu palette: {} rows, every declared command reachable", items.len());
}

/// ⚖️ LAW: os commands still expand per option and still route to the LOCAL os funnel (never across the
/// command boundary, which refuses an `Os` owner outright).
#[test]
fn os_commands_keep_their_local_per_option_rows() {
    let shell = palette_shell();
    let items = shell.command_search_items();
    let appearance: Vec<&SearchPaletteItem> = items.iter().filter(|item| item.id.starts_with("command.os:os.setAppearance")).collect();
    assert_eq!(appearance.len(), 3, "system / light / dark");
    assert!(appearance.iter().all(|item| item.action.as_deref().is_some_and(|action| action.starts_with("os-command:os.setAppearance:"))));
    let fullscreen = items.iter().find(|item| item.id == "command.os:os.toggleFullscreen").expect("the fullscreen command is in the palette");
    assert!(fullscreen.description.is_some(), "React's palette row shows the command's resolved chords as its description");
    assert!(matches!(fullscreen.category, Some(CommandOwnerAddress::Os)));
}

/// ⚖️ LAW: picking an arg-carrying command opens the Commands surface expanded on that command — the
/// wgpu spelling of React's `SET_PANEL_VISIBLE` + `SET_PANEL_PATH` + `SET_COMMAND_EXPANDED` trio.
#[test]
fn picking_an_arg_carrying_command_opens_its_form() {
    let mut shell = palette_shell();
    shell.sync_dock_tabs();
    shell.overlay_state = OverlayState::Search;
    shell.search_query = "Compose".into();
    let index = shell.filtered_search_items().iter().position(|item| item.id == "command.app:test:test-app:app.compose").expect("the query finds the command");
    semio_framework_async::block_on(shell.activate_search_item(index)).expect("activating a form redirect never faults");
    assert!(shell.anchor_open(PanelAnchor::BottomMiddle), "the bottom-middle Command anchor — React's own home for the command panel");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomMiddle).path.first().map(String::as_str), Some("framework.category.command"));
    assert_eq!(shell.expanded_command_id.as_deref(), Some("app:test:test-app:app.compose"));
    assert_eq!(shell.overlay_state, OverlayState::None, "and the palette closes behind it");
}

/// ⚖️ LAW: the palette's own rows come in React's declaration order — panels, windows, commands, then
/// the host-app verbs — because that order is what the ranker breaks ties on and what a cold palette shows.
#[test]
fn the_palette_rows_follow_the_react_declaration_order() {
    let shell = palette_shell();
    shell.build_search_items().iter().fold(0usize, |rank, item| {
        let bucket = match item.id.split('.').next() {
            Some("panel") => 0,
            Some("window") => 1,
            Some("command") => 2,
            Some("keybinding") => 3,
            Some("studio") => 4,
            other => panic!("unknown palette row family {other:?}"),
        };
        assert!(bucket >= rank, "palette row {} is out of React's declaration order", item.id);
        bucket
    });
}

//#endregion 🎛️CommandPalette
