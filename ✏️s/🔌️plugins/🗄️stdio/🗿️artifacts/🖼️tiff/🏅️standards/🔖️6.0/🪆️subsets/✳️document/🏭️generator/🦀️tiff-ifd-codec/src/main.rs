//! tiff-ifd-codec — standalone reader/writer on top of `tiff` 0.11's own IFD/tag decoder+encoder
//! (`tiff::decoder::Decoder`, `tiff::encoder::TiffEncoder`). Zero dependencies beyond `tiff`
//! itself (see this crate's own Cargo.toml — its own `[workspace]`, isolated from the
//! repository's root workspace and Cargo.lock).
//!
//! Unlike the sibling `riff-avi-codec` (which composes a GENERIC container crate and hand-writes
//! the format-specific field layout itself), this codec depends on `tiff` DIRECTLY: investigation
//! of the vendored crate source (`~/.cargo/registry/src/*/tiff-0.11.3/src/{decoder,encoder}/mod.rs`)
//! showed `tiff` itself already exposes real multi-IFD chain navigation (`more_images`/
//! `next_image`/`seek_to_image`), whole-IFD tag enumeration (`tag_iter`, returning real `Tag`/
//! `Value` pairs including an `unknown(u16)` tag variant for non-baseline tags) and the file's
//! byte-order mark (`byte_order`) — the exact structural vocabulary `insert-ifd`/`remove-ifd`/
//! `replace-tag`/`remove-tag` need, and which `image` 0.25's own public TIFF surface does NOT
//! expose (confirmed by reading `image`'s vendored `src/codecs/tiff/mod.rs`: `TiffDecoder::new`
//! only ever reads the first IFD, and `TiffEncoder::write_image` always emits exactly one).
//! `image` is therefore not a dependency of this crate at all — `tiff` alone, which is what
//! backs `image`'s own TIFF support, gives full IFD/tag/byte-order visibility on both the read
//! and the write side, so there is nothing left for a second, higher-level layer to add.
//!
//! One real limitation this investigation also confirmed (documented in this session's own
//! written report, ticket 26/08/27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING): `tiff`
//! 0.11.3's ENCODER hardcodes the byte-order mark to the compiling target's native endianness at
//! compile time (`#[cfg(target_endian = "little"|"big")]` in `src/encoder/writer.rs`
//! `write_tiff_header`/`write_bigtiff_header`), never runtime-selectable. Every platform this
//! oracle is registered for (`darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`,
//! `win32-x64`) is little-endian, so this library can only ever WRITE `II` (little-endian) TIFF
//! files — it cannot produce a `MM` (big-endian) "after" fixture for `change-byte-order` on any
//! of them. `cmd_build("change-byte-order-applied", ..)` below documents this and refuses rather
//! than hand-rolling a byte-swapped file outside the library (which this ticket's fixture rule
//! forbids: fixtures must be built BY the library).
//!
//! Every recipe's BEFORE and (where the library can actually produce it) AFTER `.tiff` bytes are
//! built DIRECTLY by this binary's own typed `IfdSpec`/`write_doc` below, handed straight to
//! `tiff::encoder::TiffEncoder` — never by "applying" this repository's own `TiffMutation`
//! dispatch, and never by importing this subset's own `🚪️io::{decode_tiff, encode_tiff}` or its
//! `🦀️oracle.rs` (that module computes what a mutation SHOULD produce and is read-only
//! spec reference for this file, never a dependency).
//!
//! Two subcommands:
//!   build   <recipe-id> <out-dir>   — writes <out-dir>/<recipe-id>/before.tiff [and after.tiff]
//!   project <path-to-tiff>          — decodes a real TIFF file and prints a typed JSON projection
//!                                     on stdout (byte order, every IFD's full tag list in file
//!                                     order, plus a size+digest of that IFD's decoded raster when
//!                                     it carries a well-formed baseline image).

use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;

use tiff::decoder::ifd::Value;
use tiff::decoder::Decoder;
use tiff::encoder::colortype::RGB8;
use tiff::encoder::TiffEncoder;
use tiff::tags::{ByteOrder, Tag};

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

/// #️⃣️ Dependency-free FNV-1a 64-bit digest, hex-formatted — same choice and same rationale the
/// sibling `🦀️oracle.rs` documents for its own `samplesDigest`: TIFF is lossless, so an
/// exact digest is the right compact stand-in for "every sample survived" without materializing a
/// `Json::Number` per byte.
fn fnv1a_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

/// 🔁️ `tiff::decoder::ifd::Value` is `#[non_exhaustive]`, so this must carry a wildcard arm.
/// Deliberately preserves the type distinction (byte vs short vs ascii vs rational, …) rather than
/// collapsing everything to plain numbers, so a comparison diff names the exact kind that changed,
/// not just a numeric mismatch.
#[allow(deprecated)]
fn value_json(v: &Value) -> String {
    match v {
        Value::Byte(x) => format!("{{\"kind\":\"byte\",\"value\":{x}}}"),
        Value::Short(x) => format!("{{\"kind\":\"short\",\"value\":{x}}}"),
        Value::SignedByte(x) => format!("{{\"kind\":\"sbyte\",\"value\":{x}}}"),
        Value::SignedShort(x) => format!("{{\"kind\":\"sshort\",\"value\":{x}}}"),
        Value::Signed(x) => format!("{{\"kind\":\"slong\",\"value\":{x}}}"),
        Value::SignedBig(x) => format!("{{\"kind\":\"sbig\",\"value\":{x}}}"),
        Value::Unsigned(x) => format!("{{\"kind\":\"long\",\"value\":{x}}}"),
        Value::UnsignedBig(x) => format!("{{\"kind\":\"big\",\"value\":{x}}}"),
        Value::Float(x) => format!("{{\"kind\":\"float\",\"value\":{x}}}"),
        Value::Double(x) => format!("{{\"kind\":\"double\",\"value\":{x}}}"),
        Value::Rational(n, d) => format!("{{\"kind\":\"rational\",\"n\":{n},\"d\":{d}}}"),
        Value::RationalBig(n, d) => format!("{{\"kind\":\"rationalBig\",\"n\":{n},\"d\":{d}}}"),
        Value::SRational(n, d) => format!("{{\"kind\":\"srational\",\"n\":{n},\"d\":{d}}}"),
        Value::SRationalBig(n, d) => format!("{{\"kind\":\"srationalBig\",\"n\":{n},\"d\":{d}}}"),
        Value::Ascii(s) => format!("{{\"kind\":\"ascii\",\"value\":{}}}", json_str(s)),
        Value::Ifd(x) => format!("{{\"kind\":\"ifd\",\"value\":{x}}}"),
        Value::IfdBig(x) => format!("{{\"kind\":\"ifdBig\",\"value\":{x}}}"),
        Value::List(items) => {
            let parts: Vec<String> = items.iter().map(value_json).collect();
            format!("{{\"kind\":\"list\",\"items\":[{}]}}", parts.join(","))
        }
        _ => "{\"kind\":\"unrecognized\"}".to_string(),
    }
}
//#endregion 🔖️Json

//#region 🔖️Write — every "after" state below is a literal typed value this binary chose, never
//  computed from an input document by executing this repository's own mutation dispatch.
/// 🧬 One IFD's own content — `description` is deliberately NOT one of the tags
/// `tiff::encoder::ImageEncoder::new` auto-writes (`ImageWidth`/`ImageLength`/`Compression`/
/// `Predictor`/`PhotometricInterpretation`/`RowsPerStrip`/`SamplesPerPixel`/`XResolution`/
/// `YResolution`/`ResolutionUnit`), so adding/removing it never collides with the encoder's own
/// baseline tags — the ONLY field `replace-tag`/`remove-tag`'s recipes touch.
struct IfdSpec {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    description: Option<&'static str>,
}

/// 🎨 Deterministic RGB8 fill — no wall-clock/process-global state anywhere in this binary, so
/// regenerating a recipe alone or as part of the whole corpus produces byte-identical output
/// every time (the reproducibility trap this ticket's own `📓️reproducibility.md` documents for
/// OCCT's STEP timestamps does not apply here: `tiff::encoder::ImageEncoder::new` writes only
/// `XResolution`/`YResolution` as constant `Rational{1,1}` — no `DateTime`/`Software` stamp).
fn fill(width: u32, height: u32, seed: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(width as usize * height as usize * 3);
    for y in 0..height {
        for x in 0..width {
            out.push(((x * 40) as u8).wrapping_add(seed));
            out.push(((y * 60) as u8).wrapping_add(seed));
            out.push((((x + y) * 20) as u8).wrapping_add(seed));
        }
    }
    out
}

fn write_doc(ifds: &[IfdSpec]) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    {
        let mut encoder = TiffEncoder::new(&mut cursor).expect("tiff: create encoder");
        for spec in ifds {
            let mut image = encoder.new_image::<RGB8>(spec.width, spec.height).expect("tiff: new_image");
            if let Some(desc) = spec.description {
                image.encoder().write_tag(Tag::ImageDescription, desc).expect("tiff: write ImageDescription");
            }
            image.write_data(&spec.pixels).expect("tiff: write pixel data");
        }
    }
    cursor.into_inner()
}
//#endregion 🔖️Write

//#region 🔖️Read
/// 📥️ Walks the WHOLE IFD chain (`more_images`/`next_image`), collecting every IFD's tags (file
/// order, `StripOffsets`/`StripByteCounts` excluded — layout-computed offsets into THIS file, never
/// semantic content, same exclusion the sibling `🦀️oracle.rs` documents for its own
/// projection) plus, where the IFD decodes as a well-formed baseline image, a size+digest of its
/// raster — never raw sample bytes past this function, per this artifact's opaque-payload
/// convention for large pixel data.
fn project(path: &str) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let mut dec = Decoder::new(Cursor::new(bytes)).map_err(|e| format!("tiff::decoder::Decoder::new: {e}"))?;
    let byte_order = match dec.byte_order() {
        ByteOrder::LittleEndian => "little-endian",
        ByteOrder::BigEndian => "big-endian",
    };

    let mut ifds_json: Vec<String> = Vec::new();
    let mut index = 0usize;
    loop {
        let mut entries: Vec<(u16, Value)> = Vec::new();
        for item in dec.tag_iter() {
            let (tag, value) = item.map_err(|e| format!("tag_iter ifd {index}: {e}"))?;
            let id = tag.to_u16();
            if id == Tag::StripOffsets.to_u16() || id == Tag::StripByteCounts.to_u16() {
                continue;
            }
            entries.push((id, value));
        }
        entries.sort_by_key(|(t, _)| *t);
        let entries_json: Vec<String> = entries.iter().map(|(t, v)| format!("{{\"tag\":{t},\"value\":{}}}", value_json(v))).collect();

        let raster_json = match (dec.dimensions(), dec.colortype(), dec.read_image()) {
            (Ok((width, height)), Ok(color_type), Ok(mut result)) => {
                let bytes = result.as_buffer(0).as_bytes().to_vec();
                format!(
                    "{{\"width\":{width},\"height\":{height},\"colorType\":{},\"sampleByteLength\":{},\"samplesDigest\":\"fnv1a64:{}\"}}",
                    json_str(&format!("{color_type:?}")),
                    bytes.len(),
                    fnv1a_hex(&bytes)
                )
            }
            _ => "null".to_string(),
        };

        ifds_json.push(format!("{{\"index\":{index},\"entries\":[{}],\"raster\":{raster_json}}}", entries_json.join(",")));

        if !dec.more_images() {
            break;
        }
        dec.next_image().map_err(|e| format!("next_image after ifd {index}: {e}"))?;
        index += 1;
    }

    Ok(format!(
        "{{\"format\":\"tiff\",\"byteOrder\":{},\"ifdCount\":{},\"ifds\":[{}]}}",
        json_str(byte_order),
        ifds_json.len(),
        ifds_json.join(",")
    ))
}
//#endregion 🔖️Read

//#region 🔖️Recipes
/// 🧪 One recipe: BEFORE always, AFTER only when this library can actually produce it. Every
/// AFTER state below is a hand-picked literal value, never the result of executing this
/// repository's own `TiffMutation::diff`/`apply`.
fn recipe(id: &str) -> Option<(Vec<IfdSpec>, Vec<IfdSpec>)> {
    match id {
        // 🧬 InsertIfd{index: 1} — a second, smaller IFD is appended after the first.
        "insert-ifd-applied" => Some((
            vec![IfdSpec { width: 4, height: 3, pixels: fill(4, 3, 0), description: None }],
            vec![IfdSpec { width: 4, height: 3, pixels: fill(4, 3, 0), description: None }, IfdSpec { width: 2, height: 2, pixels: fill(2, 2, 100), description: None }],
        )),

        // 🧬 RemoveIfd{index: 1} — the second IFD of a two-IFD document is dropped.
        "remove-ifd-applied" => Some((
            vec![IfdSpec { width: 4, height: 3, pixels: fill(4, 3, 0), description: None }, IfdSpec { width: 2, height: 2, pixels: fill(2, 2, 100), description: None }],
            vec![IfdSpec { width: 4, height: 3, pixels: fill(4, 3, 0), description: None }],
        )),

        // 🧬 ReplaceTag{ifdIndex: 0, tag: ImageDescription, ..} — the tag's VALUE changes; every
        // other tag (including the raster) is untouched.
        "replace-tag-applied" => Some((
            vec![IfdSpec { width: 3, height: 2, pixels: fill(3, 2, 0), description: Some("original scan") }],
            vec![IfdSpec { width: 3, height: 2, pixels: fill(3, 2, 0), description: Some("rescanned copy") }],
        )),

        // 🧬 RemoveTag{ifdIndex: 0, tag: ImageDescription} — the tag is omitted entirely on encode.
        "remove-tag-applied" => Some((
            vec![IfdSpec { width: 3, height: 2, pixels: fill(3, 2, 0), description: Some("scan notes") }],
            vec![IfdSpec { width: 3, height: 2, pixels: fill(3, 2, 0), description: None }],
        )),

        // 🧬 ReplacePixels{ifdIndex: 0, ..} — same IFD 0 dimensions/tags, wholly different raster.
        "replace-pixels-applied" => Some((
            vec![IfdSpec { width: 4, height: 4, pixels: fill(4, 4, 0), description: None }],
            vec![IfdSpec { width: 4, height: 4, pixels: fill(4, 4, 200), description: None }],
        )),

        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &["change-byte-order-applied", "insert-ifd-applied", "remove-ifd-applied", "replace-tag-applied", "remove-tag-applied", "replace-pixels-applied"];
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str) -> i32 {
    if id == "change-byte-order-applied" {
        eprintln!(
            "[tiff-ifd-codec] {id}: REFUSED — tiff 0.11.3's encoder hardcodes the byte-order mark \
             to the compiling target's native endianness at compile time \
             (#[cfg(target_endian=\"little\"|\"big\")] in src/encoder/writer.rs write_tiff_header/ \
             write_bigtiff_header), never runtime-selectable. Every platform this oracle targets \
             (darwin-arm64, darwin-x64, linux-x64, linux-arm64, win32-x64) is little-endian, so \
             this library can only ever WRITE II (little-endian) TIFF bytes on them — it cannot \
             produce an MM (big-endian) after.tiff without this binary hand-swapping bytes outside \
             the library, which this ticket's fixture-authoring rule forbids. No before.tiff or \
             after.tiff written for this recipe."
        );
        return 1;
    }
    let Some((before, after)) = recipe(id) else {
        eprintln!("[tiff-ifd-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(id);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("before.tiff"), write_doc(&before)).expect("write before.tiff");
    fs::write(dir.join("after.tiff"), write_doc(&after)).expect("write after.tiff");
    eprintln!("[tiff-ifd-codec] {id}: before.tiff + after.tiff -> {}", dir.display());
    0
}

fn cmd_project(path: &str) -> i32 {
    match project(path) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(e) => {
            eprintln!("[tiff-ifd-codec] project {path}: {e}");
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: tiff-ifd-codec build <recipe-id> <out-dir>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: tiff-ifd-codec project <path-to-tiff>");
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
            eprintln!("usage: tiff-ifd-codec build <recipe-id> <out-dir> | project <path-to-tiff> | list-recipes");
            2
        }
    };
    std::process::exit(code);
}
//#endregion 🔖️Entry

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_applied_recipe_id_resolves_except_the_refused_one() {
        for id in RECIPE_IDS {
            if *id == "change-byte-order-applied" {
                assert!(recipe(id).is_none(), "the refused recipe must not resolve a buildable pair");
                continue;
            }
            assert!(recipe(id).is_some(), "recipe {id} must resolve");
        }
    }

    #[test]
    fn encode_decode_round_trips_a_two_ifd_document() {
        let ifds = vec![IfdSpec { width: 4, height: 3, pixels: fill(4, 3, 0), description: Some("hi") }, IfdSpec { width: 2, height: 2, pixels: fill(2, 2, 9), description: None }];
        let bytes = write_doc(&ifds);
        fs::write("/tmp/tiff-ifd-codec-test.tiff", &bytes).unwrap();
        let json = project("/tmp/tiff-ifd-codec-test.tiff").expect("project the just-written file");
        assert!(json.contains("\"ifdCount\":2"));
        assert!(json.contains("little-endian"));
        assert!(json.contains("\"kind\":\"ascii\",\"value\":\"hi\""));
    }
}
