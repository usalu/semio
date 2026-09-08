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
        let placement = Part21Instance { id: 10, entities: vec![("IFCLOCALPLACEMENT".into(), vec![])] };
        let project = Part21Instance {
            id: 1,
            entities: vec![(
                "IFCPROJECT".into(),
                vec![
                    Part21Value::Str("guid".into()),
                    Part21Value::Unset,
                    Part21Value::Str("Project".into()),
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Ref(20), // UnitsInContext
                ],
            )],
        };
        let wall = Part21Instance {
            id: 2,
            entities: vec![(
                "IFCWALL".into(),
                vec![
                    Part21Value::Str("guid2".into()),
                    Part21Value::Unset,
                    Part21Value::Str("Wall".into()),
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Ref(10), // ObjectPlacement
                ],
            )],
        };
        let units = Part21Instance { id: 20, entities: vec![("IFCUNITASSIGNMENT".into(), vec![])] };
        Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header: header("CoordinationView"), instances: vec![placement, project, wall, units] }, edm_preamble: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_has_no_hard_diagnostics() {
        let diagnostics = check_cv20_conformance(&conforming_snapshot());
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn wrong_file_schema_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("IFC4".into())])];
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILE_SCHEMA && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_view_definition_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header = header("StructuralAnalysisView");
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn structural_entity_present_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.instances.push(Part21Instance { id: 99, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] });
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRUCTURAL_ENTITY && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_unit_assignment_is_soft() {
        let mut snap = conforming_snapshot();
        for (name, args) in snap.document.instances[1].entities.iter_mut() {
            if name == "IFCPROJECT" {
                args[8] = Part21Value::Unset;
            }
        }
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PROJECT_UNITS && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn product_without_placement_is_soft() {
        let mut snap = conforming_snapshot();
        for (name, args) in snap.document.instances[2].entities.iter_mut() {
            if name == "IFCWALL" {
                args[5] = Part21Value::Unset;
            }
        }
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRODUCT_PLACEMENT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
