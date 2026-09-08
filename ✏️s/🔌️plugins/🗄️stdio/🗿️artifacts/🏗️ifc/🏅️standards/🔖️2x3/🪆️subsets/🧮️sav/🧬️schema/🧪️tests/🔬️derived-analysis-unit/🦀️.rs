mod tests {
    use super::*;
    use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn header(view: &str) -> Part21Header {
        Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str(format!("ViewDefinition [{view}]"))]), Part21Value::Str("2;1".into())],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_snapshot() -> Ifc2x3Snapshot {
        let model = Part21Instance { id: 1, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] };
        let group = Part21Instance { id: 2, entities: vec![("IFCRELASSIGNSTOGROUP".into(), vec![])] };
        let loads = Part21Instance { id: 3, entities: vec![("IFCSTRUCTURALLOADGROUP".into(), vec![])] };
        Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header: header("StructuralAnalysisView"), instances: vec![model, group, loads] }, edm_preamble: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_has_no_hard_diagnostics() {
        let diagnostics = check_sav_conformance(&conforming_snapshot());
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_analysis_model_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCSTRUCTURALANALYSISMODEL"));
        let diagnostics = check_sav_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_ANALYSIS_MODEL && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn wrong_view_definition_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header = header("CoordinationView");
        let diagnostics = check_sav_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_loads_and_group_assignment_are_soft() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCRELASSIGNSTOGROUP") && !i.is_type("IFCSTRUCTURALLOADGROUP"));
        let diagnostics = check_sav_conformance(&snap);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_GROUP_ASSIGNMENT));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_LOADS));
    }
}
