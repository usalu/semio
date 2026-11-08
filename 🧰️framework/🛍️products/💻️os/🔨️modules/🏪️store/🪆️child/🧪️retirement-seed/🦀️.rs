//! 🪆️ A child's retirement seed keeps its local materialization alive until a paid step releases it.
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_value::retirement::RetireOwned;
use semio_os_store_child::ArtifactChild;
use std::sync::Arc;

#[test]
fn child_retirement_seed_keeps_the_local_materialization_until_a_paid_step() {
    let owner = Arc::new(String::from("local"));
    let target = ArtifactRef { artifact_id: "a".into(), dialect: ArtifactDialect { artifact_kind: "k".into(), standard: "s".into(), subset: "x".into() } };
    let cursor = ArtifactChild::<()>::new("c".into(), target).with_local_owner(owner.clone()).retirement_seed();
    assert_eq!(Arc::strong_count(&owner), 2);
    std::mem::forget(cursor);
}
