use super::*;
use crate::{Dictionary as DocDictionary, Step as DocStep};
use std::collections::BTreeMap as StdBTreeMap;

fn step(id: &str, kind: &str) -> DocStep {
    DocStep { id: id.into(), kind: kind.into(), params: DocDictionary::new(), bodies: StdBTreeMap::new() }
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_dsl_round_trips() {
    let document = parse_dsl(PROCEDURE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

//#region DSL text round trips and error paths
/// 🔁 Replaces the retired `dsl_parses_seed_and_nested_control_bodies` — the artifact's own DSL
/// text no longer carries `path`/`seed` content directly (only the opaque `flow`/`text` composed
/// child HANDLES do, ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`), so the equivalent
/// real-behavior law now lives at the CONVERTER: `Path`s with nested `control.*` bodies round-trip
/// losslessly through `flow_content_snapshot_from_path`/`path_from_flow_content_snapshot`, and the
/// full snapshot (built from that `Path`) still satisfies the DSL/pack round-trip laws.
#[semio_framework_async_macros::async_test]
async fn flow_content_round_trips_nested_control_bodies() {
    let inner = step("step-inner", "log.print");
    let mut owner = step("step-if", "control.if");
    owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
    let path = Path { steps: vec![owner] };

    let flow_snapshot = crate::flow_content_snapshot_from_path(&path);
    let restored = crate::path_from_flow_content_snapshot(&flow_snapshot);
    assert_eq!(restored, path);
    assert_eq!(restored.steps[0].bodies.get("then").map(|body| body.steps.len()), Some(1));

    let seed = StdBTreeMap::from([("counter".into(), Value::Atom(Atom::Integer(1))), ("label".into(), Value::Atom(Atom::String("x".into())))]);
    let document = crate::procedure_snapshot_with_content("procedure.document", &path, &seed);
    store::os_store::test_support::assert_dsl_round_trip(&document);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    assert_pack_keeps_owned_content(&document);
}

/// 🔁 Replaces the retired `dsl_parses_dictionary_and_atom_variants` — same rationale as
/// [`flow_content_round_trips_nested_control_bodies`], for `seed`'s `text_content_snapshot_from_seed`/
/// `seed_from_text_content_snapshot` converter and every `Value`/`Atom` variant it carries.
#[semio_framework_async_macros::async_test]
async fn text_content_round_trips_dictionary_and_atom_variants() {
    let seed = StdBTreeMap::from([
        ("a".into(), Value::Atom(Atom::Null)),
        ("b".into(), Value::Atom(Atom::Boolean(true))),
        ("c".into(), Value::Atom(Atom::Boolean(false))),
        ("d".into(), Value::Atom(Atom::Decimal(1.5))),
        ("e".into(), Value::Atom(Atom::Decimal(-1.0))),
        ("f".into(), Value::Dictionary(DocDictionary::new())),
    ]);

    let text_snapshot = crate::text_content_snapshot_from_seed(&seed);
    let restored = crate::seed_from_text_content_snapshot(&text_snapshot);
    assert_eq!(restored, seed);

    let document = crate::procedure_snapshot_with_content("procedure.document", &Path::new(), &seed);
    store::os_store::test_support::assert_dsl_round_trip(&document);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    assert_pack_keeps_owned_content(&document);
}

/// 🔁 Retired-format twin was `dsl_rejects_unterminated_string`; the new hand-rolled body grammar
/// has no quoted-string literals, so the equivalent rejection is a malformed hex value (odd length).
#[semio_framework_async_macros::async_test]
async fn dsl_rejects_malformed_hex_value() {
    let text = "schema=zzz";
    assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
}

/// 🔁 Retired-format twin was `dsl_rejects_wrong_leading_keyword`; the new hand-rolled body is
/// line-based (`schema=`/`flow=`/`text=`), not keyword-based, so the equivalent rejection is an
/// unrecognized line.
#[semio_framework_async_macros::async_test]
async fn dsl_rejects_unrecognized_body_line() {
    let text = r#"notimperative schema="x""#;
    assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
}

/// 🔁 Retired-format twin was `dsl_rejects_invalid_number_literal`; the new hand-rolled body
/// requires all three lines (`schema=`/`flow=`/`text=`), so the equivalent rejection is a body
/// missing a required line.
#[semio_framework_async_macros::async_test]
async fn dsl_rejects_incomplete_body_missing_required_line() {
    let text = "schema=696d70657261746976652e646f63756d656e74";
    assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
}
//#endregion DSL text round trips and error paths

//#region 🛤️ContentLines
fn seeded_program_snapshot() -> ProcedureSnapshot {
    let seed = StdBTreeMap::from([("counter".into(), Value::Atom(Atom::Integer(3)))]);
    crate::procedure_snapshot_with_content("procedure.document", &crate::procedure_working_scene(&crate::schema::default_snapshot()).path, &seed)
}

/// 🛤️ The `flow` and `text` children's content is the document, so `print_dsl`/`parse_dsl` must carry
/// it: `assert_dsl_round_trip` compares snapshots whose child handles compare by identity only, which is
/// exactly how the bare-handle body passed it while losing every step (ticket 26/09/19).
#[semio_framework_async_macros::async_test]
async fn the_program_survives_a_text_round_trip() {
    for document in [crate::schema::default_snapshot(), seeded_program_snapshot()] {
        let reparsed = parse_dsl(&print_dsl(&document)).expect("reparse");
        let (before, after) = (crate::procedure_working_scene(&document), crate::procedure_working_scene(&reparsed));
        assert!(!after.path.steps.is_empty(), "the parsed document owns its program");
        assert_eq!(after.path, before.path);
        assert_eq!(after.seed, before.seed);
    }
}

/// 🛤️ The pack twin of [`the_program_survives_a_text_round_trip`].
#[semio_framework_async_macros::async_test]
async fn the_program_survives_a_pack_round_trip() {
    use store::ArtifactPack;
    for document in [crate::schema::default_snapshot(), seeded_program_snapshot()] {
        let decoded = ProcedureSnapshot::decode_pack(&document.encode_pack()).expect("decode");
        let (before, after) = (crate::procedure_working_scene(&document), crate::procedure_working_scene(&decoded));
        assert_eq!(after.path, before.path);
        assert_eq!(after.seed, before.seed);
    }
}

/// 🎬️ The curated `demo` example is what `setActiveExample("demo")` loads into the play pane, so it
/// must carry the default program the Steps table lists.
#[semio_framework_async_macros::async_test]
async fn the_demo_asset_carries_the_default_program() {
    let parsed = parse_dsl(PROCEDURE_EXAMPLE_TEXT).expect("parse example");
    assert_eq!(crate::procedure_working_scene(&parsed).path, crate::procedure_working_scene(&crate::schema::default_snapshot()).path);
}

/// 📖️ `path` and `seed` are required fields of `📖️.grammar.semio`; the derived text prints them last.
#[semio_framework_async_macros::async_test]
async fn a_body_without_its_content_lines_is_refused() {
    let printed = print_dsl(&crate::schema::default_snapshot());
    for field in ["path=", "seed="] {
        let stripped = &printed[..printed.find(field).expect("printed content field")];
        assert!(parse_dsl(stripped).is_err(), "a body without its {field} field must be refused");
    }
}
//#endregion 🛤️ContentLines

/// 🧬️ The derived pack declares its schema identity and re-attaches the exact flow/text content.
fn assert_pack_keeps_owned_content(document: &ProcedureSnapshot) {
    store::os_store::test_support::assert_pack_schema_identity(document);
    let decoded: ProcedureSnapshot = store::ArtifactPack::decode_pack(&store::ArtifactPack::encode_pack(document)).expect("decode");
    let (before, after) = (crate::procedure_working_scene(document), crate::procedure_working_scene(&decoded));
    assert_eq!((after.path, after.seed), (before.path, before.seed));
}
