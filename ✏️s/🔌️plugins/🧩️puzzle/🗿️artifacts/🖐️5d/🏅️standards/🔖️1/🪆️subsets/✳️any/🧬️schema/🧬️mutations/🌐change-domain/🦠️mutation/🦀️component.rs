//! 🌐 Puzzle5d mutation — `ChangeDomain`: changes the document's design domain classification.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
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
pub fn change_domain(new_domain: String) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangeDomain(ChangeDomain { new_domain })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeDomain {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "domain", kind: "change-domain", record: "ChangedDomain" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change domain to \"{}\"", self.new_domain)
    }
}
//#endregion 🔖️Mutation
