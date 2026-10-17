//! 🗃️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-embedded-file`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveEmbeddedFile {
    pub id: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveEmbeddedFile
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveEmbeddedFile> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_embedded_file::SetEmbeddedFile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "embedded-file", kind: "remove-embedded-file", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_embedded_file(base, &self.id))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.embedded_files.iter().position(|item| item.id == self.id).map(|index| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::set_embedded_file::SetEmbeddedFile { file: base.embedded_files[index].clone(), index: Some(index) })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove embedded-file {}", self.id), &format!("Eingebettete Datei {} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

