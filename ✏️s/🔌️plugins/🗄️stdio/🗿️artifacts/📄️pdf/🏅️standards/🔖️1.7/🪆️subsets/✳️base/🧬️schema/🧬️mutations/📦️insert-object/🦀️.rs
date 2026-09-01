//! 📦️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-object`.

use super::remove_object::RemoveObject;
use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{ObjRef, PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertObject {
    pub id: ObjRef,
    pub value: PdfObject,
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "object", kind: "insert-object", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_insert_object(self.id, base.objects.len(), self.value.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if base.objects.iter().any(|object| object.id == self.id) { Vec::new() } else { vec![PdfMutation::RemoveObject(RemoveObject { id: self.id })] }
    }

    fn label(&self) -> String {
        format!("Insert object {} {}", self.id.num, self.id.gen)
    }

    fn target(&self) -> Vec<String> {
        vec![format!("{} {}", self.id.num, self.id.gen)]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<InsertObject as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "insert-object");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
