//! 🔗️ `bind-representation` — appends a new independent-lifecycle LINK to the kit's
//! `representations` pool (FINAL-state addressing). `role` joins this link to the
//! `SemioKitType.id` it represents (module doc comment on `📸️snapshot/🦀️.rs`).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, unbind_representation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct BindRepresentation {
    pub target: store::os_io::ArtifactRef,
    pub pin: store::LinkPin,
    pub role: String,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for BindRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "representation", kind: "bind-representation", record: "BoundRepresentation" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Bind representation for {}", self.role)
    }
    fn target(&self) -> Vec<String> {
        vec![self.role.clone()]
    }
}
//#endregion 🔖️Payload
