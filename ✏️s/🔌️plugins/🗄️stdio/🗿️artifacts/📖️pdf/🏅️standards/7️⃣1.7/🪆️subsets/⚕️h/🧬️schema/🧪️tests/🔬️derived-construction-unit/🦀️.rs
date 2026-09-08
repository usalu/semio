mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn build_always_succeeds() {
        let snapshot = PdfHBuilderConstruction::new().add_page(PdfPage::new(200.0, 200.0)).build().expect("PDF/H build() never fails");
        assert_eq!(snapshot.pages.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_info_clears_title_author_advisory() {
        let snapshot = PdfHBuilderConstruction::new().set_info(PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..PdfInfo::default() }).build().unwrap();
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != crate::standards::v1_7::subsets::h::schema::CODE_INFO_TITLE_OR_AUTHOR), "got {diagnostics:?}");
    }
}
