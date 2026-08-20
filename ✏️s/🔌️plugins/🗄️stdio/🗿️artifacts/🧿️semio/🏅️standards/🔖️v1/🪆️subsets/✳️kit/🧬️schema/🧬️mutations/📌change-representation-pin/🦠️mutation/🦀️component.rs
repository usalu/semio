//! 📌️ `change-representation-pin` — re-pins the link at `index` (target/role stay put; only
//! `pin` changes) — `change`, not `update`: a single narrow field, not an inseparable ≥2-field
//! facet rewritten atomically (📌️important.md's `change-link-pin` ruling).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeRepresentationPin {
    pub index: usize,
    pub pin: store::LinkPin,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for ChangeRepresentationPin {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation-pin", kind: "change-representation-pin", record: "ChangedRepresentationPin" };

    async fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Re-pin representation at #{}", self.index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
