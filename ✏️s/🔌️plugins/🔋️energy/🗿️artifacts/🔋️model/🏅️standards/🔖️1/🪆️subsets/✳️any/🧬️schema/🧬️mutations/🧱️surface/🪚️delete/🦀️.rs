//! 🪚️ Energy model mutation — `DeleteSurface`: Removes one surface and CASCADES: every fenestration hosted on it and every adjacency pair naming it go with it, reported at info level as `mutation.cascade`. It still RESTRICTS on the one reference a cascade could not answer for — another surface naming this one as its interzone partner — because silently rewriting that surface's boundary condition is a physics decision no delete may take.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪚️ `delete-surface` payload. Removes one surface and CASCADES: every fenestration hosted on it and every adjacency pair naming it go with it, reported at info level as `mutation.cascade`. It still RESTRICTS on the one reference a cascade could not answer for — another surface naming this one as its interzone partner — because silently rewriting that surface's boundary condition is a physics decision no delete may take.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-surface")]
pub struct DeleteSurface {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_surface<Op: From<DeleteSurface>>(id: crate::model::EntityId) -> Op {
    DeleteSurface { id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteSurface
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteSurface> + From<crate::boundary_mutations::ConnectSurfaces> + From<crate::fenestration_mutations::CreateFenestration> + From<crate::CreateSurface> + From<crate::fenestration_mutations::ReplaceFenestrationVertices>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "surface", kind: "delete-surface", record: "DeletedSurface" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete surface {}", self.id.0), &format!("Oberfläche {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteSurface
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteSurface> + From<crate::boundary_mutations::ConnectSurfaces> + From<crate::fenestration_mutations::CreateFenestration> + From<crate::CreateSurface> + From<crate::fenestration_mutations::ReplaceFenestrationVertices>,
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
