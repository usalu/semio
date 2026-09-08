mod tests {
    use super::*;
    use crate::standards::v1::subsets::text::schema::snapshot::SemioTextMarkKind;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_a_populated_snapshot() {
        let snapshot = SemioTextBuilderConstruction::new().add_run("en", "hello", vec![]).add_run("en", "world", vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }]).build().expect("build");
        assert_eq!(snapshot.runs.len(), 2);
        assert_eq!(snapshot.runs[1].marks.len(), 1);
    }
}
