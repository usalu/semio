//! 🔗️ Authoritative PDF/A mutation for setting an attached file relationship.

use super::remove_af_relationship::RemoveAfRelationship;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAfRelationship {
    pub file_name: String,
    pub relationship: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for SetAfRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "af-relationship", kind: "set-af-relationship", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::file_spec_named(&next, &self.file_name) {
            support::set_entry(&mut next, id, "AFRelationship", PdfObject::Name(self.relationship.clone()));
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfAMutation> {
        let Some(id) = support::file_spec_named(base, &self.file_name) else { return Vec::new() };
        match support::object(base, id).and_then(|value| support::dict_name(value, "AFRelationship")) {
            Some(previous) => vec![PdfAMutation::SetAfRelationship(SetAfRelationship { file_name: self.file_name.clone(), relationship: previous.to_string() })],
            None => vec![PdfAMutation::RemoveAfRelationship(RemoveAfRelationship { file_name: self.file_name.clone() })],
        }
    }

    fn label(&self) -> String {
        format!("Set AF relationship for \"{}\"", self.file_name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.file_name.clone(), self.relationship.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
