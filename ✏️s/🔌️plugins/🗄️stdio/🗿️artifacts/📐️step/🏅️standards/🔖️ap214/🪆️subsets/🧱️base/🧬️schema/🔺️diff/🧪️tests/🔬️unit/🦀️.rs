use super::*;

#[semio_framework_async_macros::async_test]
async fn invalid_collection_targets_are_rejected_before_mutation() {
    let base = StepSnapshot::default();
    let diff = StepDiff { entities: Some(StepEntitiesDiff { removed: vec![1], ..Default::default() }), ..Default::default() };
    let error = diff.apply(&base).expect_err("missing entity target must be rejected");
    assert_eq!(error.code, "invalid-remove-target");
    assert_eq!(error.target, vec!["entities", "1"]);
    assert_eq!(base, StepSnapshot::default());
}
use crate::schema::snapshot::{StepFileDescription, StepFileName, StepFileSchema, StepHeader};
use crate::STDIO_STEP_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
    StepEntity { id, name: name.into(), args, complex: Vec::new() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> StepSnapshot {
    StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: StepHeader {
            file_description: StepFileDescription { description: vec!["".into()], implementation_level: "2;1".into() },
            file_name: StepFileName {
                name: "a.step".into(),
                timestamp: "2026-08-10T00:00:00".into(),
                author: vec!["Ueli".into()],
                organization: vec!["semio".into()],
                preprocessor_version: "semio".into(),
                originating_system: "".into(),
                authorization: "".into(),
            },
            file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
        },
        entities: vec![
            entity(1, "CARTESIAN_POINT", vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(0.0), StepValue::Real(0.0), StepValue::Real(0.0)])]),
            entity(2, "CARTESIAN_POINT", vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(1.0), StepValue::Real(0.0), StepValue::Real(0.0)])]),
            entity(3, "DIRECTION", vec![StepValue::String("".into()), StepValue::Reference(99)]),
        ],
    }
}

/// 🧪️ Canonical absorb case 1: `InsertEntity(2,e)` then `RemoveEntity(base-id-at-0)` →
/// removed base id survives, added index shifts down by one.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let e = entity(50, "THING", vec![]);
    let d1 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 2, entity: e.clone() }], ..Default::default() };
    let d2 = StepEntitiesDiff { removed: vec![1], ..Default::default() };
    let d1 = absorb_entities(Some(d1), Some(d2)).expect("absorb of two non-empty diffs must be Some");
    assert_eq!(d1.removed, vec![1]);
    assert_eq!(d1.added, vec![StepEntityAdded { index: 1, entity: e }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `InsertEntity(2,e)` then `InsertEntity(2,f)` → BOTH survive.
/// Id-keyed `entities` (like zip's name-keyed `entries`) does not renumber colliding `added`
/// indices in the merged diff itself — `apply()`'s stable sort-by-index + sequential
/// `insert(at, ..)` is what resolves the collision: d1's entry (listed first) inserts at 2,
/// then d2's entry (listed second) also inserts at 2, pushing d1's entry to 3. Applying the
/// merged diff proves both survive at the right FINAL positions even though the stored
/// `index` fields are both still `2`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let e = entity(50, "A", vec![]);
    let f = entity(51, "B", vec![]);
    let d1 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 2, entity: e.clone() }], ..Default::default() };
    let d2 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 2, entity: f.clone() }], ..Default::default() };
    let d1 = absorb_entities(Some(d1), Some(d2)).expect("absorb of two non-empty diffs must be Some");
    assert_eq!(d1.added, vec![StepEntityAdded { index: 2, entity: e.clone() }, StepEntityAdded { index: 2, entity: f.clone() }]);
    let base = vec![entity(1, "BASE0", vec![]), entity(2, "BASE1", vec![])];
    let applied = d1.apply(&base);
    let pos_e = applied.iter().position(|x| x.id == 50).expect("e survives");
    let pos_f = applied.iter().position(|x| x.id == 51).expect("f survives");
    assert_eq!(pos_f, 2, "later-absorbed insert (f) lands at the target index");
    assert_eq!(pos_e, 3, "earlier insert (e) is pushed one position later by f");
}

/// 🧪️ Canonical absorb case 3: `InsertEntity(1,e)` then `SetEntityName(e.id, "X")` patches
/// INTO the added payload.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let e = entity(50, "A", vec![]);
    let d1 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 1, entity: e.clone() }], ..Default::default() };
    let d2 = StepEntitiesDiff { modified: vec![StepEntityModified { id: 50, diff: StepEntityDiff { name: Some("X".into()), ..Default::default() } }], ..Default::default() };
    let d1 = absorb_entities(Some(d1), Some(d2)).expect("absorb of two non-empty diffs must be Some");
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert_eq!(d1.added[0].entity.name, "X");
    assert_eq!(d1.added[0].index, 1);
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_holds_over_curated_ops() {
    let base = base_snapshot();
    let mid = {
        let mut s = base.clone();
        s.entities.insert(1, entity(60, "NEW", vec![StepValue::Unset]));
        s.entities.retain(|e| e.id != 1);
        s
    };
    let after = {
        let mut s = mid.clone();
        if let Some(e) = s.entities.iter_mut().find(|e| e.id == 60) {
            e.args.push(StepValue::Integer(7));
        }
        s.entities.push(entity(70, "MORE", vec![]));
        s
    };
    let mut d1 = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&base, &mid);
    let d2 = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert_eq!(d1.apply(&base).expect("valid absorbed diff"), after);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = base_snapshot();
    let mut b = a.clone();
    b.entities.push(entity(4, "EXTRA", vec![StepValue::Enum("T".into())]));
    b.header.file_schema.schemas.push("CONFIG_CONTROL_DESIGN".into());
    let ab = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&a, &b);
    assert_eq!(ab.apply(&a).expect("valid forward diff"), b);
    let ba = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&b, &a);
    assert_eq!(ba.apply(&b).expect("valid backward diff"), a);
    assert!(<StepDiff as DiffAlgebra<StepSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = base_snapshot();
    let next = {
        let mut s = base.clone();
        s.entities[0].name = "RENAMED_POINT".into();
        s.entities.remove(2);
        s.entities.push(entity(9, "NEWTHING", vec![StepValue::Derived]));
        s.header.file_name.originating_system = "semio".into();
        s
    };
    let d = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&base, &next);
    let mutated = d.apply(&base).expect("valid forward diff");
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&mutated).expect("valid inverse diff"), base);
}

/// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
/// field, with asymmetric collection lengths split across both `between()` directions (F1's
/// structural trap — a single index/id-keyed `between()` call can show `removed` XOR `added`,
/// never both, from one direction alone).
#[semio_framework_async_macros::async_test]
async fn field_sweep_covers_every_mutable_field() {
    let sweep_a = StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: StepHeader {
            file_description: StepFileDescription { description: vec!["a".into()], implementation_level: "2;1".into() },
            file_name: StepFileName {
                name: "a.step".into(),
                timestamp: "2026-01-01T00:00:00".into(),
                author: vec!["A".into()],
                organization: vec!["OrgA".into()],
                preprocessor_version: "pvA".into(),
                originating_system: "sysA".into(),
                authorization: "authA".into(),
            },
            file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
        },
        entities: vec![entity(1, "CARTESIAN_POINT", vec![StepValue::String("p1".into()), StepValue::Real(1.0)]), entity(2, "TO_REMOVE", vec![StepValue::Unset])],
    };
    let sweep_b = StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: StepHeader {
            file_description: StepFileDescription { description: vec!["b".into(), "b2".into()], implementation_level: "2;2".into() },
            file_name: StepFileName {
                name: "b.step".into(),
                timestamp: "2026-02-02T00:00:00".into(),
                author: vec!["B".into()],
                organization: vec!["OrgB".into()],
                preprocessor_version: "pvB".into(),
                originating_system: "sysB".into(),
                authorization: "authB".into(),
            },
            file_schema: StepFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] },
        },
        entities: vec![entity(1, "RENAMED_POINT", vec![StepValue::String("p1changed".into()), StepValue::Real(2.0), StepValue::Enum("T".into())]), entity(3, "ADDED_ENTITY", vec![StepValue::Reference(1)]), entity(4, "ANOTHER_ADDED", vec![])],
    };

    let ab = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&sweep_a, &sweep_b);
    assert_eq!(ab.apply(&sweep_a).expect("valid forward sweep diff"), sweep_b);
    assert!(ab.file_description.is_some());
    assert!(ab.file_name.is_some());
    assert!(ab.file_schema.is_some());
    let entities_ab = ab.entities.as_ref().expect("entities must differ");
    assert!(!entities_ab.removed.is_empty(), "sweep must exercise a removed entity (id 2 absent from b)");
    assert!(!entities_ab.modified.is_empty(), "sweep must exercise a modified entity (id 1 changed)");
    assert!(!entities_ab.added.is_empty(), "sweep must exercise an added entity (b has ids 3,4)");
    let e1_diff = &entities_ab.modified.iter().find(|m| m.id == 1).expect("id 1 modified").diff;
    assert!(e1_diff.name.is_some());
    let args_diff = e1_diff.args.as_ref().expect("args must differ");
    assert!(!args_diff.modified.is_empty(), "arg 0/1 changed value");
    assert!(!args_diff.added.is_empty(), "arg 2 added (b's entity 1 has 3 args, a's has 2)");

    let ba = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&sweep_b, &sweep_a);
    assert_eq!(ba.apply(&sweep_b).expect("valid backward sweep diff"), sweep_a);
    let entities_ba = ba.entities.as_ref().expect("entities must differ");
    assert!(!entities_ba.removed.is_empty(), "reverse direction must exercise removed (ids 3,4 absent from a)");
    assert!(!entities_ba.added.is_empty(), "reverse direction must exercise added (id 2 absent from b)");
    let e1_diff_ba = &entities_ba.modified.iter().find(|m| m.id == 1).expect("id 1 modified").diff;
    let args_diff_ba = e1_diff_ba.args.as_ref().expect("args must differ");
    assert!(!args_diff_ba.removed.is_empty(), "reverse direction must exercise a removed arg");

    assert!(<StepDiff as DiffAlgebra<StepSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
}
