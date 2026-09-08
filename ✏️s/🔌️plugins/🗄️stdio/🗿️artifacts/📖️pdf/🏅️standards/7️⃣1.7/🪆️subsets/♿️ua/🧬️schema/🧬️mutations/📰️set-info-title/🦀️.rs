//! 🏷️ Authoritative PDF/UA mutation for setting the document title conformance axis.

use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInfoTitle {
    pub title: String,
}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetInfoTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info-title", kind: "set-info-title", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        next.info.title = Some(self.title.clone());
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        vec![PdfUaMutation::SetInfoTitle(SetInfoTitle { title: base.info.title.clone().unwrap_or_default() })]
    }

    fn label(&self) -> String {
        format!("Set PDF/UA title \"{}\"", self.title)
    }

    fn target(&self) -> Vec<String> {
        vec!["Info.Title".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn sets_and_can_restore_the_document_title() {
        let mut base = PdfSnapshot::default();
        base.info.title = Some("before".to_string());
        let mutation = SetInfoTitle { title: "after".to_string() };
        let outcome = <SetInfoTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(next.info.title.as_deref(), Some("after"));
        assert_eq!(<SetInfoTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetInfoTitle(SetInfoTitle { title: "before".to_string() })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
