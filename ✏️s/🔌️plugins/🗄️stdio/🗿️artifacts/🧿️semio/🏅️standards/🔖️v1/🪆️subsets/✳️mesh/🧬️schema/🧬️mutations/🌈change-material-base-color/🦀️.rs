//! 🌈 `change-material-base-color` — sets a material's PBR base color. SMO's stroke-color precedent: a color is treated as ONE cohesive value (never edited channel-by-channel from outside), so `change`, not `replace`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioRgba;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeMaterialBaseColor {
    pub id: String,
    pub new_base_color: SemioRgba,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for ChangeMaterialBaseColor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-base-color", kind: "change-material-base-color", record: "ChangedMaterialBaseColor" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change material \"{}\" base color", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
