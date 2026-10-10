//! 📎️ Authoritative PDF/A mutation for inserting an attached file specification.

use super::remove_embedded_file::RemoveEmbeddedFile;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertEmbeddedFile {
    pub file_name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for InsertEmbeddedFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "embedded-file", kind: "insert-embedded-file", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::insert_file_spec_rows(base, &self.file_name, &self.placements)))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfAMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfAMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { file_name: self.file_name.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert embedded file \"{}\"", self.file_name), &format!("Eingebettete Datei \"{}\" einfügen", self.file_name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.file_name.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
