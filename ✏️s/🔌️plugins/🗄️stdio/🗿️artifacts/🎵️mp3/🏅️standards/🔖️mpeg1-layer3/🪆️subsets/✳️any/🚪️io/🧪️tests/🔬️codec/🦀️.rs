use super::*;

/// 🌱 Real ID3v2.3.0 + 4× MPEG1 Layer III fixture — byte-identical to the artifact's own
/// `📚️examples/🎬️demo/🖼️assets/🔊️.mp3` (per ticket `fixtures/mp3/NOTES.md`), duplicated
/// here as a literal so the test doesn't reach across an emoji-path `include_bytes!`
/// boundary.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_fixture() -> Vec<u8> {
    include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🔊️.mp3").to_vec()
}

#[semio_framework_async_macros::async_test]
async fn detects_a_synthetic_id3v2_header() {
    let mut bytes = b"ID3".to_vec();
    bytes.extend_from_slice(&[0x03, 0x00, 0x00]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x02, 0x01]);
    let hdr = detect_id3v2_header(&bytes).expect("id3v2");
    assert_eq!(hdr.major_version, 3);
    assert_eq!(hdr.size, 257);
}

#[semio_framework_async_macros::async_test]
async fn finds_a_synthetic_mpeg1_layer3_frame_sync() {
    let bytes = [0x00, 0x00, 0xFF, 0xFB, 0x90, 0x00];
    assert_eq!(find_frame_sync(&bytes), Some(2));
}

#[semio_framework_async_macros::async_test]
async fn no_id3v2_header_returns_none() {
    assert!(detect_id3v2_header(b"not an id3 tag").is_none());
}

#[semio_framework_async_macros::async_test]
async fn frame_header_bit_layout_round_trips() {
    // FF FB 90 C4: MPEG1(11) Layer III(01) no-CRC(1), bitrate idx 9, sr idx 0, mono, original.
    // 128kbps/44100Hz gives a real 417-byte frame (`144*128000/44100`, no padding) —
    // `parse_frame_header` honestly bounds-checks the WHOLE frame against the buffer (not
    // just its 4 header bytes), so the buffer must actually be 417 bytes long.
    let header_bytes = [0xFF, 0xFB, 0x90, 0xC4];
    let mut bytes = header_bytes.to_vec();
    bytes.resize(417, 0);
    let (header, size) = parse_frame_header(&bytes, 0).expect("header");
    assert_eq!(header.mpeg_version_id, 0b11);
    assert_eq!(header.layer, 0b01);
    assert!(header.protection_bit);
    assert_eq!(header.bitrate_index, 9);
    assert_eq!(header.sample_rate_index, 0);
    assert!(!header.padding);
    assert_eq!(header.channel_mode, 0b11);
    assert!(header.original);
    assert_eq!(size, 417);
    assert_eq!(encode_frame_header(&header), header_bytes);
}

//#region codec_retention_law
/// 🧪️ `codec_retention_law`: mp3's honest boundary is the container level — frame COUNT and
/// every typed header field must round-trip exactly, and the full byte stream (incl. opaque
/// payload bytes) must re-encode byte-identical, even though the payload itself is never
/// Huffman-decoded.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let fixture = real_fixture();
    let decoded = decode_mp3(&fixture).expect("decode real fixture");

    let tag = decoded.id3v2.as_ref().expect("id3v2 tag present");
    assert_eq!(tag.major_version, 3);
    assert_eq!(tag.minor_version, 0);
    assert_eq!(tag.frames.len(), 2, "TIT2 + TPE1");
    assert_eq!(tag.frames[0].id, "TIT2");
    assert_eq!(tag.frames[1].id, "TPE1");
    assert_eq!(String::from_utf8_lossy(&tag.frames[0].data[1..]), "semio fixture");
    assert_eq!(String::from_utf8_lossy(&tag.frames[1].data[1..]), "W0 handcraft");

    assert_eq!(decoded.frames.len(), 4, "4 MPEG frames per fixtures/mp3/NOTES.md");
    for frame in &decoded.frames {
        assert_eq!(frame.header.mpeg_version_id, 0b11, "MPEG1");
        assert_eq!(frame.header.layer, 0b01, "Layer III");
        assert!(frame.header.protection_bit, "no CRC");
        assert_eq!(frame.header.bitrate_index, 9, "128kbps");
        assert_eq!(frame.header.sample_rate_index, 0, "44100Hz");
        assert!(!frame.header.padding);
        assert_eq!(frame.header.channel_mode, 0b11, "mono");
        assert!(frame.header.original);
        assert_eq!(frame.payload.len(), 413, "417 - 4 header bytes");
    }
    assert!(decoded.id3v1.is_none(), "fixture has no trailing ID3v1 tag");

    let re_encoded = encode_mp3(&decoded);
    assert_eq!(re_encoded, fixture, "encode(decode(real fixture)) must be byte-identical");

    let re_decoded = decode_mp3(&re_encoded).expect("decode re-encoded");
    assert_eq!(re_decoded, decoded);
}
//#endregion codec_retention_law

#[semio_framework_async_macros::async_test]
async fn incremental_cursor_yields_bounded_pages_matching_the_real_fixture() {
    let fixture = include_bytes!("../../../🧫️fixtures/🔊️.mp3");
    let decoded = decode_mp3(fixture).expect("decode real LAME fixture");
    let mut cursor = Mp3EncodeCursor::new(&decoded);
    let mut encoded = Vec::new();
    let mut progress_steps = 0usize;
    let mut chunks = 0usize;
    loop {
        match cursor.advance(&decoded, 257).expect("bounded cursor advance") {
            Mp3EncodeAdvance::Progress => progress_steps += 1,
            Mp3EncodeAdvance::Chunk(chunk) => {
                assert!(!chunk.is_empty());
                assert!(chunk.len() <= 257);
                chunks += 1;
                encoded.extend_from_slice(&chunk);
            }
            Mp3EncodeAdvance::Complete => break,
        }
    }
    assert!(progress_steps > 1, "ID3 measurement must itself yield");
    assert!(chunks > 1, "the real fixture must cross multiple output grants");
    assert_eq!(cursor.emitted_bytes(), fixture.len() as u64);
    assert_eq!(encoded, fixture);
    assert_eq!(encoded, encode_mp3(&decoded));
}

#[semio_framework_async_macros::async_test]
async fn playback_snapshot_retirement_obeys_each_byte_and_item_grant() {
    use semio_framework_plugin::{ArtifactSnapshotDisposer, PluginCloseStep};
    let decoded = decode_mp3(include_bytes!("../../../🧫️fixtures/🔊️.mp3")).expect("decode real LAME fixture");
    let mut snapshot = Some(std::sync::Arc::new(decoded));
    let mut disposer = playback::Mp3ExportSnapshotDisposer::default();
    assert!(matches!(disposer.close_step(&mut snapshot, 0, 0).expect("zero grant"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
    let mut steps = 0usize;
    while !disposer.terminal_is_empty(&snapshot) {
        match disposer.close_step(&mut snapshot, 1, 4_096).expect("bounded retirement") {
            PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 4_096);
            }
            PluginCloseStep::Complete => {}
            other => panic!("unshared MP3 retirement must progress: {other:?}"),
        }
        steps += 1;
        assert!(steps < 10_000);
    }
    assert!(steps > 10, "the real fixture must retire over many bounded grants");
}

//#region 🔖️Id3v1Retention
#[semio_framework_async_macros::async_test]
async fn id3v1_trailer_round_trips() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[0xFF, 0xFB, 0x90, 0xC4]);
    bytes.extend(std::iter::repeat(0u8).take(413));
    let mut tag = b"TAG".to_vec();
    tag.resize(128, 0);
    bytes.extend_from_slice(&tag);

    let decoded = decode_mp3(&bytes).expect("decode");
    assert_eq!(decoded.frames.len(), 1);
    let id1 = decoded.id3v1.as_ref().expect("id3v1 tag present");
    assert_eq!(id1.raw.len(), 128);
    assert_eq!(&id1.raw[0..3], b"TAG");

    let re_encoded = encode_mp3(&decoded);
    assert_eq!(re_encoded, bytes);
}
//#endregion 🔖️Id3v1Retention
