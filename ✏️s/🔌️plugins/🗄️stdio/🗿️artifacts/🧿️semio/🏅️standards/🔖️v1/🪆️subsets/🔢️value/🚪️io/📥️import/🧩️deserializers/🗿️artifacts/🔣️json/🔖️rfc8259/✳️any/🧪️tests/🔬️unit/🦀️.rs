use super::*;
use semio_s_artifact_stdio_json::schema::snapshot::JsonMember;

#[semio_framework_async_macros::async_test]
async fn number_lexeme_splits_into_int_or_float_by_grammar_shape() {
    assert_eq!(semio_value_from_json(&JsonValue::Number { lexeme: "42".into() }), SemioValue::Int { lexeme: "42".into() });
    assert_eq!(semio_value_from_json(&JsonValue::Number { lexeme: "-7".into() }), SemioValue::Int { lexeme: "-7".into() });
    assert_eq!(semio_value_from_json(&JsonValue::Number { lexeme: "3.500".into() }), SemioValue::Float { lexeme: "3.500".into() });
    assert_eq!(semio_value_from_json(&JsonValue::Number { lexeme: "1e10".into() }), SemioValue::Float { lexeme: "1e10".into() });
    assert_eq!(semio_value_from_json(&JsonValue::Number { lexeme: "9007199254740993".into() }), SemioValue::Int { lexeme: "9007199254740993".into() }, "arbitrary-precision int lexeme untouched");
}

#[semio_framework_async_macros::async_test]
async fn nested_structure_maps_directly() {
    let json = JsonValue::Object {
        members: vec![JsonMember { key: "name".into(), value: JsonValue::String { value: "semio".into() } }, JsonMember { key: "tags".into(), value: JsonValue::Array { items: vec![JsonValue::Bool { value: true }, JsonValue::Null] } }],
    };
    let value = semio_value_from_json(&json);
    match value {
        SemioValue::Map { entries } => {
            assert_eq!(entries[0].key, "name");
            assert_eq!(entries[0].value, SemioValue::Str { value: "semio".into() });
            match &entries[1].value {
                SemioValue::List { items } => assert_eq!(items, &vec![SemioValue::Bool { value: true }, SemioValue::Null]),
                other => panic!("expected list, got {other:?}"),
            }
        }
        other => panic!("expected map, got {other:?}"),
    }
}
