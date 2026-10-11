//! 🧬️ EnergyModel snapshot schema — artifact-lane fields only.

use crate::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;


//#region 🔖️Snapshot
/// 📸️ Persisted energy-model document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`energy→C:value,table R:model`): the old
/// `model_json: String` opaque-JSON field is replaced by two fixed composed CHILD slots — this
/// artifact no longer defines its own persisted-value/table content model, it composes stdio's
/// `value`/`table` subsets instead (see the artifact root's `🔖️Composition` region for the full
/// before/after and the honest exception carve-out for `Surface.vertices_m`). `referenced_model` is
/// a new forward `ArtifactLink` slot. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Text and pack both encode
/// the derived `EnergyModelPackRecord` below.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::RetireOwned)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelSnapshot {
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
    /// 🌦️ Forward link to the `🌦️epw` stdio artifact this model is simulated against — a link slot
    /// exactly like `referenced_model`, never an inlined `WeatherData` (ticket
    /// 26/09/06/ENERGY-PLUGIN-END-TO-END).
    #[state(artifact)]
    #[link_slot(roles("weather"))]
    pub weather_link: Option<store::ArtifactLink>,
}

impl Default for EnergyModelSnapshot {
    fn default() -> Self {
        energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model::default(), None)
    }
}

// 🌱️ Hand-written, not derived — `structure`/`zones` are `store::ArtifactChild<S>` and
// `referenced_model` is `Option<store::ArtifactLink>`, neither of which has a
// `#[derive(ToValue, FromValue)]`-reachable impl (fan-out playbook trap #3; `ArtifactLink` mirrors
// the same framework-exempt shape). `model: crate::model::Model` goes through `ToValue`/`FromValue`
// directly — `Model` now derives both.
impl ToValue for EnergyModelSnapshot {
    fn to_value(&self) -> DslValue {
        semio_framework_value::DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("model".to_string(), self.model.to_value()),
            ("structure".to_string(), semio_framework_value::ToValue::to_value(&self.structure)),
            ("zones".to_string(), semio_framework_value::ToValue::to_value(&self.zones)),
            ("referencedModel".to_string(), semio_framework_value::ToValue::to_value(&self.referenced_model)),
            ("weatherLink".to_string(), semio_framework_value::ToValue::to_value(&self.weather_link)),
        ])
    }
}
impl FromValue for EnergyModelSnapshot {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = semio_framework_value::DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(semio_framework_value::DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            schema: String::from_value(field("schema"))?,
            model: crate::model::Model::from_value(field("model"))?,
            structure: semio_framework_value::FromValue::from_value(field("structure"))?,
            zones: semio_framework_value::FromValue::from_value(field("zones"))?,
            referenced_model: semio_framework_value::FromValue::from_value(field("referencedModel"))?,
            weather_link: semio_framework_value::FromValue::from_value(field("weatherLink"))?,
        })
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️PackRecord







//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️round-trip/🦀️.rs"]
mod round_trip_tests;

//#endregion 🧪️Tests

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge
