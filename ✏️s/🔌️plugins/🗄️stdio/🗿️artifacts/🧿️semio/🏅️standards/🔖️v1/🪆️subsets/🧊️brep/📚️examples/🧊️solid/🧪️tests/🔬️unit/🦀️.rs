
use super::*;
#[semio_framework_async_macros::async_test]
async fn solid_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}
