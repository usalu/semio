mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfInfo;

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_reports_only_soft_findings() {
        let snapshot = PdfSnapshot::default();
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error && d.severity != Severity::Fatal), "PDF/H must never emit a hard diagnostic: got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_INFO_TITLE_OR_AUTHOR));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SIGNATURE_FIELD));
    }

    #[semio_framework_async_macros::async_test]
    async fn title_and_author_present_clears_that_finding() {
        let snapshot = PdfSnapshot { info: PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_INFO_TITLE_OR_AUTHOR), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn javascript_action_is_soft_never_hard() {
        let objects = vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }]) }];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn signature_field_present_clears_that_finding() {
        let objects = vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "AcroForm".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) }]),
            },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Fields".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 3, gen: 0 })]) }]) },
            PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "FT".into(), value: PdfObject::Name("Sig".into()) }]) },
        ];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_SIGNATURE_FIELD), "got {diagnostics:?}");
    }
}
