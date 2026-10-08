//! ✒️ Authoritative PDF/H mutation for inserting a named signature field.

use super::remove_signature_field::RemoveSignatureField;
use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertSignatureField {
    pub name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub field_index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for InsertSignatureField {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "signature-field", kind: "insert-signature-field", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::insert_signature_field_rows(base, &self.name, &self.placements, self.field_index, self.entry_index)))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfHMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfHMutation::RemoveSignatureField(RemoveSignatureField { name: self.name.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert signature field \"{}\"", self.name), &format!("Signaturfeld \"{}\" einfügen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
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
