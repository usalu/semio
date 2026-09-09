use super::*;
use crate::schema::{demo_bmp_snapshot, empty_bmp_snapshot};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn gradient_checkerboard_rgba(w: u32, h: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
            out.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
        }
    }
    out
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_bmp_snapshot();
    assert_eq!(snapshot.schema, STDIO_BMP_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = empty_bmp_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <BmpSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region RowPadding
/// 🔬 width=5 at 24bpp is 15 raw bytes/row, padded to 16 — a width that divides evenly would
/// not catch a broken padding formula.
#[semio_framework_async_macros::async_test]
async fn row_bytes_padding_is_exact() {
    assert_eq!(row_bytes(5, 24), 16);
    assert_eq!(row_bytes(4, 24), 12);
    assert_eq!(row_bytes(1, 1), 4);
    assert_eq!(row_bytes(9, 1), 4);
    assert_eq!(row_bytes(5, 4), 4);
    assert_eq!(row_bytes(6, 8), 8);
    assert_eq!(row_bytes(3, 32), 12);
}
//#endregion RowPadding

//#region TwentyFourBitRoundTrip
/// 🔬 Load-bearing regression: non-solid 6x4 checkerboard/gradient through real 24-bit
/// BI_RGB encode+decode, width chosen so raw row bytes (18) is NOT a multiple of 4 —
/// exercises row padding on both the encode and decode sides.
#[semio_framework_async_macros::async_test]
async fn gradient_checkerboard_24bit_round_trip() {
    let (w, h) = (6u32, 4u32);
    let pixels = gradient_checkerboard_rgba(w, h);
    let snap = BmpSnapshot { width: w, height: h, pixels: pixels.clone(), ..BmpSnapshot::default() };
    let encoded = encode_bmp(&snap).expect("encode");
    // sanity: row padded to 4-byte boundary (6*3=18 raw -> 20 padded)
    assert_eq!(row_bytes(w, 24), 20);
    let decoded = decode_bmp(&encoded).expect("decode");
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    assert_eq!(decoded.pixels, pixels, "decoded pixels must exactly match the original (alpha forced to 255)");
}
//#endregion TwentyFourBitRoundTrip

//#region IndexedFixture
/// 🧪 Hand-encodes a 4-bit indexed (16-color-capable, 4 used) BMP with a non-trivial
/// checkerboard-ish index pattern and asserts `decode_bmp` reconstructs the exact palette
/// colors — proves palette lookup + sub-byte (nibble) unpacking + bottom-up row order.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn hand_encode_indexed(width: u32, height: u32, bpp: u16, palette: &[[u8; 4]], indices: &[Vec<usize>]) -> Vec<u8> {
    let rb = row_bytes(width, bpp);
    let palette_bytes = palette.len() * 4;
    let data_offset = 14 + 40 + palette_bytes;
    let pixel_bytes = rb * height as usize;
    let file_size = data_offset + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);
    out.extend_from_slice(&BMP_MAGIC);
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&(data_offset as u32).to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(width as i32).to_le_bytes());
    out.extend_from_slice(&(height as i32).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&bpp.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 8]);
    out.extend_from_slice(&(palette.len() as u32).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    for entry in palette {
        out.extend_from_slice(entry);
    }
    // indices[0] is the file's bottom-most (first-written) row per BMP's default bottom-up order
    for row_indices in indices {
        let mut row_buf = vec![0u8; rb];
        for (x, &idx) in row_indices.iter().enumerate() {
            match bpp {
                8 => row_buf[x] = idx as u8,
                4 => {
                    let byte = &mut row_buf[x / 2];
                    if x % 2 == 0 {
                        *byte = (*byte & 0x0F) | ((idx as u8) << 4);
                    } else {
                        *byte = (*byte & 0xF0) | (idx as u8);
                    }
                }
                1 => {
                    let byte = &mut row_buf[x / 8];
                    let bit = 7 - (x % 8);
                    if idx != 0 {
                        *byte |= 1 << bit;
                    }
                }
                _ => unreachable!(),
            }
        }
        out.extend_from_slice(&row_buf);
    }
    out
}

#[semio_framework_async_macros::async_test]
async fn indexed_4bit_palette_round_trip() {
    // 5x3 image (row_bytes(5,4) = 4, NOT equal to raw 5*4bits/8=2.5->3 bytes — exercises padding),
    // palette of 4 colors, non-trivial (non-solid) index pattern.
    let palette = [
        [255u8, 0, 0, 0],  // B,G,R,pad -> red
        [0u8, 255, 0, 0],  // green
        [0u8, 0, 255, 0],  // blue
        [10u8, 20, 30, 0], // arbitrary
    ];
    assert_eq!(row_bytes(5, 4), 4);
    // file rows are bottom-up; row 0 here = bottom of the image
    let file_rows = vec![
        vec![0usize, 1, 2, 3, 0], // bottom row (displayed last)
        vec![3usize, 2, 1, 0, 1],
        vec![1usize, 0, 3, 2, 3], // top row (displayed first)
    ];
    let bytes = hand_encode_indexed(5, 3, 4, &palette, &file_rows);
    let decoded = decode_bmp(&bytes).expect("decode 4-bit indexed");
    assert_eq!(decoded.width, 5);
    assert_eq!(decoded.height, 3);
    // top displayed row (out_y=0) must equal file_rows[2] (last file row, bottom-up)
    let expect_row = |row_indices: &[usize]| -> Vec<u8> {
        row_indices
            .iter()
            .flat_map(|&i| {
                let e = palette[i];
                [e[2], e[1], e[0], 255]
            })
            .collect()
    };
    let top = &decoded.pixels[0..5 * 4];
    let mid = &decoded.pixels[5 * 4..10 * 4];
    let bottom = &decoded.pixels[10 * 4..15 * 4];
    assert_eq!(top, expect_row(&file_rows[2]).as_slice());
    assert_eq!(mid, expect_row(&file_rows[1]).as_slice());
    assert_eq!(bottom, expect_row(&file_rows[0]).as_slice());
}
//#endregion IndexedFixture

//#region BitfieldsFixture
/// 🧪 Hand-encodes a 16-bit `BI_BITFIELDS` (5-5-5) BMP and checks the classic "count
/// trailing zeros then scale by mask bit-width" extraction is exact for known values.
#[semio_framework_async_macros::async_test]
async fn bitfields_16bit_555_round_trip() {
    let (w, h) = (4u32, 2u32);
    // masks: R=0x7C00 G=0x03E0 B=0x001F, no alpha
    let r_mask = 0x7C00u32;
    let g_mask = 0x03E0u32;
    let b_mask = 0x001Fu32;
    let pack = |r5: u16, g5: u16, b5: u16| -> u16 { (r5 << 10) | (g5 << 5) | b5 };
    // 4 distinct pixels per row, 2 rows — deliberately not a solid color.
    let raw_pixels: [[u16; 4]; 2] = [[pack(31, 0, 0), pack(0, 31, 0), pack(0, 0, 31), pack(31, 31, 31)], [pack(16, 8, 4), pack(4, 16, 8), pack(8, 4, 16), pack(0, 0, 0)]];
    let rb = row_bytes(w, 16);
    assert_eq!(rb, 8); // 4px * 2bytes = 8, already 4-byte aligned
    let header_size = 40u32;
    let masks_size = 12usize;
    let data_offset = 14 + header_size as usize + masks_size;
    let pixel_bytes = rb * h as usize;
    let file_size = data_offset + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);
    out.extend_from_slice(&BMP_MAGIC);
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&(data_offset as u32).to_le_bytes());
    out.extend_from_slice(&header_size.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    out.extend_from_slice(&(h as i32).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(&3u32.to_le_bytes()); // BI_BITFIELDS
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&r_mask.to_le_bytes());
    out.extend_from_slice(&g_mask.to_le_bytes());
    out.extend_from_slice(&b_mask.to_le_bytes());
    // file rows bottom-up: write row 1 (h-1) first, then row 0
    for file_row in 0..h as usize {
        let src_row = h as usize - 1 - file_row;
        for &px in &raw_pixels[src_row] {
            out.extend_from_slice(&px.to_le_bytes());
        }
    }
    let decoded = decode_bmp(&out).expect("decode 16-bit bitfields");
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    // top displayed row (out_y=0) corresponds to raw_pixels[0]
    assert_eq!(&decoded.pixels[0..4 * 4], &[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255][..]);
    // second row: verify scaled mid-intensity channel values (16/31*255 rounded = 132, 8/31*255=66, 4/31*255=33)
    let row2 = &decoded.pixels[4 * 4..8 * 4];
    assert_eq!(row2[0], 132); // r of pack(16,8,4)
    assert_eq!(row2[1], 66); // g of pack(16,8,4)
    assert_eq!(row2[2], 33); // b of pack(16,8,4)
    assert_eq!(row2[3], 255);
}
//#endregion BitfieldsFixture

#[semio_framework_async_macros::async_test]
async fn sniff_rejects_non_bmp_bytes() {
    let err = decode_bmp(b"not a bmp at all").unwrap_err();
    assert!(err.contains("signature"));
}

//#region 🔖️CodecRetentionLaw
/// 🔬 `codec_retention_law`: decode(encode(snap)) is byte-preserving for every field encode
/// actually controls — `row_order` (both directions), metadata (`x/y_pixels_per_meter`,
/// `colors_used`, `colors_important`, `image_size`), and pixels — while the DOCUMENTED
/// EncodeScopeNote normalization for the DIRECT (no-palette) path (`header_size`→40,
/// `planes`→1, `bits_per_pixel`→24, `compression`→0) is asserted explicitly rather than
/// silently ignored. `BmpSnapshot::default()` declares no palette, so this exercises
/// `encode_bmp_direct`; `indexed_palette_round_trip`/`palette_mutations_are_observable_in_encoded_bytes`
/// below cover the indexed path's own contract.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let (w, h) = (6u32, 4u32);
    let pixels = gradient_checkerboard_rgba(w, h);

    let bottom_up = BmpSnapshot { width: w, height: h, row_order: BmpRowOrder::BottomUp, x_pixels_per_meter: 2835, y_pixels_per_meter: 2835, colors_used: 0, colors_important: 0, pixels: pixels.clone(), ..BmpSnapshot::default() };
    let encoded = encode_bmp(&bottom_up).expect("encode bottom-up");
    let decoded = decode_bmp(&encoded).expect("decode bottom-up");
    assert_eq!(decoded.width, bottom_up.width);
    assert_eq!(decoded.height, bottom_up.height);
    assert_eq!(decoded.row_order, BmpRowOrder::BottomUp);
    assert_eq!(decoded.x_pixels_per_meter, bottom_up.x_pixels_per_meter);
    assert_eq!(decoded.y_pixels_per_meter, bottom_up.y_pixels_per_meter);
    assert_eq!(decoded.colors_used, bottom_up.colors_used);
    assert_eq!(decoded.colors_important, bottom_up.colors_important);
    assert_eq!(decoded.image_size, row_bytes(w, 24) as u32 * h);
    assert_eq!(decoded.pixels, pixels, "decoded pixels must exactly match the original");
    assert_eq!(decoded.header_size, 40, "documented normalization: encode always emits a 40-byte header");
    assert_eq!(decoded.planes, 1, "documented normalization: encode always emits planes=1");
    assert_eq!(decoded.bits_per_pixel, 24, "documented normalization: encode always emits 24bpp");
    assert_eq!(decoded.compression, 0, "documented normalization: encode always emits BI_RGB");

    // 🔁 Same fixture, top-down: proves `row_order` drives BOTH the sign of the on-disk
    // `height` field AND the physical row-write direction, not just the enum's own equality.
    let top_down = BmpSnapshot { row_order: BmpRowOrder::TopDown, ..bottom_up.clone() };
    let encoded_td = encode_bmp(&top_down).expect("encode top-down");
    assert_ne!(encoded_td, encoded, "top-down encode must differ from bottom-up (row order + height sign)");
    let decoded_td = decode_bmp(&encoded_td).expect("decode top-down");
    assert_eq!(decoded_td.row_order, BmpRowOrder::TopDown);
    assert_eq!(decoded_td.pixels, pixels, "canonical pixels (row 0 = top) must match regardless of row_order");
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️IndexedRoundTrip
/// 🎨 A small hand-built 8-bit indexed snapshot (a real `BmpSnapshot`, not hand-encoded
/// bytes): 4 colors actually painted onto a 4x3 canvas (index 0..3, each used 3 times) plus
/// a genuinely UNUSED 5th palette entry (index 4) — the unused entry is what lets
/// `palette_mutations_are_observable_in_encoded_bytes` (below) edit the palette table itself
/// without ever making any real pixel unrepresentable.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn small_indexed_snapshot() -> BmpSnapshot {
    let palette = vec![
        BmpPaletteEntry { b: 0, g: 0, r: 255, reserved: 0 },     // 0: red, used
        BmpPaletteEntry { b: 0, g: 255, r: 0, reserved: 0 },     // 1: green, used
        BmpPaletteEntry { b: 255, g: 0, r: 0, reserved: 0 },     // 2: blue, used
        BmpPaletteEntry { b: 10, g: 20, r: 30, reserved: 0 },    // 3: arbitrary, used
        BmpPaletteEntry { b: 200, g: 200, r: 200, reserved: 0 }, // 4: unused by any pixel
    ];
    const USED_COLORS: usize = 4;
    let mut pixels = Vec::with_capacity(4 * 3 * 4);
    for i in 0..(4 * 3) {
        let e = &palette[i % USED_COLORS];
        pixels.extend_from_slice(&[e.r, e.g, e.b, 255]);
    }
    BmpSnapshot { width: 4, height: 3, row_order: BmpRowOrder::BottomUp, bits_per_pixel: 8, colors_used: palette.len() as u32, colors_important: 0, palette, pixels, ..BmpSnapshot::default() }
}

/// 🔬 `indexed_palette_round_trip`: the core bug fix — `encode_bmp` on a snapshot that
/// declares an 8-bit palette must emit a REAL indexed BITMAPINFOHEADER (not a 24-bit
/// fallback), and `decode_bmp` on that output must recover the exact same `bits_per_pixel`,
/// palette, and canonical pixels.
#[semio_framework_async_macros::async_test]
async fn indexed_palette_round_trip() {
    let snap = small_indexed_snapshot();
    let encoded = encode_bmp(&snap).expect("encode indexed");
    assert_eq!(u16::from_le_bytes([encoded[28], encoded[29]]), 8, "on-disk biBitCount must be 8, not the old hardcoded 24");
    let data_offset = u32::from_le_bytes([encoded[10], encoded[11], encoded[12], encoded[13]]) as usize;
    assert_eq!(data_offset, 14 + 40 + snap.palette.len() * 4, "pixel data must start after a real palette table, not immediately after a bare 40-byte header");
    let decoded = decode_bmp(&encoded).expect("decode indexed");
    assert_eq!(decoded.bits_per_pixel, 8);
    assert_eq!(decoded.palette, snap.palette, "palette must round-trip exactly, not be discarded");
    assert_eq!(decoded.pixels, snap.pixels, "canonical pixels must round-trip exactly through the indexed path");
}

/// 🔬 `encode_bmp` falls back to 24-bit `BI_RGB` for a `bits_per_pixel` outside {1,4,8} even
/// when a palette is (nonsensically) present, and for an empty palette regardless of
/// `bits_per_pixel` — `BmpSnapshot::default()`/`demo_bmp_snapshot()` are exactly this case,
/// which is what keeps `fixture_honesty_law` (below) byte-stable across this fix.
#[semio_framework_async_macros::async_test]
async fn no_palette_still_falls_back_to_direct_24bit() {
    let snap = BmpSnapshot { width: 2, height: 2, pixels: vec![1, 2, 3, 255, 4, 5, 6, 255, 7, 8, 9, 255, 10, 11, 12, 255], ..BmpSnapshot::default() };
    assert!(snap.palette.is_empty());
    let encoded = encode_bmp(&snap).expect("encode direct");
    assert_eq!(u16::from_le_bytes([encoded[28], encoded[29]]), 24);
    let data_offset = u32::from_le_bytes([encoded[10], encoded[11], encoded[12], encoded[13]]) as usize;
    assert_eq!(data_offset, 54, "no palette table when the snapshot declares none");
}
//#endregion 🔖️IndexedRoundTrip

//#region 🔖️PaletteMutationObservability
/// 🔬 `palette_mutations_are_observable_in_encoded_bytes`: before this fix, `InsertPaletteEntry`/
/// `RemovePaletteEntry`/`SetPaletteEntry` changed the snapshot and then vanished on encode
/// (always 24-bit, palette dropped outright) — the exact bug this ticket exists to fix. Each
/// mutation here targets `small_indexed_snapshot`'s own deliberately-unused index-4 entry,
/// so every real pixel stays representable (isolating "does the palette table itself change
/// on the wire" from the separate, genuinely-lossy case
/// `unrepresentable_palette_edit_is_reported_not_narrowed` covers below) while still proving
/// each mutation reaches the encoded bytes and survives a real decode.
#[semio_framework_async_macros::async_test]
async fn palette_mutations_are_observable_in_encoded_bytes() {
    let base = small_indexed_snapshot();
    let base_encoded = encode_bmp(&base).expect("encode base");

    // ➕ InsertPaletteEntry: the color table grows by one real entry — observable as both a
    // bigger `data_offset` and a genuinely different byte sequence.
    let mut inserted = base.clone();
    inserted.palette.insert(0, BmpPaletteEntry { b: 99, g: 88, r: 77, reserved: 0 });
    let inserted_encoded = encode_bmp(&inserted).expect("encode after insert");
    assert_ne!(inserted_encoded, base_encoded, "insert-palette-entry must change the re-encoded bytes");
    let inserted_offset = u32::from_le_bytes([inserted_encoded[10], inserted_encoded[11], inserted_encoded[12], inserted_encoded[13]]) as usize;
    assert_eq!(inserted_offset, 14 + 40 + inserted.palette.len() * 4);
    let decoded_inserted = decode_bmp(&inserted_encoded).expect("decode after insert");
    assert_eq!(decoded_inserted.palette, inserted.palette, "the inserted entry must survive to the decoded palette");
    assert_eq!(decoded_inserted.pixels, base.pixels, "insert alone must not change any decoded pixel color (every original color is still reachable)");

    // ➖ RemovePaletteEntry: drop the unused index-4 entry — the table genuinely shrinks
    // (smaller `data_offset`, different bytes) while every real pixel stays representable.
    let mut removed = base.clone();
    let popped = removed.palette.pop().expect("fixture has a palette");
    assert_eq!(popped, BmpPaletteEntry { b: 200, g: 200, r: 200, reserved: 0 }, "must be removing the fixture's own deliberately-unused entry");
    let removed_encoded = encode_bmp(&removed).expect("encode after remove");
    assert_ne!(removed_encoded, base_encoded, "remove-palette-entry must change the re-encoded bytes");
    let removed_offset = u32::from_le_bytes([removed_encoded[10], removed_encoded[11], removed_encoded[12], removed_encoded[13]]) as usize;
    assert_eq!(removed_offset, 14 + 40 + removed.palette.len() * 4);
    let decoded_removed = decode_bmp(&removed_encoded).expect("decode after remove");
    assert_eq!(decoded_removed.palette, removed.palette);
    assert_eq!(decoded_removed.pixels, base.pixels, "removing an unused entry must not change any decoded pixel color");

    // ✏️ SetPaletteEntry: recolor the same unused index-4 entry — the table entry's bytes
    // change in place (same length, different content) while every real pixel is untouched.
    let mut recolored = base.clone();
    recolored.palette[4] = BmpPaletteEntry { b: 1, g: 2, r: 3, reserved: 9 };
    let recolored_encoded = encode_bmp(&recolored).expect("encode after recolor");
    assert_ne!(recolored_encoded, base_encoded, "replace-palette-entry must change the re-encoded bytes");
    assert_eq!(recolored_encoded.len(), base_encoded.len(), "recoloring in place must not change the file's overall length");
    let decoded_recolored = decode_bmp(&recolored_encoded).expect("decode after recolor");
    assert_eq!(decoded_recolored.palette, recolored.palette);
    assert_eq!(decoded_recolored.pixels, base.pixels, "recoloring an unused entry must not change any decoded pixel color");
}

/// 🚫 `unrepresentable_palette_edit_is_reported_not_narrowed`: `SetPaletteEntry`/
/// `RemovePaletteEntry` only ever touch `palette`, never remap `pixels` — so recoloring or
/// removing the ONLY entry that still matches some pixel's canonical color makes that pixel
/// genuinely unrepresentable as an index into the new table. `encode_bmp` must report that
/// with an `Err`, not silently pick the nearest color or fall back to 24-bit behind the
/// caller's back.
#[semio_framework_async_macros::async_test]
async fn unrepresentable_palette_edit_is_reported_not_narrowed() {
    let mut snap = small_indexed_snapshot();
    // Recolor every entry to the SAME single color: none of the four distinct pixel colors
    // this fixture actually uses can possibly survive that.
    for entry in &mut snap.palette {
        *entry = BmpPaletteEntry { b: 1, g: 2, r: 3, reserved: 0 };
    }
    let error = encode_bmp(&snap).expect_err("must report, not silently narrow, an unrepresentable pixel color");
    assert!(error.contains("no matching entry"), "error must name the real cause, got: {error}");
}
//#endregion 🔖️PaletteMutationObservability

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG2: per-artifact conformance laws (this ticket's own recipe §4 checklist item) —
/// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
/// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/
/// `encode_diff` bytes, and the fixture-honesty round-trip. Lives here (the engine's own
/// test region), not any framework file — `m5` auto-discovers the snapshot grammar+
/// `.dsl.semio`/protocol+`.pack.semio` pairs independently
/// (`🧪️fixture/🦀️-sweep.rs`'s `m5_auto_discovery`); these tests are this
/// artifact's OWN early-warning, plus direct coverage of the mutations/diff facets that
/// harness does not auto-discover at all. Mirrors `stdio.png`'s own `conformance_laws`
/// module verbatim in shape.
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

    /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — BMP has no
    /// textual syntax of its own, see that file's own doc comment) recognizes real
    /// `print_dsl` output for the demo snapshot — same preamble-stripped body
    /// reconstruction `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture`
    /// uses, so this is a direct proof this artifact will pass that harness once
    /// graduated, not merely an analogue.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&demo_bmp_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `BmpMutation` variant (`mutations::demo_mutation_cases()`).
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
    /// output for every representative `BmpDiff` (`diff::demo_diff_cases()`), incl. the
    /// empty diff and every collection-triple shape.
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
    /// mutation's `encode_op`, and every demo diff's `encode_diff` — asserting `consumed
    /// == bytes.len()`.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_bmp_snapshot());
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
    /// GENUINE `print_dsl`/`encode_pack` output of `demo_bmp_snapshot()` —
    /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and
    /// the pack twin — so the fixtures can never silently drift back to a fake again (the
    /// pre-this-wave committed fixture WAS a fake "hello" placeholder — see
    /// `demo_bmp_snapshot`'s own doc comment).
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_bmp_snapshot();

        let parsed = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_bmp_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_bmp_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <BmpSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_bmp_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_bmp_snapshot()) drifted from the shipped .pack.semio fixture");
    }

    /// ✅️ `schema_spec_registration_resolves`: `register_schema_specs` genuinely resolves
    /// both the snapshot AND diff schema ids through `dsl::registry::full_resolver().await` once
    /// called (real `BmpSnapshot::__dsl_spec`/`BmpDiff::__dsl_diff_spec`, not fabricated).
    #[semio_framework_async_macros::async_test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn schema_spec_registration_resolves() {
        use dsl::os_pack::cli::SchemaResolver;
        register_schema_specs();
        let resolver = dsl::registry::full_resolver().await;
        assert!(resolver.resolve("stdio.bmp").await.is_some(), "stdio.bmp must resolve");
        assert!(resolver.resolve("stdio.bmp#diff").await.is_some(), "stdio.bmp#diff must resolve");
    }
}
//#endregion 🔖️ConformanceLaws
