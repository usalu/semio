mod tests {
        use super::*;
        use crate::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn output_intent_objects(condition: &str) -> Vec<PdfIndirectObject> {
            vec![
                PdfIndirectObject {
                    id: ObjRef { num: 1, gen: 0 },
                    value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) }]),
                },
                PdfIndirectObject {
                    id: ObjRef { num: 2, gen: 0 },
                    value: PdfObject::Dict(vec![
                        PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                        PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFA1".into()) },
                        PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(condition.as_bytes().to_vec()) },
                    ]),
                },
            ]
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_snapshot_with_output_intent_reports_only_level_info() {
            let snapshot = PdfSnapshot { objects: output_intent_objects("sRGB IEC61966-2.1"), ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert_eq!(diagnostics.len(), 1, "expected exactly the level-detection Info diagnostic, got {diagnostics:?}");
            assert_eq!(diagnostics[0].code.0, CODE_LEVEL);
            assert_eq!(diagnostics[0].severity, Severity::Info);
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_output_intent_is_soft_and_reports_no_level() {
            let snapshot = PdfSnapshot::default();
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(diagnostics[0].code.0, CODE_OUTPUT_INTENT);
            assert_eq!(diagnostics[0].severity, Severity::Warning);
            assert!(detect_pdfa_level(&snapshot).is_none());
        }

        #[semio_framework_async_macros::async_test]
        async fn encryption_dict_shape_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                    PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                    PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                    PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                    PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn javascript_action_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }, PdfDictEntry { key: "JS".into(), value: PdfObject::Str(b"app.alert(1)".to_vec()) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert_eq!(diagnostics.iter().filter(|d| d.code.0 == CODE_JAVASCRIPT).count(), 1, "must not double-report S=JavaScript + JS key on the same object: got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Error));
        }

        #[semio_framework_async_macros::async_test]
        async fn launch_action_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("Launch".into()) }, PdfDictEntry { key: "F".into(), value: PdfObject::Str(b"calc.exe".to_vec()) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LAUNCH && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn non_embedded_font_is_soft() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) },
                    PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Type1".into()) },
                    PdfDictEntry { key: "BaseFont".into(), value: PdfObject::Name("Helvetica".into()) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FONT_NOT_EMBEDDED && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn embedded_font_via_descriptor_has_no_diagnostic() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) },
                    PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("TrueType".into()) },
                    PdfDictEntry { key: "FontDescriptor".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                ]),
            });
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 4, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("FontDescriptor".into()) }, PdfDictEntry { key: "FontFile2".into(), value: PdfObject::Ref(ObjRef { num: 5, gen: 0 }) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_FONT_NOT_EMBEDDED), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn embedded_file_missing_afrelationship_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Filespec".into()) }, PdfDictEntry { key: "EF".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_EMBEDDED_FILE_AFRELATIONSHIP && d.severity == Severity::Error), "got {diagnostics:?}");
            // No genuine part-3 signal (missing /AFRelationship is exactly the violation), so the
            // level detector must not credit this document with Part 3.
            assert_eq!(detect_pdfa_level(&snapshot), Some(PdfALevel::L2b));
        }

        #[semio_framework_async_macros::async_test]
        async fn embedded_file_with_afrelationship_detects_level_3b_and_is_clean() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Filespec".into()) },
                    PdfDictEntry { key: "EF".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                    PdfDictEntry { key: "AFRelationship".into(), value: PdfObject::Name("Data".into()) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            assert_eq!(detect_pdfa_level(&snapshot), Some(PdfALevel::L3b));
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_EMBEDDED_FILE_AFRELATIONSHIP), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LEVEL && d.message.contains("3b")), "got {diagnostics:?}");
        }
    }
