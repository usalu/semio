//! 🔺 `create-primitive` — inserts a new id-keyed primitive into mesh `mesh_id`. A duplicate `primitive_id` already present, or an absent `mesh_id`, is a no-op.

use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioPrimitive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreatePrimitive {
    pub mesh_id: String,
    pub primitive: SemioPrimitive,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreatePrimitive {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "primitive", kind: "create-primitive", record: "CreatedPrimitive" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create primitive \"{}\" in mesh \"{}\"", self.primitive.id, self.mesh_id), &format!("Primitiv \"{}\" in Netz \"{}\" erstellen", self.primitive.id, self.mesh_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive.id.clone()]
    }
}
//#endregion 🔖️Payload
