//! 🔓️ Authoritative PDF/A mutation for removing a matching encryption dictionary.

use super::insert_encryption_dictionary::InsertEncryptionDictionary;
use super::PdfAMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEncryptionDictionary {
    pub version: i64,
    pub revision: i64,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for RemoveEncryptionDictionary {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "encryption-dictionary", kind: "remove-encryption-dictionary", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::encryption_dictionary_with(&next, self.version, self.revision) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfAMutation> {
        support::encryption_dictionary_with(base, self.version, self.revision)
            .map(|_| PdfAMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: self.version, revision: self.revision }))
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
        let outcome = <RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::encryption_dictionary_with(&next, 2, 3).is_none());
        assert_eq!(<RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).len(), 1);
    }
}
//#endregion 🧪️Tests
