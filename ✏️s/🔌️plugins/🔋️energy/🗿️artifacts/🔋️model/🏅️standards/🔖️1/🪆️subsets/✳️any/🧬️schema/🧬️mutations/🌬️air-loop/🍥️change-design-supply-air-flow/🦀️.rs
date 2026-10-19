//! 🍥️ Energy model mutation — `ChangeAirLoopDesignSupplyAirFlow`: Sets the volumetric air flow the loop is designed around, in m³/s.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍥️ `change-air-loop-design-supply-air-flow` payload. Sets the volumetric air flow the loop is designed around, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-air-loop-design-supply-air-flow")]
pub struct ChangeAirLoopDesignSupplyAirFlow {
    pub id: crate::model::EntityId,
    pub new_design_supply_air_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_air_loop_design_supply_air_flow<Op: From<ChangeAirLoopDesignSupplyAirFlow>>(id: crate::model::EntityId, new_design_supply_air_flow_m3_s: f64) -> Op {
    ChangeAirLoopDesignSupplyAirFlow { id, new_design_supply_air_flow_m3_s }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeAirLoopDesignSupplyAirFlow
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeAirLoopDesignSupplyAirFlow>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change air loop {} design supply air flow to {:?}", self.id.0, self.new_design_supply_air_flow_m3_s), &format!("Bemessungszuluftvolumenstrom von Luftkreislauf {} auf {:?} ändern", self.id.0, self.new_design_supply_air_flow_m3_s))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeAirLoopDesignSupplyAirFlow
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeAirLoopDesignSupplyAirFlow>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "air-loop", kind: "change-air-loop-design-supply-air-flow", record: "ChangedAirLoopDesignSupplyAirFlow" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
