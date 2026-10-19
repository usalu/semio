//! 🟧️ Energy model mutation — `ChangeIdealLoadsSystemMaxCoolingCapacity`: Sets or clears the ideal-loads cooling capacity limit in watts. Cleared means autosized.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟧️ `change-ideal-loads-system-max-cooling-capacity` payload. Sets or clears the ideal-loads cooling capacity limit in watts. Cleared means autosized.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-max-cooling-capacity")]
pub struct ChangeIdealLoadsSystemMaxCoolingCapacity {
    pub id: crate::model::EntityId,
    pub new_capacity_present: bool,
    pub new_max_cooling_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_max_cooling_capacity<Op: From<ChangeIdealLoadsSystemMaxCoolingCapacity>>(id: crate::model::EntityId, new_capacity_present: bool, new_max_cooling_capacity_w: f64) -> Op {
    ChangeIdealLoadsSystemMaxCoolingCapacity { id, new_capacity_present, new_max_cooling_capacity_w }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMaxCoolingCapacity
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMaxCoolingCapacity>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-max-cooling-capacity", record: "ChangedIdealLoadsSystemMaxCoolingCapacity" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ideal loads system {} maximum cooling capacity to {:?}", self.id.0, self.new_capacity_present.then_some(self.new_max_cooling_capacity_w)), &format!("Maximale Kühlleistung von Ideallastsystem {} auf {:?} ändern", self.id.0, self.new_capacity_present.then_some(self.new_max_cooling_capacity_w)))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMaxCoolingCapacity
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMaxCoolingCapacity>,
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
