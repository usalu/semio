
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_energy_model_snapshot();
    assert_eq!(snapshot.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
}

/// 🌱 Relocated from `⚙️engine/🦀️.rs` — DSL-parse sanity check with zero
/// `EnergyModelEngine` dependency, so it survives that struct's deletion. Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: `model_json` is gone — asserts the composed
/// `structure`/`zones` child handles are real (non-empty ids) instead.
#[semio_framework_async_macros::async_test]
async fn example_fixture_parses() {
    let document = crate::document_dsl::parse_dsl(crate::document_dsl::SEMIO_ENERGY_MODEL_EXAMPLE_TEXT).expect("parse");
    assert_eq!(document.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
    assert!(!document.structure.child_id.is_empty());
    assert!(!document.zones.child_id.is_empty());
}
