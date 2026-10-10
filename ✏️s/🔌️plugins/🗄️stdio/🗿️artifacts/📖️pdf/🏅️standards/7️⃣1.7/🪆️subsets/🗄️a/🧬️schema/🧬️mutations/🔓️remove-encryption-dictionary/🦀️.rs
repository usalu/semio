//! 🔓️ Authoritative PDF/A mutation for removing a matching encryption dictionary.

use super::insert_encryption_dictionary::InsertEncryptionDictionary;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveEncryptionDictionary {
    pub version: i64,
    pub revision: i64,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for RemoveEncryptionDictionary {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "encryption-dictionary", kind: "remove-encryption-dictionary", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::encryption_dictionary_with(base, self.version, self.revision).map_or_else(PdfDiff::default, |id| support::remove_object_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfAMutation>, semio_framework_value::ValueError> {
        Ok({
            support::encryption_dictionary_with(base, self.version, self.revision).map(|id| PdfAMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: self.version, revision: self.revision, placements: support::placements_of(base, &[id]) })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove encryption dictionary V{} R{}", self.version, self.revision), &format!("Verschlüsselungswörterbuch V{} R{} entfernen", self.version, self.revision))
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
