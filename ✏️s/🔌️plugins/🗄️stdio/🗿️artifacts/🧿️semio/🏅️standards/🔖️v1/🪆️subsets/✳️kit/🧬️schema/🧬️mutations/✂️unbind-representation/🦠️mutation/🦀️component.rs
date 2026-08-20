//! ✂️ `unbind-representation` — removes the link at `index` from `representations` (BASE-state
//! addressing). Idempotent no-op if out of range; inverse escrows the removed link from BASE.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnbindRepresentation {
    pub index: usize,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for UnbindRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "representation", kind: "unbind-representation", record: "UnboundRepresentation" };

    async fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Unbind representation at #{}", self.index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
