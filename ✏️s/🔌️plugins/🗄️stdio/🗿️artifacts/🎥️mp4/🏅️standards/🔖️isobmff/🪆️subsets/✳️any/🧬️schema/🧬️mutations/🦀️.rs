//! 🧬️ Mp4Mutation — named-variant vocabulary (imperative verbs, gif/svg precedent). Every
//! variant's `diff()` is handcrafted (constructs the sparse `Mp4Diff` directly — apply-and-capture
//! is banned); `inverse()` is handcrafted per variant, index-aware.

use crate::standards::isobmff::subsets::any::schema::diff::{IndexedAdded, IndexedDiff, IndexedModified, Mp4Diff, Mp4SampleDiff, Mp4TrackDiff};
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Snapshot, Mp4Track};
#[cfg(test)]
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Movie, Mp4TrackMetadata};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-ftyp/🦀️.rs"]
pub mod set_ftyp;
#[path = "➕insert-track/🦀️.rs"]
pub mod insert_track;
#[path = "➖remove-track/🦀️.rs"]
pub mod remove_track;
#[path = "📐set-track-dimensions/🦀️.rs"]
pub mod set_track_dimensions;
#[path = "🎛️set-track-codec/🦀️.rs"]
pub mod set_track_codec;
#[path = "🧱insert-sample/🦀️.rs"]
pub mod insert_sample;
#[path = "🗑️remove-sample/🦀️.rs"]
pub mod remove_sample;
#[path = "⭐set-sample-sync/🦀️.rs"]
pub mod set_sample_sync;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none — and `no`
/// is not an approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, dsl::DslOps)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Mp4Snapshot, diff = Mp4Diff, schema = "Mp4Mutation")]
pub enum Mp4Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetFtyp(set_ftyp::SetFtyp),
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
pub const KINDS: &[&str] = &["set-snapshot", "set-ftyp", "insert-track", "remove-track", "set-track-dimensions", "set-track-codec", "insert-sample", "remove-sample", "set-sample-sync"];

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
/// 🎙️ Structured operation text through the shared `DslVariants` record machinery.
impl OpText for Mp4Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 32 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(candidate, _)| candidate == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Structured operation binary through the shared tagged-record protocol.
impl OpBinary for Mp4Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion OpCodecs

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &Mp4Mutation, base: &Mp4Snapshot) -> protocol::MutationOutcome<Mp4Diff> {
    protocol::MutationOutcome::new(match this {
        Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => <Mp4Diff as protocol::command::DiffAlgebra<Mp4Snapshot>>::between(base, snapshot),
        Mp4Mutation::SetFtyp(set_ftyp::SetFtyp { ftyp }) => Mp4Diff { ftyp: Some(ftyp.clone()), movie: None, tracks: None },
        Mp4Mutation::InsertTrack(insert_track::InsertTrack { index, track }) => Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: track.clone() }] }) },
        Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index }) => Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }) },
        Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index, width, height }) => track_diff_for(*track_index, Mp4TrackDiff { width: Some(*width), height: Some(*height), ..Mp4TrackDiff::default() }),
        Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index, codec }) => track_diff_for(*track_index, Mp4TrackDiff { codec: Some(codec.clone()), ..Mp4TrackDiff::default() }),
        Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index, index, sample }) => {
            let count = base.tracks.get(*track_index).map_or(1, |track| track.samples.len() as u32 + 1);
            sample_diff_for(*track_index, IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: sample.clone() }] }, Some(vec![count]))
        }
        Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index, index }) => {
            let count = base.tracks.get(*track_index).map_or(0, |track| track.samples.len().saturating_sub(1) as u32);
            sample_diff_for(*track_index, IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }, Some(vec![count]))
        }
        Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index, index, sync }) => {
            sample_diff_for(*track_index, IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: *index, diff: Mp4SampleDiff { data: None, duration: None, cts_offset: None, sync: Some(*sync) } }], added: vec![] }, None)
        }
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &Mp4Mutation, base: &Mp4Snapshot) -> Vec<Mp4Mutation> {
    match this {
        Mp4Mutation::SetSnapshot(_) => vec![Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        Mp4Mutation::SetFtyp(_) => vec![Mp4Mutation::SetFtyp(set_ftyp::SetFtyp { ftyp: base.ftyp.clone() })],
        Mp4Mutation::InsertTrack(insert_track::InsertTrack { index, .. }) => vec![Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index: *index })],
        Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index }) => match base.tracks.get(*index) {
            Some(track) => vec![Mp4Mutation::InsertTrack(insert_track::InsertTrack { index: *index, track: track.clone() })],
            None => Vec::new(),
        },
        Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index, .. }) => match base.tracks.get(*track_index) {
            Some(track) => vec![Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index: *track_index, width: track.width, height: track.height })],
            None => Vec::new(),
        },
        Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index, .. }) => match base.tracks.get(*track_index) {
            Some(track) => vec![Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index: *track_index, codec: track.codec.clone() })],
            None => Vec::new(),
        },
        Mp4Mutation::InsertSample(_) | Mp4Mutation::RemoveSample(_) => vec![Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index, index, .. }) => match base.tracks.get(*track_index).and_then(|t| t.samples.get(*index)) {
            Some(sample) => vec![Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index: *track_index, index: *index, sync: sample.sync })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
