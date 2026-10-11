//! 🔒️ Authoritative PDF/E mutation for inserting a Standard Security Handler dictionary.

use super::remove_encryption_dictionary::RemoveEncryptionDictionary;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertEncryptionDictionary {
    pub version: i64,
    pub revision: i64,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for InsertEncryptionDictionary {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "encryption-dictionary", kind: "insert-encryption-dictionary", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let (_, rows) = support::insert_object_rows(base, support::encryption_dictionary(self.version, self.revision), &self.placements);
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfEMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfEMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: self.version, revision: self.revision })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert encryption dictionary V{} R{}", self.version, self.revision), &format!("Verschlüsselungswörterbuch V{} R{} einfügen", self.version, self.revision))
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
//#endregion 🔖️Facets
