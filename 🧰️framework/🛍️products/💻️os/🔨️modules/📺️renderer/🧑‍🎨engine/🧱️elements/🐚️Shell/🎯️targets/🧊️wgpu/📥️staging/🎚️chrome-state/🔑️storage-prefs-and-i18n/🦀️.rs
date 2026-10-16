//! 🎚️ `shell-chrome-state` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🎨️ThemeRegistry
/// 🎛️ Owned preference snapshot consumed while constructing chrome.
#[derive(Clone)]
pub(crate) struct ChromePrefsState {
    pub(crate) ui_layout: String,
    pub(crate) theme_id: String,
    pub(crate) custom_drivers: HashMap<String, OsUiDriver>,
    pub(crate) custom_themes: HashMap<String, String>,
    pub(crate) keybinding_overrides: HashMap<String, String>,
    /// 🎨️ The UNSAVED theme document the settings panel is editing, mirrored here so the present
    /// side's `resolve_theme_for_ids` re-tokenises from it — React's `uiThemeDraft` reaching
    /// `applyUiThemeToRoot`.
    pub(crate) theme_document_draft: Option<ThemeDocument>,
    /// 🎨️ Bumped on every draft/registry write, so the cache below can never answer a stale theme.
    pub(crate) theme_document_generation: u64,
    /// 🧊️ `(theme id, dark, generation) -> resolved Theme` — one document walk per change, not one
    /// per frame.
    pub(crate) theme_document_resolved: Option<((String, bool, u64), Theme)>,
    pub(crate) worker_count: u32,
}

impl Default for ChromePrefsState {
    fn default() -> Self {
        Self {
            ui_layout: "desktop".to_string(),
            theme_id: "semio".to_string(),
            custom_drivers: HashMap::new(),
            custom_themes: HashMap::new(),
            keybinding_overrides: HashMap::new(),
            theme_document_draft: None,
            theme_document_generation: 0,
            theme_document_resolved: None,
            worker_count: default_compute_worker_count(),
        }
    }
}

//#endregion 🎨️ThemeRegistry
//#region 💾️PrefsSync
#[derive(Clone, Default, PartialEq)]
pub(crate) struct UiPrefsSnapshot {
    pub(crate) appearance_id: String,
    pub(crate) locale_id: String,
    pub(crate) terminology_id: String,
    pub(crate) driver_id: String,
    pub(crate) theme_id: String,
    pub(crate) ui_layout: String,
    pub(crate) custom_drivers: HashMap<String, OsUiDriver>,
    pub(crate) custom_themes: HashMap<String, String>,
    pub(crate) keybinding_overrides: HashMap<String, String>,
    pub(crate) worker_count: u32,
}

impl UiPrefsSnapshot {
    pub(crate) fn capture(chrome: &ChromeState, prefs: &PrefsState) -> Self {
        Self {
            appearance_id: prefs.appearance_id.clone(),
            locale_id: prefs.locale_id.clone(),
            terminology_id: prefs.terminology_id.clone(),
            driver_id: prefs.driver_id.clone(),
            theme_id: chrome.chrome_build.preferences.theme_id.clone(),
            ui_layout: chrome.chrome_build.preferences.ui_layout.clone(),
            custom_drivers: chrome.chrome_build.preferences.custom_drivers.clone(),
            custom_themes: chrome.chrome_build.preferences.custom_themes.clone(),
            keybinding_overrides: chrome.chrome_build.preferences.keybinding_overrides.clone(),
            worker_count: chrome.chrome_build.preferences.worker_count,
        }
    }
}

/// 🌓️ The persisted appearance this shell's OWN preference store answered, or `None` when it holds
/// no `setAppearance` event — React's `preferences.appearance`, before its `?? "system"` fallback,
/// so [`resolve_appearance_id`] can slot the host's read in between.
pub(crate) fn appearance_id(value: Option<OsUiAppearance>) -> Option<String> {
    match value? {
        OsUiAppearance::Light => Some("light".to_string()),
        OsUiAppearance::Dark => Some("dark".to_string()),
        OsUiAppearance::System => Some("system".to_string()),
    }
}

//#endregion 💾️PrefsSync
//#endregion ShellChrome


