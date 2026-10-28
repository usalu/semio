//! 🪟️ Energy model mutation — `CreateFenestration`: Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪟️ `create-fenestration` payload. Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
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

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
