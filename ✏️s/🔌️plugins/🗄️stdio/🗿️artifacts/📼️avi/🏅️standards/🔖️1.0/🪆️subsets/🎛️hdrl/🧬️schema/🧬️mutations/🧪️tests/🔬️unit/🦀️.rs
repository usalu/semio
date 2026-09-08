
use super::*;
use crate::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA;

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
