//! 📎️ `set-other-chunks` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetOtherChunks {
    pub chunks: Vec<RiffChunk>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub chunk_order: Option<Vec<WavChunkRef>>,
}

impl SetOtherChunks {
    /// 🔺️ The sparse diff this payload asks of `base`: the new list, and the stated chunk order — else the base order reconciled with the list — where they differ.
    fn wanted(&self, base: &WavSnapshot) -> WavDiff {
        let mut diff = diff_set_other_chunks(base, self.chunks.clone());
        if let Some(order) = &self.chunk_order {
            diff.chunk_order = Some(order.clone());
        }
        sparse_against(base, diff)
    }
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for SetOtherChunks {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "other-chunks", kind: "set-other-chunks", record: "SetOtherChunks" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set other chunks", "Sonstige Chunks setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<WavSnapshot, WavMutation> for SetOtherChunks {
    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as protocol::ColdMutation<WavSnapshot>>::ReferenceDiff> {
        match validate_wav_serialization(&WavSnapshot { other_chunks: self.chunks.clone(), ..base.clone() }) {
            Ok(()) => protocol::MutationOutcome::new(self.wanted(base)),
            Err(issue) => protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target),
        }
    }
    fn inverse(&self, base: &WavSnapshot) -> Result<Vec<WavMutation>, semio_framework_value::ValueError> {
        Ok((!protocol::DiffAlgebra::is_empty(&self.wanted(base))).then(|| WavMutation::SetOtherChunks(SetOtherChunks { chunks: base.other_chunks.clone(), chunk_order: Some(base.chunk_order.clone()) })).into_iter().collect())
    }
}
//#endregion 🔖️Payload
