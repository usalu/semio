//! 🗑️ `delete-face` — removes an id-keyed face. Does NOT cascade into `shell.faces` membership: no modify-verb exists for a shell's face list (only `create-shell`/`delete-shell` govern the shell's own existence), so severing that membership here would be uninvertible within the approved vocabulary — flagged, not invented. Absent `id` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, create_face, delete_face};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteFace {
    pub id: String,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for DeleteFace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "face", kind: "delete-face", record: "DeletedFace" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete face \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
