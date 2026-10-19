//! 🎐️ Energy model mutation — `ChangeLightingGainReturnAirFraction`: Sets return-air fraction on one lighting gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎐️ `change-lighting-gain-return-air-fraction` payload. Sets return-air fraction on one lighting gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-lighting-gain-return-air-fraction")]
pub struct ChangeLightingGainReturnAirFraction {
    pub id: crate::model::EntityId,
    pub new_return_air_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_lighting_gain_return_air_fraction<Op: From<ChangeLightingGainReturnAirFraction>>(id: crate::model::EntityId, new_return_air_fraction: f64) -> Op {
    ChangeLightingGainReturnAirFraction { id, new_return_air_fraction }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeLightingGainReturnAirFraction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeLightingGainReturnAirFraction>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "lighting-gain", kind: "change-lighting-gain-return-air-fraction", record: "ChangedLightingGainReturnAirFraction" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Lighting Gain Return Air Fraction of lighting gain {}", self.id.0), &format!("Rückluftanteil von Beleuchtungswärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeLightingGainReturnAirFraction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeLightingGainReturnAirFraction>,
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
