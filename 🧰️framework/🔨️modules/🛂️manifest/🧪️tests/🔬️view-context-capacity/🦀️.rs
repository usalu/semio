//! 📏️ The surface view context's capacities, pinned against the language-neutral schema the
//! TypeScript mirror (`parseResolvedPluginViewState` in `🛂️manifest/🟦️.ts`) and its Ajv oracle
//! validate against — neither side may drift from `🪟️view-context/🧬️schema/🔣️.json`.

use super::{
    MAX_SURFACE_BODY_KEY_BYTES, MAX_SURFACE_VIEW_CONTEXT_BYTES, VIEW_CONTEXT_IDENTIFIER_CHARS, VIEW_CONTEXT_IDENTIFIER_FIELDS, VIEW_CONTEXT_LONG_STRING_CHARS, VIEW_CONTEXT_LONG_STRING_FIELDS,
    VIEW_CONTEXT_UTILITY_ENTRIES, VIEW_CONTEXT_WINDOW_INSTANCES,
};

fn schema() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🪟️view-context/🧬️schema/🔣️.json")).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn capacities_match_the_neutral_schema() {
    let schema = schema();
    let properties = &schema["properties"];
    assert_eq!(schema["$defs"]["Identifier"]["maxLength"].as_u64().unwrap() as usize, VIEW_CONTEXT_IDENTIFIER_CHARS);
    assert_eq!(properties["panelJson"]["maxLength"].as_u64().unwrap() as usize, VIEW_CONTEXT_LONG_STRING_CHARS);
    assert_eq!(properties["contributionsJson"]["maxLength"].as_u64().unwrap() as usize, VIEW_CONTEXT_LONG_STRING_CHARS);
    assert_eq!(properties["activeUtilityByWindowId"]["maxProperties"].as_u64().unwrap() as usize, VIEW_CONTEXT_UTILITY_ENTRIES);
    assert_eq!(properties["windowInstances"]["maxItems"].as_u64().unwrap() as usize, VIEW_CONTEXT_WINDOW_INSTANCES);

    let identifier_fields = properties.as_object().unwrap().values().filter(|field| field["$ref"].as_str() == Some("#/$defs/Identifier")).count();
    assert_eq!(identifier_fields, VIEW_CONTEXT_IDENTIFIER_FIELDS);
    let long_string_fields = properties.as_object().unwrap().values().filter(|field| field["maxLength"].as_u64() == Some(VIEW_CONTEXT_LONG_STRING_CHARS as u64)).count();
    assert_eq!(long_string_fields, VIEW_CONTEXT_LONG_STRING_FIELDS);
}

#[semio_framework_async_macros::async_test]
async fn the_admission_bound_covers_every_schema_valid_context() {
    let content_chars = VIEW_CONTEXT_IDENTIFIER_FIELDS * VIEW_CONTEXT_IDENTIFIER_CHARS
        + VIEW_CONTEXT_LONG_STRING_FIELDS * VIEW_CONTEXT_LONG_STRING_CHARS
        + VIEW_CONTEXT_UTILITY_ENTRIES * 2 * VIEW_CONTEXT_IDENTIFIER_CHARS
        + VIEW_CONTEXT_WINDOW_INSTANCES * 2 * VIEW_CONTEXT_IDENTIFIER_CHARS;
    assert!(MAX_SURFACE_VIEW_CONTEXT_BYTES >= content_chars * 4, "the bound must admit the schema's own characters at worst-case UTF-8 width");
    assert!(MAX_SURFACE_BODY_KEY_BYTES >= VIEW_CONTEXT_IDENTIFIER_CHARS * 4, "a body key is Identifier-shaped");
    assert!(MAX_SURFACE_VIEW_CONTEXT_BYTES > 262_144, "the borrowed public-action body cap rejected schema-valid contexts");
}

/// 📏️ The measured worst case: a view context filled to every schema capacity must fit the
/// admission bound. Measured as JSON, which is never smaller than the pack wire the shell's
/// `encodePackValue` produces for the same value (JSON repeats every field name as quoted text and
/// escapes payload bytes), so passing here proves the pack-encoded context passes too.
#[semio_framework_async_macros::async_test]
async fn a_capacity_filled_context_fits_the_bound() {
    use crate::{Locale, Terminology, ViewModel, ViewWindowInstance};
    let identifier = || "i".repeat(VIEW_CONTEXT_IDENTIFIER_CHARS);
    let long = || "l".repeat(VIEW_CONTEXT_LONG_STRING_CHARS);
    let view = ViewModel {
        active_mode_id: Some(identifier()),
        active_window_kind_id: Some(identifier()),
        active_utility_id: Some(identifier()),
        active_utility_by_window_id: (0..VIEW_CONTEXT_UTILITY_ENTRIES).map(|index| (format!("{index}{}", identifier()), identifier())).collect(),
        active_tool_id: Some(identifier()),
        panel_json: Some(long()),
        contributions_json: Some(long()),
        locale: Locale::En,
        terminology: Terminology::Native,
        window_id: Some(identifier()),
        window_instances: (0..VIEW_CONTEXT_WINDOW_INSTANCES).map(|index| ViewWindowInstance { id: format!("{index}{}", identifier()), window_kind_id: identifier() }).collect(),
    };
    let encoded = serde_json::to_vec(&view).unwrap();
    println!("[STATS] capacity-filled view context encoded={} bound={}", encoded.len(), MAX_SURFACE_VIEW_CONTEXT_BYTES);
    assert!(encoded.len() <= MAX_SURFACE_VIEW_CONTEXT_BYTES, "encoded {} exceeds bound {MAX_SURFACE_VIEW_CONTEXT_BYTES}", encoded.len());
}
