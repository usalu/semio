use super::*;
use crate::schema::snapshot::{StepHeader, StepValue as SV};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
    StepEntity { id, name: name.into(), args, complex: Vec::new() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> StepSnapshot {
    StepSnapshot { schema: crate::STDIO_STEP_DOCUMENT_SCHEMA.into(), header: StepHeader::default(), entities: vec![entity(1, "CARTESIAN_POINT", vec![SV::String("".into()), SV::Real(1.0)]), entity(2, "DIRECTION", vec![SV::Unset])] }
}

/// 🧪️ `mutation_diff_law`: ∀ variant, `m.diff(base).diff().apply(base) == { apply(&mut s, m); s }`
/// and the returned diff equals `m.diff(base)`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_mutation_diff_law(base: &StepSnapshot, m: StepMutation) {
    let expected_diff = <StepMutation as Mutation<StepSnapshot>>::diff(&m, base);
    let expected_state = expected_diff.diff().apply(base).expect("valid mutation diff");
    let mut actual_state = base.clone();
    let actual_diff = apply_step_mutation(&mut actual_state, &m);
    assert_eq!(actual_diff, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
    assert_eq!(actual_state, expected_state, "applied state must match for {m:?}");
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_covers_every_variant() {
    let base = base_snapshot();
    let mut next = base.clone();
    next.entities[0].name = "X".into();
    assert_mutation_diff_law(&base, StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: next }));
    assert_mutation_diff_law(&base, StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: StepFileDescription { description: vec!["d".into()], implementation_level: "2;1".into() } }));
    assert_mutation_diff_law(&base, StepMutation::SetFileName(set_file_name::SetFileName { file_name: StepFileName { name: "n".into(), ..Default::default() } }));
    assert_mutation_diff_law(&base, StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["X".into()] } }));
    assert_mutation_diff_law(&base, StepMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(50, "NEW", vec![]) }));
    assert_mutation_diff_law(&base, StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }));
    assert_mutation_diff_law(&base, StepMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "RENAMED".into() }));
    assert_mutation_diff_law(&base, StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, arg_index: 1, value: SV::Real(9.0) }));
    assert_mutation_diff_law(&base, StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, arg_index: 2, value: SV::Enum("T".into()) }));
    assert_mutation_diff_law(&base, StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, arg_index: 0 }));
}

#[semio_framework_async_macros::async_test]
async fn missing_and_out_of_range_targets_are_rejected_without_mutating() {
    let base = base_snapshot();
    let mut snapshot = base.clone();
    let outcome = apply_step_mutation(&mut snapshot, &StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 999 }));
    assert_eq!(snapshot, base);
    assert_eq!(outcome.messages()[0].target, vec!["entities", "999"]);
    let outcome = apply_step_mutation(&mut snapshot, &StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, arg_index: 99 }));
    assert_eq!(snapshot, base);
    assert_eq!(outcome.messages()[0].target, vec!["entities", "1", "args", "99"]);
}

/// 🧪️ `inverse_law` (mutation level): every variant's `inverse()` round-trips.
#[semio_framework_async_macros::async_test]
async fn inverse_law_mutation_level_round_trips_every_variant() {
    let base = base_snapshot();
    let variants = vec![
        StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] } }),
        StepMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(50, "NEW", vec![SV::Integer(3)]) }),
        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        StepMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "RENAMED".into() }),
        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, arg_index: 1, value: SV::Real(42.0) }),
        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, arg_index: 2, value: SV::Enum("F".into()) }),
        StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, arg_index: 0 }),
    ];
    for m in variants {
        let mut state = base.clone();
        apply_step_mutation(&mut state, &m);
        let inverses = <StepMutation as Mutation<StepSnapshot>>::inverse(&m, &base);
        let mut restored = state.clone();
        for inv in &inverses {
            apply_step_mutation(&mut restored, inv);
        }
        assert_eq!(restored, base, "mutation-level inverse must restore base for {m:?}");
    }
}

/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `StepMutation` grammar —
/// exercises every variant incl. `InsertEntity`'s bare `StepEntity` payload and
/// `SetEntityArg`/`InsertEntityArg`'s bare `StepValue` payload (every `StepValue` variant,
/// incl. the recursive `Aggregate`/`TypedValue` cases).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = base_snapshot();
    let mutations = vec![
        StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: StepFileDescription { description: vec!["d1".into(), "d2".into()], implementation_level: "2;1".into() } }),
        StepMutation::SetFileName(set_file_name::SetFileName {
            file_name: StepFileName {
                name: "n.step".into(),
                timestamp: "2026-08-10T00:00:00".into(),
                author: vec!["A".into()],
                organization: vec!["O".into()],
                preprocessor_version: "pv".into(),
                originating_system: "sys".into(),
                authorization: "auth".into(),
            },
        }),
        StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into(), "CONFIG_CONTROL_DESIGN".into()] } }),
        StepMutation::InsertEntity(insert_entity::InsertEntity {
            index: 1,
            entity: entity(
                50,
                "NEW",
                vec![
                    SV::Unset,
                    SV::Derived,
                    SV::Integer(-42),
                    SV::Real(3.5),
                    SV::String("s".into()),
                    SV::Enum("T".into()),
                    SV::Reference(9),
                    SV::Aggregate(vec![SV::Integer(1), SV::Real(2.0)]),
                    SV::TypedValue { type_name: "LENGTH_MEASURE".into(), value: Box::new(SV::Real(3000.0)) },
                ],
            ),
        }),
        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        StepMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "RENAMED".into() }),
        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, arg_index: 1, value: SV::Aggregate(vec![SV::Real(1.0), SV::Real(2.0), SV::Real(3.0)]) }),
        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, arg_index: 2, value: SV::TypedValue { type_name: "X".into(), value: Box::new(SV::Aggregate(vec![SV::Integer(1), SV::Integer(2)])) } }),
        StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, arg_index: 0 }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = StepMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = StepMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}

/// 🧪️ Wave-7 gate: `KINDS` must match the enum's own variants, in declaration order, and its
/// spellings must match `print_op`'s own keyword for each — the two lists the mutation catalog
/// (`../🔣️oracle.json`) and the feature file are checked against never drift apart.
#[semio_framework_async_macros::async_test]
async fn kinds_const_matches_enum_variants_in_declaration_order() {
    let base = base_snapshot();
    let one_per_variant = vec![
        StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: StepFileDescription::default() }),
        StepMutation::SetFileName(set_file_name::SetFileName { file_name: StepFileName::default() }),
        StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: StepFileSchema::default() }),
        StepMutation::InsertEntity(insert_entity::InsertEntity { index: 0, entity: entity(50, "NEW", vec![]) }),
        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        StepMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "RENAMED".into() }),
        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, arg_index: 1, value: SV::Real(9.0) }),
        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, arg_index: 2, value: SV::Enum("T".into()) }),
        StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, arg_index: 0 }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        let printed = mutation.print_op();
        let keyword = printed.split(' ').next().unwrap_or(&printed);
        assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
    }
}
