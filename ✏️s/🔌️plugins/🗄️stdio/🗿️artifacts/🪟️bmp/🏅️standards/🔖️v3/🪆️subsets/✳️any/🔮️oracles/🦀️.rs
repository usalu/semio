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
//! resolves those to RGBA and throws both away, so an indexed paint could not be performed on it.
//! `image` does expose the indexed layer: `BmpDecoder::set_indexed_color(true)` hands back the raw
//! index buffer instead of resolved pixels, `BmpDecoder::get_palette` hands back the table, and
//! `BmpEncoder::encode_with_palette` writes both back as a real indexed BITMAPINFOHEADER.
//! [`OracleDoc`] therefore carries indices and a palette for a palettized file and RGBA for a
//! direct-colour one.
//!
//! `get_palette` always returns 256 entries — `read_palette` zero-pads deliberately, "to prevent
//! corrupt files from causing an out-of-bounds array access" — and the crate exposes `biClrUsed`
//! nowhere, so the table's real length comes from the BITMAPINFOHEADER directly. The same header
//! walk supplies `row_order` (the sign of `biHeight`) and the two pixels-per-metre fields, which
//! `image` neither reads nor writes: its encoder hard-codes both to `0` and always stores rows
//! bottom-up. Those are patched back onto the encoder's own output, in the fixed BMP v3 layout,
//! the same way the GIF subsets patch their Logical Screen Descriptor scalars.
//!
//! # The vocabulary
//!
//! `BmpSnapshot` is byte-authoritative (`{schema, bytes}`, the file's own octets), and its four kinds are performed here
//! independently of the subject: `set-snapshot` installs a whole file, `patch-snapshot` is one pointer operation on the
//! `{schema, bytes}` reading, and the two region paints write one palette index or one colour into an image-top-relative
//! rectangle. Both paints carry the document revision they were authored against (64-bit FNV-1a over the schema text
//! and then the octets), which this module recomputes from that definition and refuses on mismatch, as the subject does.
//! Every inverse is the untouched original, because the vocabulary's own inverse of each kind is a whole `set-snapshot`
//! of its base.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`BmpMutation`).

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use image::ImageDecoder;
    use semio_repo_test_host::{digest, Json};

    const FORMAT: &str = "bmp";

    //#region 🔖️Json
    /// 🔎️ Numeric member, or `None` for an absent one — `params` is the leaf's wire payload, whose `Option`
    /// members the vocabulary leaves out when unset.
    fn num(params: &Json, key: &str) -> Option<f64> {
        match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    /// 🔢️ A byte-array member of a wire value.
    fn bytes_of(params: &Json, key: &str) -> Result<Vec<u8>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items.iter().map(|item| match item { Json::Number(n) if (0.0..=255.0).contains(n) && n.fract() == 0.0 => Ok(*n as u8), other => Err(format!("`{key}` carries {} where a byte belongs", other.to_string())) }).collect(),
            other => Err(format!("`{key}` must be a byte array, not {}", other.map(Json::to_string).unwrap_or_else(|| "nothing".to_string()))),
        }
    }
    fn empty_params() -> Json {
        Json::Object(Vec::new())
    }
    //#endregion 🔖️Json

    //#region 🔖️Doc
    /// 🧾️ This oracle's own, independent BMP v3 document model. `content` distinguishes the two
    /// storage forms BMP v3 actually has, rather than flattening both to RGBA and losing the index
    /// layer an indexed paint operates on.
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
    /// (hard-coded `0`) and the row order (always bottom-up), so a re-encoded document keeps them.
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
                out = semio_s_plugin_stdio_raster_test_oracle::oracle_create_image(&semio_s_plugin_stdio_raster_test_oracle::RasterSpec { width: doc.width, height: doc.height, rgba: rgba.clone() }, FORMAT)?;
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
    /// 🔖️ The revision a region paint must name: 64-bit FNV-1a over the `schema` text `stdio.bmp`, then the file's octets.
    fn revision(input: &[u8]) -> String {
        let hash = b"stdio.bmp".iter().chain(input).fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3));
        format!("{hash:016x}")
    }

    /// 🔲️ A paint payload's `{x, y, width, height}` rectangle, `y` counted from the image top; empty or out of bounds is refused.
    fn region(doc: &OracleDoc, params: &Json) -> Result<(usize, usize, usize, usize), String> {
        let [x, y, width, height] = ["x", "y", "width", "height"].map(|key| num(params, key).unwrap_or(0.0) as usize);
        if width == 0 || height == 0 || x + width > doc.width as usize || y + height > doc.height as usize {
            return Err(format!("paint region {x},{y} {width}x{height} is empty or leaves the {}x{} image", doc.width, doc.height));
        }
        Ok((x, y, width, height))
    }

    /// 🎨️ One region paint on the decoded model: an index into the colour table of an indexed file, or one RGBA colour
    /// into a direct-colour file, each refused on the other storage form.
    fn paint(doc: &mut OracleDoc, kind: &str, params: &Json) -> Result<(), String> {
        let (x, y, width, height) = region(doc, params)?;
        let stride = doc.width as usize;
        match (kind, &mut doc.content) {
            ("paint-indexed-region", Content::Indexed { indices, palette }) => {
                let index = num(params, "paletteIndex").unwrap_or(0.0) as usize;
                if index >= palette.len() {
                    return Err(format!("palette index {index} is outside the {}-entry colour table", palette.len()));
                }
                for row in y..y + height {
                    indices[row * stride + x..row * stride + x + width].fill(index as u8);
                }
            }
            ("paint-direct-region", Content::Direct { rgba }) => {
                let color = ["red", "green", "blue", "alpha"].map(|key| num(params, key).unwrap_or(0.0) as u8);
                for row in y..y + height {
                    for column in x..x + width {
                        let at = (row * stride + column) * 4;
                        rgba[at..at + 4].copy_from_slice(&color);
                    }
                }
            }
            (kind, _) => return Err(format!("{kind} does not apply to this document's storage form")),
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
        let params = spec.get("params").cloned().unwrap_or_else(empty_params);
        match kind.as_str() {
            "patch-snapshot" => patched_bytes(input, &params),
            "set-snapshot" => {
                let snapshot = params.get("snapshot").ok_or("set-snapshot carries no snapshot")?;
                if snapshot.str("schema") != "stdio.bmp" {
                    return Err(format!("set-snapshot installs schema {:?}, not stdio.bmp", snapshot.str("schema")));
                }
                encode(&decode(&bytes_of(snapshot, "bytes")?)?)
            }
            "paint-indexed-region" | "paint-direct-region" => {
                if params.str("revision") != revision(input) {
                    return Err(format!("{kind} names revision {:?}, not this document's {}", params.str("revision"), revision(input)));
                }
                let mut doc = decode(input)?;
                paint(&mut doc, &kind, &params)?;
                encode(&doc)
            }
            "" => Err("mutation spec carries no `kind`".to_string()),
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }

    /// 🩹️ A `patch-snapshot` row's one pointer operation applied to the `BmpSnapshot` reading of the file (`{schema,
    /// bytes}` — the file's own octets), via `semio_repo_test_host::law::patched_snapshot`; the patched octets are the
    /// document, re-parsed by the reference decoder so an unreadable result fails here.
    fn patched_bytes(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let reading = Json::Object(vec![("schema".to_string(), Json::String("stdio.bmp".to_string())), ("bytes".to_string(), Json::Array(input.iter().map(|byte| Json::Number(f64::from(*byte))).collect()))]);
        let patched = semio_repo_test_host::law::patched_snapshot(&reading, params.get("patch").ok_or("patch-snapshot carries no patch")?)?;
        let bytes = bytes_of(&patched, "bytes")?;
        decode(&bytes)?;
        Ok(bytes)
    }

    /// ↩️ The `inverse-<kind>` scenarios' oracle: every kind of this vocabulary is undone by restoring the pre-mutation
    /// document, re-encoded by the reference writer. Routing through `mutated` first is what gives the law teeth: a
    /// forward result the independent reader cannot re-parse fails here instead of passing as `inverse-<kind>`.
    pub fn undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        decode(mutated)?;
        match spec.str("kind").as_str() {
            "set-snapshot" | "patch-snapshot" | "paint-indexed-region" | "paint-direct-region" => encode(&decode(original_input)?),
            "" => Err("mutation spec carries no `kind`".to_string()),
            other => Err(format!("mutation kind {other:?} has no oracle inverse")),
        }
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Project
    /// 👁️ The surface every scenario compares through, read back by THIS module's own independent
    /// [`decode`]. BMP is lossless, so every claim here is exact; `indicesDigest`/`pixelsDigest` are
    /// digests rather than arrays only because the real fixture is 5 975 040 pixels and the
    /// comparison engine would otherwise be diffing ~24 million JSON numbers per scenario.
    ///
    /// The palette is reported as its own length and digest, separately from the index buffer and
    /// the resolved samples, so a table change and an index change stay distinguishable.
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

/// 🖼️ Independent image-rs visual meaning behind an owned test interface.
#[cfg(feature = "oracles")]
pub fn oracle_visual_rgba8(input: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    let decoded = image::load_from_memory_with_format(input, image::ImageFormat::Bmp).map_err(|error| error.to_string())?.to_rgba8();
    Ok((decoded.width(), decoded.height(), decoded.into_raw()))
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
/// `shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp` out of the real
/// `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png`: the
/// independent `png` decoder recovers that file's genuine 233-entry PLTE and its real index buffer,
/// and the `image` reference encoder writes both back as an 8-bit indexed BITMAPINFOHEADER.
///
/// The colour table is padded from its 233 real, all-referenced colours (index 0 alone covers
/// 5 659 668 of 5 975 040 pixels) to 240 entries, so a paint has table entries no pixel resolves to.
///
/// The spare colours are a deterministic ramp chosen at derivation time from values the real
/// palette does not already contain, and the derivation asserts that no pixel references any of
/// them — a padded table whose padding collided with a real colour would silently reintroduce
/// exactly the problem it exists to avoid.
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️fixture-derivation/🦀️.rs"]
mod fixture_derivation;
//#endregion 🔖️FixtureDerivation

//#region RoundTrip
#[cfg(feature = "oracles")]
pub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> { oracles::encode(&oracles::decode(input)?) }
#[cfg(not(feature = "oracles"))]
pub fn oracle_identity_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> { Err("the oracles feature is disabled".into()) }
//#endregion RoundTrip

#[cfg(all(test, feature = "oracles"))]
mod canonical_byte_authority {
    use image::ImageDecoder;

    fn accepted() -> [(&'static str, &'static [u8]); 9] {
        [
            ("direct-rgb24-padding-gap-trailer", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")),
            ("indexed-rgb1-duplicate-palette", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb1-duplicate-palette.bmp")),
            ("indexed-rgb4-duplicate-palette", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb4-duplicate-palette.bmp")),
            ("indexed-rgb8-duplicate-palette", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp")),
            ("direct-rgb16-555", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb16-555.bmp")),
            ("direct-rgb32-reserved-sample", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb32-reserved-sample.bmp")),
            ("direct-bitfields16-565", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields16-565.bmp")),
            ("direct-bitfields32", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields32.bmp")),
            ("direct-rgb24-top-down", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-top-down.bmp")),
        ]
    }

    #[test]
    fn image_decoder_reopens_every_accepted_canonical_fixture() {
        for (id, source) in accepted() {
            let decoder = image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(source)).unwrap_or_else(|failure| panic!("{id}: {failure}"));
            let (width, height) = decoder.dimensions();
            assert!(width > 0 && height > 0, "{id}: dimensions");
            let decoded = image::load_from_memory(source).unwrap_or_else(|failure| panic!("{id}: {failure}")).to_rgba8();
            assert_eq!(decoded.len(), width as usize * height as usize * 4, "{id}: independent RGBA projection");
        }
    }

    #[test]
    fn image_decoder_reopens_neutral_opaque_rgba8_conversion() {
        let source = include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/rgba8-opaque-direct-rgb24.bmp");
        let decoded = image::load_from_memory(source).unwrap().to_rgba8();
        assert_eq!((decoded.width(), decoded.height()), (1, 1));
        assert_eq!(decoded.get_pixel(0, 0).0, [10, 20, 30, 255]);
    }

    #[test]
    fn image_decoder_preserves_eight_bit_duplicate_index_selection() {
        let source = include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp");
        let mut decoder = image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(source)).unwrap();
        decoder.set_indexed_color(true);
        let (width, height) = decoder.dimensions();
        let mut indices = vec![0; width as usize * height as usize];
        decoder.read_image(&mut indices).unwrap();
        assert_eq!(indices, [2, 1, 0]);
    }

    fn fixture_bytes(source: &str) -> Vec<u8> {
        let source: String = source.chars().filter(|character| !character.is_whitespace()).collect();
        assert!(source.len().is_multiple_of(2));
        (0..source.len()).step_by(2).map(|index| u8::from_str_radix(&source[index..index + 2], 16).unwrap()).collect()
    }

    #[test]
    fn image_decoder_reopens_canonical_paint_results() {
        let direct = fixture_bytes(include_str!("../🧫️fixtures/🧬️history-edits/🖌️paint-direct-region/🎯️direct/📸️snapshot/➡️after/🗣️.dsl.semio"));
        let direct = image::load_from_memory(&direct).unwrap().to_rgba8();
        assert_eq!((direct.width(), direct.height()), (3, 2));
        assert_eq!(direct.get_pixel(0, 0).0, [17, 34, 51, 255]);

        let indexed = fixture_bytes(include_str!("../🧫️fixtures/🧬️history-edits/🎨️paint-indexed-region/🎯️direct/📸️snapshot/➡️after/🗣️.dsl.semio"));
        let mut decoder = image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(indexed)).unwrap();
        decoder.set_indexed_color(true);
        let mut indices = vec![0; decoder.total_bytes() as usize];
        decoder.read_image(&mut indices).unwrap();
        assert_eq!(indices, [1, 1, 0]);
    }
}
