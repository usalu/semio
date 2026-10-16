//! 📄️ `shell-docs` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🎨️ThemeDocumentModel
pub(crate) fn custom_theme_document_from(prefs: &ChromePrefsState, custom_id: &str) -> Option<ThemeDocument> {
    ThemeDocument::parse(prefs.custom_themes.get(custom_id)?)
}

//#endregion 🎨️ThemeDocumentModel
//#region ⚖️MergePolicyAndThemeCommits
/// ⚖️ React's `MERGE_POLICY_OPTIONS` (`📌️ChromePanels/🟦️.tsx:339`) with each option's label key —
/// the variant names `protocol::MergePolicy` serializes to, so the value a user picks here is the
/// value an authority reads.
pub(crate) const SHELL_MERGE_POLICY_OPTIONS: &[(&str, &str)] = &[("LaissezFaire", "settings.mergePolicy.laissezFaire"), ("Normal", "settings.mergePolicy.normal"), ("Vigilant", "settings.mergePolicy.vigilant")];

//#endregion ⚖️MergePolicyAndThemeCommits

