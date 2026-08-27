//! ⏹️ Authoritative PDF/VT mutation for removing a matching Movie or Sound annotation.

use super::insert_media_annotation::InsertMediaAnnotation;
use super::PdfVtMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveMediaAnnotation {
    pub subtype: String,
    pub title: String,
}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveMediaAnnotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "media-annotation", kind: "remove-media-annotation", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::media_annotation(&next, &self.subtype, &self.title) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        support::media_annotation(base, &self.subtype, &self.title)
            .map(|_| PdfVtMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: self.subtype.clone(), title: self.title.clone() }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        format!("Remove {} media annotation", self.subtype)
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
    fn removes_only_the_matching_media_annotation() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::media_annotation_object("Sound", "narration"));
        let mutation = RemoveMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() };
        let outcome = <RemoveMediaAnnotation as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_none());
        assert_eq!(<RemoveMediaAnnotation as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).len(), 1);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
