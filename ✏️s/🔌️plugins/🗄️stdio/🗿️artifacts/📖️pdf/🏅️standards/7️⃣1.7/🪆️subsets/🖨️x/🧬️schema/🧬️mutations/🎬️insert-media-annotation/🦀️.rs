//! 🎬️ Authoritative PDF/X mutation for inserting a Movie or Sound annotation.

use super::remove_media_annotation::RemoveMediaAnnotation;
use super::PdfXMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertMediaAnnotation {
    pub subtype: String,
    pub title: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl MutationKind<PdfSnapshot, PdfXMutation> for InsertMediaAnnotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "media-annotation", kind: "insert-media-annotation", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let (_, rows) = support::insert_object_rows(base, support::media_annotation_object(&self.subtype, &self.title), &self.placements);
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfXMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfXMutation::RemoveMediaAnnotation(RemoveMediaAnnotation { subtype: self.subtype.clone(), title: self.title.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert {} media annotation", self.subtype), &format!("{} Medienanmerkung einfügen", self.subtype))
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
