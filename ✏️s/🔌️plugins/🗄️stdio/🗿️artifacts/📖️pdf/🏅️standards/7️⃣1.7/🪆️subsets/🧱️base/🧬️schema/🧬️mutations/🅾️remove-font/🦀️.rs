//! 🅾️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-font`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveFont {
    pub id: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveFont {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "font", kind: "remove-font", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_font(base, &self.id))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.fonts.iter().find(|item| item.id == self.id).map(|item| PdfMutation::SetFont(super::set_font::SetFont { font: item.clone() })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove font {}", self.id), &format!("Schrift {} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

