use crate::{standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot, standards::mpeg1_layer3::subsets::any::schema::snapshot::STDIO_MP3_DOCUMENT_SCHEMA};
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3Content;
use crate::standards::mpeg1_layer3::subsets::any::io::{parse_frame_header, encode_frame_header};
use crate::standards::mpeg1_layer3::subsets::any::io::*;
use semio_framework_test_fixtures::fixture_path;

/// 🌱 The artifact's own shipped demo stream (`📚️examples/🎬️demo/🖼️assets/🔊️.mp3`), embedded with `include_bytes!` so the test reads exactly the bytes the editor ships.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_fixture() -> Vec<u8> {
    include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🔊️.mp3").to_vec()
}

#[semio_framework_async_macros::async_test]
async fn a_synthetic_id3v2_header_with_a_syncsafe_size_but_no_body_is_refused() {
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
async fn a_buffer_without_an_id3_marker_is_refused() {
    assert!(metadata::decode_id3v2(b"not an id3 tag").is_err());
}

#[semio_framework_async_macros::async_test]
async fn frame_header_bit_layout_round_trips() {
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
    let fixture = include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3");
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
fn playback_snapshot_installed_supplier_preserves_actual_owned_payload(){
 use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,RetainedCloneGrant};use semio_framework_value_factory::FactorySharedRetirement;use std::sync::Arc;
 let snapshot=Arc::new(decode_mp3(include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3")).unwrap());let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<Mp3Snapshot>>=Arc::new(semio_framework_value::OwnedValueRetirementFactory::<Mp3Snapshot>::default());let mut owner=FactorySharedRetirement::new(snapshot,factory);
 for turn in 0..100000{if owner.terminal_is_empty(){break}let demand=owner.next_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(4096),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};assert_eq!(owner.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Default::default());assert!(owner.close_step(grant).unwrap().progress().fits(grant));assert!(turn<99999)}assert!(owner.terminal_is_empty());
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
    playback_snapshot_installed_supplier_preserves_actual_owned_payload();
    use store::{snapshot::ArtifactDsl, snapshot::ArtifactPack};
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
    let current = std::fs::read(fixture_path!("../../../../../🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();
    assert_eq!(current.as_slice(), compiled, "the live native receipt requires the current compiled protocol");
    let digest = semio_framework_hash::Sha256::digest(compiled);
    assert_ne!(codec.pack_schema_hash, [0; 32]);
    assert_eq!(kind.id, crate::MP3_ARTIFACT_SCHEMA_ID);
    assert_eq!(codec.schema, crate::STDIO_MP3_DOCUMENT_SCHEMA);
    assert_eq!(codec.extension, "semio");
    let hex = |bytes: &[u8]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let receipt = serde_json::json!({"schemaVersion":1,"artifactKind":kind.id,"artifactSchema":codec.schema,"factoryId":factory.id,"extension":codec.extension,"packSchemaHash":hex(&codec.pack_schema_hash),"protocolSourceSha256":hex(&digest)});
    eprintln!("native-codec-publication={receipt}");
}

/// 🔬️ Runs the independent `ffprobe` binary over `bytes` and returns its exact `key=value` report; a missing `ffprobe` is a hard failure because this cross-check is required.
fn ffprobe_fields(bytes: &[u8], suffix: &str, arguments: &[&str]) -> std::collections::BTreeMap<String, String> {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!("semio-{}-{}.{suffix}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
    std::fs::write(&path, bytes).expect("temporary stream is writable");
    let output = std::process::Command::new("ffprobe").args(["-v", "error", "-of", "default=noprint_wrappers=1"]).args(arguments).arg(&path).output().expect("ffprobe is required for this cross-check and must be installed");
    let _ = std::fs::remove_file(&path);
    assert!(output.status.success(), "ffprobe rejected the stream: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout).lines().filter_map(|line| line.split_once('=')).map(|(key, value)| (key.to_string(), value.to_string())).collect()
}

#[test]
fn ffprobe_counts_exactly_the_frames_left_by_a_frame_boundary_trim() {
    let fixture = include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3");
    let mut trimmed = decode_mp3(fixture).expect("decode real LAME fixture");
    let kept = 100usize;
    trimmed.frames.truncate(kept);
    let bytes = encode_mp3(&trimmed).expect("trimmed stream encodes");
    assert_eq!(decode_mp3(&bytes).expect("trimmed stream reopens"), trimmed);
    let report = ffprobe_fields(&bytes, "mp3", &["-count_packets", "-select_streams", "a:0", "-show_entries", "stream=codec_name,sample_rate,nb_read_packets"]);
    assert_eq!(report["codec_name"], "mp3");
    assert_eq!(report["sample_rate"], "44100");
    assert_eq!(report["nb_read_packets"], kept.to_string());
}

#[test]
fn a_title_edit_retains_every_other_id3_frame_and_the_audio_frames_and_ffprobe_reads_the_new_title() {
    let fixture = include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3");
    let source = decode_mp3(fixture).expect("decode real LAME fixture");
    let mut edited = source.clone();
    let tag = edited.id3v2.as_mut().expect("the real fixture carries an ID3v2 tag");
    let title = tag.frames.iter_mut().find(|frame| frame.id == "TIT2").expect("the real fixture carries TIT2");
    title.content = Id3Content::Text { values: vec!["Ümläut Titel 日本".into()] };
    let bytes = encode_mp3(&edited).expect("edited stream encodes");
    let reopened = decode_mp3(&bytes).expect("edited stream reopens");
    assert_eq!(reopened, edited);
    assert_eq!(reopened.frames, source.frames, "audio frames are untouched by a tag edit");
    let before = &source.id3v2.as_ref().unwrap().frames;
    let after = &reopened.id3v2.as_ref().unwrap().frames;
    assert_eq!(before.len(), after.len());
    for (old, new) in before.iter().zip(after) {
        assert_eq!(old.id, new.id, "frame order is retained");
        if old.id != "TIT2" {
            assert_eq!(old.content, new.content, "frame {} is retained verbatim", old.id);
        }
    }
    let report = ffprobe_fields(&bytes, "mp3", &["-show_entries", "format_tags=title"]);
    assert_eq!(report["TAG:title"], "Ümläut Titel 日本");
}

#[test]
fn a_patch_frames_cut_through_the_typed_mutation_leaves_exactly_the_remaining_packets_for_ffprobe_and_undoes_exactly() {
    use crate::standards::mpeg1_layer3::subsets::any::schema::mutations::{patch_frames::PatchFrames, Mp3Mutation};
    use protocol::ColdMutation;
    let fixture = include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3");
    let source = decode_mp3(fixture).expect("decode real LAME fixture");
    let cut = Mp3Mutation::PatchFrames(PatchFrames { index: 100, remove_count: 300, frames: Vec::new() });
    let mut edited = source.clone();
    crate::apply_mutation(&mut edited, &cut);
    assert_eq!(edited.frames.len(), source.frames.len() - 300);
    assert_eq!(edited.frames[..100], source.frames[..100]);
    assert_eq!(edited.frames[100..], source.frames[400..]);
    let bytes = encode_mp3(&edited).expect("cut stream encodes");
    assert_eq!(decode_mp3(&bytes).expect("cut stream reopens"), edited);
    let report = ffprobe_fields(&bytes, "mp3", &["-count_packets", "-select_streams", "a:0", "-show_entries", "stream=codec_name,nb_read_packets"]);
    assert_eq!(report["codec_name"], "mp3");
    assert_eq!(report["nb_read_packets"], edited.frames.len().to_string());
    let mut undone = edited;
    for inverse in <Mp3Mutation as ColdMutation<Mp3Snapshot>>::inverse(&cut, &source).expect("cut inverse").into_iter().rev() {
        crate::apply_mutation(&mut undone, &inverse);
    }
    assert_eq!(undone, source);
}

/// 📖️ Every authored grammar and protocol of the diff and mutation facets parses in the semio dialects they are written in.
#[test]
fn the_diff_and_mutation_grammars_and_protocols_parse_in_their_dialects() {
    for (name, source) in [
        ("text diff grammar", include_str!("../../📝️text/🔺️diff/📖️.grammar.semio")),
        ("text mutation grammar", include_str!("../../📝️text/🧬️mutations/📖️.grammar.semio")),
    ] {
        semio_framework_dsl::parse_grammar(source).unwrap_or_else(|error| panic!("{name} must parse: {error:?}"));
    }
    for (name, source) in [
        ("binary diff protocol", include_str!("../../💾️binary/🔺️diff/📡️.protocol.semio")),
        ("binary mutation protocol", include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio")),
    ] {
        semio_framework_dsl::parse_protocol(source).unwrap_or_else(|error| panic!("{name} must parse: {error:?}"));
    }
}
