//! 📚️ Negative example — well-formed XML without `<!DOCTYPE>` (fails ✳️valid hard gate).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "no-doctype";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("No doctype (invalid for valid)", "Kein Doctype (ungültig für valid)")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/💥️broken.xml");
pub const EXPECTED_HARD_CODES: &[&str] = &["stdio.xml.valid.doctype-missing"];

pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::XmlSnapshot;
    use crate::artifacts::xml::standards::v1_0::subsets::valid::schema::check_valid_conformance;
    use dsl::Severity;

    #[test]
    async fn negative_asset_is_well_formed_but_not_valid() {
        assert!(!PRIMARY_TEXT.is_empty());
        let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("well-formed");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing" && matches!(d.severity, Severity::Error)), "got {diagnostics:?}");
    }
}
