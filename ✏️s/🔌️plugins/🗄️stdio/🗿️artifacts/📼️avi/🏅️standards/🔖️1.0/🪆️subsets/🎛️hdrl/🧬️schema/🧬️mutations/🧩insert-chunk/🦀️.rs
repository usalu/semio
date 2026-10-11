//! 🧩️ `insert-chunk` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertChunk {
    pub stream_index: usize,
    pub index: usize,
    pub chunk: AviChunk,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for InsertChunk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "chunk", kind: "insert-chunk", record: "InsertChunk" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { stream_index, index, chunk } = self;
        protocol::MutationOutcome::new(chunk_diff_for(*stream_index, IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: chunk.clone() }] }))
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { stream_index, index, .. } = self;
        Ok({ vec![AviMutation::RemoveChunk(remove_chunk::RemoveChunk { stream_index: *stream_index, index: *index })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert chunk", "Chunk einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
