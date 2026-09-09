use super::*;

#[semio_framework_async_macros::async_test]
async fn base_glb_decodes_to_a_nonempty_real_document() {
    let snapshot = decoded_snapshot();
    assert_eq!(snapshot.document.asset.version, "2.0");
    assert!(!snapshot.buffers.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    let source = source();
    assert_eq!(source.id(), ID);
    assert!(!source.document_json().is_empty());
}
