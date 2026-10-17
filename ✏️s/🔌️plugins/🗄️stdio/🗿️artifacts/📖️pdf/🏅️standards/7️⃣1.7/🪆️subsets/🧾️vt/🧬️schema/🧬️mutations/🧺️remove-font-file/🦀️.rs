//! 🧺️ Authoritative PDF/VT mutation for detaching a font program from a font descriptor.

use crate::standards::v1_7::subsets::vt::schema::mutations::embed_font_file::EmbedFontFile;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveFontFile {
    pub descriptor_ordinal: usize,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveFontFile
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveFontFile> + From<crate::standards::v1_7::subsets::vt::schema::mutations::embed_font_file::EmbedFontFile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "font-file", kind: "remove-font-file", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::font_descriptors(base).get(self.descriptor_ordinal).copied().and_then(|id| support::font_program(base, id).map(|(key, _)| support::remove_entry_rows(base, id, &key))).unwrap_or_default();
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            let Some(id) = support::font_descriptors(base).get(self.descriptor_ordinal).copied() else { return Ok(Vec::new()) };
            support::font_program(base, id).map(|(key, program)| {
                let entry_index = support::entry_position(base, id, &key);
                Op::from(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program, entry_index })
            }).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove font program from descriptor {}", self.descriptor_ordinal), &format!("Schriftprogramm aus Deskriptor {} entfernen", self.descriptor_ordinal))
    }

    fn target(&self) -> Vec<String> {
        vec![self.descriptor_ordinal.to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
