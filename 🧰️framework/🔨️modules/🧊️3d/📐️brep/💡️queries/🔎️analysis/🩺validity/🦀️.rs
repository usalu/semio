//! 🩺️ Per-shape validity: the kernel's structural and geometric checks, reported only for the entities the
//! selected shape reaches (a verdict on one handle is never spoiled by an unrelated broken body).

use super::ShapeScope;
use crate::brep::queries::validation::{issue_reaches, validate_body};
use crate::brep::representation::error::KernelError;
use crate::brep::representation::topology::Body;

/// 🚦 How serious one issue is; `warning-` codes never make a shape invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
#[value(rename_all = "camelCase")]
pub enum IssueSeverity {
    Error,
    Warning,
}

/// 🩹 One finding: the arena entity label it names, a stable kebab-case code and a message.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ValidityIssue {
    pub entity: String,
    pub code: String,
    pub message: String,
    pub severity: IssueSeverity,
}

/// 🩺️ The validity of one shape.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ValidityReport {
    pub ok: bool,
    pub errors: usize,
    pub warnings: usize,
    pub issues: Vec<ValidityIssue>,
}

/// 🌡️ Runs the whole-body validator and keeps the issues that name an entity `scope` reaches.
///
/// An issue whose label cannot be read is kept (an unreadable diagnostic is not evidence of health).
pub fn validity(body: &Body, scope: &ShapeScope) -> Result<ValidityReport, KernelError> {
    scope.members(body)?;
    let reach = body.reachable_from(&scope.roots);
    let issues: Vec<ValidityIssue> = validate_body(body)
        .into_iter()
        .filter(|issue| issue_reaches(&reach, &issue.entity))
        .map(|issue| {
            let severity = if issue.code.starts_with("warning-") { IssueSeverity::Warning } else { IssueSeverity::Error };
            ValidityIssue { entity: issue.entity, code: issue.code.to_string(), message: issue.message, severity }
        })
        .collect();
    let errors = issues.iter().filter(|issue| issue.severity == IssueSeverity::Error).count();
    Ok(ValidityReport { ok: errors == 0, errors, warnings: issues.len() - errors, issues })
}
