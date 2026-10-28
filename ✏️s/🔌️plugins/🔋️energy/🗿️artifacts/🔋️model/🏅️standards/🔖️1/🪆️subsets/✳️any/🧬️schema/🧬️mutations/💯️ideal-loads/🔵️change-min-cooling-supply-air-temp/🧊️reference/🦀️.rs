//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeIdealLoadsSystemMinCoolingSupplyAirTemp;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMinCoolingSupplyAirTemp
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMinCoolingSupplyAirTemp>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-min-cooling-supply-air-temp", record: "ChangedIdealLoadsSystemMinCoolingSupplyAirTemp" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ideal loads system {} minimum cooling supply air temperature to {:?}", self.id.0, self.new_min_cooling_supply_air_temp_c), &format!("Minimale Kühlzulufttemperatur von Ideallastsystem {} auf {:?} ändern", self.id.0, self.new_min_cooling_supply_air_temp_c))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMinCoolingSupplyAirTemp
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMinCoolingSupplyAirTemp>,
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
