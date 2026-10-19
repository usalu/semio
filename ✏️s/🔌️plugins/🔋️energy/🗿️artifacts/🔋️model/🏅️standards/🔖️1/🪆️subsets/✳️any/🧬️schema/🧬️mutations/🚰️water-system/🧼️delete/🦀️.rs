//! 🧼️ Energy model mutation — `DeleteWaterSystem`: Removes one cold-water use system. Nothing in the document references one, so this never refuses for use.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧼️ `delete-water-system` payload. Removes one cold-water use system. Nothing in the document references one, so this never refuses for use.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-water-system")]
pub struct DeleteWaterSystem {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_water_system<Op: From<DeleteWaterSystem>>(id: crate::model::EntityId) -> Op {
    DeleteWaterSystem { id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteWaterSystem
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteWaterSystem> + From<crate::CreateWaterSystem>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "water-system", kind: "delete-water-system", record: "DeletedWaterSystem" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete Water System {}", self.id.0), &format!("Wasseranlage {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteWaterSystem
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteWaterSystem> + From<crate::CreateWaterSystem>,
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
