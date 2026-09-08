
use super::*;
use crate::{CuratedItem, GeometryRecipe, ObjectKind};

fn object_kind(id: &str) -> ObjectKind {
    ObjectKind { id: id.into(), name: id.into(), module_id: "beams".into(), typology_path: vec!["beams".into()], availability: 1, geometry: Box::new(GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }) }
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_yields_a_zero_census() {
    let entries = compute_curation_entries(&CurationSnapshot::default());
    assert_eq!(entries, CurationEntries::default());
}

#[semio_framework_async_macros::async_test]
async fn stock_and_curated_lines_are_counted_exactly() {
    let snapshot = crate::curation_snapshot_from_stock(&[object_kind("a"), object_kind("b"), object_kind("c")], vec![CuratedItem { object_id: "a".into(), count: 5 }, CuratedItem { object_id: "b".into(), count: 3 }]);
    let entries = compute_curation_entries(&snapshot);
    assert_eq!(entries.stock_count, 3);
    assert_eq!(entries.entry_count, 2);
    assert_eq!(entries.total_count, 8);
}

#[semio_framework_async_macros::async_test]
async fn entries_is_deterministic() {
    let snapshot = crate::curation_snapshot_from_stock(&[], vec![CuratedItem { object_id: "a".into(), count: 1 }]);
    assert_eq!(compute_curation_entries(&snapshot), compute_curation_entries(&snapshot));
}
