//! ✂️ Authoritative PDF/A mutation for removing an attached file relationship.

use super::set_af_relationship::SetAfRelationship;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveAfRelationship {
    pub file_name: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for RemoveAfRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "af-relationship", kind: "remove-af-relationship", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::file_spec_named(&next, &self.file_name) {
            support::remove_entry(&mut next, id, "AFRelationship");
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfAMutation> {
        support::file_spec_named(base, &self.file_name)
            .and_then(|id| support::object(base, id))
            .and_then(|value| support::dict_name(value, "AFRelationship"))
            .map(|relationship| PdfAMutation::SetAfRelationship(SetAfRelationship { file_name: self.file_name.clone(), relationship: relationship.to_string() }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        format!("Remove AF relationship from \"{}\"", self.file_name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.file_name.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
