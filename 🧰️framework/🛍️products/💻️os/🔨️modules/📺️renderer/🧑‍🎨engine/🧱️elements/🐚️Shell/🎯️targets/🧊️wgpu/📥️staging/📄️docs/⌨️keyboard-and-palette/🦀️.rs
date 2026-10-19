//! 📄️ `shell-docs` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl DocumentState {
    /// ⌨️ The app keybinding matching the current key event, if any.
    ///
    /// 🩸️ A binding's `keys` is a COMMA-SEPARATED chord list — React's `handleAppKeydown` splits it
    /// (`parseKeys`) and puzzle2d's `deleteSelection` declares `"delete,backspace"`. This target fed
    /// the whole string to [`key_event_matches_chord`], which splits on `+` alone, so every
    /// multi-chord binding in the repo was dead on this renderer: `"delete,backspace"` resolved the
    /// key token `"backspace"` with a literal `delete,` prefix and matched neither key.
    ///
    /// 🛡️ The reserved check is PER CHORD, exactly where React puts it (`if (reservedChords.has(chord))
    /// continue`), so a binding whose second chord is the shell's still fires on its first.
    pub fn match_app_keybinding(&self, action: &semio_framework_ui_wgpu_input::KeyAction, modifiers: &PointerModifiers, chrome: &ChromeState) -> Option<ActionDescriptor> {
        let session = self.session.as_ref()?;
        let reserved = reserved_shell_chords_v1(&chrome.shortcut_table(), &[]);
        session
            .app
            .keybindings
            .iter()
            .find(|binding| binding.keys.split(',').map(|chord| chord.trim().to_ascii_lowercase()).filter(|chord| !chord.is_empty()).any(|chord| key_event_matches_chord(action, modifiers, &chord) && !reserved.contains(&chord)))
            .map(|binding| binding.action.clone())
    }
}

//#endregion ShellInput

/// 🪶️ Drops the spawned-app entry a closed window owned and re-points `active_spawned_id` at
/// whatever is left — the pure half of [`Self::teardown_spawned_window`], mirroring React's
/// `buildSpacePanelState(nextSpawned, tab, nextSpawned[0]?.id)` (`🏛️ShellHost/🟦️.tsx:10537`).
pub fn take_spawned_entry(panel: &mut SpacePanelState, window_id: &str) -> Option<SpawnedAppEntry> {
    let slot = panel.spawned_apps.iter().position(|entry| entry.id == window_id)?;
    let closed = panel.spawned_apps.remove(slot);
    panel.active_spawned_id = panel.spawned_apps.first().map(|entry| entry.id.clone());
    Some(closed)
}
