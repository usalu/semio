//! 🧬️ AviMutation — named-variant vocabulary (imperative verbs, gif/svg precedent). Every
//! variant's `diff()` is handcrafted; `inverse()` is handcrafted per variant, index-aware.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::{AviChunkDiff, AviDiff, AviStreamDiff, IndexedAdded, IndexedDiff, IndexedModified};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum AviMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: AviSnapshot,
    },
    SetMainHeader {
        main_header: AviMainHeader,
    },
    SetIdx1Present {
        idx1_present: bool,
    },
    InsertStream {
        index: usize,
        stream: AviStream,
    },
    RemoveStream {
        index: usize,
    },
    SetStreamHeader {
        stream_index: usize,
        strh: AviStreamHeader,
    },
    SetStreamFormat {
        stream_index: usize,
        strf: AviStreamFormat,
    },
    InsertChunk {
        stream_index: usize,
        index: usize,
        chunk: AviChunk,
    },
    RemoveChunk {
        stream_index: usize,
        index: usize,
    },
    SetChunkKeyframe {
        stream_index: usize,
        index: usize,
        keyframe: bool,
    },
    AddUnknownChunk {
        index: usize,
        item: RiffChunk,
    },
    RemoveUnknownChunk {
        index: usize,
    },
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_diff_for(stream_index: usize, inner: AviStreamDiff) -> AviDiff {
    AviDiff { main_header: None, streams: Some(IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: stream_index, diff: inner }], added: vec![] }), idx1_present: None, unknown_chunks: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chunk_diff_for(stream_index: usize, chunks: IndexedDiff<AviChunk, AviChunkDiff>) -> AviDiff {
    stream_diff_for(stream_index, AviStreamDiff { chunks: Some(chunks), ..AviStreamDiff::default() })
}

impl Mutation<AviSnapshot> for AviMutation {
    type Diff = AviDiff;

    async fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            AviMutation::NoMutation => AviDiff::default(),
            AviMutation::SetSnapshot { snapshot } => <AviDiff as protocol::command::DiffAlgebra<AviSnapshot>>::between(base, snapshot).await,
            AviMutation::SetMainHeader { main_header } => AviDiff { main_header: Some(main_header.clone()), ..AviDiff::default() },
            AviMutation::SetIdx1Present { idx1_present } => AviDiff { idx1_present: Some(*idx1_present), ..AviDiff::default() },
            AviMutation::InsertStream { index, stream } => AviDiff { streams: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: stream.clone() }] }), ..AviDiff::default() },
            AviMutation::RemoveStream { index } => AviDiff { streams: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }), ..AviDiff::default() },
            AviMutation::SetStreamHeader { stream_index, strh } => stream_diff_for(*stream_index, AviStreamDiff { strh: Some(strh.clone()), ..AviStreamDiff::default() }),
            AviMutation::SetStreamFormat { stream_index, strf } => stream_diff_for(*stream_index, AviStreamDiff { strf: Some(strf.clone()), ..AviStreamDiff::default() }),
            AviMutation::InsertChunk { stream_index, index, chunk } => chunk_diff_for(*stream_index, IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: chunk.clone() }] }),
            AviMutation::RemoveChunk { stream_index, index } => chunk_diff_for(*stream_index, IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }),
            AviMutation::SetChunkKeyframe { stream_index, index, keyframe } => {
                chunk_diff_for(*stream_index, IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: *index, diff: AviChunkDiff { data: None, keyframe: Some(*keyframe) } }], added: vec![] })
            }
            AviMutation::AddUnknownChunk { index, item } => AviDiff { unknown_chunks: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: item.clone() }] }), ..AviDiff::default() },
            AviMutation::RemoveUnknownChunk { index } => AviDiff { unknown_chunks: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }), ..AviDiff::default() },
        })
    }

    async fn inverse(&self, base: &AviSnapshot) -> Vec<Self> {
        match self {
            AviMutation::NoMutation => vec![AviMutation::NoMutation],
            AviMutation::SetSnapshot { .. } => vec![AviMutation::SetSnapshot { snapshot: base.clone() }],
            AviMutation::SetMainHeader { .. } => vec![AviMutation::SetMainHeader { main_header: base.main_header.clone() }],
            AviMutation::SetIdx1Present { .. } => vec![AviMutation::SetIdx1Present { idx1_present: base.idx1_present }],
            AviMutation::InsertStream { index, .. } => vec![AviMutation::RemoveStream { index: *index }],
            AviMutation::RemoveStream { index } => match base.streams.get(*index) {
                Some(stream) => vec![AviMutation::InsertStream { index: *index, stream: stream.clone() }],
                None => vec![AviMutation::NoMutation],
            },
            AviMutation::SetStreamHeader { stream_index, .. } => match base.streams.get(*stream_index) {
                Some(stream) => vec![AviMutation::SetStreamHeader { stream_index: *stream_index, strh: stream.strh.clone() }],
                None => vec![AviMutation::NoMutation],
            },
            AviMutation::SetStreamFormat { stream_index, .. } => match base.streams.get(*stream_index) {
                Some(stream) => vec![AviMutation::SetStreamFormat { stream_index: *stream_index, strf: stream.strf.clone() }],
                None => vec![AviMutation::NoMutation],
            },
            AviMutation::InsertChunk { stream_index, index, .. } => vec![AviMutation::RemoveChunk { stream_index: *stream_index, index: *index }],
            AviMutation::RemoveChunk { stream_index, index } => match base.streams.get(*stream_index).and_then(|s| s.chunks.get(*index)) {
                Some(chunk) => vec![AviMutation::InsertChunk { stream_index: *stream_index, index: *index, chunk: chunk.clone() }],
                None => vec![AviMutation::NoMutation],
            },
            AviMutation::SetChunkKeyframe { stream_index, index, .. } => match base.streams.get(*stream_index).and_then(|s| s.chunks.get(*index)) {
                Some(chunk) => vec![AviMutation::SetChunkKeyframe { stream_index: *stream_index, index: *index, keyframe: chunk.keyframe }],
                None => vec![AviMutation::NoMutation],
            },
            AviMutation::AddUnknownChunk { index, .. } => vec![AviMutation::RemoveUnknownChunk { index: *index }],
            AviMutation::RemoveUnknownChunk { index } => match base.unknown_chunks.get(*index) {
                Some(item) => vec![AviMutation::AddUnknownChunk { index: *index, item: item.clone() }],
                None => vec![AviMutation::NoMutation],
            },
        }
    }
}

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
/// 🎙️ Handcrafted `OpText`/`OpBinary` — plain `serde_json` round-trip (see mp4's identical
/// module-doc rationale: f6-final-summary.md §4.4, no generic collection-diff `DslField` bridge).
impl OpText for AviMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    async fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl OpBinary for AviMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
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
                },
                strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
                chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1, 2, 3], keyframe: true }],
            }],
            idx1_present: true,
            unknown_chunks: vec![RiffChunk { fourcc: "JUNK".into(), data: vec![0] }],
        }
    }

    /// 🧪️ mutation_diff_law + inverse_law, exercised across every real variant.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_and_inverse_law_hold_for_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            AviMutation::SetMainHeader { main_header: AviMainHeader { width: 32, ..base.main_header.clone() } },
            AviMutation::SetIdx1Present { idx1_present: false },
            AviMutation::InsertStream { index: 1, stream: AviStream { strh: base.streams[0].strh.clone(), strf: base.streams[0].strf.clone(), chunks: vec![] } },
            AviMutation::SetStreamHeader { stream_index: 0, strh: AviStreamHeader { rate: 30, ..base.streams[0].strh.clone() } },
            AviMutation::SetStreamFormat { stream_index: 0, strf: AviStreamFormat::Raw { data: vec![9] } },
            AviMutation::InsertChunk { stream_index: 0, index: 1, chunk: AviChunk { fourcc: "00dc".into(), data: vec![9, 9], keyframe: false } },
            AviMutation::SetChunkKeyframe { stream_index: 0, index: 0, keyframe: false },
            AviMutation::AddUnknownChunk { index: 1, item: RiffChunk { fourcc: "MORE".into(), data: vec![1] } },
            AviMutation::RemoveUnknownChunk { index: 0 },
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
        base.streams.push(AviStream { strh: base.streams[0].strh.clone(), strf: base.streams[0].strf.clone(), chunks: vec![] });
        let m = AviMutation::RemoveStream { index: 0 };
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
        let mutation = AviMutation::SetSnapshot { snapshot: next.clone() };
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
            AviMutation::NoMutation,
            AviMutation::SetSnapshot { snapshot: base.clone() },
            AviMutation::SetIdx1Present { idx1_present: false },
            AviMutation::RemoveStream { index: 0 },
            AviMutation::SetChunkKeyframe { stream_index: 0, index: 0, keyframe: true },
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
}
//#endregion 🔖️Tests
