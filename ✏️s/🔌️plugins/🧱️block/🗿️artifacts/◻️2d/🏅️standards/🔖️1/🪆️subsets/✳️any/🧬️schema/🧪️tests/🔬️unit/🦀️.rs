
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_definition_matches_default() {
    assert_eq!(empty_block2d_snapshot(), Block2dSnapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn next_id_skips_existing() {
    let existing = ["h0", "h1"];
    assert_eq!(next_id(existing.into_iter(), "h"), "h2");
    assert_eq!(next_id(std::iter::empty(), "h"), "h0");
}
