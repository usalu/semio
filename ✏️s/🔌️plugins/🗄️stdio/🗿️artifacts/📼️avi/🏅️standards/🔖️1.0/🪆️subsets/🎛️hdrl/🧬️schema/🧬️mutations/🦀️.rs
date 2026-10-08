//! 🧬️ AviMutation — named-variant vocabulary (imperative verbs, gif/svg precedent). Every
//! variant's `diff()` is handcrafted; `inverse()` is handcrafted per variant, index-aware.

use crate::standards::v1_0::subsets::any::schema::diff::{AviChunkDiff, AviDiff, AviStreamDiff, IndexedAdded, IndexedDiff, IndexedModified};
use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::Mutation;


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
    &["set-main-header", "set-idx1-present", "insert-stream", "remove-stream", "set-stream-header", "set-stream-format", "insert-chunk", "remove-chunk", "set-chunk-keyframe", "add-unknown-chunk", "remove-unknown-chunk"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_diff_for(stream_index: usize, inner: AviStreamDiff) -> AviDiff {
    AviDiff { main_header: None, streams: Some(IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: stream_index, diff: inner }], added: vec![] }), idx1_present: None, unknown_chunks: None, hdrl_extra: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chunk_diff_for(stream_index: usize, chunks: IndexedDiff<AviChunk, AviChunkDiff>) -> AviDiff {
    stream_diff_for(stream_index, AviStreamDiff { chunks: Some(chunks), ..AviStreamDiff::default() })
}

//#endregion 🔖️MutationTrait

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the main header and `idx1` flag if they moved, each stream in place (its
/// header, its format, its chunks; a stream whose retained auxiliaries differ is removed and inserted anew), the diverging stream
/// tail, and the unknown chunks (replaced in place by remove-then-add). A chunk differing only in its keyframe flag is re-flagged;
/// any other chunk change is remove-then-insert. The `hdrl` auxiliaries have no leaf, so a change to them is left unaddressed.
pub fn net_mutations(base: &AviSnapshot, next: &AviSnapshot) -> Vec<AviMutation> {
    let mut leaves = Vec::new();
    if base.main_header != next.main_header {
        leaves.push(AviMutation::SetMainHeader(set_main_header::SetMainHeader { main_header: next.main_header.clone() }));
    }
    if base.idx1_present != next.idx1_present {
        leaves.push(AviMutation::SetIdx1Present(set_idx1_present::SetIdx1Present { idx1_present: next.idx1_present }));
    }
    let paired = base.streams.len().min(next.streams.len());
    for (stream_index, (before, after)) in base.streams.iter().zip(&next.streams).enumerate().filter(|(_, (before, after))| before != after) {
        if before.strl_extra != after.strl_extra {
            leaves.push(AviMutation::RemoveStream(remove_stream::RemoveStream { index: stream_index }));
            leaves.push(AviMutation::InsertStream(insert_stream::InsertStream { index: stream_index, stream: after.clone() }));
            continue;
        }
        if before.strh != after.strh {
            leaves.push(AviMutation::SetStreamHeader(set_stream_header::SetStreamHeader { stream_index, strh: after.strh.clone() }));
        }
        if before.strf != after.strf {
            leaves.push(AviMutation::SetStreamFormat(set_stream_format::SetStreamFormat { stream_index, strf: after.strf.clone() }));
        }
        let chunks_paired = before.chunks.len().min(after.chunks.len());
        for (index, (old, new)) in before.chunks.iter().zip(&after.chunks).enumerate().filter(|(_, (old, new))| old != new) {
            if (&old.fourcc, &old.data) == (&new.fourcc, &new.data) {
                leaves.push(AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index, index, keyframe: new.keyframe }));
            } else {
                leaves.push(AviMutation::RemoveChunk(remove_chunk::RemoveChunk { stream_index, index }));
                leaves.push(AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index, index, chunk: new.clone() }));
            }
        }
        leaves.extend((chunks_paired..before.chunks.len()).rev().map(|index| AviMutation::RemoveChunk(remove_chunk::RemoveChunk { stream_index, index })));
        leaves.extend(after.chunks.iter().enumerate().skip(chunks_paired).map(|(index, chunk)| AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index, index, chunk: chunk.clone() })));
    }
    leaves.extend((paired..base.streams.len()).rev().map(|index| AviMutation::RemoveStream(remove_stream::RemoveStream { index })));
    leaves.extend(next.streams.iter().enumerate().skip(paired).map(|(index, stream)| AviMutation::InsertStream(insert_stream::InsertStream { index, stream: stream.clone() })));
    let unknown_paired = base.unknown_chunks.len().min(next.unknown_chunks.len());
    for (index, (_, new)) in base.unknown_chunks.iter().zip(&next.unknown_chunks).enumerate().filter(|(_, (old, new))| old != new) {
        leaves.push(AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index }));
        leaves.push(AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index, item: new.clone() }));
    }
    leaves.extend((unknown_paired..base.unknown_chunks.len()).rev().map(|index| AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index })));
    leaves.extend(next.unknown_chunks.iter().enumerate().skip(unknown_paired).map(|(index, item)| AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index, item: item.clone() })));
    leaves
}
//#endregion 🔖️Net

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_avi_mutation(snapshot: &mut AviSnapshot, mutation: &AviMutation) -> protocol::MutationOutcome<AviDiff> {
    let outcome = <AviMutation as Mutation<AviSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Mutation

//#region OpCodecs





//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
