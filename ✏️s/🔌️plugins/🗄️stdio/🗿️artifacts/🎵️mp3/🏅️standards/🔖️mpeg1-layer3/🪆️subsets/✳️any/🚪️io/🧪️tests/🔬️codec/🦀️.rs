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
    assert_eq!(metadata::syncsafe(&bytes[6..10]).unwrap(),257);
    assert!(metadata::decode_id3v2(&bytes).is_err());
}

#[semio_framework_async_macros::async_test]
async fn finds_a_synthetic_mpeg1_layer3_frame_sync() {
    let bytes = [0x00, 0x00, 0xFF, 0xFB, 0x90, 0x00];
    assert_eq!(find_frame_sync(&bytes), Some(2));
}

#[semio_framework_async_macros::async_test]
async fn no_id3v2_header_returns_none() {
    assert!(metadata::decode_id3v2(b"not an id3 tag").is_err());
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
    assert_eq!(tag.frames.len(), 2, "TIT2 + TPE1");
    assert_eq!(tag.frames[0].id, "TIT2");
    assert_eq!(tag.frames[1].id, "TPE1");
    assert_eq!(tag.frames[0].content, Id3Content::Text{values:vec!["semio fixture".into()]});
    assert_eq!(tag.frames[1].content, Id3Content::Text{values:vec!["W0 handcraft".into()]});

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

    let re_encoded = encode_mp3(&decoded).unwrap();
    assert_eq!(re_encoded[3],4,"metadata emits canonical ID3v2.4");

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
    assert_eq!(cursor.emitted_bytes(), encoded.len() as u64);
    assert_eq!(encoded, encode_mp3(&decoded).unwrap());
    assert_eq!(decode_mp3(&encoded).unwrap(),decoded);
}

#[test]
fn playback_snapshot_retirement_preserves_original_observers_and_each_exact_grant() {
    use semio_framework_plugin::{ArtifactSnapshotDisposer, PluginLifecycleStep};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    for weak in [false, true] {
        let decoded = decode_mp3(include_bytes!("../../../🧫️fixtures/🔊️.mp3")).unwrap();
        let mut snapshot = Some(std::sync::Arc::new(decoded));
        let pointer = std::sync::Arc::as_ptr(snapshot.as_ref().unwrap());
        let strong = (!weak).then(|| snapshot.as_ref().unwrap().clone());
        let observer = weak.then(|| std::sync::Arc::downgrade(snapshot.as_ref().unwrap()));
        let mut disposer = playback::Mp3ExportSnapshotDisposer::default();
        let demand = disposer.retirement_demands(&snapshot, 0).unwrap();
        assert_eq!(demand.copy_bytes, std::mem::size_of::<Mp3Snapshot>());
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        assert_eq!(disposer.close_step(&mut snapshot, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap().progress().unwrap(), Default::default());
        assert!(matches!(disposer.close_step(&mut snapshot, grant).unwrap(), PluginLifecycleStep::AwaitingInput { .. }));
        assert_eq!(std::sync::Arc::as_ptr(snapshot.as_ref().unwrap()), pointer);
        if let Some(observer) = observer.as_ref() { assert_eq!(std::sync::Arc::as_ptr(&observer.upgrade().unwrap()), pointer); }
        drop(strong);
        drop(observer);
        let demand = disposer.retirement_demands(&snapshot, 0).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        assert_eq!(disposer.close_step(&mut snapshot, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - 1, ..grant }).unwrap().progress().unwrap(), Default::default());
        assert_eq!(std::sync::Arc::as_ptr(snapshot.as_ref().unwrap()), pointer);
        assert_eq!(disposer.close_step(&mut snapshot, RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant }).unwrap().progress().unwrap(), Default::default());
        assert_eq!(std::sync::Arc::as_ptr(snapshot.as_ref().unwrap()), pointer);
        let mut turns = 0;
        while !disposer.terminal_is_empty(&snapshot) {
            let demand = disposer.retirement_demands(&snapshot, 4_096).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(4_096), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let step = disposer.close_step(&mut snapshot, grant).unwrap();
            assert!(step.progress().unwrap().fits(grant));
            turns += 1;
            assert!(turns < 100_000);
        }
        assert!(turns > 10);
        eprintln!("[DEBUG] MP3 original observer weak={weak} header/payload closure turns={turns} terminal=true");
    }
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
    assert_eq!(id1.title, "");
    assert_eq!(id1.genre, Some(0));

    let re_encoded = encode_mp3(&decoded).unwrap();
    assert_eq!(re_encoded, bytes);
}
//#endregion 🔖️Id3v1Retention

/// 🎵️ Proves the actual MP3 native metadata oracle, logical carriers and compiled protocol.
#[test]
fn owned_fixture_publication_reports_canonical_logical_carriers() {
    playback_snapshot_retirement_preserves_original_observers_and_each_exact_grant();
    use store::{ArtifactDsl, ArtifactPack};
    use semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::project_mp3;
    let native = real_fixture();
    let snapshot = decode_mp3(&native).unwrap();
    let encoded = encode_mp3(&snapshot).unwrap();
    assert_eq!(project_mp3(&native).unwrap(), project_mp3(&encoded).unwrap());
    let text = snapshot.print_dsl();
    let binary = snapshot.encode_pack_with(&Default::default()).unwrap();
    assert_eq!(Mp3Snapshot::parse_dsl(&text).unwrap(), snapshot);
    assert_eq!(Mp3Snapshot::decode_pack_with(&binary, &Default::default()).unwrap(), snapshot);
    let factories = crate::native_codecs();
    assert_eq!(factories.len(), 1);
    let factory = &factories[0];
    let codec = (factory.codec)();
    let kind = (factory.kind)();
    let compiled = include_bytes!("../../💾️binary/📸️snapshot/📡️.protocol.semio");
    let current = std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();
    assert_eq!(current.as_slice(), compiled, "the live native receipt requires the current compiled protocol");
    let digest = semio_framework_hash::Sha256::digest(compiled);
    assert_ne!(codec.pack_schema_hash, [0; 32]);
    assert_eq!(kind.id, crate::MP3_ARTIFACT_SCHEMA_ID);
    assert_eq!(codec.schema, crate::STDIO_MP3_DOCUMENT_SCHEMA);
    assert_eq!(codec.extension, "semio");
    let hex = |bytes: &[u8]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let receipt = serde_json::json!({"schemaVersion":1,"artifactKind":kind.id,"artifactSchema":codec.schema,"factoryId":factory.id,"extension":codec.extension,"packSchemaHash":hex(&codec.pack_schema_hash),"protocolSourceSha256":hex(&digest)});
    eprintln!("[DEBUG] native-codec-publication={receipt}");
}
