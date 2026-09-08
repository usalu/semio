
use super::*;

/// 🎵️ The real committed fixture — 193,275 bytes of genuinely encoded MPEG-1 Layer III audio,
/// derived once from the repository's own real camera-captured video and encoded by `lame`, a
/// real third-party encoder. Provenance and the exact derivation command are in the case's
/// feature description and in the ticket's `mp3-fixture-derive/🐍️derive-real-mp3-fixture.py`.
/// The gherkin case reads the same file through `ctx.copy_fixture`.
fn fixture() -> Vec<u8> {
    include_bytes!("../../../🧫️fixtures/🔊️.mp3").to_vec()
}

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

fn object(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

#[test]
fn the_three_layers_of_the_real_fixture_are_found_where_the_specification_puts_them() {
    let input = fixture();
    assert_eq!(input.len(), 193_275, "the committed real fixture");
    let regions = layers::split(&input).unwrap();
    assert_eq!(regions.v2.len(), 179, "the ID3v2.3 region LAME wrote is a 10-byte header plus a 169-byte body");
    assert_eq!(regions.audio.len(), 193_096);
    assert_eq!(regions.v2.len() + regions.audio.len(), input.len(), "the three layers must partition the stream, leaving nothing unaccounted for");
    assert!(regions.v1.is_empty(), "`lame --id3v2-only` wrote no ID3v1 trailer, which is what keeps `set-id3v1` an ADD");
}

/// 🚶 The frame walk over a REAL encoded stream, which is what makes this different from a
/// walk over digital silence: 462 frames, and — the point — BOTH padding-slot values genuinely
/// occur, so `144·bitrate/rate + pad` is exercised on both of its branches. 128000/44100 is not
/// an integer, so a real CBR encoder MUST alternate the padding slot to hold the average rate;
/// a fixture whose frames all measure 417 bytes never tests the `+ pad` term at all.
#[test]
fn the_frame_walk_reads_every_real_mpeg1_layer3_frame_and_both_padding_slots() {
    let input = fixture();
    let frames = layers::walk(&layers::split(&input).unwrap().audio).unwrap();
    assert_eq!(frames.len(), 462);
    for frame in &frames {
        assert_eq!((frame.mpeg_version_id, frame.layer), (3, 1), "MPEG1 Layer III");
        assert_eq!((frame.bitrate_kbps, frame.sample_rate_hz), (128, 44_100));
        assert_eq!(frame.channel_mode, 3, "mono");
        assert_eq!(frame.size, if frame.padding { 418 } else { 417 }, "144·128000/44100 = 417, plus the padding slot");
    }
    assert_eq!(frames.iter().filter(|frame| !frame.padding).count(), 20);
    assert_eq!(frames.iter().filter(|frame| frame.padding).count(), 442);
    assert_eq!(frames.iter().map(|frame| frame.size).sum::<usize>(), 193_096, "the walk must consume the audio region exactly");
}

/// 🏷️ The reference reads the tag a REAL encoder wrote, including two frames in encoding `1`
/// (UTF-16 with a byte-order mark) — the form a real-world writer emits and the previous
/// handcrafted fixture, which was ISO-8859-1 throughout, never exercised.
#[test]
fn the_reference_reads_the_text_frames_a_real_encoder_wrote() {
    let input = fixture();
    let projection = project_mp3(&input).unwrap();
    let v2 = projection.get("id3v2").unwrap();
    assert_eq!(v2.get("majorVersion").unwrap().clone(), Json::Number(3.0));
    let frame = |id: &str, text: &str| object(vec![("id", Json::String(id.to_string())), ("text", Json::String(text.to_string()))]);
    assert_eq!(v2.array("frames"), vec![frame("TSSE", "LAME 64bits version 3.100 (http://lame.sf.net)"), frame("TIT2", "Bauen mit Bestand (Ausschnitt)"), frame("TPE1", "semio"), frame("TLEN", "12000"),]);
    assert_eq!(projection.get("id3v1").unwrap().clone(), Json::Null);
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let input = fixture();
    assert_eq!(oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).unwrap(), input);
}

/// 🧾️ The case's OWN `Examples` rows, transcribed in the feature file's order. Checking the
/// laws against the parameters the scenarios actually carry is the point — a row whose params
/// address nothing would report green while testing nothing, and the runner never dispatches an
/// observability check of its own. `set-frames`'s `take` of 231 truncates at the midpoint of a
/// 462-frame stream and `set-snapshot`'s take of 3 crosses the first padding-slot change
/// (frames 0 and 1 are 417 bytes, frame 2 is 418), so both land on real offset arithmetic
/// rather than on the head of the region.
fn feature_example_rows() -> Vec<Json> {
    let v1 = |title: &str| {
        object(vec![
            ("title", Json::String(title.to_string())),
            ("artist", Json::String("semio".to_string())),
            ("album", Json::String(String::new())),
            ("year", Json::String("2026".to_string())),
            ("comment", Json::String(String::new())),
            ("genreId", Json::Number(12.0)),
        ])
    };
    let text = |id: &str, value: &str| object(vec![("id", Json::String(id.to_string())), ("text", Json::String(value.to_string()))]);
    vec![
        spec("no-mutation", Json::Object(vec![])),
        spec("set-snapshot", object(vec![("text", Json::Array(vec![text("TALB", "replaced wholesale")])), ("take", Json::Number(3.0)), ("v1", v1("snapshot"))])),
        spec("set-id3v2", object(vec![("text", Json::Array(vec![text("TIT2", "renamed by the oracle"), text("TPE1", "semio")]))])),
        spec("set-frames", object(vec![("take", Json::Number(231.0))])),
        spec("set-id3v1", object(vec![("v1", v1("added trailer"))])),
    ]
}

#[test]
fn every_kind_is_observable_and_its_own_inverse_restores_the_projection() {
    let input = fixture();
    let original = project_mp3(&input).unwrap();
    for case in feature_example_rows() {
        let kind = case.str("kind");
        let mutated = oracle_apply_mutation(&input, &case).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
        let after = project_mp3(&mutated).unwrap();
        if kind != "no-mutation" {
            assert_ne!(after, original, "{kind} left the projection unchanged — a mutation that is not observable proves nothing");
        }
        let inverse = oracle_inverse_spec(&input, &case).unwrap();
        let restored = oracle_apply_mutation(&mutated, &inverse).unwrap_or_else(|error| panic!("{kind} inverse failed: {error}"));
        assert_eq!(project_mp3(&restored).unwrap(), original, "applying {kind} and then its own inverse must restore the original projection");
    }
}

#[test]
fn the_round_trip_preserves_the_projection_without_handing_the_input_back() {
    let input = fixture();
    let output = oracle_round_trip(&input).unwrap();
    assert_ne!(output, input, "id3's writer re-derives the tag region, so a bit-identical result would mean nothing was parsed");
    assert_eq!(project_mp3(&output).unwrap(), project_mp3(&input).unwrap());
}

#[test]
fn an_unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(&fixture(), &spec("set-bitrate", Json::Object(vec![]))).is_err());
    assert!(oracle_apply_mutation(&fixture(), &spec("set-id3v2", Json::Object(vec![]))).unwrap_err().contains("no `text`"));
}

/// 🏷️ `KINDS` must equal the committed catalog AND the committed production vocabulary. The
/// framework never parses Rust, so the catalog is what the contract gate counts against; this
/// reads both files as text and fails the moment any of the three drift apart.
#[test]
fn kinds_match_the_catalog_and_the_vocabulary() {
    let manifest = include_str!("../../🔣️.json");
    let vocabulary = include_str!("../../../🧬️schema/🧬️mutations/🦀️.rs");
    let variants = ["SetSnapshot", "SetId3v2", "SetFrames", "SetId3v1"];
    assert_eq!(KINDS.len(), variants.len());
    for (kind, variant) in KINDS.iter().zip(variants.iter()) {
        assert!(manifest.contains(&format!("\"{kind}\"")), "catalog is missing kind {kind:?}");
        assert!(vocabulary.contains(&format!("{variant} ")) || vocabulary.contains(&format!("{variant},")) || vocabulary.contains(&format!("{variant} {{")), "Mp3Mutation is missing variant {variant:?} for kind {kind:?}");
    }
    let feature = include_str!("../../../🧪️tests/🎛️mutate-mp3-mpeg1-layer3/🥒️.feature");
    for kind in KINDS {
        assert!(feature.contains(&format!("| {kind} ")) || feature.contains(&format!("| {kind}\t")) || feature.contains(&format!("| {kind}  ")), "the case's Examples table is missing kind {kind:?}");
    }
}
