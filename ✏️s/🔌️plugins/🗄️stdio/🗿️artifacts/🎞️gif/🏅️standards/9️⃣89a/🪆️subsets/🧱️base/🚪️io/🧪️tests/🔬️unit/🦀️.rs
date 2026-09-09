use super::*;
use crate::standards::v89a::subsets::any::schema::demo_gif_snapshot;

/// 📐️ Pads a quantized palette to the on-disk power-of-two size `write_color_table` would pad
/// it to anyway — so freshly-constructed test fixtures are already disk-canonical and an exact
/// `decoded == snap` round-trip assertion is meaningful (a non-power-of-two-length table is a
/// real, documented, one-way encode normalization — see `validated_color_table_size_field`'s
/// doc comment — not something `decode(encode(x))` can ever undo for arbitrary `x`).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn pad_to_disk_size(mut colors: Vec<codec::Rgb>) -> Vec<codec::Rgb> {
    let size_field = codec::color_table_size_field(colors.len());
    let target = 1usize << (size_field as usize + 1);
    while colors.len() < target {
        colors.push([0, 0, 0]);
    }
    colors
}

/// 🧪️ Builds a real, lossless `GifFrame` (LCT + indices, no GCT) from a synthetic RGBA pattern
/// via `quantize_rgba` — this test helper stays byte-level while the codec itself now writes
/// whatever palette + indices are already in the snapshot, never re-quantizing.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame(left: u32, top: u32, width: u32, height: u32, base_color: [u8; 3], delay_cs: u16, disposal: GifDisposal, transparent_corner: bool) -> GifFrame {
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
        for x in 0..width {
            let o = ((y * width + x) * 4) as usize;
            let on = (x + y) % 3 == 0;
            if transparent_corner && x == 0 && y == 0 {
                rgba[o..o + 4].copy_from_slice(&[0, 0, 0, 0]);
                continue;
            }
            rgba[o] = if on { base_color[0] } else { base_color[0].wrapping_add(40) };
            rgba[o + 1] = if on { base_color[1] } else { base_color[1].wrapping_add(40) };
            rgba[o + 2] = if on { base_color[2] } else { base_color[2].wrapping_add(40) };
            rgba[o + 3] = 255;
        }
    }
    let (palette, indices, transparent_index) = codec::quantize_rgba(&rgba).expect("quantize");
    GifFrame { left, top, width, height, interlace: false, lct: Some(color_table_from_bytes(pad_to_disk_size(palette), false)), indices, delay_cs, disposal, transparent_index, user_input: false, plain_text: None }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> GifSnapshot {
    GifSnapshot {
        schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width: 12,
        height: 10,
        loop_count: Some(0),
        frames: vec![frame(0, 0, 12, 10, [200, 20, 20], 50, GifDisposal::DoNotDispose, false), frame(2, 1, 6, 5, [20, 200, 20], 8, GifDisposal::RestoreToBackground, true), frame(0, 0, 12, 10, [20, 20, 200], 8, GifDisposal::Unspecified, false)],
        ..GifSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn decode_gif_rejects_garbage_and_wrong_magic() {
    assert!(decode_gif(b"not a gif at all").is_err());
    assert!(decode_gif(b"GIF87a").is_err(), "89a decoder must reject 87a magic");
}

/// 🧪️ Multi-frame, multi-region, GCE (delay/disposal/transparency) + NETSCAPE loop round trip.
#[semio_framework_async_macros::async_test]
async fn encode_decode_round_trip_multiframe() {
    let snap = sample_snapshot();
    let bytes = encode_gif(&snap).expect("encode");
    assert_eq!(&bytes[0..6], b"GIF89a");
    let decoded = decode_gif(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

/// 🧪️ decode(encode(decode(x))) snapshot equality across frames, delays, disposal, loop count.
#[semio_framework_async_macros::async_test]
async fn encode_decode_encode_decode_is_stable() {
    let snap = sample_snapshot();
    let once = decode_gif(&encode_gif(&snap).unwrap()).unwrap();
    let twice = decode_gif(&encode_gif(&once).unwrap()).unwrap();
    assert_eq!(once, twice);
}

#[semio_framework_async_macros::async_test]
async fn encode_gif_rejects_empty_frame_list() {
    let snap = GifSnapshot { schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(), width: 4, height: 4, loop_count: None, frames: vec![], ..GifSnapshot::default() };
    assert!(encode_gif(&snap).is_err());
}

#[semio_framework_async_macros::async_test]
async fn encode_gif_rejects_frame_exceeding_logical_screen() {
    let mut snap = sample_snapshot();
    snap.frames[0].left = 100; // pushes the frame past the 12x10 logical screen
    assert!(encode_gif(&snap).is_err());
}

#[semio_framework_async_macros::async_test]
async fn no_loop_extension_when_loop_count_is_none() {
    let mut snap = sample_snapshot();
    snap.loop_count = None;
    let bytes = encode_gif(&snap).expect("encode");
    // NETSCAPE2.0 must not appear anywhere in the stream when there's no loop count to encode.
    assert!(!bytes.windows(11).any(|w| w == b"NETSCAPE2.0"));
    let decoded = decode_gif(&bytes).expect("decode");
    assert_eq!(decoded.loop_count, None);
}

/// 🧪️ Comments, an unrecognized application extension, AND the NETSCAPE loop extension all
/// round-trip losslessly and don't corrupt one another — the real spec-fidelity gain this
/// rewrite delivers over the prior stub, which dropped everything but GCE/loop.
#[semio_framework_async_macros::async_test]
async fn comments_and_app_extensions_round_trip() {
    let mut snap = sample_snapshot();
    snap.comments = vec!["hello gif".into(), "second comment".into()];
    snap.app_extensions = vec![GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1, 2, 3, 4] }];
    let bytes = encode_gif(&snap).expect("encode");
    let decoded = decode_gif(&bytes).expect("decode");
    assert_eq!(decoded.comments, snap.comments);
    assert_eq!(decoded.app_extensions, snap.app_extensions);
    assert_eq!(decoded.loop_count, snap.loop_count);
    assert_eq!(decoded, snap);
}

/// 🧪️ A plain-text-only frame (no image data, `plain_text: Some`) round-trips as a real Plain
/// Text Extension block, including its preceding GCE.
#[semio_framework_async_macros::async_test]
async fn plain_text_only_frame_round_trips() {
    let mut snap = sample_snapshot();
    snap.frames.push(GifFrame {
        left: 0,
        top: 0,
        width: 0,
        height: 0,
        interlace: false,
        lct: None,
        indices: Vec::new(),
        delay_cs: 100,
        disposal: GifDisposal::DoNotDispose,
        transparent_index: None,
        user_input: false,
        plain_text: Some(GifPlainText { left: 1, top: 1, width: 8, height: 2, cell_width: 4, cell_height: 8, fg_color_index: 0, bg_color_index: 1, text: "hi gif".into() }),
    });
    let bytes = encode_gif(&snap).expect("encode");
    let decoded = decode_gif(&bytes).expect("decode");
    assert_eq!(decoded, snap);
    assert_eq!(decoded.frames.last().unwrap().plain_text.as_ref().unwrap().text, "hi gif");
}

/// 🧪️ A frame combining real image data with a plain-text extension is a documented
/// unsupported combo — must be a typed encode error, never silently drop one or the other.
#[semio_framework_async_macros::async_test]
async fn encode_gif_rejects_image_plus_plain_text_combo() {
    let mut snap = sample_snapshot();
    snap.frames[0].plain_text = Some(GifPlainText::default());
    assert!(encode_gif(&snap).is_err());
}

/// 🧪️ `interlace` is a real, round-trippable field — encode must reorder rows into the
/// on-disk interlaced pass order, and decode must invert it back to natural-order indices.
#[semio_framework_async_macros::async_test]
async fn interlace_flag_round_trips_through_real_encode() {
    let mut snap = sample_snapshot();
    snap.frames[0].interlace = true;
    let original_indices = snap.frames[0].indices.clone();
    let bytes = encode_gif(&snap).expect("encode");
    let decoded = decode_gif(&bytes).expect("decode");
    assert!(decoded.frames[0].interlace);
    assert_eq!(decoded.frames[0].indices, original_indices);
}

/// 🧪️ An index referencing past the end of its color table is a typed encode error.
#[semio_framework_async_macros::async_test]
async fn encode_gif_rejects_index_past_color_table() {
    let mut snap = sample_snapshot();
    let len = snap.frames[0].indices.len();
    snap.frames[0].indices = vec![250u8; len];
    assert!(encode_gif(&snap).is_err());
}

/// 🧪️ `rgba()` derived accessor: a transparent index normalizes to `[0,0,0,0]`.
#[semio_framework_async_macros::async_test]
async fn rgba_derived_accessor_honors_transparent_index() {
    let snap = sample_snapshot();
    let transparent_frame = &snap.frames[1]; // built with transparent_corner=true
    assert!(transparent_frame.transparent_index.is_some());
    let rgba = transparent_frame.rgba(snap.gct.as_ref());
    assert_eq!(&rgba[0..4], &[0, 0, 0, 0], "top-left pixel must be the normalized-transparent color");
}

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG2: per-artifact conformance laws (recipe §4 item 6) — grammar/protocol
/// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff`
/// output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and
/// the fixture-honesty round-trip. Lives here (the engine's own test region), not any
/// framework file — mirrors 87a's own `conformance_laws` module shape verbatim. Per the
/// ticket's own instruction, `demo_gif_snapshot()` reuses the REAL `dancing.gif` fixture
/// (54 frames, 800×800) for byte-real conformance, not a synthetic stand-in.
mod conformance_laws {
    use super::*;
    use crate::standards::v89a::subsets::any::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio`
    /// files parse under the real dialect.
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
            dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — GIF89a has
    /// no textual syntax of its own, see that file's own doc comment) recognizes real
    /// `print_dsl` output for the demo (dancing.gif) snapshot.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&demo_gif_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `GifMutation` variant (`mutations::demo_mutation_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
    /// output for every representative `GifDiff` (`diff::demo_diff_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
    /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
    /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed ==
    /// bytes.len()`.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_gif_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are
    /// GENUINE `print_dsl`/`encode_pack` output of `demo_gif_snapshot()` (the real
    /// dancing.gif fixture decoded via the real 89a codec).
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_gif_snapshot();

        let parsed = <GifSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_gif_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_gif_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <GifSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_gif_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_gif_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
