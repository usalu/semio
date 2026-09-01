//! 🔗️ Authoritative PDF/A mutation for setting an attached file relationship.

use super::remove_af_relationship::RemoveAfRelationship;
use super::PdfAMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn sets_the_relationship_on_the_named_file() {
        let mut base = PdfSnapshot::default();
        support::insert_file_spec(&mut base, "measurements.csv");
        let mutation = SetAfRelationship { file_name: "measurements.csv".to_string(), relationship: "Data".to_string() };
        let outcome = <SetAfRelationship as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        let id = support::file_spec_named(&next, &mutation.file_name).unwrap();
        assert_eq!(support::object(&next, id).and_then(|value| support::dict_name(value, "AFRelationship")), Some("Data"));
    }
}
//#endregion 🧪️Tests
