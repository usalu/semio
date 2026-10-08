use super::*;
use crate::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn chunk(n: u8) -> AviChunk {
    AviChunk { fourcc: "00dc".into(), data: vec![n], keyframe: n % 2 == 0 }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn stream(chunks: Vec<AviChunk>) -> AviStream {
    AviStream {
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
            length: chunks.len() as u32,
            suggested_buffer_size: 0,
            quality: -1,
            sample_size: 0,
            rc_frame_left: 0,
            rc_frame_top: 0,
            rc_frame_right: 16,
            rc_frame_bottom: 16,
            rc_frame_width: 16,
            strh_extra: vec![],
        },
        strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 0, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
        chunks,
        strl_extra: vec![],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap(streams: Vec<AviStream>) -> AviSnapshot {
    AviSnapshot {
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        main_header: AviMainHeader {
            micro_sec_per_frame: 100_000,
            max_bytes_per_sec: 0,
            padding_granularity: 0,
            flags: 0x10,
            total_frames: 0,
            initial_frames: 0,
            streams: streams.len() as u32,
            suggested_buffer_size: 0,
            width: 16,
            height: 16,
            reserved: vec![0, 0, 0, 0],
        },
        streams,
        idx1_present: true,
        unknown_chunks: vec![],
        hdrl_extra: vec![],
    }
}

#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_matches_sequential() {
    let base: Vec<AviChunk> = vec![chunk(1), chunk(2)];
    let f = AviChunk { fourcc: "00dc".into(), data: vec![0xAA], keyframe: true };
    let mut d1: AviChunksDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 2, item: f.clone() }] };
    let mid = apply_indexed(&base, &d1, apply_chunk_diff);
    let d2: AviChunksDiff = IndexedDiff { removed: vec![0], modified: vec![], added: vec![] };
    let after = apply_indexed(&mid, &d2, apply_chunk_diff);
    let sequential = after.clone();
    absorb_indexed(&mut d1, d2, absorb_chunk_rows, apply_chunk_diff_mut);
    assert_eq!(apply_indexed(&base, &d1, apply_chunk_diff), sequential);
}

#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let base: Vec<AviChunk> = vec![chunk(1)];
    let f = AviChunk { fourcc: "00dc".into(), data: vec![0xAA], keyframe: true };
    let g = AviChunk { fourcc: "00dc".into(), data: vec![0xBB], keyframe: false };
    let mut d1: AviChunksDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: f }] };
    let mid = apply_indexed(&base, &d1, apply_chunk_diff);
    let d2: AviChunksDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: g }] };
    let after = apply_indexed(&mid, &d2, apply_chunk_diff);
    let sequential = after.clone();
    absorb_indexed(&mut d1, d2, absorb_chunk_rows, apply_chunk_diff_mut);
    let combined = apply_indexed(&base, &d1, apply_chunk_diff);
    assert_eq!(combined, sequential);
    assert_eq!(combined.len(), 3);
}

