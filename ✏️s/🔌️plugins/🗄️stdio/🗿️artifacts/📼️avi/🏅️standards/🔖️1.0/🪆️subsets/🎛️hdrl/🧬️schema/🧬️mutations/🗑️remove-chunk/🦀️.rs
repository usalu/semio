//! 🗑️ `remove-chunk` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveChunk {
    pub stream_index: usize,
    pub index: usize,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for RemoveChunk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "chunk", kind: "remove-chunk", record: "RemoveChunk" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { stream_index, index } = self;
        protocol::MutationOutcome::new(chunk_diff_for(*stream_index, IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }))
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { stream_index, index } = self;
        Ok({
            match base.streams.get(*stream_index).and_then(|s| s.chunks.get(*index)) {
                Some(chunk) => vec![AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index: *stream_index, index: *index, chunk: chunk.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove chunk", "Chunk entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
