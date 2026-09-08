
use super::*;

#[semio_framework_async_macros::async_test]
async fn documents_unsupported_direction_as_a_real_error_not_fabricated_bytes() {
    let err = semio_framework_plugin::resolve_ready(SemioCadToDwg::serialize(&SemioCadSnapshot::default())).unwrap_err();
    match err {
        store::PackError::Schema(msg) => assert!(msg.contains("unsupported")),
        other => panic!("expected PackError::Schema, got {other:?}"),
    }
}
