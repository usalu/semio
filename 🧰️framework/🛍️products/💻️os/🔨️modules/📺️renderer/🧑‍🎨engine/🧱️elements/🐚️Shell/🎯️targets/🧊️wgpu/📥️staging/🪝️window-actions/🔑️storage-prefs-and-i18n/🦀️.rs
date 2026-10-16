//! 🪝 `shell-window-actions` staging of `storage-prefs-and-i18n`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
//#region 🎨️ThemeRegistry
pub fn delete_custom_theme(id: &str) {
    with_chrome_prefs(|prefs| {
        prefs.custom_themes.remove(id);
        if prefs.theme_id == id {
            prefs.theme_id = "semio".to_string();
        }
    });
}
//#endregion 🎨️ThemeRegistry
//#endregion ShellChrome
