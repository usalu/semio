//! 🧩️ Shared native concept provenance, selected targets and diagnostic wire fields.

#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcConceptTarget {
    pub representation: String,
    pub applicable: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcConceptSource {
    pub commit: &'static str,
    pub url: &'static str,
    pub sha256: &'static str,
}

#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcConceptLabel {
    pub en: &'static str,
    pub de: &'static str,
}

#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcConceptDiagnostic {
    pub code: &'static str,
    pub path: Vec<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
    pub label: IfcConceptLabel,
}

#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcConceptTargetReport {
    pub representation: String,
    pub applicable: Option<bool>,
    pub status: &'static str,
    pub diagnostics: Vec<IfcConceptDiagnostic>,
}
