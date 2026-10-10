//! 🧽️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-trailer-entry`.

use super::set_trailer_entry::SetTrailerEntry;
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
pub struct RemoveTrailerEntry {
    pub key: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveTrailerEntry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "trailer-entry", kind: "remove-trailer-entry", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(diff::diff_remove_trailer_entry(base, &self.key)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        base.trailer.iter().position(|entry| entry.key == self.key).map(|position| PdfMutation::SetTrailerEntry(SetTrailerEntry { key: self.key.clone(), value: base.trailer[position].value.clone(), index: Some(position) })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove trailer entry {}", self.key), &format!("Trailer-Eintrag {} entfernen", self.key))
    }

    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

