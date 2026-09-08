mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_only_reports_soft_findings() {
        let snapshot = PdfSnapshot::default();
        let diagnostics = check_e_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_OUTPUT_INTENT));
    }

    #[semio_framework_async_macros::async_test]
    async fn encryption_dict_shape_is_hard() {
        let objects = vec![PdfIndirectObject {
            id: ObjRef { num: 1, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
            ]),
        }];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_e_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn javascript_action_is_hard() {
        let objects = vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }]) }];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_e_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn launch_action_is_hard() {
        let objects = vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("Launch".into()) }]) }];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_e_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LAUNCH && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn movie_annotation_is_hard_but_3d_is_never_flagged() {
        let objects = vec![
            PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Movie".into()) }]) },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("3D".into()) }]) },
        ];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_e_conformance(&snapshot);
        let movie_hits: Vec<_> = diagnostics.iter().filter(|d| d.code.0 == CODE_MOVIE_OR_SOUND).collect();
        assert_eq!(movie_hits.len(), 1, "only the Movie object must be flagged, never the 3D one: got {diagnostics:?}");
        assert_eq!(movie_hits[0].severity, Severity::Error);
    }

    #[semio_framework_async_macros::async_test]
    async fn sound_annotation_is_hard() {
        let objects = vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Sound".into()) }]) }];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_e_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MOVIE_OR_SOUND && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn output_intent_present_clears_the_soft_finding() {
        let objects = vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) }]),
            },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) }]) },
        ];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_e_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_OUTPUT_INTENT), "got {diagnostics:?}");
    }
}
