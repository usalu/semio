//! 🛍️ Laws for the packed-text carrier's `data_attributes` map — the ONE map shape the app-static
//! catalogue document is made of. `semio_framework_plugin::app::section_text_chunks` packs
//! `1 + UI_FIXED_LIST_ITEMS` slices of `UI_TEXT_MAX_BYTES` into one `Component::Text`: the first as
//! `TextProps.value`, the rest as `data_attributes` keyed `"01".."32"`.
//!
//! 🚨️ A `UiFixedMap` is a SORTED map, but its wire form is a JSON object, and JSON object key order
//! carries no meaning. The wgpu bridge's `renderDocument` hands the guest document to `JSON.stringify`
//! (`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`), and every ECMAScript engine emits array-index-like
//! keys (`"10".."32"`) first in numeric order and the remaining string keys (`"01".."09"`) after — so
//! the object arrives out of ascending order. Decoding must reconstruct the sorted map from ANY key
//! order, exactly like the React renderer's reader already does (`🧳️packed-text/🟦️.ts` sorts).
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

use semio_framework_ui_contract::{Component, Label, TextProps, UiFixedMap, UiMap, UiText, UiValue, UI_FIXED_LIST_ITEMS, UI_VALUE_MAX_ITEMS};

const FIXTURE: &str = include_str!("../../🧫️fixtures/🛍️catalogue-carrier-map.json");

/// 🧫️ The pinned carrier leaf: the label slice, the `data_attributes` object exactly as a JavaScript
/// engine emits it, the key order it emits, and the payload the readers must recover.
struct Fixture {
    value: String,
    ascending_keys: Vec<String>,
    javascript_keys: Vec<String>,
    data_attributes_json: String,
    payload: String,
}

fn fixture() -> Fixture {
    let parsed: serde_json::Value = serde_json::from_str(FIXTURE).expect("carrier map fixture parses");
    let strings = |key: &str| parsed[key].as_array().expect("fixture key list").iter().map(|entry| entry.as_str().expect("fixture key").to_string()).collect::<Vec<_>>();
    let text = |key: &str| parsed[key].as_str().expect("fixture text").to_string();
    Fixture { value: text("value"), ascending_keys: strings("ascendingKeys"), javascript_keys: strings("javascriptKeys"), data_attributes_json: text("dataAttributesJson"), payload: text("payload") }
}

/// 📜️ The fixture is only meaningful while a JavaScript engine really does reorder these keys — the
/// generator refuses to write one that does not, and this pins the reordering the decoder must absorb.
#[test]
fn the_catalogue_carrier_fixture_pins_a_reordered_javascript_object() {
    let fixture = fixture();
    assert_eq!(fixture.ascending_keys.len(), UI_FIXED_LIST_ITEMS, "the carrier packs exactly one full map");
    assert_eq!(fixture.javascript_keys.len(), UI_FIXED_LIST_ITEMS, "reordering never drops an entry");
    assert_ne!(fixture.javascript_keys, fixture.ascending_keys, "a fixture that is already ascending would be vacuous");
    let mut sorted = fixture.javascript_keys.clone();
    sorted.sort();
    assert_eq!(sorted, fixture.ascending_keys, "the same keys, in a different order");
    println!("[DEBUG] catalogue-carrier-map javascript-order first={} last={}", fixture.javascript_keys[0], fixture.javascript_keys[UI_FIXED_LIST_ITEMS - 1]);
}

/// 📜️ The defect: the wgpu shell refused the whole catalogue document with `UiFixedMap requires at
/// most 32 ascending unique entries` because this object does not arrive ascending.
#[test]
fn a_javascript_ordered_carrier_map_decodes_into_the_sorted_map() {
    let fixture = fixture();
    let map: UiFixedMap<UiText> = serde_json::from_str(&fixture.data_attributes_json).unwrap_or_else(|error| panic!("a carrier map must decode from any JSON object key order, failed at column {}: {error}", error.column()));
    assert_eq!(map.len(), UI_FIXED_LIST_ITEMS, "every entry is admitted");
    let keys: Vec<String> = map.iter().map(|(key, _)| key.as_str().to_string()).collect();
    assert_eq!(keys, fixture.ascending_keys, "a decoded fixed map is sorted regardless of wire order");
    println!("[DEBUG] catalogue-carrier-map decoded entries={}", map.len());
}

/// 📜️ The same wire shape embedded in `UiValue::Map` must decode in JavaScript key order — action args
/// and extension props use `UiMap`, not `UiFixedMap`.
#[test]
fn a_javascript_ordered_ui_map_decodes_into_the_sorted_map() {
    let fixture = fixture();
    let map: UiMap = serde_json::from_str(&fixture.data_attributes_json).unwrap_or_else(|error| panic!("a UiMap must decode from any JSON object key order, failed at column {}: {error}", error.column()));
    assert_eq!(map.len(), UI_FIXED_LIST_ITEMS, "every entry is admitted");
    let keys: Vec<String> = map.iter().map(|(key, _)| key.as_str().to_string()).collect();
    assert_eq!(keys, fixture.ascending_keys, "a decoded UiMap is sorted regardless of wire order");
    println!("[DEBUG] catalogue-carrier-map ui-map decoded entries={}", map.len());
}

/// 📜️ …and each value round-trips as the text slice the carrier stores.
#[test]
fn a_javascript_ordered_ui_map_preserves_text_values() {
    let fixture = fixture();
    let parsed: serde_json::Value = serde_json::from_str(&fixture.data_attributes_json).expect("fixture object parses");
    let map: UiMap = serde_json::from_str(&fixture.data_attributes_json).expect("UiMap decodes");
    for key in &fixture.ascending_keys {
        let (_, value) = map.iter().find(|(entry_key, _)| entry_key.as_str() == key).expect("sorted map carries every key");
        let expected = parsed[key].as_str().expect("fixture entry");
        assert_eq!(value, &UiValue::Text(UiText::try_from_str(expected).expect("bounded fixture text")));
    }
    println!("[DEBUG] catalogue-carrier-map ui-map value-check keys={}", fixture.ascending_keys.len());
}

/// 📜️ A duplicate key is still a refusal for `UiMap`.
#[test]
fn a_ui_map_still_refuses_duplicate_keys() {
    let duplicated = r#"{"02":"b","01":"a","02":"c"}"#;
    let error = serde_json::from_str::<UiMap>(duplicated).expect_err("a duplicate key must be refused");
    assert!(error.to_string().contains("unique"), "the decoder names duplication: {error}");
    println!("[DEBUG] catalogue-carrier-map ui-map duplicate-refusal {error}");
}

/// 📜️ …and so is one entry past capacity, whatever order it arrives in.
#[test]
fn a_ui_map_still_refuses_one_entry_past_capacity() {
    let entries: Vec<String> = (0..=UI_VALUE_MAX_ITEMS).rev().map(|index| format!(r#""{index:03}":"v{index}""#)).collect();
    let overflowing = format!("{{{}}}", entries.join(","));
    let error = serde_json::from_str::<UiMap>(&overflowing).expect_err("one entry past capacity must be refused");
    assert!(error.to_string().contains("capacity"), "the decoder surfaces capacity: {error}");
    println!("[DEBUG] catalogue-carrier-map ui-map capacity-refusal entries={} {error}", UI_VALUE_MAX_ITEMS + 1);
}

/// 📜️ …and the whole leaf reassembles byte-for-byte, which is what `read_paged_text_document` and the
/// React renderer's `packedTextLeaf` both depend on.
#[test]
fn a_javascript_ordered_carrier_leaf_recovers_its_payload() {
    let fixture = fixture();
    let attributes: UiFixedMap<UiText> = serde_json::from_str(&fixture.data_attributes_json).expect("carrier map decodes");
    let props = TextProps { value: Label(UiText::try_from_str(&fixture.value).expect("carrier leaf label")), emphasize: None, data_attributes: Some(attributes) };
    assert_eq!(props.packed_payload(), fixture.payload, "the carrier leaf must recover the catalogue payload byte-for-byte");
    println!("[DEBUG] catalogue-carrier-map payload-bytes={}", fixture.payload.len());
}

/// 📜️ A duplicate key is still a refusal — an unordered wire must not silently collapse two entries.
#[test]
fn a_carrier_map_still_refuses_duplicate_keys() {
    let duplicated = r#"{"02":"b","01":"a","02":"c"}"#;
    let error = serde_json::from_str::<UiFixedMap<UiText>>(duplicated).expect_err("a duplicate key must be refused");
    println!("[DEBUG] catalogue-carrier-map duplicate-refusal {error}");
}

/// 📜️ …and so is one entry past capacity, whatever order it arrives in.
#[test]
fn a_carrier_map_still_refuses_one_entry_past_capacity() {
    let entries: Vec<String> = (0..=UI_FIXED_LIST_ITEMS).rev().map(|index| format!(r#""{index:02}":"v{index}""#)).collect();
    let overflowing = format!("{{{}}}", entries.join(","));
    let error = serde_json::from_str::<UiFixedMap<UiText>>(&overflowing).expect_err("one entry past capacity must be refused");
    println!("[DEBUG] catalogue-carrier-map capacity-refusal entries={} {error}", UI_FIXED_LIST_ITEMS + 1);
}

/// 📜️ A refusal must NAME its cause. The two are different defects with different owners — a producer
/// publishing more than `UI_FIXED_LIST_ITEMS` keys, versus the aggregate page grant running out
/// mid-document — and the wgpu bridge surfaces this text verbatim inside `renderDocument result parse
/// failed`, where it is the only evidence a reader gets. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
#[test]
fn a_carrier_map_refusal_names_capacity_and_page_grant_apart() {
    let mut full: UiFixedMap<UiText> = UiFixedMap::default();
    for index in 0..UI_FIXED_LIST_ITEMS {
        full.try_insert(UiText::try_from_str(&format!("{index:02}")).expect("bounded key"), UiText::try_from_str("v").expect("bounded value")).expect("a map admits its whole capacity");
    }
    assert_eq!(full.len(), UI_FIXED_LIST_ITEMS);
    let at_capacity = full.refusal();
    assert!(at_capacity.contains("capacity"), "a full map says so: {at_capacity}");
    assert!(!at_capacity.contains("page grant"), "a full map is not a budget refusal: {at_capacity}");

    let mut partial: UiFixedMap<UiText> = UiFixedMap::default();
    partial.try_insert(UiText::try_from_str("00").expect("bounded key"), UiText::try_from_str("v").expect("bounded value")).expect("a fresh map admits one entry");
    let below_capacity = partial.refusal();
    assert!(below_capacity.contains("page grant"), "a map refused below capacity names the grant: {below_capacity}");
    assert!(below_capacity.contains("entry 2"), "it names WHICH entry was refused: {below_capacity}");

    let entries: Vec<String> = (0..=UI_FIXED_LIST_ITEMS).rev().map(|index| format!(r#""{index:02}":"v{index}""#)).collect();
    let error = serde_json::from_str::<UiFixedMap<UiText>>(&format!("{{{}}}", entries.join(","))).expect_err("one entry past capacity must be refused");
    assert!(error.to_string().contains("capacity"), "the decoder surfaces the same vocabulary: {error}");
    println!("[DEBUG] catalogue-carrier-map refusal vocabulary: capacity={at_capacity:?} grant={below_capacity:?}");
}

/// 📜️ The `FromValue` decoder is the same wire in the pack codec the guest speaks, so it carries the
/// same law — a document that survives `serde_json` must survive it too.
#[test]
fn a_javascript_ordered_carrier_map_decodes_through_from_value() {
    let fixture = fixture();
    let value: serde_json::Value = serde_json::from_str(&fixture.data_attributes_json).expect("fixture object parses");
    let mut entries = Vec::new();
    for key in &fixture.javascript_keys {
        entries.push((key.clone(), protocol::value::DslValue::String(value[key].as_str().expect("fixture entry").to_string())));
    }
    let map = <UiFixedMap<UiText> as protocol::value::FromValue>::from_value(protocol::value::DslValue::Object(entries)).unwrap_or_else(|error| panic!("a carrier map must decode from any pack object order: {error:?}"));
    let keys: Vec<String> = map.iter().map(|(key, _)| key.as_str().to_string()).collect();
    assert_eq!(keys, fixture.ascending_keys, "a decoded fixed map is sorted regardless of pack order");
    println!("[DEBUG] catalogue-carrier-map from-value entries={}", map.len());
}

/// 📜️ The producer's own ascending path is unchanged — a full pack built the way
/// `section_text_chunks` builds it still round-trips through the renderer's reader.
#[test]
fn an_ascending_carrier_leaf_still_round_trips_through_json() {
    let fixture = fixture();
    let attributes: UiFixedMap<UiText> = serde_json::from_str(&fixture.data_attributes_json).expect("carrier map decodes");
    let props = TextProps { value: Label(UiText::try_from_str(&fixture.value).expect("carrier leaf label")), emphasize: None, data_attributes: Some(attributes) };
    let text = serde_json::to_string(&Component::Text(props.clone())).expect("carrier leaf serializes");
    let read: Component = serde_json::from_str(&text).expect("carrier leaf round-trips");
    assert_eq!(read, Component::Text(props), "an ascending carrier leaf survives the reader byte-for-byte");
    println!("[DEBUG] catalogue-carrier-map ascending-round-trip json-bytes={}", text.len());
}
