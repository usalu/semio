//! 🚪️ IO stdio.bmp (v3/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::bmp::standards::v_v3::subsets::any::schema::BmpAnalyzer;
    use crate::artifacts::bmp::BmpSnapshot;
    use semio_framework_plugin::ArtifactAnalyzer as _;
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

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
// bytes, row 0 = image top, regardless of the file's on-disk row order). Encode always emits a
// 24-bit `BI_RGB`, 40-byte-header, uncompressed bitmap (row order honors `row_order`; all other
// metadata fields — resolution, colors used/important, image size — round-trip from the
// snapshot) — see 🚫️EncodeScopeNote below. `BmpEngine` (zero construction sites) deleted
// outright. `register`/`register_artifact_schema`/`register_artifact_inferences`/
// `register_pilot_languages`/`register_schema_specs` kept together here (not dead: `register()`
// is reached by stdio's protected imperative `crate::artifacts::bmp::engine::register()`
// plugin-root call via this standard's own inline `engine` barrel). `empty_bmp_snapshot`/
// `demo_bmp_snapshot` moved to `../🧬️schema`.
use crate::artifacts::bmp::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::artifacts::bmp::{BmpMutation, BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

//#region ByteIo
const BMP_MAGIC: [u8; 2] = *b"BM";

fn read_u16(b: &[u8], pos: usize) -> Result<u16, String> {
    b.get(pos..pos + 2).map(|s| u16::from_le_bytes([s[0], s[1]])).ok_or_else(|| "bmp: truncated (u16)".into())
}
fn read_u32(b: &[u8], pos: usize) -> Result<u32, String> {
    b.get(pos..pos + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or_else(|| "bmp: truncated (u32)".into())
}
fn read_i32(b: &[u8], pos: usize) -> Result<i32, String> {
    b.get(pos..pos + 4).map(|s| i32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or_else(|| "bmp: truncated (i32)".into())
}
//#endregion ByteIo

//#region RowGeometry
/// 📏 BMP scanlines are padded to a 4-byte boundary: `((width*bpp + 31) / 32) * 4`. `pub(crate)`
/// so `../🧬️schema`'s own `demo_bmp_snapshot()` can compute a real `image_size`.
pub(crate) fn row_bytes(width: u32, bpp: u16) -> usize {
    (((width as usize * bpp as usize) + 31) / 32) * 4
}
//#endregion RowGeometry

//#region Bitfields
/// 🧮 `(shift, bit-width)` of a contiguous bitfield mask, used to extract+normalize a channel.
fn mask_shift_width(mask: u32) -> (u32, u32) {
    if mask == 0 {
        return (0, 0);
    }
    let shift = mask.trailing_zeros();
    let width = (mask >> shift).trailing_ones();
    (shift, width)
}

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
fn unpack_index(row: &[u8], x: usize, bpp: u16) -> usize {
    match bpp {
        8 => row[x] as usize,
        4 => {
            let byte = row[x / 2];
            if x % 2 == 0 {
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
//#endregion IndexUnpack

//#region Codec
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

/// 🚫 EncodeScopeNote: always emits 24-bit `BI_RGB`, a standard 40-byte BITMAPINFOHEADER,
/// uncompressed (`header_size`/`planes`/`bits_per_pixel`/`compression` are therefore FIXED on
/// output, not read from the snapshot — decode supports the wider palette/bitfields/extended-
/// header input diversity documented above, encode does not attempt to reproduce it). `row_order`
/// IS honored (drives the sign of the on-disk `height` field and the row-write direction); the
/// remaining metadata fields (`x_pixels_per_meter`, `y_pixels_per_meter`, `colors_used`,
/// `colors_important`) round-trip verbatim from the snapshot. `pixels` is treated as canonical
/// 8-bit RGBA (row 0 = image top); encode drops the alpha channel (`BI_RGB` has none) and does
/// not emit a palette section (24-bit has none, even if `snap.palette` is non-empty — that
/// metadata simply doesn't apply to this encode target). A real implementation could reasonably
/// restrict *encode* to 24/32-bit only while *decode* covers every depth, which is exactly the
/// scope cut made here.
pub fn encode_bmp(snap: &BmpSnapshot) -> Result<Vec<u8>, String> {
    let (w, h) = (snap.width, snap.height);
    let expected = w as usize * h as usize * 4;
    if snap.pixels.len() != expected {
        return Err("bmp: pixels length mismatch (expected width*height*4 RGBA)".into());
    }
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
    Ok(out)
}
//#endregion Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::bmp::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<BmpSnapshot, BmpMutation>(STDIO_BMP_DOCUMENT_SCHEMA));
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
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.bmp", BmpSnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.bmp#diff", crate::artifacts::bmp::schema::diff::BmpDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}

/// 📌️ Registers the full 5-role `LanguageSpec` set (Document/Ops/Diff/Pack/Spr — this ticket's
/// own recipe §4 checklist item, json's own exemplar shape) for handcrafted facet grammars
/// (text) and protocols (binary) — was a single Document-only registration before this wave.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp",
        extension: Some("bmp"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::bmp::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::bmp::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::bmp::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::bmp::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::bmp::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::bmp::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::bmp::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::bmp::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::bmp::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::bmp::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        // 🎫️ The 5-role scheme has no dedicated "diff binary" role even when a real diff
        // protocol file exists (this ticket's own recipe §4 checklist item) — `BmpDiff`'s own
        // `.spk`-container protocol IS real (see ../🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/
        // 📡️component.protocol.semio), just not registered here.
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
        protocol: Some(crate::artifacts::bmp::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::bmp::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.bmp.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::bmp::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::bmp::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.bmp.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.bmp`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::bmp::schema::bmp_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.bmp.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::bmp::standards::v_v3::subsets::any::schema::inferences::bmp_artifact_inference_descriptor());
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::bmp::schema::{demo_bmp_snapshot, empty_bmp_snapshot};

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

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_bmp_snapshot();
        assert_eq!(snapshot.schema, STDIO_BMP_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
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
    #[test]
    fn row_bytes_padding_is_exact() {
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
    #[test]
    fn gradient_checkerboard_24bit_round_trip() {
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

    #[test]
    fn indexed_4bit_palette_round_trip() {
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
    #[test]
    fn bitfields_16bit_555_round_trip() {
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

    #[test]
    fn sniff_rejects_non_bmp_bytes() {
        let err = decode_bmp(b"not a bmp at all").unwrap_err();
        assert!(err.contains("signature"));
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔬 `codec_retention_law`: decode(encode(snap)) is byte-preserving for every field encode
    /// actually controls — `row_order` (both directions), metadata (`x/y_pixels_per_meter`,
    /// `colors_used`, `colors_important`, `image_size`), and pixels — while the DOCUMENTED
    /// EncodeScopeNote normalization (`header_size`→40, `planes`→1, `bits_per_pixel`→24,
    /// `compression`→0) is asserted explicitly rather than silently ignored.
    #[test]
    fn codec_retention_law() {
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

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG2: per-artifact conformance laws (this ticket's own recipe §4 checklist item) —
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/
    /// `encode_diff` bytes, and the fixture-honesty round-trip. Lives here (the engine's own
    /// test region), not any framework file — `m5` auto-discovers the snapshot grammar+
    /// `.dsl.semio`/protocol+`.pack.semio` pairs independently
    /// (`🧪️fixture-sweep/🦀️component.rs`'s `m5_auto_discovery`); these tests are this
    /// artifact's OWN early-warning, plus direct coverage of the mutations/diff facets that
    /// harness does not auto-discover at all. Mirrors `stdio.png`'s own `conformance_laws`
    /// module verbatim in shape.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::bmp::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio`
        /// files parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a
        /// clearer message).
        #[test]
        fn committed_facet_files_parse() {
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
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_bmp_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `BmpMutation` variant (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
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
        #[test]
        fn diff_grammar_conformance_law() {
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
        #[test]
        fn protocol_walk_law() {
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
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_bmp_snapshot();

            let parsed = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_bmp_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_bmp_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <BmpSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_bmp_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_bmp_snapshot()) drifted from the shipped .pack.semio fixture");
        }

        /// ✅️ `schema_spec_registration_resolves`: `register_schema_specs` genuinely resolves
        /// both the snapshot AND diff schema ids through `dsl::registry::full_resolver()` once
        /// called (real `BmpSnapshot::__dsl_spec`/`BmpDiff::__dsl_diff_spec`, not fabricated).
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn schema_spec_registration_resolves() {
            use dsl::os_pack::cli::SchemaResolver;
            register_schema_specs();
            let resolver = dsl::registry::full_resolver();
            assert!(resolver.resolve("stdio.bmp").is_some(), "stdio.bmp must resolve");
            assert!(resolver.resolve("stdio.bmp#diff").is_some(), "stdio.bmp#diff must resolve");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::bmp::standards::v_v3::subsets::any::schema::BmpComposer as BmpRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<BmpRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
