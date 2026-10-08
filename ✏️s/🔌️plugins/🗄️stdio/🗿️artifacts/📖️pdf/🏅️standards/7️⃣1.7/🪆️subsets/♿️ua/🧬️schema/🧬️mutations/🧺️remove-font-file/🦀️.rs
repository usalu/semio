//! 🧺️ Authoritative PDF/UA mutation for detaching a font program from a font descriptor.

use super::embed_font_file::EmbedFontFile;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveFontFile {
    pub descriptor_ordinal: usize,
}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveFontFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "font-file", kind: "remove-font-file", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::font_descriptors(base).get(self.descriptor_ordinal).copied().and_then(|id| support::font_program(base, id).map(|(key, _)| support::remove_entry_rows(base, id, &key))).unwrap_or_default();
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        support::font_descriptors(base)
            .get(self.descriptor_ordinal)
            .copied()
            .and_then(|id| support::font_program(base, id))
            .map(|(key, program)| PdfUaMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program }))
            .into_iter()
            .collect()
    
    })())
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
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
