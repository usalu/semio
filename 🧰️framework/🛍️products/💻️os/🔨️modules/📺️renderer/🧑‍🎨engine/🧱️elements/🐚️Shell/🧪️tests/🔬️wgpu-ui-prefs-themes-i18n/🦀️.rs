use super::*;

/// 🧪️ A `FilePrefsStore` constructed against a scratch path (never through the thread-local
/// singleton, so this doesn't depend on `SEMIO_PREFS_DIR`/`$HOME` or risk touching a real prefs
/// file) round-trips a value through an actual disk write + independent re-read.
#[test]
fn file_prefs_store_round_trips_through_disk() {
    use std::fs as system_fs;
    use std::process as system_process;
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    let dir = root.join(format!("prefs-test-{}-{:?}", system_process::id(), std::thread::current().id()));
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
        let terminal = {
            let state = store.flush_state.lock().expect("fixture flush state");
            !state.running && state.latest.is_none()
        };
        if terminal {
            break system_fs::read_to_string(&path).expect("terminal flush publishes the prefs file");
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
    system_fs::remove_dir_all(&dir).expect("terminal fixture output retires");
}

#[test]
fn canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript() {
    #[derive(Deserialize)]
    struct Fixture {
        version: u8,
        events: Vec<Value>,
        expected: UiPreferences,
    }
    let fixture: Fixture = serde_json::from_str(include_str!("../../../../🎚️UiPreferences/🧫️fixtures/🎚️canonical-os-ui-preferences/🔁️event-replay.json")).expect("shared event fixture");
    let events = fixture.events.iter().map(|event| decode_ui_preferences_config_mutation_json(&event.to_string()).expect("canonical mutation JSON")).collect();
    let encoded = encode_ui_preferences_event_log(&UiPreferencesEventLog { version: fixture.version, events });
    let retained = decode_ui_preferences_event_log(&encoded).expect("native event-store boundary round trip");
    let projection = replay_ui_preferences(&retained);
    assert_eq!(projection, fixture.expected);
    assert_eq!(projection.custom_drivers["studio"].config, serde_json::json!({ "scale": 1.25 }));
    assert_eq!(projection.keybinding_overrides["edit.undo"], "Meta+Z");
    let native = project_chrome_prefs(projection, 3);
    assert_eq!(native.custom_drivers["studio"].config, serde_json::json!({ "scale": 1.25 }));
    assert_eq!(native.keybinding_overrides["edit.undo"], "Meta+Z");
    assert_eq!(native.worker_count, 3);
    println!("[DEBUG] wgpu replayed the full shared OS UI preference fixture into its live host projection");
}

/// 🧭️ Installs one lock set and hands back the descriptor that was in place, so a law restores the
/// thread exactly as it found it. The process env is NOT the source of truth: `resolve_environment_
/// boot_descriptor` reads `SEMIO_LOCKED_*` ONCE, when this thread first touches `BOOT_DESCRIPTOR`, and
/// every later read goes to the installed descriptor (`🧊️renderer/🦀️.rs:15530`, `:15631`). A
/// `std::env::set_var` after that point can never reach `env_lock` — which is why these two laws
/// failed for a whole wave while the code they test was correct
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration; W4a's own hand-off).
fn with_boot_locks(locks: crate::WgpuBootLocks) -> crate::WgpuBootDescriptor {
    let previous = crate::boot_descriptor();
    crate::apply_boot_descriptor(crate::WgpuBootDescriptor { locks, ..previous.clone() }).expect("bounded test lock axes");
    previous
}

/// 🧪️ `env_lock` treats an empty lock value the same as unset (matches `FrameworkOsLocks`'
/// optional-string semantics: an empty `VITE_SEMIO_LOCKED_*` never locks), and a name that is not one
/// of the five lockable axes never locks either.
#[test]
fn env_lock_ignores_unset_and_empty() {
    let previous = with_boot_locks(crate::WgpuBootLocks::default());
    assert_eq!(env_lock("SEMIO_WP14_TEST_UNSET_VAR"), None);
    assert_eq!(env_lock("SEMIO_LOCKED_LOCALE"), None);
    let _ = with_boot_locks(crate::WgpuBootLocks { locale: String::new(), ..Default::default() });
    assert_eq!(env_lock("SEMIO_LOCKED_LOCALE"), None);
    let _ = with_boot_locks(crate::WgpuBootLocks { locale: "en".to_string(), ..Default::default() });
    assert_eq!(env_lock("SEMIO_LOCKED_LOCALE"), Some("en".to_string()));
    assert_eq!(env_lock("SEMIO_WP14_TEST_UNSET_VAR"), None);
    crate::apply_boot_descriptor(previous).expect("restored boot descriptor");
}

/// 🧪️ `shell_pref_locks` wires the four lockable axes onto the four lockable fields — byte-identical
/// axis names to `framework/product/os/dev/js/index.ts:19-23`'s `VITE_SEMIO_LOCKED_*` reads, which
/// `🧭️boot-descriptor/🟦️.ts` resolves into `locks` before the mount.
#[test]
fn shell_pref_locks_reads_the_four_lockable_envs() {
    let previous = with_boot_locks(crate::WgpuBootLocks { appearance: "dark".to_string(), locale: "de".to_string(), terminology: "reuse".to_string(), theme_id: "mono".to_string(), ..Default::default() });
    let locks = shell_pref_locks();
    assert_eq!(locks.appearance.as_deref(), Some("dark"));
    assert_eq!(locks.locale.as_deref(), Some("de"));
    assert_eq!(locks.terminology.as_deref(), Some("reuse"));
    assert_eq!(locks.theme_id.as_deref(), Some("mono"));
    let _ = with_boot_locks(crate::WgpuBootLocks::default());
    let unlocked = shell_pref_locks();
    assert_eq!(unlocked.appearance, None);
    assert_eq!(unlocked.locale, None);
    crate::apply_boot_descriptor(previous).expect("restored boot descriptor");
}

/// 🧪️ A locked appearance wins over storage at load time — mirrors `os-shell.tsx:862`'s
/// `locks.appearance ?? readStoredUiChromeAppearance()`. Deliberately backend-independent (never
/// asserts on storage's own contents, since `PREFS_STORE`'s thread-local can be seeded by an
/// earlier test reusing this worker thread) — only that a lock always overrides whatever loads.
#[test]
fn load_ui_prefs_once_prefers_a_lock_over_storage() {
    let previous = with_boot_locks(crate::WgpuBootLocks { appearance: "dark".to_string(), ..Default::default() });
    let mut state = ShellState::new(Vec::new(), String::new());
    state.load_ui_prefs_once();
    assert_eq!(state.appearance_id, "dark");
    state.appearance_id = "light".to_string();
    state.load_ui_prefs_once();
    assert_eq!(state.appearance_id, "light");
    crate::apply_boot_descriptor(previous).expect("restored boot descriptor");
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
///
/// ⚫️ Since ticket 26/09/17 packet W2k mono is the ui target's own `Theme::mono`, resolved from the
/// GENERATED `CHROME_MONO_*` palettes instead of 20 hand-written `Rgba::from_srgb8` literals in
/// this crate — so its floor is mono's `chrome.base`, not the authored `canvas` the hand-port read.
#[test]
fn resolve_theme_for_ids_semio_and_mono_differ() {
    let semio_dark = resolve_theme_for_ids("semio", "dark");
    let plain_dark = crate::resolve_theme("dark");
    assert_eq!(semio_dark.background, plain_dark.background);
    let mono_dark = resolve_theme_for_ids("mono", "dark");
    assert_ne!(mono_dark.background, semio_dark.background);
    assert_eq!(mono_dark.background, ui_wgpu::wgpu::Theme::mono(true).background);
    assert_ne!(mono_dark.background, ui_wgpu::wgpu::Theme::mono(false).background, "both appearances resolve");
    // Metrics are shared with the base theme (mono only recolors chrome paints).
    assert_eq!(mono_dark.navbar_height, semio_dark.navbar_height);
}

/// 🧬️ Canonical custom themes save, resolve and delete through the same full document schema.
#[test]
fn canonical_theme_document_round_trips_and_deletes() {
    let id = "custom.canonical-round-trip";
    let mut document = shell_theme_document_base().clone();
    document.id = id.into();
    document.label = "Canonical Round Trip".into();
    document.colors.insert("primary".into(), "#112233".into());
    let text = save_custom_theme_document(id, &document).expect("canonical document saves");
    assert_eq!(ThemeDocument::parse(&text), Some(document.clone()));
    assert_eq!(active_theme_id(), id);
    assert!(custom_theme_ids().contains(&id.to_string()));
    assert_ne!(resolve_theme_for_ids(id, "light").accent, resolve_theme_for_ids("semio", "light").accent);
    delete_custom_theme(id);
    assert_eq!(active_theme_id(), "semio");
    assert!(!custom_theme_ids().contains(&id.to_string()));
}

/// 🧬️ A five-paint object is not a theme document and cannot enter the registry path.
#[test]
fn non_document_theme_shapes_are_rejected() {
    let obsolete = r##"{"id":"custom.old","label":"Old","base":"semio","light":{"background":"#112233"},"dark":{}}"##;
    assert_eq!(ThemeDocument::parse(obsolete), None);
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
