
use semio_s_artifact_stdio_semio_base_examples::examples::tone::*;
#[semio_framework_async_macros::async_test]
async fn tone_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}
