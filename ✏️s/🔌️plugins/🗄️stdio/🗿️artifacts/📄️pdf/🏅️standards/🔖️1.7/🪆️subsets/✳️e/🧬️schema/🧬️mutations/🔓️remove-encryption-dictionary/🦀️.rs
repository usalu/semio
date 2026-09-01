//! 🔓️ Authoritative PDF/E mutation for removing a matching encryption dictionary.

use super::insert_encryption_dictionary::InsertEncryptionDictionary;
use super::PdfEMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveEncryptionDictionary {
    pub version: i64,
    pub revision: i64,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for RemoveEncryptionDictionary {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "encryption-dictionary", kind: "remove-encryption-dictionary", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::encryption_dictionary_with(&next, self.version, self.revision) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfEMutation> {
        support::encryption_dictionary_with(base, self.version, self.revision)
            .map(|_| PdfEMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: self.version, revision: self.revision }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        format!("Remove encryption dictionary V{} R{}", self.version, self.revision)
    }

    fn target(&self) -> Vec<String> {
        vec![self.version.to_string(), self.revision.to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn removes_only_a_present_security_handler() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::encryption_dictionary(2, 3));
        let mutation = RemoveEncryptionDictionary { version: 2, revision: 3 };
        let outcome = <RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfEMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::encryption_dictionary_with(&next, 2, 3).is_none());
        assert_eq!(<RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfEMutation>>::inverse(&mutation, &base).len(), 1);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
