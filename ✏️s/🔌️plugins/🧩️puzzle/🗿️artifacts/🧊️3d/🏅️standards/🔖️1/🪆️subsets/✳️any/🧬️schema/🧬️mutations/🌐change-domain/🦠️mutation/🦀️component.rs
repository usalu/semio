//! 🌐 Puzzle3d mutation — `ChangeDomain`: changes the document's design domain classification.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌐 `change-domain` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-domain")]
pub struct ChangeDomain {
    pub new_domain: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_domain(new_domain: String) -> Puzzle3dMutation {
    Puzzle3dMutation::ChangeDomain(ChangeDomain { new_domain })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ChangeDomain {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "domain", kind: "change-domain", record: "ChangedDomain" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change domain to \"{}\"", self.new_domain)
    }
}
//#endregion 🔖️Mutation
