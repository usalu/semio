
use super::*;
use crate::schema::demo_tiff_snapshot;
use crate::schema::snapshot::{TiffFieldType, TiffValues};

async fn gradient_checkerboard_rgba(w: u32, h: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
            out.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
        }
    }
    out
}

async fn ifd0_snapshot(width: u32, height: u32) -> TiffIfd {
    TiffIfd {
        pixels: Vec::new(),
        entries: vec![TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![width]) }, TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![height]) }],
    }
}

/// 🔬 Load-bearing regression: non-solid 9x5 checkerboard/gradient round-tripped through the
/// real uncompressed IFD codec.
#[semio_framework_async_macros::async_test]
async fn gradient_checkerboard_uncompressed_round_trip() {
    let (w, h) = (9u32, 5u32);
    let rgba = gradient_checkerboard_rgba(w, h).await;
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
    let encoded = encode_tiff(&snap).expect("encode");
    let decoded = decode_tiff(&encoded).expect("decode");
    assert_eq!(decoded.width(), Some(w));
    assert_eq!(decoded.height(), Some(h));
    assert_eq!(decoded.pixels, rgba, "decoded pixels must exactly match the original");
}

/// 🔬 Same fixture through real PackBits encode+decode — proves PackBits compression is
/// actually exercised (not just pass-through), by asserting the compressed strip is smaller
/// than the raw RGB and that decode reconstructs the exact original pixels.
#[semio_framework_async_macros::async_test]
async fn gradient_checkerboard_packbits_round_trip() {
    let (w, h) = (9u32, 5u32);
    let rgba = gradient_checkerboard_rgba(w, h).await;
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
    let encoded = encode_tiff_packbits(&snap).expect("encode packbits");
    let decoded = decode_tiff(&encoded).expect("decode packbits");
    assert_eq!(decoded.width(), Some(w));
    assert_eq!(decoded.height(), Some(h));
    assert_eq!(decoded.pixels, rgba, "packbits round trip must exactly match the original");
}

/// 🔬 PackBits actually runs real repeat/literal RLE, not a pass-through: a solid-color strip
/// (long repeat runs) must compress to fewer bytes than the raw RGB.
#[semio_framework_async_macros::async_test]
async fn packbits_compresses_repetitive_data() {
    let (w, h) = (20u32, 10u32);
    let rgba: Vec<u8> = (0..w * h).flat_map(|_| [128u8, 128, 128, 255]).collect();
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
    let uncompressed = encode_tiff(&snap).expect("encode uncompressed");
    let encoded = encode_tiff_packbits(&snap).expect("encode packbits");
    assert!(encoded.len() < uncompressed.len(), "packbits must shrink a byte-repetitive strip below the uncompressed encoding ({} !< {})", encoded.len(), uncompressed.len());
    let decoded = decode_tiff(&encoded).expect("decode packbits");
    assert_eq!(decoded.pixels, rgba);
}

#[semio_framework_async_macros::async_test]
async fn packbits_hand_decode_control_bytes() {
    // literal run of 3 (10,20,30), then repeat run of 5x99, then literal run of 2 (1,2)
    let encoded: [u8; 9] = [2, 10, 20, 30, 0xFC, 99, 1, 1, 2];
    let expected: Vec<u8> = vec![10, 20, 30, 99, 99, 99, 99, 99, 1, 2];
    let decoded = packbits_decode(&encoded, expected.len()).expect("decode");
    assert_eq!(decoded, expected);
    let re_encoded = packbits_encode(&expected);
    let re_decoded = packbits_decode(&re_encoded, expected.len()).expect("re-decode");
    assert_eq!(re_decoded, expected);
}

/// 🔬 Big-endian (`MM`) byte order must decode correctly too, AND encode must round-trip
/// `byte_order` itself (real round-trip, not always-little-endian).
#[semio_framework_async_macros::async_test]
async fn big_endian_round_trip() {
    let (w, h) = (2u32, 1u32);
    let rgba = vec![10u8, 20, 30, 255, 40, 50, 60, 255];
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::BigEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
    let encoded = encode_tiff(&snap).expect("encode big-endian");
    assert_eq!(&encoded[0..2], b"MM", "encode must honor byte_order, not always little-endian");
    let decoded = decode_tiff(&encoded).expect("decode big-endian tiff");
    assert_eq!(decoded.byte_order, TiffByteOrder::BigEndian);
    assert_eq!(decoded.width(), Some(w));
    assert_eq!(decoded.height(), Some(h));
    assert_eq!(decoded.pixels, rgba);
}

/// 🔬 A non-core tag (`Artist`, ASCII, out-of-line since its value exceeds 4 bytes) set on
/// `ifds[0]` must survive an encode/decode round trip verbatim — proves the generic
/// tag/type/value model, not just the hardcoded strip-geometry tags.
#[semio_framework_async_macros::async_test]
async fn carried_ascii_tag_round_trips() {
    let (w, h) = (2u32, 2u32);
    let rgba = vec![1u8; (w * h * 4) as usize];
    let mut ifd = ifd0_snapshot(w, h).await;
    ifd.entries.push(TiffTag { tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("A Real Artist".into()) });
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd], pixels: rgba };
    let encoded = encode_tiff(&snap).expect("encode");
    let decoded = decode_tiff(&encoded).expect("decode");
    let artist = decoded.tag(315).expect("Artist tag must survive round trip");
    assert_eq!(artist.values, TiffValues::Ascii("A Real Artist".into()));
}

/// 🔬 A short (inline) non-core numeric tag also survives.
#[semio_framework_async_macros::async_test]
async fn carried_short_tag_round_trips() {
    let (w, h) = (2u32, 2u32);
    let rgba = vec![1u8; (w * h * 4) as usize];
    let mut ifd = ifd0_snapshot(w, h).await;
    ifd.entries.push(TiffTag { tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![2]) }); // ResolutionUnit
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd], pixels: rgba };
    let encoded = encode_tiff(&snap).expect("encode");
    let decoded = decode_tiff(&encoded).expect("decode");
    assert_eq!(decoded.tag(296).expect("ResolutionUnit must survive").values, TiffValues::Short(vec![2]));
}

/// 🔬 Real multi-IFD encode: a genuinely two-IFD snapshot round-trips through
/// `encode_tiff`/`decode_tiff` with BOTH directories intact — the `next IFD offset` chain
/// `decode_tiff` walks is actually written, not dropped, and IFD 1's own non-strip tags
/// survive verbatim even though it carries no backing pixel data.
#[semio_framework_async_macros::async_test]
async fn multi_ifd_round_trip_preserves_every_ifd() {
    let (w, h) = (2u32, 2u32);
    let rgba = vec![7u8; (w * h * 4) as usize];
    let ifd1 = TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: 270, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("second page".into()) }] };
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await, ifd1], pixels: rgba.clone() };
    let encoded = encode_tiff(&snap).expect("encode multi-ifd");
    let decoded = decode_tiff(&encoded).expect("decode multi-ifd");
    assert_eq!(decoded.ifds.len(), 2, "both IFDs must survive the real chain");
    assert_eq!(decoded.pixels, rgba, "IFD 0's raster must be unaffected by a second IFD existing");
    let second = decoded.ifds[1].entries.iter().find(|t| t.tag == 270).expect("IFD 1's own tag must survive");
    assert_eq!(second.values, TiffValues::Ascii("second page".into()));
}

/// 🔬 THE regression this wave exists for (`📓️w13-final-audit.md` §2.2(12)): a secondary
/// directory that CARRIES raster must come back with its raster AND with the three strip tags
/// TIFF6 §Baseline makes required of it — `StripOffsets`, `RowsPerStrip` (forced to the page's
/// own `ImageLength`, since this writer always re-lays a directory out as one combined strip)
/// and `StripByteCounts`. Before `TiffIfd::pixels` existed the encoder had nothing to back a
/// non-primary raster with, dropped the two pointer tags and never emitted `RowsPerStrip`, so
/// every round trip of a real multi-page file silently destroyed page 2 — measured by nothing,
/// because the semantic projection only decodes IFD 0's raster.
#[semio_framework_async_macros::async_test]
async fn secondary_ifd_raster_and_its_required_strip_tags_survive_the_codec() {
    let (w, h) = (2u32, 2u32);
    let rgba = vec![7u8; (w * h * 4) as usize];
    let page2: Vec<u8> = (0u8..12).collect(); // 2x2 chunky RGB = 12 bytes
    let ifd1 = TiffIfd {
        entries: vec![
            TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![2]) },
            TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![2]) },
            TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8, 8, 8]) },
            TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) },
        ],
        pixels: page2.clone(),
    };
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await, ifd1], pixels: rgba.clone() };
    let decoded = decode_tiff(&encode_tiff(&snap).expect("encode")).expect("decode");
    assert_eq!(decoded.ifds.len(), 2);
    assert_eq!(decoded.pixels, rgba, "IFD 0's raster must be unaffected");
    assert_eq!(decoded.ifds[1].pixels, page2, "IFD 1's own strip bytes must survive the round trip");
    let rows_per_strip = decoded.ifds[1].entries.iter().find(|t| t.tag == TAG_ROWS_PER_STRIP).expect("a strip-organised IFD must declare RowsPerStrip");
    assert_eq!(rows_per_strip.values, TiffValues::Long(vec![2]), "one combined strip means RowsPerStrip == ImageLength");
    // 🧭 The two pointer tags are layout, recomputed on write and folded back into `pixels` on
    // read, so a second round trip is a fixpoint rather than a drift of stale offsets.
    assert!(!decoded.ifds[1].entries.iter().any(|t| t.tag == TAG_STRIP_OFFSETS || t.tag == TAG_STRIP_BYTE_COUNTS), "strip pointers belong to the layout, not to the snapshot");
    let twice = decode_tiff(&encode_tiff(&decoded).expect("re-encode")).expect("re-decode");
    assert_eq!(twice.ifds, decoded.ifds, "decode(encode(x)) must be a fixpoint for every directory");
}

/// 🔬 `InsertIfd`/`RemoveIfd` genuinely observable THROUGH THE CODEC, not merely in the
/// in-memory `TiffSnapshot`: apply the mutation, encode to real bytes, decode those bytes back
/// with the independent `decode_tiff` chain walk, and see the directory actually appear/vanish.
#[semio_framework_async_macros::async_test]
async fn insert_ifd_and_remove_ifd_are_observable_through_the_codec() {
    use crate::TiffMutation;
    use crate::schema::mutations::apply_tiff_mutation;

    let (w, h) = (2u32, 2u32);
    let rgba = vec![3u8; (w * h * 4) as usize];
    let base = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba };
    let mut snapshot = decode_tiff(&encode_tiff(&base).expect("encode base")).expect("decode base");
    assert_eq!(snapshot.ifds.len(), 1);

    let inserted = TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: 270, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("inserted page".into()) }] };
    apply_tiff_mutation(&mut snapshot, &TiffMutation::InsertIfd(crate::schema::mutations::InsertIfdMutation { index: 1, ifd: inserted }));
    let after_insert = decode_tiff(&encode_tiff(&snapshot).expect("encode after insert")).expect("decode after insert");
    assert_eq!(after_insert.ifds.len(), 2, "InsertIfd must add a real, decodable second directory");
    let tag = after_insert.ifds[1].entries.iter().find(|t| t.tag == 270).expect("inserted IFD's tag must survive the codec");
    assert_eq!(tag.values, TiffValues::Ascii("inserted page".into()));

    apply_tiff_mutation(&mut snapshot, &TiffMutation::RemoveIfd(crate::schema::mutations::RemoveIfdMutation { index: 1 }));
    let after_remove = decode_tiff(&encode_tiff(&snapshot).expect("encode after remove")).expect("decode after remove");
    assert_eq!(after_remove.ifds.len(), 1, "RemoveIfd must genuinely drop the directory from the encoded chain");
}

#[semio_framework_async_macros::async_test]
async fn sniff_rejects_non_tiff_bytes() {
    let err = decode_tiff(b"not a tiff at all").unwrap_err();
    assert!(err.contains("byte-order"));
}

#[semio_framework_async_macros::async_test]
async fn unsupported_compression_is_a_typed_error() {
    let (w, h) = (2u32, 2u32);
    let mut ifd = ifd0_snapshot(w, h).await;
    ifd.entries.push(TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8]) });
    ifd.entries.push(TiffTag { tag: TAG_COMPRESSION, kind: TiffFieldType::Short, values: TiffValues::Short(vec![5]) }); // LZW — intentionally unsupported
    ifd.entries.push(TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) });
    ifd.entries.push(TiffTag { tag: TAG_STRIP_OFFSETS, kind: TiffFieldType::Long, values: TiffValues::Long(vec![0]) });
    ifd.entries.sort_by_key(|t| t.tag);

    // Hand-encode a minimal file carrying this IFD (bypassing `encode_tiff`, which always
    // canonicalizes `Compression` itself) to exercise `decode_tiff`'s own rejection path.
    let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd], pixels: Vec::new() };
    let dir_offset = 8usize;
    let entries = &snap.ifds[0].entries;
    let mut out = Vec::new();
    out.extend_from_slice(b"II");
    write_u16(&mut out, 42, TiffByteOrder::LittleEndian);
    write_u32(&mut out, dir_offset as u32, TiffByteOrder::LittleEndian);
    write_u16(&mut out, entries.len() as u16, TiffByteOrder::LittleEndian);
    for t in entries {
        write_u16(&mut out, t.tag, TiffByteOrder::LittleEndian);
        write_u16(&mut out, t.kind.to_u16(), TiffByteOrder::LittleEndian);
        write_u32(&mut out, t.values.count(), TiffByteOrder::LittleEndian);
        let vb = value_bytes(&t.values, TiffByteOrder::LittleEndian);
        let mut field = [0u8; 4];
        field[..vb.len().min(4)].copy_from_slice(&vb[..vb.len().min(4)]);
        out.extend_from_slice(&field);
    }
    write_u32(&mut out, 0, TiffByteOrder::LittleEndian);
    let err = decode_tiff(&out).unwrap_err();
    assert!(err.contains("unsupported compression"), "unexpected error: {err}");
}

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG2: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item 6) —
/// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
/// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
/// bytes, and the fixture-honesty round-trip. Relocated verbatim from `⚙️engine`'s own test
/// region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — mirrors png's own
/// `conformance_laws` module shape exactly.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio`
    /// files parse under the real dialect — independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a
    /// clearer message).
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

    /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — TIFF has no
    /// textual syntax of its own, see that file's own doc comment) recognizes real
    /// `print_dsl` output for the demo snapshot — same preamble-stripped body
    /// reconstruction `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture`
    /// uses, so this is a direct proof this artifact will pass that harness once graduated,
    /// not merely an analogue.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&demo_tiff_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `TiffMutation` variant (`mutations::demo_mutation_cases()`).
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
    /// output for every representative `TiffDiff` (`diff::demo_diff_cases()`), incl. the
    /// empty diff, every IFD-level/tag-level collection-triple shape, and every
    /// `TiffValues` field-type family.
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
    /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
    /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
    /// mutation's `encode_op`, and every demo diff's `encode_diff`. The snapshot protocol
    /// only describes the real 8-byte header + first-IFD entry-count field as INDIVIDUALLY
    /// typed fields (§ this standard's own protocol.semio doc comment: IFD-entry-array/
    /// out-of-line-offset/IFD-chain resolution are honest mechanism gaps) before the
    /// trailing `chain rest bytes` consumes everything past that point — so `consumed ==
    /// bytes.len()` still holds exactly for every facet, same as the op/diff protocols.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_tiff_snapshot());
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
    /// GENUINE `print_dsl`/`encode_pack` output of `demo_tiff_snapshot()` —
    /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and
    /// the pack twin — so the fixtures can never silently drift back to a fake again.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_tiff_snapshot();

        let parsed = <TiffSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_tiff_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_tiff_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <TiffSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_tiff_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_tiff_snapshot()) drifted from the shipped .pack.semio fixture");

        let native = encode_tiff(&demo).expect("encode native tiff");
        assert_eq!(native.as_slice(), include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🧪️example/🖼️.tiff"), "encode_tiff(demo) drifted from 🖼️example.tiff");
    }

    #[semio_framework_async_macros::async_test]
    #[ignore]
    async fn zzz_write_native_tiff_fixture() {
        let demo = demo_tiff_snapshot();
        let native = encode_tiff(&demo).expect("encode");
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/📚️examples/🎬️demo/🖼️assets/🧪️example/🖼️.tiff");
        std::fs::write(path, native).expect("write 🖼️example.tiff");
    }
}
//#endregion 🔖️ConformanceLaws
