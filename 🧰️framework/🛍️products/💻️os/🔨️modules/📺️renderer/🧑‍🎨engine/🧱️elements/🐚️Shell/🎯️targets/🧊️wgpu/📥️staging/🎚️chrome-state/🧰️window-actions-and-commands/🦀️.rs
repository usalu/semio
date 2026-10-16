//! 🎚️ `shell-chrome-state` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region ⌨️ShellShortcutTable
/// ⌨️ The shell's OWN shortcut table, control id → chords, transcribed from React's `SHELL_KEYBINDINGS`
/// (`🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`). One table drives all three consumers —
/// [`shell_shortcut_for`] (what a key event means), [`is_reserved_shell_chord`] (what an app may not
/// shadow) and the report's shortcut matrix — so a row can never be reserved without being dispatched,
/// or dispatched without being reserved.
///
/// Only the rows whose verb is a SHELL-chrome transition live here. React's `ui.introduction.*`,
/// `ui.dialog.*` rows carry no accelerator (`escape`/`enter`/arrows): they belong to whichever overlay
/// has focus, and this target answers them in `handle_keyboard`'s tour/sync-card/palette arms ahead of
/// this table, exactly as React answers them inside the dialog/introduction components.
///
/// ⏪️ The three `ui.timeTravel.*` rows are React's too: they dispatch the reserved `historyEdit*` verbs, only
/// while the open session offers them, and none of them is Escape — Escape never discards a history edit.
pub const SHELL_SHORTCUT_ROWS: &[(&str, &str)] = &[
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
    ("ui.timeTravel.accept", "alt+enter"),
    ("ui.timeTravel.discard", "alt+backspace"),
    ("ui.timeTravel.exit", "alt+shift+backspace"),
];

/// ⌨️ The REMAPPABLE shortcut table: [`SHELL_SHORTCUT_ROWS`] with the user's `keybindingOverrides`
/// applied per control id — the Rust twin of React's `composeControlKeybindings`
/// (`🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`), whose precedence is
/// shell defaults, then app-declared action bindings (only for ids the shell does not claim), then
/// the user override, unconditionally. App-declared bindings never enter *this* table: they are
/// keyed by action id and dispatched by `match_app_keybinding` on the rung below the shell's, so the
/// only layer over the defaults here is the override map.
///
/// An override whose value carries no parsable chord is dropped rather than blanking the row, exactly
/// like React's `parseUiKeybindingOverrides` gate.
pub fn shell_shortcut_table(overrides: &HashMap<String, String>) -> Vec<(String, String)> {
    SHELL_SHORTCUT_ROWS
        .iter()
        .map(|(control_id, keys)| {
            let resolved = overrides.get(*control_id).filter(|keys| keys.split(',').any(|chord| !chord.trim().is_empty())).map(String::as_str).unwrap_or(keys);
            ((*control_id).to_string(), resolved.to_string())
        })
        .collect()
}

//#endregion ⌨️ShellShortcutTable
//#endregion ActionPanelAndUtilities

//#region ActionPanelAndUtilities
//#region 🎬️WindowActionsAndSearchPanes
/// 🔎️ The retained engine surface one window instance's SEARCH pane body paints into.
pub fn window_search_surface_id(window_id: &str) -> String {
    format!("{window_id}/{WINDOW_SEARCH_BODY_KEY}")
}

/// 🎬️ The retained engine surface one window instance's ACTIONS pane body paints into — React's
/// top-left `Pane` (`🪟️Window/🟦️.tsx:379`), whose body is `<Engagement/>` followed by the
/// `WindowActionPane` tree. Keyed like the Measures overlay: the window instance, then the reserved
/// section body key, so it can never collide with an app-authored surface.
pub fn window_actions_surface_id(window_id: &str) -> String {
    format!("{window_id}/{}", semio_framework::UiRefreshSection::Engagements.body_key())
}

/// 🔀️ Reserved retained surface suffix for the World Projection pane's Tree body.
pub const WINDOW_PROJECTION_BODY_KEY: &str = "framework.worldOrbit.projection.body";

/// 🔎️ The reserved body key of the per-window SEARCH pane. React mounts it as a second `Pane` fed by
/// the SAME `WindowEngagement` (`windowEngagementToSearchSpec`, `🛠️ShellHelpers/🟦️.tsx:1826`), so it
/// has no `UiRefreshSection` of its own and takes the engagements key with its own suffix.
pub const WINDOW_SEARCH_BODY_KEY: &str = "framework.section.engagements.search";

/// 🔀️ The retained Tree surface owned by one World window instance's Projection pane.
pub fn window_projection_surface_id(window_id: &str) -> String {
    format!("{window_id}/{WINDOW_PROJECTION_BODY_KEY}")
}
//#endregion 🎬️WindowActionsAndSearchPanes
//#endregion ActionPanelAndUtilities
