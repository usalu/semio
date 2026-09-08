mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfInfo;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tagged_catalog_objects() -> Vec<PdfIndirectObject> {
        vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                    PdfDictEntry { key: "MarkInfo".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) },
                    PdfDictEntry { key: "StructTreeRoot".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
                    PdfDictEntry { key: "Lang".into(), value: PdfObject::Str(b"en-US".to_vec()) },
                    PdfDictEntry { key: "ViewerPreferences".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                ]),
            },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Marked".into(), value: PdfObject::Bool(true) }]) },
            PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("StructTreeRoot".into()) }]) },
            PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "DisplayDocTitle".into(), value: PdfObject::Bool(true) }]) },
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn fully_tagged_conforming_document_has_no_diagnostics() {
        let snapshot = PdfSnapshot { objects: tagged_catalog_objects(), info: PdfInfo { title: Some("A Title".into()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
        let diagnostics = check_ua_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_markinfo_and_structtreeroot_are_hard() {
        let objects = vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }]) }];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_ua_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MARKINFO && d.severity == Severity::Error), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRUCT_TREE_ROOT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn marked_false_is_still_hard() {
        let objects = vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                    PdfDictEntry { key: "MarkInfo".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) },
                    PdfDictEntry { key: "StructTreeRoot".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
                ]),
            },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Marked".into(), value: PdfObject::Bool(false) }]) },
            PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![]) },
        ];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_ua_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MARKINFO && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_lang_title_and_displaydoctitle_are_soft() {
        let snapshot = PdfSnapshot { objects: tagged_catalog_objects().into_iter().filter(|o| o.id.num != 4).collect(), ..PdfSnapshot::default() };
        let mut objects = snapshot.objects.clone();
        if let Some(cat) = objects.iter_mut().find(|o| o.id.num == 1) {
            if let PdfObject::Dict(d) = &mut cat.value {
                d.retain(|e| e.key != "Lang" && e.key != "ViewerPreferences");
            }
        }
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_ua_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LANG && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DISPLAY_DOC_TITLE && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_INFO_TITLE && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
