//! 🧬️ EnergyModel diff schema — sparse field delta over the artifact.

use crate::artifacts::model::{EnergyStructureChild, EnergyZonesChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_os_kernel::{from_dsl_value, to_dsl_value, DslValue, FromValue, ToValue, ValueError};

//#region 🔖️LinkSlotDelta
/// 🔗️ A link slot's delta. The field is `Option<EnergyLinkSlotDelta>`, and ABSENT means the slot did
/// not change at all — a typed three-state instead of the `Option<Option<ArtifactLink>>` double
/// option, whose JSON form collapsed "unchanged" and "now detached" onto the same `null` and made a
/// detach undecodable (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum EnergyLinkSlotDelta {
    Detached,
    Attached { link: store::ArtifactLink },
}
//#endregion 🔖️LinkSlotDelta

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the energy-model artifact. `structure`/`zones` are always-present
/// slots (never absent, only ever replaced) — single-`Option`, matching `mathematical`'s/`forms`'s
/// diff shape. `referenced_model` uses the optional-slot double-`Option` shape (outer = "did the
/// presence/identity change", inner = "is it now present") per the migration recipe's §8
/// convention, matching `layout`'s own `referenced_model` diff field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::model::schema::EnergyModelArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub model: Option<crate::model::Model>,
    #[state(artifact)]
    pub structure: Option<EnergyStructureChild>,
    #[state(artifact)]
    pub zones: Option<EnergyZonesChild>,
    #[state(artifact)]
    pub referenced_model: Option<EnergyLinkSlotDelta>,
    #[state(artifact)]
    pub weather_link: Option<EnergyLinkSlotDelta>,
    #[state(artifact)]
    pub results_json: Option<String>,
}

// 🌱️ Hand-written, not derived — `structure`/`zones`/`referenced_model` are composed-child/link
// shapes without a `#[derive(ToValue, FromValue)]`-reachable impl (fan-out playbook trap #3, same
// as `📸️snapshot/🦀️.rs`'s `EnergyModelSnapshot`/`🧬️schema/🦀️component.rs`'s
// `EnergyModelArtifact`, bridged the same way here). `artifact: Option<Box<EnergyModelArtifact>>`
// needs no bridge — `EnergyModelArtifact` itself now has a hand-written `ToValue`/`FromValue`, and
// the blanket `Box<T: ToValue>`/`Option<T: ToValue>` impls compose straight through it.
impl ToValue for EnergyModelDiff {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("artifact".to_string(), self.artifact.to_value()),
            ("schema".to_string(), self.schema.to_value()),
            ("model".to_string(), self.model.to_value()),
            ("structure".to_string(), to_dsl_value(&self.structure).unwrap_or(DslValue::Null)),
            ("zones".to_string(), to_dsl_value(&self.zones).unwrap_or(DslValue::Null)),
            ("referencedModel".to_string(), self.referenced_model.to_value()),
            ("weatherLink".to_string(), self.weather_link.to_value()),
            ("resultsJson".to_string(), self.results_json.to_value()),
        ])
    }
}
impl FromValue for EnergyModelDiff {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or(DslValue::Null);
        Ok(Self {
            artifact: Option::from_value(field("artifact"))?,
            schema: Option::from_value(field("schema"))?,
            model: Option::from_value(field("model"))?,
            structure: from_dsl_value(field("structure")).map_err(ValueError::new)?,
            zones: from_dsl_value(field("zones")).map_err(ValueError::new)?,
            referenced_model: Option::from_value(field("referencedModel"))?,
            weather_link: Option::from_value(field("weatherLink"))?,
            results_json: Option::from_value(field("resultsJson"))?,
        })
    }
}
//#endregion 🔖️Diff
