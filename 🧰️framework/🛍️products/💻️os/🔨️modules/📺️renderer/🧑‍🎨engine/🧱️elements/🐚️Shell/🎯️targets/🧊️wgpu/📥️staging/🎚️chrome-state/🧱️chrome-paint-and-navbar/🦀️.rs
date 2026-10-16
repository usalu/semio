//! 🎚️ `shell-chrome-state` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
/// ⌨️ Formats the first chord of a keybinding for chip badges, palette rows and context-menu shortcut
/// labels — the port of `formatKeybindingShortcut`
/// (`🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`), pinned glyph for glyph by
/// `🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json`, which React's own suite reads too.
///
/// 🩸️ `apple` used to be `cfg!(target_os = "macos")`. That is a COMPILE-time fact and it is FALSE on
/// `wasm32-unknown-unknown`, so every browser boot — macOS included — painted `Ctrl+Alt+E` where
/// React painted `⌘️⌥️E`. It now reads the host's own platform, published by whoever owns a
/// `navigator` (`🧊️renderer/🦀️.rs`, region ⌨️HostPlatform).
pub(crate) fn format_keybinding_shortcut(keys: &str) -> String {
    let chord = keys.split(',').next().unwrap_or(keys).trim().to_ascii_lowercase();
    if chord.is_empty() {
        return String::new();
    }
    let apple = crate::host_platform_uses_meta();
    let glyph = |part: &str| -> String {
        match part {
            "mod" if apple => "⌘️".into(),
            "mod" => "Ctrl".into(),
            "ctrl" if apple => "⌃️".into(),
            "ctrl" => "Ctrl".into(),
            "meta" => "⌘️".into(),
            "alt" if apple => "⌥️".into(),
            "alt" => "Alt".into(),
            "shift" if apple => "⇧️".into(),
            "shift" => "Shift".into(),
            "backspace" => "⌫️".into(),
            "delete" => "⌦️".into(),
            "enter" if apple => "↵️".into(),
            "enter" => "Enter".into(),
            "escape" if apple => "⎋️".into(),
            "escape" => "Esc".into(),
            "up" | "arrowup" => "↑".into(),
            "down" | "arrowdown" => "↓".into(),
            "left" | "arrowleft" => "←".into(),
            "right" | "arrowright" => "→".into(),
            other if other.len() == 1 => other.to_ascii_uppercase(),
            other => {
                let mut s = other.to_string();
                if let Some(first) = s.get_mut(0..1) {
                    first.make_ascii_uppercase();
                }
                s
            }
        }
    };
    let parts: Vec<String> = chord.split('+').map(str::trim).filter(|part| !part.is_empty()).map(glyph).collect();
    if apple {
        parts.join("")
    } else {
        parts.join("+")
    }
}

//#region 🔀️ChromeParity
/// ⌨️ The inline hotkey badge React's `ControlHotkeyBadge` paints INSIDE a chrome control
/// (`🔨️modules/⌨️control-hotkey-presentation/🟦️.tsx:32`) whenever the active driver's `hotkeys` axis is
/// `"inline"` — which `DEFAULT_UI_DRIVER` sets (`🧱️elements/🚗️UiDriver/🟦️.tsx:41`), so the desktop
/// navbar's role buttons read `Editor ⌘️⌥️E` / `Viewer ⌘️⌥️V`. The chord comes from this session's own
/// remappable table, so an override is reflected in the badge exactly as React's
/// `useControlHotkey(id)` reads `controlKeybindings`.
pub(crate) fn shell_control_hotkey_badge(table: &[(String, String)], control_id: &str) -> Option<String> {
    table.iter().find(|(id, _)| id == control_id).map(|(_, keys)| format_keybinding_shortcut(keys))
}

//#endregion 🔀️ChromeParity
//#endregion ActionPanelAndUtilities

