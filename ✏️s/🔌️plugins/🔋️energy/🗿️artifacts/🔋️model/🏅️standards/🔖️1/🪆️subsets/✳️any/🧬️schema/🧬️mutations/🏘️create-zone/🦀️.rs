//! 🏘️ Energy model mutation — `CreateZone`: Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏘️ `create-zone` payload. Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-zone")]
pub struct CreateZone {
    pub id: crate::model::EntityId,
    pub name: String,
    pub volume_m3: f64,
    pub multiplier: u32,
    pub conditioned: bool,
    pub part_of_total_floor_area: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_zone(id: crate::model::EntityId, name: String, volume_m3: f64, multiplier: u32, conditioned: bool, part_of_total_floor_area: bool) -> EnergyModelMutation {
    EnergyModelMutation::CreateZone(CreateZone { id, name, volume_m3, multiplier, conditioned, part_of_total_floor_area })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "zone", kind: "create-zone", record: "CreatedZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create zone \"{}\"", self.name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
