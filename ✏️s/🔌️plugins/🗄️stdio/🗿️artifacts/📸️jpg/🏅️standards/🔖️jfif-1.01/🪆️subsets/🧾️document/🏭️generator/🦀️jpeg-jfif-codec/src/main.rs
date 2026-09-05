//! jpeg-jfif-codec — standalone JFIF 1.01 fixture codec on top of the `image` 0.25 crate's own
//! public JPEG encoder/decoder. Zero dependencies beyond `image` itself (see this crate's own
//! Cargo.toml — its own `[workspace]`, isolated from the repository's root workspace and Cargo.lock).
//!
//! This binary is the code `../../🔣️oracle.json`'s reader oracle registration
//! (`image-jpeg-jfif-1-01-mutate-reader`) points at. It never links against, calls, or copies
//! logic from this subset's own `🚪️io/🦀️.rs` (the production JFIF codec) or its
//! `🦀️oracle.rs` (the reclassified `cross-semio-implementation` oracle, which COMPUTES
//! mutation results and therefore shares a spec reading with production) — every recipe below is a
//! literal, hand-picked byte value written directly by `image`'s own encoder, never a mutation
//! applied to a document.
//!
//! Two subcommands:
//!   build   <recipe-id> <out-dir> <directory-name> — writes the handpicked fixture directory
//!   project <path-to-jpg>           — decodes a real JPEG file and prints a typed JSON projection
//!                                     on stdout, using ONLY `image`'s public `ImageDecoder` surface.
//!
//! # Investigated: what `image` 0.25.10 (zune-jpeg 0.5.15) can actually witness
//!
//! `image::codecs::jpeg::JpegDecoder`'s `ImageDecoder` impl exposes `dimensions`, `color_type`,
//! `icc_profile`, `exif_metadata`, `xmp_metadata`, `iptc_metadata`, `orientation` and the decoded
//! raster (`read_image`) — nothing else. Confirmed by reading the vendored crate source directly
//! (`~/.cargo/registry/src/…/image-0.25.10/src/codecs/jpeg/decoder.rs` and
//! `zune-jpeg-0.5.15/src/decoder.rs`/`headers.rs`), not assumed:
//!
//!   - No DQT/DHT/DRI accessor exists anywhere in either crate's public API — `zune_jpeg::decoder`
//!     has no `pub fn` returning quantization tables, Huffman tables or the restart interval at all.
//!   - `zune_jpeg::decoder::ImageInfo` DECLARES `pub x_density: u16` / `pub y_density: u16` fields
//!     documented "Found in the APP(0) marker" — but the setters `set_x`/`set_y` that would
//!     populate them from a real APP0 segment are never called anywhere in the crate (`grep -rn
//!     "set_x(\|set_y("` finds only the dead `pub(crate) fn` definitions, zero call sites), so
//!     these fields always read back their `#[derive(Default)]` zero regardless of the file's real
//!     density. `ImageInfo.pixel_density` is a different, misleadingly-named field entirely — it is
//!     set from the SOF sample precision byte (`headers.rs::parse_start_of_frame`,
//!     `img.info.set_density(dt_precision)`), not from the APP0 density UNIT byte. There is no
//!     working JFIF-header read path in this crate version.
//!   - `exif`/`xmp`/`iptc` ARE real, populated reads: `zune-jpeg-0.5.15/src/headers.rs::parse_app1`
//!     recognizes the literal APP1 payload prefixes `b"Exif\x00\x00"` and
//!     `b"http://ns.adobe.com/xap/1.0/\0"`, and `headers.rs::parse_app13` (Photoshop 3.0 IPTC)
//!     likewise — these are genuinely third-party, spec-shaped reads, not something this binary
//!     invents.
//!
//! This governs which of the 10 declared mutation kinds this reader can honestly be registered
//! against: `insert-other-segment`/`remove-other-segment` are built here using an XMP APP1 payload
//! specifically (the one generic-segment shape `image` can actually see — a COM or an
//! unrecognized APPn would not be); `change-jfif-header` gets a real, `image`-encoder-written
//! density difference between before/after (via the encoder's own `set_pixel_density`, a real
//! public API) that this reader still cannot read back, so it is registered `-uncarried` rather
//! than falsely claimed; `replace-pixels` and `change-re-encode-quality` are witnessed through the
//! decoded raster digest.
//!
//! # Why the other five kinds are `-uncarried` for a SECOND, independent reason
//!
//! `replace-quant-table`, `remove-quant-table`, `replace-huffman-table`, `remove-huffman-table` and
//! `change-restart-interval` are not just unreadable through `image` — this subset's OWN production
//! encoder (`../../🚪️io/🦀️.rs::encode_jpg`) provably never carries them into the bytes at
//! all: it regenerates fresh Annex K DQT/DHT tables scaled by `re_encode_quality` on every encode
//! and never emits a DRI/restart marker (confirmed directly in that file, and independently
//! documented in this subset's own `🦀️oracle.rs` module docstring). A perfect reader
//! would still have nothing to witness, so these five recipes below hand-author `before == after`
//! byte-identically — the literal truth of what production does with them — rather than fabricate a
//! difference no encoder in this repository can produce.

use image::codecs::jpeg::{JpegEncoder, PixelDensity, PixelDensityUnit};
use image::{ExtendedColorType, ImageDecoder, ImageEncoder, RgbImage};
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

//#region 🔖️BaseRaster
/// 🌈 32x24 deterministic gradient-plus-checkerboard raster — real DCT-relevant texture (never a
/// flat fill) so a re-encode at a different quality genuinely moves decoded sample bytes, and small
/// enough that ten fixture pairs stay cheap to commit and reproduce.
fn base_image() -> RgbImage {
    let (width, height) = (32u32, 24u32);
    RgbImage::from_fn(width, height, |x, y| {
        let r = ((x * 255) / (width - 1)) as u8;
        let g = ((y * 255) / (height - 1)) as u8;
        let b = if (x / 4 + y / 4) % 2 == 0 { 40 } else { 220 };
        image::Rgb([r, g, b])
    })
}

/// 🎨 A uniform fill — mirrors `replace-pixels`' own default mid-grey opaque fill in this subset's
/// reclassified `cross-semio-implementation` oracle, reimplemented independently here.
fn solid_image(width: u32, height: u32, rgb: [u8; 3]) -> RgbImage {
    RgbImage::from_pixel(width, height, image::Rgb(rgb))
}
//#endregion 🔖️BaseRaster

//#region 🔖️Encode
/// ✍️ Encodes through `image`'s own `JpegEncoder` — the ONLY place bytes are produced. `density`
/// is a real public encoder option (`set_pixel_density`); passing `None` leaves the encoder's own
/// default JFIF APP0 density untouched.
fn encode(img: &RgbImage, quality: u8, density: Option<(u16, u16, PixelDensityUnit)>) -> Vec<u8> {
    let mut out = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut out, quality);
    if let Some((x, y, unit)) = density {
        encoder.set_pixel_density(PixelDensity { density: (x, y), unit });
    }
    encoder.write_image(img.as_raw(), img.width(), img.height(), ExtendedColorType::Rgb8).expect("image::JpegEncoder::write_image to an in-memory Vec cannot fail");
    out
}

const XMP_NAMESPACE_PREFIX: &[u8] = b"http://ns.adobe.com/xap/1.0/\0";

/// 🏷️ A minimal, real XMP packet — the literal bytes `zune-jpeg-0.5.15/src/headers.rs::parse_app1`
/// recognizes by its `XMP_NAMESPACE_PREFIX`, so this is the one `insert-other-segment` payload
/// shape this reader can actually witness (an unrecognized APPn or a bare COM would not surface
/// through any public accessor — see the module docstring).
fn xmp_packet() -> Vec<u8> {
    b"<?xpacket begin=\"\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?><x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"><rdf:Description rdf:about=\"\"/></rdf:RDF></x:xmpmeta><?xpacket end=\"w\"?>".to_vec()
}

/// 🧷 Splices one APP1 XMP segment immediately after the APP0 `image`'s own encoder always writes
/// first (SOI 0..2, APP0 marker 2..4, its 2-byte length 4..6 — T.871 §B.2.4.6.2, the same public,
/// fixed offsets this subset's `cross-semio-implementation` oracle documents and this binary
/// independently re-derives; no shared code). Splicing is pure concatenation of bytes this binary
/// already chose — it predicts nothing about what a mutation should produce.
fn splice_xmp(jpg: &[u8], xmp: &[u8]) -> Vec<u8> {
    assert_eq!(&jpg[2..4], &[0xFF, 0xE0], "image's JpegEncoder must write APP0 immediately after SOI");
    let app0_length = ((jpg[4] as usize) << 8) | jpg[5] as usize;
    let app0_end = 4 + app0_length;
    let mut segment = vec![0xFFu8, 0xE1];
    let payload_len = xmp.len() + XMP_NAMESPACE_PREFIX.len() + 2;
    assert!(payload_len <= 0xFFFF, "xmp packet too large for one JPEG marker segment");
    segment.push((payload_len >> 8) as u8);
    segment.push((payload_len & 0xFF) as u8);
    segment.extend_from_slice(XMP_NAMESPACE_PREFIX);
    segment.extend_from_slice(xmp);

    let mut out = Vec::with_capacity(jpg.len() + segment.len());
    out.extend_from_slice(&jpg[..app0_end]);
    out.extend_from_slice(&segment);
    out.extend_from_slice(&jpg[app0_end..]);
    out
}
//#endregion 🔖️Encode

//#region 🔖️Recipes
const DEFAULT_QUALITY: u8 = 90;
const LOW_QUALITY: u8 = 20;

/// 🧪 One recipe: `before` always, `after` for this catalog's own `applied`-only vocabulary (this
/// subset registers no `rejected` outcomes — see `../../🔣️oracle.json`). See the module
/// docstring for exactly why each of the ten kinds is shaped the way it is below.
fn recipe(id: &str) -> Option<(Vec<u8>, Vec<u8>)> {
    let base = base_image();
    match id {
        // 🫥️ Reader-uncarried: `image` writes a real density difference but has no decode-side
        // getter for it in this crate version (see module docstring). The bytes genuinely differ.
        "change-jfif-header-applied" => {
            let before = encode(&base, DEFAULT_QUALITY, None);
            let after = encode(&base, DEFAULT_QUALITY, Some((300, 300, PixelDensityUnit::Inches)));
            Some((before, after))
        }

        // 🫥️ Carrier-uncarried: this subset's own production encoder regenerates DQT/DHT fresh
        // from `re_encode_quality` and never emits DRI — none of these five survive ANY encoder,
        // so `before == after` is the literal truth, not a shortcut.
        "replace-quant-table-applied" | "remove-quant-table-applied" | "replace-huffman-table-applied" | "remove-huffman-table-applied" | "change-restart-interval-applied" => {
            let bytes = encode(&base, DEFAULT_QUALITY, None);
            Some((bytes.clone(), bytes))
        }

        // 👁️ Witnessable: `xmp_metadata()` reads exactly this APP1 shape.
        "insert-other-segment-applied" => {
            let before = encode(&base, DEFAULT_QUALITY, None);
            let after = splice_xmp(&before, &xmp_packet());
            Some((before, after))
        }
        "remove-other-segment-applied" => {
            let plain = encode(&base, DEFAULT_QUALITY, None);
            let before = splice_xmp(&plain, &xmp_packet());
            Some((before, plain))
        }

        // 👁️ Witnessable: the decoded raster digest changes from a real gradient to a flat fill.
        "replace-pixels-applied" => {
            let before = encode(&base, DEFAULT_QUALITY, None);
            let after = encode(&solid_image(base.width(), base.height(), [128, 128, 128]), DEFAULT_QUALITY, None);
            Some((before, after))
        }

        // 👁️ Witnessable: same source raster, two quality settings — quantization noise on this
        // textured base moves decoded sample bytes measurably (unlike a flat fill, see
        // `replace_pixels_and_re_encode_quality_recipes_change_the_decoded_raster` below).
        "change-re-encode-quality-applied" => {
            let before = encode(&base, DEFAULT_QUALITY, None);
            let after = encode(&base, LOW_QUALITY, None);
            Some((before, after))
        }

        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &[
    "change-jfif-header-applied",
    "replace-quant-table-applied",
    "remove-quant-table-applied",
    "replace-huffman-table-applied",
    "remove-huffman-table-applied",
    "change-restart-interval-applied",
    "insert-other-segment-applied",
    "remove-other-segment-applied",
    "replace-pixels-applied",
    "change-re-encode-quality-applied",
];
//#endregion 🔖️Recipes

//#region 🔖️Project
fn digest(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn opaque_json(name: &str, bytes: Option<&[u8]>) -> String {
    match bytes {
        Some(data) => format!("\"{name}\":{{\"present\":true,\"size\":{},\"digest\":{}}}", data.len(), json_str(&digest(data))),
        None => format!("\"{name}\":{{\"present\":false}}"),
    }
}

/// 👁️ Decodes with `image`'s own `JpegDecoder`, reading every metadata accessor its `ImageDecoder`
/// impl exposes, then a second time through `image::load_from_memory` for the decoded raster
/// (`read_image` consumes the decoder, so metadata is read first through a fresh instance).
/// Everything here reads; nothing here computes what a mutation should have produced.
fn project(path: &str) -> String {
    let file = File::open(path).unwrap_or_else(|error| panic!("cannot open {path}: {error}"));
    let mut decoder = image::codecs::jpeg::JpegDecoder::new(BufReader::new(file)).expect("image::codecs::jpeg::JpegDecoder::new on a real JPEG file");
    let (width, height) = decoder.dimensions();
    let color_type = format!("{:?}", decoder.color_type());
    let icc = decoder.icc_profile().expect("icc_profile() read");
    let exif = decoder.exif_metadata().expect("exif_metadata() read");
    let xmp = decoder.xmp_metadata().expect("xmp_metadata() read");
    let iptc = decoder.iptc_metadata().expect("iptc_metadata() read");

    let decoded = image::open(path).unwrap_or_else(|error| panic!("image::open {path}: {error}"));
    let rgb = decoded.to_rgb8();

    format!(
        "{{\"dimensions\":{},\"colorType\":{},\"raster\":{{\"size\":{},\"digest\":{}}},{},{},{},{}}}",
        json_str(&format!("{width}x{height}")),
        json_str(&color_type),
        rgb.as_raw().len(),
        json_str(&digest(rgb.as_raw())),
        opaque_json("xmp", xmp.as_deref()),
        opaque_json("exif", exif.as_deref()),
        opaque_json("iptc", iptc.as_deref()),
        opaque_json("iccProfile", icc.as_deref()),
    )
}
//#endregion 🔖️Project

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str, directory_name: &str) -> i32 {
    let Some((before, after)) = recipe(id) else {
        eprintln!("[jpeg-jfif-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(directory_name);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("⬅️before.jpg"), &before).expect("write ⬅️before.jpg");
    fs::write(dir.join("➡️after.jpg"), &after).expect("write ➡️after.jpg");
    eprintln!("[jpeg-jfif-codec] {id}: ⬅️before.jpg ({} bytes) + ➡️after.jpg ({} bytes) -> {}", before.len(), after.len(), dir.display());
    0
}

fn cmd_project(path: &str) -> i32 {
    println!("{}", project(path));
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir), Some(directory_name)) = (args.get(2), args.get(3), args.get(4)) else {
                eprintln!("usage: jpeg-jfif-codec build <recipe-id> <out-dir> <directory-name>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir, directory_name)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: jpeg-jfif-codec project <path-to-jpg>");
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
            eprintln!("usage: jpeg-jfif-codec build <recipe-id> <out-dir> | project <path-to-jpg> | list-recipes");
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
    fn uncarried_table_and_restart_recipes_are_byte_identical() {
        for id in ["replace-quant-table-applied", "remove-quant-table-applied", "replace-huffman-table-applied", "remove-huffman-table-applied", "change-restart-interval-applied"] {
            let (before, after) = recipe(id).unwrap();
            assert_eq!(before, after, "recipe {id} must be byte-identical — production discards this field");
        }
    }

    #[test]
    fn change_jfif_header_recipe_differs_in_bytes_despite_being_reader_uncarried() {
        let (before, after) = recipe("change-jfif-header-applied").unwrap();
        assert_ne!(before, after);
    }

    #[test]
    fn xmp_round_trips_through_the_splice() {
        let (before, after) = recipe("insert-other-segment-applied").unwrap();
        assert_ne!(before, after);
        let (before2, after2) = recipe("remove-other-segment-applied").unwrap();
        assert_ne!(before2, after2);
    }

    #[test]
    fn replace_pixels_and_re_encode_quality_recipes_change_the_decoded_raster() {
        for id in ["replace-pixels-applied", "change-re-encode-quality-applied"] {
            let (before, after) = recipe(id).unwrap();
            let before_rgb = image::load_from_memory(&before).unwrap().to_rgb8();
            let after_rgb = image::load_from_memory(&after).unwrap().to_rgb8();
            assert_ne!(before_rgb.into_raw(), after_rgb.into_raw(), "recipe {id} must change the decoded raster");
        }
    }
}
//#endregion 🔖️Tests
