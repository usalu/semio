//! 🌐️ `change-run-language` — sets one run's BCP-47 `language` tag, addressed by BASE-state index.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeRunLanguage {
    pub index: usize,
    pub new_language: String,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for ChangeRunLanguage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run-language", kind: "change-run-language", record: "ChangedRunLanguage" };

    fn diff(&self, base: &SemioTextSnapshot) -> protocol::MutationOutcome<<SemioTextMutation as protocol::Mutation<SemioTextSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change run #{} language to {}", self.index, self.new_language)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
