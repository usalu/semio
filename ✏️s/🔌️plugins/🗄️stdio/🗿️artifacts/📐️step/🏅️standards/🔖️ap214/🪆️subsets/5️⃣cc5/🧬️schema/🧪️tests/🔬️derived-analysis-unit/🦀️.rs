mod tests {
    use super::*;
    use crate::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_doc() -> Part21Document {
        Part21Document {
            header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
            instances: vec![
                Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
            ],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_document_reports_no_diagnostics() {
        let snapshot = StepSnapshot::from_part21_document(&base_doc());
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_file_schema_is_hard() {
        let mut doc = base_doc();
        doc.header.file_schema = vec![];
        let snapshot = StepSnapshot::from_part21_document(&doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILE_SCHEMA && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_product_chain_is_soft() {
        let mut doc = base_doc();
        doc.instances.clear();
        let snapshot = StepSnapshot::from_part21_document(&doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRODUCT_CHAIN && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn representation_at_max_rung_is_clean() {
        let mut doc = base_doc();
        doc.instances.push(Part21Instance { id: 4, entities: vec![("FACETED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
        let snapshot = StepSnapshot::from_part21_document(&doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_LADDER), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn representation_above_max_rung_is_hard() {
        let mut doc = base_doc();
        doc.instances.push(Part21Instance { id: 4, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
        let snapshot = StepSnapshot::from_part21_document(&doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LADDER && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
