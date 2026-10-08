//! ⏹️ Authoritative PDF/E mutation for removing a matching Movie or Sound annotation.

use super::insert_media_annotation::InsertMediaAnnotation;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveMediaAnnotation {
    pub subtype: String,
    pub title: String,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for RemoveMediaAnnotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "media-annotation", kind: "remove-media-annotation", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::media_annotation(base, &self.subtype, &self.title).map_or_else(PdfDiff::default, |id| support::remove_object_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfEMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        support::media_annotation(base, &self.subtype, &self.title).map(|_| PdfEMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: self.subtype.clone(), title: self.title.clone() })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove {} media annotation", self.subtype), &format!("{} Medienanmerkung entfernen", self.subtype))
    }

    fn target(&self) -> Vec<String> {
        vec![self.subtype.clone(), self.title.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
