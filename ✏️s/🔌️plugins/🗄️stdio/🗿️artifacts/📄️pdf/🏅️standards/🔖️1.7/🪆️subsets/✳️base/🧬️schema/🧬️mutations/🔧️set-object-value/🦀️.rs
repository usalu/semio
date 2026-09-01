//! 🔧️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-object-value`.

use super::remove_object::RemoveObject;
use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{ObjRef, PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetObjectValue {
    pub id: ObjRef,
    pub value: PdfObject,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetObjectValue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "object-value", kind: "set-object-value", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_object_value(base, self.id, self.value.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        match base.objects.iter().find(|object| object.id == self.id) { Some(object) => vec![PdfMutation::SetObjectValue(SetObjectValue { id: self.id, value: object.value.clone() })], None => vec![PdfMutation::RemoveObject(RemoveObject { id: self.id })] }
    }

    fn label(&self) -> String {
        format!("Set object {} {} value", self.id.num, self.id.gen)
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
        assert_eq!(<SetObjectValue as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-object-value");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
