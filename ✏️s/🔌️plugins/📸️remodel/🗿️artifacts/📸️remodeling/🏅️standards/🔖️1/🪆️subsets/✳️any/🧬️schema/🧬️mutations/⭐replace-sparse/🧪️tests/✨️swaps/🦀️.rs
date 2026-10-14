//! 🧪️ `replace-sparse` fixture — `✨️swaps`.
//!
//! Source of truth is the committed JSON quintet of the fixture mirror, read at test runtime (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `replace-sparse-realworld` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ a re-triangulation replaces the whole buffer; there is no partial-point mutation in this vocabulary

/// ⚖️ The vector holds the law set its declared outcome selects, its inverse restores the before-document and its committed JSON is canonical.
#[test]
fn holds_its_laws() {
    crate::vectors::assert_case("⭐replace-sparse", "✨️swaps");
}

/// ➕️ The inverse rows of this applied vector sum to the negative of its diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::vectors::assert_inverse_sum("⭐replace-sparse", "✨️swaps").await;
}
