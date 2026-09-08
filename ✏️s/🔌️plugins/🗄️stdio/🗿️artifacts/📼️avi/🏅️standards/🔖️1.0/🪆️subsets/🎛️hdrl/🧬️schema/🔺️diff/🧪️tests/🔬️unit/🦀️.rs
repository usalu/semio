
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
async fn field_sweep_covers_every_mutable_field() {
    let a = snap(vec![stream(vec![chunk(1), chunk(2)]), stream(vec![chunk(3)])]);
    let mut b = a.clone();
    b.main_header.width = 32;
    b.streams[0].chunks.remove(0);
    b.streams[0].chunks.push(chunk(9));
    b.streams[0].strl_extra.push(RiffChunk { fourcc: "vprp".into(), data: vec![7] });
    b.streams.remove(1);
    b.streams.push(stream(vec![chunk(5)]));
    b.idx1_present = false;
    b.unknown_chunks.push(RiffChunk { fourcc: "JUNK".into(), data: vec![1] });
    b.hdrl_extra.push(RiffChunk { fourcc: "JUNK".into(), data: vec![2] });

    let d = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &b);
    assert!(d.main_header.is_some());
    assert!(d.streams.is_some());
    assert!(d.idx1_present.is_some());
    assert!(d.unknown_chunks.is_some());
    assert!(d.hdrl_extra.is_some());
    assert_eq!(d.apply(&a).unwrap(), b);
    assert_eq!(<AviDiff as DiffAlgebra<AviSnapshot>>::between(&b, &a).apply(&b).unwrap(), a);
    assert!(<AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law_round_trips_through_apply() {
    let a = snap(vec![stream(vec![chunk(1), chunk(2)])]);
    let mut b = a.clone();
    b.streams[0].chunks[0].keyframe = !b.streams[0].chunks[0].keyframe;
    let d = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &b);
    let after = d.apply(&a).unwrap();
    assert_eq!(after, b);
    assert_eq!(d.inverse(&a).apply(&after).unwrap(), a);
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
    absorb_indexed(&mut d1, d2, absorb_chunk_diff, apply_chunk_diff_mut);
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
    absorb_indexed(&mut d1, d2, absorb_chunk_diff, apply_chunk_diff_mut);
    let combined = apply_indexed(&base, &d1, apply_chunk_diff);
    assert_eq!(combined, sequential);
    assert_eq!(combined.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn absorb_associativity_over_three_diffs() {
    let a = snap(vec![stream(vec![chunk(1), chunk(2)])]);
    let mut mid1 = a.clone();
    mid1.streams[0].chunks[0].keyframe = false;
    let mut mid2 = mid1.clone();
    mid2.streams.push(stream(vec![chunk(5)]));
    let mut after = mid2.clone();
    after.main_header.width = 999;

    let d1 = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &mid1);
    let d2 = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&mid1, &mid2);
    let d3 = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&mid2, &after);

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());
    let mut d23 = d2;
    d23.absorb(d3);
    let mut right = d1;
    right.absorb(d23);

    assert_eq!(left.apply(&a).unwrap(), after);
    assert_eq!(right.apply(&a).unwrap(), after);
    assert_eq!(left.apply(&a).unwrap(), right.apply(&a).unwrap());
}
