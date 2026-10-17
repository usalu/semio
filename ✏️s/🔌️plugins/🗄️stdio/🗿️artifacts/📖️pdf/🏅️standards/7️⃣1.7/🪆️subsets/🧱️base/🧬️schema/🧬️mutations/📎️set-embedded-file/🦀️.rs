//! 📎️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-embedded-file`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetEmbeddedFile {
    pub file: PdfEmbeddedFile,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetEmbeddedFile
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetEmbeddedFile> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_embedded_file::RemoveEmbeddedFile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "embedded-file", kind: "set-embedded-file", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_embedded_file(base, self.file.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.embedded_files.iter().find(|item| item.id == self.file.id) { Some(previous) => vec![Op::from(SetEmbeddedFile { file: previous.clone(), index: None })], None => vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_embedded_file::RemoveEmbeddedFile { id: self.file.id.clone() })] }
    
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

//#endregion 🧪️Tests

