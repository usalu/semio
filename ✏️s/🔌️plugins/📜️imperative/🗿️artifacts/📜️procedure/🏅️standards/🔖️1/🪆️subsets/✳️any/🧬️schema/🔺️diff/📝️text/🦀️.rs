//! 🔺️ Imperative artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::procedure::schema::diff::ProcedureDiff;
use crate::artifacts::procedure::schema::ProcedureArtifact;
use crate::artifacts::procedure::ProcedureSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl ProcedureDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
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
            if let Some(list) = &self.selected_step_ids {
                next.selected_step_ids = list.values.clone();
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            if let Some(value) = &self.contributions_json {
                next.contributions_json = value.clone();
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
        take!(selected_step_ids);
        take!(locale);
        take!(contributions_json);
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
mod tests {
    use super::*;
    use crate::artifacts::procedure::schema::default_snapshot;

    #[semio_framework_async_macros::async_test]
    async fn imperative_diff_absorb_whole_artifact_wins() {
        let mut diff = ProcedureDiff { flow: Some(crate::artifacts::procedure::procedure_flow_child_with_owner(&crate::artifacts::procedure::Path::new())), ..Default::default() };
        let replacement = ProcedureDiff { artifact: Some(Box::new(ProcedureArtifact::default())), ..Default::default() };
        diff.absorb(replacement);
        assert!(diff.artifact.is_some());
        assert!(diff.flow.is_none());
    }

    /// 🔁 Replaces the retired `path_delta_remove_round_trips_via_apply` — whole-list
    /// `ProcedurePathDelta` deltas no longer exist, since composed children are opaque and a diff
    /// only ever whole-handle-replaces `flow` — with the equivalent real-behavior law: a `flow`
    /// handle minted from an edited working scene applies as a clean whole-handle replace.
    #[semio_framework_async_macros::async_test]
    async fn flow_handle_replace_round_trips_via_apply() {
        let base = default_snapshot();
        let mut path = crate::artifacts::procedure::procedure_working_scene(&base).path;
        assert!(path.steps.iter().any(|step| step.id == "step-1"));
        path.steps.retain(|step| step.id != "step-1");
        let diff = crate::artifacts::procedure::diff_replace_flow(&path);
        let next = diff.apply(&base).expect("valid mutation diff");
        let next_path = crate::artifacts::procedure::procedure_working_scene(&next).path;
        assert!(next_path.steps.iter().all(|step| step.id != "step-1"));
    }
}
//#endregion 🧪️Tests
