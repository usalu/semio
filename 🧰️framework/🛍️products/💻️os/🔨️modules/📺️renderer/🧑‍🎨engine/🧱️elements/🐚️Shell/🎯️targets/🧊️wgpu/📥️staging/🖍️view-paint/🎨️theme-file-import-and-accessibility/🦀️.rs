//! 🖍 `shell-view-paint` staging of `theme-file-import-and-accessibility`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ♿️ChromeAccessibleNames
/// ♿️ The accessible NAME each chrome control painted this walk, keyed by control id.
///
/// 🩸️ A GPU canvas has no elements, so the only accessible names this target ever published came out
/// of the retained DOCUMENT projection (`dumpAccessibility` over `UI_ENGINE`) — the scene trees. Every
/// navbar chip, footer pill, panel tab, pane chip and surface control was invisible to a reader
/// (audit W14 §C4), even though React gives each of them an `sr-only` label for free. The two central
/// chrome item painters record the label they are ABOUT to paint here, so the published name is
/// always the text on screen rather than a second, drifting table.
///
/// ⛓️ Staged per walk and promoted by `publish_retained_hit_registry` together with the hit registry,
/// so names and targets can never describe different frames — the same double-buffer discipline
/// `retained_hit_windows` follows. Bounded by [`SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY`].
pub(super) const SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY: usize = 512;

/// ♿️ Semantic properties captured beside the chrome label during its paint walk.
pub(super) struct ChromeControlPresentation {
    pub(crate) label: String,
    pub(crate) description: Option<String>,
    pub(crate) role: Option<&'static str>,
    pub(crate) pressed: Option<bool>,
    pub(crate) selected: Option<bool>,
    pub(crate) controls: Option<String>,
    pub(crate) value_text: Option<String>,
    pub(crate) value_range: Option<[f64; 3]>,
    pub(crate) disabled: bool,
}
//#endregion ♿️ChromeAccessibleNames
