mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_x_objects() -> Vec<PdfIndirectObject> {
        vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                    PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) },
                    PdfDictEntry { key: "DPartRoot".into(), value: PdfObject::Ref(ObjRef { num: 10, gen: 0 }) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 2, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                    PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFX".into()) },
                    PdfDictEntry { key: "DestOutputProfile".into(), value: PdfObject::Ref(ObjRef { num: 9, gen: 0 }) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Page".into()) },
                    PdfDictEntry { key: "TrimBox".into(), value: PdfObject::Array(vec![PdfObject::Int(0), PdfObject::Int(0), PdfObject::Int(100), PdfObject::Int(100)]) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 10, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("DPartRoot".into()) }, PdfDictEntry { key: "DParts".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 11, gen: 0 })]) }]),
            },
            PdfIndirectObject { id: ObjRef { num: 11, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("DPart".into()) }, PdfDictEntry { key: "DPM".into(), value: PdfObject::Dict(vec![]) }]) },
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn fully_conforming_vt_document_has_no_hard_diagnostics() {
        let snapshot = PdfSnapshot { objects: conforming_x_objects(), ..PdfSnapshot::default() };
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_dpartroot_is_hard() {
        let mut objects = conforming_x_objects();
        if let Some(catalog_obj) = objects.iter_mut().find(|o| o.id.num == 1) {
            if let PdfObject::Dict(d) = &mut catalog_obj.value {
                d.retain(|e| e.key != "DPartRoot");
            }
        }
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DPART_ROOT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn x_violations_are_inherited_as_hard() {
        // No OutputIntent at all -- an X-4 violation must surface through vt too.
        let snapshot = PdfSnapshot::default();
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::x::schema::CODE_OUTPUT_INTENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn dpart_missing_dpm_is_soft() {
        let mut objects = conforming_x_objects();
        if let Some(dpart) = objects.iter_mut().find(|o| o.id.num == 11) {
            if let PdfObject::Dict(d) = &mut dpart.value {
                d.retain(|e| e.key != "DPM");
            }
        }
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DPM && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
