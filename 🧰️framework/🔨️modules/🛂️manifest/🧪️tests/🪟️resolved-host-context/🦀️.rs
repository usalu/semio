//! 🗣️ The GUEST half of the resolved host context, driven by the SAME language-agnostic
//! `🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json` the TypeScript admission twin
//! (`🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts`) reads.
//!
//! The fixture's `guestDecode` rows pin the exact fault a plugin raises for a context that never
//! passed through `parseResolvedPluginViewState`. Before this law only the host side was pinned, so
//! a host path that dispatched a raw `ActiveSession.viewState` (`{ activeModeId }`, no preferences)
//! produced the anonymous `missing field `locale`` app fault with nothing naming the crossing —
//! every deferred `flowEvalTick` re-arm in the browser
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use super::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostContextFixture {
    valid: serde_json::Value,
    invalid: Vec<InvalidRow>,
    guest_decode: Vec<GuestDecodeRow>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InvalidRow {
    name: String,
    #[serde(default)]
    remove: Vec<String>,
    #[serde(default)]
    set: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    repeat: Option<RepeatRow>,
}

/// 📏️ A capacity row the fixture states rather than inlines — a 65 537-character literal would make
/// the shared fixture unreadable in both twins.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RepeatRow {
    field: String,
    character: String,
    count: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GuestDecodeRow {
    name: String,
    fault: String,
}

fn fixture() -> HostContextFixture {
    serde_json::from_str(include_str!("../../🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json")).expect("the resolved-host-context fixture must parse")
}

fn row_json(valid: &serde_json::Value, row: &InvalidRow) -> String {
    let mut object = valid.as_object().expect("the valid context is an object").clone();
    for key in &row.remove {
        object.remove(key);
    }
    for (key, value) in &row.set {
        object.insert(key.clone(), value.clone());
    }
    if let Some(repeat) = &row.repeat {
        object.insert(repeat.field.clone(), serde_json::Value::String(repeat.character.repeat(repeat.count)));
    }
    serde_json::Value::Object(object).to_string()
}

#[semio_framework_async_macros::async_test]
async fn the_resolved_context_decodes_in_the_guest() {
    let fixture = fixture();
    let view: ViewModel = dsl::os_pack::json::from_json_str(&fixture.valid.to_string()).expect("a resolved host context decodes");
    assert_eq!(view.locale, Locale::De);
    assert_eq!(view.terminology, Terminology::Reuse);
    assert_eq!(view.window_instances.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn an_unresolved_context_names_the_field_it_lost() {
    let fixture = fixture();
    assert!(!fixture.guest_decode.is_empty(), "the fixture must pin at least one guest-side rejection");
    for expected in &fixture.guest_decode {
        let row = fixture.invalid.iter().find(|candidate| candidate.name == expected.name).unwrap_or_else(|| panic!("guestDecode row {} must also be an `invalid` row the host admission rejects", expected.name));
        let error = dsl::os_pack::json::from_json_str::<ViewModel>(&row_json(&fixture.valid, row)).err().unwrap_or_else(|| panic!("{} must not decode in the guest", expected.name));
        assert_eq!(error.to_string(), expected.fault, "{}", expected.name);
    }
}

/// ⚖️ LAW: contributions never reach a guest through a view context. The host publisher installs
/// them by the paged `setContributions` run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`) and the guest
/// folds them into its own registry (`🌊️flow/📔️registry/🦀️.rs`), so even a context that smuggles
/// the field decodes into a `ViewModel` that cannot expose it — there is no reader to route.
#[semio_framework_async_macros::async_test]
async fn a_smuggled_contributions_field_reaches_no_guest_reader() {
    let fixture = fixture();
    let row = fixture.invalid.iter().find(|candidate| candidate.name == "contributions-in-view-state").expect("the fixture must pin a contributions row");
    assert_eq!(row.set.keys().collect::<Vec<_>>(), vec!["contributionsJson"], "the row must set exactly the refused field");
    let view: ViewModel = dsl::os_pack::json::from_json_str(&row_json(&fixture.valid, row)).expect("an unknown field is ignored, never routed");
    let reprojected = serde_json::to_string(&view).expect("the guest projection encodes");
    assert!(!reprojected.contains("contributionsJson"), "no guest reader may re-emit contributions from a view context: {reprojected}");
}

/// 📏️ The one long field the contract keeps, pinned at the schema capacity the TypeScript twin
/// asserts against Ajv — `panelJson` at capacity decodes, and one character past it is the row the
/// host admission refuses.
#[semio_framework_async_macros::async_test]
async fn the_panel_capacity_row_is_the_only_long_field() {
    let fixture = fixture();
    let row = fixture.invalid.iter().find(|candidate| candidate.name == "oversized-panel").expect("the fixture must pin a panel capacity row");
    let repeat = row.repeat.as_ref().expect("the capacity row is stated, never inlined");
    assert_eq!(repeat.field, "panelJson");
    assert_eq!(repeat.count, VIEW_CONTEXT_LONG_STRING_CHARS + 1, "the refused row is exactly one character past the schema bound");
    let view: ViewModel = dsl::os_pack::json::from_json_str(&row_json(&fixture.valid, row)).expect("the guest decodes what the host admission refuses");
    assert_eq!(view.panel_json.map(|json| json.chars().count()), Some(repeat.count));
}
