mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_clean() {
        let snapshot = ZipIso21320BuilderConstruction::new().with_stored_entry("a.txt", b"hello".to_vec()).with_deflate_entry("b.txt", b"world, compressed".to_vec()).with_comment("archive").build().expect("conforming construction must build");
        assert_eq!(snapshot.entries.len(), 2);
        assert_eq!(snapshot.comment, "archive");
    }
}
