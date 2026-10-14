//! 🧬️ EnergyModel diff schema — a sparse, typed, per-entity delta over the snapshot. [`EnergyModelDiff::model`] mirrors
//! [`crate::model::Model`] field for field with [`ModelPatch`]: every collection is keyed rows (removed, inserted,
//! modified), every record a field patch. Only the central applier turns a diff into a snapshot.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff_patch::Unchanged;
use crate::snapshot::EnergyModelSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[path = "🧱️entities/🦀️.rs"]
pub mod entities;

pub use entities::*;

//#region 🔖️LinkSlotDelta
/// 🔗️ A link slot's delta. The field is `Option<EnergyLinkSlotDelta>`, and ABSENT means the slot did
/// not change at all — a typed three-state instead of the `Option<Option<ArtifactLink>>` double
/// option, whose JSON form collapsed "unchanged" and "now detached" onto the same `null` and made a
/// detach undecodable (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum EnergyLinkSlotDelta {
    Detached,
    Attached { link: store::ArtifactLink },
}

impl EnergyLinkSlotDelta {
    /// 🔗️ The delta that leaves a slot holding `link`.
    pub fn leaving(link: &Option<store::ArtifactLink>) -> Self {
        link.as_ref().map_or(Self::Detached, |link| Self::Attached { link: link.clone() })
    }

    fn link(&self) -> Option<store::ArtifactLink> {
        match self {
            Self::Detached => None,
            Self::Attached { link } => Some(link.clone()),
        }
    }
}
//#endregion 🔖️LinkSlotDelta

//#region 🔖️Diff
/// 🔺️ Sparse delta for the energy-model artifact: the model patch plus the two link slots. Absent means unchanged.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
#[artifact_schema(id = "s.energy.model")]
#[value(rename_all = "camelCase")]
pub struct EnergyModelDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Unchanged::unchanged")]
    pub model: ModelPatch,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<EnergyLinkSlotDelta>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub weather_link: Option<EnergyLinkSlotDelta>,
}

impl EnergyModelDiff {
    /// 🩹 The diff that carries exactly `model`.
    pub fn of(model: ModelPatch) -> Self {
        Self { model, ..Self::default() }
    }

    /// 🌦️ The diff that attaches or detaches the weather file slot.
    pub fn weather(delta: EnergyLinkSlotDelta) -> Self {
        Self { weather_link: Some(delta), ..Self::default() }
    }

    /// 🪢️ The diff that attaches or detaches the referenced-model slot.
    pub fn referenced(delta: EnergyLinkSlotDelta) -> Self {
        Self { referenced_model: Some(delta), ..Self::default() }
    }
}

impl MutationDiff<EnergyModelSnapshot> for EnergyModelDiff {
    fn apply(&self, base: &EnergyModelSnapshot, _capability: ApplyCapability) -> MutationApplyResult<EnergyModelSnapshot> {
        let mut next = base.clone();
        self.model.commit_onto(&mut next.model)?;
        if let Some(delta) = &self.referenced_model {
            next.referenced_model = delta.link();
        }
        if let Some(delta) = &self.weather_link {
            next.weather_link = delta.link();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.model.absorb(other.model);
        if other.referenced_model.is_some() {
            self.referenced_model = other.referenced_model;
        }
        if other.weather_link.is_some() {
            self.weather_link = other.weather_link;
        }
    }
}

impl DiffAlgebra<EnergyModelSnapshot> for EnergyModelDiff {
    fn inverse(&self, base: &EnergyModelSnapshot) -> Self {
        Self {
            model: self.model.inverse(&base.model),
            referenced_model: self.referenced_model.as_ref().map(|_| EnergyLinkSlotDelta::leaving(&base.referenced_model)),
            weather_link: self.weather_link.as_ref().map(|_| EnergyLinkSlotDelta::leaving(&base.weather_link)),
        }
    }

    fn is_empty(&self) -> bool {
        self.model.unchanged() && self.referenced_model.is_none() && self.weather_link.is_none()
    }
}
//#endregion 🔖️Diff

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧬️Facet
/// 🧬️ The diff facet's five handcrafted language leaves, owned here beside the diff they describe; the artifact schema
/// descriptor (`semio_s_artifact_energy_model_schema`) assembles them.
pub const FACET: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {
    rust: include_str!("🦀️.rs"),
    typescript: include_str!("🟦️.ts"),
    graphql: include_str!("🔗️.graphql"),
    json_schema: include_str!("🔣️.json"),
    proto: include_str!("🛰️.proto"),
};
//#endregion 🧬️Facet

//#region 🚪️Representations
#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
pub mod text;
#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
pub mod binary;
//#endregion 🚪️Representations
