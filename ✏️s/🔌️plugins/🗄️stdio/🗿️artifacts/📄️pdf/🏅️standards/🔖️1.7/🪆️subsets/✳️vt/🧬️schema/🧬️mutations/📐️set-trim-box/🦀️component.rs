//! 📐️ Authoritative PDF/VT mutation for set trim box.

use super::remove_trim_box::RemoveTrimBox;
use super::PdfVtMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTrimBox {
    pub page_index: usize,
    pub trim_box: [f64; 4],
}

impl MutationKind<PdfSnapshot, PdfVtMutation> for SetTrimBox {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "trim-box", kind: "set-trim-box", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(page) = support::page_objects(&next).get(self.page_index).copied() {
            support::set_entry(&mut next, page, "TrimBox", support::box_object(self.trim_box));
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        match support::page_objects(base).get(self.page_index).copied().and_then(|page| support::page_box(base, page, "TrimBox")) {
            Some(trim_box) => vec![PdfVtMutation::SetTrimBox(SetTrimBox { page_index: self.page_index, trim_box })],
            None => vec![PdfVtMutation::RemoveTrimBox(RemoveTrimBox { page_index: self.page_index })],
        }
    }

    fn label(&self) -> String {
        format!("Set PDF/VT trim box on page {}", self.page_index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.page_index.to_string(), "TrimBox".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
        let mut base = PdfSnapshot::default();
        let page = support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Page".to_string()))]));
        let mutation = SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0] };
        let outcome = <SetTrimBox as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(support::page_box(&next, page, "TrimBox"), Some([1.0, 2.0, 300.0, 400.0]));
        assert_eq!(<SetTrimBox as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::RemoveTrimBox(RemoveTrimBox { page_index: 0 })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
