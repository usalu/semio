//! 🆔 A single-field tuple struct (`struct Foo(pub u32);`, no `#[value(...)]` attribute at all)
//! derives `ToValue`/`FromValue` transparently — see `🦀️.rs`'s module docs' `#[value(transparent)]`
//! entry for why the tuple shape needs no attribute. Proves the wire form is byte-identical to the
//! inner field's own `ToValue`/`FromValue`, and that a round trip recovers the original value —
//! a genuine `tests/*.rs` integration crate, so `#[derive(ToValue, FromValue)]` runs exactly as any
//! downstream consumer's derive invocation would (a proc-macro crate cannot exercise its own
//! derives from inside its own `src`). Motivated by `id_newtype!` in
//! `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
//! 💡️inferences/🧩️wfc-engine/🆔️ids/🦀️.rs`, which generates exactly this shape. Ticket
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.

// 🌿️ See the sibling `🛡️deny-unknown-fields-enums.rs` test file's identical docstring for why
// `semio_framework_os_kernel` alone (not a separate `semio_framework_value_derive` import) is the
// correct single import here.
use semio_framework_os_kernel::{DslValue, FromValue, ToValue};

//#region 🔖️NewtypeU32
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToValue, FromValue)]
struct NodeId(pub u32);

#[test]
fn newtype_to_value_matches_inner_field() {
    let id = NodeId(42);
    assert_eq!(id.to_value(), 42u32.to_value());
    let DslValue::Number(_) = id.to_value() else { panic!("expected a bare number, no object wrapper") };
}

#[test]
fn newtype_round_trips_through_from_value() {
    let id = NodeId(7);
    assert_eq!(NodeId::from_value(id.to_value()), Ok(id));
}

#[test]
fn newtype_from_value_rejects_non_numeric() {
    assert!(NodeId::from_value(DslValue::String("nope".to_string())).is_err());
}
//#endregion 🔖️NewtypeU32

//#region 🔖️NewtypeString — proves the transparent forwarding is not u32-specific
#[derive(Debug, Clone, PartialEq, Eq, ToValue, FromValue)]
struct Slug(pub String);

#[test]
fn string_newtype_round_trips() {
    let slug = Slug("hello-world".to_string());
    assert_eq!(slug.to_value(), DslValue::String("hello-world".to_string()));
    assert_eq!(Slug::from_value(slug.to_value()), Ok(slug));
}
//#endregion 🔖️NewtypeString
