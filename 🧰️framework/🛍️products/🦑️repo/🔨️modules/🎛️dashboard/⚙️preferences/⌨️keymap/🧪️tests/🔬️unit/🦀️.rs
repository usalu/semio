use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../🧫️fixtures/⌨️keymap/🔣️.json")).unwrap() }

fn overrides(value: &serde_json::Value) -> BTreeMap<String, Vec<String>> {
    value.as_object().unwrap().iter().map(|(id, keys)| (id.clone(), keys.as_array().unwrap().iter().map(|key| key.as_str().unwrap().to_string()).collect())).collect()
}

fn scope(name: &str) -> Scope { Scope::ALL.into_iter().find(|scope| scope.as_str() == name).unwrap() }

fn resolve_all(keymap: &Keymap, rows: &serde_json::Value) {
    for row in rows.as_array().unwrap() {
        let key = KeySpec::parse(row["key"].as_str().unwrap()).unwrap();
        assert_eq!(keymap.resolve(scope(row["scope"].as_str().unwrap()), &key), row["action"].as_str(), "{row}");
    }
}

#[test]
fn spellings_fold_into_one_canonical_key_with_a_readable_label() {
    for row in fixture()["spellings"].as_array().unwrap() {
        let key = KeySpec::parse(row["spec"].as_str().unwrap()).unwrap();
        assert_eq!(key.to_string(), row["canonical"].as_str().unwrap(), "{row}");
        assert_eq!(key.label(), row["label"].as_str().unwrap(), "{row}");
        assert_eq!(KeySpec::parse(&key.to_string()).unwrap(), key, "canonical form must parse back to itself: {row}");
    }
    for text in fixture()["rejected"].as_array().unwrap() { assert!(KeySpec::parse(text.as_str().unwrap()).is_err(), "{text}"); }
}

#[test]
fn shipped_defaults_resolve_per_the_shared_vectors_and_never_collide() {
    let defaults = Keymap::defaults();
    assert_eq!(defaults.prefix().to_string(), fixture()["defaults"]["prefix"].as_str().unwrap());
    resolve_all(&defaults, &fixture()["defaults"]["resolve"]);
    assert_eq!(defaults.conflict(), None);
    let mut ids: Vec<String> = Scope::ALL.into_iter().flat_map(|scope| defaults.bindings(scope).map(Binding::id).collect::<Vec<_>>()).collect();
    let count = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), count, "a scope states an action once");
    for scope in [Scope::Window, Scope::View] { for binding in defaults.bindings(scope) { assert!(binding.keys.iter().all(|key| !key.is_plain_printable()), "{} would block typing", binding.id()); } }
    assert!(Keymap::check_prefix(&defaults.prefix().to_string()).is_ok());
}

#[test]
fn customizations_fold_over_the_defaults_and_collisions_are_named() {
    for case in fixture()["cases"].as_array().unwrap() {
        let (keymap, problems) = Keymap::build(case["prefix"].as_str().unwrap(), &overrides(&case["overrides"]));
        resolve_all(&keymap, &case["resolve"]);
        assert_eq!(problems.len() as u64, case["problems"].as_u64().unwrap(), "{}: {problems:?}", case["name"]);
        assert_eq!(keymap.conflict(), None, "{}", case["name"]);
    }
}

#[test]
fn every_action_stays_reachable_by_keyboard_under_any_customization() {
    let defaults = Keymap::defaults();
    let (keymap, _) = Keymap::build("ctrl+a", &overrides(&serde_json::json!({ "prefix.detach": ["n"], "view.back": ["q"], "window.close-window": ["ctrl+q"] })));
    for scope in Scope::ALL { for action in defaults.actions(scope) { assert!(keymap.keys_label(scope, &action).is_some_and(|label| !label.is_empty()), "{action} in {}", scope.as_str()); } }
    assert_eq!(keymap.keys_label(Scope::Prefix, "send-prefix").as_deref(), Some("Ctrl+A Ctrl+A"));
    assert_eq!(keymap.keys_label(Scope::Prefix, "split-right").as_deref(), Some("Ctrl+A | / v"));
    assert_eq!(keymap.keys_label(Scope::Window, "close-window").as_deref(), Some("Ctrl+Q"));
}

#[test]
fn every_scenario_of_the_keymap_feature_is_proved_by_a_test() {
    crate::tests::assert_proved(include_str!("../../../../🧪️tests/⌨️keymap/🥒️.feature"), &[include_str!("🦀️.rs"), include_str!("../../../../🖥️terminal/🧪️tests/🔬️unit/🦀️.rs")], &[
        ("Key spellings fold into one canonical key", &["spellings_fold_into_one_canonical_key_with_a_readable_label"]),
        ("The keymap is data with action ids", &["shipped_defaults_resolve_per_the_shared_vectors_and_never_collide"]),
        ("Customizations fold over the defaults without losing a key", &["customizations_fold_over_the_defaults_and_collisions_are_named"]),
        ("Every action stays reachable by keyboard under any customization", &["every_action_stays_reachable_by_keyboard_under_any_customization"]),
        ("Terminal bytes become the canonical key spellings of the keymap", &["terminal_bytes_become_the_keymaps_canonical_spellings"]),
    ]);
}
