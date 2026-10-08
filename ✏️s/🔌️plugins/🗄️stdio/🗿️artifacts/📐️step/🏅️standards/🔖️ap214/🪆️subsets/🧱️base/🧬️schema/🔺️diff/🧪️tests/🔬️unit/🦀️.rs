use super::*;

#[semio_framework_async_macros::async_test]
async fn invalid_collection_targets_are_rejected_before_mutation() {
    let base = StepSnapshot::default();
    let diff = StepDiff { entities: Some(StepEntitiesDiff { removed: vec![1], ..Default::default() }), ..Default::default() };
    let error = protocol::apply_diff(&diff, &base).expect_err("missing entity target must be rejected");
    assert_eq!(error.code, "mutation.apply.invalid-remove-target");
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
            entity(3, "DIRECTION", vec![StepValue::String("".into()), StepValue::Reference(3)]),
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
    let applied = protocol::apply_diff(&d1, &base);
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

