//! 💦️ Energy model mutation — `ChangeEquipmentGainLatentFraction`: Sets latent fraction on one equipment gain, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💦️ `change-equipment-gain-latent-fraction` payload. Sets latent fraction on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-latent-fraction")]
pub struct ChangeEquipmentGainLatentFraction {
    pub id: crate::model::EntityId,
    pub new_latent_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_equipment_gain_latent_fraction(id: crate::model::EntityId, new_latent_fraction: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeEquipmentGainLatentFraction(ChangeEquipmentGainLatentFraction { id, new_latent_fraction })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeEquipmentGainLatentFraction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-latent-fraction", record: "ChangedEquipmentGainLatentFraction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Equipment Gain Latent Fraction of equipment gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
