//! ✂️ Authoritative PDF/H mutation for removing a matching signature field.

use super::insert_signature_field::InsertSignatureField;
use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
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
        let mut next = base.clone();
        support::remove_signature_field(&mut next, &self.name);
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfHMutation> {
        support::signature_field_named(base, &self.name)
            .map(|_| PdfHMutation::InsertSignatureField(InsertSignatureField { name: self.name.clone() }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        format!("Remove signature field \"{}\"", self.name)
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
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
