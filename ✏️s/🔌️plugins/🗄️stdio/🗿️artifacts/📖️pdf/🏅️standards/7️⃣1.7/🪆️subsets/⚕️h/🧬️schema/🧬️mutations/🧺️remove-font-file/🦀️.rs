//! 🧺️ Authoritative PDF/H mutation for detaching a font program from a font descriptor.

use super::embed_font_file::EmbedFontFile;
use super::PdfHMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveFontFile {
    pub descriptor_ordinal: usize,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for RemoveFontFile {
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

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfHMutation> {
        support::font_descriptors(base)
            .get(self.descriptor_ordinal)
            .copied()
            .and_then(|id| support::font_program(base, id))
            .map(|(key, program)| PdfHMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: self.descriptor_ordinal, key, program }))
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
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
    use protocol::MutationDiff;

    #[test]
    fn detaches_and_can_restore_the_font_program() {
        let mut base = PdfSnapshot::default();
        let program = support::insert_object(&mut base, PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() });
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string())), ("FontFile2", PdfObject::Ref(program))]));
        let mutation = RemoveFontFile { descriptor_ordinal: 0 };
        let outcome = <RemoveFontFile as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        let descriptor = support::font_descriptors(&next)[0];
        assert!(support::font_program(&next, descriptor).is_none());
        assert_eq!(<RemoveFontFile as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base).len(), 1);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
