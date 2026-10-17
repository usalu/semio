//! 🧽️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-trailer-entry`.

use crate::standards::v1_7::subsets::base::schema::mutations::set_trailer_entry::SetTrailerEntry;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveTrailerEntry {
    pub key: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveTrailerEntry
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveTrailerEntry> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_trailer_entry::SetTrailerEntry>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "trailer-entry", kind: "remove-trailer-entry", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(diff::diff_remove_trailer_entry(base, &self.key)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        base.trailer.iter().position(|entry| entry.key == self.key).map(|position| Op::from(SetTrailerEntry { key: self.key.clone(), value: base.trailer[position].value.clone(), index: Some(position) })).into_iter().collect()
    
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

//#endregion 🧪️Tests

