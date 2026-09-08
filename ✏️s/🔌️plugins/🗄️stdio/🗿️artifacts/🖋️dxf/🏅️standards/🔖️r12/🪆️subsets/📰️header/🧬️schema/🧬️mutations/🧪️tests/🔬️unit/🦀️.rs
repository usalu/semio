
use super::*;
use crate::schema::diff::DxfEntitiesDiff;
use crate::schema::snapshot::{DxfOtherTable, DxfTables, DxfTag, DxfValue, DxfVertex};
use protocol::command::DiffAlgebra;

#[semio_framework_async_macros::async_test]
async fn missing_entity_target_is_rejected_before_mutation() {
    let base = DxfSnapshot::default();
    let diff = DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![0], ..Default::default() }), ..Default::default() };
    let error = diff.apply(&base).expect_err("missing entity target must be rejected");
    assert_eq!(error.code, "invalid-remove-index");
    assert_eq!(error.target, vec!["entities", "0"]);
    assert_eq!(base, DxfSnapshot::default());
}

//#region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> DxfSnapshot {
    DxfSnapshot {
        schema: "stdio.dxf".into(),
        header_vars: vec![DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] }],
        tables: DxfTables {
            layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }],
            styles: vec![DxfStyle { name: "STANDARD".into(), flags: 0, font_name: "txt".into(), unknown_group_codes: vec![] }],
            linetypes: vec![DxfLinetype { name: "CONTINUOUS".into(), flags: 0, description: "Solid".into(), unknown_group_codes: vec![] }],
        },
        other_tables: vec![],
        blocks: vec![DxfBlock { name: "B1".into(), base_point: [0.0, 0.0, 0.0], entities: vec![], unknown_group_codes: vec![] }],
        entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }, DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 5.0, layer: "0".into(), unknown_group_codes: vec![] }],
    }
}

/// 🔁️ Every DxfMutation-generic property test below (`mutation_diff_law`/`inverse_law`/
/// `op_text_binary_roundtrip_law`) shares ONE fixture with `⚙️engine/🦀️.rs`'s
/// conformance laws — `demo_mutation_cases()` (`#region 🔖️DemoCases` above).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn variants() -> Vec<DxfMutation> {
    demo_mutation_cases()
}
//#endregion 🔖️Fixtures

//#region 🔖️FieldSweepFixtures
/// 🧬️ Canonical "differs in every mutable field" snapshot A — every collection carries a
/// stable-prefix item plus one that will be modified (index-keyed) or removed (name-keyed).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> DxfSnapshot {
    DxfSnapshot {
        schema: "stdio.dxf".into(),
        header_vars: vec![
            DxfHeaderVar { name: "$KEEP".into(), group_code: 1, value: DxfValue::Str { value: "same".into() }, extra_group_codes: vec![] },
            DxfHeaderVar { name: "$DROP".into(), group_code: 70, value: DxfValue::Int { value: 1 }, extra_group_codes: vec![] },
            DxfHeaderVar { name: "$MOD".into(), group_code: 40, value: DxfValue::Double { value: 1.0 }, extra_group_codes: vec![] },
        ],
        tables: DxfTables {
            layers: vec![
                DxfLayer { name: "KEEP".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                DxfLayer { name: "DROP".into(), color: 1, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                DxfLayer { name: "MOD".into(), color: 2, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
            ],
            styles: vec![DxfStyle { name: "S".into(), flags: 0, font_name: "a".into(), unknown_group_codes: vec![] }],
            linetypes: vec![DxfLinetype { name: "L".into(), flags: 0, description: "d".into(), unknown_group_codes: vec![] }],
        },
        other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
        blocks: vec![
            DxfBlock { name: "B0".into(), base_point: [0.0, 0.0, 0.0], entities: vec![], unknown_group_codes: vec![] },
            DxfBlock { name: "B1".into(), base_point: [1.0, 1.0, 1.0], entities: vec![DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] },
        ],
        entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }, DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }],
    }
}

/// 🧬️ Sweep B: every index-keyed collection's index-0 item is UNCHANGED, index-1 is MODIFIED
/// in every field, and a brand-new item appears at the end — proven ADDED via `between(a,b)`
/// (`b` is longer) and REMOVED via `between(b,a)`. Name-keyed collections show removed +
/// modified + added simultaneously from ONE `between(a,b)` call. `entities[1]` changes KIND
/// (Circle → Text), proving `DxfEntityDiff::Replace`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> DxfSnapshot {
    DxfSnapshot {
        schema: "stdio.dxf".into(),
        header_vars: vec![
            DxfHeaderVar { name: "$KEEP".into(), group_code: 1, value: DxfValue::Str { value: "same".into() }, extra_group_codes: vec![] },
            DxfHeaderVar { name: "$MOD".into(), group_code: 41, value: DxfValue::Point { value: [1.0, 2.0, 3.0] }, extra_group_codes: vec![(999, DxfValue::Str { value: "note".into() })] },
            DxfHeaderVar { name: "$NEW".into(), group_code: 70, value: DxfValue::Int { value: 9 }, extra_group_codes: vec![] },
        ],
        tables: DxfTables {
            layers: vec![
                DxfLayer { name: "KEEP".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                DxfLayer { name: "MOD".into(), color: 5, linetype: "DASHED".into(), flags: 1, unknown_group_codes: vec![(1, DxfValue::Str { value: "x".into() })] },
                DxfLayer { name: "NEW".into(), color: 3, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
            ],
            styles: vec![DxfStyle { name: "S".into(), flags: 1, font_name: "b".into(), unknown_group_codes: vec![] }],
            linetypes: vec![DxfLinetype { name: "L".into(), flags: 1, description: "e".into(), unknown_group_codes: vec![] }],
        },
        other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
        blocks: vec![
            DxfBlock { name: "B0".into(), base_point: [0.0, 0.0, 0.0], entities: vec![], unknown_group_codes: vec![] },
            DxfBlock { name: "B1renamed".into(), base_point: [9.0, 9.0, 9.0], entities: vec![], unknown_group_codes: vec![(5, DxfValue::Str { value: "h".into() })] },
            DxfBlock { name: "B2".into(), base_point: [2.0, 2.0, 2.0], entities: vec![], unknown_group_codes: vec![] },
        ],
        entities: vec![
            DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Text { position: [1.0, 1.0, 1.0], height: 2.0, value: "swapped-kind".into(), layer: "T".into(), unknown_group_codes: vec![] },
            DxfEntity::Arc { center: [0.0, 0.0, 0.0], radius: 3.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] },
        ],
    }
}
//#endregion 🔖️FieldSweepFixtures

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    for m in variants() {
        let diff = m.diff(&base);
        let expected = diff.diff().apply(&base).expect("valid mutation diff");

        let mut via_apply = base.clone();
        let returned_diff = apply_dxf_mutation(&mut via_apply, &m);

        assert_eq!(via_apply, expected, "apply_dxf_mutation mismatch for {m:?}");
        assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = base_snapshot();
    for m in variants() {
        let mut forward = base.clone();
        apply_dxf_mutation(&mut forward, &m);
        for inv in m.inverse(&base) {
            apply_dxf_mutation(&mut forward, &inv);
        }
        assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

        let d = m.diff(&base);
        let mid = d.diff().apply(&base).expect("valid forward diff");
        let back = d.diff().inverse(&base).apply(&mid).expect("valid inverse diff");
        assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    let base = base_snapshot();

    // 🧩 Insert(2)+Remove(0) on entities: the two-op sequence base → mid → after.
    let new_entity = DxfEntity::Arc { center: [0.0, 0.0, 0.0], radius: 1.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] };
    let d1 = DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 2, entity: new_entity.clone() }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: 0 }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Remove-before absorb mismatch");

    // 🧩 Insert(2,f)+Insert(2,g): both must survive.
    let d1 = DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 2, entity: new_entity.clone() }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let other_entity = DxfEntity::Text { position: [0.0, 0.0, 0.0], height: 1.0, value: "g".into(), layer: "0".into(), unknown_group_codes: vec![] };
    let d2 = DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 2, entity: other_entity }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Insert-same-index absorb mismatch");
    assert_eq!(after.entities.len(), base.entities.len() + 2, "both inserts must survive");

    // 🧩 Add+SetField (kind-preserving): patch into the added payload.
    let d1 = DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: new_entity.clone() }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let patched = DxfEntity::Arc { center: [9.0, 9.0, 9.0], radius: 1.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] };
    let d2 = DxfMutation::SetEntity(set_entity::SetEntity { index: 1, entity: patched }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+SetField absorb mismatch");
    match &after.entities[1] {
        DxfEntity::Arc { center, .. } => assert_eq!(*center, [9.0, 9.0, 9.0]),
        other => panic!("expected Arc, got {other:?}"),
    }

    // 🧩 Add+SetField ACROSS a kind change: patch-into-Replace (the entity-diff-specific
    // canonical case — SetEntity with a different kind produces `Replace`, which must still
    // absorb cleanly into a preceding Insert's carried payload).
    let d1 = DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 1, entity: new_entity.clone() }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let swapped = DxfEntity::Text { position: [0.0, 0.0, 0.0], height: 3.0, value: "swap".into(), layer: "0".into(), unknown_group_codes: vec![] };
    let d2 = DxfMutation::SetEntity(set_entity::SetEntity { index: 1, entity: swapped.clone() }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+Replace(kind-change) absorb mismatch");
    assert_eq!(after.entities[1], swapped);

    // 🧩 Modify+Remove: modifying then removing the same entity collapses to a removal.
    let d1 = DxfMutation::SetEntity(set_entity::SetEntity { index: 1, entity: DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 9.0, layer: "0".into(), unknown_group_codes: vec![] } }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: 1 }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Modify+Remove absorb mismatch");

    // 🧩 Name-keyed: Add layer + remove-of-added annihilates the add.
    let d1 = DxfMutation::InsertLayer(insert_layer::InsertLayer { index: 2, layer: DxfLayer { name: "Fresh".into(), color: 1, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] } }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: "Fresh".into() }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+Remove(name-keyed) absorb mismatch");
    assert_eq!(after.tables.layers, base.tables.layers, "add-then-remove of the same name must be a full no-op");

    // 🧩 Associativity over a triple.
    let base = base_snapshot();
    let d1 = DxfMutation::InsertEntity(insert_entity::InsertEntity { index: 0, entity: new_entity.clone() }).diff(&base);
    let s1 = d1.diff().apply(&base).expect("valid first diff");
    let d2 = DxfMutation::SetEntity(set_entity::SetEntity { index: 0, entity: DxfEntity::Circle { center: [2.0, 2.0, 2.0], radius: 4.0, layer: "0".into(), unknown_group_codes: vec![] } }).diff(&s1);
    let s2 = d2.diff().apply(&s1).expect("valid second diff");
    let d3 = DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: 2 }).diff(&s2);
    let s3 = d3.diff().apply(&s2).expect("valid third diff");

    let mut left = d1.diff().clone();
    left.absorb(d2.diff().clone());
    left.absorb(d3.diff().clone());

    let mut d23 = d2.diff().clone();
    d23.absorb(d3.diff().clone());
    let mut right = d1.diff().clone();
    right.absorb(d23);

    assert_eq!(left.apply(&base).expect("valid left diff"), s3);
    assert_eq!(right.apply(&base).expect("valid right diff"), s3);
    assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "absorb must be associative");
}
//#endregion 🔖️AbsorbLaw

//#region 🔖️BetweenRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(DxfDiff::between(&a, &b).apply(&a).expect("valid forward diff"), b);
    assert_eq!(DxfDiff::between(&b, &a).apply(&b).expect("valid backward diff"), a);
    assert!(DxfDiff::between(&a, &a).is_empty());
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️FieldSweep
#[semio_framework_async_macros::async_test]
async fn field_sweep_every_mutable_field_changes() {
    let a = sweep_a();
    let b = sweep_b();

    let d_ab = DxfDiff::between(&a, &b);
    assert_eq!(d_ab.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");
    let d_ba = DxfDiff::between(&b, &a);
    assert_eq!(d_ba.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");
    assert!(DxfDiff::between(&a, &a).is_empty());

    // 🔍 header_vars (name-keyed): removed + modified + added from ONE between(a,b) call.
    let hv = d_ab.header_vars.as_ref().expect("header_vars diff populated");
    assert_eq!(hv.removed, vec!["$DROP".to_string()]);
    assert!(!hv.modified.is_empty() && !hv.added.is_empty());
    let hvm = &hv.modified.iter().find(|m| m.name == "$MOD").expect("$MOD modified").diff;
    assert!(hvm.group_code.is_some() && hvm.value.is_some() && hvm.extra_group_codes.is_some(), "every DxfHeaderVarDiff field must be patched");

    // 🔍 layers (name-keyed).
    let ld = d_ab.tables.as_ref().and_then(|t| t.layers.as_ref()).expect("layers diff populated");
    assert_eq!(ld.removed, vec!["DROP".to_string()]);
    assert!(!ld.modified.is_empty() && !ld.added.is_empty());
    let lm = &ld.modified.iter().find(|m| m.name == "MOD").expect("MOD layer modified").diff;
    assert!(lm.color.is_some() && lm.linetype.is_some() && lm.flags.is_some() && lm.unknown_group_codes.is_some());

    // 🔍 styles/linetypes (name-keyed, single-entry modify).
    let sd = d_ab.tables.as_ref().and_then(|t| t.styles.as_ref()).expect("styles diff populated");
    assert!(!sd.modified.is_empty());
    assert!(sd.modified[0].diff.flags.is_some() && sd.modified[0].diff.font_name.is_some());
    let ltd = d_ab.tables.as_ref().and_then(|t| t.linetypes.as_ref()).expect("linetypes diff populated");
    assert!(!ltd.modified.is_empty());
    assert!(ltd.modified[0].diff.flags.is_some() && ltd.modified[0].diff.description.is_some());

    // 🔍 blocks (index-keyed): modified+added from between(a,b); modified+removed from between(b,a).
    let bd_ab = d_ab.blocks.as_ref().expect("blocks diff populated (a->b)");
    assert!(bd_ab.removed.is_empty() && !bd_ab.modified.is_empty() && !bd_ab.added.is_empty());
    let bm = &bd_ab.modified[0].diff;
    assert!(bm.name.is_some() && bm.base_point.is_some() && bm.unknown_group_codes.is_some(), "every DxfBlockDiff scalar field must be patched");
    let bd_ba = d_ba.blocks.as_ref().expect("blocks diff populated (b->a)");
    assert!(!bd_ba.removed.is_empty() && !bd_ba.modified.is_empty() && bd_ba.added.is_empty());

    // 🔍 entities (index-keyed): modified (kind-preserving Line stays Line) + added from
    // between(a,b); Text(index 1) proves the kind-change `Replace` path.
    let ed_ab = d_ab.entities.as_ref().expect("entities diff populated (a->b)");
    assert!(ed_ab.removed.is_empty() && !ed_ab.modified.is_empty() && !ed_ab.added.is_empty());
    let em1 = &ed_ab.modified.iter().find(|m| m.index == 1).expect("entities[1] modified").diff;
    assert!(matches!(em1, crate::schema::diff::DxfEntityDiff::Replace { .. }), "kind change (Circle->Text) must be a Replace");
    let ed_ba = d_ba.entities.as_ref().expect("entities diff populated (b->a)");
    assert!(!ed_ba.removed.is_empty() && !ed_ba.modified.is_empty() && ed_ba.added.is_empty());
}
//#endregion 🔖️FieldSweep

//#region 🔖️VertexUnknownGroupCodesRetained
/// 🕳️ `DxfVertex.unknown_group_codes` participates in equality (weak leaf, whole-vec
/// replaced by the parent `Polyline` diff — confirms it isn't silently dropped by the
/// snapshot type even though no dedicated mutation targets it directly).
#[semio_framework_async_macros::async_test]
async fn vertex_unknown_group_codes_are_part_of_equality() {
    let v1 = DxfVertex { x: 0.0, y: 0.0, z: 0.0, bulge: 0.0, unknown_group_codes: vec![] };
    let v2 = DxfVertex { x: 0.0, y: 0.0, z: 0.0, bulge: 0.0, unknown_group_codes: vec![(40, DxfValue::Double { value: 1.0 })] };
    assert_ne!(v1, v2);
}
//#endregion 🔖️VertexUnknownGroupCodesRetained

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ `OpText`/`OpBinary` round-trip laws over the hand-rolled `DxfMutation` grammar — every
/// variant from the existing `variants()` fixture, including `SetSnapshot` (exercises the
/// whole-snapshot grammar, incl. `other_tables` raw retention and a nested block's own
/// entities) and every typed-entity/table Insert/Set/Remove keyword.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for m in variants() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must never contain a newline, for {m:?}");
        let parsed = DxfMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e:?}, for {m:?}"));
        assert_eq!(parsed, m, "parse_op(print_op(m)) == m");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e:?}, for {m:?}"));
        let decoded = DxfMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e:?}, for {m:?}"));
        assert_eq!(decoded, m, "decode_op(encode_op(m)) == m");

        let printed2 = m.print_op();
        assert_eq!(printed, printed2, "print_op must be deterministic, for {m:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️KindsCatalogLaw
/// 🧾️ `KINDS` matches the enum's own variant set (via `demo_mutation_cases`' one-instance-per-
/// variant coverage) and every entry parses/prints as its own keyword -- what keeps
/// `../../🔮️oracle/🔣️.json`'s `mutationCatalogs[].kinds` honest against Rust, per the
/// wave 7 fleet brief's registration rule ("the framework never parses Rust").
#[semio_framework_async_macros::async_test]
async fn kinds_const_matches_enum_variants_in_declaration_order() {
    assert_eq!(KINDS.len(), 18, "DxfMutation has 18 variants");
    let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for m in demo_mutation_cases() {
        let printed = m.print_op();
        let keyword = printed.split(' ').next().unwrap_or_default();
        assert!(KINDS.contains(&keyword), "KINDS is missing {keyword:?} for {m:?}");
        seen.insert(keyword.to_string());
    }
    assert_eq!(seen.len(), KINDS.len(), "every KINDS entry must be exercised by demo_mutation_cases()");
    for kind in KINDS {
        assert!(seen.contains(*kind), "KINDS entry {kind:?} has no demo_mutation_cases() coverage");
    }
}
//#endregion 🔖️KindsCatalogLaw
