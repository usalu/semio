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

use semio_s_artifact_stdio_json::schema::snapshot::JsonValue;
use semio_s_artifact_stdio_json::JsonSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioValueFromJson;

impl ArtifactDeserializer for SemioValueFromJson {
    type From = JsonSnapshot;
    type Into = SemioValueSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_json(&from.value), nodes: Vec::new() })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Convert
/// 🔢️ RFC8259 §6: `int frac? exp?` — a lexeme with no `.`/`e`/`E` is a bare `int`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_float_lexeme(lexeme: &str) -> bool {
    lexeme.contains('.') || lexeme.contains('e') || lexeme.contains('E')
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_value_from_json(v: &JsonValue) -> SemioValue {
    match v {
        JsonValue::Null => SemioValue::Null,
        JsonValue::Bool { value } => SemioValue::Bool { value: *value },
        JsonValue::Number { lexeme } => {
            if is_float_lexeme(lexeme) {
                SemioValue::Float { lexeme: lexeme.clone() }
            } else {
                SemioValue::Int { lexeme: lexeme.clone() }
            }
        }
        JsonValue::String { value } => SemioValue::Str { value: value.clone() },
        JsonValue::Array { items } => SemioValue::List { items: items.iter().map(semio_value_from_json).collect() },
        JsonValue::Object { members } => SemioValue::Map { entries: members.iter().map(|m| SemioValueEntry { key: m.key.clone(), value: semio_value_from_json(&m.value) }).collect() },
    }
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
