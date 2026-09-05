//! 🛡️ `#[value(deny_unknown_fields)]` enforcement on every `Data::Enum` representation the derive
//! supports (see `🦀️.rs`'s module docs for the semantics chosen per representation) — a
//! genuine `tests/*.rs` integration crate, so `#[derive(ToValue, FromValue)]` runs exactly as any
//! downstream consumer's derive invocation would (a proc-macro crate cannot exercise its own
//! derives from inside its own `src`). Ticket
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.

// 🌿️ `semio_framework_os_kernel`'s crate root re-exports BOTH the `ToValue`/`FromValue` TRAITS
// (from its own `os_dsl::schema`) AND the `#[derive(ToValue, FromValue)]` proc-macros themselves
// under the same two names — see its `🦀️.rs`'s `🌱️`/`🌿️` docstrings — so importing them from
// here alone brings in everything this file's `#[derive(...)]` lines and trait calls need. A
// separate `use semio_framework_value_derive::{FromValue, ToValue};` would collide with the
// re-exported macro names (`E0252`, "defined multiple times ... macro namespace").
use semio_framework_os_kernel::{DslValue, FromValue, ToValue};

//#region 🔖️UnitOnly — bare-string wire form, deny_unknown_fields not applicable (documented N/A)
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
enum UnitOnly {
    First,
    SecondOne,
}

#[test]
fn unit_only_known_variant_decodes() {
    assert_eq!(UnitOnly::from_value(DslValue::String("first".to_string())), Ok(UnitOnly::First));
    assert_eq!(UnitOnly::from_value(DslValue::String("secondOne".to_string())), Ok(UnitOnly::SecondOne));
}

#[test]
fn unit_only_unrecognized_string_still_errors_attribute_or_not() {
    assert!(UnitOnly::from_value(DslValue::String("bogus".to_string())).is_err());
}
//#endregion 🔖️UnitOnly

//#region 🔖️ExternallyTagged — {"VariantName": payload}; deny_unknown_fields scopes to a
// named-field variant's own payload keys (the outer one-key shape is inherent, not gated).
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
enum ExternallyTagged {
    Empty,
    Wrapped(String),
    Detailed { name: String, count: i32 },
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
enum ExternallyTaggedLax {
    Detailed { name: String },
}

#[test]
fn externally_tagged_known_keys_round_trip() {
    let value = ExternallyTagged::Detailed { name: "a".to_string(), count: 3 };
    let encoded = value.to_value();
    assert_eq!(ExternallyTagged::from_value(encoded), Ok(value));
    assert_eq!(ExternallyTagged::from_value(DslValue::String("empty".to_string())), Ok(ExternallyTagged::Empty));
    assert_eq!(ExternallyTagged::from_value(DslValue::object([("wrapped".to_string(), DslValue::String("x".to_string()))])), Ok(ExternallyTagged::Wrapped("x".to_string())));
}

#[test]
fn externally_tagged_denies_unknown_payload_key() {
    let bad = DslValue::object([(
        "detailed".to_string(),
        DslValue::object([
            ("name".to_string(), DslValue::String("a".to_string())),
            ("count".to_string(), DslValue::int(3)),
            ("extra".to_string(), DslValue::Bool(true)),
        ]),
    )]);
    assert!(ExternallyTagged::from_value(bad).is_err());
}

#[test]
fn externally_tagged_without_attribute_accepts_unknown_payload_key() {
    let permissive = DslValue::object([(
        "detailed".to_string(),
        DslValue::object([("name".to_string(), DslValue::String("a".to_string())), ("extra".to_string(), DslValue::Bool(true))]),
    )]);
    assert_eq!(ExternallyTaggedLax::from_value(permissive), Ok(ExternallyTaggedLax::Detailed { name: "a".to_string() }));
}
//#endregion 🔖️ExternallyTagged

//#region 🔖️AdjacentlyTagged — {tag, content}; deny_unknown_fields scopes to BOTH the outer
// {tag, content} key set AND a named-field variant's own content keys.
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", content = "data", rename_all = "camelCase", deny_unknown_fields)]
enum AdjacentlyTagged {
    Empty,
    Wrapped(String),
    Detailed { name: String, count: i32 },
}

#[test]
fn adjacently_tagged_known_keys_round_trip() {
    let value = AdjacentlyTagged::Detailed { name: "a".to_string(), count: 3 };
    let encoded = value.to_value();
    assert_eq!(AdjacentlyTagged::from_value(encoded), Ok(value));
    assert_eq!(AdjacentlyTagged::from_value(DslValue::object([("kind".to_string(), DslValue::String("empty".to_string()))])), Ok(AdjacentlyTagged::Empty));
}

#[test]
fn adjacently_tagged_denies_unknown_outer_key() {
    let bad = DslValue::object([
        ("kind".to_string(), DslValue::String("empty".to_string())),
        ("stray".to_string(), DslValue::Bool(true)),
    ]);
    assert!(AdjacentlyTagged::from_value(bad).is_err());
}

#[test]
fn adjacently_tagged_denies_unknown_content_key() {
    let bad = DslValue::object([
        ("kind".to_string(), DslValue::String("detailed".to_string())),
        (
            "data".to_string(),
            DslValue::object([
                ("name".to_string(), DslValue::String("a".to_string())),
                ("count".to_string(), DslValue::int(3)),
                ("extra".to_string(), DslValue::Bool(true)),
            ]),
        ),
    ]);
    assert!(AdjacentlyTagged::from_value(bad).is_err());
}
//#endregion 🔖️AdjacentlyTagged

//#region 🔖️InternallyTagged — tag inline with fields; deny_unknown_fields scopes per matched
// variant (`{tag}` for a unit variant, `{tag} ∪ field names` for a named-field variant). A
// single-unnamed-payload variant hands the WHOLE entries object minus the tag to the payload
// type's own `FromValue` — proving the tag-stripping fix below, a payload struct that itself
// carries `#[value(deny_unknown_fields)]` must NOT see the wrapper's tag key as unknown.
#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct InnerPayload {
    value: i32,
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum InternallyTagged {
    Empty,
    Detailed { name: String, count: i32 },
    Wrapped(InnerPayload),
}

#[derive(Debug, Clone, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
enum InternallyTaggedLax {
    Detailed { name: String },
}

#[test]
fn internally_tagged_known_keys_round_trip() {
    let value = InternallyTagged::Detailed { name: "a".to_string(), count: 3 };
    let encoded = value.to_value();
    assert_eq!(InternallyTagged::from_value(encoded), Ok(value));
    assert_eq!(InternallyTagged::from_value(DslValue::object([("kind".to_string(), DslValue::String("empty".to_string()))])), Ok(InternallyTagged::Empty));
}

#[test]
fn internally_tagged_denies_unknown_key_on_unit_variant() {
    let bad = DslValue::object([
        ("kind".to_string(), DslValue::String("empty".to_string())),
        ("stray".to_string(), DslValue::Bool(true)),
    ]);
    assert!(InternallyTagged::from_value(bad).is_err());
}

#[test]
fn internally_tagged_denies_unknown_key_on_named_variant() {
    let bad = DslValue::object([
        ("kind".to_string(), DslValue::String("detailed".to_string())),
        ("name".to_string(), DslValue::String("a".to_string())),
        ("count".to_string(), DslValue::int(3)),
        ("extra".to_string(), DslValue::Bool(true)),
    ]);
    assert!(InternallyTagged::from_value(bad).is_err());
}

#[test]
fn internally_tagged_newtype_variant_strips_tag_before_reaching_payloads_own_deny_check() {
    let good = DslValue::object([
        ("kind".to_string(), DslValue::String("wrapped".to_string())),
        ("value".to_string(), DslValue::int(5)),
    ]);
    assert_eq!(InternallyTagged::from_value(good), Ok(InternallyTagged::Wrapped(InnerPayload { value: 5 })));
}

#[test]
fn internally_tagged_newtype_variant_payloads_own_deny_check_still_applies() {
    let bad = DslValue::object([
        ("kind".to_string(), DslValue::String("wrapped".to_string())),
        ("value".to_string(), DslValue::int(5)),
        ("extra".to_string(), DslValue::Bool(true)),
    ]);
    assert!(InternallyTagged::from_value(bad).is_err());
}

#[test]
fn internally_tagged_without_attribute_accepts_unknown_key() {
    let permissive = DslValue::object([
        ("kind".to_string(), DslValue::String("detailed".to_string())),
        ("name".to_string(), DslValue::String("a".to_string())),
        ("extra".to_string(), DslValue::Bool(true)),
    ]);
    assert_eq!(InternallyTaggedLax::from_value(permissive), Ok(InternallyTaggedLax::Detailed { name: "a".to_string() }));
}
//#endregion 🔖️InternallyTagged
