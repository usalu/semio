//! 🧬️ Imperative diff schema — sparse field delta over the artifact.

use crate::{ProcedureFlowChild, ProcedureTextChild};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the imperative artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff). `flow`/`text` carry a whole-handle replacement
/// (content-addressed, so a changed handle IS the change signal — see
/// `📓️wave3-reports/writer-report.md`'s `document: Option<WriterDocumentChild>` precedent; both
/// slots are never absent, only ever replaced, so a single `Option<…Child>` — not the double-
/// `Option` an optional slot needs — is the sparse-vs-unchanged signal here).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::ProcedureArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub flow: Option<ProcedureFlowChild>,
    #[state(artifact)]
    pub text: Option<ProcedureTextChild>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ProcedureStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

use crate::schema::ProcedureArtifact;
use crate::ProcedureSnapshot;
use protocol::MutationDiff;

impl ProcedureDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &ProcedureArtifact) -> protocol::MutationApplyResult<ProcedureArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(handle) = &self.flow {
                next.flow = handle.clone();
            }
            if let Some(handle) = &self.text {
                next.text = handle.clone();
            }
            next
        })
    }
}

impl MutationDiff<ProcedureSnapshot> for ProcedureDiff {
    fn apply(&self, snapshot: &ProcedureSnapshot) -> protocol::MutationApplyResult<ProcedureSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(handle) = &self.flow {
                next.flow = handle.clone();
            }
            if let Some(handle) = &self.text {
                next.text = handle.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(flow);
        take!(text);
    }
}

/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: ProcedureSnapshot) -> ProcedureDiff {
    ProcedureDiff { artifact: Some(Box::new(ProcedureArtifact::from_snapshot(snapshot))), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
