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
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureDiff {
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

use crate::ProcedureSnapshot;
use protocol::MutationDiff;

impl MutationDiff<ProcedureSnapshot> for ProcedureDiff {
    fn apply(&self, snapshot: &ProcedureSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ProcedureSnapshot> {
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
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
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

impl protocol::DiffAlgebra<ProcedureSnapshot> for ProcedureDiff {
    fn inverse(&self, base: &ProcedureSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), flow: self.flow.as_ref().map(|_| base.flow.clone()), text: self.text.as_ref().map(|_| base.text.clone()) }
    }
    fn between(base: &ProcedureSnapshot, other: &ProcedureSnapshot) -> Self {
        Self { schema: (base.schema != other.schema).then(|| other.schema.clone()), flow: (base.flow != other.flow).then(|| other.flow.clone()), text: (base.text != other.text).then(|| other.text.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.flow.is_none() && self.text.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
