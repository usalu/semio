use super::*;

#[test]
fn preferences_match_independent_language_neutral_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⚙️preferences/🔣️.json")).unwrap();
    assert_eq!(serde_json::to_value(Preferences::default()).unwrap(), vectors["defaults"]);
    for value in vectors["valid"].as_array().unwrap() { assert!(Change::parse(&value.to_string()).is_ok()); }
    for value in vectors["invalid"].as_array().unwrap() { assert!(Change::parse(&value.to_string()).is_err(), "{value}"); }
    for case in vectors["cases"].as_array().unwrap() {
        let mut preferences = Preferences::default();
        for layer in case["layers"].as_array().unwrap() { preferences.apply(&Change::parse(&layer.to_string()).unwrap()); }
        assert_eq!(serde_json::to_value(preferences).unwrap(), case["expected"]);
    }
}

#[test]
fn preference_events_replay_and_serialize_concurrent_commands() {
    let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from);
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = base.join(format!("dashboard-preferences-{nonce}"));
    let path = root.join("preferences.jsonl");
    let writers: Vec<_> = (0..8).map(|_| { let path = path.clone(); std::thread::spawn(move || append(&path, &Change::parse(r#"{"language":"de","terminology":"reuse"}"#).unwrap()).unwrap()) }).collect();
    for writer in writers { writer.join().unwrap(); }
    let mut preferences = Preferences::default();
    assert_eq!(replay(&path, &mut preferences).unwrap(), 8);
    assert_eq!(preferences.language, "de");
    assert_eq!(preferences.terminology, "reuse");
    fs::write(&path, "{\"version\":1,\"revision\":2,\"change\":{\"language\":\"de\"}}\n").unwrap();
    assert!(replay(&path, &mut preferences).is_err());
    fs::write(&path, "{\"version\":1,\"revision\":1,\"change\":{\"language\":\"de\",\"appearance\":null}}\n").unwrap();
    assert!(replay(&path, &mut preferences).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn explicit_arguments_override_environment_and_reject_mistakes() {
    let mut preferences = Preferences::default();
    preferences.apply(&Change::parse(r#"{"language":"de","renderer":"wgpu-native"}"#).unwrap());
    let parsed = crate::args::parse(&["dashboard".into(), "--language".into(), "en".into(), "--layout".into(), "rows".into()]);
    apply_arguments(&mut preferences, &parsed).unwrap();
    assert_eq!(preferences.language, "en");
    assert_eq!(preferences.layout, "rows");
    for argv in [vec!["dashboard", "--language", "xx"], vec!["dashboard", "--unknown", "yes"], vec!["dashboard", "--root"], vec!["dashboard", "--config", ""], vec!["dashboard", "--workspace", "yes"]] {
        assert!(apply_arguments(&mut preferences, &crate::args::parse(&argv.into_iter().map(String::from).collect::<Vec<_>>())).is_err());
    }
}

#[test]
fn keymap_customizations_travel_as_preference_events_and_layers_merge_per_binding() {
    let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from);
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = base.join(format!("dashboard-keymap-{nonce}"));
    let (shared, local) = (root.join("shared.jsonl"), root.join("local.jsonl"));
    append(&shared, &Change::parse(r#"{"prefix":"ctrl+a","bindings":{"prefix.split-right":["w"],"prefix.detach":["q"]}}"#).unwrap()).unwrap();
    append(&local, &Change::parse(r#"{"bindings":{"prefix.split-right":["v"]}}"#).unwrap()).unwrap();
    let mut preferences = Preferences::default();
    replay(&shared, &mut preferences).unwrap();
    replay(&local, &mut preferences).unwrap();
    assert_eq!(preferences.prefix, "ctrl+a");
    assert_eq!(preferences.bindings["prefix.split-right"], ["v"], "the later layer wins per binding");
    assert_eq!(preferences.bindings["prefix.detach"], ["q"], "bindings the later layer does not mention stay");
    let (keymap, problems) = preferences.keymap();
    assert!(problems.is_empty(), "{problems:?}");
    let spec = |text: &str| keymap::KeySpec::parse(text).unwrap();
    assert_eq!(keymap.resolve(keymap::Scope::Prefix, &spec("v")), Some("split-right"));
    assert_eq!(keymap.resolve(keymap::Scope::Prefix, &spec("q")), Some("detach"));
    assert_eq!(keymap.resolve(keymap::Scope::Prefix, &spec("ctrl+a")), Some(keymap::SEND_PREFIX));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn invalid_keymap_changes_are_rejected_before_they_can_be_published() {
    for text in [r#"{"prefix":"x"}"#, r#"{"prefix":"ctrl+i"}"#, r#"{"bindings":{}}"#, r#"{"bindings":{"prefix.teleport":["t"]}}"#, r#"{"bindings":{"prefix.detach":[]}}"#, r#"{"bindings":{"view.back":["q"]}}"#, r#"{"bindings":{"prefix.detach":["d","d"]}}"#, r#"{"bindings":null}"#, r#"{"bindings":{"prefix.detach":"d"}}"#] {
        assert!(Change::parse(text).is_err(), "{text}");
    }
    let path = std::env::temp_dir().join("dashboard-keymap-unused.jsonl");
    assert!(append(&path, &Change { prefix: Some("q".into()), ..Default::default() }).is_err());
    assert!(!path.exists());
}

#[test]
fn keymap_arguments_override_the_journal_and_reject_mistakes() {
    let mut preferences = Preferences::default();
    let parsed = crate::args::parse(&["dashboard".into(), "--prefix".into(), "alt+x".into(), "--bindings".into(), r#"{"prefix.detach":["q"]}"#.into()]);
    apply_arguments(&mut preferences, &parsed).unwrap();
    assert_eq!((preferences.prefix.as_str(), preferences.bindings["prefix.detach"][0].as_str()), ("alt+x", "q"));
    for argv in [vec!["dashboard", "--prefix", "x"], vec!["dashboard", "--bindings", "not-json"], vec!["dashboard", "--bindings", r#"{"prefix.nothing":["x"]}"#]] {
        let words: Vec<String> = argv.iter().map(|word| (*word).to_string()).collect();
        assert!(apply_arguments(&mut preferences, &crate::args::parse(&words)).is_err(), "{argv:?}");
    }
}

#[test]
fn preferences_pre_select_only_the_launch_parameters_a_command_offers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔎️launcher/🔣️.json")).unwrap();
    let facts: crate::registry::Facts = serde_json::from_value(fixture["workspace"].clone()).unwrap();
    let registry = crate::registry::Registry::build(Path::new("workspace"), &facts);
    let preferences = Preferences { language: "de".into(), renderer: "wgpu-wasm".into(), ..Default::default() };
    let playground = registry.find("playground:puzzle3d").unwrap();
    let defaults = preferences.launch_defaults(&registry.parameters(playground));
    assert!(defaults.contains(&("renderer".to_string(), "wgpu-wasm".to_string())) && defaults.contains(&("language".to_string(), "de".to_string())), "{defaults:?}");
    assert!(preferences.launch_defaults(&registry.parameters(registry.find("@fx/quiz:test").unwrap())).is_empty());
}

fn scratch(name: &str) -> PathBuf {
    let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from);
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    base.join(format!("dashboard-{name}-{nonce}"))
}

#[test]
fn p2_1_a_torn_last_line_is_ignored_and_cut_off_by_the_next_writer() {
    let root = scratch("torn");
    let path = root.join("preferences.jsonl");
    append(&path, &Change::parse(r#"{"language":"de"}"#).unwrap()).unwrap();
    let mut file = OpenOptions::new().append(true).open(&path).unwrap();
    file.write_all(b"{\"version\":1,\"revision\":2,\"change\":{\"appeara").unwrap();
    drop(file);
    let mut preferences = Preferences::default();
    assert_eq!(replay(&path, &mut preferences).unwrap(), 1, "the unfinished event is not part of the journal");
    assert_eq!(preferences.language, "de");
    assert_eq!(append(&path, &Change::parse(r#"{"appearance":"light"}"#).unwrap()).unwrap(), 2);
    let mut again = Preferences::default();
    assert_eq!(replay(&path, &mut again).unwrap(), 2);
    assert_eq!((again.language.as_str(), again.appearance.as_str()), ("de", "light"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn p2_1_lock_contention_is_bounded_and_names_the_file() {
    let root = scratch("busy");
    let path = root.join("preferences.jsonl");
    append(&path, &Change::parse(r#"{"language":"de"}"#).unwrap()).unwrap();
    let holder = OpenOptions::new().read(true).write(true).open(&path).unwrap();
    holder.lock().unwrap();
    let started = std::time::Instant::now();
    let error = replay(&path, &mut Preferences::default()).unwrap_err().to_string();
    assert!(started.elapsed() < std::time::Duration::from_secs(5), "the wait is bounded");
    assert!(error.contains("preferences.jsonl") && error.contains("busy"), "{error}");
    drop(holder);
    assert!(replay(&path, &mut Preferences::default()).is_ok());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn p2_1_a_full_journal_is_compacted_instead_of_refusing_changes() {
    let root = scratch("compact");
    let path = root.join("preferences.jsonl");
    append(&path, &Change::parse(r#"{"language":"de","prefix":"ctrl+a","bindings":{"prefix.detach":["q"]}}"#).unwrap()).unwrap();
    let filler = "x".repeat(900);
    let mut file = OpenOptions::new().append(true).open(&path).unwrap();
    let mut revision = 1;
    while fs::metadata(&path).unwrap().len() < LIMIT - 40 {
        revision += 1;
        let line = format!("{{\"version\":1,\"revision\":{revision},\"change\":{{\"language\":\"de\",\"terminology\":\"native\"}},\"note\":\"{filler}\"}}\n");
        let _ = line;
        let plain = format!("{{\"version\":1,\"revision\":{revision},\"change\":{{\"layout\":\"rows\"}}}}\n").repeat(1);
        file.write_all(plain.as_bytes()).unwrap();
        if revision > 40_000 { break; }
    }
    drop(file);
    assert!(fs::metadata(&path).unwrap().len() > LIMIT - 120);
    let after = append(&path, &Change::parse(r#"{"appearance":"light"}"#).unwrap()).unwrap();
    assert_eq!(after, 2, "the journal starts over with a snapshot and the new event");
    let mut preferences = Preferences::default();
    assert_eq!(replay(&path, &mut preferences).unwrap(), 2);
    assert_eq!((preferences.language.as_str(), preferences.appearance.as_str(), preferences.layout.as_str(), preferences.prefix.as_str()), ("de", "light", "rows", "ctrl+a"));
    assert_eq!(preferences.bindings["prefix.detach"], ["q"]);
    assert!(fs::metadata(&path).unwrap().len() < 2000);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn p2_1_a_broken_journal_never_stops_the_dashboard_from_starting() {
    let root = scratch("broken");
    let parsed = crate::args::parse(&["dashboard".into(), "--config".into(), root.join("local.jsonl").display().to_string()]);
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("local.jsonl"), "this is not an event\n").unwrap();
    assert!(load(&root, &parsed).is_err(), "the strict loader still reports it");
    let (preferences, problems) = load_lenient(&root, &parsed).unwrap();
    assert_eq!(preferences, Preferences::default());
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("local.jsonl"), "{problems:?}");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn every_scenario_of_the_preferences_feature_is_proved_by_a_test() {
    crate::tests::assert_proved(include_str!("../../../🧪️tests/⚙️preferences/🥒️.feature"), &[
        include_str!("🦀️.rs"),
        include_str!("../../../🖥️terminal/🧪️tests/🔬️unit/🦀️.rs"),
        include_str!("../../../🖥️terminal/🚀️launcher/🧪️tests/🔬️unit/🦀️.rs"),
        include_str!("../../../🖥️terminal/🌐️labels/🧪️tests/🔬️unit/🦀️.rs"),
        include_str!("../../../📚️inventory/🧪️tests/🔬️unit/🦀️.rs"),
    ], &[
        ("Starting without preferences", &["preferences_match_independent_language_neutral_vectors"]),
        ("Resolving layered preferences", &["preferences_match_independent_language_neutral_vectors", "explicit_arguments_override_environment_and_reject_mistakes"]),
        ("Customization survives reopening", &["preference_events_replay_and_serialize_concurrent_commands"]),
        ("The prefix key and single bindings are customized by events", &["keymap_customizations_travel_as_preference_events_and_layers_merge_per_binding", "invalid_keymap_changes_are_rejected_before_they_can_be_published"]),
        ("The language follows the preference everywhere", &["the_catalogue_follows_the_chosen_language", "p1_11_no_user_visible_text_is_hard_coded_outside_the_catalogue"]),
        ("Discovery does not block interaction", &["the_job_publishes_the_known_commands_first_then_the_result_of_the_walk_and_finishes", "a_cancelled_discovery_returns_nothing_and_leaves_no_snapshot"]),
        ("Keyboard and mouse use the same command selection", &["journeys_match_the_shared_vectors"]),
        ("Preferences pre-select what a command offers", &["preferences_pre_select_only_the_launch_parameters_a_command_offers"]),
    ]);
}
