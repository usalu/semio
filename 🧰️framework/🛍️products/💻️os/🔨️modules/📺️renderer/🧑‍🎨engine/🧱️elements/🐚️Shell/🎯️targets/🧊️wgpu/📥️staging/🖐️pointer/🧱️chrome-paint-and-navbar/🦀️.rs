//! 🖐️ `shell-pointer` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

fn floating_panel_available_width(body: Rect, theme: &Theme) -> f32 {
    (body.w - theme.panel_inset * 2.0).max(theme.panel_min_width)
}

pub fn floating_panel_max_width(body: Rect, theme: &Theme) -> f32 {
    theme.panel_max_width.min(floating_panel_available_width(body, theme)).max(theme.panel_min_width)
}

/// 📑️ The window id behind a dock tab's LABEL target (`dock.tab.<path>.<windowId>`) — `None` for the
/// action chips (`….focus`/`….close`/`….drag`), which carry one dot more. React's label is its own
/// `<button>` beside those chips (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1049-1071`).
pub fn dock_tab_select_window_id(control_id: &str) -> Option<&str> {
    let rest = control_id.strip_prefix("dock.tab.")?;
    let (_, window_id) = rest.split_once('.')?;
    (!window_id.contains('.')).then_some(window_id)
}

/// ↔ Which anchor a `panel.resize.<anchor>.<inner|outer>` control belongs to, and whether it is the
/// inner (canvas-facing) handle — the one id shape both the registration and the drag arm parse.
pub fn panel_resize_anchor(control_id: &str) -> Option<(PanelAnchor, bool)> {
    let rest = control_id.strip_prefix("panel.resize.")?;
    let (anchor_id, side) = rest.rsplit_once('.')?;
    Some((PanelAnchor::from_str(anchor_id)?, side == "inner"))
}

//#region ActionPanelAndUtilities
/// ⌨️ {@link is_reserved_shell_chord} against a RESOLVED table, so a REMAPPED shell chord is reserved
/// at its new spelling and the default it replaced becomes an app's to claim — React's
/// `reservedShellChordsV1(SHELL_KEYBINDINGS, uiKeybindingOverrides)`.
pub fn is_reserved_shell_chord_in(table: &[(String, String)], action: &semio_framework_ui_wgpu_input::KeyAction, modifiers: &PointerModifiers) -> bool {
    shell_shortcut_for_in(table, action, modifiers).is_some()
}

//#region 🔀️ChromeParity
/// 🛑️ The declared shape of a `World3dScene.statusJson` cancel affordance — the projection of
/// [`World3dComputeStatus`] the overlay control layer needs and nothing more.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct World3dCancelAffordance {
    pub cancellable: bool,
    pub cancel_action: String,
    pub cancel_args: serde_json::Map<String, Value>,
}

/// 🛑️ Reads the cancel contract out of one `statusJson`. The shell learns no domain verb from code:
/// whatever id `cancelAction` names is what the control dispatches.
pub fn world3d_cancel_affordance(status_json: Option<&str>) -> World3dCancelAffordance {
    let status = world3d_compute_status(status_json);
    World3dCancelAffordance { cancellable: status.cancellable, cancel_action: status.cancel_action, cancel_args: status.cancel_args }
}

//#endregion 🔀️ChromeParity
//#region 🪟️WindowPaneChrome
/// 🧰️ The routing namespace every row of a pane's UNFOLDED Utilities rail carries, its TAIL being the
/// utility's own framework element id — the id React renders on the leaf (`UtilityTree`'s
/// `id={entry.id}`, `🎛️UtilityTree/🟦️.tsx:237`) and the id an introduction step addresses it by, which
/// this renderer also publishes through `ShellChromeBuildState::register_element_rect`. React needs no
/// namespace because its rail is DOM above the canvas; this renderer routes a press by id, and the rail
/// is painted INSIDE a world pane's rect, so without a claimable prefix every utility press was handed
/// to the world surface instead of the shell — the same gap the pane chips had
/// (`📓️w12d-pane-chip-ids-and-projection-toggle.md` §3).
pub const WINDOW_UTILITY_RAIL_PARENT: &str = "framework.utility.";

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

//#region ActionPanelAndUtilities
/// ⌨️ Whether a key event is one of the hardcoded shell chords (palette/find/panels/nav/surface-role/
/// mode-cycle) that must win over app-declared keybindings (P4 — "reserved shell chords still win").
///
/// The `mod+alt+e`/`mod+alt+v` and `mod+alt+←`/`mod+alt+→` rows are the wgpu twin of React's
/// `SHELL_KEYBINDINGS` entries for `playground.navbar.roles.{editor,viewer}` and
/// `ui.shell.mode.{next,previous}` — see `🧱️elements/🏛️ShellHost/🟦️.tsx` and
/// `🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json`'s `keybindings` rows, which pin both spellings.
#[cfg(any(test, feature = "testkit"))]
pub fn is_reserved_shell_chord(action: &semio_framework_ui_wgpu_input::KeyAction, modifiers: &PointerModifiers) -> bool {
    is_reserved_shell_chord_in(&shell_shortcut_table(&HashMap::new()), action, modifiers)
}
//#endregion ActionPanelAndUtilities
