//! ⏳️ `shell-async` staging of `storage-prefs-and-i18n`: RENDERER-INTEGRATION-2, the items the direction graph homes in this crate (they were in a higher crate or in the old chunk file).

use super::*;

//#region ShellChrome
//#region 💾️PrefsSync
/// 🌱️ The shell's two language axes from their sources, in React's order (`🏛️ShellHost/🟦️.tsx`: locks, then
/// `resolveUiPreferences`, then its seeds): a lock wins, then the stored preference, then — on a first visit, with an empty
/// profile — the seed. The locale's seed is the host's own language read (the page negotiates it from the browser's
/// languages; no language is assumed here, so a locale nobody named refuses); the terminology's seed is React's
/// `UI_TERMINOLOGY_NATIVE`, which is no language and always answers, so a fresh profile boots. A stored terminology this
/// build does not know falls back to the seed; an unsupported lock is a configuration fault and refuses.
pub fn resolve_language_axes(locked_locale: Option<String>, locked_terminology: Option<String>, preferences: &UiPreferences, host_locale: Option<&str>) -> Result<(Locale, Terminology), String> {
    let locale_id = locked_locale
        .or_else(|| preferences.locale.map(|locale| match locale { OsUiLocale::En => "en".to_string(), OsUiLocale::De => "de".to_string() }))
        .or_else(|| host_locale.map(ToOwned::to_owned))
        .ok_or_else(|| "missing explicit shell locale authority".to_string())?;
    let terminology_id = locked_terminology.or_else(|| preferences.terminology.clone().filter(|stored| Terminology::parse(stored).is_some())).unwrap_or_else(|| Terminology::Native.as_str().to_string());
    let locale = Locale::parse(&locale_id).ok_or_else(|| "unsupported shell locale authority".to_string())?;
    let terminology = Terminology::parse(&terminology_id).ok_or_else(|| "unsupported shell terminology authority".to_string())?;
    Ok((locale, terminology))
}
//#endregion 💾️PrefsSync
//#endregion ShellChrome

//#region ShellChrome
//#region 🗄️PrefsStore
/// 🗄️ Cross-platform key-value persistence for uiPrefs, over React's OWN key spellings. The native
/// backend is a bounded file page per key; the browser backend is the page-serviced [`🚪️StorageDoor`]
/// below. No new dependency either way (`serde_json` is already a dep) — zero-touch across
/// devcontainer/win/mac/linux.
///
/// 🩸️ What the browser backend replaces: a `WebLocalStorage` shim that resolved `localStorage` through
/// `web_sys::window()`. The wgpu shell runs inside the frame Worker, whose realm owns NO `window`, so
/// the shim silently resolved to `None` and EVERY `prefs_get`/`prefs_set` on the browser build was a
/// no-op — appearance, locale, terminology, themes, custom drivers, keybinding overrides, the compute
/// worker count, the dock skeleton and `ui.introduction.seen.*` all read as empty and written to
/// nothing (`📓️w4a-boot-appearance-and-tour.md` §6, hand-off 1).
#[cfg(any(test, feature = "testkit"))]
pub trait PrefsStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
}

#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
#[derive(Default)]
pub struct FilePrefsFlushState {
    pub latest: Option<HashMap<String, String>>,
    pub running: bool,
}
//#endregion 🗄️PrefsStore
//#endregion ShellChrome

//#region ShellChrome
//#region 🚪️StorageDoor
/// 📏️ Ceiling on the whole boot snapshot — it is structured-cloned into this isolate before the first
/// frame, so it is priced like any other boot payload instead of growing with the origin's store.
pub const HOST_STORAGE_SNAPSHOT_MAX_BYTES: usize = 128 * 1024;
//#endregion 🚪️StorageDoor
//#endregion ShellChrome
