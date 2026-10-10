//! 🔄️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-rotation`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageRotation {
    pub index: usize,
    pub rotation: u16,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageRotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-rotation", kind: "set-page-rotation", record: "Set" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_rotation(self.index, self.rotation as i32))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        base.pages.get(self.index).map(|page| PdfMutation::SetPageRotation(SetPageRotation { index: self.index, rotation: page.rotate.rem_euclid(360) as u16 })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set page {} rotation to {}", self.index, self.rotation), &format!("Drehung von Seite {} auf {} setzen", self.index, self.rotation))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

