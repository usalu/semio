//! 🏗️ `create-vertex` — brings a new id-keyed vertex into existence at `point`. A duplicate `id` already present in `base` is a no-op (never a duplicate id).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, delete_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepVertex, SemioBrepSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateVertex {
    pub id: String,
    pub point: SemioPoint3,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for CreateVertex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "vertex", kind: "create-vertex", record: "CreatedVertex" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create vertex \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
