//! 📥️ `SemioValueFromJson` — the cleanest pair in this wave: `value`'s `SemioValue` was
//! literally modeled ON json's `JsonValue` (w1b-type-ownership.md), so this is a near-direct
//! structural mapping, not a reinterpretation.
//!
//! - `Null`/`Bool`/`String`/`Array`/`Value` map 1:1 onto `Null`/`Bool`/`Str`/`List`/`Map`.
//! - json's single `Number{lexeme}` SPLITS into semio's typed `Int{lexeme}`/`Float{lexeme}` by
//!   inspecting the lexeme's own RFC8259 grammar shape (a `.`/`e`/`E` makes it a `Float`, matching
//!   §6's `int frac? exp?` production) — the lexeme itself is never touched, so this is a
//!   classification, not a reparse.
//! - json has no binary or graph-reference primitive: `SemioValue::Bytes`/`Ref` are NEVER produced
//!   by this direction (the `nodes` backing store always decodes empty) — see the serializer's
//!   own doc comment for what happens going the other way.

use crate::artifacts::json::schema::snapshot::JsonValue;
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioValueFromJson;

impl ArtifactDeserializer for SemioValueFromJson {
    type From = JsonSnapshot;
    type Into = SemioValueSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_json(&from.value).await, nodes: Vec::new() })
    }
}

pub async fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Convert
/// 🔢️ RFC8259 §6: `int frac? exp?` — a lexeme with no `.`/`e`/`E` is a bare `int`.
async fn is_float_lexeme(lexeme: &str) -> bool {
    lexeme.contains('.') || lexeme.contains('e') || lexeme.contains('E')
}

pub async fn semio_value_from_json(v: &JsonValue) -> SemioValue {
    match v {
        JsonValue::Null => SemioValue::Null,
        JsonValue::Bool { value } => SemioValue::Bool { value: *value },
        JsonValue::Number { lexeme } => {
            if is_float_lexeme(lexeme).await {
                SemioValue::Float { lexeme: lexeme.clone() }
            } else {
                SemioValue::Int { lexeme: lexeme.clone() }
            }
        }
        JsonValue::String { value } => SemioValue::Str { value: value.clone() },
        JsonValue::Array { items } => SemioValue::List { items: items.iter().map(semio_value_from_json).collect() },
        JsonValue::Object { members } => SemioValue::Map { entries: members.iter().map(|m| SemioValueEntry { key: m.key.clone(), value: semio_framework_plugin::resolve_ready(semio_value_from_json(&m.value)) }).collect() },
    }
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::json::schema::snapshot::JsonMember;

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
}
//#endregion 🧪️Tests
