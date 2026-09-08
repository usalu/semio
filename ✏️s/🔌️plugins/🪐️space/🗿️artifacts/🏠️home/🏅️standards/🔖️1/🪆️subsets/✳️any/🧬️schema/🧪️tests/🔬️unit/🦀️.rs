
use super::*;

/// 🌱️ Relocated verbatim from `⚙️engine`'s own test module — the sibling
/// `engine_apply_updates_catalog_generation` test is NOT carried forward: it exercised
/// `SHomeEngine::apply` via `use dsl::ArtifactEngine;`, a trait that has zero implementations and
/// zero definitions anywhere in shipped source (`grep -rn "trait ArtifactEngine"` → 0 repo-wide),
/// so that test could never have compiled — a pre-existing dead reference to the never-shipped
/// trait this whole ticket is repealing, not a surviving assertion. `SHomeEngine` itself had zero
/// external references and is deleted outright per the ticket's D5a ruling.
#[semio_framework_async_macros::async_test]
async fn empty_snapshot_uses_home_schema() {
    let snapshot = empty_shome_snapshot();
    assert_eq!(snapshot.schema, crate::S_HOME_DOCUMENT_SCHEMA);
}
