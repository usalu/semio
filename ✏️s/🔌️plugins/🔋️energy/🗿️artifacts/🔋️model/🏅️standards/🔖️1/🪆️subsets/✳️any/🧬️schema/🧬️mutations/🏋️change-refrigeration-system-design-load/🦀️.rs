//! 🏋️ Energy model mutation — `ChangeRefrigerationSystemDesignLoad`: Sets design load (W) on one refrigeration system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏋️ `change-refrigeration-system-design-load` payload. Sets design load (W) on one refrigeration system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-refrigeration-system-design-load")]
pub struct ChangeRefrigerationSystemDesignLoad {
    pub id: crate::model::EntityId,
    pub new_design_load_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_refrigeration_system_design_load(id: crate::model::EntityId, new_design_load_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRefrigerationSystemDesignLoad(ChangeRefrigerationSystemDesignLoad { id, new_design_load_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRefrigerationSystemDesignLoad {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "refrigeration-system", kind: "change-refrigeration-system-design-load", record: "ChangedRefrigerationSystemDesignLoad" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Refrigeration System Design Load of refrigeration system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
