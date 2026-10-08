use super::*;
use crate::schema::diff::IfcEntitiesDiff;
use crate::schema::snapshot::{IfcComplexType, IfcHeader};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

#[test]
fn missing_entity_target_is_rejected_before_mutation() {
    let base = IfcSnapshot::default();
    let diff = IfcDiff { entities: Some(IfcEntitiesDiff { removed: vec![1], ..Default::default() }), ..Default::default() };
    let error = protocol::apply_diff(&diff, &base).expect_err("missing entity target must be rejected");
    assert_eq!(error.code, "mutation.apply.invalid-remove-target");
    assert_eq!(error.target, vec!["entities", "1"]);
    assert_eq!(base, IfcSnapshot::default());
}

//#region Fixtures
fn entity(id: u64, name: &str, args: Vec<IfcValue>) -> IfcEntity {
    IfcEntity { id, name: name.into(), args, complex: vec![] }
}

fn base_snapshot() -> IfcSnapshot {
    IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: IfcHeader { file_description: vec![IfcValue::String("".into())], file_name: vec![IfcValue::String("semio.ifc".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] },
        entities: vec![
            entity(1, "IFCPROJECT", vec![IfcValue::String("gid".into()), IfcValue::Reference(2)]),
            entity(2, "IFCOWNERHISTORY", vec![IfcValue::Unset, IfcValue::Integer(0)]),
            entity(6, "IFCWALL", vec![IfcValue::String("gid-wall".into()), IfcValue::Reference(2), IfcValue::String("Wall-01".into())]),
        ],
    }
}
//#endregion Fixtures

//#region 🔖️mutation_diff_law
fn assert_mutation_diff_law(base: &IfcSnapshot, mutation: IfcMutation) {
    let expected_diff = mutation.diff(base);
    let mut applied_snapshot = base.clone();
    let returned_diff = apply_ifc_mutation(&mut applied_snapshot, &mutation);
    assert_eq!(returned_diff, expected_diff, "apply_ifc_mutation must return mutation.diff(base) for {mutation:?}");
    assert_eq!(protocol::apply_diff(expected_diff.diff(), base).expect("valid mutation diff"), applied_snapshot, "protocol::apply_diff(diff.diff(), base) must equal the imperative mutation result for {mutation:?}");
}

#[test]
fn mutation_diff_law() {
    let base = base_snapshot();
    let mut alt = base.clone();
    alt.header.file_name = vec![IfcValue::String("other.ifc".into())];
    assert_mutation_diff_law(&base, IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("new desc".into())] }));
    assert_mutation_diff_law(&base, IfcMutation::SetFileName(set_file_name::SetFileName { values: vec![IfcValue::String("renamed.ifc".into())] }));
    assert_mutation_diff_law(&base, IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3".into())])] }));
    assert_mutation_diff_law(&base, IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(99, "IFCSITE", vec![IfcValue::Unset]) }));
    assert_mutation_diff_law(&base, IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }));
    assert_mutation_diff_law(&base, IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 6, name: "IFCSLAB".into() }));
    assert_mutation_diff_law(&base, IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 6, index: 2, value: IfcValue::String("Wall-02".into()) }));
    assert_mutation_diff_law(&base, IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 6, index: 1, value: IfcValue::Derived }));
    assert_mutation_diff_law(&base, IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 6, index: 0 }));
}
//#endregion 🔖️mutation_diff_law

//#region 🔖️inverse_law
#[test]
fn inverse_law() {
    let base = base_snapshot();
    let variants = vec![
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("changed".into())] }),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(99, "IFCSITE", vec![IfcValue::Unset]) }),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 6, name: "IFCSLAB".into() }),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 6, index: 2, value: IfcValue::String("Wall-02".into()) }),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 6, index: 1, value: IfcValue::Derived }),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 6, index: 0 }),
    ];
    for m in variants {
        let mut snap = base.clone();
        apply_ifc_mutation(&mut snap, &m);
        for inv in m.inverse(&base).expect("valid retained mutation inverse fixture") {
            apply_ifc_mutation(&mut snap, &inv);
        }
        assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

        let d = m.diff(&base);
        let mutated = protocol::apply_diff(d.diff(), &base).expect("valid forward diff");
        let inv_d = d.diff().inverse(&base);
        assert_eq!(protocol::apply_diff(&inv_d, &mutated).expect("valid inverse diff"), base, "diff-level inverse must restore base for {m:?}");
    }
}
//#endregion 🔖️inverse_law

//#region 🔖️absorb_law
fn assert_absorb_law(base: &IfcSnapshot, m1: IfcMutation, m2: IfcMutation) {
    let d1 = m1.diff(base);
    let mid = protocol::apply_diff(d1.diff(), base).expect("valid first diff");
    let d2 = m2.diff(&mid);
    let sequential = protocol::apply_diff(d2.diff(), &mid).expect("valid second diff");

    let mut merged = d1.diff().clone();
    merged.absorb(d2.diff().clone());
    assert_eq!(protocol::apply_diff(&merged, base).expect("valid absorbed diff"), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
}

#[test]
fn absorb_law() {
    let base = base_snapshot();

    // Insert+Remove-before: added entity's carried final index shifts once an earlier base
    // survivor is removed by the second mutation (the recipe's own canonical shift case).
    assert_absorb_law(&base, IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(100, "IFCSITE", vec![]) }), IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 1 }));

    // Insert+Insert-same-index: both survive.
    assert_absorb_law(
        &base,
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(100, "IFCSITE", vec![]) }),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(101, "IFCBUILDING", vec![]) }),
    );

    // Add+SetField: the second mutation patches directly into the still-pending added entity.
    assert_absorb_law(
        &base,
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 0, entity: entity(100, "IFCSITE", vec![IfcValue::Unset]) }),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 100, name: "IFCBUILDING".into() }),
    );

    // Modify+Remove: a pending field patch on a since-removed base entity vanishes.
    assert_absorb_law(&base, IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 6, name: "IFCSLAB".into() }), IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 6 }));

    // Insert then annihilate the very same insert.
    assert_absorb_law(&base, IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 0, entity: entity(100, "IFCSITE", vec![]) }), IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 100 }));

    // Insert-arg then set-that-same-arg patches into the still-pending added arg.
    assert_absorb_law(&base, IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 6, index: 0, value: IfcValue::Unset }), IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 6, index: 0, value: IfcValue::Derived }));

    // Two unrelated scalar sets absorb via LWW.
    assert_absorb_law(
        &base,
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("first".into())] }),
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("second".into())] }),
    );
}

#[test]
fn absorb_law_associativity() {
    let base = base_snapshot();
    let d1 = IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("one".into())] }).diff(&base);
    let mid1 = protocol::apply_diff(d1.diff(), &base).expect("valid first diff");
    let d2 = IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 0, entity: entity(100, "IFCSITE", vec![]) }).diff(&mid1);
    let mid2 = protocol::apply_diff(d2.diff(), &mid1).expect("valid second diff");
    let d3 = IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 100, name: "IFCBUILDING".into() }).diff(&mid2);

    let mut left = d1.diff().clone();
    left.absorb(d2.diff().clone());
    left.absorb(d3.diff().clone());

    let mut d23 = d2.diff().clone();
    d23.absorb(d3.diff().clone());
    let mut right = d1.diff().clone();
    right.absorb(d23);

    assert_eq!(protocol::apply_diff(&left, &base).expect("valid left diff"), protocol::apply_diff(&right, &base).expect("valid right diff"), "absorb must associate");
    assert_eq!(protocol::apply_diff(&left, &base).expect("valid associated diff"), protocol::apply_diff(d3.diff(), &mid2).expect("valid third diff"), "associated absorb must match full sequential application");
}
//#endregion 🔖️between_roundtrip_law

//#region 🔖️codec_retention_law
#[test]
fn codec_retention_law() {
    let document = semio_s_artifact_stdio_contract::part21::parse_part21(crate::examples::ifc4_demo::EXCHANGE_TEXT).expect("parse external IFC4 fixture");
    let decoded = crate::schema::snapshot::from_part21_document(crate::STDIO_IFC_DOCUMENT_SCHEMA, &document);
    let reencoded = store::ArtifactDsl::print_dsl(&decoded);
    let redecoded = <IfcSnapshot as store::ArtifactDsl>::parse_dsl(&reencoded).expect("re-decode fixture");
    assert_eq!(decoded.header, redecoded.header);
    assert_eq!(decoded.entities, redecoded.entities);
}
//#endregion 🔖️codec_retention_law

//#region 🔖️field_sweep
/// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field: HEADER's three records, one removed
/// entity, one entity modified in every field (name, an arg removed/modified/added, and
/// `complex` exercising the COMPLEX-instance weak-list replace), one added entity.
fn sweep_a() -> IfcSnapshot {
    IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: IfcHeader { file_description: vec![IfcValue::String("before desc".into())], file_name: vec![IfcValue::String("before.ifc".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] },
        entities: vec![
            entity(1, "IFCPROJECT", vec![IfcValue::String("gone".into())]),
            IfcEntity {
                id: 2,
                name: "IFCQUANTITYAREA".into(),
                args: vec![IfcValue::String("stay".into()), IfcValue::Real(1.0), IfcValue::Reference(9)],
                complex: vec![IfcComplexType { name: "IFCPHYSICALSIMPLEQUANTITY".into(), args: vec![IfcValue::Unset] }],
            },
        ],
    }
}

fn sweep_b() -> IfcSnapshot {
    IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: IfcHeader { file_description: vec![IfcValue::String("after desc".into())], file_name: vec![IfcValue::String("after.ifc".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3".into())])] },
        entities: vec![
            IfcEntity {
                id: 2,
                name: "IFCQUANTITYVOLUME".into(),
                // index 0 modified, index 1 removed (b is shorter here), index 2 unchanged
                // relative position collapses -- exercised precisely via direct field asserts
                // below rather than index-fragile equality.
                args: vec![IfcValue::String("changed".into()), IfcValue::Reference(9)],
                complex: vec![],
            },
            entity(300, "IFCBUILDINGSTOREY", vec![IfcValue::Real(3.0)]),
        ],
    }
}

//#endregion 🔖️field_sweep

#[test]
fn out_of_range_entity_mutation_is_rejected_without_mutating() {
    let base = base_snapshot();
    let mut snap = base.clone();
    let outcome = apply_ifc_mutation(&mut snap, &IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 404, name: "X".into() }));
    assert_eq!(snap, base);
    assert_eq!(outcome.messages()[0].target, vec!["entities", "404"]);
    let outcome = apply_ifc_mutation(&mut snap, &IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 404, index: 0 }));
    assert_eq!(snap, base);
    assert_eq!(outcome.messages()[0].target, vec!["entities", "404"]);
}

//#region 🔖️op_text_binary_roundtrip_law
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `IfcMutation` grammar —
/// exercises every variant incl. every `IfcValue`
/// tag (`Unset`/`Derived`/`Integer`/`Real`/`String`/`Enum`/`Reference`/`Aggregate`/`TypedValue`).
#[test]
fn op_text_binary_roundtrip_law() {
    let base = base_snapshot();
    let mutations = vec![
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("new desc".into())] }),
        IfcMutation::SetFileName(set_file_name::SetFileName { values: vec![IfcValue::Aggregate(vec![IfcValue::String("a".into()), IfcValue::Unset])] }),
        IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: vec![] }),
        IfcMutation::InsertEntity(insert_entity::InsertEntity {
            index: 1,
            entity: entity(
                99,
                "IFCSITE",
                vec![
                    IfcValue::Unset,
                    IfcValue::Derived,
                    IfcValue::Integer(-7),
                    IfcValue::Real(3.25),
                    IfcValue::String("hi".into()),
                    IfcValue::Enum("EDGE".into()),
                    IfcValue::Reference(42),
                    IfcValue::Aggregate(vec![IfcValue::Integer(1), IfcValue::Integer(2)]),
                    IfcValue::TypedValue { name: "IFCLENGTHMEASURE".into(), items: vec![IfcValue::Real(3000.0)] },
                ],
            ),
        }),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 6, name: "IFCSLAB".into() }),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 6, index: 2, value: IfcValue::String("Wall-02".into()) }),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 6, index: 1, value: IfcValue::Derived }),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 6, index: 0 }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = IfcMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = IfcMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️op_text_binary_roundtrip_law

//#region 🔖️kinds_const
/// 🧪️ Wave-7 gate: `KINDS` must match the enum's own variants, in declaration order, and its
/// spellings must match `print_op`'s own keyword for each — the two lists the mutation catalog
/// (`../../🔣️oracle.json`) and the feature file are checked against never drift apart.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let base = base_snapshot();
    let one_per_variant = vec![
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("d".into())] }),
        IfcMutation::SetFileName(set_file_name::SetFileName { values: vec![IfcValue::String("n".into())] }),
        IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] }),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 0, entity: entity(50, "IFCSITE", vec![]) }),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "RENAMED".into() }),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, index: 1, value: IfcValue::Real(9.0) }),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, index: 2, value: IfcValue::Enum("T".into()) }),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, index: 0 }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        let printed = mutation.print_op();
        let keyword = printed.split(' ').next().unwrap_or(&printed);
        assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
    }
}
//#endregion 🔖️kinds_const

#[semio_framework_async_macros::async_test]
async fn every_leaf_satisfies_the_inverse_sum_law() {
    let base = base_snapshot();
    for mutation in [
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("new desc".into())] }),
        IfcMutation::SetFileName(set_file_name::SetFileName { values: vec![IfcValue::String("renamed.ifc".into())] }),
        IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3".into())])] }),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: entity(9, "IFCSITE", vec![IfcValue::Unset]) }),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 6, name: "IFCSLAB".into() }),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 6, index: 2, value: IfcValue::String("Wall-02".into()) }),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 6, index: 1, value: IfcValue::Unset }),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 6, index: 0 }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
