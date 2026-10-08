//! ✂️ Authoritative PDF/H mutation for removing a matching signature field.

use super::insert_signature_field::InsertSignatureField;
use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveSignatureField {
    pub name: String,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for RemoveSignatureField {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "signature-field", kind: "remove-signature-field", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_signature_field_rows(base, &self.name)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfHMutation>, semio_framework_value::ValueError> {
        Ok({
            support::signature_field_named(base, &self.name).map(|field| {
                let fields = support::signature_fields(base);
                PdfHMutation::InsertSignatureField(InsertSignatureField { name: self.name.clone(), placements: support::placements_of(base, &[field]), field_index: fields.iter().position(|candidate| *candidate == field), entry_index: (fields.len() == 1).then(|| support::catalog_entry_position(base, "AcroForm")).flatten() })
            }).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove signature field \"{}\"", self.name), &format!("Signaturfeld \"{}\" entfernen", self.name))
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
