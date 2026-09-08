
use super::*;

#[semio_framework_async_macros::async_test]
async fn same_snapshot_yields_same_digest() {
    let snapshot = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    assert_eq!(compute_content_digest(&snapshot), compute_content_digest(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn changing_generation_changes_digest() {
    let a = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let mut b = a.clone();
    b.catalog_generation = 4;
    assert_ne!(compute_content_digest(&a), compute_content_digest(&b));
}
