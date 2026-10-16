//! 🎛️ `shell-prefs` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔑️StorageKeys
/// 🔑️ Byte-identical to `UI_COMPUTE_WORKER_COUNT_STORAGE_KEY` (`ui/js/react/index.tsx:2267`).
pub(crate) const UI_COMPUTE_WORKER_COUNT_STORAGE_KEY: &str = "ui.compute.workerCount";

//#endregion 🔑️StorageKeys
//#region 🗄️PrefsStore
/// 🗝️ One bounded top-level key, the shape every per-frame chrome reader uses. It is deliberately NOT
/// [`prefs_get`]: React keeps `ui.compute.workerCount` and `ui.introduction.seen.<appId>` as their own
/// flat entries and only `os.config.ui-preferences` inside `semio.os.config`'s `preferences` map, so a
/// renderer that nested them would answer nothing to a profile React had written.
pub(crate) fn stored_field_get(key: &str) -> Option<String> {
    if key.len() > SHELL_CHROME_IO_FIELD_BYTES {
        return None;
    }
    raw_prefs_get(key).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES)
}

/// 🗝️ The writer twin of [`stored_field_get`].
pub(crate) fn stored_field_set(key: &str, value: &str) {
    if key.len() <= SHELL_CHROME_IO_FIELD_BYTES && value.len() <= SHELL_CHROME_IO_FIELD_BYTES {
        raw_prefs_set(key, value);
    }
}

//#endregion 🗄️PrefsStore
//#region 🔒️PrefLocks
/// 🔒️ `SEMIO_LOCKED_*` env-driven pref locks — mirrors `os-shell.tsx`'s `FrameworkOsLocks`
/// (`:372-378`, read via `VITE_SEMIO_LOCKED_*` in `framework/product/os/dev/js/index.ts:19-23`):
/// appearance/locale/terminology/themeId may be locked; driver/layout/customThemes/customDrivers
/// deliberately stay unlocked in React too. A locked pref skips its localStorage write. Native-only
/// in practice (wasm32-unknown-unknown has no process env at runtime, so this is always empty
/// there — kiosk/demo locking is a native `semio-wgpu-native` deployment concern).
#[derive(Clone, Debug, Default)]
#[cfg(test)]
struct ShellPrefLocks {
    pub(crate) appearance: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) terminology: Option<String>,
    pub(crate) theme_id: Option<String>,
}

/// 🔒️ Resolves one `SEMIO_LOCKED_*` lock through the BOOT DESCRIPTOR rather than the process env.
/// The descriptor already carries the native env seeds (`resolve_environment_boot_descriptor`), so a
/// native deployment behaves exactly as before — and a BROWSER build, which has no process env at all
/// and was therefore unlockable, now locks from `<meta name="semio-locked-*">`/the library door's
/// `locks` option through the same one vocabulary React reads as `FrameworkOsLocks`.
pub(crate) fn env_lock(name: &str) -> Option<String> {
    let locks = crate::boot_locks();
    let value = match name {
        "SEMIO_LOCKED_APPEARANCE" => locks.appearance,
        "SEMIO_LOCKED_LOCALE" => locks.locale,
        "SEMIO_LOCKED_TERMINOLOGY" => locks.terminology,
        "SEMIO_LOCKED_THEME" => locks.theme_id,
        "SEMIO_LOCKED_EXAMPLE" => locks.example_id,
        _ => String::new(),
    };
    (!value.is_empty()).then_some(value)
}

#[cfg(test)]
pub(super) fn shell_pref_locks() -> ShellPrefLocks {
    ShellPrefLocks { appearance: env_lock("SEMIO_LOCKED_APPEARANCE"), locale: env_lock("SEMIO_LOCKED_LOCALE"), terminology: env_lock("SEMIO_LOCKED_TERMINOLOGY"), theme_id: env_lock("SEMIO_LOCKED_THEME") }
}

//#endregion 🔒️PrefLocks
//#region 🎨️ThemeRegistry
#[cfg(not(target_arch = "wasm32"))]
pub(super) static CHROME_PREFS: crate::element_contracts::WorkerCell<Option<ChromePrefsState>> = crate::element_contracts::WorkerCell::new(Default::default);

pub(crate) fn default_compute_worker_count() -> u32 {
    std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(1)
}

pub(super) fn project_chrome_prefs(preferences: UiPreferences, worker_count: u32) -> ChromePrefsState {
    let custom_themes = custom_themes_from(&preferences);
    let custom_drivers = preferences.custom_drivers;
    let ui_layout = match preferences.layout {
        Some(OsUiChromeLayout::Tablet) => "tablet",
        _ => "desktop",
    }
    .to_string();
    let theme_id = preferences.theme_id.unwrap_or_else(|| "semio".to_string());
    let keybinding_overrides = preferences.keybinding_overrides;
    ChromePrefsState { ui_layout, theme_id, custom_drivers, custom_themes, keybinding_overrides, theme_document_draft: None, theme_document_generation: 0, theme_document_resolved: None, worker_count }
}

fn load_chrome_prefs() -> ChromePrefsState {
    let worker_count = stored_field_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()).filter(|count| *count >= 1).unwrap_or_else(default_compute_worker_count);
    project_chrome_prefs(read_ui_preferences(), worker_count)
}

pub(crate) fn with_chrome_prefs<R>(f: impl FnOnce(&mut ChromePrefsState) -> R) -> R {
    CHROME_PREFS.with(|cell| {
        let mut guard = cell.borrow_mut();
        if guard.is_none() {
            *guard = Some(load_chrome_prefs());
        }
        f(guard.as_mut().expect("chrome prefs just initialized"))
    })
}

pub(crate) fn set_active_theme_id(id: &str) {
    with_chrome_prefs(|prefs| prefs.theme_id = id.to_string());
}

//#endregion 🎨️ThemeRegistry
//#region 🗣️ChromeI18n

//#endregion 🗣️ChromeI18n
//#region 💾️PrefsSync
#[derive(Clone, Debug, Default)]
pub(super) struct UiPreferencesEventLog {
    pub(crate) version: u8,
    pub(crate) events: Vec<UiPreferencesConfigMutation>,
}

/// 🚫️ A stored preference log this renderer cannot read whole — unparsable, another version, or carrying an event
/// it does not know (one a newer renderer wrote). Typed, never read as an empty log: it refuses every write, since a
/// rewrite from this renderer's own events would destroy what it could not read, while reads replay `readable`, the
/// events it did decode.
#[derive(Clone, Debug)]
struct UiPreferencesLogRefusal {
    pub(crate) reason: &'static str,
    pub(super) event_index: Option<usize>,
    pub(super) readable: UiPreferencesEventLog,
}

pub(super) fn decode_ui_preferences_event_log(raw: &str) -> Result<UiPreferencesEventLog, UiPreferencesLogRefusal> {
    let refuse = |reason: &'static str, event_index: Option<usize>, events: Vec<UiPreferencesConfigMutation>| UiPreferencesLogRefusal { reason, event_index, readable: UiPreferencesEventLog { version: 1, events } };
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return Err(refuse("json", None, Vec::new()));
    };
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        return Err(refuse("version", None, Vec::new()));
    }
    let Some(stored) = value.get("events").and_then(Value::as_array) else {
        return Err(refuse("events", None, Vec::new()));
    };
    let mut events = Vec::with_capacity(stored.len());
    let mut undecodable = None;
    for (index, event) in stored.iter().enumerate() {
        match decode_ui_preferences_config_mutation_json(&event.to_string()) {
            Ok(mutation) => events.push(mutation),
            Err(_) => {
                undecodable.get_or_insert(index);
            }
        }
    }
    match undecodable {
        None => Ok(UiPreferencesEventLog { version: 1, events }),
        Some(index) => Err(refuse("event", Some(index), events)),
    }
}

pub(super) fn encode_ui_preferences_event_log(log: &UiPreferencesEventLog) -> String {
    let events: Vec<Value> = log.events.iter().map(|event| dsl_value_as_json(&semio_framework_value::ToValue::to_value(event))).collect();
    serde_json::json!({ "version": log.version, "events": events }).to_string()
}

/// ✍️ The ONE write rule over a stored preference log: an absent log is an empty one, a stored one is decoded
/// whole or the write refuses (the stored text is never replaced by a log this renderer could not read); `append`
/// adds this write's events, and a write that added none stores nothing.
pub(super) fn ui_preferences_log_write(stored: Option<&str>, append: impl FnOnce(&mut UiPreferencesEventLog)) -> Result<Option<String>, UiPreferencesLogRefusal> {
    let mut log = match stored {
        None => UiPreferencesEventLog { version: 1, events: Vec::new() },
        Some(raw) => decode_ui_preferences_event_log(raw)?,
    };
    let before = log.events.len();
    append(&mut log);
    Ok((log.events.len() > before).then(|| encode_ui_preferences_event_log(&log)))
}

/// 💾️ Commits one preference write through [`ui_preferences_log_write`]: stored, or one refusal line and the stored
/// log left exactly as it was.
pub(crate) fn commit_ui_preferences_write(append: impl FnOnce(&mut UiPreferencesEventLog)) {
    match ui_preferences_log_write(prefs_get(UI_PREFERENCES_CONFIG_SCHEMA).as_deref(), append) {
        Ok(Some(encoded)) => prefs_set(UI_PREFERENCES_CONFIG_SCHEMA, &encoded),
        Ok(None) => {}
        Err(refusal) => debug_log(&format!("[wgpu-shell] preference write refused: the stored log is unreadable ({}{}) and stays untouched", refusal.reason, refusal.event_index.map(|index| format!(" at event {index}")).unwrap_or_default())),
    }
}

/// 📖️ The stored log for READING: an unreadable one still answers the events this renderer decoded.
fn read_ui_preferences_event_log() -> UiPreferencesEventLog {
    match prefs_get(UI_PREFERENCES_CONFIG_SCHEMA) {
        None => UiPreferencesEventLog { version: 1, events: Vec::new() },
        Some(raw) => decode_ui_preferences_event_log(&raw).unwrap_or_else(|refusal| refusal.readable),
    }
}

pub(crate) fn replay_ui_preferences(log: &UiPreferencesEventLog) -> UiPreferences {
    let mut preferences = UiPreferences::default();
    for mutation in &log.events {
        apply_ui_preferences_config_mutation(&mut preferences, mutation).expect("canonical OS UI-preferences mutation must apply");
    }
    preferences
}

pub(crate) fn read_ui_preferences() -> UiPreferences {
    replay_ui_preferences(&read_ui_preferences_event_log())
}

/// 🌓️ The ONE appearance-resolution law, mirroring `🏛️ShellHost/🟦️.tsx:2174`'s
/// `locks.appearance ?? resolveUiPreferences(preferences, { appearance: "system", … }).appearance`
/// term for term — a lock wins, then this process's own preference store, then the HOST's read of
/// that same store, then React's own `"system"` seed.
///
/// The host term is the page's own replay of that same log, published through
/// `semioWgpuSetHostAppearance`. It is a fallback and never an override, and since the
/// [`🚪️StorageDoor`] seeds this isolate's store before the shell boots, this process's own term now
/// answers on the browser too — the host term survives for the embeddable door and for a realm whose
/// snapshot was refused.
pub(crate) fn resolve_appearance_id(preferences: &UiPreferences) -> String {
    env_lock("SEMIO_LOCKED_APPEARANCE").or_else(|| appearance_id(preferences.appearance)).or_else(|| crate::host_appearance_preference().map(ToOwned::to_owned)).unwrap_or_else(|| "system".to_string())
}

pub(crate) fn resolve_locale_id(lock: Option<String>, preferences: &UiPreferences, selected: Locale) -> String {
    lock.or_else(|| {
        preferences.locale.map(|locale| match locale {
            OsUiLocale::De => "de".to_string(),
            OsUiLocale::En => "en".to_string(),
        })
    })
    .or_else(|| crate::host_locale().map(ToOwned::to_owned))
    .unwrap_or_else(|| selected.as_str().to_string())
}

pub(crate) fn custom_themes_from(preferences: &UiPreferences) -> HashMap<String, String> {
    preferences
        .custom_themes
        .iter()
        .filter_map(|(id, theme)| {
            let mut config = theme.config.as_object()?.to_vec();
            config.retain(|(key,_)|key!="id"&&key!="label");
            config.push(("id".to_owned(),DslValue::String(theme.theme_id.clone())));
            config.push(("label".to_owned(),DslValue::String(theme.label.clone())));
            Some((id.clone(),semio_framework_pack_json::to_json_string(&DslValue::Object(config))))
        })
        .collect()
}

fn canonical_custom_themes(preferences: &ChromePrefsState) -> HashMap<String, OsUiTheme> {
    preferences
        .custom_themes
        .iter()
        .filter_map(|(id, raw)| {
            let document = ThemeDocument::parse(raw)?;
            let label = document.label.clone();
            let DslValue::Object(mut entries)=document.to_value() else{return None};
            entries.retain(|(key,_)|key!="id"&&key!="label");
            let config=DslValue::Object(entries);
            Some((id.clone(), OsUiTheme { theme_id: id.clone(), label, config }))
        })
        .collect()
}

pub(crate) fn persist_ui_preferences(chrome: &ChromeState, prefs: &PrefsState, previous: Option<&UiPrefsSnapshot>) {
    commit_ui_preferences_write(|log| {
        let changed = |current: &str, prior: fn(&UiPrefsSnapshot) -> &str| previous.is_none_or(|snapshot| current != prior(snapshot));
        if env_lock("SEMIO_LOCKED_APPEARANCE").is_none() && changed(&prefs.appearance_id, |snapshot| &snapshot.appearance_id) {
            log.events.push(set_appearance(Some(match prefs.appearance_id.as_str() {
                "light" => OsUiAppearance::Light,
                "dark" => OsUiAppearance::Dark,
                _ => OsUiAppearance::System,
            })));
        }
        if changed(&chrome.chrome_build.preferences.ui_layout, |snapshot| &snapshot.ui_layout) {
            log.events.push(set_layout(Some(if chrome.chrome_build.preferences.ui_layout == "tablet" { OsUiChromeLayout::Tablet } else { OsUiChromeLayout::Desktop })));
        }
        if changed(&prefs.driver_id, |snapshot| &snapshot.driver_id) {
            log.events.push(set_driver(Some(prefs.driver_id.clone())));
        }
        if previous.is_none_or(|snapshot| snapshot.custom_drivers != chrome.chrome_build.preferences.custom_drivers) {
            let before = previous.map(|snapshot| &snapshot.custom_drivers).cloned().unwrap_or_default();
            let after = &chrome.chrome_build.preferences.custom_drivers;
            let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
            let projected = replay_ui_preferences(&log);
            for id in ids {
                let next = after.get(&id).cloned();
                if projected.custom_drivers.get(&id) != next.as_ref() {
                    log.events.push(set_custom_driver(id, next));
                }
            }
        }
        if env_lock("SEMIO_LOCKED_LOCALE").is_none() && changed(&prefs.locale_id, |snapshot| &snapshot.locale_id) {
            log.events.push(set_locale(Some(if prefs.locale_id == "de" { OsUiLocale::De } else { OsUiLocale::En })));
        }
        if env_lock("SEMIO_LOCKED_TERMINOLOGY").is_none() && changed(&prefs.terminology_id, |snapshot| &snapshot.terminology_id) {
            log.events.push(set_terminology(Some(prefs.terminology_id.clone())));
        }
        if env_lock("SEMIO_LOCKED_THEME").is_none() && changed(&chrome.chrome_build.preferences.theme_id, |snapshot| &snapshot.theme_id) {
            log.events.push(set_theme(Some(chrome.chrome_build.preferences.theme_id.clone())));
        }
        if previous.is_none_or(|snapshot| snapshot.custom_themes != chrome.chrome_build.preferences.custom_themes) {
            let before = previous.map(|snapshot| &snapshot.custom_themes).cloned().unwrap_or_default();
            let after = canonical_custom_themes(&chrome.chrome_build.preferences);
            let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
            let projected = replay_ui_preferences(&log);
            for id in ids {
                let next = after.get(&id).cloned();
                if projected.custom_themes.get(&id) != next.as_ref() {
                    log.events.push(set_custom_theme(id, next));
                }
            }
        }
        if previous.is_none_or(|snapshot| snapshot.keybinding_overrides != chrome.chrome_build.preferences.keybinding_overrides) {
            let before = previous.map(|snapshot| &snapshot.keybinding_overrides).cloned().unwrap_or_default();
            let after = &chrome.chrome_build.preferences.keybinding_overrides;
            let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
            let projected = replay_ui_preferences(&log);
            for id in ids {
                let next = after.get(&id).cloned();
                if projected.keybinding_overrides.get(&id) != next.as_ref() {
                    log.events.push(set_keybinding_override(id, next));
                }
            }
        }
    });
}

impl PrefsState {
    /// 💾️ Loads persisted uiPrefs into `self` exactly once per process, mirroring `os-shell.tsx`'s
    /// boot-time `locks.appearance ?? readStoredUiChromeAppearance()` fallback chain (`:862-868`).
    /// A locked pref (`SEMIO_LOCKED_*`) wins over storage, matching `resolveShellLocks`.
    #[cfg(test)]
    pub(super) fn load_ui_prefs_once(&mut self, chrome: &mut ChromeState) {
        if chrome.chrome_present.preferences_loaded {
            return;
        }
        chrome.chrome_present.preferences_loaded = true;
        let locks = shell_pref_locks();
        let preferences = read_ui_preferences();
        self.appearance_id = resolve_appearance_id(&preferences);
        self.locale_id = resolve_locale_id(locks.locale.clone(), &preferences, self.active_locale());
        crate::interpreter::install_ui_engine_locale(self.active_locale());
        self.terminology_id = locks.terminology.clone().or(preferences.terminology).unwrap_or_else(|| self.active_terminology().as_str().to_string());
        self.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
        chrome.chrome_build.preferences = with_chrome_prefs(|preferences| preferences.clone());
        chrome.chrome_build.driver = resolve_ui_driver_chrome(&self.driver_id, &chrome.chrome_build.preferences.custom_drivers);
        if let Some(locked_theme) = &locks.theme_id {
            set_active_theme_id(locked_theme);
            chrome.chrome_build.preferences.theme_id = locked_theme.clone();
        }
        chrome.chrome_present.last_synced_preferences = Some(UiPrefsSnapshot::capture(chrome, self));
    }

    /// 💾️ Writes any changed uiPrefs field to the store (skipping locked ones), mirroring
    /// `os-shell.tsx`'s persistence `useEffect` (`:3477-3491`): one combined-dependency effect that
    /// rewrites every non-locked pref whenever any of them changes, not a per-field diff.
    #[cfg(test)]
    pub(super) fn persist_ui_prefs_if_changed(&mut self, chrome: &mut ChromeState) {
        let snapshot = UiPrefsSnapshot::capture(chrome, self);
        if chrome.chrome_present.last_synced_preferences.as_ref() == Some(&snapshot) {
            return;
        }
        persist_ui_preferences(chrome, self, chrome.chrome_present.last_synced_preferences.as_ref());
        stored_field_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &snapshot.worker_count.to_string());
        chrome.chrome_present.last_synced_preferences = Some(snapshot);
    }
}

//#endregion 💾️PrefsSync
//#endregion ShellChrome


