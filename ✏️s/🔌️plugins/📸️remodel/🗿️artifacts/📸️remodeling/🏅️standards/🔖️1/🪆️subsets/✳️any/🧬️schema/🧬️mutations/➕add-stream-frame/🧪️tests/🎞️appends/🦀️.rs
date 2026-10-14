//! 🧪️ `add-stream-frame` fixture — `🎞️appends`.
//!
//! Source of truth is the committed JSON quintet of the fixture mirror, read at test runtime (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `add-stream-frame` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ the pre-existing two-stream unit vector, regenerated onto the corrected toy base

/// ⚖️ The vector holds the law set its declared outcome selects, its inverse restores the before-document and its committed JSON is canonical.
#[test]
fn holds_its_laws() {
    crate::vectors::assert_case("➕add-stream-frame", "🎞️appends");
}

/// ➕️ The inverse rows of this applied vector sum to the negative of its diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::vectors::assert_inverse_sum("➕add-stream-frame", "🎞️appends").await;
}
