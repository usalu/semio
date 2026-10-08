//! 🔎️ Native array item changes retain the neutral pointer and primitive control bucket.

use super::*;

fn item_schema(shape: &str) -> ArgSchema {
    let label = || LocalizedLabel::native("Value", "Wert");
    let text = || ActionArgDef::text("", label()).schema;
    match shape {
        "text" => text(),
        "boolean" => ArgSchema::Boolean,
        "number" => ActionArgDef::number("", label()).schema,
        "option" => ArgSchema::String { options: ["a", "b"].into_iter().map(|value| semio_framework::ActionArgOption { value: value.into(), label: label() }).collect(), option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        "vector" => ActionArgDef::vector("", label(), 2).schema,
        "object" => ArgSchema::Object { fields: vec![ActionArgDef::text("/text", label()), ActionArgDef::number("/count", label())] },
        "nestedText" => ArgSchema::Array { items: Box::new(text()), min_items: None, max_items: None },
        _ => panic!("unknown authored control shape"),
    }
}

#[test]
fn history_edit_acceptance_array_slots_retain_all_five_primitive_controls_and_exact_pointers() {
    let corpus = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(include_str!("🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let cases = corpus.get("cases").and_then(DslValue::as_array).unwrap();
    assert_eq!(cases.len(), 8);
    for case in cases {
        let input = ActionArgDef { schema: ArgSchema::Array { items: Box::new(item_schema(case.get("shape").and_then(DslValue::as_str).unwrap())), min_items: None, max_items: None }, ..ActionArgDef::text("/values", LocalizedLabel::native("Values", "Werte")) };
        let before = case.get("before").unwrap();
        let actual = acceptance_change_buckets(&[input.clone()], before).into_iter().enumerate().flat_map(|(bucket, changes)| changes.into_iter().map(move |(pointer, value)| DslValue::object([("bucket".into(), DslValue::int(bucket as i64)), ("pointer".into(), DslValue::String(pointer)), ("value".into(), value)]))).collect::<Vec<_>>();
        assert_eq!(DslValue::Array(actual), *case.get("expected").unwrap(), "{}", case.get("id").and_then(DslValue::as_str).unwrap());
        let hidden = ActionArgDef { presentation: Some(semio_framework::ArgPresentation::Hidden), ..input };
        assert!(acceptance_change_buckets(&[hidden], before).iter().all(Vec::is_empty));
        eprintln!("[DEBUG] generic history array {} retained exact primitive bucket/pointers and hidden-parent refusal", case.get("id").and_then(DslValue::as_str).unwrap());
    }
}
