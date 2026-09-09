//! 🔤️ Authoritative PDF/A mutation for attaching a font program to a font descriptor.

use super::remove_font_file::RemoveFontFile;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{
    conformance_support as support,
    diff::PdfDiff,
    snapshot::{ObjRef, PdfObject, PdfSnapshot},
};
use protocol::command::DiffAlgebra;
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

impl MutationKind<PdfSnapshot, PdfAMutation> for EmbedFontFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "font-file", kind: "embed-font-file", record: "Inserted" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::font_descriptors(&next).get(self.descriptor_ordinal).copied() {
            support::set_entry(&mut next, id, &self.key, PdfObject::Ref(self.program));
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfAMutation> {
        let Some(id) = support::font_descriptors(base).get(self.descriptor_ordinal).copied() else { return Vec::new() };
        match support::font_program(base, id) {
            Some((key, program)) => vec![PdfAMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program })],
            None => vec![PdfAMutation::RemoveFontFile(RemoveFontFile { descriptor_ordinal: self.descriptor_ordinal })],
        }
    }

    fn label(&self) -> String {
        format!("Embed {} on font descriptor {}", self.key, self.descriptor_ordinal)
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
