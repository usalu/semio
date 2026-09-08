//! 🏷️ Authoritative PDF/H mutation for setting the document title conformance axis.

use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInfoTitle {
    pub title: String,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for SetInfoTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info-title", kind: "set-info-title", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        next.info.title = Some(self.title.clone());
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfHMutation> {
        vec![PdfHMutation::SetInfoTitle(SetInfoTitle { title: base.info.title.clone().unwrap_or_default() })]
    }

    fn label(&self) -> String {
        format!("Set PDF/H title \"{}\"", self.title)
    }

    fn target(&self) -> Vec<String> {
        vec!["Info.Title".to_string()]
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
