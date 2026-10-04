use super::*;
use crate::schema::demo_jpg_snapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn gradient_image(w: u32, h: u32) -> Vec<u8> {
    let mut out = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * 4) as usize;
            out[idx] = ((x * 255) / w.max(1)) as u8;
            out[idx + 1] = ((y * 255) / h.max(1)) as u8;
            out[idx + 2] = (((x + y) * 255) / (w + h).max(1)) as u8;
            out[idx + 3] = 255;
        }
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn checkerboard_image(w: u32, h: u32) -> Vec<u8> {
    let mut out = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * 4) as usize;
            let on = ((x / 8) + (y / 8)) % 2 == 0;
            let v = if on { 230u8 } else { 20u8 };
            out[idx] = v;
            out[idx + 1] = v;
            out[idx + 2] = v;
            out[idx + 3] = 255;
        }
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn mae(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mut sum = 0f64;
    let mut n = 0usize;
    for i in (0..a.len()).step_by(4) {
        for c in 0..3 {
            sum += (a[i + c] as i32 - b[i + c] as i32).abs() as f64;
            n += 1;
        }
    }
    sum / n as f64
}

#[semio_framework_async_macros::async_test]
async fn idct_fdct_is_identity() {
    let mut block = [0f64; 64];
    for (i, v) in block.iter_mut().enumerate() {
        *v = ((i * 37 % 255) as f64) - 128.0;
    }
    let coeff = fdct_8x8(&block);
    let recon = idct_8x8(&coeff);
    let maxerr = block.iter().zip(recon.iter()).fold(0f64, |m, (a, b)| m.max((a - b).abs()));
    assert!(maxerr < 1e-6, "maxerr={maxerr}");
}

#[semio_framework_async_macros::async_test]
async fn huffman_round_trips_all_dc_luma_symbols() {
    let table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
    let mut bw = BitWriter::new();
    for v in 0u8..=11 {
        let (l, c) = *table.encode.get(&v).unwrap();
        bw.put_bits(c, l);
    }
    bw.flush();
    let source: &[u8] = &bw.bytes;
    let mut br = BitReader::new(&source, 0);
    for v in 0u8..=11 {
        assert_eq!(br.decode_symbol(&table).unwrap(), v);
    }
}

#[semio_framework_async_macros::async_test]
async fn single_block_round_trips_through_huffman() {
    let dc_table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
    let ac_table = build_huffman(&AC_LUMA_BITS, &ac_luma_values()).unwrap();
    let mut zz = [0i32; 64];
    zz[0] = 120;
    zz[1] = 5;
    zz[2] = -3;
    zz[20] = 1;
    let mut bw = BitWriter::new();
    let mut dc_pred = 0i32;
    encode_block(&mut bw, &zz, &mut dc_pred, &dc_table, &ac_table).unwrap();
    bw.flush();
    let source: &[u8] = &bw.bytes;
    let mut br = BitReader::new(&source, 0);
    let mut dc_pred2 = 0i32;
    let decoded = decode_block(&mut br, &mut dc_pred2, &dc_table, &ac_table).unwrap();
    assert_eq!(decoded, zz);
}

/// 🖼️ Non-solid-color round trip — the case the old "solid-color only"
/// codec could never have passed. Gradient exercises AC energy across
/// every block; asserts mean-absolute-pixel-error stays well under a
/// visually-lossless budget of 10/255.
#[semio_framework_async_macros::async_test]
async fn gradient_round_trip_under_mae_threshold() {
    let (w, h) = (48u32, 40u32);
    let img = gradient_image(w, h);
    let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: img.clone(), ..JpgSnapshot::default() };
    let bytes = encode_jpg(&snap).expect("encode");
    assert!(bytes.starts_with(&[0xFF, 0xD8]));
    assert!(bytes.ends_with(&[0xFF, 0xD9]));
    let decoded = decode_jpg(&bytes).expect("decode");
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    let err = mae(&img, &decoded.pixels);
    println!("gradient round-trip MAE = {err}");
    assert!(err < 10.0, "gradient MAE too high: {err}");
}

/// 🖼️ Checkerboard: high-frequency content, harder for quantization to
/// preserve than a gradient — same bar (MAE < 10/255).
#[semio_framework_async_macros::async_test]
async fn checkerboard_round_trip_under_mae_threshold() {
    let (w, h) = (32u32, 32u32);
    let img = checkerboard_image(w, h);
    let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: img.clone(), ..JpgSnapshot::default() };
    let bytes = encode_jpg(&snap).expect("encode");
    let decoded = decode_jpg(&bytes).expect("decode");
    let err = mae(&img, &decoded.pixels);
    println!("checkerboard round-trip MAE = {err}");
    assert!(err < 10.0, "checkerboard MAE too high: {err}");
}

#[semio_framework_async_macros::async_test]
async fn solid_color_still_round_trips() {
    let (w, h) = (16u32, 16u32);
    let mut img = vec![0u8; (w * h * 4) as usize];
    for px in img.chunks_mut(4) {
        px[0] = 200;
        px[1] = 100;
        px[2] = 50;
        px[3] = 255;
    }
    let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: img.clone(), ..JpgSnapshot::default() };
    let bytes = encode_jpg(&snap).expect("encode");
    let decoded = decode_jpg(&bytes).expect("decode");
    let err = mae(&img, &decoded.pixels);
    assert!(err < 5.0, "solid MAE too high: {err}");
}

/// 🚫 Progressive (SOF2) must be a typed `Unsupported` error, never
/// silently decoded — hand-crafted minimal SOF2 segment.
#[semio_framework_async_macros::async_test]
async fn progressive_sof2_is_explicit_unsupported() {
    let mut bytes = vec![0xFFu8, 0xD8];
    bytes.extend_from_slice(&[0xFF, 0xC2, 0x00, 0x0B, 0x08, 0x00, 0x08, 0x00, 0x08, 0x01, 0x01, 0x11, 0x00]);
    bytes.extend_from_slice(&[0xFF, 0xD9]);
    let result = decode_jpg(&bytes);
    assert!(matches!(result, Err(JpgError::Unsupported(_))), "expected Unsupported, got {result:?}");
}

#[semio_framework_async_macros::async_test]
async fn non_jpeg_input_is_malformed_not_panic() {
    let result = decode_jpg(&[0x00, 0x01, 0x02, 0x03]);
    assert!(matches!(result, Err(JpgError::Malformed(_))));
}

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG2: per-artifact conformance laws (the recipe's §4 deliverable checklist item 6) —
/// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
/// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
/// bytes, and the fixture-honesty round-trip. Relocated verbatim from `⚙️engine`'s own test
/// region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — mirrors png's own
/// identically-named module exactly (same six laws, same structure, only the demo-case helpers
/// differ per the recipe's own note that every pilot's `conformance_laws` module is
/// near-identical).
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect — independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
    /// message).
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
            semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar (hex-dump grammar of the TEXT DSL
    /// form — jpg's real internal marker structure is `../💾️binary/📡️.protocol.semio`'s
    /// job, not this leaf's, per the recipe's own png precedent) recognizes real `print_dsl`
    /// output for the demo snapshot — same preamble-stripped body reconstruction
    /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_host_snapshot` uses.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        let text = store::ArtifactDsl::print_dsl(&demo_jpg_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `JpgMutation` variant (`mutations::demo_mutation_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `JpgDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
    /// every tri-state/`JpgFrameChange`/collection-triple shape.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
    /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
    /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
    /// mutation's `encode_op`, and every demo diff's `encode_diff`.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = semio_framework_dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_jpg_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = semio_framework_dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = semio_framework_dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// 🧪️ Shipped Text/Pack preserve every owned demo field and equal their declared codec output.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_jpg_snapshot();

        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_jpg_snapshot()) drifted from the shipped .dsl.semio fixture");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_jpg_snapshot()) drifted from the shipped .pack.semio fixture");

        let parsed = <JpgSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed,demo);
        let decoded=<JpgSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode owned Pack fixture");
        assert_eq!(decoded,demo);
    }
}
//#endregion 🔖️ConformanceLaws
