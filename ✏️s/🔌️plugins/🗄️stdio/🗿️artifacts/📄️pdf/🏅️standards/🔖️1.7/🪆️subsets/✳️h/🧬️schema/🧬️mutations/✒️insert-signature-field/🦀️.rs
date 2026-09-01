//! ✒️ Authoritative PDF/H mutation for inserting a named signature field.

use super::remove_signature_field::RemoveSignatureField;
use super::PdfHMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertSignatureField {
    pub name: String,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for InsertSignatureField {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "signature-field", kind: "insert-signature-field", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::insert_signature_field(&mut next, &self.name);
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfHMutation> {
        vec![PdfHMutation::RemoveSignatureField(RemoveSignatureField { name: self.name.clone() })]
    }

    fn label(&self) -> String {
        format!("Insert signature field \"{}\"", self.name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn inserts_the_named_signature_field() {
        let base = PdfSnapshot::default();
        let mutation = InsertSignatureField { name: "Signature1".to_string() };
        let outcome = <InsertSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::signature_field_named(&next, &mutation.name).is_some());
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
