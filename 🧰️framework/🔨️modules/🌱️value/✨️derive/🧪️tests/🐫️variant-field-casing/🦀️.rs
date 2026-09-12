//! 🐫 Two serde-semantics laws of `#[derive(ToValue, FromValue)]` that this derive used to get
//! wrong, ticket
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS` rows 47a/47b:
//!
//! 1. **Container `rename_all` never cases an enum variant's own named fields.** Container
//!    `rename_all_fields` supplies their default; variant `rename_all` overrides it and an explicit
//!    field rename wins. `ContainerAttrs::field_rename_all()` used to fall back from
//!    `rename_all_fields` to `rename_all`, so every `#[value(rename_all = "camelCase")]` enum with
//!    multi-word named variant fields wired them camelCase where serde wires them verbatim. Every
//!    case below carries a `serde`/`serde_json` oracle (dev-dependency only, CLAUDE.md's sanctioned
//!    validation pattern — see this crate's `Cargo.toml`) with the identical attribute pair, so the
//!    assertion is "byte-identical to serde", not "matches what I believe serde does".
//!
//! 2. **The internally tagged single-unnamed-field variant is symmetric.** Encoding a payload that
//!    does not become an object carries it as `{tag, "value": <payload>}`; decoding now unwraps
//!    that carrier instead of handing the whole object to a scalar `FromValue` that can never
//!    accept it. serde refuses this shape at serialization time rather than defining a carrier, so
//!    the oracle here is serde's own error — asserted, not asserted-about.
use semio_framework_os_kernel::{DslValue, FromValue, ToValue};

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "snake_case", deny_unknown_fields)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "snake_case", deny_unknown_fields)]
enum VariantOverrideInternal {
    #[value(rename_all = "camelCase")]
    #[serde(rename_all = "camelCase")]
    Bar { material_id: String, #[value(rename = "section")] #[serde(rename = "section")] section_id: String },
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(rename_all_fields = "snake_case", deny_unknown_fields)]
#[serde(rename_all_fields = "snake_case", deny_unknown_fields)]
enum VariantOverrideExternal {
    #[value(rename = "bar-view", rename_all = "camelCase")]
    #[serde(rename = "bar-view", rename_all = "camelCase")]
    Bar { material_id: String, #[value(rename = "section")] #[serde(rename = "section")] section_id: String },
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(tag = "kind", content = "payload", rename_all = "camelCase", rename_all_fields = "snake_case", deny_unknown_fields)]
#[serde(tag = "kind", content = "payload", rename_all = "camelCase", rename_all_fields = "snake_case", deny_unknown_fields)]
enum VariantOverrideAdjacent {
    #[value(rename_all = "camelCase")]
    #[serde(rename_all = "camelCase")]
    Bar { material_id: String, #[value(rename = "section")] #[serde(rename = "section")] section_id: String },
}

#[test]
fn variant_level_field_casing_overrides_container_and_matches_neutral_serde_oracle() {
    fn verify<T: ToValue + FromValue + serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq>(value: T, expected: &serde_json::Value) {
        assert_eq!(serde_json::to_value(&value).expect("serde encoding"), *expected);
        assert_eq!(serde_json::Value::from(&value.to_value()), *expected);
        assert_eq!(T::from_value(expected.clone().into()).expect("native decoding"), value);
        assert_eq!(serde_json::from_value::<T>(expected.clone()).expect("serde decoding"), value);
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🐫️variant-field-casing/🔣️.json")).expect("neutral field casing corpus");
    verify(VariantOverrideInternal::Bar { material_id: "m1".into(), section_id: "s1".into() }, &corpus["internal"]);
    verify(VariantOverrideExternal::Bar { material_id: "m1".into(), section_id: "s1".into() }, &corpus["external"]);
    verify(VariantOverrideAdjacent::Bar { material_id: "m1".into(), section_id: "s1".into() }, &corpus["adjacent"]);
    eprintln!("[DEBUG] variant-owned field casing agrees with serde and three neutral wire shapes");
}

/// 🔑 The wire key names this derive emits, in emission order. `serde_json::Value`'s map compares
/// (and, for the workspace's feature resolution, orders) independently of insertion order, so the
/// oracle comparisons below assert equality of the two documents while this asserts the names —
/// which is the whole subject of row 47a.
fn wire_keys(value: &DslValue) -> Vec<String> {
    let DslValue::Object(entries) = value else { panic!("expected an object, found {value:?}") };
    entries.iter().map(|(key, _)| key.clone()).collect()
}

// 🌿️ See the sibling `🛡️deny-unknown-fields-enums.rs` test file's identical docstring for why
// `semio_framework_os_kernel` alone (not a separate `semio_framework_value_derive` import) is the
// correct single import here.

//#region 🔖️RenameAllAlone — internally tagged
/// 🧿️ Shaped after stdio's `GeometryRef` (`🧿️semio/…/🏛️model/🧬️schema/📸️snapshot/🦀️.rs`), the
/// enum whose committed fixture disagreed with the derive.
#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(tag = "kind", rename_all = "camelCase")]
#[serde(tag = "kind", rename_all = "camelCase")]
enum RenameAllAlone {
    BrepRef { brep_id: String, mesh_id: u32 },
}

#[test]
fn rename_all_alone_cases_the_variant_tag_and_leaves_its_fields_verbatim() {
    let value = RenameAllAlone::BrepRef { brep_id: "b-1".to_string(), mesh_id: 7 };
    let ours = serde_json::Value::from(&value.to_value());
    let theirs = serde_json::to_value(&value).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(wire_keys(&value.to_value()), vec!["kind", "brep_id", "mesh_id"]);
}

#[test]
fn rename_all_alone_round_trips_and_decodes_the_same_wire_serde_decodes() {
    let value = RenameAllAlone::BrepRef { brep_id: "b-2".to_string(), mesh_id: 9 };
    let encoded = value.to_value();
    assert_eq!(RenameAllAlone::from_value(encoded), Ok(value.clone()));
    let oracle: RenameAllAlone = serde_json::from_str(r#"{"kind":"brepRef","brep_id":"b-2","mesh_id":9}"#).expect("serde decodes the verbatim wire");
    assert_eq!(oracle, value);
}

#[test]
fn rename_all_alone_rejects_the_camel_cased_field_names_exactly_as_serde_does() {
    let camel = DslValue::object([
        ("kind".to_string(), DslValue::String("brepRef".to_string())),
        ("brepId".to_string(), DslValue::String("b-3".to_string())),
        ("meshId".to_string(), DslValue::uint(1)),
    ]);
    assert!(RenameAllAlone::from_value(camel).is_err());
    assert!(serde_json::from_str::<RenameAllAlone>(r#"{"kind":"brepRef","brepId":"b-3","meshId":1}"#).is_err());
}
//#endregion 🔖️RenameAllAlone

//#region 🔖️RenameAllFields — the attribute that actually cases variant fields
#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum RenameAllFieldsToo {
    BrepRef { brep_id: String, mesh_id: u32 },
}

#[test]
fn rename_all_fields_cases_the_variant_fields_byte_for_byte_with_serde() {
    let value = RenameAllFieldsToo::BrepRef { brep_id: "b-1".to_string(), mesh_id: 7 };
    let ours = serde_json::Value::from(&value.to_value());
    let theirs = serde_json::to_value(&value).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(wire_keys(&value.to_value()), vec!["kind", "brepId", "meshId"]);
    assert_eq!(RenameAllFieldsToo::from_value(value.to_value()), Ok(value));
}

/// 🔀 The two attributes are independent axes, so the case that reads the tags is not the case that
/// reads the fields — the `📇️directory/🧬️schema` shape named in the derive's own module docstring.
#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(tag = "kind", rename_all = "snake_case", rename_all_fields = "camelCase")]
#[serde(tag = "kind", rename_all = "snake_case", rename_all_fields = "camelCase")]
enum SplitCasing {
    BrepRef { brep_id: String },
}

#[test]
fn rename_all_and_rename_all_fields_case_independently_like_serde() {
    let value = SplitCasing::BrepRef { brep_id: "b".to_string() };
    let ours = serde_json::Value::from(&value.to_value());
    let theirs = serde_json::to_value(&value).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(wire_keys(&value.to_value()), vec!["kind", "brepId"]);
}
//#endregion 🔖️RenameAllFields

//#region 🔖️ExternallyTaggedAndAdjacent — the same law on the other two representations
#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
enum ExternallyTagged {
    DocBlock { style_id: String },
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(tag = "kind", content = "payload", rename_all = "camelCase")]
#[serde(tag = "kind", content = "payload", rename_all = "camelCase")]
enum AdjacentlyTagged {
    DocBlock { style_id: String },
}

#[test]
fn externally_tagged_variant_fields_stay_verbatim_like_serde() {
    let value = ExternallyTagged::DocBlock { style_id: "s".to_string() };
    let ours = serde_json::Value::from(&value.to_value());
    let theirs = serde_json::to_value(&value).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(wire_keys(&value.to_value()), vec!["docBlock"]);
    assert_eq!(ours["docBlock"], serde_json::json!({ "style_id": "s" }));
    assert_eq!(ExternallyTagged::from_value(value.to_value()), Ok(value));
}

#[test]
fn adjacently_tagged_variant_fields_stay_verbatim_like_serde() {
    let value = AdjacentlyTagged::DocBlock { style_id: "s".to_string() };
    let ours = serde_json::Value::from(&value.to_value());
    let theirs = serde_json::to_value(&value).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(wire_keys(&value.to_value()), vec!["kind", "payload"]);
    assert_eq!(ours["payload"], serde_json::json!({ "style_id": "s" }));
    assert_eq!(AdjacentlyTagged::from_value(value.to_value()), Ok(value));
}
//#endregion 🔖️ExternallyTaggedAndAdjacent

//#region 🔖️StructFieldsUnaffected — `rename_all` still cases a plain struct's fields
#[derive(Debug, Clone, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct PlainStruct {
    style_id: String,
}

#[test]
fn plain_struct_fields_are_still_cased_by_rename_all() {
    let value = PlainStruct { style_id: "s".to_string() };
    let ours = serde_json::Value::from(&value.to_value());
    let theirs = serde_json::to_value(&value).expect("serde_json::to_value");
    assert_eq!(ours, theirs);
    assert_eq!(wire_keys(&value.to_value()), vec!["styleId"]);
    assert_eq!(PlainStruct::from_value(value.to_value()), Ok(value));
}
//#endregion 🔖️StructFieldsUnaffected

//#region 🔖️DenyUnknownFieldsFollowsTheSameRule
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum DenyingRenameAllAlone {
    BrepRef { brep_id: String },
}

#[test]
fn deny_unknown_fields_allows_the_verbatim_name_and_rejects_the_cased_one() {
    let verbatim = DslValue::object([
        ("kind".to_string(), DslValue::String("brepRef".to_string())),
        ("brep_id".to_string(), DslValue::String("b".to_string())),
    ]);
    assert_eq!(DenyingRenameAllAlone::from_value(verbatim), Ok(DenyingRenameAllAlone::BrepRef { brep_id: "b".to_string() }));
    let cased = DslValue::object([
        ("kind".to_string(), DslValue::String("brepRef".to_string())),
        ("brepId".to_string(), DslValue::String("b".to_string())),
    ]);
    assert!(DenyingRenameAllAlone::from_value(cased).is_err());
}
//#endregion 🔖️DenyUnknownFieldsFollowsTheSameRule

//#region 🔖️InternallyTaggedScalarNewtype — row 47b
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
struct Coordinate {
    x: f64,
    y: f64,
}

/// 🪆 Shaped after stdio's `JsonPathSegment` (json/i-json) and `XlsxCellValue` (xlsx): an
/// internally tagged enum whose variants carry a bare scalar, an array, an object, and — the
/// ambiguity case — an object type whose only field is itself named `value`.
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
struct ValueNamedField {
    value: String,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(tag = "segment")]
enum PathSegment {
    Key(String),
    Index(u64),
    Ratio(f64),
    Flag(bool),
    Span(Vec<u64>),
    At(Coordinate),
    Wrapped(ValueNamedField),
}

#[test]
fn internally_tagged_scalar_newtype_encodes_under_a_value_carrier() {
    assert_eq!(
        PathSegment::Key("name".to_string()).to_value(),
        DslValue::object([("segment".to_string(), DslValue::String("Key".to_string())), ("value".to_string(), DslValue::String("name".to_string()))])
    );
    assert_eq!(
        PathSegment::Span(vec![1, 2]).to_value(),
        DslValue::object([
            ("segment".to_string(), DslValue::String("Span".to_string())),
            ("value".to_string(), DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2)])),
        ])
    );
}

#[test]
fn internally_tagged_object_newtype_still_splices_its_entries_beside_the_tag() {
    assert_eq!(
        PathSegment::At(Coordinate { x: 1.0, y: 2.0 }).to_value(),
        DslValue::object([
            ("segment".to_string(), DslValue::String("At".to_string())),
            ("x".to_string(), DslValue::float(1.0)),
            ("y".to_string(), DslValue::float(2.0)),
        ])
    );
}

#[test]
fn every_internally_tagged_newtype_payload_shape_round_trips() {
    for value in [
        PathSegment::Key("name".to_string()),
        PathSegment::Index(42),
        PathSegment::Ratio(0.5),
        PathSegment::Flag(true),
        PathSegment::Span(vec![1, 2, 3]),
        PathSegment::At(Coordinate { x: -1.5, y: 2.25 }),
        PathSegment::Wrapped(ValueNamedField { value: "inner".to_string() }),
    ] {
        assert_eq!(PathSegment::from_value(value.to_value()), Ok(value.clone()), "round-trip failed for {value:?}");
    }
}

/// 🛡️ `Wrapped` and `Key` produce the same key set (`{segment, value}`) with the same value type,
/// so the decoder cannot tell them apart from the wire alone — only from the tag. This is the case
/// a key-shape test would mis-decode and object-first gets right.
#[test]
fn the_value_named_field_payload_is_not_mistaken_for_the_scalar_carrier() {
    let wrapped = DslValue::object([
        ("segment".to_string(), DslValue::String("Wrapped".to_string())),
        ("value".to_string(), DslValue::String("inner".to_string())),
    ]);
    assert_eq!(PathSegment::from_value(wrapped), Ok(PathSegment::Wrapped(ValueNamedField { value: "inner".to_string() })));
    let key = DslValue::object([
        ("segment".to_string(), DslValue::String("Key".to_string())),
        ("value".to_string(), DslValue::String("inner".to_string())),
    ]);
    assert_eq!(PathSegment::from_value(key), Ok(PathSegment::Key("inner".to_string())));
}

#[test]
fn a_payload_that_is_neither_shape_reports_the_payload_types_own_error() {
    let bogus = DslValue::object([
        ("segment".to_string(), DslValue::String("Key".to_string())),
        ("unrelated".to_string(), DslValue::uint(1)),
    ]);
    assert!(PathSegment::from_value(bogus).is_err());
    let wrong_scalar_type = DslValue::object([
        ("segment".to_string(), DslValue::String("Index".to_string())),
        ("value".to_string(), DslValue::String("not-a-number".to_string())),
    ]);
    assert!(PathSegment::from_value(wrong_scalar_type).is_err());
}

/// 🔬 serde's own position on this shape, executed rather than quoted: it has no carrier and fails
/// at serialization time, which is why the wire form above is this derive's to define.
#[derive(serde::Serialize)]
#[serde(tag = "segment")]
enum SerdeScalarNewtype {
    Key(String),
}

#[test]
fn serde_refuses_the_shape_this_derive_carries_under_value() {
    let error = serde_json::to_value(SerdeScalarNewtype::Key("name".to_string())).expect_err("serde has no carrier for a scalar internally tagged newtype");
    assert!(error.to_string().contains("tagged newtype variant"), "unexpected serde error: {error}");
}
//#endregion 🔖️InternallyTaggedScalarNewtype
