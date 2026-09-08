mod tests {
    use super::*;
    use crate::standards::v1_0::subsets::base::schema::snapshot::{XmlDeclaration, XmlDocument};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot_with(doctype: Option<&str>, standalone: Option<bool>, root_name: &str) -> XmlSnapshot {
        XmlSnapshot {
            doc: XmlDocument {
                declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: None, standalone }),
                doctype: doctype.map(Into::into),
                prolog: Vec::new(),
                root: Some(XmlNode::Element { name: root_name.into(), attrs: Vec::new(), children: Vec::new() }),
            },
            ..XmlSnapshot::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_doctype_reports_only_the_always_on_advisory() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html>"), None, "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
        assert_eq!(diagnostics[0].code.0, CODE_VALIDITY_NOT_VERIFIED);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_doctype_is_hard() {
        let snapshot = snapshot_with(None, None, "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DOCTYPE_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn root_name_mismatch_is_hard() {
        let snapshot = snapshot_with(Some("<!DOCTYPE book>"), None, "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ROOT_NAME_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn standalone_yes_with_external_subset_is_soft() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html SYSTEM \"http://example.com/html.dtd\">"), Some(true), "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STANDALONE_EXTERNAL_SUBSET && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn standalone_yes_without_external_subset_is_clean() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html>"), Some(true), "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_STANDALONE_EXTERNAL_SUBSET), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn public_external_subset_reference_is_detected() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">"), Some(true), "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STANDALONE_EXTERNAL_SUBSET), "got {diagnostics:?}");
    }
}
