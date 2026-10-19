//! ⚛️ Energy model mutation — `ChangePvSystemDcCapacity`: Sets DC capacity (W) on one pv system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚛️ `change-pv-system-dc-capacity` payload. Sets DC capacity (W) on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-dc-capacity")]
pub struct ChangePvSystemDcCapacity {
    pub id: crate::model::EntityId,
    pub new_dc_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_pv_system_dc_capacity<Op: From<ChangePvSystemDcCapacity>>(id: crate::model::EntityId, new_dc_capacity_w: f64) -> Op {
    ChangePvSystemDcCapacity { id, new_dc_capacity_w }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePvSystemDcCapacity
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePvSystemDcCapacity>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pv-system", kind: "change-pv-system-dc-capacity", record: "ChangedPvSystemDcCapacity" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Pv System Dc Capacity of pv system {}", self.id.0), &format!("DC-Nennleistung von PV-Anlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePvSystemDcCapacity
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePvSystemDcCapacity>,
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
