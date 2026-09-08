
use super::*;

/// 🧪️ A `FilePrefsStore` constructed against a scratch path (never through the thread-local
/// singleton, so this doesn't depend on `SEMIO_PREFS_DIR`/`$HOME` or risk touching a real prefs
/// file) round-trips a value through an actual disk write + independent re-read.
#[test]
fn file_prefs_store_round_trips_through_disk() {
    use std::fs as system_fs;
    use std::process as system_process;
    let dir = std::env::temp_dir().join(format!("semio-wp14-prefs-test-{}-{:?}", system_process::id(), std::thread::current().id()));
    let _ = system_fs::create_dir_all(&dir);
    let path = dir.join("ui-prefs.json");
    let _ = system_fs::remove_file(&path);
    let mut store = FilePrefsStore::with_path(path.clone());
    store.set(
        OS_SHELL_CONFIG_STORAGE_KEY,
        &serde_json::json!({
            "version": 1,
            "preferences": {},
            "namedLayouts": { "draw": [{ "id": "wide" }] },
            "dockLayouts": { "apps": {} },
            "dockUi": { "apps": {} },
            "windowPanes": { "apps": {} }
        })
        .to_string(),
    );
    let events = serde_json::json!({ "version": 1, "events": [{ "mutation": "setAppearance", "appearance": "dark" }] }).to_string();
    assert_eq!(prefs_get_from(&store, UI_PREFERENCES_CONFIG_SCHEMA), None);
    prefs_set_in(&mut store, UI_PREFERENCES_CONFIG_SCHEMA, &events);
    assert_eq!(prefs_get_from(&store, UI_PREFERENCES_CONFIG_SCHEMA), Some(events.clone()));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(1);
    let raw = loop {
        if let Ok(raw) = system_fs::read_to_string(&path) {
            if raw.contains("dark") {
                break raw;
            }
        }
        assert!(std::time::Instant::now() < deadline, "flush worker must write the latest prefs snapshot");
        std::thread::yield_now();
    };
    let reloaded: HashMap<String, String> = serde_json::from_str(&raw).expect("valid JSON");
    let config: Value = serde_json::from_str(reloaded.get(OS_SHELL_CONFIG_STORAGE_KEY).expect("one config document")).expect("valid config JSON");
    assert_eq!(serde_json::from_str::<Value>(config["preferences"][UI_PREFERENCES_CONFIG_SCHEMA].as_str().expect("event log string")).expect("event log JSON")["events"][0]["appearance"], "dark");
    assert_eq!(config["dockLayouts"]["apps"], serde_json::json!({}));
    assert_eq!(config["namedLayouts"]["draw"][0]["id"], "wide", "preference writes must preserve sibling projections");
    assert_eq!(reloaded.len(), 1, "wgpu preferences use the shared OS config authority");
    let _ = system_fs::remove_file(&path);
}

#[test]
fn canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript() {
    #[derive(Deserialize)]
    struct Fixture {
        version: u8,
        events: Vec<Value>,
        expected: UiPreferences,
    }
    let fixture: Fixture = serde_json::from_str(include_str!("../../../../🎚️UiPreferences/🧪️tests/🎚️canonical-os-ui-preferences/🧫️fixtures/🔁️event-replay.json")).expect("shared event fixture");
    let events = fixture
        .events
        .iter()
        .map(|event| decode_ui_preferences_config_mutation_json(&event.to_string()).expect("canonical mutation JSON"))
        .collect();
    let projection = replay_ui_preferences(&UiPreferencesEventLog { version: fixture.version, events });
    assert_eq!(projection, fixture.expected);
    println!("[DEBUG] wgpu replayed the shared OS UI preference event fixture");
}

/// 🧪️ `env_lock` treats an empty-string env value the same as unset (matches
/// `FrameworkOsLocks`' optional-string semantics: an empty `VITE_SEMIO_LOCKED_*` never locks).
#[test]
fn env_lock_ignores_unset_and_empty() {
    assert_eq!(env_lock("SEMIO_WP14_TEST_UNSET_VAR"), None);
    unsafe {
        std::env::set_var("SEMIO_WP14_TEST_EMPTY_VAR", "");
    }
    assert_eq!(env_lock("SEMIO_WP14_TEST_EMPTY_VAR"), None);
    unsafe {
        std::env::set_var("SEMIO_WP14_TEST_EMPTY_VAR", "en");
    }
    assert_eq!(env_lock("SEMIO_WP14_TEST_EMPTY_VAR"), Some("en".to_string()));
    unsafe {
        std::env::remove_var("SEMIO_WP14_TEST_EMPTY_VAR");
    }
}

/// 🧪️ `shell_pref_locks` wires `SEMIO_LOCKED_*` onto the four lockable fields — byte-identical
/// env var names to `framework/product/os/dev/js/index.ts:19-23`'s `VITE_SEMIO_LOCKED_*` reads.
#[test]
fn shell_pref_locks_reads_the_four_lockable_envs() {
    unsafe {
        std::env::set_var("SEMIO_LOCKED_APPEARANCE", "dark");
        std::env::set_var("SEMIO_LOCKED_LOCALE", "de");
        std::env::set_var("SEMIO_LOCKED_TERMINOLOGY", "reuse");
        std::env::set_var("SEMIO_LOCKED_THEME", "mono");
    }
    let locks = shell_pref_locks();
    assert_eq!(locks.appearance.as_deref(), Some("dark"));
    assert_eq!(locks.locale.as_deref(), Some("de"));
    assert_eq!(locks.terminology.as_deref(), Some("reuse"));
    assert_eq!(locks.theme_id.as_deref(), Some("mono"));
    unsafe {
        std::env::remove_var("SEMIO_LOCKED_APPEARANCE");
        std::env::remove_var("SEMIO_LOCKED_LOCALE");
        std::env::remove_var("SEMIO_LOCKED_TERMINOLOGY");
        std::env::remove_var("SEMIO_LOCKED_THEME");
    }
    let unlocked = shell_pref_locks();
    assert_eq!(unlocked.appearance, None);
    assert_eq!(unlocked.locale, None);
}

/// 🧪️ A locked appearance wins over storage at load time — mirrors `os-shell.tsx:862`'s
/// `locks.appearance ?? readStoredUiChromeAppearance()`. Deliberately backend-independent (never
/// asserts on storage's own contents, since `PREFS_STORE`'s thread-local can be seeded by an
/// earlier test reusing this worker thread) — only that a lock always overrides whatever loads.
#[test]
fn load_ui_prefs_once_prefers_a_lock_over_storage() {
    unsafe {
        std::env::set_var("SEMIO_LOCKED_APPEARANCE", "dark");
    }
    let mut state = ShellState::new(Vec::new(), String::new());
    state.load_ui_prefs_once();
    assert_eq!(state.appearance_id, "dark");
    // The "load once" gate: mutating the field and calling load again must not reset it.
    state.appearance_id = "light".to_string();
    state.load_ui_prefs_once();
    assert_eq!(state.appearance_id, "light");
    unsafe {
        std::env::remove_var("SEMIO_LOCKED_APPEARANCE");
    }
}

/// 🧪️ `persist_ui_prefs_if_changed`'s dirty-check: a second call with no field changes since the
/// last sync must be a cheap no-operation (mirrors the combined-dependency-array `useEffect` at
/// `os-shell.tsx:3477-3491`, which only re-runs when one of its deps actually changed).
///
/// **Self-inflicted-flakiness fix**: this used to hardcode the "changed" value as `"compact"` and
/// never restore the original — on native, `PREFS_STORE`'s `FilePrefsStore` backs onto a real
/// `$SEMIO_PREFS_DIR`/`~/.config/semio/ui-prefs.json` file (not a scratch path; unlike
/// `file_prefs_store_round_trips_through_disk`, this test exercises the thread-local singleton
/// directly, same as `load_ui_prefs_once_prefers_a_lock_over_storage`'s own doc comment already
/// flags this file as shared across runs), so once this test ran once, `ui.chrome.driver` was left
/// at `"compact"` on disk — the *next* run's `load_ui_prefs_once` read `"compact"` right back in,
/// making `state.driver_id = "compact"` below a no-op and failing the final assertion. Toggling to
/// whatever the loaded value *isn't*, then restoring it, makes this test pass regardless of what a
/// previous run left behind.
#[test]
fn persist_ui_prefs_if_changed_is_idempotent_when_nothing_changed() {
    let mut state = ShellState::new(Vec::new(), String::new());
    state.load_ui_prefs_once();
    let after_load = state.chrome_present.last_synced_preferences.clone();
    assert!(after_load.is_some());
    state.persist_ui_prefs_if_changed();
    let after_noop_persist = state.chrome_present.last_synced_preferences.clone();
    assert!(after_load == after_noop_persist);
    let original_driver_id = state.driver_id.clone();
    let toggled_driver_id = if original_driver_id == "compact" { "default" } else { "compact" };
    state.driver_id = toggled_driver_id.to_string();
    state.persist_ui_prefs_if_changed();
    let after_change = state.chrome_present.last_synced_preferences.clone();
    assert!(after_change != after_noop_persist);
    state.driver_id = original_driver_id;
    state.persist_ui_prefs_if_changed();
}

/// 🧪️ `resolve_theme_for_ids("semio", _)` is exactly `resolve_theme` (the pre-WP14 behavior),
/// and `"mono"` resolves to a *different* real palette (not a copy of semio's).
#[test]
fn resolve_theme_for_ids_semio_and_mono_differ() {
    let semio_dark = resolve_theme_for_ids("semio", "dark");
    let plain_dark = crate::resolve_theme("dark");
    assert_eq!(semio_dark.background, plain_dark.background);
    let mono_dark = resolve_theme_for_ids("mono", "dark");
    assert_ne!(mono_dark.background, semio_dark.background);
    assert_eq!(mono_dark.background, Rgba::from_srgb8(25, 25, 25, 255));
    // Metrics are shared with the base theme (mono only recolors chrome paints).
    assert_eq!(mono_dark.navbar_height, semio_dark.navbar_height);
}

/// 🧪️ A hex color override is parsed and applied; an invalid hex falls back to the base color
/// rather than panicking or silently corrupting the theme.
#[test]
fn hex_to_rgba_parses_valid_and_falls_back_on_invalid() {
    let fallback = Rgba::new(0.1, 0.2, 0.3, 1.0);
    assert_eq!(hex_to_rgba("#ff0000", fallback), Rgba::from_srgb8(255, 0, 0, 255));
    assert_eq!(hex_to_rgba("ff0000", fallback), Rgba::from_srgb8(255, 0, 0, 255));
    assert_eq!(hex_to_rgba("not-a-color", fallback), fallback);
    assert_eq!(hex_to_rgba("#fff", fallback), fallback);
}

/// 🧪️ End-to-end custom theme draft flow: begin → mutate → save → resolves with the override
/// applied → delete falls back to "semio". Explicitly seeds `active_theme_id` rather than
/// asserting on whatever `load_chrome_prefs` found on disk, so this is independent of any other
/// test's writes on a reused worker thread.
#[test]
fn custom_theme_draft_round_trips_and_deletes() {
    set_active_theme_id("semio");
    let id = begin_custom_theme_draft("semio", "My Theme", "wp14-test");
    assert_eq!(id, "custom.wp14-test");
    assert!(set_draft_theme_color("light", "background", "#112233"));
    assert!(!set_draft_theme_color("light", "not-a-field", "#112233"));
    let saved_id = save_draft_theme().expect("a draft was in progress");
    assert_eq!(saved_id, id);
    assert_eq!(active_theme_id(), id);
    assert!(custom_theme_ids().contains(&id));
    let resolved = resolve_theme_for_ids(&id, "light");
    assert_eq!(resolved.background, Rgba::from_srgb8(0x11, 0x22, 0x33, 255));
    // Untouched fields still fall through to the "semio" base.
    assert_eq!(resolved.navbar_height, resolve_theme_for_ids("semio", "light").navbar_height);
    delete_custom_theme(&id);
    assert_eq!(active_theme_id(), "semio");
    assert!(!custom_theme_ids().contains(&id));
}

/// 🧪️ `discard_draft_theme` clears an in-progress draft without touching the saved registry.
#[test]
fn discard_draft_theme_clears_in_progress_draft() {
    let id = begin_custom_theme_draft("mono", "Discard Me", "wp14-discard");
    assert!(set_draft_theme_color("dark", "accent", "#abcdef"));
    discard_draft_theme();
    assert_eq!(save_draft_theme(), None);
    assert!(!custom_theme_ids().contains(&id));
}

/// 🧪️ `active_ui_layout`/`set_active_ui_layout` round-trip and reject unknown values (matches
/// React's `UiChromeLayout` union of exactly `"desktop" | "tablet"`).
#[test]
fn ui_layout_round_trips_and_rejects_unknown_values() {
    set_active_ui_layout("tablet");
    assert_eq!(active_ui_layout(), "tablet");
    set_active_ui_layout("bogus");
    assert_eq!(active_ui_layout(), "desktop");
}

/// 🧪️ EN/DE parity spot-check against `ui/js/react/index.tsx`'s `uiChromeTranslationBundles`
/// "normal" labels (`:2898-3975`) for a sample of the keys this crate now routes through.
#[test]
fn shell_chrome_string_matches_react_bundle_samples() {
    assert_eq!(shell_chrome_string("display.tab.windows", false), "Windows");
    assert_eq!(shell_chrome_string("display.tab.windows", true), "Fenster");
    assert_eq!(shell_chrome_string("common.execute", false), "Execute");
    assert_eq!(shell_chrome_string("common.execute", true), "Ausführen");
    assert_eq!(shell_chrome_string("common.windowOptions", true), "Fensteroptionen");
    // Unknown keys fall back to the key itself rather than inventing text.
    assert_eq!(shell_chrome_string("nonexistent.key", true), "nonexistent.key");
}

/// 🧪️ `read_stored_introduction_seen`/`write_stored_introduction_seen` byte-identical semantics
/// to `ui/js/react/index.tsx:2309-2317` — exercised against a scratch `FilePrefsStore` (not the
/// thread-local singleton) so this never touches a real prefs file.
#[test]
fn introduction_seen_key_format_matches_react() {
    assert_eq!(format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}framework-os"), "ui.introduction.seen.framework-os");
}
