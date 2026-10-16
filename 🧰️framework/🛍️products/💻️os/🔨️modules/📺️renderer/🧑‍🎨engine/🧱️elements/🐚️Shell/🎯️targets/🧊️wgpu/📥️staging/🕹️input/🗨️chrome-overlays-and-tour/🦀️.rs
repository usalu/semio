//! 🕹️ `shell-input` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🗨️ChromeDialog
/// 🎯️ One focus stop of the dialog, in visiting order. `Chip(field, index)` is one removable reference of a
/// reference field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChromeDialogStop {
    Field(usize),
    Chip(usize, usize),
    Cancel,
    Choice(usize),
    Confirm,
}

/// ♿️ What a reader hears beyond a control's name: its role, pressed state and value.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ChromeDialogSemantics {
    pub(crate) role: Option<&'static str>,
    pub(crate) pressed: Option<bool>,
    pub(crate) value_text: Option<String>,
    pub(crate) value_range: Option<[f64; 3]>,
}

/// 🖌️ One step of the dialog's flat paint program — the retained chrome step walks it by index, so a
/// glyph run that needs another opportunity simply does not advance. `Segment` and `Nudge` are the pressable
/// parts of a segmented and a stepper field.
pub(crate) enum ChromeDialogPaintOp {
    Scrim,
    Modal(Rect),
    Fill { rect: Rect, color: Rgba },
    Text { value: String, x: f32, y: f32, max_w: f32, size: f32, color: Rgba },
    Hit { rect: Rect, stop: ChromeDialogStop, control_id: String, label: String, description: Option<String>, kind: HitKind, event: Option<ActionDescriptor>, semantics: ChromeDialogSemantics, disabled: bool },
    Segment { rect: Rect, field: usize, option: usize, control_id: String, label: String, selected: bool },
    Nudge { rect: Rect, field: usize, sign: f64, control_id: String, label: String },
    Clicks { dialog: Rect },
}

/// 👆️ What one click on the open dialog landed on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ChromeDialogPress {
    Stop { stop: ChromeDialogStop, enabled: bool, rect: Rect },
    Segment { field: usize, option: usize },
    Nudge { field: usize, sign: f64 },
}

//#endregion 🗨️ChromeDialog
/// 🆔️ React's `panelTabElementId` (`🖥️platform/🟦️.ts`): the id an introduction step names a panel tab
/// by. The tab id is already dotted, so it is appended verbatim — no `element_id_segment` pass.
pub(crate) fn panel_tab_introduction_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}")
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

