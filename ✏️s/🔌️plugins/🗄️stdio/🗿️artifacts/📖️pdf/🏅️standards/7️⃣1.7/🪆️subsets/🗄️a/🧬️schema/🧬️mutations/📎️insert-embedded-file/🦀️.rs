//! 📎️ Authoritative PDF/A mutation for inserting an attached file specification.

use crate::standards::v1_7::subsets::a::schema::mutations::remove_embedded_file::RemoveEmbeddedFile;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertEmbeddedFile {
    pub file_name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for InsertEmbeddedFile
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<InsertEmbeddedFile> + From<crate::standards::v1_7::subsets::a::schema::mutations::remove_embedded_file::RemoveEmbeddedFile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "embedded-file", kind: "insert-embedded-file", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::insert_file_spec_rows(base, &self.file_name, &self.placements)))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Op::from(RemoveEmbeddedFile { file_name: self.file_name.clone() })]
    
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

//#endregion 🧪️Tests
