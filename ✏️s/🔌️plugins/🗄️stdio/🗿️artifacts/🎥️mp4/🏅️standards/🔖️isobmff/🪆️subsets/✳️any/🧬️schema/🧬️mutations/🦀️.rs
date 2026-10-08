//! 🧬️ Mp4Mutation — named-variant vocabulary (imperative verbs, gif/svg precedent). Every
//! variant's `diff()` is handcrafted (constructs the sparse `Mp4Diff` directly — apply-and-capture
//! is banned); `inverse()` is handcrafted per variant, index-aware.

use crate::standards::isobmff::subsets::any::schema::diff::{IndexedAdded, IndexedDiff, IndexedModified, Mp4Diff, Mp4SampleDiff, Mp4TrackDiff};
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Movie, Mp4Sample, Mp4Snapshot, Mp4Track};
#[cfg(test)]
use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4TrackMetadata;
use protocol::Mutation;


//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "🧱insert-sample/🦀️.rs"]
pub mod insert_sample;
#[path = "➕insert-track/🦀️.rs"]
pub mod insert_track;
#[path = "🗑️remove-sample/🦀️.rs"]
pub mod remove_sample;
#[path = "➖remove-track/🦀️.rs"]
pub mod remove_track;
#[path = "🏷️set-ftyp/🦀️.rs"]
pub mod set_ftyp;
#[path = "🎬set-movie/🦀️.rs"]
pub mod set_movie;
#[path = "⭐set-sample-sync/🦀️.rs"]
pub mod set_sample_sync;
#[path = "🎛️set-track-codec/🦀️.rs"]
pub mod set_track_codec;
#[path = "📐set-track-dimensions/🦀️.rs"]
pub mod set_track_dimensions;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none — and `no`
/// is not an approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Mp4Snapshot, diff = Mp4Diff, schema = "Mp4Mutation")]
pub enum Mp4Mutation {
    SetFtyp(set_ftyp::SetFtyp),
    SetMovie(set_movie::SetMovie),
    InsertTrack(insert_track::InsertTrack),
    RemoveTrack(remove_track::RemoveTrack),
    SetTrackDimensions(set_track_dimensions::SetTrackDimensions),
    SetTrackCodec(set_track_codec::SetTrackCodec),
    InsertSample(insert_sample::InsertSample),
    RemoveSample(remove_sample::RemoveSample),
    SetSampleSync(set_sample_sync::SetSampleSync),
}

/// 📇️ Kebab-case spelling of every `Mp4Mutation` variant, in declaration order — the ground truth
/// `../../🔣️oracle.json`'s own `kinds` list is checked against (the framework never
/// parses Rust, so `kinds_const_matches_enum_variants_in_declaration_order` below is what keeps the
/// declaration honest). Wave 7 fleet brief, ticket 26/08/23/END-TO-END-TESTING-REFACTOR.
pub const KINDS: &[&str] = &["set-ftyp", "set-movie", "insert-track", "remove-track", "set-track-dimensions", "set-track-codec", "insert-sample", "remove-sample", "set-sample-sync"];

fn track_diff_for(track_index: usize, inner: Mp4TrackDiff) -> Mp4Diff {
    Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: track_index, diff: inner }], added: vec![] }) }
}

fn sample_diff_for(track_index: usize, samples: IndexedDiff<Mp4Sample, Mp4SampleDiff>, chunk_sample_counts: Option<Vec<u32>>) -> Mp4Diff {
    track_diff_for(track_index, Mp4TrackDiff { samples: Some(samples), chunk_sample_counts, ..Mp4TrackDiff::default() })
}



//#endregion 🔖️Mutation


//#region OpCodecs



//#endregion OpCodecs

//#endregion 🔖️MutationTrait

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests


#[cfg(test)]
use protocol::{OpBinary,OpText};
