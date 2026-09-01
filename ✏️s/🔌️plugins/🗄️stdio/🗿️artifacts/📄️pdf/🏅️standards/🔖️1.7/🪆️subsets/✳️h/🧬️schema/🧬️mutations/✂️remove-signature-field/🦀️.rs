//! ✂️ Authoritative PDF/H mutation for removing a matching signature field.

use super::insert_signature_field::InsertSignatureField;
use super::PdfHMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn removes_and_can_restore_the_named_signature_field() {
        let mut base = PdfSnapshot::default();
        support::insert_signature_field(&mut base, "Signature1");
        let mutation = RemoveSignatureField { name: "Signature1".to_string() };
        let outcome = <RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::signature_field_named(&next, &mutation.name).is_none());
        assert_eq!(<RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base).len(), 1);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
