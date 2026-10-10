//! 📝️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-annotation`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAnnotation {
    pub index: usize,
    pub at: usize,
    pub annotation: PdfAnnotation,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetAnnotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "annotation", kind: "set-annotation", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_annotation(self.index, self.at, self.annotation.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.pages.get(self.index).and_then(|page| page.annotations.get(self.at)).map(|annotation| PdfMutation::SetAnnotation(SetAnnotation { index: self.index, at: self.at, annotation: annotation.clone() })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set annotation {} on page {}", self.at, self.index), &format!("Anmerkung {} auf Seite {} setzen", self.at, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string(), self.at.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

