//! 🔤️ Authoritative PDF/X mutation for attaching a font program to a font descriptor.

use super::remove_font_file::RemoveFontFile;
use super::PdfXMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{ObjRef, PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct EmbedFontFile {
    pub descriptor_ordinal: usize,
    pub key: String,
    pub program: ObjRef,
}

impl MutationKind<PdfSnapshot, PdfXMutation> for EmbedFontFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "font-file", kind: "embed-font-file", record: "Embed" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::font_descriptors(&next).get(self.descriptor_ordinal).copied() {
            support::set_entry(&mut next, id, &self.key, PdfObject::Ref(self.program));
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfXMutation> {
        let Some(id) = support::font_descriptors(base).get(self.descriptor_ordinal).copied() else { return Vec::new() };
        match support::font_program(base, id) {
            Some((key, program)) => vec![PdfXMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program })],
            None => vec![PdfXMutation::RemoveFontFile(RemoveFontFile { descriptor_ordinal: self.descriptor_ordinal })],
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn attaches_the_program_to_the_selected_descriptor() {
        let mut base = PdfSnapshot::default();
        let program = support::insert_object(&mut base, PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() });
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string()))]));
        let mutation = EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program };
        let outcome = <EmbedFontFile as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        let descriptor = support::font_descriptors(&next)[0];
        assert_eq!(support::font_program(&next, descriptor), Some(("FontFile2".to_string(), program)));
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
