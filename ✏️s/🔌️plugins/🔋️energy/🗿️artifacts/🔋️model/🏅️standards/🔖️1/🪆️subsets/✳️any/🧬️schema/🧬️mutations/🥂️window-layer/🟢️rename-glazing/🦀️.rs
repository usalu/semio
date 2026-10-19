//! 🟢️ Energy model mutation — `RenameGlazingMaterial`: Renames one glazing material, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟢️ `rename-glazing-material` payload. Renames one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-glazing-material")]
pub struct RenameGlazingMaterial {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_glazing_material<Op: From<RenameGlazingMaterial>>(id: crate::model::EntityId, new_name: String) -> Op {
    RenameGlazingMaterial { id, new_name }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RenameGlazingMaterial
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RenameGlazingMaterial>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "glazing-material", kind: "rename-glazing-material", record: "RenamedGlazingMaterial" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename Glazing Material of glazing material {}", self.id.0), &format!("Verglasungsmaterial {} umbenennen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RenameGlazingMaterial
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RenameGlazingMaterial>,
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
