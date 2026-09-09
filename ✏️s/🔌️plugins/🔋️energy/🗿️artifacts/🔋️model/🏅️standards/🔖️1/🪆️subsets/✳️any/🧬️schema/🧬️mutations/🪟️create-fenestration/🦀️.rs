//! 🪟️ Energy model mutation — `CreateFenestration`: Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪟️ `create-fenestration` payload. Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-fenestration")]
pub struct CreateFenestration {
    pub id: crate::model::EntityId,
    pub name: String,
    pub surface_id: crate::model::EntityId,
    pub u_value_w_m2k: f64,
    pub shgc: f64,
    pub vlt: f64,
    pub area_m2: f64,
    pub height_m: f64,
    pub sill_height_m: f64,
    pub frame_conductance_w_k: f64,
    pub divider_conductance_w_k: f64,
    pub overhang_depth_m: f64,
    pub overhang_offset_m: f64,
    pub fin_depth_m: f64,
    pub fin_offset_m: f64,
    pub glazing_construction_id: Option<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_fenestration(
    id: crate::model::EntityId,
    name: String,
    surface_id: crate::model::EntityId,
    u_value_w_m2k: f64,
    shgc: f64,
    vlt: f64,
    area_m2: f64,
    height_m: f64,
    sill_height_m: f64,
    frame_conductance_w_k: f64,
    divider_conductance_w_k: f64,
    overhang_depth_m: f64,
    overhang_offset_m: f64,
    fin_depth_m: f64,
    fin_offset_m: f64,
    glazing_construction_id: Option<crate::model::EntityId>,
) -> EnergyModelMutation {
    EnergyModelMutation::CreateFenestration(CreateFenestration {
        id,
        name,
        surface_id,
        u_value_w_m2k,
        shgc,
        vlt,
        area_m2,
        height_m,
        sill_height_m,
        frame_conductance_w_k,
        divider_conductance_w_k,
        overhang_depth_m,
        overhang_offset_m,
        fin_depth_m,
        fin_offset_m,
        glazing_construction_id,
    })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateFenestration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "fenestration", kind: "create-fenestration", record: "CreatedFenestration" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create fenestration \"{}\"", self.name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
