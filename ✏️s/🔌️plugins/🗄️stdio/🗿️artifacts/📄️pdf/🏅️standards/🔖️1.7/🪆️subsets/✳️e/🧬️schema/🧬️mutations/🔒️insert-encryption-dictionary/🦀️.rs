//! 🔒️ Authoritative PDF/E mutation for inserting a Standard Security Handler dictionary.

use super::remove_encryption_dictionary::RemoveEncryptionDictionary;
use super::PdfEMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct InsertEncryptionDictionary {
    pub version: i64,
    pub revision: i64,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for InsertEncryptionDictionary {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "encryption-dictionary", kind: "insert-encryption-dictionary", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::insert_object(&mut next, support::encryption_dictionary(self.version, self.revision));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfEMutation> {
        vec![PdfEMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: self.version, revision: self.revision })]
    }

    fn label(&self) -> String {
        format!("Insert encryption dictionary V{} R{}", self.version, self.revision)
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
    fn inserts_the_requested_security_handler() {
        let base = PdfSnapshot::default();
        let mutation = InsertEncryptionDictionary { version: 2, revision: 3 };
        let outcome = <InsertEncryptionDictionary as MutationKind<PdfSnapshot, PdfEMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::encryption_dictionary_with(&next, 2, 3).is_some());
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
