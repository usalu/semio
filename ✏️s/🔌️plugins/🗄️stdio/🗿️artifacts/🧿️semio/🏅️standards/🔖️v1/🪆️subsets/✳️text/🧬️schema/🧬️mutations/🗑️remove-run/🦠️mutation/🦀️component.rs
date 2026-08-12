//! ➖️ `remove-run` — takes one run out of the sequence, addressed by BASE-state index.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveRun {
    pub index: usize,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for RemoveRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "run", kind: "remove-run", record: "RemovedRun" };

    fn diff(&self, base: &SemioTextSnapshot) -> <SemioTextMutation as protocol::Mutation<SemioTextSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove run #{}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
