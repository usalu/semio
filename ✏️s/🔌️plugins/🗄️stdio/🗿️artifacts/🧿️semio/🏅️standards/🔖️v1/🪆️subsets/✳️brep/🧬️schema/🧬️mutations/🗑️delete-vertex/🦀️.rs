//! 🗑️ `delete-vertex` — removes an id-keyed vertex, cascading to every edge severed by its removal
//! (an edge cannot exist with a dangling endpoint, and `edge` is itself a full create/delete-able
//! entity — captures the removed payload + severed cascade for its inverse, per `📌️important.md`'s
//! ruling: "delete captures payload + severed cascade"). Does NOT cascade further into loop
//! membership: `Loop` carries no `PersistentLabel` (SMO's ruling: `create-loop`/`delete-loop`
//! excluded) and no modify-verb exists for `loop.edges`, so a loop referencing a cascade-deleted
//! edge is left with a stale reference — the same honestly-flagged limitation the loop exclusion
//! already accepts, not something this triad can close without inventing an unaddressed verb.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, create_edge, create_vertex, delete_edge};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteVertex {
    pub id: String,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for DeleteVertex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "vertex", kind: "delete-vertex", record: "DeletedVertex" };

    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete vertex \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
