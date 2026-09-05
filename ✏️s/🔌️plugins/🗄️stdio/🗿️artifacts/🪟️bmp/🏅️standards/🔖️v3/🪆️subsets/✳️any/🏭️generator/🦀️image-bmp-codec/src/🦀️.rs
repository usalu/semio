//! image-bmp-codec — standalone BMP v3 fixture builder + reader projection on top of `image`
//! 0.25's own `bmp` feature (`image::codecs::bmp::{BmpEncoder, BmpDecoder}`). Zero dependencies
//! beyond `image` itself (see this crate's own Cargo.toml — its own `[workspace]`, isolated from
//! the repository's root workspace and Cargo.lock).
//!
//! This binary is the READER the repository's `image-bmp-3-mutate-reader` oracle registration
//! (`../../🔣️oracle.json`) points at. It never applies a mutation and never predicts what one
//! should produce: `build` writes each recipe's before/after bytes as literal, hand-picked typed
//! values (never by executing this repository's own `BmpMutation` dispatch), and `project` only
//! decodes an already-real file through `image`'s own public decoder API and reports what it
//! found. Nothing here re-implements BITMAPINFOHEADER byte-offset parsing the way the sibling
//! `../../../🔮️oracle/🦀️.rs` (a `cross-semio-implementation`, read only as a spec reference, never
//! copied from) does — every field this binary reports comes from a method `image` itself
//! exposes: `BmpDecoder::dimensions`/`get_palette`/`set_indexed_color`/`read_image`, or
//! `image::load_from_memory(..).to_rgb8()` for a direct-colour file. Fields `image`'s public API
//! does not expose at all (row storage order, x/y pixels-per-metre, compression, colorsUsed as a
//! value distinct from palette length, colorsImportant, an exact bitsPerPixel/headerSize/
//! imageSize) are never read here by any other means — see this subset's own
//! `📓️bmp-v3-any-reader-oracle-retrofit.md` for the source-level evidence (`image-0.25.10`'s
//! vendored `src/codecs/bmp/{decoder,encoder}.rs`) that those fields are architecturally
//! unreachable through this crate's public surface, which is exactly why
//! `change-header-fields` is registered `bmp-3-mutate-uncarried` rather than against this oracle.
//!
//! Two subcommands:
//!   build   <recipe-id> <out-dir>   — writes the recipe's handpicked directory and arrow-named BMP pair
//!   project <path-to-bmp>           — decodes a real BMP file and prints a typed JSON projection
//!                                     on stdout (width/height/storage plus, for an indexed file,
//!                                     `image`'s own 256-entry zero-padded palette table and the
//!                                     raw index buffer as hex; for a direct-colour file, the
//!                                     resolved RGB buffer as hex). The caller hashes the hex
//!                                     payloads into size+digest pairs and drops the raw bytes,
//!                                     per this artifact's comparisonProfile's own opaque-payload
//!                                     treatment of large raster data.
//!
//! @see ../../../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🦀️riff-avi-codec/src/main.rs
//!      — the sibling this file's CLI/recipe/JSON-emission shape is mirrored from.
//! @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

use image::codecs::bmp::{BmpDecoder, BmpEncoder};
use image::{ExtendedColorType, ImageDecoder};
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;

//#region 🔖️Types
#[derive(Clone)]
enum Content {
    /// 🎨️ Palette indices (natural row order as `image` delivers them) plus the REAL colour table
    /// this recipe was built with — not the 256-entry zero-padded shape `get_palette` returns on
    /// read, which is a decode-time artefact this binary's own `project` command reproduces
    /// faithfully rather than trimming back down.
    Indexed { indices: Vec<u8>, palette: Vec<[u8; 3]> },
    /// 🖼️ Direct-colour 24-bit RGB, row 0 = image top (pre-flip; `image`'s encoder stores bottom-up
    /// internally and this binary never reasons about that storage order itself).
    Direct { rgb: Vec<u8> },
}

#[derive(Clone)]
struct Doc {
    width: u32,
    height: u32,
    content: Content,
}
//#endregion 🔖️Types

//#region 🔖️ByteHelpers
fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}
//#endregion 🔖️ByteHelpers

//#region 🔖️Encode
/// 🏭️ Writes `doc` with `image`'s own `BmpEncoder` — the ONLY place this file produces bytes.
fn encode(doc: &Doc) -> Vec<u8> {
    let mut out = Vec::new();
    match &doc.content {
        Content::Indexed { indices, palette } => {
            BmpEncoder::new(&mut out).encode_with_palette(indices, doc.width, doc.height, ExtendedColorType::L8, Some(palette)).expect("image crate: encode indexed BMP");
        }
        Content::Direct { rgb } => {
            BmpEncoder::new(&mut out).encode(rgb, doc.width, doc.height, ExtendedColorType::Rgb8).expect("image crate: encode direct-colour BMP");
        }
    }
    out
}
//#endregion 🔖️Encode

//#region 🔖️Project
struct Projected {
    width: u32,
    height: u32,
    storage: &'static str,
    palette: Option<Vec<[u8; 3]>>,
    indices_hex: Option<String>,
    pixels_hex: Option<String>,
}

/// 👁️ Reads `bytes` with `image`'s own `BmpDecoder`. Indexed-ness is discovered the same way
/// `image` itself discovers it — `get_palette()` returns `Some` precisely when the file's own
/// `image_type` is `Palette | RLE4 | RLE8`, decided during `image`'s own metadata read, never by
/// this binary inspecting a bit-count byte itself.
fn project(bytes: &[u8]) -> Result<Projected, String> {
    let mut decoder = BmpDecoder::new(Cursor::new(bytes)).map_err(|error| format!("image crate: could not parse BMP: {error}"))?;
    let (width, height) = decoder.dimensions();
    if let Some(palette) = decoder.get_palette().map(<[[u8; 3]]>::to_vec) {
        decoder.set_indexed_color(true);
        let mut indices = vec![0u8; width as usize * height as usize];
        decoder.read_image(&mut indices).map_err(|error| format!("image crate: could not decode BMP index buffer: {error}"))?;
        Ok(Projected { width, height, storage: "indexed", palette: Some(palette), indices_hex: Some(to_hex(&indices)), pixels_hex: None })
    } else {
        let decoded = image::load_from_memory(bytes).map_err(|error| format!("image crate: could not parse BMP: {error}"))?;
        let rgb = decoded.to_rgb8().into_raw();
        Ok(Projected { width, height, storage: "direct", palette: None, indices_hex: None, pixels_hex: Some(to_hex(&rgb)) })
    }
}
//#endregion 🔖️Project

//#region 🔖️Json
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    out.push_str(s);
    out.push('"');
    out
}

fn json_opt_str(s: &Option<String>) -> String {
    match s {
        Some(value) => json_str(value),
        None => "null".to_string(),
    }
}

fn palette_json(palette: &[[u8; 3]]) -> String {
    let entries: Vec<String> = palette.iter().map(|[r, g, b]| format!("{{\"r\":{r},\"g\":{g},\"b\":{b}}}")).collect();
    format!("[{}]", entries.join(","))
}

fn projected_json(p: &Projected) -> String {
    let palette = match &p.palette {
        Some(table) => palette_json(table),
        None => "null".to_string(),
    };
    format!(
        "{{\"format\":\"bmp\",\"width\":{},\"height\":{},\"storage\":{},\"palette\":{},\"indicesHex\":{},\"pixelsHex\":{}}}",
        p.width,
        p.height,
        json_str(p.storage),
        palette,
        json_opt_str(&p.indices_hex),
        json_opt_str(&p.pixels_hex),
    )
}
//#endregion 🔖️Json

//#region 🔖️Recipes
/// 🍳️ One entry per witnessable `bmp-3-mutate` kind this reader's oracle covers — see this
/// subset's own report for why `change-header-fields` has no recipe here at all: `image`'s public
/// BMP API exposes neither the row-order flag, x/y pixels-per-metre, compression, colorsUsed as a
/// value distinct from palette length, nor colorsImportant, so nothing that mutation touches
/// round-trips through this reader. All four base every recipe on the SAME 4x4 indices/geometry so
/// the only difference between `before` and `after` is exactly the field the recipe names.
const WIDTH: u32 = 4;
const HEIGHT: u32 = 4;

/// 🎨️ Five real, referenced colours plus TWO spares (index 5, 6) no pixel below references, so
/// removing or replacing index 5 never orphans a colour a pixel still needs — the same
/// representability constraint this subset's own diff component documents for a real
/// `encode_bmp`. The second spare exists so `remove-palette-entry` is actually witnessable:
/// `image`'s own `get_palette()` always zero-pads its return to 256 entries (confirmed by reading
/// `image-0.25.10`'s vendored decoder source — see this file's own header comment), so removing
/// the LAST real entry would shift only zero-padding into its place, an invisible no-op diff. A
/// non-zero colour at index 6 means the removal is provably witnessed: position 5 gains index 6's
/// colour and position 6 reverts to padding.
fn base_palette() -> Vec<[u8; 3]> {
    vec![[255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 255, 0], [255, 0, 255], [0, 0, 0], [10, 20, 30]]
}

/// 🔢️ 16 indices over 0..=4 only — index 5 (the spare) is never referenced.
fn base_indices() -> Vec<u8> {
    (0..WIDTH * HEIGHT).map(|i| (i % 5) as u8).collect()
}

fn base_indexed_doc() -> Doc {
    Doc { width: WIDTH, height: HEIGHT, content: Content::Indexed { indices: base_indices(), palette: base_palette() } }
}

fn solid_direct_doc(rgb: [u8; 3]) -> Doc {
    Doc { width: WIDTH, height: HEIGHT, content: Content::Direct { rgb: rgb.iter().copied().cycle().take(WIDTH as usize * HEIGHT as usize * 3).collect() } }
}

/// 🍳️ Returns `(before, after)` for a known recipe id, or `None` for an unknown one.
fn recipe(id: &str) -> Option<(Doc, Doc)> {
    match id {
        "insert-palette-entry-applied" => {
            let before = base_indexed_doc();
            let mut palette = base_palette();
            palette.insert(2, [128, 64, 32]);
            let after = Doc { width: WIDTH, height: HEIGHT, content: Content::Indexed { indices: base_indices(), palette } };
            Some((before, after))
        }
        "remove-palette-entry-applied" => {
            let before = base_indexed_doc();
            let mut palette = base_palette();
            palette.remove(5);
            let after = Doc { width: WIDTH, height: HEIGHT, content: Content::Indexed { indices: base_indices(), palette } };
            Some((before, after))
        }
        "replace-palette-entry-applied" => {
            let before = base_indexed_doc();
            let mut palette = base_palette();
            palette[5] = [200, 150, 100];
            let after = Doc { width: WIDTH, height: HEIGHT, content: Content::Indexed { indices: base_indices(), palette } };
            Some((before, after))
        }
        "replace-pixel-data-applied" => {
            let before = solid_direct_doc([10, 20, 30]);
            let after = solid_direct_doc([200, 100, 50]);
            Some((before, after))
        }
        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &["insert-palette-entry-applied", "remove-palette-entry-applied", "replace-palette-entry-applied", "replace-pixel-data-applied"];

fn fixture_directory_name(id: &str) -> Option<&'static str> {
    match id {
        "insert-palette-entry-applied" => Some("📥️insert-palette-entry-applied"),
        "remove-palette-entry-applied" => Some("📤️remove-palette-entry-applied"),
        "replace-palette-entry-applied" => Some("🎨️replace-palette-entry-applied"),
        "replace-pixel-data-applied" => Some("🧮️replace-pixel-data-applied"),
        _ => None,
    }
}
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str) -> i32 {
    let Some((before, after)) = recipe(id) else {
        eprintln!("[image-bmp-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(fixture_directory_name(id).expect("known recipe has a fixture directory"));
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("⬅️before.bmp"), encode(&before)).expect("write ⬅️before.bmp");
    fs::write(dir.join("➡️after.bmp"), encode(&after)).expect("write ➡️after.bmp");
    eprintln!("[image-bmp-codec] {id}: ⬅️before.bmp + ➡️after.bmp -> {}", dir.display());
    0
}

fn cmd_project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[image-bmp-codec] cannot read {path}: {e}");
            return 1;
        }
    };
    match project(&bytes) {
        Ok(doc) => {
            println!("{}", projected_json(&doc));
            0
        }
        Err(error) => {
            eprintln!("[image-bmp-codec] project {path} failed: {error}");
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: image-bmp-codec build <recipe-id> <out-dir>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: image-bmp-codec project <path-to-bmp>");
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
            eprintln!("usage: image-bmp-codec build <recipe-id> <out-dir> | project <path-to-bmp> | list-recipes");
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
    fn indexed_round_trip_preserves_indices_and_palette() {
        let doc = base_indexed_doc();
        let bytes = encode(&doc);
        let projected = project(&bytes).expect("project the encoded indexed BMP");
        assert_eq!(projected.storage, "indexed");
        assert_eq!(projected.width, WIDTH);
        assert_eq!(projected.height, HEIGHT);
        let palette = projected.palette.expect("indexed file reports a palette");
        assert_eq!(&palette[..7], &base_palette()[..]);
        assert_eq!(projected.indices_hex.expect("indices"), to_hex(&base_indices()));
    }

    #[test]
    fn direct_round_trip_preserves_solid_fill() {
        let doc = solid_direct_doc([10, 20, 30]);
        let bytes = encode(&doc);
        let projected = project(&bytes).expect("project the encoded direct-colour BMP");
        assert_eq!(projected.storage, "direct");
        assert!(projected.palette.is_none());
        let expected: Vec<u8> = [10u8, 20, 30].iter().copied().cycle().take(WIDTH as usize * HEIGHT as usize * 3).collect();
        assert_eq!(projected.pixels_hex.expect("pixels"), to_hex(&expected));
    }

    #[test]
    fn insert_palette_entry_recipe_leaves_indices_untouched() {
        let (before, after) = recipe("insert-palette-entry-applied").unwrap();
        let (Content::Indexed { indices: bi, .. }, Content::Indexed { indices: ai, palette: ap, .. }) = (&before.content, &after.content) else {
            panic!("expected indexed content");
        };
        assert_eq!(bi, ai, "a palette-only mutation must leave the index buffer untouched");
        assert_eq!(ap.len(), 8, "one entry was inserted into the 7-entry base palette");
    }
}
//#endregion 🔖️Tests
