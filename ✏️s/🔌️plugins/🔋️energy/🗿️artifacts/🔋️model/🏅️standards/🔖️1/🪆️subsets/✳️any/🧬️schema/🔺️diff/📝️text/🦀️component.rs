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
    pub async fn apply_to_artifact(&self, artifact: &EnergyModelArtifact) -> protocol::MutationApplyResult<EnergyModelArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(structure) = &self.structure {
                next.structure = structure.clone();
            }
            if let Some(zones) = &self.zones {
                next.zones = zones.clone();
            }
            if let Some(referenced_model) = &self.referenced_model {
                next.referenced_model = referenced_model.clone();
            }
            if let Some(results_json) = &self.results_json {
                next.results_json = results_json.clone();
            }
            next
        })
    }
}

impl MutationDiff<EnergyModelSnapshot> for EnergyModelDiff {
    async fn apply(&self, snapshot: &EnergyModelSnapshot) -> protocol::MutationApplyResult<EnergyModelSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(structure) = &self.structure {
                next.structure = structure.clone();
            }
            if let Some(zones) = &self.zones {
                next.zones = zones.clone();
            }
            if let Some(referenced_model) = &self.referenced_model {
                next.referenced_model = referenced_model.clone();
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
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
        take!(structure);
        take!(zones);
        take!(referenced_model);
        take!(results_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🖼️ Whole-snapshot replacement diff.
pub async fn diff_set_snapshot(snapshot: &EnergyModelSnapshot) -> EnergyModelDiff {
    EnergyModelDiff {
        artifact: Some(Box::new(EnergyModelArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

/// 🏢️ Whole-model replacement diff — mints+caches `structure`/`zones` together from `model` via
/// [`crate::artifacts::model::energy_children_from_model`] (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM). Replaces the old `diff_set_model_json` (which set
/// the now-removed `model_json` field directly).
pub async fn diff_from_model(model: &crate::model::Model) -> EnergyModelDiff {
    let (structure, zones) = crate::artifacts::model::energy_children_from_model(model);
    EnergyModelDiff { structure: Some(structure), zones: Some(zones), ..Default::default() }
}

/// 📋️ Preview results-json field delta (not applied by MutationDiff).
pub async fn diff_set_results_json(results_json: impl Into<String>) -> EnergyModelDiff {
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

    #[semio_framework_async_macros::async_test]
    async fn empty_diff_is_a_no_operation() {
        let base = crate::artifacts::model::schema::empty_energy_model_snapshot();
        let diff = EnergyModelDiff::default();
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_results_do_not_enter_snapshot() {
        let base = crate::artifacts::model::schema::empty_energy_model_snapshot();
        let diff = diff_set_results_json("{\"ok\":true}");
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
        let artifact = EnergyModelArtifact::from_snapshot(base);
        let next = diff
            .apply_to_artifact(&artifact)
            .expect("valid artifact diff");
        assert_eq!(next.results_json, "{\"ok\":true}");
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_from_model_regenerates_structure_and_zones_together() {
        let base = crate::artifacts::model::schema::empty_energy_model_snapshot();
        let model = crate::model::Model { name: "Demo".into(), ..crate::model::Model::default() };
        let diff = diff_from_model(&model);
        let applied = diff.apply(&base).expect("valid mutation diff");
        assert_eq!(crate::artifacts::model::energy_model(&applied), model);
        assert_eq!(applied.structure.child_id, applied.zones.child_id, "structure/zones must share one scene id");
    }
}
//#endregion 🧪️Tests
