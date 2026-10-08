use crate::apply_mutation;
use super::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn member(key: &str, value: JsonValue) -> JsonMember {
    JsonMember { key: key.to_string(), value }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn number(lexeme: &str) -> JsonValue {
    JsonValue::Number { lexeme: lexeme.to_string() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base() -> JsonSnapshot {
    JsonSnapshot {
        value: JsonValue::Object {
            members: vec![
                member("revision", number("4")),
                member("title", JsonValue::String { value: "hexagonal cut".to_string() }),
                member("tags", JsonValue::Array { items: vec![JsonValue::String { value: "a".to_string() }, JsonValue::String { value: "b".to_string() }] }),
            ],
        },
        ..JsonSnapshot::default()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn key(name: &str) -> JsonPath {
    vec![JsonPathSegment::Key(name.to_string())]
}

/// 📇️ The honesty gate the fleet brief requires: `KINDS` is the enum's own variant list, in
/// declaration order — checked here directly against the enum's own tagged serialization.
#[test]
fn kinds_match_the_enum() {
    let sample = vec![
        JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root: JsonIJsonRoot::Array { items: Vec::new() } }),
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: String::new(), value: JsonValue::Null, index: None }),
        JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: Vec::new(), key: String::new() }),
        JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: String::new(), to: String::new() }),
        JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: Vec::new(), lexeme: "0".to_string() }),
        JsonIJsonMutation::SetString(set_string::SetString { path: Vec::new(), value: String::new() }),
        JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: Vec::new(), index: 0, value: JsonValue::Null }),
        JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: Vec::new(), index: 0 }),
    ];
    assert_eq!(sample.len(), KINDS.len(), "one sample per declared kind");
    for (mutation, kind) in sample.iter().zip(KINDS) {
        let tag = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(mutation))).expect("serializes")["mutation"].as_str().expect("the internally-tagged variant name").to_string();
        let kebab = kind.split('-').enumerate().map(|(index, part)| if index == 0 { part.to_string() } else { format!("{}{}", part[..1].to_uppercase(), &part[1..]) }).collect::<String>();
        assert_eq!(tag, kebab, "KINDS entry {kind} must name the variant it stands for");
    }
}

#[test]
fn set_top_level_cannot_spell_a_scalar_root() {
    assert!(JsonIJsonRoot::from_value(&JsonValue::String { value: "bare".to_string() }).is_none());
    assert!(JsonIJsonRoot::from_value(&number("1")).is_none());
    assert!(JsonIJsonRoot::from_value(&JsonValue::Array { items: Vec::new() }).is_some());
}

#[test]
fn set_safe_number_at_the_boundary_is_accepted() {
    let mut snapshot = base();
    let outcome = apply_mutation(&mut snapshot, &JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: key("revision"), lexeme: "9007199254740991".to_string() }));
    assert!(outcome.messages().is_empty(), "got {:?}", outcome.messages());
    assert_eq!(resolve(&snapshot.value, &key("revision")), Some(&number("9007199254740991")));
}

#[test]
fn set_safe_number_one_past_the_boundary_is_refused_and_never_applied() {
    let mut snapshot = base();
    let outcome = apply_mutation(&mut snapshot, &JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: key("revision"), lexeme: "9007199254740992".to_string() }));
    assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_INVARIANT), "got {:?}", outcome.messages());
    assert_eq!(resolve(&snapshot.value, &key("revision")), Some(&number("4")), "a refused edit must leave the snapshot untouched");
}

#[test]
fn a_fractional_lexeme_is_outside_the_safe_integer_clause() {
    assert!(is_safe_number_lexeme("9007199254740993.5"));
    assert!(is_safe_number_lexeme("4.44089209850063e-16"));
    assert!(!is_safe_number_lexeme("-9007199254740993"));
    assert!(!is_safe_number_lexeme("100000000000000000000000000000"));
}

#[test]
fn set_string_refuses_a_unicode_noncharacter() {
    let mut snapshot = base();
    let outcome = apply_mutation(&mut snapshot, &JsonIJsonMutation::SetString(set_string::SetString { path: key("title"), value: "before\u{FFFE}after".to_string() }));
    assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_INVARIANT), "got {:?}", outcome.messages());
    assert_eq!(resolve(&snapshot.value, &key("title")), Some(&JsonValue::String { value: "hexagonal cut".to_string() }));
}

#[test]
fn rename_member_is_atomic_and_position_preserving() {
    let mut snapshot = base();
    apply_mutation(&mut snapshot, &JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: "revision".to_string(), to: "version".to_string() }));
    let JsonValue::Object { members } = &snapshot.value else { panic!("root stays an object") };
    assert_eq!(members.iter().map(|member| member.key.as_str()).collect::<Vec<_>>(), vec!["version", "title", "tags"]);
}

#[test]
fn rename_member_onto_an_existing_name_is_refused() {
    let mut snapshot = base();
    let outcome = apply_mutation(&mut snapshot, &JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: "revision".to_string(), to: "title".to_string() }));
    assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_INVARIANT), "got {:?}", outcome.messages());
    let JsonValue::Object { members } = &snapshot.value else { panic!("root stays an object") };
    assert_eq!(members.iter().map(|member| member.key.as_str()).collect::<Vec<_>>(), vec!["revision", "title", "tags"]);
}

/// ↩️ The metamorphic law every `inverse-<kind>` scenario of `🔀️mutate-json-rfc8259-i-json` rests
/// on, proved here for all nine kinds against one snapshot rather than only end-to-end.
#[test]
fn applying_a_mutation_and_then_its_inverse_restores_the_snapshot() {
    let original = base();
    let mutations = vec![
        JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root: JsonIJsonRoot::Array { items: vec![JsonValue::Null] } }),
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "revision".to_string(), value: number("9"), index: None }),
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "fresh".to_string(), value: JsonValue::Null, index: None }),
        JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: Vec::new(), key: "title".to_string() }),
        JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: "title".to_string(), to: "heading".to_string() }),
        JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: key("revision"), lexeme: "9007199254740991".to_string() }),
        JsonIJsonMutation::SetString(set_string::SetString { path: key("title"), value: "Ünïcödé, mit Sonderzeichen".to_string() }),
        JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: key("tags"), index: 1, value: JsonValue::String { value: "inserted".to_string() } }),
        JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: key("tags"), index: 0 }),
    ];
    for mutation in mutations {
        let mut snapshot = original.clone();
        let undo = <JsonIJsonMutation as Mutation<JsonSnapshot>>::inverse(&mutation, &snapshot).expect("valid retained mutation inverse fixture");
        apply_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_mutation(&mut snapshot, step);
        }
        assert_eq!(snapshot, original, "{mutation:?} did not invert cleanly");
    }
}

/// 🧬️ Every inherited verb means exactly what the ✳️any sibling means by it — the claim this
/// leaf's header makes, checked rather than asserted in prose.
#[test]
fn the_four_inherited_verbs_mean_what_their_any_counterparts_mean() {
    let snapshot = base();
    let path = key("tags");
    let lowered = |ijson: JsonIJsonMutation, any: JsonMutation| {
        assert_eq!(<JsonIJsonMutation as Mutation<JsonSnapshot>>::diff(&ijson, &snapshot), <JsonMutation as Mutation<JsonSnapshot>>::diff(&any, &snapshot), "{ijson:?} must mean what {any:?} means");
    };
    lowered(
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "revision".to_string(), value: number("9"), index: None }),
        JsonMutation::SetMember(SetMemberPayload { path: Vec::new(), key: "revision".to_string(), value: number("9"), index: None }),
    );
    lowered(JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: Vec::new(), key: "title".to_string() }), JsonMutation::RemoveMember(RemoveMemberPayload { path: Vec::new(), key: "title".to_string() }));
    lowered(
        JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: path.clone(), index: 1, value: JsonValue::Null }),
        JsonMutation::InsertArrayElement(InsertArrayElementPayload { path: path.clone(), index: 1, value: JsonValue::Null }),
    );
    lowered(JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: path.clone(), index: 0 }), JsonMutation::RemoveArrayElement(RemoveArrayElementPayload { path, index: 0 }));
}

/// ⚖️ `i_json_mutation_inverse_sum_law`: for every leaf the inverse diffs sum to the negative forward diff.
#[semio_framework_async_macros::async_test]
async fn i_json_mutation_inverse_sum_law_holds_for_every_leaf() {
    let base = base();
    for mutation in [
        JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root: JsonIJsonRoot::Array { items: vec![JsonValue::Null] } }),
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "revision".to_string(), value: number("9"), index: None }),
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "fresh".to_string(), value: JsonValue::Null, index: Some(1) }),
        JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: Vec::new(), key: "title".to_string() }),
        JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: "title".to_string(), to: "heading".to_string() }),
        JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: key("revision"), lexeme: "9007199254740991".to_string() }),
        JsonIJsonMutation::SetString(set_string::SetString { path: key("title"), value: "Ünïcödé".to_string() }),
        JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: key("tags"), index: 1, value: JsonValue::String { value: "inserted".to_string() } }),
        JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: key("tags"), index: 0 }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
