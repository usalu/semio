
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_mode_is_the_apps_default() {
    assert_eq!(definition().id, SPACE_INDEX_MODE_EDIT);
}
