//! 🏚️ Energy model mutation — `DeleteZone`: Removes one thermal zone. It RESTRICTS rather than cascades: while any space, surface, gain, HVAC object, grouping or airflow node still names the zone it is refused with `mutation.invariant`, because a cascade here would have to delete surfaces, which cascade again to fenestrations and adjacency pairs.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏚️ `delete-zone` payload. Removes one thermal zone. It RESTRICTS rather than cascades: while any space, surface, gain, HVAC object, grouping or airflow node still names the zone it is refused with `mutation.invariant`, because a cascade here would have to delete surfaces, which cascade again to fenestrations and adjacency pairs.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-zone")]
pub struct DeleteZone {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_zone(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteZone(DeleteZone { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "zone", kind: "delete-zone", record: "DeletedZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete zone {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
