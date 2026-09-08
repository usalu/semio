//! 🔺️ EnergyModel artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::diff::*;

use crate::schema::EnergyModelArtifact;
use crate::EnergyModelSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
/// 🔗️ The link a slot delta leaves behind — `Detached` clears the slot, `Attached` fills it.
fn link_of(delta: &EnergyLinkSlotDelta) -> Option<store::ArtifactLink> {
    match delta {
        EnergyLinkSlotDelta::Detached => None,
        EnergyLinkSlotDelta::Attached { link } => Some(link.clone()),
    }
}

impl EnergyModelDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &EnergyModelArtifact) -> protocol::MutationApplyResult<EnergyModelArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(model) = &self.model {
                next.model = model.clone();
            }
            if let Some(structure) = &self.structure {
                next.structure = structure.clone();
            }
            if let Some(zones) = &self.zones {
                next.zones = zones.clone();
            }
            if let Some(delta) = &self.referenced_model {
                next.referenced_model = link_of(delta);
            }
            if let Some(delta) = &self.weather_link {
                next.weather_link = link_of(delta);
            }
            if let Some(results_json) = &self.results_json {
                next.results_json = results_json.clone();
            }
            next
        })
    }
}

impl MutationDiff<EnergyModelSnapshot> for EnergyModelDiff {
    fn apply(&self, snapshot: &EnergyModelSnapshot) -> protocol::MutationApplyResult<EnergyModelSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(model) = &self.model {
                next.model = model.clone();
            }
            if let Some(structure) = &self.structure {
                next.structure = structure.clone();
            }
            if let Some(zones) = &self.zones {
                next.zones = zones.clone();
            }
            if let Some(delta) = &self.referenced_model {
                next.referenced_model = link_of(delta);
            }
            if let Some(delta) = &self.weather_link {
                next.weather_link = link_of(delta);
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
        take!(model);
        take!(structure);
        take!(zones);
        take!(referenced_model);
        take!(weather_link);
        take!(results_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &EnergyModelSnapshot) -> EnergyModelDiff {
    EnergyModelDiff { artifact: Some(Box::new(EnergyModelArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

/// 🏢️ Whole-model replacement diff — mints+caches `structure`/`zones` together from `model` via
/// [`crate::energy_children_from_model`] (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM). Replaces the old `diff_set_model_json` (which set
/// the now-removed `model_json` field directly).
pub fn diff_from_model(model: crate::model::Model) -> EnergyModelDiff {
    let (structure, zones) = crate::energy_children_from_model(&model);
    EnergyModelDiff { model: Some(model), structure: Some(structure), zones: Some(zones), ..Default::default() }
}

/// 📋️ Preview results-json field delta (not applied by MutationDiff).
pub fn diff_set_results_json(results_json: impl Into<String>) -> EnergyModelDiff {
    EnergyModelDiff { results_json: Some(results_json.into()), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type EnergyModelDiffText = String;
//#endregion 🚚️Carrier
