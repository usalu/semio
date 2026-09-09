use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> AviSnapshot {
    AviSnapshot {
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        main_header: AviMainHeader {
            micro_sec_per_frame: 100_000,
            max_bytes_per_sec: 1400,
            padding_granularity: 0,
            flags: 0x10,
            total_frames: 2,
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
                length: 2,
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
            chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1, 2, 3, 4], keyframe: true }, AviChunk { fourcc: "00dc".into(), data: vec![5, 6, 7, 8], keyframe: true }],
            strl_extra: vec![],
        }],
        idx1_present: true,
        unknown_chunks: vec![],
        hdrl_extra: vec![],
    }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips_via_real_avi_bytes() {
    let snap = sample_snapshot();
    let bytes = <AviSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <AviSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips_via_real_avi_bytes() {
    let snap = sample_snapshot();
    let text = <AviSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <AviSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_round_trips_through_real_codec() {
    // 🧭️ `..AviSnapshot::default()` gives `schema: ""`/`reserved: vec![]` (derived `Default`,
    // not the real codec's own normal form): `decode_avi` always stamps `schema` from
    // `STDIO_AVI_DOCUMENT_SCHEMA` and `avih`'s `dwReserved[4]` is always 4 real DWORDs on the
    // wire, so a snapshot claiming to round-trip through the real codec must start in that
    // codec's own normal form, not the bare struct-derive default.
    let snap = AviSnapshot { schema: STDIO_AVI_DOCUMENT_SCHEMA.into(), main_header: AviMainHeader { reserved: vec![0; 4], ..AviMainHeader::default() }, idx1_present: false, ..AviSnapshot::default() };
    let bytes = <AviSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <AviSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}
