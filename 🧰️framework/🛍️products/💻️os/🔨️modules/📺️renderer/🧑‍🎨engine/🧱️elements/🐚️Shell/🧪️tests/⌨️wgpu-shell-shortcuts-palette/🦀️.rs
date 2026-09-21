//! ⌨️🔍️ Laws for the wgpu shell's shortcut table and command palette against the React shell
//! (packet W1c of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
//!
//! Two contracts are pinned here:
//! - `SHELL_SHORTCUT_ROWS` is the executable chrome subset of React's `SHELL_KEYBINDINGS`
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

/// ⚖️ LAW: the wgpu table is React's executable chrome-control subset. OS commands are excluded:
/// their platform-scoped `CommandDefinition.keybindings` are matched by the command route itself.
#[test]
fn the_shortcut_table_transcribes_every_react_shell_keybinding_row() {
    let expected: Vec<(&str, &str)> = vec![
        ("ui.search.toggle", "mod+p"),
        ("ui.find.toggle", "mod+f"),
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
    assert_eq!(SHELL_SHORTCUT_ROWS.to_vec(), expected, "every executable chrome accelerator row");
    assert!(!SHELL_SHORTCUT_ROWS.iter().any(|(id, _)| *id == "os.toggleFullscreen"));
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
    assert_eq!(chords, 28, "20 rows, eight of which spell both the ctrl and the meta accelerator");
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
///
/// 🎛️ The bottom-middle Command anchor is React's own home for the command panel, and `ctrl+alt+m` is
/// React's own chord for it — neither existed on this renderer before this wave.
///
/// 🖥️ Bottom-left always carries the framework Display branch (React's `frameworkDisplayTabs` are
/// framework chrome, not an app contribution), so its chord always has something to open — even for
/// a fixture that declares no Display-group tab of its own.
///
/// 🧭️ An anchor NO source assigns a tab to stays shut: left-middle carries none in either renderer.
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

    press(&mut shell, "ctrl+alt+m", &mut input);
    assert!(shell.anchor_open(PanelAnchor::BottomMiddle), "ctrl+alt+m opens the bottom-middle Command anchor");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomMiddle).path.first().map(String::as_str), Some("framework.category.command"));

    press(&mut shell, "ctrl+alt+b", &mut input);
    assert!(shell.anchor_open(PanelAnchor::BottomLeft), "ctrl+alt+b opens the bottom-left Display anchor");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomLeft).path.first().map(String::as_str), Some("framework.category.display"));

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
    assert_eq!(input.focused_id.as_deref(), Some("ui.find.input"));
    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::None, "the same chord closes it");
    assert_eq!(input.focused_id, None);
}

#[test]
fn dismissing_and_reopening_a_palette_preserves_its_query() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = chord_event("mod+p");
    shell.handle_keyboard(action.clone(), &modifiers, &mut input);
    shell.set_palette_query(ShellPaletteKind::Search, "Rück".into(), &mut input);
    shell.handle_keyboard(action.clone(), &modifiers, &mut input);
    assert_eq!(shell.search_query, "Rück", "dismissal preserves the editable query");
    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.search_query, "Rück", "reopening seeds the preserved query");
    assert_eq!(input.focused_id.as_deref(), Some("ui.search.input"));
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
    let first_close = shell.deferred_actions.last().expect("the shortcut journals its exact closed window");
    assert_eq!(first_close.action, "noteShellCommand");
    assert_eq!(first_close.args.as_ref().and_then(|args| args.get("commandId")).and_then(DslValue::as_str), Some("shell.windowClose"));
    assert_eq!(first_close.args.as_ref().and_then(|args| args.get("detail")).and_then(|detail| detail.get("windowId")).and_then(DslValue::as_str), Some("main"));
    press(&mut shell, "mod+shift+w", &mut input);
    assert!(shell.dock.window_instances().is_empty(), "and keeps closing: React's `closeWindow` has no last-window guard, so an empty mode is reachable by chord too");
    assert_eq!(shell.deferred_actions.len(), 2, "each successful shortcut close queues one journal note");
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

#[test]
fn fuzzy_normalization_matches_the_neutral_nfkd_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔎️ShellSearch/🧫️fixtures/🔣️.json")).expect("neutral palette fixture");
    for row in fixture["normalization"]["cases"].as_array().expect("normalization cases") {
        let value = row["value"].as_str().expect("normalization value");
        let expected = row["expected"].as_str().expect("normalization expected");
        assert_eq!(normalize_fuzzy_text(value), expected, "{value:?}");
    }
    assert_eq!(normalize_fuzzy_text("Rückgängig"), normalize_fuzzy_text("Ru\u{308}ckga\u{308}ngig"));
}

#[test]
fn built_in_command_labels_match_the_neutral_react_locale_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔎️ShellSearch/🧫️fixtures/🔣️.json")).expect("neutral palette fixture");
    let mut shell = ShellState::new(Vec::new(), String::new());
    for row in fixture["producer"]["localizedCommands"].as_array().expect("localized command rows") {
        let command_id = row["id"].as_str().expect("localized command id");
        for locale in ["en", "de"] {
            shell.locale_id = locale.to_string();
            let actual = shell.build_os_commands().into_iter().find(|command| command.id == command_id).expect("native registry contains the canonical React command").label.resolve(shell.active_terminology(), shell.active_locale()).to_string();
            assert_eq!(actual, row["labels"][locale].as_str().expect("localized command label"), "{command_id}:{locale}");
        }
    }
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

fn accept_find_fixture_action(_instance_id: u32, _action_json: &str, _view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
    Ok(semio_framework::kernel::InvocationResult {
        output: DslValue::Null,
        mutations: Vec::new(),
        inverse_group: semio_framework::kernel::UndoGroup {
            invocation_id: semio_framework::kernel::InvocationId(String::new()),
            mutations: Vec::new(),
            inverse_mutations: Vec::new(),
            member_edits: Vec::new(),
        },
        diagnostics: Vec::new(),
        requested_effects: Vec::new(),
        events: Vec::new(),
        ui_scope: semio_framework::kernel::UiDirtyScope::default(),
        history_patch: None,
    })
}

/// ⚖️ LAW: every in-palette command of every owner is REACHABLE. Arg-carrying App/Plugin/Mode-scope
/// commands used to be dropped on the (stale) premise that the plugin bridge had no `handle_command`
/// RPC — it has one on both backends, and `dispatch_command` has always driven it.
#[test]
fn the_palette_lists_and_executes_every_in_palette_command() {
    let shell = palette_shell();
    let items = shell.command_search_items();
    let by_id = |id: &str| items.iter().find(|item| item.id == id).unwrap_or_else(|| panic!("{id} is missing from the palette: {:?}", items.iter().map(|item| item.id.as_str()).collect::<Vec<_>>()));

    let fire = by_id("command.app.test.test-app.app.fire");
    assert_eq!(fire.label, "Fire", "a zero-arg command carries its plain label");
    assert!(fire.action.as_deref().is_some_and(|action| action.starts_with("command:")), "and fires across the command boundary: {:?}", fire.action);

    for (id, label) in [("command.app.test.test-app.app.pick", "Pick…"), ("command.app.test.test-app.app.compose", "Compose…"), ("command.app.test.test-app.app.rename", "Rename…")] {
        let row = by_id(id);
        assert_eq!(row.label, label, "a command whose args cannot be expanded carries React's `…` suffix");
        assert!(row.action.as_deref().is_some_and(|action| action.starts_with("command-form:")), "and redirects to its staged form rather than firing a guess: {:?}", row.action);
    }
    eprintln!("[DEBUG] wgpu palette: {} rows, every declared command reachable", items.len());
}

/// ⚖️ LAW: an arg-carrying os command is the same single staged row React publishes; only a
/// zero-arg os command routes to the local os funnel.
#[test]
fn os_arg_commands_are_single_staged_rows() {
    let shell = palette_shell();
    let items = shell.command_search_items();
    let appearance: Vec<&SearchPaletteItem> = items.iter().filter(|item| item.id.starts_with("command.os.os.setAppearance")).collect();
    assert_eq!(appearance.len(), 1);
    assert_eq!(appearance[0].label, "Set Appearance…");
    assert_eq!(appearance[0].action.as_deref(), Some("command-form:os:os.setAppearance"));
    let fullscreen = items.iter().find(|item| item.id == "command.os.os.toggleFullscreen").expect("the fullscreen command is in the palette");
    assert!(fullscreen.description.is_some(), "React's palette row shows the command's resolved chords as its description");
    assert!(matches!(fullscreen.category, Some(CommandOwnerAddress::Os)));
}

/// ⚖️ LAW: picking an arg-carrying command opens the Commands surface expanded on that command — the
/// wgpu spelling of React's `SET_PANEL_VISIBLE` + `SET_PANEL_PATH` + `SET_COMMAND_EXPANDED` trio.
#[test]
fn picking_an_arg_carrying_command_opens_its_form() {
    let mut shell = palette_shell();
    shell.sync_dock_tabs();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔎️ShellSearch/🧫️fixtures/🔣️.json")).expect("neutral palette fixture");
    let staged = &fixture["producer"]["stagedCommand"];
    let surface = staged["categorySurfaceId"].as_str().expect("category surface id");
    let initial = shell.publish_shell_panel_document(surface).expect("initial Command category publication").expect("Command category owns a retained document");
    let before = initial.header().expect("published Command category header");
    shell.panel_documents.insert(surface.to_string(), initial);
    shell.overlay_state = OverlayState::Search;
    shell.search_query = "Set Theme".into();
    let index = shell.filtered_search_items().iter().position(|item| item.id == staged["id"].as_str().expect("staged command id")).expect("the query finds the command");
    let vacancy = shell.closing_documents.first_vacant_index().expect("retirement vacancy");
    shell.closing_documents.epochs[vacancy] = u64::MAX;
    let error = semio_framework_async::block_on(shell.activate_search_item(index)).expect_err("retirement refusal rejects the staged replacement");
    assert!(error.contains("retirement registry refused"));
    assert_eq!(shell.panel_documents.get(surface).unwrap().header().unwrap(), before, "refusal keeps the exact readable category owner");
    assert_eq!(shell.expanded_command_id, None, "refusal restores the exact prior expansion");
    assert_eq!(shell.overlay_state, OverlayState::Search, "refusal keeps the palette available for an exact retry");
    assert_eq!(shell.search_query, "Set Theme");

    shell.closing_documents.epochs[vacancy] = 0;
    semio_framework_async::block_on(shell.activate_search_item(index)).expect("activating a form redirect never faults");
    assert!(shell.anchor_open(PanelAnchor::BottomMiddle), "the bottom-middle Command anchor — React's own home for the command panel");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomMiddle).path.first().map(String::as_str), Some("framework.category.command"));
    assert_eq!(shell.anchor_state(PanelAnchor::BottomMiddle).path.last().map(String::as_str), Some(surface));
    assert_eq!(shell.expanded_command_id.as_deref(), staged["expandedKey"].as_str());
    assert_eq!(shell.overlay_state, OverlayState::None, "and the palette closes behind it");

    let after = shell.panel_documents.get(surface).expect("mounted Command category successor").header().expect("published Command category successor header");
    assert!(after.generation > before.generation && after.revision != before.revision, "activation atomically replaces the mounted category document");
    let read = shell.panel_documents.get(surface).expect("mounted Command category successor").try_read().expect("Command category successor is readable");
    let keys = (0..read.len()).filter_map(|ordinal| read.node_at(ordinal).map(|record| record.key.as_str().to_string())).collect::<Vec<_>>();
    for field in ["formId", "executeId", "resetId"] {
        let id = staged[field].as_str().expect("retained staged form id");
        assert!(keys.iter().any(|key| key.ends_with(&format!("/{id}"))), "activation successor contains {id}: {keys:?}");
    }
    let unexpanded = staged["unexpandedRowId"].as_str().expect("unexpanded row id");
    assert!(!keys.iter().any(|key| key.ends_with(&format!("/{unexpanded}"))), "the expanded command is absent from the successor list: {keys:?}");
    assert!(shell.closing_documents.terminal_is_empty(), "the exact replaced category owner retires after the retry");
}

fn publish_palette_chrome(shell: &mut ShellState, input: &mut InputState<ActionDescriptor>) {
    while input.retire_hit_step() {}
    shell.screen_w = 1280.0;
    shell.screen_h = 720.0;
    let mut frame = ShellChromeFrameCursor::default();
    let mut draw = DrawList::default();
    let mut overlay = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let theme = Theme::light();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    for _ in 0..65_536 {
        if shell.render_chrome_step(&mut frame, &mut draw, &mut overlay, &mut atlas, &icons, input, &theme, &mut world_resources) {
            shell.publish_retained_hit_registry(input);
            return;
        }
    }
    panic!("palette chrome walk did not reach publication");
}

/// ⚖️ LAW: the normal chrome walk publishes a real dialog textbox and row; the accessibility
/// value route filters that SAME plan, and Enter activates the existing command funnel before the
/// next publication retires the dialog.
#[test]
fn the_command_palette_publishes_filters_and_activates_through_normal_chrome() {
    let mut shell = palette_shell();
    shell.sync_dock_tabs();
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = chord_event("mod+p");
    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(input.focused_id.as_deref(), Some("ui.search.input"), "the authored React textbox owns focus immediately");
    assert!(shell.chrome_surface_census().iter().any(|(id, level, _)| id == "ui.search.dialog" && *level == "dialog"), "the open command surface is a dialog in the chrome census");

    publish_palette_chrome(&mut shell, &mut input);
    let input_hit = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some("ui.search.input") && hit.kind == HitKind::Input).expect("the normal walk published the palette textbox");
    assert!((input_hit.rect.w - 512.0).abs() < 0.01, "React's sm:max-w-lg width is the hit geometry too: {:?}", input_hit.rect);
    assert!(input.hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("ui.search.item."))), "the normal walk publishes command rows, not empty glass");
    let input_node = shell.chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == "ui.search.input").expect("palette textbox is projected to accessibility");
    assert_eq!(input_node.role, "combobox");
    assert!(input_node.editable);
    assert_eq!(input_node.controls.as_deref(), Some("ui.search.list"));
    assert_eq!(input_node.value_text.as_deref(), Some(""));
    assert!(input_node.focused, "shortcut focus and accessibility focus are the same input");
    let target = ui_render::AccessibilityTarget {
        window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(),
        window_generation: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_GENERATION,
        node_id: input_node.node_id,
        node_key: input_node.key,
    };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Value("Set Theme".into()), &mut input)).expect("palette accessibility value"));
    assert_eq!(shell.search_query, "Set Theme");

    publish_palette_chrome(&mut shell, &mut input);
    let rows = shell.chrome_accessibility_nodes(input.hits()).into_iter().filter(|node| node.role == "option").collect::<Vec<_>>();
    assert_eq!(rows.len(), 1, "the query is reflected by the published row set: {rows:?}");
    assert_eq!(rows[0].key, fixture_staged_palette_id());
    assert_eq!(rows[0].label.as_deref(), Some("Set Theme…"), "the deterministic result is the staged command");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Enter, &PointerModifiers::default(), &mut input)).expect("palette activation");
    assert_eq!(shell.overlay_state, OverlayState::None);
    assert_eq!(input.focused_id, None, "closing the dialog retires textbox focus");
    assert!(shell.anchor_open(PanelAnchor::BottomMiddle), "activation reaches the existing Command panel funnel");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomMiddle).path.last().map(String::as_str), Some("command.category.appearance"));
    assert_eq!(shell.expanded_command_id.as_deref(), Some("os:os.setThemeId"));
    publish_palette_chrome(&mut shell, &mut input);
    assert!(!shell.chrome_surface_census().iter().any(|(_, level, _)| *level == "dialog"), "the closed palette is absent from the next dialog census");
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("ui.search."))), "its published textbox and rows retire together");
}

/// ⚖️ LAW: Find uses the same published textbox/row plan as Search, and a physical row click
/// activates its existing selection funnel and retires the modal focus owner.
#[test]
fn find_publishes_filters_and_activates_with_a_physical_row_click() {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("host fixture guest program").install_fixture_action(accept_find_fixture_action);
    shell.dock.root = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("main")], active: "main".into() };
    shell.dock.active_window_id = Some("main".into());
    shell.active_window_id = Some("main".into());
    shell.sync_dock_tabs();
    let window_id = shell.dock.window_instances().first().map(|(id, _)| id.clone()).expect("the app contributes one mounted window");
    let mut graph: ui_wgpu::wgpu::NodeGraphScene = serde_json::from_value(serde_json::json!({
        "nodes": [],
        "edges": [],
        "viewport": { "x": 0.0, "y": 0.0, "zoom": 1.0 }
    }))
    .expect("NodeGraph find fixture decodes");
    graph.find_items.push(ui_wgpu::wgpu::NodeGraphFindItem { id: "media.node".into(), label: "Media Node".into(), category: "Nodes".into() });
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::NodeGraph, &graph).expect("bounded NodeGraph find scene encodes");
    let records = vec![super::shell_input_tests::tree_pointer_record(1, "find-node-graph", ui_contract::Component::Surface(surface), &[], None)];
    let document = shell.publish_surface_records(&window_id, records).expect("NodeGraph production ingress");
    shell.window_ui.insert(window_id, document);
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = chord_event("mod+f");
    shell.handle_keyboard(action, &modifiers, &mut input);
    publish_palette_chrome(&mut shell, &mut input);
    let input_node = shell.chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == "ui.find.input").expect("Find textbox projects to accessibility");
    assert_eq!(input_node.role, "combobox");
    let target = ui_render::AccessibilityTarget {
        window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(),
        window_generation: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_GENERATION,
        node_id: input_node.node_id,
        node_key: input_node.key,
    };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Value("Media".into()), &mut input)).expect("Find accessibility value"));
    publish_palette_chrome(&mut shell, &mut input);
    let row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some("ui.find.item.0")).expect("filtered Find row publishes a physical hit").rect;
    assert_eq!(shell.chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == "media.node").and_then(|node| node.label), Some("Media Node".into()));
    let x = row.x + row.w * 0.5;
    let y = row.y + row.h * 0.5;
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &Theme::light())).expect("Find physical row press");
    assert_eq!(shell.overlay_state, OverlayState::Find, "press only arms the click");
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, false, 0, &mut input, &Theme::light())).expect("Find physical row release");
    assert_eq!(shell.overlay_state, OverlayState::None);
    assert_eq!(input.focused_id, None);
}

fn fixture_staged_palette_id() -> String {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔎️ShellSearch/🧫️fixtures/🔣️.json")).expect("neutral palette fixture");
    fixture["producer"]["stagedCommand"]["id"].as_str().expect("staged palette id").to_string()
}

#[test]
fn palette_dialog_absorbs_interior_clicks_and_rows_activate_only_on_same_row_release() {
    let mut shell = palette_shell();
    shell.overlay_state = OverlayState::Search;
    shell.search_open = true;
    shell.search_query = "Fire".into();
    let mut input = InputState::<ActionDescriptor>::default();
    publish_palette_chrome(&mut shell, &mut input);
    let dialog = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some("ui.search.dialog")).expect("full dialog background hit").rect;
    let row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some("ui.search.item.0")).expect("first row").rect;
    let group = shell
        .command_palette_plan(&Theme::light(), 1280.0, 720.0)
        .expect("open palette plan")
        .entries
        .into_iter()
        .find_map(|entry| match entry {
            ShellPalettePlanEntry::Group { rect, .. } => Some(rect),
            _ => None,
        })
        .expect("palette group heading");
    let heading_point = (group.x + group.w * 0.5, group.y + group.h * 0.5);
    assert_eq!(input.hit_at(heading_point.0, heading_point.1).and_then(|hit| hit.control_id.as_deref()), Some("ui.search.dialog"), "group/padding area belongs to the dialog");
    semio_framework_async::block_on(shell.handle_pointer_button(heading_point.0, heading_point.1, true, 0, &mut input, &Theme::light())).expect("dialog interior press");
    assert_eq!(shell.overlay_state, OverlayState::Search, "interior glass does not dismiss its dialog");

    let x = row.x + row.w * 0.5;
    let y = row.y + row.h * 0.5;
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &Theme::light())).expect("row press");
    semio_framework_async::block_on(shell.handle_pointer_button(dialog.x + 1.0, dialog.y + dialog.h - 1.0, false, 0, &mut input, &Theme::light())).expect("cancelled release");
    assert_eq!(shell.overlay_state, OverlayState::Search, "release away from the armed row cancels activation");
    assert_eq!(shell.search_query, "Fire", "cancellation preserves the query");
    assert_eq!(shell.search_selected, 0, "cancellation preserves the selected row");
    semio_framework_async::block_on(shell.handle_pointer_button(0.0, 0.0, true, 0, &mut input, &Theme::light())).expect("outside press");
    assert_eq!(shell.overlay_state, OverlayState::None, "an outside press dismisses the modal even over underlying chrome");
    assert_eq!(shell.search_query, "Fire", "outside dismissal preserves the query");
}

#[test]
fn palette_accessibility_is_a_dialog_tree_with_close_list_groups_status_and_stable_items() {
    let mut shell = palette_shell();
    shell.overlay_state = OverlayState::Search;
    shell.search_open = true;
    let mut input = InputState::<ActionDescriptor>::default();
    publish_palette_chrome(&mut shell, &mut input);
    let nodes = shell.chrome_accessibility_nodes(input.hits());
    assert_eq!(nodes.first().map(|node| (node.key.as_str(), node.role.as_str(), node.depth)), Some(("ui.search.dialog", "dialog", 0)));
    let close = nodes.iter().find(|node| node.key == "ui.search.close").expect("accessible Close");
    assert_eq!((close.role.as_str(), close.depth, close.actionable), ("button", 1, true));
    let input_node = nodes.iter().find(|node| node.key == "ui.search.input").expect("editable combobox");
    assert_eq!((input_node.role.as_str(), input_node.depth, input_node.editable, input_node.controls.as_deref()), ("combobox", 1, true, Some("ui.search.list")));
    let list = nodes.iter().find(|node| node.key == "ui.search.list").expect("owned listbox");
    assert_eq!((list.role.as_str(), list.depth), ("listbox", 1));
    assert!(nodes.iter().any(|node| node.role == "group" && node.depth == 2));
    assert!(nodes.iter().any(|node| node.role == "option" && node.depth == 3 && node.key.starts_with("command.")), "option keys are canonical palette item IDs");
    assert!(input_node.active_descendant.as_ref().is_some_and(|key| nodes.iter().any(|node| &node.key == key && node.selected == Some(true))));

    shell.set_palette_query(ShellPaletteKind::Search, "zzzz".into(), &mut input);
    publish_palette_chrome(&mut shell, &mut input);
    let empty_nodes = shell.chrome_accessibility_nodes(input.hits());
    assert!(empty_nodes.iter().any(|node| node.role == "status" && node.live == "polite"), "no-results is an announced status");
    let close = empty_nodes.iter().find(|node| node.key == "ui.search.close").expect("accessible close after filtering");
    let target = ui_render::AccessibilityTarget {
        window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(),
        window_generation: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_GENERATION,
        node_id: close.node_id,
        node_key: close.key.clone(),
    };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("accessible Close activation"));
    assert_eq!(shell.overlay_state, OverlayState::None);
    assert_eq!(shell.search_query, "zzzz", "Close preserves the query");
}

/// ⚖️ LAW: the palette's own rows come in React's declaration order — panels, windows, commands, then
/// the host-app verbs — because that order is what the ranker breaks ties on and what a cold palette shows.
#[test]
fn the_palette_rows_follow_the_react_declaration_order() {
    let shell = super::panel_anchor_model_tests::host_test_shell();
    assert!(shell.space_mode);
    let items = shell.build_search_items();
    items.iter().fold(0usize, |rank, item| {
        let bucket = match item.id.split('.').next() {
            Some("panel") => 0,
            Some("window") => 1,
            Some("command") => 2,
            Some("spawn") => 3,
            Some("studio") => 4,
            other => panic!("unknown palette row family {other:?}"),
        };
        assert!(bucket >= rank, "palette row {} is out of React's declaration order", item.id);
        bucket
    });
    let ids = items.iter().map(|item| item.id.as_str()).collect::<Vec<_>>();
    assert_eq!(&ids[ids.len() - 5..], ["spawn.space", "spawn.space", "studio.undo", "studio.redo", "studio.home"]);
    assert!(!ids.iter().any(|id| id.starts_with("keybinding.") || matches!(*id, "studio.commitCheckpoint" | "studio.goHome")));
    let home = items.iter().find(|item| item.id == "studio.home").expect("home row");
    assert_eq!(home.group, "Navigation");

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔎️ShellSearch/🧫️fixtures/🔣️.json")).expect("neutral palette fixture");
    let staged = &fixture["producer"]["stagedCommand"];
    let staged_id = staged["id"].as_str().expect("staged fixture id");
    let staged_row = items.iter().find(|item| item.id == staged_id).expect("native producer publishes the fixture's canonical React id");
    assert_eq!(staged_row.label, staged["label"].as_str().expect("staged fixture label"));
    let staged_action = format!("{}{}", staged["actionPrefix"].as_str().unwrap(), staged["expandedKey"].as_str().unwrap());
    assert_eq!(staged_row.action.as_deref(), Some(staged_action.as_str()));
    for forbidden in fixture["producer"]["forbiddenIds"].as_array().expect("forbidden fixture ids") {
        assert!(!ids.contains(&forbidden.as_str().expect("forbidden id")));
    }
}

#[test]
fn a_palette_without_a_session_is_empty_like_react() {
    let shell = ShellState::new(Vec::new(), String::new());
    assert!(shell.build_search_items().is_empty());
}

//#endregion 🎛️CommandPalette
