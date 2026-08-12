//! 🔺️ EnergyModel artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::model::schema::diff::*;

use crate::artifacts::model::schema::EnergyModelArtifact;
use crate::artifacts::model::EnergyModelSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
impl EnergyModelDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &EnergyModelArtifact) -> EnergyModelArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(model_json) = &self.model_json {
            next.model_json = model_json.clone();
        }
        if let Some(results_json) = &self.results_json {
            next.results_json = results_json.clone();
        }
        next
    }
}

impl MutationDiff<EnergyModelSnapshot> for EnergyModelDiff {
    fn apply(&self, snapshot: &EnergyModelSnapshot) -> EnergyModelSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(model_json) = &self.model_json {
            next.model_json = model_json.clone();
        }
        next
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
        take!(model_json);
        take!(results_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &EnergyModelSnapshot) -> EnergyModelDiff {
    EnergyModelDiff {
        artifact: Some(Box::new(EnergyModelArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

/// 🏢️ Model-json field delta.
pub fn diff_set_model_json(model_json: impl Into<String>) -> EnergyModelDiff {
    EnergyModelDiff {
        model_json: Some(model_json.into()),
        ..Default::default()
    }
}

/// 📋️ Preview results-json field delta (not applied by MutationDiff).
pub fn diff_set_results_json(results_json: impl Into<String>) -> EnergyModelDiff {
    EnergyModelDiff {
        results_json: Some(results_json.into()),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_diff_is_a_no_operation() {
        let base = crate::artifacts::model::schema::empty_energy_model_snapshot();
        let diff = EnergyModelDiff::default();
        assert_eq!(diff.apply(&base), base);
    }

    #[test]
    fn preview_results_do_not_enter_snapshot() {
        let base = crate::artifacts::model::schema::empty_energy_model_snapshot();
        let diff = diff_set_results_json("{\"ok\":true}");
        assert_eq!(diff.apply(&base), base);
        let artifact = EnergyModelArtifact::from_snapshot(base);
        let next = diff.apply_to_artifact(&artifact);
        assert_eq!(next.results_json, "{\"ok\":true}");
    }
}
//#endregion 🧪️Tests
