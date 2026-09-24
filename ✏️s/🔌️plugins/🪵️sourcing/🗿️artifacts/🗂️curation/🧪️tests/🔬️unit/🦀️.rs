
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


/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io::io_mechanism::{Deserializer, Serializer};
    use semio_framework::io_schema::IoPayload;
    let snapshot = crate::CurationSnapshot::default();
    let exported = export::txt::v_utf_8::any::CurationIntoTxt::serialize(&snapshot).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = import::txt::v_utf_8::any::TxtIntoCuration::deserialize(&IoPayload::Text(text)).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}
