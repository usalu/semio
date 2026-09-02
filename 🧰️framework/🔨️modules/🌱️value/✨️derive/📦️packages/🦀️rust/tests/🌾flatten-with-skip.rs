//! 🌾 `#[value(flatten)]`, `#[value(with = "…")]`, and `#[value(skip)]` field attributes — a
//! genuine `tests/*.rs` integration crate, so `#[derive(ToValue, FromValue)]` runs exactly as any
//! downstream consumer's derive invocation would (a proc-macro crate cannot exercise its own
//! derives from inside its own `src`). Ticket
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
//!
//! The `flatten` cases carry a `serde`/`serde_json` oracle (dev-dependency only, never a
//! production dependency of this crate — see this crate's `Cargo.toml`) proving the wire shape
//! this derive produces is byte-identical (same keys, same values, same JSON text once both sides
//! are funneled through `serde_json::to_string`) to what `#[derive(serde::Serialize)]` +
//! `#[serde(flatten)]` produces for the equivalent struct shape.
use semio_framework_os_kernel::{DslValue, FromValue, ToValue};
use std::collections::BTreeMap;

// 🌿️ See the sibling `🛡️deny-unknown-fields-enums.rs` test file's identical docstring for why
// `semio_framework_os_kernel` alone (not a separate `semio_framework_value_derive` import) is the
// correct single import here.

//#region 🔖️Flatten — nested-struct field
#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    #[value(flatten)]
    #[serde(flatten)]
    address: Address,
    age: u32,
}

#[test]
fn flatten_nested_struct_splices_entries_into_parent_object() {
    let person = Person { name: "Ada".to_string(), address: Address { street: "1 Infinite Loop".to_string(), city: "Cupertino".to_string() }, age: 30 };
    let encoded = person.to_value();
    let DslValue::Object(entries) = &encoded else { panic!("expected an object") };
    let keys: Vec<&str> = entries.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(keys, vec!["name", "street", "city", "age"]);
    assert_eq!(Person::from_value(encoded), Ok(person));
}

#[test]
fn flatten_nested_struct_matches_serde_json_byte_for_byte() {
    let person = Person { name: "Grace".to_string(), address: Address { street: "2 Turing Way".to_string(), city: "Arlington".to_string() }, age: 85 };
    let ours = serde_json::Value::from(&person.to_value());
    let theirs = serde_json::to_value(&person).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(serde_json::to_string(&ours).unwrap(), serde_json::to_string(&theirs).unwrap());
}
//#endregion 🔖️Flatten — nested-struct field

//#region 🔖️Flatten — catch-all map field
#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize)]
struct Extensible {
    id: u32,
    #[value(flatten)]
    #[serde(flatten)]
    extra: BTreeMap<String, String>,
}

#[test]
fn flatten_catch_all_map_round_trips() {
    let mut extra = BTreeMap::new();
    extra.insert("color".to_string(), "red".to_string());
    extra.insert("size".to_string(), "large".to_string());
    let value = Extensible { id: 7, extra };
    let encoded = value.to_value();
    assert_eq!(Extensible::from_value(encoded), Ok(value));
}

#[test]
fn flatten_catch_all_map_matches_serde_json_byte_for_byte() {
    let mut extra = BTreeMap::new();
    extra.insert("alpha".to_string(), "1".to_string());
    extra.insert("beta".to_string(), "2".to_string());
    let value = Extensible { id: 42, extra };
    let ours = serde_json::Value::from(&value.to_value());
    let theirs = serde_json::to_value(&value).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(serde_json::to_string(&ours).unwrap(), serde_json::to_string(&theirs).unwrap());
}

#[test]
fn flatten_absorbs_unknown_keys_without_deny_unknown_fields() {
    let encoded = DslValue::object([("id".to_string(), DslValue::uint(1)), ("nickname".to_string(), DslValue::String("Al".to_string()))]);
    let mut expected_extra = BTreeMap::new();
    expected_extra.insert("nickname".to_string(), "Al".to_string());
    assert_eq!(Extensible::from_value(encoded), Ok(Extensible { id: 1, extra: expected_extra }));
}
//#endregion 🔖️Flatten — catch-all map field

//#region 🔖️With — serialize_with/deserialize_with shorthand
mod hex_u32 {
    use semio_framework_os_kernel::{DslValue, ValueError};

    /// 🔟 Encodes as a `0x`-prefixed hex string instead of `u32`'s own plain-number `ToValue`.
    pub fn to_value(value: &u32) -> DslValue {
        DslValue::String(format!("0x{value:x}"))
    }

    pub fn from_value(value: DslValue) -> Result<u32, ValueError> {
        let DslValue::String(s) = value else { return Err(ValueError::new("expected a hex string".to_string())) };
        let digits = s.strip_prefix("0x").ok_or_else(|| ValueError::new("expected a 0x-prefixed hex string".to_string()))?;
        u32::from_str_radix(digits, 16).map_err(|error| ValueError::new(error.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
struct WithField {
    #[value(with = "hex_u32")]
    color: u32,
}

#[test]
fn with_shorthand_wires_both_directions() {
    let value = WithField { color: 255 };
    let encoded = value.to_value();
    assert_eq!(encoded, DslValue::object([("color".to_string(), DslValue::String("0xff".to_string()))]));
    assert_eq!(WithField::from_value(encoded), Ok(value));
}
//#endregion 🔖️With

//#region 🔖️Skip — omitted on serialize, Default on deserialize
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
struct SkipBare {
    kept: String,
    #[value(skip)]
    cache: i32,
}

fn seeded_cache() -> i32 {
    99
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
struct SkipWithDefault {
    kept: String,
    #[value(skip, default = "seeded_cache")]
    cache: i32,
}

#[test]
fn skip_omits_field_on_serialize() {
    let value = SkipBare { kept: "x".to_string(), cache: 123 };
    let encoded = value.to_value();
    assert_eq!(encoded, DslValue::object([("kept".to_string(), DslValue::String("x".to_string()))]));
}

#[test]
fn skip_uses_default_on_deserialize_even_if_key_present() {
    let encoded = DslValue::object([("kept".to_string(), DslValue::String("x".to_string())), ("cache".to_string(), DslValue::int(7))]);
    assert_eq!(SkipBare::from_value(encoded), Ok(SkipBare { kept: "x".to_string(), cache: 0 }));
}

#[test]
fn skip_with_default_path_uses_that_path_not_type_default() {
    let encoded = DslValue::object([("kept".to_string(), DslValue::String("x".to_string()))]);
    assert_eq!(SkipWithDefault::from_value(encoded), Ok(SkipWithDefault { kept: "x".to_string(), cache: 99 }));
}
//#endregion 🔖️Skip

//#region 🔖️FlattenDenyUnknownFieldsCompileError — documented via a trybuild-free direct check:
// the combination is rejected at proc-macro expansion time, so this crate would simply fail to
// compile if this struct's attributes were left in the source tree uncommented. Kept as a comment
// (not `trybuild`, not present in this crate's dependency graph) rather than a runtime assertion,
// since there is no runtime artifact to assert on — the derive's `syn::Error` becomes a
// `compile_error!` token stream the compiler reports directly.
//
// #[derive(ToValue, FromValue)]
// #[value(deny_unknown_fields)]
// struct BadCombo {
//     #[value(flatten)]
//     extra: std::collections::BTreeMap<String, String>,
// }
//#endregion 🔖️FlattenDenyUnknownFieldsCompileError
