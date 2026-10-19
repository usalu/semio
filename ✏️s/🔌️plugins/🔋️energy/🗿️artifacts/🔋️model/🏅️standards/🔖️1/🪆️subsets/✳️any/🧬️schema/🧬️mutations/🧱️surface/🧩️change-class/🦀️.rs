//! 🧩️ Energy model mutation — `ChangeSurfaceClass`: Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧩️ `change-surface-class` payload. Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-class")]
pub struct ChangeSurfaceClass {
    pub id: crate::model::EntityId,
    pub new_class: crate::model::SurfaceClass,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_class<Op: From<ChangeSurfaceClass>>(id: crate::model::EntityId, new_class: crate::model::SurfaceClass) -> Op {
    ChangeSurfaceClass { id, new_class }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSurfaceClass
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSurfaceClass>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change surface {} class to {:?}", self.id.0, self.new_class), &format!("Klasse von Oberfläche {} auf {:?} ändern", self.id.0, self.new_class))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSurfaceClass
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSurfaceClass>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-class", record: "ChangedSurfaceClass" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
