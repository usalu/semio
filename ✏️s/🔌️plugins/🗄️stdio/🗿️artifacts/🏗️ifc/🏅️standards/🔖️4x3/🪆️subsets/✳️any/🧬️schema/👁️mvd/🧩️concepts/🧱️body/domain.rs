//! 🧾️ Canonical native selected Body inputs and outputs share exact representation wire fields.
use super::super::concept_types::*;
use super::super::source_index::IfcSourceIds;
use super::super::preparation::IfcMvdRequest;
use super::super::native::positive_id;
use semio_framework_value::ValueError;

#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct IfcBodyConceptRequest {
    pub targets: Vec<IfcConceptTarget>,
}

#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct IfcBodyConceptReport {
    pub concept: &'static str,
    pub source: IfcConceptSource,
    pub schema_conformance: &'static str,
    pub view_conformance: &'static str,
    pub status: &'static str,
    pub targets: Vec<IfcConceptTargetReport>,
    pub diagnostics: Vec<IfcConceptDiagnostic>,
}
impl IfcSourceIds for IfcBodyConceptRequest {
    fn source_len(&self) -> usize { self.targets.len() }
    fn source_id(&self, row: usize) -> Result<u64, ValueError> { positive_id(&self.targets[row].representation) }
}
impl IfcMvdRequest for IfcBodyConceptRequest {
    fn valid_row(&self, row: usize) -> bool { self.targets.get(row).is_some() }
}
