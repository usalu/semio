//! 🎬️ Authoritative PDF/E mutation for inserting a Movie or Sound annotation.

use super::remove_media_annotation::RemoveMediaAnnotation;
use super::PdfEMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn inserts_the_requested_media_annotation() {
        let base = PdfSnapshot::default();
        let mutation = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() };
        let outcome = <InsertMediaAnnotation as MutationKind<PdfSnapshot, PdfEMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_some());
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
