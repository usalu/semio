//! 🧰️ Window actions, search panes, window scope, shortcuts and the command registry.
//!
//! Window actions and search panes, window scope resolution, chord helpers, the shell shortcut table, staged
//! arguments, the OS command registry and panel, and fuzzy ranking.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ActionPanelAndUtilities


//#region ⌨️ShellShortcutTable

/// ⌨️ The shell verb a key event asks for with NO override applied — the default-table projection the
/// shortcut LAWS assert against (production always resolves through `ShellState::shortcut_table`, so an
/// override is never bypassed).
#[cfg(test)]
pub(crate) fn shell_shortcut_for(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ShellShortcut> {
    shell_shortcut_for_in(&shell_shortcut_table(&HashMap::new()), action, modifiers)
}

//#endregion ⌨️ShellShortcutTable


//#endregion ActionPanelAndUtilities
