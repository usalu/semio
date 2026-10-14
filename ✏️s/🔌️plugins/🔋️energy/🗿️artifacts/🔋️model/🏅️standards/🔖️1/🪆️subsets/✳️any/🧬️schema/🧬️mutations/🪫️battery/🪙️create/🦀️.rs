//! 🪙️ Energy model mutation — `CreateBattery`: Adds one electrical storage unit. Capacity bounds the state of charge; the two power limits bound each timestep's charge and discharge, and the round-trip efficiency is what the stored energy is debited by.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪙️ `create-battery` payload. Adds one electrical storage unit. Capacity bounds the state of charge; the two power limits bound each timestep's charge and discharge, and the round-trip efficiency is what the stored energy is debited by.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-battery")]
pub struct CreateBattery {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub capacity_kwh: f64,
    pub max_charge_w: f64,
    pub max_discharge_w: f64,
    pub round_trip_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_battery<Op: From<CreateBattery>>(index: u32, id: crate::model::EntityId, capacity_kwh: f64, max_charge_w: f64, max_discharge_w: f64, round_trip_efficiency: f64) -> Op {
    CreateBattery { index, id, capacity_kwh, max_charge_w, max_discharge_w, round_trip_efficiency }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateBattery
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateBattery> + From<crate::DeleteBattery>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "battery", kind: "create-battery", record: "CreatedBattery" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Battery {} at index {}", self.id.0, self.index), &format!("Batterie {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
