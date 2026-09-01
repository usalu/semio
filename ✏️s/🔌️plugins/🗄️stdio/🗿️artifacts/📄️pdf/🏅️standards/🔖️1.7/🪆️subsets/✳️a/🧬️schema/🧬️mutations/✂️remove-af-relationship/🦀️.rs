//! ✂️ Authoritative PDF/A mutation for removing an attached file relationship.

use super::set_af_relationship::SetAfRelationship;
use super::PdfAMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
    use protocol::MutationDiff;

    #[test]
    fn removes_and_can_restore_the_relationship() {
        let mut base = PdfSnapshot::default();
        let id = support::insert_file_spec(&mut base, "measurements.csv");
        support::set_entry(&mut base, id, "AFRelationship", PdfObject::Name("Data".to_string()));
        let mutation = RemoveAfRelationship { file_name: "measurements.csv".to_string() };
        let outcome = <RemoveAfRelationship as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        let next_id = support::file_spec_named(&next, &mutation.file_name).unwrap();
        assert!(support::object(&next, next_id).and_then(|value| support::dict_name(value, "AFRelationship")).is_none());
        assert_eq!(<RemoveAfRelationship as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).len(), 1);
    }
}
//#endregion 🧪️Tests
