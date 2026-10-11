//! ➖️ `remove-mark` — detaches one inline mark from a run, addressed by BASE-state
//! `{run_index, index}`.

use crate::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    fn inverse(&self, base: &SemioTextSnapshot) -> Result<Vec<SemioTextMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove mark #{} from run #{}", self.index, self.run_index), &format!("Auszeichnung #{} aus Textlauf #{} entfernen", self.index, self.run_index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.run_index.to_string(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
