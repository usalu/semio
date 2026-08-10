//! 🧪️ Tests for example `💃️dancing` — the real animated GIF89a fixture. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's core acceptance bar:
//! (a) real-file decode invariants, (b) decode→encode→decode snapshot equality, (c) analyzer→
//! builder round trip using ONLY the 89a builder's typed constructors.

use semio_framework_plugin::{AnalyzeSource, ArtifactAnalyzer, ArtifactBuilder};
use crate::artifacts::gif::standards::v89a::engine::{decode_gif, encode_gif};
use crate::artifacts::gif::standards::v89a::subsets::any::analyzer::GifAnalyzer;
use crate::artifacts::gif::standards::v89a::subsets::any::builder::GifBuilder;

const DANCING_GIF_BYTES: &[u8] = include_bytes!("../🖼️assets/🖼️dancing.gif");

/// 🧪️ (a) Real-file decode: non-trivial invariants that a black-frame or invalid-file stub could
/// never satisfy — frame count > 1 (it's animated), loop count matches the file's NETSCAPE2.0
/// extension (confirmed via independent byte-level inspection: `Some(0)`, infinite loop), and at
/// least one frame has a non-zero delay.
#[test]
fn decodes_real_fixture_with_nontrivial_invariants() {
    let snapshot = decode_gif(DANCING_GIF_BYTES).expect("dancing.gif must decode via the real 89a codec");
    assert!(snapshot.frames.len() > 1, "dancing.gif is animated, must decode to more than one frame");
    assert_eq!(snapshot.width, 800);
    assert_eq!(snapshot.height, 800);
    assert_eq!(snapshot.loop_count, Some(0), "NETSCAPE2.0 loop count must be decoded (0 == infinite)");
    assert!(snapshot.frames.iter().any(|f| f.delay_cs > 0), "at least one frame must have a real (non-zero) delay");
    for frame in &snapshot.frames {
        assert_eq!(frame.rgba.len(), (frame.width as usize) * (frame.height as usize) * 4, "frame rgba must be fully populated, not a black/empty stub");
    }
}

/// 🧪️ (b) decode→encode→decode snapshot equality — the fixtures-section acceptance bar (model
/// equality after a second round trip, not necessarily byte-identical GIF output).
#[test]
fn decode_encode_decode_round_trip_is_stable() {
    let once = decode_gif(DANCING_GIF_BYTES).expect("first decode");
    let reencoded = encode_gif(&once).expect("re-encode decoded snapshot");
    let twice = decode_gif(&reencoded).expect("second decode");
    assert_eq!(once, twice, "decode(encode(decode(x))) must equal decode(x)");
}

/// 🧪️ (c) THE core acceptance test: analyzer output drives ONLY the 89a builder's typed
/// constructors (`GifBuilder::new`/`add_frame`/`set_loop_count`) to reconstruct an equivalent
/// document -- never `from_snapshot`/`SetSnapshot` as a shortcut -- then the two REAL
/// `GifAnalyzer::analyze` outputs (original fixture vs. rebuilt-then-re-encoded) must match.
#[test]
fn analyzer_builder_round_trip_matches() {
    let original = decode_gif(DANCING_GIF_BYTES).expect("decode real fixture");
    let packed_original = <crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot as store::ArtifactPack>::encode_pack(&original);
    let analysis_a = GifAnalyzer::analyze(&[AnalyzeSource::Binary(&packed_original)]);
    let parts_a = analysis_a.parts.snapshot.clone().expect("analyzer must report a snapshot for a valid real fixture");

    let mut builder = GifBuilder::new(parts_a.width, parts_a.height);
    for frame in &parts_a.frames {
        builder = builder.add_frame(frame.clone());
    }
    builder = builder.set_loop_count(parts_a.loop_count);
    let rebuilt = builder.build().expect("typed-constructor rebuild must succeed");

    let packed_rebuilt = <crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot as store::ArtifactPack>::encode_pack(&rebuilt);
    let analysis_b = GifAnalyzer::analyze(&[AnalyzeSource::Binary(&packed_rebuilt)]);
    let parts_b = analysis_b.parts.snapshot.clone().expect("analyzer must report a snapshot for the rebuilt document");

    assert_eq!(parts_a, parts_b, "analyzer(original) and analyzer(builder-rebuilt-from-analyzer-output) must match");
    assert_eq!(parts_b.frames.len(), 54, "rebuild must preserve every frame from the real fixture");
}
