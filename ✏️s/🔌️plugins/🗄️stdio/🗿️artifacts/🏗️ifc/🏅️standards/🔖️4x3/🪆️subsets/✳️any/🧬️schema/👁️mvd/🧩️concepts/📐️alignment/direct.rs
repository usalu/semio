//! 🚄️ Original retained source and requests evaluate only the selected direct zero-counterpart clause.
use super::alignment_roles::{PARAMETERS, kind};
use super::native::*;
use super::concept_types::{IfcConceptDiagnostic, IfcConceptLabel, IfcConceptSource};
use crate::catalog::query::{Ifc4x3AttributeMetadata, IfcAttributeQueryStatus, IfcAttributeQueryWork};
use crate::validation::metadata;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader, IfcSnapshot, IfcValue};
use std::{mem::size_of, sync::Arc};
use super::source_index::{IdMatch, IfcSourceIds};
impl super::preparation::IfcMvdRequest for IfcAlignmentDirectRequest { fn valid_row(&self, ordinal: usize) -> bool { kind(&self.targets[ordinal].kind).is_some() } }
#[path = "direct-native-data.rs"]
mod data;
use data::*;
#[cfg(test)]
#[path = "direct-tests.rs"]
mod tests;
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcAlignmentDirectTarget {
    pub segment: String,
    pub kind: String,
    pub applicable: bool,
}
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct IfcAlignmentDirectRequest {
    pub targets: Vec<IfcAlignmentDirectTarget>,
}
impl IfcSourceIds for IfcAlignmentDirectRequest {
    fn source_len(&self) -> usize {
        self.targets.len()
    }
    fn source_id(&self, row: usize) -> Result<u64, ValueError> {
        positive_id(&self.targets[row].segment)
    }
}
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
pub struct IfcAlignmentDirectTargetReport {
    pub segment: String,
    pub kind: String,
    pub applicable: bool,
    pub status: &'static str,
    pub diagnostics: Vec<IfcConceptDiagnostic>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub curve: Option<String>,
}
#[derive(Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct IfcAlignmentDirectReport {
    pub concept: &'static str,
    pub predicate: &'static str,
    pub source: IfcConceptSource,
    pub schema_conformance: &'static str,
    pub view_conformance: &'static str,
    pub geometry_conformance: &'static str,
    pub layout_terminal_conformance: &'static str,
    pub parent_curve_conformance: &'static str,
    pub status: &'static str,
    pub targets: Vec<IfcAlignmentDirectTargetReport>,
    pub diagnostics: Vec<IfcConceptDiagnostic>,
}
#[path = "direct-clause.rs"]
mod clause;
#[path = "direct-work.rs"]
mod work;
pub use work::IfcAlignmentDirectWork;
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
