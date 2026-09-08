
use super::*;
use protocol::DiffCodec;

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
        id3v2: Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![Id3Frame { id: "TIT2".into(), flags: 0, data: vec![0, b'x'] }] }),
        frames: vec![frame(), frame()],
        id3v1: Some(Id3v1Tag { raw: vec![b'T', b'A', b'G'] }),
        ..Mp3Snapshot::default()
    }
}

//#region field_sweep
/// 🧪️ `field_sweep`: `sweep_a`/`sweep_b` differ in EVERY mutable field, exercising both
/// tri-state directions (`Some(Some(_))` a→b, `Some(None)` b→a).
#[semio_framework_async_macros::async_test]
async fn field_sweep_between_covers_every_field() {
    let a = sweep_a();
    let b = sweep_b();
    let ab = Mp3Diff::between(&a, &b);
    assert!(matches!(ab.id3v2, Some(Some(_))));
    assert!(ab.frames.is_some());
    assert!(matches!(ab.id3v1, Some(Some(_))));
    assert_eq!(ab.apply(&a).unwrap(), b);

    let ba = Mp3Diff::between(&b, &a);
    assert_eq!(ba.id3v2, Some(None));
    assert!(ba.frames.is_some());
    assert_eq!(ba.id3v1, Some(None));
    assert_eq!(ba.apply(&b).unwrap(), a);

    assert!(Mp3Diff::between(&a, &a).is_empty());
}
//#endregion field_sweep

//#region between_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(Mp3Diff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(Mp3Diff::between(&b, &a).apply(&b).unwrap(), a);
}
//#endregion between_roundtrip_law

//#region absorb_law
#[semio_framework_async_macros::async_test]
async fn absorb_law_disjoint_and_lww_and_associativity() {
    let base = sweep_a();
    let d1 = diff_set_frames(vec![frame(), frame(), frame()]);
    let d2 = diff_set_id3v1(Some(Id3v1Tag { raw: vec![1, 2, 3] }));
    let mut absorbed = d1.clone();
    absorbed.absorb(d2.clone());
    assert_eq!(absorbed.apply(&base).unwrap(), d2.apply(&d1.apply(&base).unwrap()).unwrap());

    let d3 = diff_set_id3v2(Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![] }));
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
    assert_eq!(left.apply(&base).unwrap(), dc.apply(&db.apply(&da.apply(&base).unwrap()).unwrap()).unwrap());
}
//#endregion absorb_law

//#region inverse_law
#[semio_framework_async_macros::async_test]
async fn inverse_law_diff_level() {
    let base = sweep_a();
    let d = Mp3Diff::between(&base, &sweep_b());
    let applied = d.apply(&base).unwrap();
    let undone = d.inverse(&base).apply(&applied).unwrap();
    assert_eq!(undone, base);
}
//#endregion inverse_law

//#region diff_codec_text_binary_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    let cases = vec![Mp3Diff::default(), Mp3Diff::between(&a, &b), Mp3Diff::between(&b, &a), diff_set_id3v2(None), diff_set_id3v1(None), diff_set_frames(vec![])];
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
