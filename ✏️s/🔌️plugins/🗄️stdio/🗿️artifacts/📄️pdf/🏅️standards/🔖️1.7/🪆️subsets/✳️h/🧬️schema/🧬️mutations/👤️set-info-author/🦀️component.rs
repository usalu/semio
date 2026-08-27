//! 👤️ Authoritative PDF/H mutation for setting the document author conformance axis.

use super::PdfHMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInfoAuthor {
    pub author: String,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for SetInfoAuthor {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info-author", kind: "set-info-author", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        next.info.author = Some(self.author.clone());
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfHMutation> {
        vec![PdfHMutation::SetInfoAuthor(SetInfoAuthor { author: base.info.author.clone().unwrap_or_default() })]
    }

    fn label(&self) -> String {
        format!("Set PDF/H author \"{}\"", self.author)
    }

    fn target(&self) -> Vec<String> {
        vec!["Info.Author".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn sets_and_can_restore_the_document_author() {
        let mut base = PdfSnapshot::default();
        base.info.author = Some("before".to_string());
        let mutation = SetInfoAuthor { author: "after".to_string() };
        let outcome = <SetInfoAuthor as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(next.info.author.as_deref(), Some("after"));
        assert_eq!(<SetInfoAuthor as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base), vec![PdfHMutation::SetInfoAuthor(SetInfoAuthor { author: "before".to_string() })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
