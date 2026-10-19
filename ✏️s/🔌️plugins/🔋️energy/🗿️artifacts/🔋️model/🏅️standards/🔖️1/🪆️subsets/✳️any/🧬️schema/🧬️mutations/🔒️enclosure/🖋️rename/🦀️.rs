//! 🖋️ Energy model mutation — `RenameThermalEnclosure`: Sets one thermal enclosure's identity field; a name a sibling already holds is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖋️ `rename-thermal-enclosure` payload. Sets one thermal enclosure's identity field; a name a sibling already holds is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-thermal-enclosure")]
pub struct RenameThermalEnclosure {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_thermal_enclosure<Op: From<RenameThermalEnclosure>>(id: crate::model::EntityId, new_name: String) -> Op {
    RenameThermalEnclosure { id, new_name }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RenameThermalEnclosure
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RenameThermalEnclosure>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "thermal-enclosure", kind: "rename-thermal-enclosure", record: "RenamedThermalEnclosure" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename Thermal Enclosure of thermal enclosure {}", self.id.0), &format!("Zonenverbund {} umbenennen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RenameThermalEnclosure
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RenameThermalEnclosure>,
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
