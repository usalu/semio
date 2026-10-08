use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame() -> Mp3Frame {
    Mp3Frame {
        header: Mp3FrameHeader { mpeg_version_id: 3, layer: 1, protection_bit: true, bitrate_index: 9, sample_rate_index: 0, padding: false, private_bit: false, channel_mode: 3, mode_extension: 0, copyright: false, original: true, emphasis: 0 },
        payload: vec![0u8; 4],
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> Mp3Snapshot {
    Mp3Snapshot { id3v2: None, frames: vec![frame()], id3v1: None, ..Mp3Snapshot::default() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> Mp3Snapshot {
    Mp3Snapshot {
        id3v2: Some(Id3v2Tag { frames: vec![Id3Frame { id: "TIT2".into(), content: crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3Content::Text { values: vec!["x".into()] } }] }),
        frames: vec![frame(), frame()],
        id3v1: Some(Id3v1Tag::default()),
        ..Mp3Snapshot::default()
    }
}

//#endregion between_roundtrip_law

//#region absorb_law
#[semio_framework_async_macros::async_test]
async fn absorb_law_disjoint_and_lww_and_associativity() {
    let base = sweep_a();
    let d1 = diff_set_frames(vec![frame(), frame(), frame()]);
    let d2 = diff_set_id3v1(Some(Id3v1Tag::default()));
    let mut absorbed = d1.clone();
    absorbed.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&absorbed, &base).unwrap(), protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).unwrap()).unwrap());

    let d3 = diff_set_id3v2(Some(Id3v2Tag { frames: vec![] }));
    let d4 = diff_set_id3v2(None);
    let mut lww = d3.clone();
    lww.absorb(d4.clone());
    assert_eq!(lww.id3v2, Some(None));

    let da = diff_set_frames(vec![frame()]);
    let db = diff_set_id3v2(None);
    let dc = diff_set_id3v1(None);
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
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = Mp3Diff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = Mp3Diff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion diff_codec_text_binary_roundtrip_law
