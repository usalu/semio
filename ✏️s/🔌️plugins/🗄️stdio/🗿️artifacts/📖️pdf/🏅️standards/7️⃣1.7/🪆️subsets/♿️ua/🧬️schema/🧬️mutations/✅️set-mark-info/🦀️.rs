//! 🏷️ Authoritative PDF/UA mutation for set mark info.

use super::remove_mark_info::RemoveMarkInfo;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{
    conformance_support as support,
    diff::PdfDiff,
    snapshot::{PdfObject, PdfSnapshot},
};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetMarkInfo {
    pub marked: bool,
}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetMarkInfo {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "mark-info", kind: "set-mark-info", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::set_catalog_entry(&mut next, "MarkInfo", support::single_entry_dict("Marked", PdfObject::Bool(self.marked)));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        match support::catalog_flag(base, "MarkInfo", "Marked") {
            Some(marked) => vec![PdfUaMutation::SetMarkInfo(SetMarkInfo { marked })],
            None => vec![PdfUaMutation::RemoveMarkInfo(RemoveMarkInfo {})],
        }
    }

    fn label(&self) -> String {
        format!("Set PDF/UA marked flag to {}", self.marked)
    }

    fn target(&self) -> Vec<String> {
        vec!["MarkInfo.Marked".to_string()]
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
