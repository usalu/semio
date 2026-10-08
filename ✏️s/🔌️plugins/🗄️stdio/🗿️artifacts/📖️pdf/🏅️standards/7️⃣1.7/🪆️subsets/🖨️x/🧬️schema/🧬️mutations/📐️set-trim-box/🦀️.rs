//! 📐️ Authoritative PDF/X mutation for set trim box.

use super::remove_trim_box::RemoveTrimBox;
use super::PdfXMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTrimBox {
    pub page_index: usize,
    pub trim_box: [f64; 4],
}

impl MutationKind<PdfSnapshot, PdfXMutation> for SetTrimBox {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "trim-box", kind: "set-trim-box", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::page_objects(base).get(self.page_index).copied().map_or_else(PdfDiff::default, |page| support::set_entry_rows(base, page, "TrimBox", support::box_object(self.trim_box)));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfXMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match support::page_objects(base).get(self.page_index).copied().and_then(|page| support::page_box(base, page, "TrimBox")) {
            Some(trim_box) => vec![PdfXMutation::SetTrimBox(SetTrimBox { page_index: self.page_index, trim_box })],
            None => vec![PdfXMutation::RemoveTrimBox(RemoveTrimBox { page_index: self.page_index })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/X trim box on page {}", self.page_index), &format!("PDF/X-TrimBox auf Seite {} setzen", self.page_index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.page_index.to_string(), "TrimBox".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
