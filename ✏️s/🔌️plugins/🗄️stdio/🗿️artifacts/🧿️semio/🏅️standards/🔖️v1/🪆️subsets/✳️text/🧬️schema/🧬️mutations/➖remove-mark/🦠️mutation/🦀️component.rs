//! ➖️ `remove-mark` — detaches one inline mark from a run, addressed by BASE-state
//! `{run_index, index}`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveMark {
    pub run_index: usize,
    pub index: usize,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for RemoveMark {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "mark", kind: "remove-mark", record: "RemovedMarkFromRun" };

    async fn diff(&self, base: &SemioTextSnapshot) -> protocol::MutationOutcome<<SemioTextMutation as protocol::Mutation<SemioTextSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove mark #{} from run #{}", self.index, self.run_index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.run_index.to_string(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
