//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `raster` module rather than by copying it.
//!
//! # The model, and why it has to be indexed
//!
//! An 8-bit BMP v3 is a colour TABLE plus per-pixel INDICES into it. `image::load_from_memory`
//! resolves those to RGBA and throws both away, which is why the earlier revision of this module
//! could not perform `insert-palette-entry`, `remove-palette-entry` or `set-palette-entry` at all
//! and returned the document unchanged for each. `image` does expose the indexed layer:
//! `BmpDecoder::set_indexed_color(true)` hands back the raw index buffer instead of resolved
//! pixels, `BmpDecoder::get_palette` hands back the table, and `BmpEncoder::encode_with_palette`
//! writes both back as a real indexed BITMAPINFOHEADER. [`OracleDoc`] therefore carries indices and
//! a palette for a palettized file and RGBA for a direct-colour one, and all three palette kinds
//! are performed for real.
//!
//! `get_palette` always returns 256 entries — `read_palette` zero-pads deliberately, "to prevent
//! corrupt files from causing an out-of-bounds array access" — and the crate exposes `biClrUsed`
//! nowhere, so the table's real length comes from the BITMAPINFOHEADER directly. The same header
//! walk supplies `row_order` (the sign of `biHeight`) and the two pixels-per-metre fields, which
//! `image` neither reads nor writes: its encoder hard-codes both to `0` and always stores rows
//! bottom-up. Those are patched back onto the encoder's own output, in the fixed BMP v3 layout,
//! the same way the GIF subsets patch their Logical Screen Descriptor scalars.
//!
//! # What a palette mutation means here, and what the fixture must therefore be
//!
//! `BmpSnapshot::pixels` is the DECODED, palette-resolved RGBA buffer, and `palette` is an
//! independent field: this subset's own semantics for a palette edit are "change the colour table,
//! leave the picture alone". `encode_bmp` re-indexes on the way out and reports an `Err` — never a
//! narrowing, never a silent fall back to 24-bit — when a pixel's colour no longer has an entry.
//!
//! That makes a targeted palette edit representable only when the entry it addresses is referenced
//! by no pixel, and an insertion representable only while the table stays inside the 256-entry
//! capacity of 8 bits. The committed fixture is derived to satisfy both (see
//! [`fixture_derivation`]); the previous fixture satisfied neither, and its feature file claimed
//! index 0 was "a palette entry no pixel actually resolves to" when index 0 is in fact the most
//! referenced entry in the image, covering 5 659 668 of its 5 975 040 pixels.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use image::ImageDecoder;
    use semio_repo_test_host::{digest, Json};

    const FORMAT: &str = "bmp";

    //#region 🔖️Json
    /// 🔎️ Numeric member, or `None` for anything else — params are authored directly in the
    /// feature file, so a missing/mistyped field is a legitimate default, never a panic.
    fn num(params: &Json, key: &str) -> Option<f64> {
        match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    fn as_arr(value: &Json) -> &[Json] {
        match value {
            Json::Array(items) => items,
            _ => &[],
        }
    }
    fn num_at(items: &[Json], index: usize) -> Option<f64> {
        match items.get(index) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    fn empty_params() -> Json {
        Json::Object(Vec::new())
    }
    fn index_of(params: &Json) -> usize {
        num(params, "index").unwrap_or(0.0).max(0.0) as usize
    }

    /// 🎨️ One `{"b":…,"g":…,"r":…,"reserved":…}` colour-table entry, in the same member spelling
    /// `BmpPaletteEntry` serializes to, so a feature row reads the same on both sides.
    fn entry_of(params: &Json) -> [u8; 3] {
        let entry = params.get("entry").cloned().unwrap_or_else(empty_params);
        [num(&entry, "r").unwrap_or(0.0) as u8, num(&entry, "g").unwrap_or(0.0) as u8, num(&entry, "b").unwrap_or(0.0) as u8]
    }
    //#endregion 🔖️Json

    //#region 🔖️Doc
    /// 🧾️ This oracle's own, independent BMP v3 document model. `content` distinguishes the two
    /// storage forms BMP v3 actually has, rather than flattening both to RGBA and losing the one
    /// three of the seven declared kinds operate on.
    pub struct OracleDoc {
        pub width: u32,
        pub height: u32,
        pub top_down: bool,
        pub x_pixels_per_meter: i32,
        pub y_pixels_per_meter: i32,
        pub content: Content,
    }

    pub enum Content {
        /// 🎨️ A palettized file: per-pixel indices in natural top-to-bottom order, plus the real
        /// colour table, trimmed to the length `biClrUsed` declares.
        Indexed { indices: Vec<u8>, palette: Vec<[u8; 3]> },
        /// 🖼️ A direct-colour file: canonical 8-bit RGBA, row 0 = image top.
        Direct { rgba: Vec<u8> },
    }
    //#endregion 🔖️Doc

    //#region 🔖️Header
    /// 🧾️ The BITMAPINFOHEADER fields the reference decoder does not expose. Fixed offsets, all of
    /// them defined by BMP v3: `bfOffBits` at 10, then the 40-byte DIB header from 14 — `biWidth`
    /// 18, `biHeight` 22 (signed; negative means top-down rows), `biBitCount` 28, `biXPelsPerMeter`
    /// 38, `biYPelsPerMeter` 42, `biClrUsed` 46.
    struct Header {
        data_offset: usize,
        height_field: i32,
        bit_count: u16,
        x_pixels_per_meter: i32,
        y_pixels_per_meter: i32,
        colors_used: u32,
    }

    fn read_u32(bytes: &[u8], at: usize) -> Result<u32, String> {
        bytes.get(at..at + 4).map(|slice| u32::from_le_bytes(slice.try_into().expect("4-byte slice"))).ok_or_else(|| format!("truncated BMP header at byte {at}"))
    }

    fn read_header(bytes: &[u8]) -> Result<Header, String> {
        if bytes.len() < 54 || &bytes[0..2] != b"BM" {
            return Err("not a BMP byte stream".to_string());
        }
        Ok(Header {
            data_offset: read_u32(bytes, 10)? as usize,
            height_field: read_u32(bytes, 22)? as i32,
            bit_count: u16::from_le_bytes(bytes[28..30].try_into().expect("2-byte slice")),
            x_pixels_per_meter: read_u32(bytes, 38)? as i32,
            y_pixels_per_meter: read_u32(bytes, 42)? as i32,
            colors_used: read_u32(bytes, 46)?,
        })
    }
    //#endregion 🔖️Header

    //#region 🔖️Decode
    /// 👁️ Decodes with the INDEPENDENT `image` reader, keeping the indexed layer intact when the
    /// file has one. In indexed mode `read_image` still delivers rows top-to-bottom regardless of
    /// how they are stored, so `indices` is natural order on both sides and `top_down` stays a pure
    /// storage fact the projection reports separately.
    pub fn decode(input: &[u8]) -> Result<OracleDoc, String> {
        let header = read_header(input)?;
        let mut decoder = image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(input)).map_err(|error| format!("independent reader could not parse the BMP: {error}"))?;
        let (width, height) = decoder.dimensions();
        let content = if header.bit_count <= 8 {
            decoder.set_indexed_color(true);
            let declared = if header.colors_used == 0 { 1usize << header.bit_count } else { header.colors_used as usize };
            let palette: Vec<[u8; 3]> = decoder.get_palette().ok_or("a BMP with a bit count of 8 or less declares no colour table")?.iter().take(declared).copied().collect();
            let mut indices = vec![0u8; width as usize * height as usize];
            decoder.read_image(&mut indices).map_err(|error| format!("independent reader could not decode the BMP index buffer: {error}"))?;
            Content::Indexed { indices, palette }
        } else {
            let mut rgba = vec![0u8; width as usize * height as usize * 4];
            let decoded = image::load_from_memory(input).map_err(|error| format!("independent reader could not parse the BMP: {error}"))?;
            rgba.copy_from_slice(&decoded.to_rgba8().into_raw());
            Content::Direct { rgba }
        };
        Ok(OracleDoc { width, height, top_down: header.height_field < 0, x_pixels_per_meter: header.x_pixels_per_meter, y_pixels_per_meter: header.y_pixels_per_meter, content })
    }
    //#endregion 🔖️Decode

    //#region 🔖️Encode
    /// 🔀️ Rewrites a bottom-up BMP as a top-down one: negate `biHeight` and reverse the stored row
    /// blocks. `image`'s encoder always stores bottom-up and offers no option, so a `row_order`
    /// mutation is performed here — and performed properly, moving the rows as well as the sign,
    /// since flipping only the sign would turn the picture upside down.
    fn store_top_down(bytes: &mut [u8], width: u32, height: u32, bits_per_pixel: u16, data_offset: usize) -> Result<(), String> {
        let stride = ((width as usize * bits_per_pixel as usize).div_ceil(32)) * 4;
        let end = data_offset + stride * height as usize;
        if bytes.len() < end {
            return Err(format!("BMP pixel array is {} byte(s) short of the {stride}-byte rows its header declares", end - bytes.len()));
        }
        let negated = -(height as i32);
        bytes[22..26].copy_from_slice(&negated.to_le_bytes());
        let rows = &mut bytes[data_offset..end];
        for row in 0..(height as usize / 2) {
            let (low, high) = (row * stride, (height as usize - 1 - row) * stride);
            for byte in 0..stride {
                rows.swap(low + byte, high + byte);
            }
        }
        Ok(())
    }

    /// 🔮️ Re-serializes the whole document with the registered `image` writer, then restores the
    /// three BITMAPINFOHEADER facts that writer emits as constants: both pixels-per-metre fields
    /// (hard-coded `0`) and the row order (always bottom-up). Without those patches
    /// `set-header-fields` is accepted and silently discarded, which reports as a passing scenario.
    pub fn encode(doc: &OracleDoc) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        let bits_per_pixel = match &doc.content {
            Content::Indexed { indices, palette } => {
                if indices.len() != doc.width as usize * doc.height as usize {
                    return Err(format!("index buffer holds {} entries, expected {}", indices.len(), doc.width as usize * doc.height as usize));
                }
                if palette.len() > 256 {
                    return Err(format!("colour table holds {} entries, past the 256 an 8-bit index can address", palette.len()));
                }
                image::codecs::bmp::BmpEncoder::new(&mut out).encode_with_palette(indices, doc.width, doc.height, image::ExtendedColorType::L8, Some(palette)).map_err(|error| format!("bmp encode: {error}"))?;
                8
            }
            Content::Direct { rgba } => {
                out = crate::raster::oracle_create_image(&crate::raster::RasterSpec { width: doc.width, height: doc.height, rgba: rgba.clone() }, FORMAT)?;
                24
            }
        };
        let data_offset = read_header(&out)?.data_offset;
        out[38..42].copy_from_slice(&doc.x_pixels_per_meter.to_le_bytes());
        out[42..46].copy_from_slice(&doc.y_pixels_per_meter.to_le_bytes());
        if doc.top_down {
            store_top_down(&mut out, doc.width, doc.height, bits_per_pixel, data_offset)?;
        }
        Ok(out)
    }
    //#endregion 🔖️Encode

    //#region 🔖️Apply
    fn fill_quad(params: &Json) -> [u8; 4] {
        let fill = as_arr(params.get("fill").unwrap_or(&Json::Null));
        [num_at(fill, 0).unwrap_or(0.0) as u8, num_at(fill, 1).unwrap_or(0.0) as u8, num_at(fill, 2).unwrap_or(0.0) as u8, num_at(fill, 3).unwrap_or(0.0) as u8]
    }

    fn solid_rgba(width: u32, height: u32, quad: [u8; 4]) -> Vec<u8> {
        quad.iter().copied().cycle().take(width as usize * height as usize * 4).collect()
    }

    fn palette_mut<'a>(doc: &'a mut OracleDoc, kind: &str) -> Result<&'a mut Vec<[u8; 3]>, String> {
        match &mut doc.content {
            Content::Indexed { palette, .. } => Ok(palette),
            Content::Direct { .. } => Err(format!("{kind} addresses a colour table, and this document is direct-colour — it has none")),
        }
    }

    /// 🦠️ One `match` arm per `BmpMutation` variant, reimplemented independently against
    /// [`OracleDoc`] rather than calling into the subject's own `apply_bmp_mutation`. Out-of-range
    /// palette indices degrade to a no-op, mirroring `BmpMutation::diff`'s own documented
    /// behaviour rather than diverging from it without reason.
    fn apply_kind(doc: &mut OracleDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => {}
            "set-snapshot" => {
                let width = num(params, "width").unwrap_or(1.0).max(1.0) as u32;
                let height = num(params, "height").unwrap_or(1.0).max(1.0) as u32;
                *doc = OracleDoc { width, height, top_down: false, x_pixels_per_meter: 0, y_pixels_per_meter: 0, content: Content::Direct { rgba: solid_rgba(width, height, fill_quad(params)) } };
            }
            "set-header-fields" => {
                if let Some(order) = params.get("rowOrder") {
                    doc.top_down = matches!(order, Json::String(value) if value == "top-down");
                }
                if let Some(value) = num(params, "xPixelsPerMeter") {
                    doc.x_pixels_per_meter = value as i32;
                }
                if let Some(value) = num(params, "yPixelsPerMeter") {
                    doc.y_pixels_per_meter = value as i32;
                }
            }
            "insert-palette-entry" => {
                let entry = entry_of(params);
                let at = index_of(params);
                let palette = palette_mut(doc, kind)?;
                let at = at.min(palette.len());
                palette.insert(at, entry);
            }
            "remove-palette-entry" => {
                let at = index_of(params);
                let palette = palette_mut(doc, kind)?;
                if at < palette.len() {
                    palette.remove(at);
                }
            }
            "set-palette-entry" => {
                let entry = entry_of(params);
                let at = index_of(params);
                let palette = palette_mut(doc, kind)?;
                if at < palette.len() {
                    palette[at] = entry;
                }
            }
            "set-pixel-data" => {
                let quad = fill_quad(params);
                doc.content = Content::Direct { rgba: solid_rgba(doc.width, doc.height, quad) };
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Apply

    //#region 🔖️Dispatch
    /// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized
    /// bytes. An unrecognised kind is an error, never a silent no-op: a mutation that is quietly
    /// skipped reports as a passing test.
    pub fn apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let params = spec.get("params").cloned().unwrap_or_else(empty_params);
        let mut doc = decode(input)?;
        apply_kind(&mut doc, &kind, &params)?;
        encode(&doc)
    }

    /// ↩️ The `inverse-<kind>` scenarios' oracle: the reference's OWN inverse, computed by reading
    /// the pre-mutation state out of `original_input` and applied to the forward mutation's real
    /// output. `BmpMutation::inverse` (`../🧬️schema/🧬️mutations/🦀️component.rs`) is defined, per
    /// variant, as "restore `base`'s own value for the field this kind touches"; every arm below is
    /// that rule reimplemented here, never that function called.
    ///
    /// Routing through `mutated` is what gives the law teeth: the forward result has to survive a
    /// real independent re-parse first, so a forward mutation that emitted an undecodable BMP fails
    /// here instead of being reported as a passing `inverse-<kind>`.
    pub fn undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let params = spec.get("params").cloned().unwrap_or_else(empty_params);
        let original = decode(original_input)?;
        let mut doc = decode(mutated)?;
        match kind.as_str() {
            "no-mutation" => {}
            "set-snapshot" | "set-pixel-data" => doc = original,
            "set-header-fields" => {
                doc.top_down = original.top_down;
                doc.x_pixels_per_meter = original.x_pixels_per_meter;
                doc.y_pixels_per_meter = original.y_pixels_per_meter;
            }
            "insert-palette-entry" => {
                let at = index_of(&params);
                let palette = palette_mut(&mut doc, &kind)?;
                if at < palette.len() {
                    palette.remove(at);
                }
            }
            "remove-palette-entry" | "set-palette-entry" => {
                let restored = match original.content {
                    Content::Indexed { palette, .. } => palette,
                    Content::Direct { .. } => return Err(format!("{kind} addresses a colour table, and the original document is direct-colour")),
                };
                *palette_mut(&mut doc, &kind)? = restored;
            }
            other => return Err(format!("mutation kind {other:?} has no oracle inverse")),
        }
        encode(&doc)
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Project
    /// 👁️ The surface every scenario compares through, read back by THIS module's own independent
    /// [`decode`]. BMP is lossless, so every claim here is exact; `indicesDigest`/`pixelsDigest` are
    /// digests rather than arrays only because the real fixture is 5 975 040 pixels and the
    /// comparison engine would otherwise be diffing ~24 million JSON numbers per scenario.
    ///
    /// The palette is reported as its own length and digest, separately from the resolved samples:
    /// that is what makes the three palette kinds observable at all under this subset's semantics,
    /// where a palette edit changes the colour table and deliberately leaves the picture alone.
    pub fn project(input: &[u8]) -> Result<Json, String> {
        let doc = decode(input)?;
        let mut members = vec![
            ("format".to_string(), Json::String(FORMAT.to_string())),
            ("width".to_string(), Json::Number(doc.width as f64)),
            ("height".to_string(), Json::Number(doc.height as f64)),
            ("rowOrder".to_string(), Json::String(if doc.top_down { "top-down".to_string() } else { "bottom-up".to_string() })),
            ("xPixelsPerMeter".to_string(), Json::Number(doc.x_pixels_per_meter as f64)),
            ("yPixelsPerMeter".to_string(), Json::Number(doc.y_pixels_per_meter as f64)),
        ];
        match &doc.content {
            Content::Indexed { indices, palette } => {
                let table: Vec<u8> = palette.iter().flatten().copied().collect();
                members.push(("storage".to_string(), Json::String("indexed".to_string())));
                members.push(("paletteEntries".to_string(), Json::Number(palette.len() as f64)));
                members.push(("paletteDigest".to_string(), Json::String(digest(&table))));
                members.push(("indicesDigest".to_string(), Json::String(digest(indices))));
                members.push(("pixelsDigest".to_string(), Json::String(digest(&resolve(indices, palette)))));
            }
            Content::Direct { rgba } => {
                members.push(("storage".to_string(), Json::String("direct".to_string())));
                members.push(("paletteEntries".to_string(), Json::Number(0.0)));
                members.push(("paletteDigest".to_string(), Json::String(digest(&[]))));
                members.push(("indicesDigest".to_string(), Json::Null));
                members.push(("pixelsDigest".to_string(), Json::String(digest(rgba))));
            }
        }
        Ok(Json::Object(members))
    }

    /// 🎨️ Palette-resolved RGBA, so the projection reports what a viewer sees as well as how the
    /// file stores it — an index edit and a table edit are then distinguishable from each other.
    fn resolve(indices: &[u8], palette: &[[u8; 3]]) -> Vec<u8> {
        indices
            .iter()
            .flat_map(|index| {
                let entry = palette.get(*index as usize).copied().unwrap_or([0, 0, 0]);
                [entry[0], entry[1], entry[2], 255]
            })
            .collect()
    }
    //#endregion 🔖️Project
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::apply_mutation(input, spec)
}

#[cfg(feature = "oracles")]
pub fn oracle_undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    oracles::undo_mutation(original_input, spec, mutated)
}

#[cfg(feature = "oracles")]
pub fn project_bmp_mutation(input: &[u8]) -> Result<Json, String> {
    oracles::project(input)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_undo_mutation(_original_input: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_bmp_mutation(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️FixtureDerivation
/// 🧫️ One-off real-world fixture derivation. NOT a test step — `#[ignore]`d, run once by hand, the
/// same convention as the TIFF subset's own `derive_real_world_fixture`. Builds the committed
/// `shared://🖼️rathaus-ahlen-grundriss.bmp` out of the real
/// `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png`: the
/// independent `png` decoder recovers that file's genuine 233-entry PLTE and its real index buffer,
/// and the `image` reference encoder writes both back as an 8-bit indexed BITMAPINFOHEADER.
///
/// The colour table is padded to 240 entries. Both halves of that number are forced by what the
/// vocabulary has to be able to express:
///
/// * the 233 real colours are ALL referenced (index 0 alone covers 5 659 668 of 5 975 040 pixels),
///   and this subset's semantics for a palette edit are "change the table, leave the picture alone"
///   — so an edit to a referenced entry orphans a colour and `encode_bmp` reports it rather than
///   narrowing. Seven spare entries no pixel resolves to are what make `set-palette-entry` and
///   `remove-palette-entry` representable at all.
/// * a full 256-entry table — what most real 8-bpp BMP writers emit — would give that slack and
///   then make `insert-palette-entry` unrepresentable, because 257 entries exceed what an 8-bit
///   index can address. 240 leaves room for both.
///
/// The spare colours are a deterministic ramp chosen at derivation time from values the real
/// palette does not already contain, and the derivation asserts that no pixel references any of
/// them — a padded table whose padding collided with a real colour would silently reintroduce
/// exactly the problem it exists to avoid.
#[cfg(all(test, feature = "oracles"))]
mod fixture_derivation {
    /// 🧭️ Walks up from `start` looking for the repo root's own `CLAUDE.md`.
    fn find_repo_root(start: &std::path::Path) -> std::path::PathBuf {
        let mut dir = start.to_path_buf();
        for _ in 0..32 {
            if dir.join("CLAUDE.md").is_file() {
                return dir;
            }
            if !dir.pop() {
                break;
            }
        }
        panic!("could not find repo root (CLAUDE.md) above {}", start.display());
    }

    #[test]
    #[ignore]
    fn derive_real_world_fixture() {
        let repo_root = find_repo_root(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
        let png_path = repo_root.join("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png");
        let out_path = repo_root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🧫️fixtures/🖼️rathaus-ahlen-grundriss.bmp");

        let png_bytes = std::fs::read(&png_path).expect("read the real PNG floor plan");
        let mut reader = png::Decoder::new(std::io::Cursor::new(&png_bytes)).read_info().expect("png: read_info");
        let mut buffer = vec![0u8; reader.output_buffer_size().unwrap_or(0)];
        let frame = reader.next_frame(&mut buffer).expect("png: next_frame");
        assert_eq!(frame.color_type, png::ColorType::Indexed, "the source is an indexed PNG; a resolved one would lose the palette this fixture exists for");
        let table = reader.info().palette.clone().expect("indexed PNG carries a PLTE");
        let mut palette: Vec<[u8; 3]> = table.chunks_exact(3).map(|entry| [entry[0], entry[1], entry[2]]).collect();
        let indices = buffer[..frame.buffer_size()].to_vec();
        assert_eq!(palette.len(), 233, "the real PLTE moved — re-check the padding arithmetic below");

        let mut referenced = [false; 256];
        for index in &indices {
            referenced[*index as usize] = true;
        }
        assert!(referenced[..palette.len()].iter().all(|used| *used), "every real palette entry is referenced; the padding below is what creates the slack");

        let spare: Vec<[u8; 3]> = (0u16..=255).map(|value| [value as u8, 0, (255 - value) as u8]).filter(|candidate| !palette.contains(candidate)).take(7).collect();
        assert_eq!(spare.len(), 7, "the deterministic ramp must yield seven colours the real table does not already hold");
        palette.extend(spare);
        assert_eq!(palette.len(), 240);
        assert!(indices.iter().all(|index| (*index as usize) < 233), "no pixel may reference a spare entry");

        let mut bytes = Vec::new();
        image::codecs::bmp::BmpEncoder::new(&mut bytes).encode_with_palette(&indices, frame.width, frame.height, image::ExtendedColorType::L8, Some(&palette)).expect("image crate: write the indexed BMP");

        let reparsed = super::oracles::decode(&bytes).expect("re-parse the derived fixture with the independent reader");
        assert_eq!((reparsed.width, reparsed.height), (frame.width, frame.height));
        match &reparsed.content {
            super::oracles::Content::Indexed { indices: back, palette: table } => {
                assert_eq!(table.len(), 240, "the derived fixture must declare its full 240-entry table");
                assert_eq!(back, &indices, "the index buffer must survive the round trip through the reference encoder");
            }
            super::oracles::Content::Direct { .. } => panic!("the derived fixture decoded as direct-colour; the palette was lost"),
        }

        std::fs::write(&out_path, &bytes).expect("write the committed shared:// fixture");
        eprintln!("wrote {} ({} bytes, {}x{}, 240-entry table)", out_path.display(), bytes.len(), frame.width, frame.height);
    }
}
//#endregion 🔖️FixtureDerivation
