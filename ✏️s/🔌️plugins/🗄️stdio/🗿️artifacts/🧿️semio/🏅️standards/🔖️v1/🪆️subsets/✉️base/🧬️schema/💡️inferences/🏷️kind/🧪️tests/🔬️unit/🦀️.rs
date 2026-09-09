use super::*;
use crate::standards::v1::subsets::base::schema::snapshot::{SemioSubsetSnapshot, STDIO_SEMIO_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(subset: SemioSubsetSnapshot) -> SemioSnapshot {
    SemioSnapshot { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset }
}

#[semio_framework_async_macros::async_test]
async fn image_dispatches_to_its_own_tag_and_ordinal() {
    let kind = compute_semio_kind(&snapshot(SemioSubsetSnapshot::Image(SemioImageSnapshot::default())));
    assert_eq!(kind, SemioKind { tag: "image".into(), ordinal: 7 });
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snap = snapshot(SemioSubsetSnapshot::Image(SemioImageSnapshot::default()));
    assert_eq!(compute_semio_kind(&snap), compute_semio_kind(&snap));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_kind(&SemioSnapshot::default()), SemioKind::default());
}
