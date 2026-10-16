//! 🖐️ `shell-pointer` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region ⌨️ShellShortcutTable
/// ⌨️ The verb a control id names — the single mapping from React's control ids to this target's chrome.
pub fn shell_shortcut_for_control_id(control_id: &str) -> Option<ShellShortcut> {
    Some(match control_id {
        "ui.search.toggle" => ShellShortcut::ToggleSearch,
        "ui.find.toggle" => ShellShortcut::ToggleFind,
        "ui.nav.back" => ShellShortcut::NavBack,
        "ui.nav.forward" => ShellShortcut::NavForward,
        "ui.nav.up" => ShellShortcut::NavUp,
        "ui.shell.panelAnchor.topLeft" => ShellShortcut::TogglePanelAnchor(PanelAnchor::TopLeft),
        "ui.shell.panelAnchor.topMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::TopMiddle),
        "ui.shell.panelAnchor.topRight" => ShellShortcut::TogglePanelAnchor(PanelAnchor::TopRight),
        "ui.shell.panelAnchor.rightMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::RightMiddle),
        "ui.shell.panelAnchor.bottomRight" => ShellShortcut::TogglePanelAnchor(PanelAnchor::BottomRight),
        "ui.shell.panelAnchor.bottomMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::BottomMiddle),
        "ui.shell.panelAnchor.bottomLeft" => ShellShortcut::TogglePanelAnchor(PanelAnchor::BottomLeft),
        "ui.shell.panelAnchor.leftMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::LeftMiddle),
        "ui.window.close" => ShellShortcut::CloseWindow,
        "ui.window.focus" => ShellShortcut::FocusWindow,
        "ui.window.newWindow" => ShellShortcut::NewWindow,
        "ui.shell.mode.next" => ShellShortcut::ModeStep(1),
        "ui.shell.mode.previous" => ShellShortcut::ModeStep(-1),
        "playground.navbar.roles.editor" => ShellShortcut::SurfaceRole(semio_framework::manifest::AppRole::Editor),
        "playground.navbar.roles.viewer" => ShellShortcut::SurfaceRole(semio_framework::manifest::AppRole::Viewer),
        other => ShellShortcut::TimeTravel(TimeTravelVerb::from_shortcut_id(other)?),
    })
}

/// ⌨️ The shell verb a key event asks for against a RESOLVED table, or `None` when the event belongs
/// to the app/content. First matching row wins, in table order, exactly like React's own scan.
pub fn shell_shortcut_for_in(table: &[(String, String)], action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ShellShortcut> {
    table.iter().find(|(_, keys)| keys.split(',').any(|chord| key_event_matches_chord(action, modifiers, chord.trim()))).and_then(|(control_id, _)| shell_shortcut_for_control_id(control_id))
}

/// ⏪️ The framework-universal edit chords React hardcodes at the very END of `handleAppKeydown`
/// (`🏛️ShellHost/🟦️.tsx`): `mod+z`, `mod+shift+z` and the `mod+y` redo alias. Deliberately NOT part of
/// [`SHELL_SHORTCUT_ROWS`] — React dispatches them AFTER the app-declared keybinding loop, so an app
/// that declares `mod+z` itself shadows them, and they are therefore never reserved chords.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellEditVerb {
    Undo,
    Redo,
}

impl ShellEditVerb {
    pub fn action_id(self) -> &'static str {
        match self {
            ShellEditVerb::Undo => "undo",
            ShellEditVerb::Redo => "redo",
        }
    }
}

/// ⏪️ The undo/redo verb a key event asks for.
pub fn shell_edit_verb_for(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ShellEditVerb> {
    if key_event_matches_chord(action, modifiers, "mod+z") {
        return Some(ShellEditVerb::Undo);
    }
    if key_event_matches_chord(action, modifiers, "mod+shift+z") || key_event_matches_chord(action, modifiers, "mod+y") {
        return Some(ShellEditVerb::Redo);
    }
    None
}

//#endregion ⌨️ShellShortcutTable
//#endregion ActionPanelAndUtilities

//#region ActionPanelAndUtilities
//#region ShellCommandHistory
impl<'ctx> PointerCtx<'ctx> {
    /// 🕒️ `ui.panelToggle.details`/`ui.panelToggle.settings` share this: both log the same
    /// `shell.panelToggle` command, differing only in the `panel` id and the post-toggle `visible`
    /// flag — the caller has already flipped `right_panel_open`/`active_right_kind` before this runs,
    /// so `visible` reads the POST-toggle state directly (no separate before/after bookkeeping).
    pub async fn note_panel_toggle_command(&mut self, control_id: &str, panel: &str) -> Result<(), String> {
        let visible = match panel {
            "details" => self.dock.anchor_open(PanelAnchor::TopRight),
            "settings" => self.dock.anchor_open(PanelAnchor::BottomRight),
            "chat" => self.dock.anchor_open(PanelAnchor::TopRight) && self.dock.anchor_state(PanelAnchor::TopRight).active_tab() == Some(FRAMEWORK_CHAT_PANEL_ID),
            _ => false,
        };
        self.as_actions().note_control_command(control_id, Some(serde_json::json!({ "panel": panel, "visible": visible }))).await
    }
}
//#endregion ShellCommandHistory
//#endregion ActionPanelAndUtilities

//#region ActionPanelAndUtilities
//#region ⌨️ShellShortcutTable
/// ⌨️ The shell verb a key event asks for with NO override applied — the default-table projection the
/// shortcut LAWS assert against (production always resolves through `ShellState::shortcut_table`, so an
/// override is never bypassed).
#[cfg(any(test, feature = "testkit"))]
pub fn shell_shortcut_for(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ShellShortcut> {
    shell_shortcut_for_in(&shell_shortcut_table(&HashMap::new()), action, modifiers)
}
//#endregion ⌨️ShellShortcutTable
//#endregion ActionPanelAndUtilities
