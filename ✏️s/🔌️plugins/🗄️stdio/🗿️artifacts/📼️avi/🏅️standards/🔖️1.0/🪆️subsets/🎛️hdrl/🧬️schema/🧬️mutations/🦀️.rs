//! 🧬️ AviMutation — named-variant vocabulary (imperative verbs, gif/svg precedent). Every
//! variant's `diff()` is handcrafted; `inverse()` is handcrafted per variant, index-aware.

use crate::standards::v1_0::subsets::any::schema::diff::{AviChunkDiff, AviDiff, AviStreamDiff, IndexedAdded, IndexedDiff, IndexedModified};
use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "🧱add-unknown-chunk/🦀️.rs"]
pub mod add_unknown_chunk;
#[path = "🧩insert-chunk/🦀️.rs"]
pub mod insert_chunk;
#[path = "📥️insert-stream/🦀️.rs"]
pub mod insert_stream;
#[path = "🗑️remove-chunk/🦀️.rs"]
pub mod remove_chunk;
#[path = "📤️remove-stream/🦀️.rs"]
pub mod remove_stream;
#[path = "🧹remove-unknown-chunk/🦀️.rs"]
pub mod remove_unknown_chunk;
#[path = "🔑set-chunk-keyframe/🦀️.rs"]
pub mod set_chunk_keyframe;
#[path = "📇️set-idx1-present/🦀️.rs"]
pub mod set_idx1_present;
#[path = "🎬set-main-header/🦀️.rs"]
pub mod set_main_header;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🎨set-stream-format/🦀️.rs"]
pub mod set_stream_format;
#[path = "🎞️set-stream-header/🦀️.rs"]
pub mod set_stream_header;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none — and `no`
/// is not an approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = AviSnapshot, diff = AviDiff, schema = "AviMutation")]
pub enum AviMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetMainHeader(set_main_header::SetMainHeader),
    SetIdx1Present(set_idx1_present::SetIdx1Present),
    InsertStream(insert_stream::InsertStream),
    RemoveStream(remove_stream::RemoveStream),
    SetStreamHeader(set_stream_header::SetStreamHeader),
    SetStreamFormat(set_stream_format::SetStreamFormat),
    InsertChunk(insert_chunk::InsertChunk),
    RemoveChunk(remove_chunk::RemoveChunk),
    SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe),
    AddUnknownChunk(add_unknown_chunk::AddUnknownChunk),
    RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk),
}

/// 📇️ Kebab-case spelling of every `AviMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] =
    &["set-snapshot", "set-main-header", "set-idx1-present", "insert-stream", "remove-stream", "set-stream-header", "set-stream-format", "insert-chunk", "remove-chunk", "set-chunk-keyframe", "add-unknown-chunk", "remove-unknown-chunk"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_diff_for(stream_index: usize, inner: AviStreamDiff) -> AviDiff {
    AviDiff { main_header: None, streams: Some(IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: stream_index, diff: inner }], added: vec![] }), idx1_present: None, unknown_chunks: None, hdrl_extra: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chunk_diff_for(stream_index: usize, chunks: IndexedDiff<AviChunk, AviChunkDiff>) -> AviDiff {
    stream_diff_for(stream_index, AviStreamDiff { chunks: Some(chunks), ..AviStreamDiff::default() })
}

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &AviMutation, base: &AviSnapshot) -> protocol::MutationOutcome<AviDiff> {
    protocol::MutationOutcome::new(match this {
        AviMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => <AviDiff as protocol::command::DiffAlgebra<AviSnapshot>>::between(base, snapshot),
        AviMutation::SetMainHeader(set_main_header::SetMainHeader { main_header }) => AviDiff { main_header: Some(main_header.clone()), ..AviDiff::default() },
        AviMutation::SetIdx1Present(set_idx1_present::SetIdx1Present { idx1_present }) => AviDiff { idx1_present: Some(*idx1_present), ..AviDiff::default() },
        AviMutation::InsertStream(insert_stream::InsertStream { index, stream }) => AviDiff { streams: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: stream.clone() }] }), ..AviDiff::default() },
        AviMutation::RemoveStream(remove_stream::RemoveStream { index }) => AviDiff { streams: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }), ..AviDiff::default() },
        AviMutation::SetStreamHeader(set_stream_header::SetStreamHeader { stream_index, strh }) => stream_diff_for(*stream_index, AviStreamDiff { strh: Some(strh.clone()), ..AviStreamDiff::default() }),
        AviMutation::SetStreamFormat(set_stream_format::SetStreamFormat { stream_index, strf }) => stream_diff_for(*stream_index, AviStreamDiff { strf: Some(strf.clone()), ..AviStreamDiff::default() }),
        AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index, index, chunk }) => chunk_diff_for(*stream_index, IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: chunk.clone() }] }),
        AviMutation::RemoveChunk(remove_chunk::RemoveChunk { stream_index, index }) => chunk_diff_for(*stream_index, IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }),
        AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index, index, keyframe }) => {
            chunk_diff_for(*stream_index, IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: *index, diff: AviChunkDiff { data: None, keyframe: Some(*keyframe) } }], added: vec![] })
        }
        AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index, item }) => {
            AviDiff { unknown_chunks: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: item.clone() }] }), ..AviDiff::default() }
        }
        AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index }) => AviDiff { unknown_chunks: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }), ..AviDiff::default() },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &AviMutation, base: &AviSnapshot) -> Vec<AviMutation> {
    match this {
        AviMutation::SetSnapshot(_) => vec![AviMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        AviMutation::SetMainHeader(_) => vec![AviMutation::SetMainHeader(set_main_header::SetMainHeader { main_header: base.main_header.clone() })],
        AviMutation::SetIdx1Present(_) => vec![AviMutation::SetIdx1Present(set_idx1_present::SetIdx1Present { idx1_present: base.idx1_present })],
        AviMutation::InsertStream(insert_stream::InsertStream { index, .. }) => vec![AviMutation::RemoveStream(remove_stream::RemoveStream { index: *index })],
        AviMutation::RemoveStream(remove_stream::RemoveStream { index }) => match base.streams.get(*index) {
            Some(stream) => vec![AviMutation::InsertStream(insert_stream::InsertStream { index: *index, stream: stream.clone() })],
            None => Vec::new(),
        },
        AviMutation::SetStreamHeader(set_stream_header::SetStreamHeader { stream_index, .. }) => match base.streams.get(*stream_index) {
            Some(stream) => vec![AviMutation::SetStreamHeader(set_stream_header::SetStreamHeader { stream_index: *stream_index, strh: stream.strh.clone() })],
            None => Vec::new(),
        },
        AviMutation::SetStreamFormat(set_stream_format::SetStreamFormat { stream_index, .. }) => match base.streams.get(*stream_index) {
            Some(stream) => vec![AviMutation::SetStreamFormat(set_stream_format::SetStreamFormat { stream_index: *stream_index, strf: stream.strf.clone() })],
            None => Vec::new(),
        },
        AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index, index, .. }) => vec![AviMutation::RemoveChunk(remove_chunk::RemoveChunk { stream_index: *stream_index, index: *index })],
        AviMutation::RemoveChunk(remove_chunk::RemoveChunk { stream_index, index }) => match base.streams.get(*stream_index).and_then(|s| s.chunks.get(*index)) {
            Some(chunk) => vec![AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index: *stream_index, index: *index, chunk: chunk.clone() })],
            None => Vec::new(),
        },
        AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index, index, .. }) => match base.streams.get(*stream_index).and_then(|s| s.chunks.get(*index)) {
            Some(chunk) => vec![AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index: *stream_index, index: *index, keyframe: chunk.keyframe })],
            None => Vec::new(),
        },
        AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index, .. }) => vec![AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index: *index })],
        AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index }) => match base.unknown_chunks.get(*index) {
            Some(item) => vec![AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index: *index, item: item.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_avi_mutation(snapshot: &mut AviSnapshot, mutation: &AviMutation) -> protocol::MutationOutcome<AviDiff> {
    let outcome = <AviMutation as Mutation<AviSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — plain `pack::json` round-trip (see mp4's identical
/// module-doc rationale: f6-final-summary.md §4.4, no generic collection-diff `DslField` bridge).
impl OpText for AviMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = pack::parse_json(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self)))
    }
}

impl OpBinary for AviMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(self))).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🔑️promotes-the-8ff733/🦀️.rs"]
mod set_snapshot_promotes_the_second_movi_chunk_to_a_keyframe;
//#endregion 🧪️FixtureCases
