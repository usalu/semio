use super::*;

fn ui_text(value: &str) -> UiText {
    UiText::try_from_str(value).expect("bounded fixture text")
}

fn ui_list(values: impl IntoIterator<Item = UiValue>) -> UiList {
    let mut builder = UiListBuilder::try_new().expect("fixed list builder");
    for value in values {
        builder.push(value).expect("fixed list page");
    }
    builder.finish()
}

fn ui_map(entries: impl IntoIterator<Item = (String, UiValue)>) -> UiMap {
    let mut entries: Vec<_> = entries.into_iter().collect();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut builder = UiMapBuilder::try_new().expect("fixed map builder");
    for (key, value) in entries {
        builder.push(key, value).expect("fixed map page");
    }
    builder.finish()
}

#[test]
fn action_id_displays_scope_dot_name_at_version() {
    let id = ActionId::try_v1("cad-play", "objectMove").expect("bounded action id");
    assert_eq!(id.to_string(), "cad-play.objectMove@1");
    assert_eq!(ActionId::new(ui_text("app"), ui_text("submit"), 3).to_string(), "app.submit@3");
}

#[test]
fn fixed_owners_keep_bounded_payloads_off_the_stack() {
    assert!(size_of::<UiFixedBytes>() <= size_of::<usize>() * 3);
    assert_eq!(size_of::<UiFixedList<UiFixedBytes>>(), size_of::<usize>() * 6);
    assert_eq!(size_of::<UiFixedList<[u8; 32769], 1>>(), size_of::<UiFixedList<u8, 65536>>());
    assert!(size_of::<UiValueArena>() <= size_of::<usize>() * 24);
}

#[test]
fn fixed_bytes_admit_the_scene_packet_census_exactly() {
    let admitted = vec![7; UI_FIXED_BYTES];
    assert_eq!(UiFixedBytes::try_from_vec(admitted).expect("exact scene packet census").len(), UI_FIXED_BYTES);
    let rejected = vec![7; UI_FIXED_BYTES + 1];
    assert_eq!(UiFixedBytes::try_from_vec(rejected).expect_err("scene packet over census").len(), UI_FIXED_BYTES + 1);
}

#[allow(clippy::needless_pass_by_value)]
fn value_round_trips(value: UiValue) {
    let first = serde_json::to_string(&value).expect("serialize");
    let deserialized: UiValue = serde_json::from_str(&first).expect("deserialize");
    let second = serde_json::to_string(&deserialized).expect("re-serialize");
    assert_eq!(first, second);
    assert_eq!(value, deserialized);
}

#[test]
fn every_ui_value_shape_round_trips() {
    value_round_trips(UiValue::Null);
    value_round_trips(UiValue::Bool(true));
    value_round_trips(UiValue::Number(-3.5));
    value_round_trips(UiValue::Text(ui_text("hi")));
    value_round_trips(UiValue::List(ui_list([UiValue::Number(1.0), UiValue::Text(ui_text("two"))])));
    value_round_trips(UiValue::Map(ui_map([("id".to_string(), UiValue::Text(ui_text("widget"))), ("nested".to_string(), UiValue::List(ui_list([UiValue::Bool(false), UiValue::Null])))])));
}

#[test]
fn ui_value_default_is_null() {
    assert_eq!(UiValue::default(), UiValue::Null);
}

#[test]
fn action_binding_round_trips_with_and_without_args() {
    let full = ActionBinding { trigger: Trigger::Change, action: ActionId::try_v1("app", "setValue").expect("bounded action id"), args: Some(UiValue::Text(ui_text("scope"))), capability: Some(ui_text("edit")) };
    let first = serde_json::to_string(&full).expect("serialize");
    let back: ActionBinding = serde_json::from_str(&first).expect("deserialize");
    assert_eq!(full, back);

    let minimal = ActionBinding::default();
    let json = serde_json::to_value(&minimal).expect("serialize");
    assert!(json.get("args").is_none());
    assert!(json.get("capability").is_none());
}

#[test]
fn menu_ref_round_trips() {
    let menu = MenuRef { id: ui_text("context.tree-item"), args: Some(UiValue::Number(2.0)) };
    let first = serde_json::to_string(&menu).expect("serialize");
    let back: MenuRef = serde_json::from_str(&first).expect("deserialize");
    assert_eq!(menu, back);
}

#[test]
fn ui_intent_round_trips() {
    let intent = UiIntent {
        surface: crate::SurfaceId::try_from("note.play.navigator").expect("bounded surface id"),
        revision: crate::UiRevision(4),
        node: crate::UiNodeId(9),
        node_key: ui_text("row-9"),
        trigger: Trigger::Delta,
        action: ActionId::try_v1("cad-play", "objectMove").expect("bounded action id"),
        args: Some(UiValue::Number(1.0)),
        input: Some(UiValue::Number(-2.0)),
        seq: 42,
    };
    let first = serde_json::to_string(&intent).expect("serialize");
    let deserialized: UiIntent = serde_json::from_str(&first).expect("deserialize");
    let second = serde_json::to_string(&deserialized).expect("re-serialize");
    assert_eq!(first, second);
    assert_eq!(intent, deserialized);
}

#[test]
fn every_trigger_variant_round_trips() {
    for trigger in [Trigger::Activate, Trigger::Change, Trigger::Commit, Trigger::Delta, Trigger::Drop, Trigger::Submit, Trigger::Abort, Trigger::RepeatLast, Trigger::HoverPreview] {
        let json = serde_json::to_string(&trigger).expect("serialize");
        let back: Trigger = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(trigger, back);
    }
}

#[test]
fn fixed_page_max_plus_one_returns_the_exact_untransferred_owner() {
    let mut arena = UiValueArena::default();
    let handle = arena.reserve_collection(UiCollectionKind::List).expect("fixed list authority");
    for index in 0..UI_VALUE_AGGREGATE_ITEMS {
        arena.try_push_page(handle, UiPageValue::List(UiValue::Number(index as f64))).expect("fixed page admission");
    }
    let refused = UiPageValue::List(UiValue::Text(ui_text("exact-refusal-owner")));
    let refused = arena.try_push_page(handle, refused).expect_err("maximum plus one must refuse before transfer");
    assert!(matches!(refused, UiPageValue::List(UiValue::Text(text)) if text.as_str() == "exact-refusal-owner"));
}

#[test]
fn ascending_map_duplicate_refusal_preserves_key_and_value() {
    let mut builder = UiMapBuilder::try_new().expect("fixed map builder");
    builder.push("a".to_string(), UiValue::Number(1.0)).expect("first key");
    let (key, value) = builder.push("a".to_string(), UiValue::Text(ui_text("owner"))).expect_err("duplicate key must be inert");
    assert_eq!(key, "a");
    assert_eq!(value, UiValue::Text(ui_text("owner")));
}

#[test]
fn credited_alias_keeps_pages_live_after_original_handle_is_lost() {
    let original = ui_list([UiValue::Text(ui_text("retained"))]);
    let handle = original.handle.unwrap();
    let alias = original.credited_clone().expect("credited alias slot");
    drop(original);
    assert_eq!(alias.cursor().next(), Some(UiValue::Text(ui_text("retained"))));
    drop(alias);
    while !close_ui_value_page_one() {}
    assert!(with_ui_value_arena(|arena| arena.collection(handle).is_none()));
}

#[test]
fn poisoned_arena_lock_recovers_without_losing_fixed_authority() {
    let _ = std::panic::catch_unwind(|| {
        let _guard = UI_VALUE_ARENA.lock().expect("unpoisoned fixture entry");
        panic!("poison fixed arena");
    });
    let list = ui_list([UiValue::Bool(true)]);
    assert_eq!(list.cursor().next(), Some(UiValue::Bool(true)));
}

#[test]
fn arena_initialization_is_a_fixed_control_and_page_taxonomy() {
    let started = std::time::Instant::now();
    let arena = UiValueArena::default();
    assert_eq!(arena.free_page_count, UI_VALUE_AGGREGATE_ITEMS);
    assert_eq!(arena.free_collection_count, UI_VALUE_ADMISSION_SLOTS);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
    eprintln!(
        "[DEBUG] arena pages={UI_VALUE_AGGREGATE_ITEMS} page-bytes={} collections={UI_VALUE_ADMISSION_SLOTS} collection-bytes={} aggregate-bytes={UI_VALUE_AGGREGATE_BYTES} backing-bytes={} elapsed-ms={}",
        size_of::<UiPageSlot>(),
        size_of::<UiCollectionSlot>(),
        resident_static_backing_bytes(),
        started.elapsed().as_millis()
    );
}

#[test]
fn ui_text_clipped_keeps_short_values_and_marks_long_ones_on_a_char_boundary() {
    assert_eq!(UiText::clipped("set-count value=1").as_str(), "set-count value=1");
    let exact = "x".repeat(UI_TEXT_MAX_BYTES);
    assert_eq!(UiText::clipped(&exact).as_str(), exact);
    let long = "é".repeat(UI_TEXT_MAX_BYTES);
    let clipped = UiText::clipped(&long);
    assert!(clipped.len() <= UI_TEXT_MAX_BYTES);
    assert!(clipped.as_str().ends_with(UI_TEXT_CLIP_MARK));
    assert_eq!(clipped.len(), UI_TEXT_MAX_BYTES - UI_TEXT_CLIP_MARK.len() - 1 + UI_TEXT_CLIP_MARK.len());
    assert!(clipped.as_str().trim_end_matches(UI_TEXT_CLIP_MARK).chars().all(|c| c == 'é'));
}
