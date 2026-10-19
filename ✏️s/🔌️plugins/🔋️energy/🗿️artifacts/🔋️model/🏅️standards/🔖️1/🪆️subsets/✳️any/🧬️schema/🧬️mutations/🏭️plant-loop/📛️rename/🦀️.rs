//! 📛️ Energy model mutation — `RenamePlantLoop`: Sets one plant loop's identity field.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📛️ `rename-plant-loop` payload. Sets one plant loop's identity field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-plant-loop")]
pub struct RenamePlantLoop {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_plant_loop<Op: From<RenamePlantLoop>>(id: crate::model::EntityId, new_name: String) -> Op {
    RenamePlantLoop { id, new_name }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RenamePlantLoop
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RenamePlantLoop>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "plant-loop", kind: "rename-plant-loop", record: "RenamedPlantLoop" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change plant loop {} name to {:?}", self.id.0, self.new_name), &format!("Name von Anlagenkreislauf {} auf {:?} ändern", self.id.0, self.new_name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RenamePlantLoop
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RenamePlantLoop>,
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
