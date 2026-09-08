//! 🟫️ Energy model mutation — `CreateSurface`: Adds one planar polygon surface to an existing zone with an existing construction. The exterior boundary arrives as its two halves — a `boundary` discriminator and the `interzoneSurfaceId` only the `Interzone` arm carries — because `dsl::DslScalar` binds unit variants only.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟫️ `create-surface` payload. Adds one planar polygon surface to an existing zone with an existing construction. The exterior boundary arrives as its two halves — a `boundary` discriminator and the `interzoneSurfaceId` only the `Interzone` arm carries — because `dsl::DslScalar` binds unit variants only.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-surface")]
pub struct CreateSurface {
    pub id: crate::model::EntityId,
    pub name: String,
    pub zone_id: crate::model::EntityId,
    pub class: crate::model::SurfaceClass,
    pub vertices_m: Vec<[f64; 3]>,
    pub construction_id: crate::model::EntityId,
    pub boundary: crate::model::OutsideBoundaryKind,
    pub interzone_surface_id: Option<crate::model::EntityId>,
    pub sun_exposed: bool,
    pub wind_exposed: bool,
    pub multiplier: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_surface(id: crate::model::EntityId, name: String, zone_id: crate::model::EntityId, class: crate::model::SurfaceClass, vertices_m: Vec<[f64; 3]>, construction_id: crate::model::EntityId, boundary: crate::model::OutsideBoundaryKind, interzone_surface_id: Option<crate::model::EntityId>, sun_exposed: bool, wind_exposed: bool, multiplier: u32) -> EnergyModelMutation {
    EnergyModelMutation::CreateSurface(CreateSurface { id, name, zone_id, class, vertices_m, construction_id, boundary, interzone_surface_id, sun_exposed, wind_exposed, multiplier })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateSurface {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "surface", kind: "create-surface", record: "CreatedSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create surface \"{}\"", self.name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
