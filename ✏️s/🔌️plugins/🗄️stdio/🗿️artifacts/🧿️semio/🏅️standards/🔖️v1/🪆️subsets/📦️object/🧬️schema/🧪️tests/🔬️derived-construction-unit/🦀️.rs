mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_a_populated_snapshot() {
        let snapshot = SemioObjectBuilderConstruction::new()
            .with_transform(SemioTransform { translation: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, ..SemioTransform::identity() })
            .with_brep("b1", store::os_io::ArtifactRef { artifact_id: "brep-a".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() } })
            .build()
            .expect("build");
        assert_eq!(snapshot.transform.translation.x, 1.0);
        assert_eq!(snapshot.brep.unwrap().child_id, "b1");
    }
}
