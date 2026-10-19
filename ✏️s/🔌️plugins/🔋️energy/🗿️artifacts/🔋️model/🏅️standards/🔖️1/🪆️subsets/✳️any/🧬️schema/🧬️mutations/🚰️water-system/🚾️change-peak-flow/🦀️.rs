//! 🚾️ Energy model mutation — `ChangeWaterSystemPeakFlow`: Sets peak flow (L/s) on one water system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚾️ `change-water-system-peak-flow` payload. Sets peak flow (L/s) on one water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-water-system-peak-flow")]
pub struct ChangeWaterSystemPeakFlow {
    pub id: crate::model::EntityId,
    pub new_peak_flow_l_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_water_system_peak_flow<Op: From<ChangeWaterSystemPeakFlow>>(id: crate::model::EntityId, new_peak_flow_l_s: f64) -> Op {
    ChangeWaterSystemPeakFlow { id, new_peak_flow_l_s }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeWaterSystemPeakFlow
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeWaterSystemPeakFlow>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Water System Peak Flow of water system {}", self.id.0), &format!("Spitzenvolumenstrom von Wasseranlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeWaterSystemPeakFlow
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeWaterSystemPeakFlow>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "water-system", kind: "change-water-system-peak-flow", record: "ChangedWaterSystemPeakFlow" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
