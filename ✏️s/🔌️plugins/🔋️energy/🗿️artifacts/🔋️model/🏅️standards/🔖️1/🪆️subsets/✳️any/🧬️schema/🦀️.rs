//! 🧬️ EnergyModel artifact schema — every field with its state class.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::snapshot::{EnergyModelSnapshot, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Relocated from `⚙️engine/🦀️.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — a pure codec helper over the document
/// type, not engine behaviour.
pub fn empty_energy_model_snapshot() -> EnergyModelSnapshot {
    EnergyModelSnapshot::default()
}

/// 🏢️ The typed `Model` behind a snapshot's composed `structure`/`zones` children — reads through
/// the working-scene cache (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM). Replaces the old
/// `serde_json::from_str(&snapshot.model_json)` decode-on-demand now that `model_json` is gone.
pub fn model_from_snapshot(snapshot: &EnergyModelSnapshot) -> Result<crate::model::Model, String> {
    Ok(crate::snapshot::energy_model(snapshot))
}

/// 📕️ Encode a typed `Model` into snapshot form — mints+caches its composed `structure`/`zones`
/// children in one call via [`crate::energy_snapshot_with_state`].
pub fn snapshot_from_model(model: &crate::model::Model) -> Result<EnergyModelSnapshot, String> {
    Ok(crate::snapshot::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None))
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Artifact
/// 🧬️ Full energy-model artifact across the artifact lane (no UI app). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: mirrors `EnergyModelSnapshot`'s `structure`/`zones`/
/// `referenced_model` field swap identically (same shape every composed exemplar's full-artifact
/// struct mirrors its snapshot). `results_json` is UNCHANGED — a preview-only field (recomputed by
/// the BEM engine, never persisted, never part of the snapshot), no composition applies to it.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub model: crate::model::Model,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub structure: EnergyStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub zones: EnergyZonesChild,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    pub referenced_model: Option<store::ArtifactLink>,
    /// 🌦️ Forward link to the `🌦️epw` stdio artifact this model is simulated against.
    #[state(artifact)]
    #[link_slot(roles("weather"))]
    pub weather_link: Option<store::ArtifactLink>,
    /// 📋️ Opaque JSON of `crate::Results` — recomputed by the BEM engine; never persisted.
    #[state(artifact)]
    pub results_json: String,
}

// 🌱️ Hand-written, not derived — same composed-child/link shape as
// `📸️snapshot/🦀️.rs`'s `EnergyModelSnapshot` impl.
impl ToValue for EnergyModelArtifact {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("model".to_string(), self.model.to_value()),
            ("structure".to_string(), ToValue::to_value(&self.structure)),
            ("zones".to_string(), ToValue::to_value(&self.zones)),
            ("referencedModel".to_string(), ToValue::to_value(&self.referenced_model)),
            ("weatherLink".to_string(), ToValue::to_value(&self.weather_link)),
            ("resultsJson".to_string(), self.results_json.to_value()),
        ])
    }
}
impl FromValue for EnergyModelArtifact {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            schema: String::from_value(field("schema"))?,
            model: crate::model::Model::from_value(field("model"))?,
            structure: FromValue::from_value(field("structure"))?,
            zones: FromValue::from_value(field("zones"))?,
            referenced_model: FromValue::from_value(field("referencedModel"))?,
            weather_link: FromValue::from_value(field("weatherLink"))?,
            results_json: String::from_value(field("resultsJson"))?,
        })
    }
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for EnergyModelArtifact {
    fn default() -> Self {
        Self::from_snapshot(EnergyModelSnapshot::default())
    }
}

impl EnergyModelArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> EnergyModelSnapshot {
        EnergyModelSnapshot { schema: self.schema.clone(), model: self.model.clone(), structure: self.structure.clone(), zones: self.zones.clone(), referenced_model: self.referenced_model.clone(), weather_link: self.weather_link.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving preview empty.
    pub fn from_snapshot(snapshot: EnergyModelSnapshot) -> Self {
        Self { schema: snapshot.schema, model: snapshot.model, structure: snapshot.structure, zones: snapshot.zones, referenced_model: snapshot.referenced_model, weather_link: snapshot.weather_link, results_json: String::new() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: EnergyModelSnapshot) {
        self.schema = snapshot.schema;
        self.model = snapshot.model;
        self.structure = snapshot.structure;
        self.zones = snapshot.zones;
        self.referenced_model = snapshot.referenced_model;
        self.weather_link = snapshot.weather_link;
    }
}
//#endregion 🔖️Conversions

#[path = "🧾️descriptor/🦀️.rs"]
mod descriptor;
pub use descriptor::energy_model_artifact_schema_descriptor;
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
