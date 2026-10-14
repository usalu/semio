//! 🧪️ `add-stream-frame` fixture — `🎞️appends2`.
//!
//! Source of truth is the committed JSON quintet of the fixture mirror, read at test runtime (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `add-stream-frame-realworld` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ index 10 sorts after every frame already present, so the canonical insert lands at the end here

/// ⚖️ The vector holds the law set its declared outcome selects, its inverse restores the before-document and its committed JSON is canonical.
#[test]
fn holds_its_laws() {
    crate::vectors::assert_case("➕add-stream-frame", "🎞️appends2");
}
