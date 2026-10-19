//! 🎈️ Energy model mutation — `ChangeMechanicalVentilationFanDeltaPressure`: Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎈️ `change-mechanical-ventilation-fan-delta-pressure` payload. Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-fan-delta-pressure")]
pub struct ChangeMechanicalVentilationFanDeltaPressure {
    pub id: crate::model::EntityId,
    pub new_fan_delta_pressure_pa: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_mechanical_ventilation_fan_delta_pressure<Op: From<ChangeMechanicalVentilationFanDeltaPressure>>(id: crate::model::EntityId, new_fan_delta_pressure_pa: f64) -> Op {
    ChangeMechanicalVentilationFanDeltaPressure { id, new_fan_delta_pressure_pa }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeMechanicalVentilationFanDeltaPressure
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeMechanicalVentilationFanDeltaPressure>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Mechanical Ventilation Fan Delta Pressure of mechanical ventilation {}", self.id.0), &format!("Ventilatordruckerhöhung von mechanischer Lüftung {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeMechanicalVentilationFanDeltaPressure
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeMechanicalVentilationFanDeltaPressure>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mechanical-ventilation", kind: "change-mechanical-ventilation-fan-delta-pressure", record: "ChangedMechanicalVentilationFanDeltaPressure" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
