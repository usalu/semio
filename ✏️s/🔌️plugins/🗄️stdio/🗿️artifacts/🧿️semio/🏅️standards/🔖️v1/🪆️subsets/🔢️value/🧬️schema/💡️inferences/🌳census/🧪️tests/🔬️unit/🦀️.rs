use super::*;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValueEntry, SemioValueNode, ValueId, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};

/// 🌱 A hand-built, non-empty graph: a 3-deep map/list root (Map -> List -> Str, depth 3) plus
/// one backing node holding a 2-deep value (Map -> Bool, depth 2) — exercises every variant and
/// a genuine max-depth comparison across root vs. nodes.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioValueSnapshot {
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map {
            entries: vec![
                SemioValueEntry { key: "tags".into(), value: SemioValue::List { items: vec![SemioValue::Str { value: "a".into() }, SemioValue::Int { lexeme: "1".into() }] } },
                SemioValueEntry { key: "linked".into(), value: SemioValue::Ref { id: ValueId::new("n1") } },
            ],
        },
        nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Map { entries: vec![SemioValueEntry { key: "flag".into(), value: SemioValue::Bool { value: true } }] } }],
    }
}

#[semio_framework_async_macros::async_test]
async fn tallies_every_variant_and_finds_the_true_max_depth() {
    let census = compute_semio_value_census(&populated());
    assert_eq!(census.map_count, 2, "root map + node's own map");
    assert_eq!(census.list_count, 1);
    assert_eq!(census.str_count, 1);
    assert_eq!(census.int_count, 1);
    assert_eq!(census.ref_count, 1);
    assert_eq!(census.bool_count, 1);
    assert_eq!(census.null_count, 0);
    assert_eq!(census.node_count, 1);
    assert_eq!(census.max_depth, 3, "root: Map(1) -> List(2) -> Str(3)");
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated();
    assert_eq!(compute_semio_value_census(&snapshot), compute_semio_value_census(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_value_census(&SemioValueSnapshot::default()), SemioValueCensus::default());
}
