
use super::*;

#[semio_framework_async_macros::async_test]
async fn bind_resolve_and_unbind_round_trip() {
    let directory = InstanceDirectory::new();
    directory.bind("artifacts/node-a", "s.cad", 7, "s.cad.document").await.unwrap();
    let location = directory.resolve("artifacts/node-a").await.expect("bound artifact must resolve");
    assert_eq!(location, InstanceLocation { plugin_id: "s.cad".into(), instance_id: 7, artifact_kind: "s.cad.document".into() });
    directory.unbind_instance("s.cad", 7).await;
    assert!(directory.resolve("artifacts/node-a").await.is_none());
}

#[semio_framework_async_macros::async_test]
async fn rebinding_the_same_artifact_id_replaces_the_prior_location() {
    let directory = InstanceDirectory::new();
    directory.bind("artifacts/node-a", "s.cad", 1, "s.cad.document").await.unwrap();
    directory.bind("artifacts/node-a", "s.cad", 2, "s.cad.document").await.unwrap();
    assert_eq!(directory.resolve("artifacts/node-a").await.unwrap().instance_id, 2);
    assert!(directory.artifact_ids_for_instance("s.cad", 1).await.is_empty(), "the stale instance no longer owns this artifact id");
}
