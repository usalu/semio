//! 🧹️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-object`.

use super::insert_object::InsertObject;
use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{ObjRef, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveObject {
    pub id: ObjRef,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "object", kind: "remove-object", record: "Remove" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_remove_object(self.id))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.objects.iter().find(|object| object.id == self.id).map(|object| PdfMutation::InsertObject(InsertObject { id: self.id, value: object.value.clone() })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Remove object {} {}", self.id.num, self.id.gen)
    }

    fn target(&self) -> Vec<String> {
        vec![format!("{} {}", self.id.num, self.id.gen)]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
