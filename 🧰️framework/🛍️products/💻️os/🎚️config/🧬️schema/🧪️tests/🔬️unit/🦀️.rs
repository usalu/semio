use super::*;

#[test]
fn opening_preferences_default_is_empty() {
    assert_eq!(OpeningPreferences::default(), OpeningPreferences { defaults: Vec::new() });
}

//#region 🌓️AppearanceBoot
/// 🧫️ The ONE language-agnostic statement of what the pre-paint appearance bootstrap must resolve.
/// The TypeScript half EXECUTES the shipped inline `<script>` string against a throwaway DOM
/// (`🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts`); this half replays the very same event logs through the
/// OS config projection that owns the vocabulary, so a boot script that agreed with itself but not
/// with the shell would still fail.
const APPEARANCE_BOOT_FIXTURE_JSON: &str = include_str!("../../../../../../🔨️modules/🖱️ui/🎨️styling/🧫️fixtures/🌓️appearance-boot.json");

/// ⚖️ LAW: replaying a case's `events` through `apply_ui_preferences_config_mutation` yields the
/// `appearance` the fixture declares — the projection the head script has to reproduce by hand.
///
/// The head script cannot import this code: it is an inline `<script>` that runs before any module
/// is fetched. That is precisely why the two must answer ONE fixture — the boot path used to read a
/// storage key nothing had written for a whole shell generation, and nothing in either implementation
/// could notice (`📓️react-i18n-a11y-customization-2026-09-13.md` §3.2).
#[test]
fn every_appearance_boot_case_replays_to_the_appearance_the_fixture_declares() {
    let fixture: serde_json::Value = serde_json::from_str(APPEARANCE_BOOT_FIXTURE_JSON).expect("appearance boot fixture");
    assert_eq!(fixture["preferenceKey"].as_str(), Some(UI_PREFERENCES_CONFIG_SCHEMA), "the fixture names the preference key this facet is registered under");
    let mut replayed = 0usize;
    for case in fixture["cases"].as_array().expect("cases") {
        if case["skipProjection"].as_bool() == Some(true) {
            continue;
        }
        let name = case["name"].as_str().expect("case name");
        let mut preferences = UiPreferences::default();
        let events = case["storage"]["preferences"][UI_PREFERENCES_CONFIG_SCHEMA]["events"].as_array().cloned().unwrap_or_default();
        for event in &events {
            let mutation = decode_ui_preferences_config_mutation_json(&event.to_string()).unwrap_or_else(|error| panic!("{name}: {event} is not a declared ui-preferences mutation: {error}"));
            apply_ui_preferences_config_mutation(&mut preferences, &mutation).unwrap_or_else(|error| panic!("{name}: replay failed: {error:?}"));
        }
        let declared = case["appearance"].as_str();
        let projected = match preferences.appearance {
            Some(UiAppearance::System) => Some("system"),
            Some(UiAppearance::Light) => Some("light"),
            Some(UiAppearance::Dark) => Some("dark"),
            None => None,
        };
        assert_eq!(projected, declared, "{name}: the replayed appearance");
        replayed += 1;
    }
    assert!(replayed >= 5, "the fixture must carry real replays: {replayed}");
    eprintln!("[DEBUG] appearance boot replayed {replayed} cases through the OS config projection");
}

/// ⚖️ LAW: every storage key the fixture retires is named by no mutation this facet declares — the
/// fixture's `deadKeys` are dead because the vocabulary moved, not because someone renamed a string.
#[test]
fn no_retired_storage_key_is_part_of_the_ui_preferences_vocabulary() {
    let fixture: serde_json::Value = serde_json::from_str(APPEARANCE_BOOT_FIXTURE_JSON).expect("appearance boot fixture");
    let dead: Vec<&str> = fixture["deadKeys"].as_array().expect("deadKeys").iter().map(|key| key.as_str().expect("dead key")).collect();
    assert!(!dead.is_empty(), "the fixture must name the keys it retires");
    for key in dead {
        assert_ne!(key, UI_PREFERENCES_CONFIG_SCHEMA, "{key} is this facet's own schema id, not a retired key");
        assert!(!key.starts_with("os.config."), "{key} looks like a live OS config schema id");
    }
}
//#endregion 🌓️AppearanceBoot
