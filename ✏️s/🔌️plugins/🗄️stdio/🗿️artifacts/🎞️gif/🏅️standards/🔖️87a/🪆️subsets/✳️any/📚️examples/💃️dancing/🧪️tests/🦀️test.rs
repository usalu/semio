//! 🧪️ Tests for example `💃️dancing` — the real animated GIF89a fixture. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's core acceptance bar:
//! (a) real-file decode invariants, (b) decode→encode→decode snapshot equality, (c) analyzer→
//! builder round trip using ONLY the 89a builder's typed constructors. (d)/(e) ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING's inference laws,
//! exercised against this same real fixture.

use crate::artifacts::gif::standards::v89a::engine::{decode_gif, encode_gif};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::inferences::GifInference;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::GifAnalyzer;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::GifBuilderConstruction as GifBuilder;
use protocol::Inference;
use semio_framework_plugin::{AnalyzeSource, ArtifactBuilder};

const DANCING_GIF_BYTES: &[u8] = include_bytes!("../🖼️assets/🖼️dancing.gif");

/// 🧪️ (a) Real-file decode: non-trivial invariants that a black-frame or invalid-file stub could
/// never satisfy — frame count > 1 (it's animated), loop count matches the file's NETSCAPE2.0
/// extension (confirmed via independent byte-level inspection: `Some(0)`, infinite loop), and at
/// least one frame has a non-zero delay.
#[semio_framework_async_macros::async_test]
async fn decodes_real_fixture_with_nontrivial_invariants() {
    let snapshot = decode_gif(DANCING_GIF_BYTES).expect("dancing.gif must decode via the real 89a codec");
    assert!(snapshot.frames.len() > 1, "dancing.gif is animated, must decode to more than one frame");
    assert_eq!(snapshot.width, 800);
    assert_eq!(snapshot.height, 800);
    assert_eq!(snapshot.loop_count, Some(0), "NETSCAPE2.0 loop count must be decoded (0 == infinite)");
    assert!(snapshot.frames.iter().any(|f| f.delay_cs > 0), "at least one frame must have a real (non-zero) delay");
    for frame in &snapshot.frames {
        assert_eq!(frame.indices.len(), (frame.width as usize) * (frame.height as usize), "frame indices must be fully populated, not a black/empty stub");
        assert_eq!(frame.rgba(snapshot.gct.as_ref()).len(), frame.indices.len() * 4, "derived rgba() must decode every index");
    }
}

/// 🧪️ (b) decode→encode→decode snapshot equality — the fixtures-section acceptance bar (model
/// equality after a second round trip, not necessarily byte-identical GIF output).
#[semio_framework_async_macros::async_test]
async fn decode_encode_decode_round_trip_is_stable() {
    let once = decode_gif(DANCING_GIF_BYTES).expect("first decode");
    let reencoded = encode_gif(&once).expect("re-encode decoded snapshot");
    let twice = decode_gif(&reencoded).expect("second decode");
    assert_eq!(once, twice, "decode(encode(decode(x))) must equal decode(x)");
}

/// 🧪️ (c) THE core acceptance test: analyzer output drives ONLY the 89a builder's typed
/// constructors (`GifBuilder::new`/`add_frame`/`set_loop_count`) to reconstruct an equivalent
/// document -- never `from_snapshot`/`SetSnapshot` as a shortcut -- then the two REAL
/// `GifAnalyzer::analyze` outputs (original fixture vs. rebuilt-then-re-encoded) must match.
#[semio_framework_async_macros::async_test]
async fn analyzer_builder_round_trip_matches() {
    let original = decode_gif(DANCING_GIF_BYTES).expect("decode real fixture");
    let packed_original = <GifSnapshot as store::ArtifactPack>::encode_pack(&original);
    let analysis_a = GifAnalyzer::analyze(&[AnalyzeSource::Binary(&packed_original)]);
    let parts_a = analysis_a.parts.snapshot.clone().expect("analyzer must report a snapshot for a valid real fixture");

    let mut builder =
        GifBuilder::new(parts_a.width, parts_a.height).set_global_color_table(parts_a.gct.clone()).set_background_color_index(parts_a.background_color_index).set_pixel_aspect_ratio(parts_a.pixel_aspect_ratio).set_loop_count(parts_a.loop_count);
    for frame in &parts_a.frames {
        builder = builder.add_frame(frame.clone());
    }
    for comment in &parts_a.comments {
        builder = builder.add_comment(comment.clone());
    }
    for ext in &parts_a.app_extensions {
        builder = builder.add_app_extension(ext.clone());
    }
    let rebuilt = builder.build().expect("typed-constructor rebuild must succeed");

    let packed_rebuilt = <GifSnapshot as store::ArtifactPack>::encode_pack(&rebuilt);
    let analysis_b = GifAnalyzer::analyze(&[AnalyzeSource::Binary(&packed_rebuilt)]);
    let parts_b = analysis_b.parts.snapshot.clone().expect("analyzer must report a snapshot for the rebuilt document");

    assert_eq!(parts_a, parts_b, "analyzer(original) and analyzer(builder-rebuilt-from-analyzer-output) must match");
    assert_eq!(parts_b.frames.len(), 54, "rebuild must preserve every frame from the real fixture");
}

/// 🧪️ (d) `infer` on the real animated fixture is deterministic — two calls over the same
/// decoded snapshot produce byte-equal results.
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = decode_gif(DANCING_GIF_BYTES).expect("decode real fixture");
    assert_eq!(GifInference::infer(&snapshot), GifInference::infer(&snapshot));
}

/// 🧪️ (e) `infer(&GifSnapshot::default())` matches `GifInference::default()` — the hand-written
/// `Default` impl (`💡️inferences/🦀️component.rs`) must stay in lockstep with `infer` itself.
#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(GifInference::infer(&GifSnapshot::default()), GifInference::default());
}

/// 🧪️ `dimensions` on the real animated fixture matches the independently-verified 800x800
/// geometry `decodes_real_fixture_with_nontrivial_invariants` above already asserts, plus
/// `hasAlpha` agreeing exactly with a direct scan of every frame's `transparent_index`.
#[semio_framework_async_macros::async_test]
async fn dimensions_matches_real_fixture_geometry() {
    let snapshot = decode_gif(DANCING_GIF_BYTES).expect("decode real fixture");
    let inferred = GifInference::infer(&snapshot);
    assert_eq!(inferred.dimensions.width, 800);
    assert_eq!(inferred.dimensions.height, 800);
    assert_eq!(inferred.dimensions.pixel_count, 800 * 800);
    let any_transparent = snapshot.frames.iter().any(|frame| frame.transparent_index.is_some());
    assert_eq!(inferred.dimensions.has_alpha, any_transparent);
}
