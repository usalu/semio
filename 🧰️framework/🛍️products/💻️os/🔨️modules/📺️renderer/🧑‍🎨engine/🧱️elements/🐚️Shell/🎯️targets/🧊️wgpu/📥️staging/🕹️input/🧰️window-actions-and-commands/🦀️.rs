//! 🕹️ `shell-input` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
/// ⌨️ The key token a captured chord writes for one key event — the inverse of
/// [`key_event_matches_chord`]'s key arm, so a chord this renderer captures is one it can match again.
/// `None` for a key no chord can name (a bare modifier never reaches here as a `KeyAction`).
pub fn keybinding_capture_key_token(action: &semio_framework_ui_wgpu_input::KeyAction) -> Option<String> {
    Some(match action {
        semio_framework_ui_wgpu_input::KeyAction::Char(key) => {
            let key = key.trim().to_ascii_lowercase();
            if key.is_empty() {
                return None;
            }
            key
        }
        semio_framework_ui_wgpu_input::KeyAction::Enter => "enter".into(),
        semio_framework_ui_wgpu_input::KeyAction::Escape => return None,
        semio_framework_ui_wgpu_input::KeyAction::Backspace => "backspace".into(),
        semio_framework_ui_wgpu_input::KeyAction::Delete => "delete".into(),
        semio_framework_ui_wgpu_input::KeyAction::Tab => "tab".into(),
        semio_framework_ui_wgpu_input::KeyAction::ArrowLeft => "arrowleft".into(),
        semio_framework_ui_wgpu_input::KeyAction::ArrowRight => "arrowright".into(),
        semio_framework_ui_wgpu_input::KeyAction::ArrowUp => "arrowup".into(),
        semio_framework_ui_wgpu_input::KeyAction::ArrowDown => "arrowdown".into(),
        semio_framework_ui_wgpu_input::KeyAction::Home => "home".into(),
        semio_framework_ui_wgpu_input::KeyAction::End => "end".into(),
        semio_framework_ui_wgpu_input::KeyAction::PageUp => "pageup".into(),
        semio_framework_ui_wgpu_input::KeyAction::PageDown => "pagedown".into(),
        semio_framework_ui_wgpu_input::KeyAction::Function(number) => format!("f{number}"),
        semio_framework_ui_wgpu_input::KeyAction::Space(_) => "space".into(),
    })
}

//#region ⌨️ShellShortcutTable
/// ⌨️ One row's remapped state for the Keybindings settings leaf: the chord in force, whether it is a
/// user override, and whether some LATER row claims the same first chord — React's
/// `buildSettingsKeybindingsTree` conflict marker (`📌️ChromePanels/🟦️.tsx`), whose rule is
/// last-writer-wins over the first chord of every row, so exactly the earlier row of a colliding pair
/// is flagged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellKeybindingRow {
    pub control_id: String,
    pub keys: String,
    pub overridden: bool,
    pub conflicted: bool,
}

/// ⌨️ Every shortcut row with its remapped chord and conflict marker, sorted by control id like
/// React's own `localeCompare` row order.
pub fn shell_keybinding_rows(overrides: &HashMap<String, String>) -> Vec<ShellKeybindingRow> {
    let table = shell_shortcut_table(overrides);
    let first_chord = |keys: &str| keys.split(',').map(str::trim).find(|chord| !chord.is_empty()).map(str::to_ascii_lowercase);
    let mut chord_owners: HashMap<String, String> = HashMap::new();
    for (control_id, keys) in &table {
        if let Some(chord) = first_chord(keys) {
            chord_owners.insert(chord, control_id.clone());
        }
    }
    let mut rows: Vec<ShellKeybindingRow> = table
        .iter()
        .map(|(control_id, keys)| ShellKeybindingRow {
            conflicted: first_chord(keys).is_some_and(|chord| chord_owners.get(&chord) != Some(control_id)),
            control_id: control_id.clone(),
            keys: keys.clone(),
            overridden: overrides.contains_key(control_id),
        })
        .collect();
    rows.sort_by(|left, right| left.control_id.cmp(&right.control_id));
    rows
}

//#endregion ⌨️ShellShortcutTable
impl ShellInputState {
    /// 🖱️ The cursor the active utility requests while the pointer is inside the active window silhouette — maps
    /// `UtilityDefinition.cursor` onto a {@link ui_wgpu::wgpu::SemioCursor} (P5). `None` when no utility/cursor applies.
    pub fn utility_cursor_override(&self, x: f32, y: f32, docs: &DocumentState, windows: &WindowState) -> Option<semio_framework_ui_wgpu_hosting::cursor::SemioCursor> {
        let session = docs.session.as_ref()?;
        let window_id = windows.active_window_id.as_deref()?;
        let utility_id = windows.active_utility_for_window(window_id)?;
        let cursor_name = session.app.utilities.iter().find(|utility| utility.id == utility_id)?.cursor.as_deref()?;
        let geometry = &self.presented_input_geometry;
        let visible = geometry.window_silhouettes.get(window_id).map_or_else(|| geometry.window_content_rects.get(window_id).is_some_and(|rect| rect.contains(x, y)), |silhouette| silhouette.contains(x, y));
        visible.then(|| semio_cursor_from_name(cursor_name))
    }
}

//#endregion ActionPanelAndUtilities

