//! 🗄️ Packet W5a's standing laws: every durable preference the wgpu shell reads or writes in a browser
//! crosses the page realm, through the SAME keys and value encodings React's own `StoragePort` uses.
//!
//! 🩸️ The defect they pin: `WebLocalStorage` resolved `localStorage` through `web_sys::window()`, and
//! the wgpu shell runs inside the frame Worker, whose realm owns no `window`. Every `prefs_get` on the
//! browser build therefore answered nothing and every `prefs_set` wrote to nothing — appearance,
//! locale, terminology, themes, custom drivers, keybinding overrides, the compute worker count, the
//! dock skeleton and `ui.introduction.seen.<appId>` all silently. A tour could be dismissed once per
//! LOAD and came back on every reload (`📓️w4a-boot-appearance-and-tour.md` §5 item 3, hand-off 1).
//!
//! Some laws read SOURCE rather than running code, for the reason `🧭️boot-axis-parity/🦀️.rs` does:
//! what they assert is that a `cfg(target_arch = "wasm32")` path exists and is wired to the page's
//! three artifacts, and that code never compiles into a native test binary.

use super::*;

/// 🌳️ `…/🧑‍🎨engine`, the root every source-reading law resolves against.
fn engine_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..").canonicalize().expect("engine root")
}

/// 🌳️ The workspace root — React's own key constants live outside this product.
fn repo_root() -> std::path::PathBuf {
    engine_root().join("../../../../../..").canonicalize().expect("repo root")
}

fn read_source(path: std::path::PathBuf) -> String {
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn wgpu_shell_source() -> String {
    read_source(engine_root().join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"))
}

fn wgpu_target_source(relative: &str) -> String {
    read_source(engine_root().join("🎯️targets/🧊️wgpu").join(relative))
}

/// 🧪️ The in-memory [`PrefsStore`] the document-encoding law drives, so it asserts the `semio.os.config`
/// SHAPE without depending on a real file store or a real browser.
#[derive(Default)]
struct MemoryPrefsStore(HashMap<String, String>);

impl PrefsStore for MemoryPrefsStore {
    fn get(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
}

/// 🔑️ Every `const …STORAGE_KEY… = "value"` / `…DIAGNOSTICS_KEY = "value"` React declares, read out of
/// its own sources. A declaration, not a transcription: a key React adds shows up here on the next run.
fn react_declared_storage_keys() -> std::collections::BTreeSet<String> {
    let files = [
        "🧰️framework/🔨️modules/🖥️platform/🟦️.ts",
        "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
        "🧰️framework/🔨️modules/🖱️ui/🔨️modules/💾️keybinding-persistence/🟦️.ts",
        "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🚗️UiDriver/🟦️.tsx",
        "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx",
        "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx",
    ];
    let mut keys = std::collections::BTreeSet::new();
    for file in files {
        for line in read_source(repo_root().join(file)).lines() {
            if !line.contains("const ") || !(line.contains("STORAGE_KEY") || line.contains("DIAGNOSTICS_KEY")) {
                continue;
            }
            let Some(rest) = line.split_once(" = \"") else { continue };
            let Some((value, _)) = rest.1.split_once('"') else { continue };
            keys.insert(value.to_string());
        }
    }
    assert!(keys.len() >= 12, "React's key readers moved: found only {keys:?}");
    keys
}

fn census_names(kind: HostStorageKeyKind) -> Vec<&'static str> {
    HOST_STORAGE_KEY_CENSUS.iter().filter(|(_, row)| *row == kind).map(|(name, _)| *name).collect()
}

fn ts_string_array(source: &str, name: &str) -> Vec<String> {
    let start = source.find(&format!("export const {name} = [")).unwrap_or_else(|| panic!("{name} is declared in 🧭️boot-descriptor/🟦️.ts"));
    let body = &source[start + format!("export const {name} = [").len()..];
    let end = body.find(']').expect("array literal is closed");
    body[..end].split(',').filter_map(|entry| entry.trim().strip_prefix('"').and_then(|entry| entry.strip_suffix('"')).map(ToOwned::to_owned)).collect()
}

//#region 🔑️KeyInventory

/// 🧪️ **The parity ledger.** Every durable key React's shell stack declares is classified here, and
/// nothing is classified that React does not declare. A key React adds therefore fails this law rather
/// than quietly going unserved by the wgpu renderer — which is exactly how the browser build came to
/// persist nothing at all without a single test noticing.
///
/// The classification itself is the honest part: `Carried`/`CarriedFamily` are what the boot snapshot
/// and the door address, `UiLibrary` are the standalone `🖱️ui` surface library's keys that React's own
/// OS shell never reads either (it persists through `semio.os.config`'s `os.config.ui-preferences`
/// event log, and so does this renderer), and `Session` is the credential-bearing presence pack.
#[test]
fn the_key_census_covers_reacts_own_key_list() {
    let declared = react_declared_storage_keys();
    let classified: std::collections::BTreeSet<String> = HOST_STORAGE_KEY_CENSUS.iter().map(|(name, _)| (*name).to_string()).collect();

    let unclassified: Vec<&String> = declared.difference(&classified).collect();
    assert!(unclassified.is_empty(), "React declares durable keys this renderer classifies nowhere: {unclassified:?}");
    let invented: Vec<&String> = classified.difference(&declared).collect();
    assert!(invented.is_empty(), "the census carries keys React does not declare: {invented:?}");

    assert_eq!(census_names(HostStorageKeyKind::Carried), vec!["SEMIO_RUNTIME_DIAGNOSTICS", "semio.os.config", "ui.compute.workerCount"]);
    assert_eq!(census_names(HostStorageKeyKind::CarriedFamily), vec![UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX]);
    assert_eq!(census_names(HostStorageKeyKind::Session), vec!["semio.presence.client"], "sessionStorage and credential-bearing — reachable only through an explicit session-scope call");
    assert_eq!(
        census_names(HostStorageKeyKind::UiLibrary),
        vec!["ui.chrome.appearance", "ui.chrome.driver", "ui.chrome.layout", "ui.chrome.locale", "ui.chrome.terminology", "ui.chrome.theme", "ui.chrome.theme.snapshot", "ui.drivers.custom", "ui.keybindings.overrides", "ui.themes.custom"],
        "the `🖱️ui` surface library's own lane; React's OS shell reads none of them either"
    );

    assert!(host_storage_carries_key(OS_SHELL_CONFIG_STORAGE_KEY));
    assert!(host_storage_carries_key(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY));
    assert!(host_storage_carries_key(&format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}s.puzzle.puzzle3d@1/*#editor")));
    assert!(!host_storage_carries_key(UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX), "the bare family prefix names no app and is not itself a key");
    assert!(!host_storage_carries_key("ui.chrome.appearance"), "a `🖱️ui`-library key is classified, not carried");
    assert!(!host_storage_carries_key("semio.presence.client"), "sessionStorage stays off the boot snapshot");
    assert!(!host_storage_carries_key("semio.os.config.evil"));
}

/// 🧪️ The page reads exactly the list this isolate serves. Two halves of one census in two languages
/// is how the old shim's key spellings were free to drift from React's; they cannot drift now.
#[test]
fn the_page_reads_the_same_census_this_isolate_serves() {
    let source = wgpu_target_source("🧭️boot-descriptor/🟦️.ts");
    assert_eq!(ts_string_array(&source, "WGPU_HOST_STORAGE_KEYS"), census_names(HostStorageKeyKind::Carried));
    assert_eq!(ts_string_array(&source, "WGPU_HOST_STORAGE_KEY_PREFIXES"), census_names(HostStorageKeyKind::CarriedFamily));
    assert!(source.contains("export const WGPU_HOST_STORAGE_VALUE_MAX_BYTES = 64 * 1024;"), "the page must refuse the same per-value bound this isolate does");
    assert_eq!(HOST_STORAGE_VALUE_MAX_BYTES, 64 * 1024);
    assert!(source.contains("export const WGPU_HOST_STORAGE_SNAPSHOT_MAX_BYTES = 128 * 1024;"));
    assert_eq!(HOST_STORAGE_SNAPSHOT_MAX_BYTES, 128 * 1024);
}
//#endregion 🔑️KeyInventory

//#region 🚪️DoorWire

/// 🧪️ Every verb and both scopes round-trip, and the two bounds refuse at the ENCODER rather than
/// travelling to a page that would drop them silently.
#[test]
fn the_door_wire_carries_every_verb_and_refuses_what_the_census_does_not() {
    let request = encode_storage_door_request(StorageDoorVerb::Get, StorageDoorScope::Local, OS_SHELL_CONFIG_STORAGE_KEY, None).expect("a carried key encodes");
    let parsed: Value = serde_json::from_str(&request).expect("the request is JSON");
    assert_eq!(parsed["op"], Value::String(STORAGE_DOOR_OP.to_string()));
    assert_eq!(parsed["verb"], Value::String("get".to_string()));
    assert_eq!(parsed["scope"], Value::String("local".to_string()));
    assert_eq!(parsed["key"], Value::String(OS_SHELL_CONFIG_STORAGE_KEY.to_string()));
    assert!(parsed.get("value").is_none(), "a read carries no value");

    let set = encode_storage_door_request(StorageDoorVerb::Set, StorageDoorScope::Local, UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, Some("8")).expect("a bounded write encodes");
    assert_eq!(serde_json::from_str::<Value>(&set).expect("json")["value"], Value::String("8".to_string()));
    assert!(encode_storage_door_request(StorageDoorVerb::Remove, StorageDoorScope::Session, "semio.presence.client", None).is_err(), "session keys are classified, not carried — nothing addresses them yet");
    assert_eq!(StorageDoorVerb::Remove.as_id(), "remove");
    assert_eq!(StorageDoorScope::Session.as_id(), "session");

    assert!(encode_storage_door_request(StorageDoorVerb::Get, StorageDoorScope::Local, "ui.chrome.appearance", None).is_err(), "an uncarried key is refused loudly, never served");
    let oversized = "x".repeat(HOST_STORAGE_VALUE_MAX_BYTES + 1);
    assert!(encode_storage_door_request(StorageDoorVerb::Set, StorageDoorScope::Local, OS_SHELL_CONFIG_STORAGE_KEY, Some(&oversized)).is_err());
}

/// 🧪️ "The store holds nothing" and "the call was refused" are different answers, and a refusal is
/// never flattened into an empty read — the same distinction the directory door keeps for exactly the
/// same reason (a retry policy branches on it).
#[test]
fn an_empty_read_and_a_refusal_are_different_answers() {
    assert_eq!(decode_storage_door_answer(r#"{"value":"true"}"#).expect("a value"), Some("true".to_string()));
    assert_eq!(decode_storage_door_answer(r#"{"value":null}"#).expect("an empty read"), None);
    assert_eq!(decode_storage_door_answer("{}").expect("a settled write"), None);
    assert_eq!(decode_storage_door_answer(r#"{"error":"this realm owns no localStorage"}"#).expect_err("a refusal"), "this realm owns no localStorage");
    assert!(decode_storage_door_answer("not json").is_err());
    assert!(decode_storage_door_answer(r#"{"value":7}"#).is_err(), "a non-string value is a page defect, not a preference");
}
//#endregion 🚪️DoorWire

//#region 🗄️BootSnapshot

/// 🧪️ The boot snapshot answers the FIRST frame's reads — appearance's event log, the tour's seen
/// flag, the dock skeleton's document and the compute worker count — synchronously, which is the whole
/// reason the page reads them before the shell boots rather than letting the shell ask across a
/// `Promise` it cannot await inside a frame build.
#[test]
fn a_seeded_snapshot_answers_the_first_frames_reads() {
    let seen_key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}s.puzzle.puzzle3d@1/*#editor");
    let document = serde_json::json!({
        "version": 1,
        "preferences": { UI_PREFERENCES_CONFIG_SCHEMA: r#"{"version":1,"events":[{"mutation":"setAppearance","appearance":"light"}]}"# },
        "namedLayouts": {},
        "dockLayouts": { "apps": {} },
        "dockUi": { "apps": {} },
        "windowPanes": { "apps": {} }
    })
    .to_string();
    let accepted = seed_host_storage(&serde_json::json!({ OS_SHELL_CONFIG_STORAGE_KEY: document, UI_COMPUTE_WORKER_COUNT_STORAGE_KEY: "6", seen_key.clone(): "true", "ui.chrome.appearance": "dark" }).to_string());

    assert_eq!(accepted, 3, "the uncarried `🖱️ui`-library key is dropped, the three carried ones land");
    assert_eq!(host_storage_get(OS_SHELL_CONFIG_STORAGE_KEY).as_deref(), Some(document.as_str()));
    assert_eq!(host_storage_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).as_deref(), Some("6"));
    assert_eq!(host_storage_get(&seen_key).as_deref(), Some("true"));
    assert_eq!(host_storage_get("ui.chrome.appearance"), None);

    let replayed = decode_ui_preferences_event_log(serde_json::from_str::<Value>(&document).expect("json")["preferences"][UI_PREFERENCES_CONFIG_SCHEMA].as_str().expect("the log is a string")).expect("the snapshot carries React's own event log");
    assert_eq!(appearance_id(replay_ui_preferences(&replayed).appearance).as_deref(), Some("light"), "a Light choice made in React is read by this renderer from the same bytes");
}

/// 🧪️ The precedence the packet owes: snapshot → later set → read. The cache write lands BEFORE the
/// door hop, because the door is a `Promise` and a frame is not — a read that follows a write in the
/// same frame must see the written value or the shell would paint a preference the user just left.
#[test]
fn a_later_set_outranks_the_boot_snapshot() {
    seed_host_storage(&serde_json::json!({ UI_COMPUTE_WORKER_COUNT_STORAGE_KEY: "2" }).to_string());
    assert_eq!(host_storage_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).as_deref(), Some("2"));

    host_storage_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, "12");
    assert_eq!(host_storage_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).as_deref(), Some("12"));

    host_storage_remove(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY);
    assert_eq!(host_storage_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY), None);

    host_storage_set("ui.chrome.appearance", "dark");
    assert_eq!(host_storage_get("ui.chrome.appearance"), None, "an uncarried key is refused by the store too, not only by the wire");
    host_storage_set(OS_SHELL_CONFIG_STORAGE_KEY, &"x".repeat(HOST_STORAGE_VALUE_MAX_BYTES + 1));
    assert_eq!(host_storage_get(OS_SHELL_CONFIG_STORAGE_KEY), None);

    seed_host_storage(&serde_json::json!({ UI_COMPUTE_WORKER_COUNT_STORAGE_KEY: "3" }).to_string());
    assert_eq!(host_storage_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).as_deref(), Some("3"), "a cross-tab re-seed replaces the store, because the page's store IS the authority");
}

/// 🧪️ A malformed or hostile snapshot costs the preferences it could not carry and nothing else — one
/// unreadable value must never cost a boot every other preference it could have served.
#[test]
fn a_refused_snapshot_entry_never_costs_the_rest() {
    let accepted = seed_host_storage(&serde_json::json!({ OS_SHELL_CONFIG_STORAGE_KEY: "{}", UI_COMPUTE_WORKER_COUNT_STORAGE_KEY: 4, "ui.introduction.seen.a": "true" }).to_string());
    assert_eq!(accepted, 2, "the non-string entry is dropped; the two readable ones land");
    assert_eq!(host_storage_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY), None);
    assert_eq!(host_storage_get("ui.introduction.seen.a").as_deref(), Some("true"));

    assert!(decode_host_storage_snapshot("not json").is_empty());
    assert!(decode_host_storage_snapshot("[]").is_empty());
    assert!(decode_host_storage_snapshot(&"x".repeat(HOST_STORAGE_SNAPSHOT_MAX_BYTES + 1)).is_empty());
    assert!(decode_host_storage_snapshot(&serde_json::json!({ OS_SHELL_CONFIG_STORAGE_KEY: "x".repeat(HOST_STORAGE_VALUE_MAX_BYTES + 1) }).to_string()).is_empty());
    seed_host_storage("{}");
}
//#endregion 🗄️BootSnapshot

//#region 🎓️KeyEncodings

/// 🧪️ Per-key value encodings, against React's own readers. Each row is what React writes and reads,
/// so a profile written by either renderer is read by the other:
///
/// * `ui.introduction.seen.<appId>` — the literal `"true"`, TOP-LEVEL (`readStoredIntroductionSeen`);
/// * `ui.compute.workerCount` — a decimal integer ≥ 1, TOP-LEVEL (`readStoredComputeWorkerCount`);
/// * `os.config.ui-preferences` — `{"version":1,"events":[…]}` INSIDE `semio.os.config.preferences`
///   (`OsShellConfig.getPreference`).
///
/// 🩸️ The nesting is the correction this packet makes: the first two used to be written into that
/// `preferences` map, where React's flat readers could never find them.
#[test]
fn every_carried_key_round_trips_in_reacts_own_encoding() {
    let app_id = "s.puzzle.puzzle3d@1/*#editor";
    let seen_key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{app_id}");
    assert_eq!(seen_key, "ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor");

    seed_host_storage("{}");
    assert_ne!(host_storage_get(&seen_key).as_deref(), Some("true"), "an unseen app reads as unseen — an absent key, not a false value");
    host_storage_set(&seen_key, "true");
    assert_eq!(host_storage_get(&seen_key).as_deref(), Some("true"));

    host_storage_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &9u32.to_string());
    assert_eq!(host_storage_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()), Some(9));

    let mut store = MemoryPrefsStore::default();
    let log = encode_ui_preferences_event_log(&UiPreferencesEventLog { version: 1, events: vec![set_locale(Some(OsUiLocale::De))] });
    prefs_set_in(&mut store, UI_PREFERENCES_CONFIG_SCHEMA, &log);
    let document: Value = serde_json::from_str(&store.get(OS_SHELL_CONFIG_STORAGE_KEY).expect("the document lands under React's own key")).expect("json");
    assert_eq!(document["version"], Value::from(1), "React's `OsShellConfig` refuses any other version");
    for projection in ["namedLayouts", "dockLayouts", "dockUi", "windowPanes"] {
        assert!(document[projection].is_object(), "a preference write must never drop the sibling projection {projection}");
    }
    assert_eq!(document["preferences"][UI_PREFERENCES_CONFIG_SCHEMA].as_str(), Some(log.as_str()));
    assert_eq!(prefs_get_from(&store, UI_PREFERENCES_CONFIG_SCHEMA).as_deref(), Some(log.as_str()));
    assert_eq!(replay_ui_preferences(&decode_ui_preferences_event_log(&log).expect("the log decodes")).locale, Some(OsUiLocale::De));
    seed_host_storage("{}");
}

/// 🧪️ **The tour answer survives a reload.** A fresh profile carries no seen flag, so the tour arms;
/// the answer is written under React's own key with React's own value, so the next boot's snapshot
/// carries it and the tour stays away — on either renderer.
///
/// 🩸️ This is `📓️w4a` §5 item 3 closed: the write used to go through `WebLocalStorage`, so the tour
/// was answered once per LOAD and returned on every reload of the browser build.
#[test]
fn the_tour_answer_is_written_where_reacts_next_boot_reads_it() {
    let app_id = "s.puzzle.puzzle3d@1/*#editor";
    let seen_key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{app_id}");

    seed_host_storage("{}");
    let fresh = host_storage_get(&seen_key).as_deref() == Some("true");
    assert!(!fresh);
    assert!(should_auto_start_introduction(app_id, true, false, fresh, false), "a fresh profile arms the tour");

    let mut chrome = ShellChromeBuildState::default();
    chrome.introduction_seen.insert(app_id.to_string(), false);
    chrome.start_introduction();
    assert!(chrome.tour_state.is_some());
    chrome.dismiss_introduction(app_id);
    assert!(chrome.tour_state.is_none(), "Skip and Done both end the tour");
    assert_eq!(chrome.introduction_seen_writes, vec![app_id.to_string()], "and both queue the ONE bounded write the maintenance step drains");

    // 🗄️ `stored_field_set` is what the maintenance step calls (pinned by
    // `the_browser_preference_lane_is_the_page_door`); on a native test binary its arm is the file
    // store, so the BROWSER arm is driven directly here — the one this packet owns.
    host_storage_set(&seen_key, "true");
    let reloaded = host_storage_get(&seen_key).as_deref() == Some("true");
    assert!(reloaded, "the answer is in the store the page will hand the next boot");
    assert!(!should_auto_start_introduction(app_id, true, false, reloaded, false));
    seed_host_storage("{}");
}
//#endregion 🎓️KeyEncodings

//#region 🚪️Wiring

/// 🧪️ The browser preference lane IS the page door — there is no Worker-local store left to fall back
/// to, and the two flat keys are read flat. Source-read, because every assertion here is about a
/// `cfg(target_arch = "wasm32")` arm that a native test binary never compiles.
#[test]
fn the_browser_preference_lane_is_the_page_door() {
    let shell = wgpu_shell_source();
    assert!(!shell.contains("struct WebLocalStorage"), "the shim that resolved localStorage through a window the Worker has not is gone");
    assert!(!shell.contains("\"localStorage\""), "nothing in the shell reaches a localStorage binding directly any more");
    assert!(shell.contains("        host_storage_get(key)\n"), "`raw_prefs_get`'s browser arm reads the page-seeded store");
    assert!(shell.contains("    host_storage_set(key, value);\n"), "`raw_prefs_set`'s browser arm writes through the door");
    assert!(shell.contains("#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetHostStorage)]"), "the seeding hook is exported to both page doors");
    assert!(shell.contains("crate::spawn_app_task(async move {"), "the write-through is fire-and-forget: a preference write must never fault a surface");

    let load = shell.split("fn advance_chrome_maintenance_step").nth(1).expect("the chrome I/O lane");
    let load = &load[..load.find("fn advance_chrome_preferences_load_step").expect("the lane ends at the load step")];
    assert!(load.contains("stored_field_get(&key)") && load.contains("stored_field_set(&key, \"true\")"), "the tour's seen flag is a FLAT key, the way `readStoredIntroductionSeen` reads it");
    assert!(!load.contains("prefs_get_bounded") && !load.contains("prefs_set_bounded"), "the nested-field accessors that hid both flat keys from React are gone");
}

/// 🧪️ All three page doors read the census and hand it over, and the Worker applies it BEFORE the
/// shell boots — a snapshot that arrived after the first frame would be a second boot pass, which is
/// the flash this design exists to avoid.
#[test]
fn every_page_door_hands_the_census_across() {
    let host_io = wgpu_target_source("🚪️host-io/🟦️.ts");
    assert!(host_io.contains("readonly op: \"storage\""), "the page services the storage op");
    assert!(host_io.contains("wgpuHostStorageCarriesKey(request.key)"), "and refuses anything outside the census");
    assert!(host_io.contains("request.scope === \"session\" ? globalThis.sessionStorage : globalThis.localStorage"), "both of React's stores, addressed by scope");

    let worker = wgpu_target_source("🎞️frame-worker/🟦️.ts");
    let environment = worker.split("ownedStep(\"runtime-environment\"").nth(1).expect("the owned boot step");
    assert!(environment[..environment.find("progress(\"plugin-graph\"").expect("the step ends before the plugin graph")].contains("semioWgpuSetHostStorage?.(JSON.stringify(message.storage))"), "the snapshot is applied in the boot step, before any shell reads it");
    assert!(worker.contains("message.kind === \"host-storage\""), "and re-applied when another tab rewrites a key");

    let boot = wgpu_target_source("🚀️browser-boot/🟦️.ts");
    assert!(boot.contains("storage: hostStorage()"), "the trunk page reads the census into the boot message");
    assert!(boot.contains("const republishHostStorage = () => transport.setHostStorage(hostStorage());") && boot.contains("\"storage\", republishHostStorage"), "and re-reads it on a `storage` event, the only way a page hears another tab");

    let embedded = wgpu_target_source("🎬️renderer-boot/🟦️.ts");
    assert!(embedded.contains("semioWgpuSetHostStorage?.(JSON.stringify(readWgpuHostStorageSnapshot(window)))"), "the embeddable door runs ON the page and seeds the same store itself");
}
//#endregion 🚪️Wiring
