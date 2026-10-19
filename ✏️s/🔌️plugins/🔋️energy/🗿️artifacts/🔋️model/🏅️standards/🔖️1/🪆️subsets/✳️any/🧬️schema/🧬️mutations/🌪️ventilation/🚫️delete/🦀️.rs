//! 🚫️ Energy model mutation — `DeleteMechanicalVentilation`: Removes one mechanical ventilation specification.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚫️ `delete-mechanical-ventilation` payload. Removes one mechanical ventilation specification.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-mechanical-ventilation")]
pub struct DeleteMechanicalVentilation {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_mechanical_ventilation<Op: From<DeleteMechanicalVentilation>>(id: crate::model::EntityId) -> Op {
    DeleteMechanicalVentilation { id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteMechanicalVentilation
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteMechanicalVentilation> + From<crate::CreateMechanicalVentilation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "mechanical-ventilation", kind: "delete-mechanical-ventilation", record: "DeletedMechanicalVentilation" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete Mechanical Ventilation {}", self.id.0), &format!("Mechanische Lüftung {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteMechanicalVentilation
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteMechanicalVentilation> + From<crate::CreateMechanicalVentilation>,
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
