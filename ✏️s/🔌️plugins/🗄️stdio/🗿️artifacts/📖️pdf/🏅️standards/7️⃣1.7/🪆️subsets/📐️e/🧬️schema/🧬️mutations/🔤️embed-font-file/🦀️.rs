//! 🔤️ Authoritative PDF/E mutation for attaching a font program to a font descriptor.

use super::remove_font_file::RemoveFontFile;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{ObjRef, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct EmbedFontFile {
    pub descriptor_ordinal: usize,
    pub key: String,
    pub program: ObjRef,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for EmbedFontFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "font-file", kind: "embed-font-file", record: "Embed" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::font_descriptors(base).get(self.descriptor_ordinal).copied().map_or_else(PdfDiff::default, |id| support::embed_font_file_rows(base, id, &self.key, self.program));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfEMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let Some(id) = support::font_descriptors(base).get(self.descriptor_ordinal).copied() else { return Vec::new() };
        match support::font_program(base, id) {
            Some((key, program)) => vec![PdfEMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program })],
            None => vec![PdfEMutation::RemoveFontFile(RemoveFontFile { descriptor_ordinal: self.descriptor_ordinal })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Embed {} on font descriptor {}", self.key, self.descriptor_ordinal), &format!("{} in Schriftdeskriptor {} einbetten", self.key, self.descriptor_ordinal))
    }

    fn target(&self) -> Vec<String> {
        vec![self.descriptor_ordinal.to_string(), self.key.clone()]
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
