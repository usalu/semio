//! 🪟️ Energy model mutation — `CreateFenestration`: Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪟️ `create-fenestration` payload. Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_fenestration<Op: From<CreateFenestration>>(
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
    index: Option<u32>,
) -> Op {
    CreateFenestration {
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
        glazing_construction_id, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateFenestration
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateFenestration> + From<crate::DeleteFenestration>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "fenestration", kind: "create-fenestration", record: "CreatedFenestration" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create fenestration \"{}\"", self.name), &format!("Fenster \"{}\" erstellen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateFenestration
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateFenestration> + From<crate::DeleteFenestration>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
