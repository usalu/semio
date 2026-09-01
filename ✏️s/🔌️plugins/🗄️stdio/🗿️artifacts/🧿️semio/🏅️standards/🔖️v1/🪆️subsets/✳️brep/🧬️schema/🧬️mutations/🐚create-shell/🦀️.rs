//! 🏗️ `create-shell` — brings a new id-keyed shell into existence with its full initial `faces` membership list (referencing already-existing faces) — the whole initial payload, per `create`'s canonical-args shape, since no per-membership verb exists to grow a shell's face list after creation. A duplicate `id` already present in `base` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, delete_shell};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepShell, BrepShellFace, SemioBrepSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateShell {
    pub id: String,
    #[value(default)]
    pub faces: Vec<BrepShellFace>,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for CreateShell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "shell", kind: "create-shell", record: "CreatedShell" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create shell \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
