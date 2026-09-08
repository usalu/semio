
use super::*;
use crate::CuratedItem;
use protocol::Inference;

fn picked_snapshot() -> CurationSnapshot {
    CurationSnapshot { curated: vec![CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 4 }, CuratedItem { object_id: "window-fixed-150x150".into(), count: 6 }], ..CurationSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = picked_snapshot();
    assert_eq!(CurationInference::infer(&snapshot), CurationInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(CurationInference::infer(&CurationSnapshot::default()), CurationInference::default());
}

#[semio_framework_async_macros::async_test]
async fn entries_counts_curated_lines_and_total_quantity() {
    let inferred = CurationInference::infer(&picked_snapshot());
    assert_eq!(inferred.entries.entry_count, 2);
    assert_eq!(inferred.entries.total_count, 10);
}

//#region 🧪️PuzzleCatalogFragment
fn sample_document() -> CurationSnapshot {
    crate::curation_snapshot_from_stock(&crate::schema::demo_stock(), Vec::new())
}

#[semio_framework_async_macros::async_test]
async fn sourcing_catalog_fragment_maps_stock_into_the_puzzle3d_kit_catalog_shape() {
    let document = sample_document();
    let stock = crate::stock_of(&document);
    let fragment = sourcing_catalog_fragment(&document);
    assert_eq!(fragment.get("schema").and_then(|value| value.as_str()), Some("manifest"));
    let object_kinds = fragment.get("objectKinds").and_then(|value| value.as_array()).expect("objectKinds array");
    assert_eq!(object_kinds.len(), stock.len());
    assert_eq!(object_kinds[0].get("id").and_then(|value| value.as_str()), Some(stock[0].id.as_str()));
    assert_eq!(object_kinds[0].get("meshUrl"), Some(&dsl::DslValue::Null));
    assert!(object_kinds[0].get("vortices").and_then(|value| value.as_array()).unwrap().is_empty());
    assert!(fragment.get("vortexKinds").and_then(|value| value.as_array()).unwrap().is_empty());
    assert!(fragment.get("cableKinds").and_then(|value| value.as_array()).unwrap().is_empty());
    assert!(fragment.get("attractionKinds").and_then(|value| value.as_array()).unwrap().is_empty());
    assert!(fragment.get("kindCompatibility").and_then(|value| value.as_array()).unwrap().is_empty());
}
//#endregion 🧪️PuzzleCatalogFragment
