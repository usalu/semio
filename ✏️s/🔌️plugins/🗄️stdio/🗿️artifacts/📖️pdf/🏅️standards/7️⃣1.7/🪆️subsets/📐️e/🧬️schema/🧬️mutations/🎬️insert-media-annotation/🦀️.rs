//! 🎬️ Authoritative PDF/E mutation for inserting a Movie or Sound annotation.

use super::remove_media_annotation::RemoveMediaAnnotation;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertMediaAnnotation {
    pub subtype: String,
    pub title: String,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for InsertMediaAnnotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "media-annotation", kind: "insert-media-annotation", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::insert_object(&mut next, support::media_annotation_object(&self.subtype, &self.title));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfEMutation> {
        vec![PdfEMutation::RemoveMediaAnnotation(RemoveMediaAnnotation { subtype: self.subtype.clone(), title: self.title.clone() })]
    }

    fn label(&self) -> String {
        format!("Insert {} media annotation", self.subtype)
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
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
