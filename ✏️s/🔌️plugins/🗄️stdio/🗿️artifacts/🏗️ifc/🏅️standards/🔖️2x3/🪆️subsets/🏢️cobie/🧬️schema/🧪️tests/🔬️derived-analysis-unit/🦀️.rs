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
        let space = Part21Instance { id: 1, entities: vec![("IFCSPACE".into(), vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("Room 101".into())])] };
        let building = Part21Instance { id: 2, entities: vec![("IFCBUILDING".into(), vec![])] };
        let storey = Part21Instance { id: 3, entities: vec![("IFCBUILDINGSTOREY".into(), vec![])] };
        let door_type = Part21Instance { id: 4, entities: vec![("IFCDOORTYPE".into(), vec![])] };
        let rel = Part21Instance { id: 5, entities: vec![("IFCRELDEFINESBYTYPE".into(), vec![])] };
        Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header: header("FMHandOverView"), instances: vec![space, building, storey, door_type, rel] }, edm_preamble: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_has_no_hard_diagnostics() {
        let diagnostics = check_cobie_conformance(&conforming_snapshot());
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn wrong_view_definition_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header = header("CoordinationView");
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn unnamed_space_is_soft() {
        let mut snap = conforming_snapshot();
        for (name, args) in snap.document.instances[0].entities.iter_mut() {
            if name == "IFCSPACE" {
                args[2] = Part21Value::Str("   ".into());
            }
        }
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SPACE_NAME && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_storey_is_soft() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCBUILDINGSTOREY"));
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_BUILDING_STOREY && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_type_assignment_is_soft() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCRELDEFINESBYTYPE"));
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TYPE_ASSIGNMENT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
