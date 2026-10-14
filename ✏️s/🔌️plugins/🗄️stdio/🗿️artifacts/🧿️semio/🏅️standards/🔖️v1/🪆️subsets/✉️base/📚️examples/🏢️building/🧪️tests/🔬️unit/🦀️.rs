
use semio_s_artifact_stdio_semio_base_examples::examples::building::*;
#[semio_framework_async_macros::async_test]
async fn building_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}
