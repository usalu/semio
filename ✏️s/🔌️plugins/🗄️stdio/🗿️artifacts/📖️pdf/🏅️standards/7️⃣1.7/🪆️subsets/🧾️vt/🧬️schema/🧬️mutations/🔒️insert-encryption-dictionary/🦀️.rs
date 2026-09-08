//! 🔒️ Authoritative PDF/VT mutation for inserting a Standard Security Handler dictionary.

use super::remove_encryption_dictionary::RemoveEncryptionDictionary;
use super::PdfVtMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertEncryptionDictionary {
    pub version: i64,
    pub revision: i64,
}

impl MutationKind<PdfSnapshot, PdfVtMutation> for InsertEncryptionDictionary {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "encryption-dictionary", kind: "insert-encryption-dictionary", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::insert_object(&mut next, support::encryption_dictionary(self.version, self.revision));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        vec![PdfVtMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: self.version, revision: self.revision })]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
