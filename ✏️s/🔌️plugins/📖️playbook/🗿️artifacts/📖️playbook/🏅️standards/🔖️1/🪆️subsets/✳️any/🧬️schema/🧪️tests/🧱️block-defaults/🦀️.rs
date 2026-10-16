use crate::standards::v1::subsets::any::schema::*;

#[semio_framework_async_macros::async_test]
async fn default_block_sets_kind_and_label() {
    assert_eq!(default_block("b1".into(), "text").kind, "text");
}
