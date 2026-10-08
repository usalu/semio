use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> WavSnapshot {
    WavSnapshot {
        fmt: WavFmt { audio_format: 1, channels: 1, sample_rate: 8000, byte_rate: 16000, block_align: 2, bits_per_sample: 16, ext: None },
        data: WavData::Pcm16(vec![0, 1, -1]),
        other_chunks: vec![RiffChunk { fourcc: "fact".into(), data: vec![1, 2, 3, 4], pad_byte: 0 }],
        chunk_order: vec![WavChunkRef::Format, WavChunkRef::Other(0), WavChunkRef::Samples],
        ..WavSnapshot::default()
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> WavSnapshot {
    WavSnapshot {
        fmt: WavFmt { audio_format: 3, channels: 2, sample_rate: 48000, byte_rate: 384000, block_align: 8, bits_per_sample: 32, ext: Some(vec![0xAA, 0xBB]) },
        data: WavData::Float32(vec![0.5, -0.5]),
        fmt_pad_byte: 0x11,
        data_pad_byte: 0x22,
        other_chunks: vec![RiffChunk { fourcc: "LIST".into(), data: b"INFO".to_vec(), pad_byte: 0 }],
        chunk_order: vec![WavChunkRef::Other(0), WavChunkRef::Format, WavChunkRef::Samples],
        ..WavSnapshot::default()
    }
}

//#endregion between_roundtrip_law

//#region absorb_law
#[semio_framework_async_macros::async_test]
async fn absorb_law_disjoint_and_lww_and_associativity() {
    let base = sweep_a();
    let d1 = diff_set_fmt(sweep_b().fmt);
    let d2 = diff_set_data(WavData::Raw(vec![9, 9]));
    let mut absorbed = d1.clone();
    absorbed.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&absorbed, &base).unwrap(), protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).unwrap()).unwrap());
    assert_eq!(absorbed.fmt, d1.fmt);
    assert_eq!(absorbed.data, d2.data);

    // Same field twice: last write wins.
    let d3 = diff_set_data(WavData::Raw(vec![1]));
    let d4 = diff_set_data(WavData::Raw(vec![2]));
    let mut lww = d3.clone();
    lww.absorb(d4.clone());
    assert_eq!(lww.data, Some(WavData::Raw(vec![2])));

    // Associativity over a triple.
    let da = diff_set_fmt(sweep_b().fmt);
    let db = diff_set_data(WavData::Pcm8(vec![7]));
    let dc = diff_set_other_chunks(&sweep_a(), vec![RiffChunk { fourcc: "cue ".into(), data: vec![], pad_byte: 0 }]);
    let mut left = da.clone();
    left.absorb(db.clone());
    left.absorb(dc.clone());
    let mut right_tail = db.clone();
    right_tail.absorb(dc.clone());
    let mut right = da.clone();
    right.absorb(right_tail);
    assert_eq!(left, right);
    assert_eq!(protocol::apply_diff(&left, &base).unwrap(), protocol::apply_diff(&dc, &protocol::apply_diff(&db, &protocol::apply_diff(&da, &base).unwrap()).unwrap()).unwrap());
}
//#endregion inverse_law

//#region diff_codec_text_binary_roundtrip_law
/// 🧪️ `DiffText::print_diff`/`parse_diff`/`encode_diff`/`decode_diff` round-trip — exercises
/// every field, `ext: None` AND `ext: Some(_)`, every `WavData` variant, and multi-chunk
/// `other_chunks`, plus the empty diff.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = WavDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = WavDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion diff_codec_text_binary_roundtrip_law
