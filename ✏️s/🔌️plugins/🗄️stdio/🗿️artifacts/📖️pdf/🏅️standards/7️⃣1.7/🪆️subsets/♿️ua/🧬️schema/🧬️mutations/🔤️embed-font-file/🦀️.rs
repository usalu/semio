//! 🔤️ Authoritative PDF/UA mutation for attaching a font program to a font descriptor.

use crate::standards::v1_7::subsets::ua::schema::mutations::remove_font_file::RemoveFontFile;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct EmbedFontFile {
    pub descriptor_ordinal: usize,
    pub key: String,
    pub program: ObjRef,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for EmbedFontFile
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<EmbedFontFile> + From<crate::standards::v1_7::subsets::ua::schema::mutations::remove_font_file::RemoveFontFile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "font-file", kind: "embed-font-file", record: "Embed" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Embed {} on font descriptor {}", self.key, self.descriptor_ordinal), &format!("{} in Schriftdeskriptor {} einbetten", self.key, self.descriptor_ordinal))
    }

    fn target(&self) -> Vec<String> {
        vec![self.descriptor_ordinal.to_string(), self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for EmbedFontFile
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<EmbedFontFile> + From<crate::standards::v1_7::subsets::ua::schema::mutations::remove_font_file::RemoveFontFile>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::font_descriptors(base).get(self.descriptor_ordinal).copied().map_or_else(PdfDiff::default, |id| support::embed_font_file_rows(base, id, &self.key, self.program, self.entry_index));
        MutationOutcome::new(diff::graph_edit(rows))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            let Some(id) = support::font_descriptors(base).get(self.descriptor_ordinal).copied() else { return Ok(Vec::new()) };
            match support::font_program(base, id) {
                Some((key, program)) => vec![Op::from(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program, entry_index: None })],
                None => vec![Op::from(RemoveFontFile { descriptor_ordinal: self.descriptor_ordinal })],
            }
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
