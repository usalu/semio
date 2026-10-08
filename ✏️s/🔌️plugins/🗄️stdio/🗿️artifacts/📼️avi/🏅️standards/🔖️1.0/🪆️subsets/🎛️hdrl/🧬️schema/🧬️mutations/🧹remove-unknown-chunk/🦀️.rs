//! 🧹️ `remove-unknown-chunk` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveUnknownChunk {
    pub index: usize,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for RemoveUnknownChunk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "unknown-chunk", kind: "remove-unknown-chunk", record: "RemoveUnknownChunk" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(AviDiff { unknown_chunks: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }), ..AviDiff::default() })
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.unknown_chunks.get(*index) {
                Some(item) => vec![AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index: *index, item: item.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove unknown chunk", "Unbekannten Chunk entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
