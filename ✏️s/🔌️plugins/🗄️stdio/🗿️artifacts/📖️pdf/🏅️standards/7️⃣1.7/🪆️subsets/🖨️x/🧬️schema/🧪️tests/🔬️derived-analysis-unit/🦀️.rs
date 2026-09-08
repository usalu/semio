mod tests {
        use super::*;
        use crate::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn conforming_objects() -> Vec<PdfIndirectObject> {
            vec![
                PdfIndirectObject {
                    id: ObjRef { num: 1, gen: 0 },
                    value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) }]),
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
            ]
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_snapshot_has_no_hard_diagnostics() {
            let snapshot = PdfSnapshot { objects: conforming_objects(), ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_output_intent_is_hard() {
            let snapshot = PdfSnapshot::default();
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_OUTPUT_INTENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn page_missing_trim_or_art_box_is_hard() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Page".into()) }]) });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TRIM_OR_ART_BOX && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn encryption_dict_shape_is_hard() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 5, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                    PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                    PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                    PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                    PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn non_embedded_font_is_soft() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 6, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) }, PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Type1".into()) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FONT_NOT_EMBEDDED && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn javascript_action_is_soft() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject { id: ObjRef { num: 7, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }]) });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn movie_annotation_is_soft() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject { id: ObjRef { num: 8, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Movie".into()) }]) });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MOVIE_OR_SOUND && d.severity == Severity::Warning), "got {diagnostics:?}");
        }
    }
