use crate::standards::v1::subsets::any::io::text::snapshot::*;
use crate::{Path, Step};
use std::collections::BTreeMap as StdBTreeMap;

fn step(id: &str, kind: &str) -> Step {
    Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: StdBTreeMap::new() }
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_dsl_round_trips() {
    let document = parse_dsl(PROCEDURE_EXAMPLE_TEXT).expect("parse the demo asset");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

//#region DSL text round trips and error paths
/// 🔁️ Programs with nested `control.*` bodies round-trip losslessly through the `flow` child converter, and the parent naming
/// that program satisfies the DSL/pack round-trip laws (it carries the handles only).
#[semio_framework_async_macros::async_test]
async fn flow_content_round_trips_nested_control_bodies() {
    let inner = step("step-inner", "log.print");
    let mut owner = step("step-if", "control.if");
    owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
    let path = Path { steps: vec![owner] };

    let restored = crate::path_from_flow_content_snapshot(&crate::flow_content_snapshot_from_path(&path));
    assert_eq!(restored, path);
    assert_eq!(restored.steps[0].bodies.get("then").map(|body| body.steps.len()), Some(1));

    let seed = StdBTreeMap::from([("counter".into(), Value::Atom(Atom::Integer(1))), ("label".into(), Value::Atom(Atom::String("x".into())))]);
    let document = crate::procedure_snapshot_naming(&path, &seed);
    store::os_store::test_support::assert_dsl_round_trip(&document);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    assert_pack_keeps_the_handles(&document);
    neural_engine::ColdRetire::retire_cold(seed);
}

/// 🔁️ The `seed` converter round-trips every `Value`/`Atom` variant through the `text` child.
#[semio_framework_async_macros::async_test]
async fn text_content_round_trips_dictionary_and_atom_variants() {
    let seed = StdBTreeMap::from([
        ("a".into(), Value::Atom(Atom::Null)),
        ("b".into(), Value::Atom(Atom::Boolean(true))),
        ("c".into(), Value::Atom(Atom::Boolean(false))),
        ("d".into(), Value::Atom(Atom::Decimal(1.5))),
        ("e".into(), Value::Atom(Atom::Decimal(-1.0))),
        ("f".into(), Value::Dictionary(Dictionary::new())),
    ]);
    let restored = crate::seed_from_text_content_snapshot(&crate::text_content_snapshot_from_seed(&seed));
    assert_eq!(restored, seed);
    let document = crate::procedure_snapshot_naming(&Path::new(), &seed);
    store::os_store::test_support::assert_dsl_round_trip(&document);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    assert_pack_keeps_the_handles(&document);
    neural_engine::ColdRetire::retire_cold(restored);
    neural_engine::ColdRetire::retire_cold(seed);
}

/// 🚫️ A malformed body is refused.
#[semio_framework_async_macros::async_test]
async fn dsl_rejects_malformed_hex_value() {
    assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl("schema=zzz").is_err());
}

/// 🚫️ An unrecognized body line is refused.
#[semio_framework_async_macros::async_test]
async fn dsl_rejects_unrecognized_body_line() {
    assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(r#"notimperative schema="x""#).is_err());
}

/// 🚫️ A body missing a required line is refused.
#[semio_framework_async_macros::async_test]
async fn dsl_rejects_incomplete_body_missing_required_line() {
    assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl("schema=696d70657261746976652e646f63756d656e74").is_err());
}
//#endregion DSL text round trips and error paths

//#region 🪆️HandlesOnly
/// 🪆️ The parent text names the two children and nothing else (design §20.15): no program or seed literal is printed, and a
/// legacy `path=`/`seed=` field is refused.
#[semio_framework_async_macros::async_test]
async fn the_parent_text_carries_only_the_child_handles() {
    let printed = print_dsl(&crate::schema::default_snapshot());
    assert!(!printed.contains("path=") && !printed.contains("seed="), "{printed}");
    assert!(parse_dsl(&format!("{printed} path={{}}")).is_err(), "a content field is not part of the parent grammar");
}

/// 🎬️ The demo asset names the demo children, whose derivable content is the default program the Steps table lists.
#[semio_framework_async_macros::async_test]
async fn the_demo_asset_names_the_default_program() {
    let parsed = parse_dsl(PROCEDURE_EXAMPLE_TEXT).expect("parse example");
    assert_eq!((parsed.flow.child_id.as_str(), parsed.text.child_id.as_str()), (crate::examples::demo::FLOW_CHILD_ID, crate::examples::demo::TEXT_CHILD_ID));
    assert_eq!(crate::procedure_derivable_scene(&parsed).expect("the demo is derivable").path, crate::schema::default_path());
}
//#endregion 🪆️HandlesOnly

/// 🧬️ The derived pack declares its schema identity and carries the exact child handles.
fn assert_pack_keeps_the_handles(document: &ProcedureSnapshot) {
    store::os_store::test_support::assert_pack_schema_identity(document);
    let decoded: ProcedureSnapshot = store::ArtifactPack::decode_pack(&store::ArtifactPack::encode_pack(document)).expect("decode");
    assert_eq!((decoded.flow.child_id.as_str(), decoded.text.child_id.as_str()), (document.flow.child_id.as_str(), document.text.child_id.as_str()));
}
