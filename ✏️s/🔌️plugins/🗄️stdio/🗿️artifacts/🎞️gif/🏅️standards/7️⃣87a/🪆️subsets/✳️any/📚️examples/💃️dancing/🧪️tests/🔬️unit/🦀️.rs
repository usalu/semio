
use super::*;

#[semio_framework_async_macros::async_test]
async fn dancing_source_nonempty_and_decodes() {
    let src = source();
    assert!(!src.document_json().is_empty());
    let _ = decoded_snapshot();
}
