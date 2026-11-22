//! ⌨️ Laws for `mod` and override reach against the fixture the React table reads (`🧪️tests/⌨️mod-platform-and-override-reach`).

use super::*;
use std::collections::HashMap;
use semio_renderer_wgpu_host_base::{compiled_host_platform, set_host_platform, set_host_platform_kind, set_host_platform_uses_meta};
use semio_renderer_wgpu_shell_chrome_state::keybinding_override;
use semio_renderer_wgpu_shell_docs::key_event_matches_chord;

fn mod_platform_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/⌨️mod-platform-and-override-reach/🔣️.json")).expect("mod platform fixture")
}

/// ⚖️ LAW: `mod` is meta on Apple and ctrl elsewhere on the host platform, and fires exactly the rows React's `parseOwnedHotkeyChords` fires.
#[test]
fn mod_resolves_from_the_host_platform_exactly_as_react() {
    let fixture = mod_platform_fixture();
    for platform in fixture["platforms"].as_array().expect("platforms") {
        set_host_platform(platform["platform"].as_str().expect("platform"));
        for row in fixture["modChords"].as_array().expect("rows") {
            let event = &row["event"];
            let modifiers = PointerModifiers { ctrl: event["ctrl"].as_bool().expect("ctrl"), meta: event["meta"].as_bool().expect("meta"), shift: event["shift"].as_bool().expect("shift"), alt: event["alt"].as_bool().expect("alt"), repeat: false };
            let action = KeyAction::Char(event["key"].as_str().expect("key").to_string());
            let expected = row["matches"][platform["id"].as_str().expect("id")].as_bool().expect("expected");
            assert_eq!(key_event_matches_chord(&action, &modifiers, row["chord"].as_str().expect("chord")), expected, "{} on {}", row["id"], platform["id"]);
        }
    }
    set_host_platform_uses_meta(cfg!(target_os = "macos"));
    set_host_platform_kind(compiled_host_platform());
}

/// ⚖️ LAW: an override applies to any control id (shell row, app action id, OS command id) and a blank one is dropped, as React's composed map and storage gate do.
#[test]
fn overrides_reach_any_control_id_and_drop_blank_values() {
    let fixture = mod_platform_fixture();
    let reach = &fixture["overrideReach"];
    let stored: HashMap<String, String> = reach["stored"].as_object().expect("stored").iter().map(|(id, keys)| (id.clone(), keys.as_str().expect("keys").to_string())).collect();
    for (id, keys) in reach["resolved"].as_object().expect("resolved") {
        let table = shell_shortcut_table(&stored);
        let resolved = keybinding_override(&stored, id).map(str::to_string).or_else(|| table.iter().find(|(row, _)| row == id).map(|(_, keys)| keys.clone()));
        assert_eq!(resolved.as_deref(), keys.as_str(), "{id}");
    }
}
