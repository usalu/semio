//! ➖️ `remove-mark` — detaches one inline mark from a run, addressed by BASE-state
//! `{run_index, index}`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::{SemioTextMutation, add_mark};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveMark {
    pub run_index: usize,
    pub index: usize,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for RemoveMark {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "mark", kind: "remove-mark", record: "RemovedMarkFromRun" };

    fn diff(&self, base: &SemioTextSnapshot) -> protocol::MutationOutcome<<SemioTextMutation as protocol::Mutation<SemioTextSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove mark #{} from run #{}", self.index, self.run_index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.run_index.to_string(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
