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
#[path = "🪵set-hdrl-extra/🦀️.rs"]
pub mod set_hdrl_extra;
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
    SetHdrlExtra(set_hdrl_extra::SetHdrlExtra),
}

/// 📇️ Kebab-case spelling of every `AviMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] =
    &["set-main-header", "set-idx1-present", "insert-stream", "remove-stream", "set-stream-header", "set-stream-format", "insert-chunk", "remove-chunk", "set-chunk-keyframe", "add-unknown-chunk", "remove-unknown-chunk", "set-hdrl-extra"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_diff_for(stream_index: usize, inner: AviStreamDiff) -> AviDiff {
    AviDiff { main_header: None, streams: Some(IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: stream_index, diff: inner }], added: vec![] }), idx1_present: None, unknown_chunks: None, hdrl_extra: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chunk_diff_for(stream_index: usize, chunks: IndexedDiff<AviChunk, AviChunkDiff>) -> AviDiff {
    stream_diff_for(stream_index, AviStreamDiff { chunks: Some(chunks), ..AviStreamDiff::default() })
}

//#endregion 🔖️MutationTrait




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
