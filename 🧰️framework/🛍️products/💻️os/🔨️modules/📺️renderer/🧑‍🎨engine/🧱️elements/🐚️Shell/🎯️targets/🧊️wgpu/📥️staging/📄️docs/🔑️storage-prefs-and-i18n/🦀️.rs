//! 📄️ `shell-docs` staging of `storage-prefs-and-i18n`: RENDERER-INTEGRATION-2, the items the direction graph homes in this crate (they were in a higher crate or in the old chunk file).

use super::*;

//#region ShellChrome
//#region 🎨️ThemeRegistry
/// 🎨️ Resolves built-ins and canonical custom theme documents through one format.
pub fn resolve_theme_for_ids(theme_id: &str, appearance_id: &str) -> Theme {
    if let Some(theme) = document_theme_for_ids(theme_id, semio_framework_os_renderer_wgpu_host_base::appearance_is_dark(appearance_id)) {
        return theme;
    }
    match theme_id {
        "" | "semio" => semio_framework_os_renderer_wgpu_host_base::resolve_theme(appearance_id),
        "mono" => Theme::mono(semio_framework_os_renderer_wgpu_host_base::appearance_is_dark(appearance_id)),
        _ => semio_framework_os_renderer_wgpu_host_base::resolve_theme(appearance_id),
    }
}
//#endregion 🎨️ThemeRegistry
//#endregion ShellChrome
