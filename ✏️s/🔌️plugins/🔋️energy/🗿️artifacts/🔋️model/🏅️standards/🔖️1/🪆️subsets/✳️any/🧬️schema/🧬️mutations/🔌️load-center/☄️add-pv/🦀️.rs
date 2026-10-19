//! ☄️ Energy model mutation — `AddElectricalLoadCenterPv`: Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☄️ `add-electrical-load-center-pv` payload. Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-electrical-load-center-pv")]
pub struct AddElectricalLoadCenterPv {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub pv_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_electrical_load_center_pv<Op: From<AddElectricalLoadCenterPv>>(id: crate::model::EntityId, index: u32, pv_id: crate::model::EntityId) -> Op {
    AddElectricalLoadCenterPv { id, index, pv_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterPv
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddElectricalLoadCenterPv> + From<crate::RemoveElectricalLoadCenterPv>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add pv system {} to electrical load center {}", self.pv_id.0, self.id.0), &format!("PV-Anlage {} zu Stromverteiler {} hinzufügen", self.pv_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterPv
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<AddElectricalLoadCenterPv> + From<crate::RemoveElectricalLoadCenterPv>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "electrical-load-center", kind: "add-electrical-load-center-pv", record: "AddedElectricalLoadCenterPv" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
