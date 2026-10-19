//! 🌘️ Energy model mutation — `RemoveElectricalLoadCenterPv`: Detaches one PV system from a load centre; the PV system itself survives, unattached.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌘️ `remove-electrical-load-center-pv` payload. Detaches one PV system from a load centre; the PV system itself survives, unattached.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-electrical-load-center-pv")]
pub struct RemoveElectricalLoadCenterPv {
    pub id: crate::model::EntityId,
    pub pv_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_electrical_load_center_pv<Op: From<RemoveElectricalLoadCenterPv>>(id: crate::model::EntityId, pv_id: crate::model::EntityId) -> Op {
    RemoveElectricalLoadCenterPv { id, pv_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveElectricalLoadCenterPv
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveElectricalLoadCenterPv> + From<crate::AddElectricalLoadCenterPv>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "electrical-load-center", kind: "remove-electrical-load-center-pv", record: "RemovedElectricalLoadCenterPv" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove pv system {} from electrical load center {}", self.pv_id.0, self.id.0), &format!("PV-Anlage {} aus Stromverteiler {} entfernen", self.pv_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveElectricalLoadCenterPv
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveElectricalLoadCenterPv> + From<crate::AddElectricalLoadCenterPv>,
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
