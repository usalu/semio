//! 🏷️ Authoritative PDF/UA mutation for set mark info.

use super::remove_mark_info::RemoveMarkInfo;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
        let mutation = SetMarkInfo { marked: true };
        let outcome = <SetMarkInfo as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(support::catalog_flag(&next, "MarkInfo", "Marked"), Some(true));
        assert_eq!(<SetMarkInfo as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::RemoveMarkInfo(RemoveMarkInfo {})]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
