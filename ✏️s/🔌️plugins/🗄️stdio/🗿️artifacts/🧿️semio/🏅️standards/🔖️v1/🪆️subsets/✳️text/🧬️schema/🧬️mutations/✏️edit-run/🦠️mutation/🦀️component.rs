//! ✏️ `edit-run` — replaces one run's authored `content` body, addressed by BASE-state index.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditRun {
    pub index: usize,
    pub new_content: String,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for EditRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "run", kind: "edit-run", record: "EditedRun" };

    fn diff(&self, base: &SemioTextSnapshot) -> <SemioTextMutation as protocol::Mutation<SemioTextSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit run #{}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
