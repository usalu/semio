//! ➕️ `add-mark` — attaches one inline mark to a run at a FINAL-state index within that run's
//! `marks` (an intrinsically ordered, anonymous collection nested one level inside `runs`).

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddMark {
    pub run_index: usize,
    pub index: usize,
    pub mark: SemioTextMark,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for AddMark {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "mark", kind: "add-mark", record: "AddedMarkToRun" };

    fn diff(&self, base: &SemioTextSnapshot) -> protocol::MutationOutcome<<SemioTextMutation as protocol::Mutation<SemioTextSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add mark to run #{} at #{}", self.run_index, self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.run_index.to_string(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
