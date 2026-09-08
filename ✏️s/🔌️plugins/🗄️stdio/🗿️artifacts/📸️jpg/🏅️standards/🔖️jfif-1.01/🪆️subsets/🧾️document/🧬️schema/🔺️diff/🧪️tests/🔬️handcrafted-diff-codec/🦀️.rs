
use super::*;
use protocol::DiffCodec;
use protocol::command::DiffAlgebra;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn quant(id: u8, seed: u16) -> JpgQuantTable {
    JpgQuantTable { id, precision: 0, values: [seed; 64] }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn huffman(class: JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable {
    JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn segment(marker: u8, data: Vec<u8>) -> JpgSegment {
    JpgSegment { marker, data }
}

/// 🌱 `snap_a`/`snap_b` differ in EVERY diffable field (both directions exercise removed XOR
/// added per id-keyed/index-keyed collection, the recipe's documented workaround — see
/// `f6-recon-report.md`'s `field_sweep` precedent). `snap_c` has `frame: None`, exercising
/// `JpgFrameChange::Replace` against both `a` and `c`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap_a() -> JpgSnapshot {
    JpgSnapshot {
        schema: "stdio.jpg".into(),
        width: 4,
        height: 4,
        pixels: vec![0u8; 16],
        re_encode_quality: Some(80),
        jfif_version: (1, 1),
        jfif_density_units: JfifDensityUnits::PixelsPerInch,
        jfif_x_density: 72,
        jfif_y_density: 72,
        jfif_thumbnail: Some(JfifThumbnail { width: 2, height: 1, rgb_data: vec![1, 2, 3, 4, 5, 6] }),
        frame: Some(JpgFrameHeader { precision: 8, width: 4, height: 4, components: vec![JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 }, JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 1 }] }),
        sof_marker: 0xC0,
        arithmetic: false,
        quant_tables: vec![quant(0, 10), quant(9, 20)],
        huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1), huffman(JpgHuffmanClass::Ac, 9, 2)],
        restart_interval: Some(8),
        other_segments: vec![segment(0xFE, vec![1, 2, 3]), segment(0xE1, vec![9, 9])],
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap_b() -> JpgSnapshot {
    JpgSnapshot {
        schema: "stdio.jpg".into(),
        width: 8,
        height: 6,
        pixels: vec![9u8; 12],
        re_encode_quality: None,
        jfif_version: (1, 2),
        jfif_density_units: JfifDensityUnits::Aspect,
        jfif_x_density: 1,
        jfif_y_density: 1,
        jfif_thumbnail: None,
        frame: Some(JpgFrameHeader { precision: 8, width: 8, height: 6, components: vec![JpgFrameComponent { id: 1, h_sampling: 1, v_sampling: 1, quant_table_id: 5 }] }),
        sof_marker: 0xC2,
        arithmetic: true,
        quant_tables: vec![quant(0, 99)],
        huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 7)],
        restart_interval: None,
        other_segments: vec![segment(0xFE, vec![4, 5, 6])],
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap_c() -> JpgSnapshot {
    JpgSnapshot { frame: None, ..JpgSnapshot::default() }
}

/// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `JpgDiff` grammar — exercises the
/// `JpgFrameChange` enum (both `Modify` and `Replace`), all three tri-state scalars in both
/// transition directions, and all three id/index-keyed collection triples with removed,
/// modified, AND added entries all populated across the two `between()` directions.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snap_a();
    let b = snap_b();
    let c = snap_c();
    let cases = vec![JpgDiff::default(), JpgDiff::between(&a, &b), JpgDiff::between(&b, &a), JpgDiff::between(&a, &c), JpgDiff::between(&c, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = JpgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = JpgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
