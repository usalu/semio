//! 🔗️ `bind-representation` — appends a new independent-lifecycle LINK to the kit's
//! `representations` pool (FINAL-state addressing). `role` joins this link to the
//! `SemioKitType.id` it represents (module doc comment on `📸️snapshot/🦀️component.rs`).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindRepresentation {
    pub target: store::os_io::ArtifactRef,
    pub pin: store::LinkPin,
    pub role: String,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for BindRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "representation", kind: "bind-representation", record: "BoundRepresentation" };

    async fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Bind representation for {}", self.role)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.role.clone()]
    }
}
//#endregion 🔖️Payload
