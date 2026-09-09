//! 🔺️ Imperative artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::diff::ProcedureDiff;
use crate::schema::ProcedureArtifact;
use crate::ProcedureSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
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
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: ProcedureSnapshot) -> ProcedureDiff {
    ProcedureDiff { artifact: Some(Box::new(ProcedureArtifact::from_snapshot(snapshot))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ProcedureDiffText = String;
//#endregion 🚚️Carrier
