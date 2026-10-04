use super::*;
use crate::document_dsl as dsl;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = dsl::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("parse default fixture");
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    assert_eq!(store::os_store::test_support::assert_pack_schema_identity(&document), store::os_pack::schema_hash(&<semio_framework_artifact_infinite_dag::DagSnapshot as store::ArtifactPack>::record_spec().expect("framework dag record spec")));
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
