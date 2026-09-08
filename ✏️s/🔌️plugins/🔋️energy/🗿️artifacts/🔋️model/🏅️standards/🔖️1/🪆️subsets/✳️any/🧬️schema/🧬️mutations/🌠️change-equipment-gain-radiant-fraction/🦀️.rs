//! 🌠️ Energy model mutation — `ChangeEquipmentGainRadiantFraction`: Sets radiant fraction on one equipment gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌠️ `change-equipment-gain-radiant-fraction` payload. Sets radiant fraction on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-radiant-fraction")]
pub struct ChangeEquipmentGainRadiantFraction {
    pub id: crate::model::EntityId,
    pub new_radiant_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_equipment_gain_radiant_fraction(id: crate::model::EntityId, new_radiant_fraction: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeEquipmentGainRadiantFraction(ChangeEquipmentGainRadiantFraction { id, new_radiant_fraction })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeEquipmentGainRadiantFraction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-radiant-fraction", record: "ChangedEquipmentGainRadiantFraction" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Equipment Gain Radiant Fraction of equipment gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
