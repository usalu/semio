//! 🧬️ EnergyModel diff schema — sparse field delta over the artifact.

use crate::artifacts::model::{EnergyStructureChild, EnergyZonesChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_os_kernel::{from_dsl_value, to_dsl_value, DslValue, FromValue, ToValue, ValueError};

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
    pub referenced_model: Option<Option<store::ArtifactLink>>,
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
            ("referencedModel".to_string(), to_dsl_value(&self.referenced_model).unwrap_or(DslValue::Null)),
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
            referenced_model: from_dsl_value(field("referencedModel")).map_err(ValueError::new)?,
            results_json: Option::from_value(field("resultsJson"))?,
        })
    }
}
//#endregion 🔖️Diff
