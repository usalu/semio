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
    println!("[DEBUG] preference journal replay and concurrent event revisions verified");
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
