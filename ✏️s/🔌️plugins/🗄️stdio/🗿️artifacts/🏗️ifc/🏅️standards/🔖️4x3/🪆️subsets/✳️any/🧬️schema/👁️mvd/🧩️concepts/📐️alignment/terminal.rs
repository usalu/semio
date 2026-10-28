//! 🚆️ Original retained relationships preserve ordered business terminality without geometric inference.
use super::alignment_roles::*;
use super::native::*;
use super::concept_types::{IfcConceptDiagnostic, IfcConceptLabel, IfcConceptSource};
use crate::catalog::query::{Ifc4x3AttributeMetadata, IfcAttributeQueryStatus, IfcAttributeQueryWork};
use crate::validation::metadata;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader, IfcSnapshot, IfcValue};
use std::{mem::size_of, sync::Arc};
use super::source_index::{IdMatch, IfcSourceIds};
impl super::preparation::IfcMvdRequest for IfcAlignmentTerminalRequest { fn valid_row(&self, ordinal: usize) -> bool { kind(&self.targets[ordinal].kind).is_some() } }
#[path = "terminal-native-data.rs"]
mod data;
use data::*;
#[cfg(test)]
#[path = "terminal-tests.rs"]
mod tests;
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcAlignmentTerminalTarget {
    pub layout: String,
    pub kind: String,
    pub applicable: Option<bool>,
}
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct IfcAlignmentTerminalRequest {
    pub targets: Vec<IfcAlignmentTerminalTarget>,
}
impl IfcSourceIds for IfcAlignmentTerminalRequest {
    fn source_len(&self) -> usize {
        self.targets.len()
    }
    fn source_id(&self, row: usize) -> Result<u64, ValueError> {
        positive_id(&self.targets[row].layout)
    }
}
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcAlignmentTerminalTargetReport {
    pub layout: String,
    pub kind: String,
    pub applicable: Option<bool>,
    pub status: &'static str,
    pub diagnostics: Vec<IfcConceptDiagnostic>,
}
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct IfcAlignmentTerminalReport {
    pub concept: &'static str,
    pub predicate: &'static str,
    pub source: IfcConceptSource,
    pub schema_conformance: &'static str,
    pub view_conformance: &'static str,
    pub geometry_conformance: &'static str,
    pub status: &'static str,
    pub targets: Vec<IfcAlignmentTerminalTargetReport>,
    pub diagnostics: Vec<IfcConceptDiagnostic>,
}
#[path = "terminal-clause.rs"]
mod clause;
#[path = "terminal-work.rs"]
mod work;
pub use work::IfcAlignmentTerminalWork;
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
