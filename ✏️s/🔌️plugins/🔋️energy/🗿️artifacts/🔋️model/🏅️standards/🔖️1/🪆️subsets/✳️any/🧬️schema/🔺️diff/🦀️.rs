//! 🧬️ EnergyModel diff schema — sparse field delta over the artifact.

use crate::{EnergyStructureChild, EnergyZonesChild};
use framework_schema::ArtifactSchema;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};

//#region 🔖️LinkSlotDelta
/// 🔗️ A link slot's delta. The field is `Option<EnergyLinkSlotDelta>`, and ABSENT means the slot did
/// not change at all — a typed three-state instead of the `Option<Option<ArtifactLink>>` double
/// option, whose JSON form collapsed "unchanged" and "now detached" onto the same `null` and made a
/// detach undecodable (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum EnergyLinkSlotDelta {
    Detached,
    Attached { link: store::ArtifactLink },
}
//#endregion 🔖️LinkSlotDelta

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the energy-model artifact. `structure`/`zones` are always-present
/// slots (never absent, only ever replaced) — single-`Option`, matching `mathematical`'s/`forms`'s
/// diff shape. The two LINK slots use `Option<EnergyLinkSlotDelta>` instead of the migration recipe
/// §8 double-`Option` that `layout` still carries: absent still means "unchanged", but "now
/// detached" is the explicit [`EnergyLinkSlotDelta::Detached`] variant rather than an inner `None`
/// that JSON renders as the same `null` as the outer one.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::EnergyModelArtifact>>,
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
            ("structure".to_string(), semio_framework_value::ToValue::to_value(&self.structure)),
            ("zones".to_string(), semio_framework_value::ToValue::to_value(&self.zones)),
            ("referencedModel".to_string(), self.referenced_model.to_value()),
            ("weatherLink".to_string(), self.weather_link.to_value()),
            ("resultsJson".to_string(), self.results_json.to_value()),
        ])
    }
}
impl FromValue for EnergyModelDiff {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            artifact: Option::from_value(field("artifact"))?,
            schema: Option::from_value(field("schema"))?,
            model: Option::from_value(field("model"))?,
            structure: semio_framework_value::FromValue::from_value(field("structure"))?,
            zones: semio_framework_value::FromValue::from_value(field("zones"))?,
            referenced_model: Option::from_value(field("referencedModel"))?,
            weather_link: Option::from_value(field("weatherLink"))?,
            results_json: Option::from_value(field("resultsJson"))?,
        })
    }
}
//#endregion 🔖️Diff

use crate::schema::EnergyModelArtifact;
use crate::EnergyModelSnapshot;
use protocol::MutationDiff;

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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
