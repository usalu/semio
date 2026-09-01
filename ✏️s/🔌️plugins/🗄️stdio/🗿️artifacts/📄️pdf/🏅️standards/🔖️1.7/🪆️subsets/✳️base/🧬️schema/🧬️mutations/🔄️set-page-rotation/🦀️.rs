//! 🔄️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-rotation`.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageRotation {
    pub index: usize,
    pub rotation: u16,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageRotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-rotation", kind: "set-page-rotation", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_rotation(self.index, self.rotation as i32))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.pages.get(self.index).map(|page| PdfMutation::SetPageRotation(SetPageRotation { index: self.index, rotation: page.rotate.rem_euclid(360) as u16 })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Set page {} rotation to {}", self.index, self.rotation)
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<SetPageRotation as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-page-rotation");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
