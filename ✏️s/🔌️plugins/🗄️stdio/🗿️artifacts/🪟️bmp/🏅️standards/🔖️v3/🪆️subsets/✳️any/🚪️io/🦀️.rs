//! 🚪️ IO stdio.bmp (v3/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_v3::subsets::any::schema::BmpAnalyzer;
    use crate::BmpSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct BmpComposerComposition;

    impl ArtifactComposition for BmpComposerComposition {
        type Snapshot = BmpSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "BmpComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = BmpAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "BmpComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the
// real bmp codec (BITMAPFILEHEADER + BITMAPINFOHEADER) relocated here verbatim (destination
// rule 2: codecs → `🚪️io/`; rule 6: pure format algorithms with no snapshot dependency stay
// WITH the codec here, since they're BMP-specific, not artifact-independent).
//
// Decode reads the FULL BITMAPINFOHEADER (11 real fields, honestly typed on `BmpSnapshot`, see
// `schema::snapshot`) and supports 1/4/8-bit indexed (BGR[A] palette), 16/32-bit
// `BI_BITFIELDS`, 24-bit `BI_RGB`, and 32-bit `BI_RGB` (default full-byte channel masks) —
// pixel data is always canonicalized into an 8-bit RGBA `pixels` buffer (`width * height * 4`
// bytes, row 0 = image top, regardless of the file's on-disk row order). Encode mirrors that
// split: a 1/4/8-bit indexed BITMAPINFOHEADER (real palette, real per-pixel indices) when the
// snapshot declares one (`bits_per_pixel` in {1,4,8} and `palette` non-empty), a 24-bit `BI_RGB`
// bitmap otherwise — both are 40-byte-header, uncompressed, row order honors `row_order`, and
// the remaining metadata fields (resolution, colors used/important) round-trip from the
// snapshot) — see 🚫️EncodeScopeNote below. `BmpEngine` (zero construction sites) deleted
// outright. `register`/`register_artifact_schema`/`register_artifact_inferences`/
// `register_pilot_languages`/`register_schema_specs` kept together here (not dead: `register()`
// is reached by stdio's protected imperative `crate::engine::register()`
// plugin-root call via this standard's own inline `engine` barrel). `empty_bmp_snapshot`/
// `demo_bmp_snapshot` moved to `../🧬️schema`.
use crate::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::{BmpMutation, BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};
use std::collections::HashMap;

//#region ByteIo
const BMP_MAGIC: [u8; 2] = *b"BM";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_u16(b: &[u8], pos: usize) -> Result<u16, String> {
    b.get(pos..pos + 2).map(|s| u16::from_le_bytes([s[0], s[1]])).ok_or_else(|| "bmp: truncated (u16)".into())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_u32(b: &[u8], pos: usize) -> Result<u32, String> {
    b.get(pos..pos + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or_else(|| "bmp: truncated (u32)".into())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_i32(b: &[u8], pos: usize) -> Result<i32, String> {
    b.get(pos..pos + 4).map(|s| i32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or_else(|| "bmp: truncated (i32)".into())
}
//#endregion ByteIo

//#region RowGeometry
/// 📏 BMP scanlines are padded to a 4-byte boundary: `((width*bpp + 31) / 32) * 4`. `pub(crate)`
/// so `../🧬️schema`'s own `demo_bmp_snapshot()` can compute a real `image_size`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn row_bytes(width: u32, bpp: u16) -> usize {
    (width as usize * bpp as usize).div_ceil(32) * 4
}
//#endregion RowGeometry

//#region Bitfields
/// 🧮 `(shift, bit-width)` of a contiguous bitfield mask, used to extract+normalize a channel.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mask_shift_width(mask: u32) -> (u32, u32) {
    if mask == 0 {
        return (0, 0);
    }
    let shift = mask.trailing_zeros();
    let width = (mask >> shift).trailing_ones();
    (shift, width)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn extract_channel(raw: u32, mask: u32) -> u8 {
    let (shift, width) = mask_shift_width(mask);
    if width == 0 {
        return 0;
    }
    let v = (raw & mask) >> shift;
    if width >= 8 {
        (v >> (width - 8)) as u8
    } else {
        let maxval = (1u32 << width) - 1;
        ((v * 255 + maxval / 2) / maxval) as u8
    }
}
//#endregion Bitfields

//#region IndexUnpack
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unpack_index(row: &[u8], x: usize, bpp: u16) -> usize {
    match bpp {
        8 => row[x] as usize,
        4 => {
            let byte = row[x / 2];
            if x.is_multiple_of(2) {
                (byte >> 4) as usize
            } else {
                (byte & 0x0F) as usize
            }
        }
        1 => {
            let byte = row[x / 8];
            let bit = 7 - (x % 8);
            ((byte >> bit) & 1) as usize
        }
        _ => unreachable!("caller only passes 1|4|8"),
    }
}

/// ✍️ The write-side mirror of `unpack_index` — packs a `0..2^bpp` palette index into its
/// sub-byte position within an already zero-initialized row buffer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pack_index(row: &mut [u8], x: usize, bpp: u16, index: u8) {
    match bpp {
        8 => row[x] = index,
        4 => {
            let byte = &mut row[x / 2];
            if x.is_multiple_of(2) {
                *byte = (*byte & 0x0F) | (index << 4);
            } else {
                *byte = (*byte & 0xF0) | (index & 0x0F);
            }
        }
        1 => {
            if index & 1 != 0 {
                let byte = &mut row[x / 8];
                let bit = 7 - (x % 8);
                *byte |= 1 << bit;
            }
        }
        _ => unreachable!("caller only passes 1|4|8"),
    }
}
//#endregion IndexUnpack

//#region Codec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_bmp(bytes: &[u8]) -> Result<BmpSnapshot, String> {
    if bytes.len() < 14 || bytes[0..2] != BMP_MAGIC {
        return Err("bmp: bad signature".into());
    }
    let data_offset = read_u32(bytes, 10)? as usize;
    let header_size = read_u32(bytes, 14)?;
    if (header_size as usize) < 40 {
        return Err(format!("bmp: unsupported info header size {header_size}"));
    }
    let width_i = read_i32(bytes, 18)?;
    let height_i = read_i32(bytes, 22)?;
    // 🧾 The rest of BITMAPINFOHEADER's 11 real fields — read honestly regardless of which
    // branch (empty-sentinel vs. real image) follows, per the recipe's "codec fills what it
    // decodes" rule.
    let planes = read_u16(bytes, 26)?;
    let bpp = read_u16(bytes, 28)?;
    let compression = read_u32(bytes, 30)?;
    let image_size = read_u32(bytes, 34)?;
    let x_pixels_per_meter = read_i32(bytes, 38)?;
    let y_pixels_per_meter = read_i32(bytes, 42)?;
    let colors_used_field = read_u32(bytes, 46)?;
    let colors_important = read_u32(bytes, 50)?;

    if width_i == 0 && height_i == 0 {
        // 🌱 The zero-dimension "empty document" case round-tripped by encode_bmp — no pixel
        // data or palette to read, but the header fields themselves are still real bytes.
        return Ok(BmpSnapshot {
            schema: STDIO_BMP_DOCUMENT_SCHEMA.into(),
            header_size,
            width: 0,
            height: 0,
            row_order: BmpRowOrder::BottomUp,
            planes,
            bits_per_pixel: bpp,
            compression,
            image_size,
            x_pixels_per_meter,
            y_pixels_per_meter,
            colors_used: colors_used_field,
            colors_important,
            palette: Vec::new(),
            pixels: Vec::new(),
        });
    }
    if width_i <= 0 {
        return Err("bmp: non-positive width".into());
    }
    if height_i == 0 {
        return Err("bmp: zero height".into());
    }
    let width = width_i as u32;
    let top_down = height_i < 0;
    let row_order = if top_down { BmpRowOrder::TopDown } else { BmpRowOrder::BottomUp };
    let height = height_i.unsigned_abs();

    if compression != 0 && compression != 3 {
        return Err(format!("bmp: unsupported compression {compression} (only BI_RGB/BI_BITFIELDS are implemented)"));
    }

    let mut cursor = 14 + header_size as usize;
    let mut masks = [0u32; 4]; // r, g, b, a
    if compression == 3 {
        if bpp != 16 && bpp != 32 {
            return Err("bmp: BI_BITFIELDS only valid for 16/32bpp".into());
        }
        if header_size == 40 {
            // 📌 Classic Win9x extension: 3 (sometimes 4) DWORD masks immediately follow the
            // core 40-byte BITMAPINFOHEADER, before the pixel data.
            masks[0] = read_u32(bytes, cursor)?;
            masks[1] = read_u32(bytes, cursor + 4)?;
            masks[2] = read_u32(bytes, cursor + 8)?;
            cursor += 12;
            if cursor + 4 <= data_offset {
                masks[3] = read_u32(bytes, cursor)?;
                cursor += 4;
            }
        } else {
            // 📌 BITMAPV2/V3/V4/V5INFOHEADER embed the masks at fixed offsets inside the header.
            masks[0] = read_u32(bytes, 14 + 40)?;
            masks[1] = read_u32(bytes, 14 + 44)?;
            masks[2] = read_u32(bytes, 14 + 48)?;
            if header_size >= 56 {
                masks[3] = read_u32(bytes, 14 + 52)?;
            }
        }
    } else if bpp == 16 {
        masks = [0x7C00, 0x03E0, 0x001F, 0]; // BI_RGB default: X1R5G5B5
    } else if bpp == 32 {
        masks = [0x00FF0000, 0x0000FF00, 0x000000FF, 0]; // BI_RGB default: 8-8-8, no alpha
    }

    let palette_count = if bpp <= 8 {
        let raw = if colors_used_field != 0 { colors_used_field as usize } else { 1usize << bpp };
        if raw > 1usize << bpp {
            return Err("bmp: colorsUsed exceeds bit-depth capacity".into());
        }
        raw
    } else {
        0
    };
    let mut palette: Vec<BmpPaletteEntry> = Vec::with_capacity(palette_count);
    for i in 0..palette_count {
        let o = cursor + i * 4;
        if o + 4 > bytes.len() || o + 4 > data_offset {
            return Err("bmp: palette truncated".into());
        }
        palette.push(BmpPaletteEntry { b: bytes[o], g: bytes[o + 1], r: bytes[o + 2], reserved: bytes[o + 3] });
    }

    let rb = row_bytes(width, bpp);
    let mut pixels = vec![0u8; width as usize * height as usize * 4];
    for file_row in 0..height as usize {
        let row_off = data_offset + file_row * rb;
        if row_off + rb > bytes.len() {
            return Err("bmp: pixel data truncated".into());
        }
        let row = &bytes[row_off..row_off + rb];
        let out_y = if top_down { file_row } else { height as usize - 1 - file_row };
        match bpp {
            1 | 4 | 8 => {
                for x in 0..width as usize {
                    let idx = unpack_index(row, x, bpp);
                    let pentry = palette.get(idx).ok_or_else(|| format!("bmp: palette index {idx} out of range"))?;
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = pentry.r;
                    pixels[o + 1] = pentry.g;
                    pixels[o + 2] = pentry.b;
                    pixels[o + 3] = 255;
                }
            }
            24 => {
                for x in 0..width as usize {
                    let so = x * 3;
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = row[so + 2];
                    pixels[o + 1] = row[so + 1];
                    pixels[o + 2] = row[so];
                    pixels[o + 3] = 255;
                }
            }
            16 => {
                for x in 0..width as usize {
                    let so = x * 2;
                    let raw = u16::from_le_bytes([row[so], row[so + 1]]) as u32;
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = extract_channel(raw, masks[0]);
                    pixels[o + 1] = extract_channel(raw, masks[1]);
                    pixels[o + 2] = extract_channel(raw, masks[2]);
                    pixels[o + 3] = if masks[3] != 0 { extract_channel(raw, masks[3]) } else { 255 };
                }
            }
            32 => {
                for x in 0..width as usize {
                    let so = x * 4;
                    let raw = u32::from_le_bytes([row[so], row[so + 1], row[so + 2], row[so + 3]]);
                    let o = (out_y * width as usize + x) * 4;
                    pixels[o] = extract_channel(raw, masks[0]);
                    pixels[o + 1] = extract_channel(raw, masks[1]);
                    pixels[o + 2] = extract_channel(raw, masks[2]);
                    pixels[o + 3] = if masks[3] != 0 { extract_channel(raw, masks[3]) } else { 255 };
                }
            }
            _ => return Err(format!("bmp: unsupported bit depth {bpp}")),
        }
    }
    Ok(BmpSnapshot {
        schema: STDIO_BMP_DOCUMENT_SCHEMA.into(),
        header_size,
        width,
        height,
        row_order,
        planes,
        bits_per_pixel: bpp,
        compression,
        image_size,
        x_pixels_per_meter,
        y_pixels_per_meter,
        colors_used: colors_used_field,
        colors_important,
        palette,
        pixels,
    })
}

/// 🚫 EncodeScopeNote: mirrors `decode_bmp`'s own indexed/direct split rather than the old
/// always-24-bit behaviour. When the snapshot DECLARES a palette (`bits_per_pixel` is 1, 4 or 8
/// AND `palette` is non-empty) encode emits a real 1/4/8-bit indexed BITMAPINFOHEADER: the exact
/// `snap.palette` entries, in order, as the on-disk BGR-reserved color table, and per-pixel
/// indices recovered by matching each canonical RGBA pixel's RGB triple against that table
/// (`encode_bmp_indexed`, below). Every other snapshot — `bits_per_pixel` outside 1/4/8, or an
/// empty palette — falls back to the original 24-bit `BI_RGB` direct-color path
/// (`encode_bmp_direct`); 16/32-bit `BI_BITFIELDS` remain decode-only, same scope cut as before.
/// Both paths are 40-byte-header, uncompressed, honor `row_order` (drives
/// the sign of the on-disk `height` field and the row-write direction), and round-trip
/// `x_pixels_per_meter`/`y_pixels_per_meter`/`colors_used`/`colors_important` verbatim from the
/// snapshot. `pixels` is always canonical 8-bit RGBA (row 0 = image top); both paths drop the
/// alpha channel (neither `BI_RGB` variant carries one). The indexed path is the one exception to
/// "verbatim": it writes `palette.len()` as `biClrUsed` rather than `snap.colors_used`, because
/// that header field states the size of the colour table immediately following it and this encoder
/// writes exactly as many entries as the snapshot holds — see the comment at the write site.
///
/// The indexed path can genuinely fail: `pixels` and `palette` are independent fields (a
/// `SetPaletteEntry`/`RemovePaletteEntry` mutation only ever touches `palette`, never remaps
/// `pixels`), so a snapshot can legitimately describe a canonical color no longer present in its
/// own declared palette. That is reported as an `Err` — never silently narrowed to the nearest
/// palette entry, and never silently downgraded to 24-bit behind the caller's back — because
/// either of those would hide the very loss of fidelity the mutation just introduced.
///
/// ⚠️ `SetPixelData` reaches the same state for a DIFFERENT reason — the picture changed, not the
/// table — and there the refusal is contested: the registered `image` reference implements the same
/// declared kind by switching the document to 24-bit `BI_RGB`, so `🪟️mutate-bmp-v3`'s
/// `mutate-replace-pixel-data` row diverges (oracle `storage: direct`, subject an encode error). The
/// declared kind (`../🧬️schema/🧬️mutations/🦀️.rs`: "Replaces the whole decoded
/// canonical-RGBA `pixels` buffer") says nothing about storage, so BOTH sides are extrapolating and
/// neither is a codec bug. Specifying what `replace-pixel-data` means for an indexed BMP — and making
/// both sides implement that one meaning — is the fix; nothing here should be relaxed to hide it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_bmp(snap: &BmpSnapshot) -> Result<Vec<u8>, String> {
    let (w, h) = (snap.width, snap.height);
    let expected = w as usize * h as usize * 4;
    if snap.pixels.len() != expected {
        return Err("bmp: pixels length mismatch (expected width*height*4 RGBA)".into());
    }
    if matches!(snap.bits_per_pixel, 1 | 4 | 8) && !snap.palette.is_empty() {
        encode_bmp_indexed(snap, w, h)
    } else {
        Ok(encode_bmp_direct(snap, w, h))
    }
}

/// 🎨 Indexed encode path — see `encode_bmp`'s own `EncodeScopeNote` for the full contract.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_bmp_indexed(snap: &BmpSnapshot, w: u32, h: u32) -> Result<Vec<u8>, String> {
    let bpp = snap.bits_per_pixel;
    let capacity = 1usize << bpp;
    if snap.palette.len() > capacity {
        return Err(format!("bmp: palette has {} entries, which exceeds the {}-bit capacity of {} — cannot encode without narrowing", snap.palette.len(), bpp, capacity));
    }
    // 🔎 First-match-wins RGB -> index lookup (mirrors `decode_bmp`'s own first-write-wins
    // palette semantics: the earliest entry at a given color is the one every matching pixel
    // resolves to).
    let mut index_of: HashMap<(u8, u8, u8), usize> = HashMap::with_capacity(snap.palette.len());
    for (index, entry) in snap.palette.iter().enumerate() {
        index_of.entry((entry.r, entry.g, entry.b)).or_insert(index);
    }

    let rb = row_bytes(w, bpp);
    let pixel_bytes = rb * h as usize;
    let palette_bytes = snap.palette.len() * 4;
    let data_offset = 14 + 40 + palette_bytes;
    let file_size = data_offset + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);
    out.extend_from_slice(&BMP_MAGIC);
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(data_offset as u32).to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    let height_field: i32 = match snap.row_order {
        BmpRowOrder::BottomUp => h as i32,
        BmpRowOrder::TopDown => -(h as i32),
    };
    out.extend_from_slice(&height_field.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&bpp.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // BI_RGB
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&snap.x_pixels_per_meter.to_le_bytes());
    out.extend_from_slice(&snap.y_pixels_per_meter.to_le_bytes());
    // 🧾 `biClrUsed` states how many entries the colour table that follows actually has (BMP v3
    // BITMAPINFOHEADER). This path writes exactly `snap.palette.len()` of them, so any other value
    // — including a `snap.colors_used` an InsertPaletteEntry/RemovePaletteEntry mutation left
    // behind, since neither maintains that field — would describe a table this encoder did not
    // write, and every reader that sizes the table from the header would then misread the palette.
    out.extend_from_slice(&(snap.palette.len() as u32).to_le_bytes());
    out.extend_from_slice(&snap.colors_important.to_le_bytes());
    for entry in &snap.palette {
        out.push(entry.b);
        out.push(entry.g);
        out.push(entry.r);
        out.push(entry.reserved);
    }
    for file_row in 0..h as usize {
        let src_y = match snap.row_order {
            BmpRowOrder::BottomUp => h as usize - 1 - file_row,
            BmpRowOrder::TopDown => file_row,
        };
        let mut row_buf = vec![0u8; rb];
        for x in 0..w as usize {
            let i = (src_y * w as usize + x) * 4;
            let (r, g, b) = (snap.pixels[i], snap.pixels[i + 1], snap.pixels[i + 2]);
            let index = *index_of
                .get(&(r, g, b))
                .ok_or_else(|| format!("bmp: pixel ({x},{src_y}) is rgb({r},{g},{b}), which has no matching entry in the declared {}-entry palette — cannot encode as {bpp}-bit indexed without narrowing", snap.palette.len()))?;
            pack_index(&mut row_buf, x, bpp, index as u8);
        }
        out.extend_from_slice(&row_buf);
    }
    Ok(out)
}

/// 🎨 Direct-color (24-bit `BI_RGB`) encode path — the original, unconditional encode body, now
/// only reached when the snapshot declares no usable palette. See `encode_bmp`'s own
/// `EncodeScopeNote` for the full contract.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_bmp_direct(snap: &BmpSnapshot, w: u32, h: u32) -> Vec<u8> {
    let rb = row_bytes(w, 24);
    let pixel_bytes = rb * h as usize;
    let file_size = 14 + 40 + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);
    out.extend_from_slice(&BMP_MAGIC);
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    let height_field: i32 = match snap.row_order {
        BmpRowOrder::BottomUp => h as i32,
        BmpRowOrder::TopDown => -(h as i32),
    };
    out.extend_from_slice(&height_field.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // BI_RGB
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&snap.x_pixels_per_meter.to_le_bytes());
    out.extend_from_slice(&snap.y_pixels_per_meter.to_le_bytes());
    out.extend_from_slice(&snap.colors_used.to_le_bytes());
    out.extend_from_slice(&snap.colors_important.to_le_bytes());
    for file_row in 0..h as usize {
        let src_y = match snap.row_order {
            BmpRowOrder::BottomUp => h as usize - 1 - file_row,
            BmpRowOrder::TopDown => file_row,
        };
        let mut row_buf = vec![0u8; rb];
        for x in 0..w as usize {
            let i = (src_y * w as usize + x) * 4;
            let o = x * 3;
            row_buf[o] = snap.pixels[i + 2];
            row_buf[o + 1] = snap.pixels[i + 1];
            row_buf[o + 2] = snap.pixels[i];
        }
        out.extend_from_slice(&row_buf);
    }
    out
}
//#endregion Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    store::register_document_codec(store::ArtifactCodec::of::<BmpSnapshot, BmpMutation>(STDIO_BMP_DOCUMENT_SCHEMA)).expect("static Stdio registration must be available and conflict-free");
}

/// 📇️ P2-FG2: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) —
/// real, non-fabricated calls (unlike json/csv/zip/png's hand-rolled types, `BmpSnapshot`/
/// `BmpDiff` DO carry genuine derived `RecordSpec` constructors:
/// `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` emit `__dsl_spec`/`__dsl_diff_spec`
/// respectively, see ../🪆️subsets/✳️any/🧬️schema/📸️snapshot and 🔺️diff's own doc comments).
/// Covers both the document's own schema id and its `"<doc>#diff"` diff schema id, per design
/// ruling B-R4, `stdio.txt`'s own exemplar pattern. `#[cfg]`-gated to match
/// `os_dsl::registry`'s own `#[cfg(not(target_arch = "wasm32"))]` — the registry simply does not
/// exist as a compiled item on `wasm32`. `BmpMutation`'s own mutations facet is skipped
/// (`dsl::DslOps` gives per-variant specs via `DslVariants`, no single canonical id to register
/// under — `register-schema-spec-one-spec-per-artifact`, this ticket's own recipe §5).
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    semio_framework_plugin::resolve_ready(dsl::registry::register_schema_spec("stdio.bmp", BmpSnapshot::__dsl_spec));
    semio_framework_plugin::resolve_ready(dsl::registry::register_schema_spec("stdio.bmp#diff", crate::schema::diff::BmpDiff::__dsl_diff_spec));
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}

/// 📌️ Registers the full 5-role `LanguageSpec` set (Document/Ops/Diff/Pack/Spr — this ticket's
/// own recipe §4 checklist item, json's own exemplar shape) for handcrafted facet grammars
/// (text) and protocols (binary) — was a single Document-only registration before this wave.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp",
        extension: Some("bmp"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        // 🎫️ The 5-role scheme has no dedicated "diff binary" role even when a real diff
        // protocol file exists (this ticket's own recipe §4 checklist item) — `BmpDiff`'s own
        // `.spk`-container protocol IS real (see ../🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/
        // 📡️.protocol.semio), just not registered here.
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.bmp.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.bmp`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_schema() {
    ::framework_schema::register_artifact_schema_descriptor(crate::schema::bmp_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.bmp.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v_v3::subsets::any::schema::inferences::bmp_artifact_inference_descriptor());
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_v3::subsets::any::schema::BmpComposer as BmpRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<BmpRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
