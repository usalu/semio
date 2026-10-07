//! 🧒️ Real store child admission retains the original schema composition law.

use semio_framework_schema_composition::ArtifactCompositionFields;

#[semio_framework_async_macros::async_test]
async fn artifact_composition_projection_real_child_alias_has_fixed_admission_bounds() {
    use crate::os_store::{ArtifactChild, ChildRestoreProjection, ChildRestoreProjectionError};
    type ChildAlias = ArtifactChild<()>;
    #[derive(semio_framework_schema::ArtifactSchema)]
    #[artifact_schema(id = "s.test.parent")]
    struct DerivedParent {
        #[state(artifact)]
        #[child(kind = "s.test.member")]
        many: Vec<Option<ChildAlias>>,
    }
    let child = |id: String| Some(ArtifactChild::new(id.clone(), semio_framework_artifact_reference::ArtifactRef { artifact_id: id, dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "v1".into(), subset: "first".into() } }));
    let mut parent = DerivedParent { many: (0..64).map(|index| child(index.to_string())).collect() };
    assert_eq!(ChildRestoreProjection::from_snapshot(&parent).unwrap().len(), 64);
    parent.many.push(child("overflow".into()));
    assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::ReferenceLimit)));
    parent.many = (0..257).map(|_| None).collect();
    assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::TraversalLimit)));
    parent.many = vec![child("ä".repeat(128))];
    assert!(ChildRestoreProjection::from_snapshot(&parent).is_ok());
    parent.many = vec![child(format!("{}x", "ä".repeat(128)))];
    assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::InvalidReference)));
}
