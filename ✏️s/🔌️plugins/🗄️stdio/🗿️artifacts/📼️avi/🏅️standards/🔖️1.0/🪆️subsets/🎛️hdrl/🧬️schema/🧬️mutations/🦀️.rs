//! 🧬️ AviMutation — named-variant vocabulary (imperative verbs, gif/svg precedent). Every
//! variant's `diff()` is handcrafted; `inverse()` is handcrafted per variant, index-aware.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::{AviChunkDiff, AviDiff, AviStreamDiff, IndexedAdded, IndexedDiff, IndexedModified};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🎬set-main-header/🦀️.rs"]
pub mod set_main_header;
#[path = "📇️set-idx1-present/🦀️.rs"]
pub mod set_idx1_present;
#[path = "📥️insert-stream/🦀️.rs"]
pub mod insert_stream;
#[path = "📤️remove-stream/🦀️.rs"]
pub mod remove_stream;
#[path = "🎞️set-stream-header/🦀️.rs"]
pub mod set_stream_header;
#[path = "🎨set-stream-format/🦀️.rs"]
pub mod set_stream_format;
#[path = "🧩insert-chunk/🦀️.rs"]
pub mod insert_chunk;
#[path = "🗑️remove-chunk/🦀️.rs"]
pub mod remove_chunk;
#[path = "🔑set-chunk-keyframe/🦀️.rs"]
pub mod set_chunk_keyframe;
#[path = "🧱add-unknown-chunk/🦀️.rs"]
pub mod add_unknown_chunk;
#[path = "🧹remove-unknown-chunk/🦀️.rs"]
pub mod remove_unknown_chunk;
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
pub const KINDS: &[&str] = &[
    "set-snapshot",
    "set-main-header",
    "set-idx1-present",
    "insert-stream",
    "remove-stream",
    "set-stream-header",
    "set-stream-format",
    "insert-chunk",
    "remove-chunk",
    "set-chunk-keyframe",
    "add-unknown-chunk",
    "remove-unknown-chunk",
];

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
        AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index, item }) => AviDiff { unknown_chunks: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: item.clone() }] }), ..AviDiff::default() },
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
mod tests {
    use super::*;
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA;

    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> AviSnapshot {
        AviSnapshot {
            schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
            main_header: AviMainHeader {
                micro_sec_per_frame: 100_000,
                max_bytes_per_sec: 1400,
                padding_granularity: 0,
                flags: 0x10,
                total_frames: 1,
                initial_frames: 0,
                streams: 1,
                suggested_buffer_size: 140,
                width: 16,
                height: 16,
                reserved: vec![0, 0, 0, 0],
            },
            streams: vec![AviStream {
                strh: AviStreamHeader {
                    fcc_type: "vids".into(),
                    fcc_handler: "MJPG".into(),
                    flags: 0,
                    priority: 0,
                    language: 0,
                    initial_frames: 0,
                    scale: 1,
                    rate: 10,
                    start: 0,
                    length: 1,
                    suggested_buffer_size: 140,
                    quality: -1,
                    sample_size: 0,
                    rc_frame_left: 0,
                    rc_frame_top: 0,
                    rc_frame_right: 16,
                    rc_frame_bottom: 16,
                    rc_frame_width: 16,
                    strh_extra: vec![],
                },
                strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
                chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1, 2, 3], keyframe: true }],
                strl_extra: vec![],
            }],
            idx1_present: true,
            unknown_chunks: vec![RiffChunk { fourcc: "JUNK".into(), data: vec![0] }],
            hdrl_extra: vec![],
        }
    }

    /// 🧪️ mutation_diff_law + inverse_law, exercised across every real variant.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_and_inverse_law_hold_for_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            AviMutation::SetMainHeader(set_main_header::SetMainHeader { main_header: AviMainHeader { width: 32, ..base.main_header.clone() } }),
            AviMutation::SetIdx1Present(set_idx1_present::SetIdx1Present { idx1_present: false }),
            AviMutation::InsertStream(insert_stream::InsertStream { index: 1, stream: AviStream { strh: base.streams[0].strh.clone(), strf: base.streams[0].strf.clone(), chunks: vec![], strl_extra: vec![] } }),
            AviMutation::SetStreamHeader(set_stream_header::SetStreamHeader { stream_index: 0, strh: AviStreamHeader { rate: 30, ..base.streams[0].strh.clone() } }),
            AviMutation::SetStreamFormat(set_stream_format::SetStreamFormat { stream_index: 0, strf: AviStreamFormat::Raw { data: vec![9] } }),
            AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index: 0, index: 1, chunk: AviChunk { fourcc: "00dc".into(), data: vec![9, 9], keyframe: false } }),
            AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index: 0, index: 0, keyframe: false }),
            AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index: 1, item: RiffChunk { fourcc: "MORE".into(), data: vec![1] } }),
            AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index: 0 }),
        ];
        for m in variants {
            let mut snap = base.clone();
            let diff = <AviMutation as Mutation<AviSnapshot>>::diff(&m, &snap);
            let expected = diff.diff().apply(&snap).unwrap();
            let returned = apply_avi_mutation(&mut snap, &m);
            assert_eq!(returned, diff, "apply_avi_mutation must return the SAME diff as Mutation::diff for {m:?}");
            assert_eq!(snap, expected, "mutation_diff_law failed for {m:?}");

            let inv = <AviMutation as Mutation<AviSnapshot>>::inverse(&m, &base);
            assert_eq!(inv.len(), 1);
            let mut round = snap.clone();
            apply_avi_mutation(&mut round, &inv[0]);
            assert_eq!(round, base, "inverse_law failed for {m:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_stream_then_insert_stream_round_trips() {
        let mut base = base_snapshot();
        base.streams.push(AviStream { strh: base.streams[0].strh.clone(), strf: base.streams[0].strf.clone(), chunks: vec![], strl_extra: vec![] });
        let m = AviMutation::RemoveStream(remove_stream::RemoveStream { index: 0 });
        let mut snap = base.clone();
        apply_avi_mutation(&mut snap, &m);
        assert_eq!(snap.streams.len(), 1);
        let inv = <AviMutation as Mutation<AviSnapshot>>::inverse(&m, &base);
        let mut round = snap.clone();
        apply_avi_mutation(&mut round, &inv[0]);
        assert_eq!(round, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_snapshot_still_works_as_a_full_replace() {
        let base = base_snapshot();
        let mut next = base.clone();
        next.main_header.width = 999;
        let mutation = AviMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: next.clone() });
        let diff = <AviMutation as Mutation<AviSnapshot>>::diff(&mutation, &base);
        assert_eq!(diff.diff().apply(&base).unwrap(), next);
        let inv = <AviMutation as Mutation<AviSnapshot>>::inverse(&mutation, &base);
        let mut round = next.clone();
        apply_avi_mutation(&mut round, &inv[0]);
        assert_eq!(round, base);
    }

    /// 🧪️ op_text_binary_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for m in [
            AviMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            AviMutation::SetIdx1Present(set_idx1_present::SetIdx1Present { idx1_present: false }),
            AviMutation::RemoveStream(remove_stream::RemoveStream { index: 0 }),
            AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index: 0, index: 0, keyframe: true }),
        ] {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = AviMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m);

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = AviMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m);
        }
    }

    /// 🧪️ kinds_const_matches_enum_variants_in_declaration_order -- one instance per `AviMutation`
    /// variant, in the enum's own declaration order, mapped to its `KINDS` spelling by a direct
    /// match rather than through `OpText::print_op` (which emits camelCase JSON tagged
    /// `"mutation"`, not this file's own kebab-case `KINDS` convention -- see module doc comment).
    #[semio_framework_async_macros::async_test]
    async fn kinds_const_matches_enum_variants_in_declaration_order() {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) -- see R9
        fn kind_of(m: &AviMutation) -> &'static str {
            match m {
                AviMutation::SetSnapshot(_) => "set-snapshot",
                AviMutation::SetMainHeader(_) => "set-main-header",
                AviMutation::SetIdx1Present(_) => "set-idx1-present",
                AviMutation::InsertStream(_) => "insert-stream",
                AviMutation::RemoveStream(_) => "remove-stream",
                AviMutation::SetStreamHeader(_) => "set-stream-header",
                AviMutation::SetStreamFormat(_) => "set-stream-format",
                AviMutation::InsertChunk(_) => "insert-chunk",
                AviMutation::RemoveChunk(_) => "remove-chunk",
                AviMutation::SetChunkKeyframe(_) => "set-chunk-keyframe",
                AviMutation::AddUnknownChunk(_) => "add-unknown-chunk",
                AviMutation::RemoveUnknownChunk(_) => "remove-unknown-chunk",
            }
        }
        let base = base_snapshot();
        let one_per_variant = vec![
            AviMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            AviMutation::SetMainHeader(set_main_header::SetMainHeader { main_header: base.main_header.clone() }),
            AviMutation::SetIdx1Present(set_idx1_present::SetIdx1Present { idx1_present: false }),
            AviMutation::InsertStream(insert_stream::InsertStream { index: 1, stream: base.streams[0].clone() }),
            AviMutation::RemoveStream(remove_stream::RemoveStream { index: 0 }),
            AviMutation::SetStreamHeader(set_stream_header::SetStreamHeader { stream_index: 0, strh: base.streams[0].strh.clone() }),
            AviMutation::SetStreamFormat(set_stream_format::SetStreamFormat { stream_index: 0, strf: base.streams[0].strf.clone() }),
            AviMutation::InsertChunk(insert_chunk::InsertChunk { stream_index: 0, index: 0, chunk: base.streams[0].chunks[0].clone() }),
            AviMutation::RemoveChunk(remove_chunk::RemoveChunk { stream_index: 0, index: 0 }),
            AviMutation::SetChunkKeyframe(set_chunk_keyframe::SetChunkKeyframe { stream_index: 0, index: 0, keyframe: false }),
            AviMutation::AddUnknownChunk(add_unknown_chunk::AddUnknownChunk { index: 0, item: base.unknown_chunks[0].clone() }),
            AviMutation::RemoveUnknownChunk(remove_unknown_chunk::RemoveUnknownChunk { index: 0 }),
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(kind_of(mutation), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
        }
    }
}
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🔑️promotes-the-second-movi-chunk-to-a-keyframe/🦀️.rs"]
mod set_snapshot_promotes_the_second_movi_chunk_to_a_keyframe;
//#endregion 🧪️FixtureCases
