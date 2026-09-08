
use super::*;
use crate::standards::v87a::subsets::any::schema::demo_gif_snapshot;

/// 🧪️ Builds a real, lossless `GifImage` (LCT + indices) from a checkerboard RGBA pattern via
/// `quantize_rgba`/`indices_to_rgba` — those two byte-level helpers stay real and `pub` for
/// exactly this kind of "construct a frame from pixel content" test/tooling use, even though
/// `encode_gif`/`decode_gif` no longer call them (the codec now writes/reads whatever palette
/// + indices are already in the snapshot, never re-quantizing).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn checkerboard(width: u32, height: u32) -> GifImage {
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
        for x in 0..width {
            let o = ((y * width + x) * 4) as usize;
            let on = (x + y) % 2 == 0;
            rgba[o] = if on { 255 } else { 10 };
            rgba[o + 1] = if on { 0 } else { 200 };
            rgba[o + 2] = if on { 0 } else { 30 };
            rgba[o + 3] = 255;
        }
    }
    let (palette, indices, _transparent) = quantize_rgba(&rgba).expect("quantize");
    GifImage { left: 0, top: 0, width, height, interlace: false, lct: Some(color_table_from_bytes(palette, false)), indices }
}

/// 🧪️ LZW core: trivial round trip at the smallest legal minimum code size.
#[semio_framework_async_macros::async_test]
async fn lzw_round_trip_trivial() {
    let indices = vec![1u8, 2, 1, 2, 1, 2, 3, 3, 3, 3];
    let enc = lzw_encode(&indices, 2);
    let dec = lzw_decode(&enc, 2).expect("decode");
    assert_eq!(dec, indices);
}

/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: an
/// all-one-color-ish long run at min_code_size=8 forces the dictionary well past the 8-bit
/// boundary, exercising the asymmetric growth-threshold rule documented on `lzw_encode`.
#[semio_framework_async_macros::async_test]
async fn lzw_round_trip_forces_code_size_growth() {
    let indices: Vec<u8> = (0..5000).map(|i| (i % 2) as u8).collect();
    let enc = lzw_encode(&indices, 8);
    assert!(enc.len() < indices.len(), "highly repetitive data must compress");
    let dec = lzw_decode(&enc, 8).expect("decode");
    assert_eq!(dec, indices);
}

/// 🧪️ A single solid color run drives the dictionary to grow every entry from one repeated
/// symbol — the worst case for the KwKwK (code == table length) decode branch.
#[semio_framework_async_macros::async_test]
async fn lzw_round_trip_solid_run_and_kwkwk() {
    let indices = vec![7u8; 20_000];
    let enc = lzw_encode(&indices, 8);
    let dec = lzw_decode(&enc, 8).expect("decode");
    assert_eq!(dec, indices);
    assert!(enc.len() < indices.len() / 10);
}

/// 🧪️ Pseudo-random data at every legal minimum code size (2..=8), large enough to cross
/// multiple code-size growth boundaries and at least one dictionary-full clear-code reset.
#[semio_framework_async_macros::async_test]
async fn lzw_round_trip_pseudo_random_all_min_code_sizes() {
    for mcs in 2u8..=8 {
        let max_sym = (1u32 << mcs) - 1;
        let mut indices = Vec::new();
        let mut state = 12345u32;
        for _ in 0..60_000 {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            indices.push(((state >> 16) % (max_sym + 1)) as u8);
        }
        let enc = lzw_encode(&indices, mcs);
        let dec = lzw_decode(&enc, mcs).unwrap_or_else(|e| panic!("min_code_size={mcs}: {e}"));
        assert_eq!(dec, indices, "min_code_size={mcs}");
    }
}

#[semio_framework_async_macros::async_test]
async fn lzw_round_trip_empty_and_single_symbol() {
    assert_eq!(lzw_decode(&lzw_encode(&[], 2), 2).unwrap(), Vec::<u8>::new());
    assert_eq!(lzw_decode(&lzw_encode(&[3], 4), 4).unwrap(), vec![3u8]);
}

/// 🧪️ Regression for a real bug this ticket found and fixed: a plain period-2 alternating
/// sequence (`0,1,0,1,...`) at `min_code_size=2` whose dictionary crosses a code-size growth
/// boundary EXACTLY on the final symbol desynced encoder/decoder (encoder wrote the trailing
/// end-code at the OLD bit width; decoder, having just grown from its own insert on the final
/// data code, expected the NEW bit width) — the pre-existing pseudo-random/solid-run test data
/// never happened to land on this exact boundary. Swept across several lengths and min code
/// sizes to catch the boundary regardless of exactly which length triggers it.
#[semio_framework_async_macros::async_test]
async fn lzw_round_trip_period_two_alternating_hits_growth_boundary_at_tail() {
    for mcs in 2u8..=6 {
        for len in 2usize..=80 {
            let indices: Vec<u8> = (0..len).map(|i| (i % 2) as u8).collect();
            let enc = lzw_encode(&indices, mcs);
            let dec = lzw_decode(&enc, mcs).unwrap_or_else(|e| panic!("min_code_size={mcs} len={len}: {e}"));
            assert_eq!(dec, indices, "min_code_size={mcs} len={len}");
        }
    }
}

/// 🧪️ `decode_gif` must reject truncated/invalid input with a typed `Err`, never fabricate
/// pixels — regression guard for the prior stub which silently produced an all-black image.
#[semio_framework_async_macros::async_test]
async fn decode_gif_rejects_garbage() {
    assert!(decode_gif(b"not a gif at all").is_err());
    assert!(decode_gif(b"GIF89a").is_err(), "87a decoder must reject 89a magic");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> GifSnapshot {
    GifSnapshot { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), width: 37, height: 29, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, images: vec![checkerboard(37, 29)] }
}

/// 🧪️ Full byte-level codec round trip through a real (non-solid) checkerboard image,
/// exercising LCT sizing and the sub-block-packed LZW stream together — losslessly, at the
/// palette-index level, not by re-quantizing decoded RGBA.
#[semio_framework_async_macros::async_test]
async fn encode_decode_round_trip_checkerboard() {
    let snap = sample_snapshot();
    let bytes = encode_gif(&snap).expect("encode");
    assert_eq!(&bytes[0..6], b"GIF87a");
    let decoded = decode_gif(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

/// 🧪️ decode(encode(decode(x))) snapshot equality — the acceptance bar from the plan's
/// fixtures section (model equality across a second round trip, not necessarily byte-exact).
#[semio_framework_async_macros::async_test]
async fn encode_decode_encode_decode_is_stable() {
    let snap = GifSnapshot { width: 9, height: 13, images: vec![checkerboard(9, 13)], ..GifSnapshot::default() };
    let once = decode_gif(&encode_gif(&snap).unwrap()).unwrap();
    let twice = decode_gif(&encode_gif(&once).unwrap()).unwrap();
    assert_eq!(once, twice);
}

/// 🧪️ GIF87a genuinely permits more than one Image Descriptor per file (§20) even without any
/// extension block — a real spec-fidelity gain over the prior single-`RasterImage` model.
#[semio_framework_async_macros::async_test]
async fn encode_decode_round_trip_multiple_images() {
    let snap = GifSnapshot { width: 20, height: 20, images: vec![checkerboard(6, 6), checkerboard(9, 4), checkerboard(3, 3)], ..GifSnapshot::default() };
    let bytes = encode_gif(&snap).expect("encode");
    let decoded = decode_gif(&bytes).expect("decode");
    assert_eq!(decoded.images.len(), 3);
    assert_eq!(decoded, snap);
}

/// 🧪️ A global color table shared by an image with no local table round-trips, including the
/// sort flag and the real (no-longer-hardcoded) background-color-index/pixel-aspect-ratio bytes.
#[semio_framework_async_macros::async_test]
async fn encode_decode_round_trip_global_color_table_and_screen_fields() {
    let (palette, indices, _) = quantize_rgba(&{
        let mut rgba = vec![0u8; 4 * 4 * 4];
        for i in 0..16 {
            rgba[i * 4] = (i * 16) as u8;
            rgba[i * 4 + 3] = 255;
        }
        rgba
    })
    .unwrap();
    let snap = GifSnapshot {
        width: 4,
        height: 4,
        gct: Some(color_table_from_bytes(palette, true)),
        background_color_index: 2,
        pixel_aspect_ratio: 17,
        images: vec![GifImage { left: 0, top: 0, width: 4, height: 4, interlace: false, lct: None, indices }],
        ..GifSnapshot::default()
    };
    let decoded = decode_gif(&encode_gif(&snap).unwrap()).unwrap();
    assert_eq!(decoded, snap);
    assert!(decoded.gct.unwrap().sorted);
}

/// 🧪️ `interlace` is now a real, round-trippable field — encode must actually reorder rows
/// into the on-disk interlaced pass order (not just set the bit), and decode must invert it
/// back to the same natural-order indices this test started with.
#[semio_framework_async_macros::async_test]
async fn interlace_flag_round_trips_through_real_encode() {
    let mut image = checkerboard(11, 17);
    image.interlace = true;
    let snap = GifSnapshot { width: 11, height: 17, images: vec![image.clone()], ..GifSnapshot::default() };
    let bytes = encode_gif(&snap).expect("encode");
    let decoded = decode_gif(&bytes).expect("decode");
    assert!(decoded.images[0].interlace);
    assert_eq!(decoded.images[0].indices, image.indices, "de-interlaced indices must match the original natural-order indices");
}

/// 🧪️ An index referencing past the end of its color table is a typed encode error, never a
/// silently-corrupt file.
#[semio_framework_async_macros::async_test]
async fn encode_gif_rejects_index_past_color_table() {
    let mut image = checkerboard(2, 2);
    image.indices = vec![250, 250, 250, 250]; // way past the checkerboard's tiny 2-color LCT
    let snap = GifSnapshot { width: 2, height: 2, images: vec![image], ..GifSnapshot::default() };
    assert!(encode_gif(&snap).is_err());
}

#[semio_framework_async_macros::async_test]
async fn interlace_round_trip() {
    let width = 5usize;
    let height = 9usize;
    let rows: Vec<u8> = (0..(width * height) as u32).map(|i| (i % 251) as u8).collect();
    // Interlace the rows using the same pass order deinterlace_rows expects to invert.
    let mut interlaced = vec![0u8; width * height];
    let mut dst = 0usize;
    for (start, step) in [(0usize, 8usize), (4, 8), (2, 4), (1, 2)] {
        let mut row = start;
        while row < height {
            interlaced[dst * width..dst * width + width].copy_from_slice(&rows[row * width..row * width + width]);
            dst += 1;
            row += step;
        }
    }
    let restored = deinterlace_rows(&interlaced, width, height);
    assert_eq!(restored, rows);
}

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG2: per-artifact conformance laws (recipe §4 item 6) — grammar/protocol
/// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff`
/// output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and
/// the fixture-honesty round-trip. Lives here (the engine's own test region), not any
/// framework file — mirrors png's own `conformance_laws` module shape verbatim.
mod conformance_laws {
    use super::*;
    use crate::standards::v87a::subsets::any::schema::{diff, mutations, snapshot};
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

    /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — GIF87a has
    /// no textual syntax of its own, see that file's own doc comment) recognizes real
    /// `print_dsl` output for the demo snapshot.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&demo_gif_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
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
    /// GENUINE `print_dsl`/`encode_pack` output of `demo_gif_snapshot()`.
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
