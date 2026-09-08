//! 🌳️ Energy model mutation — `CreateShadingSurface`: Adds one free-standing solar obstruction — the site-level sibling of a window's own attached overhang and fins, which stay on the fenestration.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌳️ `create-shading-surface` payload. Adds one free-standing solar obstruction — the site-level sibling of a window's own attached overhang and fins, which stay on the fenestration.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-shading-surface")]
pub struct CreateShadingSurface {
    pub id: crate::model::EntityId,
    pub name: String,
    pub vertices_m: Vec<[f64; 3]>,
    pub transmittance_schedule_id: Option<crate::model::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_shading_surface(id: crate::model::EntityId, name: String, vertices_m: Vec<[f64; 3]>, transmittance_schedule_id: Option<crate::model::ScheduleId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateShadingSurface(CreateShadingSurface { id, name, vertices_m, transmittance_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateShadingSurface {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "shading-surface", kind: "create-shading-surface", record: "CreatedShadingSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create shading surface \"{}\"", self.name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
