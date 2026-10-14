
use semio_s_artifact_stdio_semio_base_examples::examples::swatch::*;
#[semio_framework_async_macros::async_test]
async fn swatch_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}
