//! 🗑️ Authoritative PDF/A mutation for removing a named attached file specification.

use super::insert_embedded_file::InsertEmbeddedFile;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveEmbeddedFile {
    pub file_name: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for RemoveEmbeddedFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "embedded-file", kind: "remove-embedded-file", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::file_spec_named(base, &self.file_name).map_or_else(PdfDiff::default, |id| support::remove_file_spec_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfAMutation>, semio_framework_value::ValueError> {
        Ok({
            support::file_spec_named(base, &self.file_name).map(|id| PdfAMutation::InsertEmbeddedFile(InsertEmbeddedFile { file_name: self.file_name.clone(), placements: support::placements_of(base, &support::file_spec_creation_ids(base, id)) })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove embedded file \"{}\"", self.file_name), &format!("Eingebettete Datei \"{}\" entfernen", self.file_name))
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
