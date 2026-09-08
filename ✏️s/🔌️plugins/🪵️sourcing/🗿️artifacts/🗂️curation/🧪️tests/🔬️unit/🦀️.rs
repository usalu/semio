
use super::*;

/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("sourcing.curation") is deliberately NOT
/// `SOURCING_CURATION_SCHEMA` ("sourcing.curation/v1") — the former names the artifact kind in the OS
/// media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
/// merge them (mirrors `flow`'s identical `artifact_kind` split-schema pin).
#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
    assert_eq!(artifact_kind().schema, "sourcing.curation");
    assert_eq!(SOURCING_CURATION_SCHEMA, "sourcing.curation/v1");
}
