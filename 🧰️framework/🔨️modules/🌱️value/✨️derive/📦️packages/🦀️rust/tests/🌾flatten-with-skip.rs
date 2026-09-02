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

//#region 🔖️FlattenOnVariantFieldCompileError — same comment-only pattern as
// `FlattenDenyUnknownFieldsCompileError` above: `flatten` on an enum variant's own named field is a
// `compile_error!` naming the field (see `check_variant_field_attrs_supported` in the derive's own
// source), not the pre-fix silent no-op.
//
// #[derive(ToValue, FromValue)]
// #[value(tag = "kind")]
// enum BadVariantFlatten {
//     Variant {
//         #[value(flatten)]
//         extra: std::collections::BTreeMap<String, String>,
//     },
// }
//#endregion 🔖️FlattenOnVariantFieldCompileError

//#region 🔖️CratePathOverride — #[value(crate = "…")]
/// 🧭️ Stands in for a sub-kernel crate's own reexport of `DslValue`/`ToValue`/`FromValue`/
/// `ValueError` (the `semio-framework-actor` scenario from
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`)
/// — proves `#[value(crate = "…")]` threads through every one of `expand_to_value`'s and
/// `expand_from_value`'s call sites, not just the impl header, since the struct below only compiles
/// at all if every one of them resolves under this non-default path.
mod value_root {
    pub use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(crate = "crate::value_root", rename_all = "camelCase")]
struct CratePathOverride {
    label: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

#[test]
fn crate_path_override_compiles_and_round_trips_from_a_non_default_path() {
    let value = CratePathOverride { label: "hello".to_string(), note: None };
    let encoded = value.to_value();
    assert_eq!(encoded, DslValue::object([("label".to_string(), DslValue::String("hello".to_string()))]));
    assert_eq!(CratePathOverride::from_value(encoded), Ok(value));
}
//#endregion 🔖️CratePathOverride

//#region 🔖️VariantFieldAttrs — skip_serializing_if / skip / serialize_with / deserialize_with on a
// named field of an enum variant (as opposed to a plain struct field, already covered above)
mod byte_len_bridge {
    use semio_framework_os_kernel::{DslValue, ValueError};

    /// 🔟 Stand-in for a hand-written wire bridge, shaped like 🏪️store's real
    /// `operation_envelope_serde`/`envelope_serde` modules backing `ArtifactActorMsg::
    /// LocalMutations`/`ArtifactEvent::RemoteMutations`/`ArtifactMutationsSaved.envelope` — encodes
    /// as the byte LENGTH instead of the bytes themselves, so silently falling back to the default
    /// `ToValue::to_value` (which would encode the raw `Vec<u8>` as a `DslValue::Array`) is
    /// trivially observable instead of merely "still compiles, wrong shape".
    pub fn to_value(bytes: &Vec<u8>) -> DslValue {
        DslValue::uint(bytes.len() as u64)
    }

    pub fn from_value(value: DslValue) -> Result<Vec<u8>, ValueError> {
        let len = value.as_u64().ok_or_else(|| ValueError::new("expected a number".to_string()))?;
        Ok(vec![0u8; len as usize])
    }
}

fn seeded_variant_cache() -> i32 {
    42
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
enum EnumVariantFieldMsg {
    Payload {
        #[value(serialize_with = "byte_len_bridge::to_value", deserialize_with = "byte_len_bridge::from_value")]
        bytes: Vec<u8>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        note: Option<String>,
        #[value(skip, default = "seeded_variant_cache")]
        cache: i32,
    },
}

#[test]
fn variant_field_serialize_with_routes_through_the_named_bridge_not_the_default_impl() {
    let value = EnumVariantFieldMsg::Payload { bytes: vec![1, 2, 3], note: None, cache: 7 };
    let encoded = value.to_value();
    let DslValue::Object(entries) = &encoded else { panic!("expected an object") };
    assert_eq!(entries.iter().find(|(k, _)| k == "bytes").map(|(_, v)| v.clone()), Some(DslValue::uint(3)));
    assert!(!entries.iter().any(|(k, _)| k == "note"), "skip_serializing_if should omit `note` when None");
    assert!(!entries.iter().any(|(k, _)| k == "cache"), "skip should omit `cache` unconditionally");
    let decoded = EnumVariantFieldMsg::from_value(encoded).expect("decodes");
    assert_eq!(decoded, EnumVariantFieldMsg::Payload { bytes: vec![0, 0, 0], note: None, cache: 42 });
}

#[test]
fn variant_field_skip_serializing_if_includes_the_field_when_the_predicate_is_false() {
    let value = EnumVariantFieldMsg::Payload { bytes: vec![], note: Some("hi".to_string()), cache: 1 };
    let encoded = value.to_value();
    let DslValue::Object(entries) = &encoded else { panic!("expected an object") };
    assert_eq!(entries.iter().find(|(k, _)| k == "note").map(|(_, v)| v.clone()), Some(DslValue::String("hi".to_string())));
}
//#endregion 🔖️VariantFieldAttrs
