mod tests {
    use super::*;
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;

    #[semio_framework_async_macros::async_test]
    async fn schema_gap_diagnostic_always_fires() {
        let snapshot = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: "hello".into() }, PageDoc { width: 612.0, height: 792.0, text: "a later page this check never reads".into() }], ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_text_is_flagged_soft() {
        let snapshot = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: String::new() }, PageDoc { width: 612.0, height: 792.0, text: "a later page this check never reads".into() }], ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TEXT_EMPTY && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert_eq!(diagnostics.len(), 2, "expected text-empty + schema-gap, got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn non_empty_text_skips_the_text_check() {
        let snapshot = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: "content".into() }, PageDoc { width: 612.0, height: 792.0, text: "a later page this check never reads".into() }], ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_TEXT_EMPTY), "got {diagnostics:?}");
        assert_eq!(diagnostics.len(), 1, "expected only schema-gap, got {diagnostics:?}");
    }
}
