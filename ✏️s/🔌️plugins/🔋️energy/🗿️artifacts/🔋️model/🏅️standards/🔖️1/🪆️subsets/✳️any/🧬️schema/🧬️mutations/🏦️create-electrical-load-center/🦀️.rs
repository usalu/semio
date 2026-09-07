//! 🏦️ Energy model mutation — `CreateElectricalLoadCenter`: Adds one electrical load centre — the node that sums on-site generation and storage against the building's electrical demand. `pvIds` and `batteryIds` are checked against the document; `generatorIds` is carried verbatim and NOT checked, because `Model` has no generator collection for it to reference (vocabulary §5.4), which is also why this group ships no `add-electrical-load-center-generator`.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏦️ `create-electrical-load-center` payload. Adds one electrical load centre — the node that sums on-site generation and storage against the building's electrical demand. `pvIds` and `batteryIds` are checked against the document; `generatorIds` is carried verbatim and NOT checked, because `Model` has no generator collection for it to reference (vocabulary §5.4), which is also why this group ships no `add-electrical-load-center-generator`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-electrical-load-center")]
pub struct CreateElectricalLoadCenter {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub generator_ids: Vec<crate::model::EntityId>,
    pub pv_ids: Vec<crate::model::EntityId>,
    pub battery_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_electrical_load_center(index: u32, id: crate::model::EntityId, name: String, generator_ids: Vec<crate::model::EntityId>, pv_ids: Vec<crate::model::EntityId>, battery_ids: Vec<crate::model::EntityId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateElectricalLoadCenter(CreateElectricalLoadCenter { index, id, name, generator_ids, pv_ids, battery_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateElectricalLoadCenter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "electrical-load-center", kind: "create-electrical-load-center", record: "CreatedElectricalLoadCenter" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Electrical Load Center {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
