//! 🖋️ Energy model mutation — `RenameThermalEnclosure`: Sets one thermal enclosure's identity field; a name a sibling already holds is refused.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖋️ `rename-thermal-enclosure` payload. Sets one thermal enclosure's identity field; a name a sibling already holds is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-thermal-enclosure")]
pub struct RenameThermalEnclosure {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_thermal_enclosure(id: crate::model::EntityId, new_name: String) -> EnergyModelMutation {
    EnergyModelMutation::RenameThermalEnclosure(RenameThermalEnclosure { id, new_name })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RenameThermalEnclosure {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "thermal-enclosure", kind: "rename-thermal-enclosure", record: "RenamedThermalEnclosure" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename Thermal Enclosure of thermal enclosure {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
