//! 🧳️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-trailer-entry`.

use super::remove_trailer_entry::RemoveTrailerEntry;
use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::{PdfObject, PdfSnapshot},
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTrailerEntry {
    pub key: String,
    pub value: PdfObject,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetTrailerEntry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "trailer-entry", kind: "set-trailer-entry", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(diff::diff_set_trailer_entry(base, &self.key, self.value.clone(), self.index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match base.trailer.iter().find(|entry| entry.key == self.key) {
            Some(entry) => vec![PdfMutation::SetTrailerEntry(SetTrailerEntry { key: self.key.clone(), value: entry.value.clone(), index: None })],
            None => vec![PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key: self.key.clone() })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set trailer entry {}", self.key), &format!("Trailer-Eintrag {} setzen", self.key))
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

