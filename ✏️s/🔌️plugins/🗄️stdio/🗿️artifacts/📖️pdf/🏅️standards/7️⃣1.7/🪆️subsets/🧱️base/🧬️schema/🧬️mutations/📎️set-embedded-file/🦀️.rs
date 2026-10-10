//! 📎️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-embedded-file`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetEmbeddedFile {
    pub file: PdfEmbeddedFile,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetEmbeddedFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "embedded-file", kind: "set-embedded-file", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_embedded_file(base, self.file.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.embedded_files.iter().find(|item| item.id == self.file.id) { Some(previous) => vec![PdfMutation::SetEmbeddedFile(SetEmbeddedFile { file: previous.clone(), index: None })], None => vec![PdfMutation::RemoveEmbeddedFile(super::remove_embedded_file::RemoveEmbeddedFile { id: self.file.id.clone() })] }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set embedded-file {}", self.file.id), &format!("Eingebettete Datei {} setzen", self.file.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.file.id.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

