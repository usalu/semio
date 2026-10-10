//! 🧬️ JsonSnapshot schema — own `JsonValue` model + a from-scratch RFC8259 recursive-descent
//! parser/serializer. Preserves object-member INSERTION ORDER (`Vec<JsonMember>`, not a map) and
//! the ORIGINAL NUMBER LEXEME verbatim (rfc8259 allows arbitrary precision — never round-tripped
//! through `f64`). No `serde_json::Value` anywhere in this file.

#[path="🔢️number/🦀️.rs"]
pub(crate) mod number;

use crate::STDIO_JSON_DOCUMENT_SCHEMA;
use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

//#region 🔖️JsonModel
/// 🍃️ One `object` member, in source order.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonMember {
    pub key: String,
    pub value: JsonValue,
}

/// 🌳 An RFC8259 JSON value. `Number` keeps the ORIGINAL LEXEME (never parsed to `f64` — rfc8259
/// permits arbitrary precision, so re-emitting a lossy `f64` round-trip would silently corrupt
/// real documents carrying e.g. 19-digit ids or high-precision decimals). `Object` is a `Vec` of
/// [`JsonMember`] (never a map) so decode->encode preserves member insertion order exactly.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
#[derive(Default)]
pub enum JsonValue {
    #[default]
    Null,
    // NOTE: every non-unit variant MUST be a struct variant (named field), never a bare tuple
    // variant — serde's internally-tagged (`tag = "kind"`) representation can only merge the tag
    // into map-shaped content; a tuple variant wrapping a non-map type (`bool`/`String`/`Vec<_>`)
    // compiles fine but fails at RUNTIME serialization ("can only flatten structs and maps").
    Bool {
        value: bool,
    },
    Number {
        lexeme: String,
    },
    String {
        value: String,
    },
    Array {
        items: Vec<JsonValue>,
    },
    Object {
        members: Vec<JsonMember>,
    },
}

//#endregion 🔖️JsonModel

//#region 🔖️Parser





//#endregion 🔖️Parser

//#region 🔖️Serializer















//#endregion 🔖️Serializer

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.json` snapshot.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json")]
pub struct JsonSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub value: JsonValue,
}

impl Default for JsonSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: JsonValue::Null }
    }
}

impl JsonSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_value(value: impl Into<JsonValue>) -> Self {
        Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: value.into() }
    }


}
//#endregion 🔖️Snapshot



//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure document helper, no engine needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_json_snapshot() -> JsonSnapshot {
    JsonSnapshot::default()
}

/// 📄️ The demo `stdio.json` document — a genuinely 3-level-nested `JsonValue` (object → array,
/// object → object → object → array) exercising every `JsonValue` variant (`Null`/`Bool`/`Number`/
/// `String`/`Array`/`Object`) at least once. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `../💡️inferences/🦀️.rs`'s `conformance_laws`) and for
/// `nontrivial_nested_value_round_trip` below, which calls this instead of duplicating the literal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_json_snapshot() -> JsonSnapshot {
    let value = JsonValue::Object {
        members: vec![
            JsonMember { key: "name".into(), value: JsonValue::String { value: "semio".into() } },
            JsonMember { key: "count".into(), value: JsonValue::Number { lexeme: "42".into() } },
            JsonMember { key: "ratio".into(), value: JsonValue::Number { lexeme: "3.5".into() } },
            JsonMember { key: "active".into(), value: JsonValue::Bool { value: true } },
            JsonMember { key: "missing".into(), value: JsonValue::Null },
            JsonMember { key: "tags".into(), value: JsonValue::Array { items: vec![JsonValue::String { value: "a".into() }, JsonValue::String { value: "b".into() }, JsonValue::String { value: "c".into() }] } },
            JsonMember {
                key: "nested".into(),
                value: JsonValue::Object {
                    members: vec![JsonMember {
                        key: "deep".into(),
                        value: JsonValue::Object {
                            members: vec![JsonMember { key: "deeper".into(), value: JsonValue::Array { items: vec![JsonValue::Number { lexeme: "1".into() }, JsonValue::Number { lexeme: "2".into() }, JsonValue::Number { lexeme: "3".into() }] } }],
                        },
                    }],
                },
            },
        ],
    };
    JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests



#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
