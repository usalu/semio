//! 💨️ Energy model mutation — `ChangePvSystemFaimanHeatLossWind`: Sets Wind heat loss on one pv system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💨️ `change-pv-system-faiman-heat-loss-wind` payload. Sets Wind heat loss on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-faiman-heat-loss-wind")]
pub struct ChangePvSystemFaimanHeatLossWind {
    pub id: crate::model::EntityId,
    pub new_faiman_heat_loss_wind_w_s_m3k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_pv_system_faiman_heat_loss_wind<Op: From<ChangePvSystemFaimanHeatLossWind>>(id: crate::model::EntityId, new_faiman_heat_loss_wind_w_s_m3k: f64) -> Op {
    ChangePvSystemFaimanHeatLossWind { id, new_faiman_heat_loss_wind_w_s_m3k }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePvSystemFaimanHeatLossWind
where Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePvSystemFaimanHeatLossWind>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pv-system", kind: "change-pv-system-faiman-heat-loss-wind", record: "ChangedPvSystemFaimanHeatLossWind" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Wind heat loss of pv system {}", self.id.0), &format!("Windwärmeverlust von PV-Anlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePvSystemFaimanHeatLossWind
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePvSystemFaimanHeatLossWind>,
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
