//! 🧺️ Authoritative PDF/E mutation for detaching a font program from a font descriptor.

use super::embed_font_file::EmbedFontFile;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveFontFile {
    pub descriptor_ordinal: usize,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for RemoveFontFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "font-file", kind: "remove-font-file", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::font_descriptors(&next).get(self.descriptor_ordinal).copied() {
            if let Some((key, _)) = support::font_program(&next, id) {
                support::remove_entry(&mut next, id, &key);
            }
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfEMutation> {
        support::font_descriptors(base)
            .get(self.descriptor_ordinal)
            .copied()
            .and_then(|id| support::font_program(base, id))
            .map(|(key, program)| PdfEMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        format!("Remove font program from descriptor {}", self.descriptor_ordinal)
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
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
