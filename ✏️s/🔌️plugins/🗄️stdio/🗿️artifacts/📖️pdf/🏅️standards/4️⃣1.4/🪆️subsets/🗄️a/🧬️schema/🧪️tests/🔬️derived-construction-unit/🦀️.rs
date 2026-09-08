mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn pass_through_build_never_fails_on_conformance_grounds() {
        let snapshot = PdfABuilderConstruction::empty().build().expect("no hard check exists at this schema; build must succeed");
        assert_eq!(snapshot.pages.len(), 1, "an empty PDF 1.4 document is one blank page, never a document with no page tree");
        assert_eq!(snapshot.first_page().expect("page 1").width, 612.0);
    }
}
