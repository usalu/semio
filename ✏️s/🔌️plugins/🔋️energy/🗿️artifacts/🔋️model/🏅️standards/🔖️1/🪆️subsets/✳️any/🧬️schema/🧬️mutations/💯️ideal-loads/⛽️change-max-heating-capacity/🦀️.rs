//! ⛽️ Energy model mutation — `ChangeIdealLoadsSystemMaxHeatingCapacity`: Sets or clears the ideal-loads heating capacity limit in watts. Cleared means autosized.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛽️ `change-ideal-loads-system-max-heating-capacity` payload. Sets or clears the ideal-loads heating capacity limit in watts. Cleared means autosized.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-max-heating-capacity")]
pub struct ChangeIdealLoadsSystemMaxHeatingCapacity {
    pub id: crate::model::EntityId,
    pub new_capacity_present: bool,
    pub new_max_heating_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_max_heating_capacity<Op: From<ChangeIdealLoadsSystemMaxHeatingCapacity>>(id: crate::model::EntityId, new_capacity_present: bool, new_max_heating_capacity_w: f64) -> Op {
    ChangeIdealLoadsSystemMaxHeatingCapacity { id, new_capacity_present, new_max_heating_capacity_w }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMaxHeatingCapacity
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMaxHeatingCapacity>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-max-heating-capacity", record: "ChangedIdealLoadsSystemMaxHeatingCapacity" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ideal loads system {} maximum heating capacity to {:?}", self.id.0, self.new_capacity_present.then_some(self.new_max_heating_capacity_w)), &format!("Maximale Heizleistung von Ideallastsystem {} auf {:?} ändern", self.id.0, self.new_capacity_present.then_some(self.new_max_heating_capacity_w)))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMaxHeatingCapacity
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMaxHeatingCapacity>,
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
