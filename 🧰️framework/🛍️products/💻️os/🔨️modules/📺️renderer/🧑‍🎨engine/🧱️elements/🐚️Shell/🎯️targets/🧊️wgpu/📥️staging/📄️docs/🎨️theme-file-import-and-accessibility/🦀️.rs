//! 📄️ `shell-docs` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🎨️ThemeDocumentModel
pub fn custom_theme_document_from(prefs: &ChromePrefsState, custom_id: &str) -> Option<ThemeDocument> {
    ThemeDocument::parse(prefs.custom_themes.get(custom_id)?)
}

//#endregion 🎨️ThemeDocumentModel
//#region ⚖️MergePolicyAndThemeCommits
/// ⚖️ React's `MERGE_POLICY_OPTIONS` (`📌️ChromePanels/🟦️.tsx:339`) with each option's label key —
/// the variant names `protocol::MergePolicy` serializes to, so the value a user picks here is the
/// value an authority reads.
pub const SHELL_MERGE_POLICY_OPTIONS: &[(&str, &str)] = &[("LaissezFaire", "settings.mergePolicy.laissezFaire"), ("Normal", "settings.mergePolicy.normal"), ("Vigilant", "settings.mergePolicy.vigilant")];

//#endregion ⚖️MergePolicyAndThemeCommits

//#region 🎨️ThemeDocumentDoors
/// 🎨️ The resolved theme for a canonical document-backed selection or live draft.
///
/// 🧊️ Cached on `(theme id, appearance, draft generation)`: `resolve_theme_for_ids` runs once per
/// frame per appearance, and re-resolving ~450 document entries there would put a JSON walk in the
/// frame loop.
pub fn document_theme_for_ids(theme_id: &str, dark: bool) -> Option<Theme> {
    with_chrome_prefs(|prefs| {
        let key = (theme_id.to_string(), dark, prefs.theme_document_generation);
        if let Some((cached, theme)) = prefs.theme_document_resolved.as_ref() {
            if cached == &key {
                return Some(*theme);
            }
        }
        let document = match prefs.theme_document_draft.clone() {
            Some(draft) => draft,
            None => custom_theme_document_from(prefs, theme_id)?,
        };
        let theme = theme_from_document(&document, dark);
        prefs.theme_document_resolved = Some((key, theme));
        Some(theme)
    })
}
//#endregion 🎨️ThemeDocumentDoors
