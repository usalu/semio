//! 🔑️ `set-chunk-keyframe` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetChunkKeyframe {
    pub stream_index: usize,
    pub index: usize,
    pub keyframe: bool,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetChunkKeyframe {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "chunk-keyframe", kind: "set-chunk-keyframe", record: "SetChunkKeyframe" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { stream_index, index, keyframe } = self;
        protocol::MutationOutcome::new({ chunk_diff_for(*stream_index, IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: *index, diff: AviChunkDiff { data: None, keyframe: Some(*keyframe) } }], added: vec![] }) })
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { stream_index, index, .. } = self;
        Ok({
            match base.streams.get(*stream_index).and_then(|s| s.chunks.get(*index)) {
                Some(chunk) => vec![AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index: *stream_index, index: *index, keyframe: chunk.keyframe })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set chunk keyframe", "Keyframe-Kennung des Chunks setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
