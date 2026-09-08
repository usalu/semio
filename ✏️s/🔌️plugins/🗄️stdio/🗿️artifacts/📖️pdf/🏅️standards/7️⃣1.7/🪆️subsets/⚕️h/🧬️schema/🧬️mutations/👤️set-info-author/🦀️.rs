//! 👤️ Authoritative PDF/H mutation for setting the document author conformance axis.

use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
