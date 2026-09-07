//! 🔻️ Energy model mutation — `DeleteElectricalLoadCenter`: Removes one electrical load centre. Nothing in the document references a load centre — it is the referencing end of every generation edge — so this never has to refuse for use and never cascades.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔻️ `delete-electrical-load-center` payload. Removes one electrical load centre. Nothing in the document references a load centre — it is the referencing end of every generation edge — so this never has to refuse for use and never cascades.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-electrical-load-center")]
pub struct DeleteElectricalLoadCenter {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_electrical_load_center(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteElectricalLoadCenter(DeleteElectricalLoadCenter { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteElectricalLoadCenter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "electrical-load-center", kind: "delete-electrical-load-center", record: "DeletedElectricalLoadCenter" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Electrical Load Center {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
