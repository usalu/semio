//! png-codec — standalone PNG 1.2 codec on top of `png` 0.18.1's own encoder/decoder. Zero
//! dependencies beyond `png` itself (see this crate's own Cargo.toml — its own `[workspace]`,
//! isolated from the repository's root workspace and Cargo.lock).
//!
//! This binary is a READER-based external oracle mechanism for this repository's own PNG 1.2/any
//! mutation vocabulary (`🧪️oracle/🔣️.json`, oracle id `png-png-1-2-mutate-reader`). Every recipe's
//! BEFORE and AFTER document is authored directly as a typed `png::Info` value below — never by
//! executing this repository's own `PngMutation` dispatch/diff — then handed to `png::Encoder` to
//! become real bytes. `project` decodes real bytes back with `png::Decoder`, independent of, and
//! never sharing code with, this repository's own `🧪️oracle/🦀️component.rs` (which computes what
//! a mutation SHOULD produce and is registered `cross-semio-implementation`, not a reader).
//!
//! Two subcommands:
//!   build   <recipe-id> <out-dir>   — writes <out-dir>/<recipe-id>/before.png + after.png
//!   project <path-to-png>           — decodes a real PNG file and prints a typed JSON projection
//!                                     on stdout (header, palette, trns, gamma, chromaticities,
//!                                     srgb intent, physical dims, background, tEXt chunks, and
//!                                     the decoded pixel buffer as hex — the caller hashes the
//!                                     pixel hex into a size+digest pair and drops the raw bytes,
//!                                     per this artifact's `semantic-png-1-2-v1` comparisonProfile).
//!
//! `change-timestamp`, `insert-unknown-chunk` and `remove-unknown-chunk` are still BUILT here (real
//! bytes, via `Writer::write_chunk`'s raw escape hatch — `png::Info` 0.18.1 has no `tIME` field and
//! the decoder skips unrecognised ancillary chunks entirely, src/decoder/stream.rs's own
//! `SkippedAncillaryChunk`) but `project` cannot surface what it wrote back out. Those three are
//! registered `png-1-2-mutate-uncarried` in the oracle manifest, never silently passed. Full
//! research: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️png-reader-witnessability.md

use png::chunk::ChunkType;
use png::text_metadata::TEXtChunk;
use png::{BitDepth, ColorType, Encoder, Info, PixelDimensions, ScaledFloat, SourceChromaticities, SrgbRenderingIntent, Unit};
use std::env;
use std::fs;
use std::io::{BufReader, Cursor};
use std::path::Path;

//#region 🔖️ByteHelpers
fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn ct(fourcc: &[u8; 4]) -> ChunkType {
    ChunkType(*fourcc)
}
//#endregion 🔖️ByteHelpers

//#region 🔖️Document
/// 🧬 One hand-authored document: the typed `Info` the `png` crate's own `Encoder` writes from,
/// the raw pixel sample bytes (never including the per-row filter byte — `Writer::write_image_data`
/// adds that), and any chunk this crate's `Info` has no typed field for at all (`bKGD` — the
/// encoder never writes it even though the decoder reads it into `Info::bkgd` — plus `tIME` and a
/// synthetic unknown ancillary chunk for the two kinds this crate's decoder cannot read back).
#[derive(Clone)]
struct Doc {
    info: Info<'static>,
    pixels: Vec<u8>,
    extra_chunks: Vec<(ChunkType, Vec<u8>)>,
}

fn gradient_rgb(width: u32, height: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            out.push(((i * 17 + 1) % 256) as u8);
            out.push(((i * 29 + 2) % 256) as u8);
            out.push(((i * 53 + 3) % 256) as u8);
        }
    }
    out
}

fn indexed_pixels(width: u32, height: u32, palette_len: u32) -> Vec<u8> {
    (0..width * height).map(|i| (i % palette_len) as u8).collect()
}

fn palette_bytes(colors: &[(u8, u8, u8)]) -> Vec<u8> {
    colors.iter().flat_map(|&(r, g, b)| [r, g, b]).collect()
}

/// 🧬 `Info` is `#[non_exhaustive]` — no struct-literal construction from outside `png` itself, even
/// with `..base` — so every field beyond width/height is set by ASSIGNMENT onto `Info::with_size`.
fn rgb_info(width: u32, height: u32) -> Info<'static> {
    let mut info = Info::with_size(width, height);
    info.color_type = ColorType::Rgb;
    info.bit_depth = BitDepth::Eight;
    info
}

fn indexed_info(width: u32, height: u32, palette: &[(u8, u8, u8)]) -> Info<'static> {
    let mut info = Info::with_size(width, height);
    info.color_type = ColorType::Indexed;
    info.bit_depth = BitDepth::Eight;
    info.palette = Some(palette_bytes(palette).into());
    info
}

/// 🧬 4x2 RGB, 8-bit — the shared starting document for every recipe that does not itself need to
/// vary dimensions/colour type/palette (change-header and replace-palette each build their own).
fn base_rgb() -> Doc {
    let (width, height) = (4, 2);
    Doc { info: rgb_info(width, height), pixels: gradient_rgb(width, height), extra_chunks: vec![] }
}

/// 🧬 4x2 Indexed, 8-bit, 4-entry palette — the shared starting document for replace-palette.
fn base_indexed(palette: &[(u8, u8, u8)]) -> Doc {
    let (width, height) = (4, 2);
    Doc { info: indexed_info(width, height, palette), pixels: indexed_pixels(width, height, palette.len() as u32), extra_chunks: vec![] }
}
//#endregion 🔖️Document

//#region 🔖️Encode — every chunk below is written by `png`'s own `Writer`, never hand-assembled
fn encode_doc(doc: &Doc) -> Vec<u8> {
    let mut out = Cursor::new(Vec::<u8>::new());
    let encoder = Encoder::with_info(&mut out, doc.info.clone()).expect("png::Encoder::with_info accepts this recipe's typed Info");
    let mut writer = encoder.write_header().expect("png::Encoder::write_header writes IHDR + every chunk driven by Info");
    for (chunk_type, data) in &doc.extra_chunks {
        writer.write_chunk(*chunk_type, data).expect("png::Writer::write_chunk writes a length/type/CRC-framed chunk this crate's own decoder can at least skip");
    }
    writer.write_image_data(&doc.pixels).expect("png::Writer::write_image_data filters+deflates+frames IDAT");
    writer.finish().expect("png::Writer::finish writes IEND");
    out.into_inner()
}
//#endregion 🔖️Encode

//#region 🔖️Decode
struct Decoded {
    info: Info<'static>,
    pixels: Vec<u8>,
}

fn decode_doc(bytes: &[u8]) -> Decoded {
    let decoder = png::Decoder::new(BufReader::new(Cursor::new(bytes)));
    let mut reader = decoder.read_info().expect("png::Decoder::read_info reads the signature, IHDR and every chunk before IDAT");
    let mut buf = vec![0u8; reader.output_buffer_size().expect("this crate's own encoder always writes a sized IDAT stream")];
    let output_info = reader.next_frame(&mut buf).expect("png::Reader::next_frame decodes+unfilters+de-deflates IDAT");
    buf.truncate(output_info.buffer_size());
    Decoded { info: reader.info().clone(), pixels: buf }
}
//#endregion 🔖️Decode

//#region 🔖️Json
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn color_type_name(c: ColorType) -> &'static str {
    match c {
        ColorType::Grayscale => "grayscale",
        ColorType::Rgb => "rgb",
        ColorType::Indexed => "indexed",
        ColorType::GrayscaleAlpha => "grayscaleAlpha",
        ColorType::Rgba => "rgba",
    }
}

fn bit_depth_value(b: BitDepth) -> u8 {
    match b {
        BitDepth::One => 1,
        BitDepth::Two => 2,
        BitDepth::Four => 4,
        BitDepth::Eight => 8,
        BitDepth::Sixteen => 16,
    }
}

fn unit_name(u: Unit) -> &'static str {
    match u {
        Unit::Unspecified => "unspecified",
        Unit::Meter => "meter",
    }
}

fn srgb_intent_name(i: SrgbRenderingIntent) -> &'static str {
    match i {
        SrgbRenderingIntent::Perceptual => "perceptual",
        SrgbRenderingIntent::RelativeColorimetric => "relativeColorimetric",
        SrgbRenderingIntent::Saturation => "saturation",
        SrgbRenderingIntent::AbsoluteColorimetric => "absoluteColorimetric",
    }
}

fn palette_json(bytes: &[u8]) -> String {
    let entries: Vec<String> = bytes.chunks(3).map(|c| format!("[{},{},{}]", c[0], c.get(1).copied().unwrap_or(0), c.get(2).copied().unwrap_or(0))).collect();
    format!("[{}]", entries.join(","))
}

fn chromaticities_json(c: &SourceChromaticities) -> String {
    let pair = |p: (ScaledFloat, ScaledFloat)| format!("[{},{}]", p.0.into_scaled(), p.1.into_scaled());
    format!("{{\"white\":{},\"red\":{},\"green\":{},\"blue\":{}}}", pair(c.white), pair(c.red), pair(c.green), pair(c.blue))
}

fn text_chunk_json(t: &TEXtChunk) -> String {
    format!("{{\"keyword\":{},\"text\":{}}}", json_str(&t.keyword), json_str(&t.text))
}

fn doc_json(d: &Decoded) -> String {
    let info = &d.info;
    let header = format!(
        "{{\"width\":{},\"height\":{},\"bitDepth\":{},\"colorType\":{},\"interlaced\":{}}}",
        info.width,
        info.height,
        bit_depth_value(info.bit_depth),
        json_str(color_type_name(info.color_type)),
        info.interlaced
    );
    let palette = info.palette.as_ref().map(|p| palette_json(p)).unwrap_or_else(|| "null".to_string());
    let trns = info.trns.as_ref().map(|t| json_str(&to_hex(t))).unwrap_or_else(|| "null".to_string());
    let gamma = info.gama_chunk.map(|g| g.into_scaled().to_string()).unwrap_or_else(|| "null".to_string());
    let chromaticities = info.chrm_chunk.as_ref().map(chromaticities_json).unwrap_or_else(|| "null".to_string());
    let srgb_intent = info.srgb.map(|s| json_str(srgb_intent_name(s))).unwrap_or_else(|| "null".to_string());
    let physical_dims = info
        .pixel_dims
        .map(|pd| format!("{{\"xppu\":{},\"yppu\":{},\"unit\":{}}}", pd.xppu, pd.yppu, json_str(unit_name(pd.unit))))
        .unwrap_or_else(|| "null".to_string());
    let background = info.bkgd.as_ref().map(|b| json_str(&to_hex(b))).unwrap_or_else(|| "null".to_string());
    let text_chunks: Vec<String> = info.uncompressed_latin1_text.iter().map(text_chunk_json).collect();
    format!(
        "{{\"header\":{},\"palette\":{},\"trns\":{},\"gamma\":{},\"chromaticities\":{},\"srgbIntent\":{},\"physicalDims\":{},\"background\":{},\"textChunks\":[{}],\"pixelsHex\":{}}}",
        header,
        palette,
        trns,
        gamma,
        chromaticities,
        srgb_intent,
        physical_dims,
        background,
        text_chunks.join(","),
        json_str(&to_hex(&d.pixels))
    )
}
//#endregion 🔖️Json

//#region 🔖️Recipes
/// 🧪 One recipe per declared `png-1-2-any` mutation kind — every kind's `outcomes` is `["applied"]`
/// only, so every recipe returns BOTH a before and an after document (no `-rejected-` counterpart
/// exists in this catalog, unlike `avi`). Each AFTER state touches exactly the field(s) that kind's
/// real `PngMutation` variant touches (see `../../../../../../🧬️schema/🧬️mutations/🦀️.rs` and each
/// `📐️<kind>/🛰️component.proto` for the field list) — the values themselves are hand-chosen by this
/// binary, never computed from any mutation dispatch.
fn recipe(id: &str) -> Option<(Doc, Doc)> {
    match id {
        // 🧬 ChangeHeaderMutation{width,height,bit_depth,color_type,interlace} — whole-value IHDR
        // replace. Dimensions change (4x2 -> 6x2); colour type/bit depth/interlace held fixed, since
        // `png` 0.18.1's `Writer::write_image_data` never performs Adam7 interlacing itself, so an
        // `interlaced: true` Info would write a mismatched, unreadable-back IDAT stream — a limit of
        // this specific crate version, not of the mutation.
        "change-header-applied" => {
            let before = base_rgb();
            let (width, height) = (6, 2);
            let after = Doc { info: rgb_info(width, height), pixels: gradient_rgb(width, height), extra_chunks: vec![] };
            Some((before, after))
        }

        // 🧬 ReplacePaletteMutation — whole-value PLTE replace; index bytes (pixels) untouched.
        "replace-palette-applied" => {
            let before = base_indexed(&[(0, 0, 0), (255, 0, 0), (0, 255, 0), (0, 0, 255)]);
            let mut after = before.clone();
            after.info.palette = Some(palette_bytes(&[(255, 255, 0), (0, 255, 255), (255, 0, 255), (32, 32, 32)]).into());
            Some((before, after))
        }

        // 🧬 ChangeTransparencyMutation — tRNS color-key add/replace over an RGB (non-alpha) base.
        "change-transparency-applied" => {
            let before = base_rgb();
            let mut after = before.clone();
            after.info.trns = Some(vec![0x00, 0x0A, 0x00, 0x1D, 0x00, 0x03].into());
            Some((before, after))
        }

        // 🧬 ChangeGammaMutation — gAMA replace (1/2.2 -> 1.0, scaled ×100000).
        "change-gamma-applied" => {
            let mut before = base_rgb();
            before.info.source_gamma = Some(ScaledFloat::from_scaled(45455));
            let mut after = before.clone();
            after.info.source_gamma = Some(ScaledFloat::from_scaled(100000));
            Some((before, after))
        }

        // 🧬 ChangeChromaticitiesMutation — cHRM replace (sRGB primaries -> arbitrary other set).
        "change-chromaticities-applied" => {
            let mut before = base_rgb();
            before.info.source_chromaticities = Some(SourceChromaticities::new((0.3127, 0.3290), (0.6400, 0.3300), (0.3000, 0.6000), (0.1500, 0.0600)));
            let mut after = before.clone();
            after.info.source_chromaticities = Some(SourceChromaticities::new((0.3457, 0.3585), (0.6800, 0.3200), (0.2650, 0.6900), (0.1500, 0.0600)));
            Some((before, after))
        }

        // 🧬 ChangeSrgbIntentMutation — sRGB rendering intent replace.
        "change-srgb-intent-applied" => {
            let mut before = base_rgb();
            before.info.srgb = Some(SrgbRenderingIntent::Perceptual);
            let mut after = before.clone();
            after.info.srgb = Some(SrgbRenderingIntent::RelativeColorimetric);
            Some((before, after))
        }

        // 🧬 ChangePhysicalDimsMutation — pHYs replace.
        "change-physical-dims-applied" => {
            let mut before = base_rgb();
            before.info.pixel_dims = Some(PixelDimensions { xppu: 2835, yppu: 2835, unit: Unit::Meter });
            let mut after = before.clone();
            after.info.pixel_dims = Some(PixelDimensions { xppu: 1000, yppu: 4000, unit: Unit::Unspecified });
            Some((before, after))
        }

        // 🧬 ChangeTimestampMutation — tIME replace. `png::Info` 0.18.1 has NO `tIME` field at all,
        // so both bytes are written through `Writer::write_chunk`'s raw escape hatch with a FIXED,
        // hand-chosen 7-byte payload (year u16BE, month, day, hour, minute, second) — never
        // `SystemTime::now()`. UNCARRIED: this crate's decoder cannot read a tIME chunk back.
        "change-timestamp-applied" => {
            let before = Doc { extra_chunks: vec![(ct(b"tIME"), vec![0x07, 0xE8, 0x01, 0x01, 0x00, 0x00, 0x00])], ..base_rgb() };
            let after = Doc { extra_chunks: vec![(ct(b"tIME"), vec![0x07, 0xE8, 0x06, 0x0F, 0x0C, 0x1E, 0x00])], ..base_rgb() };
            Some((before, after))
        }

        // 🧬 ChangeBackgroundMutation — bKGD replace. `png::Encoder` 0.18.1 has no `set_background`/
        // `Info`-driven bKGD write path at all (its own `encode_header` never checks `info.bkgd`), so
        // both bytes are written through `Writer::write_chunk` — still the crate's own length/type/
        // CRC framing, called between `write_header` (which already wrote IHDR) and
        // `write_image_data` (satisfying bKGD's spec position: after PLTE if any, before IDAT). RGB
        // format is 3x u16BE (R,G,B), 6 bytes. WITNESSABLE: the decoder DOES read bKGD into `Info::bkgd`.
        "change-background-applied" => {
            let before = Doc { extra_chunks: vec![(ct(b"bKGD"), vec![0x00, 0xFF, 0x00, 0x80, 0x00, 0x40])], ..base_rgb() };
            let after = Doc { extra_chunks: vec![(ct(b"bKGD"), vec![0x00, 0x10, 0x00, 0x20, 0x00, 0x30])], ..base_rgb() };
            Some((before, after))
        }

        // 🧬 InsertTextChunkMutation — no tEXt -> one tEXt chunk.
        "insert-text-chunk-applied" => {
            let before = base_rgb();
            let mut after = before.clone();
            after.info.uncompressed_latin1_text.push(TEXtChunk { keyword: "Comment".into(), text: "hello from png-codec".into() });
            Some((before, after))
        }

        // 🧬 RemoveTextChunkMutation — one tEXt chunk -> none.
        "remove-text-chunk-applied" => {
            let mut before = base_rgb();
            before.info.uncompressed_latin1_text.push(TEXtChunk { keyword: "Comment".into(), text: "hello from png-codec".into() });
            let mut after = before.clone();
            after.info.uncompressed_latin1_text.clear();
            Some((before, after))
        }

        // 🧬 ReplaceTextChunkMutation — the one tEXt chunk's text (keyword held fixed) is replaced.
        "replace-text-chunk-applied" => {
            let mut before = base_rgb();
            before.info.uncompressed_latin1_text.push(TEXtChunk { keyword: "Comment".into(), text: "before-value".into() });
            let mut after = before.clone();
            after.info.uncompressed_latin1_text[0].text = "after-value".into();
            Some((before, after))
        }

        // 🧬 ReplacePixelsMutation — same header, disjoint pixel bytes.
        "replace-pixels-applied" => {
            let before = base_rgb();
            let mut after = before.clone();
            after.pixels = before.pixels.iter().map(|b| 255 - b).collect();
            Some((before, after))
        }

        // 🧬 InsertUnknownChunkMutation — no unrecognised chunk -> one. Fourcc `prVt`: lowercase
        // first byte (ancillary — safe to ignore), uppercase second/third (private, reserved-bit
        // conformant), lowercase fourth (not safe-to-copy blindly) — a private, unregistered,
        // ancillary chunk type this crate's decoder never assigns to any `Info` field. UNCARRIED.
        "insert-unknown-chunk-applied" => {
            let before = base_rgb();
            let after = Doc { extra_chunks: vec![(ct(b"prVt"), vec![0xAA, 0xBB, 0xCC])], ..base_rgb() };
            Some((before, after))
        }

        // 🧬 RemoveUnknownChunkMutation — one unrecognised chunk -> none. UNCARRIED.
        "remove-unknown-chunk-applied" => {
            let before = Doc { extra_chunks: vec![(ct(b"prVt"), vec![0xAA, 0xBB, 0xCC])], ..base_rgb() };
            let after = base_rgb();
            Some((before, after))
        }

        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &[
    "change-header-applied",
    "replace-palette-applied",
    "change-transparency-applied",
    "change-gamma-applied",
    "change-chromaticities-applied",
    "change-srgb-intent-applied",
    "change-physical-dims-applied",
    "change-timestamp-applied",
    "change-background-applied",
    "insert-text-chunk-applied",
    "remove-text-chunk-applied",
    "replace-text-chunk-applied",
    "replace-pixels-applied",
    "insert-unknown-chunk-applied",
    "remove-unknown-chunk-applied",
];
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str) -> i32 {
    let Some((before, after)) = recipe(id) else {
        eprintln!("[png-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(id);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("before.png"), encode_doc(&before)).expect("write before.png");
    fs::write(dir.join("after.png"), encode_doc(&after)).expect("write after.png");
    eprintln!("[png-codec] {id}: before.png + after.png -> {}", dir.display());
    0
}

fn cmd_project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[png-codec] cannot read {path}: {e}");
            return 1;
        }
    };
    let decoded = decode_doc(&bytes);
    println!("{}", doc_json(&decoded));
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: png-codec build <recipe-id> <out-dir>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: png-codec project <path-to-png>");
                std::process::exit(2);
            };
            cmd_project(path)
        }
        Some("list-recipes") => {
            for id in RECIPE_IDS {
                println!("{id}");
            }
            0
        }
        _ => {
            eprintln!("usage: png-codec build <recipe-id> <out-dir> | project <path-to-png> | list-recipes");
            2
        }
    };
    std::process::exit(code);
}
//#endregion 🔖️Entry

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_recipe_id_resolves() {
        for id in RECIPE_IDS {
            assert!(recipe(id).is_some(), "recipe {id} must resolve");
        }
    }

    #[test]
    fn every_recipe_round_trips_before_and_after() {
        for id in RECIPE_IDS {
            let (before, after) = recipe(id).unwrap();
            let before_bytes = encode_doc(&before);
            let after_bytes = encode_doc(&after);
            let decoded_before = decode_doc(&before_bytes);
            let decoded_after = decode_doc(&after_bytes);
            assert_eq!(decoded_before.info.width, before.info.width, "{id} before width round-trips");
            assert_eq!(decoded_after.info.width, after.info.width, "{id} after width round-trips");
        }
    }

    #[test]
    fn change_gamma_recipe_actually_differs() {
        let (before, after) = recipe("change-gamma-applied").unwrap();
        let decoded_before = decode_doc(&encode_doc(&before));
        let decoded_after = decode_doc(&encode_doc(&after));
        assert_ne!(decoded_before.info.gama_chunk.map(|g| g.into_scaled()), decoded_after.info.gama_chunk.map(|g| g.into_scaled()));
    }
}
//#endregion 🔖️Tests
