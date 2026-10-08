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

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention).
pub fn apply_mp4_mutation(snapshot: &mut Mp4Snapshot, mutation: &Mp4Mutation) -> protocol::MutationOutcome<Mp4Diff> {
    let outcome = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Mutation

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the file type and the movie header if they moved, then every track in place
/// (its dimensions, its codec, and its samples row by row: a sample differing only in its sync flag is re-flagged, any other
/// change is remove-then-insert; a track whose id, timescale, metadata or chunking differ is removed and inserted anew) and the
/// diverging track tail. Sample leaves re-chunk a track into one chunk, so a track keeping its sample count must keep its chunking.
pub fn net_mutations(base: &Mp4Snapshot, next: &Mp4Snapshot) -> Vec<Mp4Mutation> {
    let mut leaves = Vec::new();
    if base.ftyp != next.ftyp {
        leaves.push(Mp4Mutation::SetFtyp(set_ftyp::SetFtyp { ftyp: next.ftyp.clone() }));
    }
    if base.movie != next.movie {
        leaves.push(Mp4Mutation::SetMovie(set_movie::SetMovie { movie: next.movie.clone() }));
    }
    let paired = base.tracks.len().min(next.tracks.len());
    for (track_index, (before, after)) in base.tracks.iter().zip(&next.tracks).enumerate().filter(|(_, (before, after))| before != after) {
        if (before.track_id, before.timescale, &before.metadata) != (after.track_id, after.timescale, &after.metadata) || (before.samples.len() == after.samples.len() && before.chunk_sample_counts != after.chunk_sample_counts) {
            leaves.push(Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index: track_index }));
            leaves.push(Mp4Mutation::InsertTrack(insert_track::InsertTrack { index: track_index, track: after.clone() }));
            continue;
        }
        if (before.width, before.height) != (after.width, after.height) {
            leaves.push(Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index, width: after.width, height: after.height }));
        }
        if before.codec != after.codec {
            leaves.push(Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index, codec: after.codec.clone() }));
        }
        let samples_paired = before.samples.len().min(after.samples.len());
        for (index, (old, new)) in before.samples.iter().zip(&after.samples).enumerate().filter(|(_, (old, new))| old != new) {
            if (&old.data, old.duration, old.cts_offset) == (&new.data, new.duration, new.cts_offset) {
                leaves.push(Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index, index, sync: new.sync }));
            } else {
                leaves.push(Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index, index }));
                leaves.push(Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index, index, sample: new.clone() }));
            }
        }
        leaves.extend((samples_paired..before.samples.len()).rev().map(|index| Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index, index })));
        leaves.extend(after.samples.iter().enumerate().skip(samples_paired).map(|(index, sample)| Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index, index, sample: sample.clone() })));
    }
    leaves.extend((paired..base.tracks.len()).rev().map(|index| Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index })));
    leaves.extend(next.tracks.iter().enumerate().skip(paired).map(|(index, track)| Mp4Mutation::InsertTrack(insert_track::InsertTrack { index, track: track.clone() })));
    leaves
}
//#endregion 🔖️Net

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
