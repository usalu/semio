//! 🧭️ Energy model mutation — `ChangeMechanicalVentilationZone`: Re-points one mechanical ventilation's zone reference; a zone the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧭️ `change-mechanical-ventilation-zone` payload. Re-points one mechanical ventilation's zone reference; a zone the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-zone")]
pub struct ChangeMechanicalVentilationZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_mechanical_ventilation_zone<Op: From<ChangeMechanicalVentilationZone>>(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> Op {
    ChangeMechanicalVentilationZone { id, new_zone_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeMechanicalVentilationZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeMechanicalVentilationZone>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mechanical-ventilation", kind: "change-mechanical-ventilation-zone", record: "ChangedMechanicalVentilationZone" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Mechanical Ventilation Zone of mechanical ventilation {}", self.id.0), &format!("Zone von mechanischer Lüftung {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeMechanicalVentilationZone
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeMechanicalVentilationZone>,
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
