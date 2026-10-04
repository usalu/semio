//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `raster` module rather than by copying it.
//!
//! Every kind here is performed against the registered `png` crate's own `Encoder`/`Decoder` API
//! directly — never by reusing this repository's own `decode_png`/`encode_png` — so the comparison
//! stays a genuine cross-check.
//!
//! # What this module carries and why
//!
//! [`OracleDoc`] is a WHOLE PNG document, not just its raster: PLTE, the five typed ancillary
//! chunks the crate models (gAMA, cHRM, sRGB, pHYs, bKGD), the tEXt chunks it models, plus tIME and
//! any private/unregistered chunk, which it does not, so a kind whose effect lands outside the raster
//! still moves the projection.
//!
//! `png::Info` is the reference reader for everything the crate models. tIME and unknown chunks
//! come from [`scan_extra_chunks`], a fixed-grammar walk over §5.3's `length/type/data/crc` layout:
//! the crate's `Info` has no `tIME` field at all and no accessor for chunk types it does not
//! recognise, so those two are unreadable through the high-level API. Writing them back is the same
//! story in reverse — `Writer::write_chunk` is the crate's own escape hatch for exactly this.
//!
//! # The vocabulary
//!
//! `PngSnapshot` is byte-authoritative (`{schema, bytes}`, the file's own octets). `set-snapshot`
//! installs a whole file, `patch-snapshot` is one pointer operation on the `{schema, bytes}` reading,
//! `change-gamma` sets or removes gAMA, `patch-pixels` paints one RGBA8 colour into an 8-bit
//! non-interlaced RGBA source, and `paint-native-samples` paints one native sample tuple. The three
//! guarded kinds name the revision they were authored against (64-bit FNV-1a over the schema text and
//! then the octets), which this module recomputes and refuses on mismatch, as the subject does; the
//! paint it compares against `paint-native-samples`' carried `result` is its own. Every inverse is
//! the untouched original, because the vocabulary's own inverse of each kind is a whole
//! `set-snapshot` of its base.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`PngMutation`).

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use semio_repo_test_host::Json;

    //#region 🔖️Json
    /// 🔎️ Numeric member, or `None` for an absent or `null` one — `params` is the leaf's wire payload, whose
    /// `Option` members are `null` when unset.
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
    /// 🧾️ This oracle's own, independent PNG 1.2 document model — every field is read back out of
    /// the registered `png` crate (or, for the two chunk families that crate does not model, out of
    /// §5.3's own chunk grammar) and written back through it. Never through this repository's own
    /// `PngSnapshot`/`decode_png`/`encode_png`.
    pub struct OracleDoc {
        pub width: u32,
        pub height: u32,
        pub rgba: Vec<u8>,
        pub palette: Option<Vec<u8>>,
        pub gama: Option<u32>,
        pub chrm: Option<[u32; 8]>,
        pub srgb: Option<u8>,
        pub phys: Option<(u32, u32, bool)>,
        pub time: Option<[u8; 7]>,
        pub bkgd: Option<[u16; 3]>,
        pub text_chunks: Vec<(String, String)>,
        pub unknown_chunks: Vec<([u8; 4], Vec<u8>)>,
    }

    /// 📇️ Every chunk type [`OracleDoc`] already carries in a typed field. Anything else a file
    /// holds is, by definition, a chunk this model does not understand — exactly the set
    /// `PngSnapshot::unknown_chunks` retains, so the two definitions agree. A type missing from
    /// this list would be captured TWICE (once typed, once verbatim) and re-emitted twice, which is
    /// how `inverse-change-background` first failed: clearing the typed `bkgd` left the verbatim copy
    /// behind and the undo restored nothing.
    const MODELLED: [&[u8; 4]; 14] = [b"IHDR", b"PLTE", b"IDAT", b"IEND", b"tRNS", b"gAMA", b"cHRM", b"sRGB", b"pHYs", b"tIME", b"bKGD", b"tEXt", b"zTXt", b"iTXt"];
    //#endregion 🔖️Doc

    //#region 🔖️ChunkScan
    /// 🔍️ Walks §5.3's `length | type | data | crc` chain and returns the tIME payload plus every
    /// chunk this reference reader does not model. `png::Info` has no `tIME` field and no accessor
    /// for unrecognised types, so this is the only way to see either.
    fn scan_extra_chunks(data: &[u8]) -> Result<(Option<[u8; 7]>, Vec<([u8; 4], Vec<u8>)>), String> {
        if data.len() < 8 || data[0..8] != [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A] {
            return Err("not a PNG byte stream".to_string());
        }
        let mut cursor = 8usize;
        let mut time = None;
        let mut unknown = Vec::new();
        while cursor + 8 <= data.len() {
            let length = u32::from_be_bytes(data[cursor..cursor + 4].try_into().map_err(|_| "truncated PNG chunk length")?) as usize;
            let kind: [u8; 4] = data[cursor + 4..cursor + 8].try_into().map_err(|_| "truncated PNG chunk type")?;
            let start = cursor + 8;
            let end = start.checked_add(length).ok_or("PNG chunk length overflows the stream")?;
            let payload = data.get(start..end).ok_or_else(|| format!("truncated PNG {} chunk payload", String::from_utf8_lossy(&kind)))?;
            match &kind {
                b"tIME" if payload.len() == 7 => time = Some(payload.try_into().expect("7-byte tIME payload")),
                other if MODELLED.contains(&other) => {}
                _ => unknown.push((kind, payload.to_vec())),
            }
            if &kind == b"IEND" {
                break;
            }
            cursor = end + 4;
        }
        Ok((time, unknown))
    }
    //#endregion 🔖️ChunkScan

    //#region 🔖️Decode
    /// 👁️ Decodes with the INDEPENDENT `png` reader, canonicalizing any colour type down to 8-bit
    /// RGBA — mirrors what this repository's own `decode_png` canonicalizes to, so a mutation
    /// applied on top is comparing like with like — and carrying every ancillary chunk alongside.
    ///
    /// ⚠️ `png` 0.18.1 defect: `Info::source_gamma` and `Info::source_chromaticities` are declared,
    /// documented as the members to "prefer … to also get the derived replacement from sRGB
    /// chunks", initialised to `None` in `Info::default` — and then assigned nowhere in the crate.
    /// `decoder/stream.rs`'s `parse_gama`/`parse_chrm` write only `gama_chunk`/`chrm_chunk`. A
    /// caller that follows the crate's own advice therefore reads `None` for every file that
    /// carries a gAMA or cHRM chunk; this module reads the chunk members instead. Found by the
    /// observability law, which failed `mutate-change-gamma` and `mutate-change-chromaticities` because
    /// the values written on the way out were invisible on the way back in.
    pub fn decode(input: &[u8]) -> Result<OracleDoc, String> {
        let mut decoder = png::Decoder::new(std::io::Cursor::new(input));
        decoder.set_transformations(png::Transformations::ALPHA | png::Transformations::STRIP_16);
        let mut reader = decoder.read_info().map_err(|error| format!("independent reader could not parse the PNG: {error}"))?;
        let mut buffer = vec![0; reader.output_buffer_size().unwrap_or(0)];
        let frame = reader.next_frame(&mut buffer).map_err(|error| format!("independent reader could not decode the PNG: {error}"))?;
        let info = reader.info();
        let rgba = rgba_from(&buffer[..frame.buffer_size()], frame.color_type, info.palette.as_deref(), info.trns.as_deref())?;
        let chrm = info.chrm_chunk.map(|c| [c.white.0.into_scaled(), c.white.1.into_scaled(), c.red.0.into_scaled(), c.red.1.into_scaled(), c.green.0.into_scaled(), c.green.1.into_scaled(), c.blue.0.into_scaled(), c.blue.1.into_scaled()]);
        let bkgd = info.bkgd.as_deref().and_then(|raw| (raw.len() >= 6).then(|| [u16::from_be_bytes([raw[0], raw[1]]), u16::from_be_bytes([raw[2], raw[3]]), u16::from_be_bytes([raw[4], raw[5]])]));
        let text_chunks = info.uncompressed_latin1_text.iter().map(|chunk| (chunk.keyword.clone(), chunk.text.clone())).collect();
        let (time, unknown_chunks) = scan_extra_chunks(input)?;
        Ok(OracleDoc {
            width: frame.width,
            height: frame.height,
            rgba,
            palette: info.palette.as_deref().map(|p| p.to_vec()),
            gama: info.gama_chunk.map(|value| value.into_scaled()),
            chrm,
            srgb: info.srgb.map(srgb_code),
            phys: info.pixel_dims.map(|dims| (dims.xppu, dims.yppu, matches!(dims.unit, png::Unit::Meter))),
            time,
            bkgd,
            text_chunks,
            unknown_chunks,
        })
    }

    fn srgb_code(intent: png::SrgbRenderingIntent) -> u8 {
        match intent {
            png::SrgbRenderingIntent::Perceptual => 0,
            png::SrgbRenderingIntent::RelativeColorimetric => 1,
            png::SrgbRenderingIntent::Saturation => 2,
            png::SrgbRenderingIntent::AbsoluteColorimetric => 3,
        }
    }

    fn srgb_intent(code: u8) -> png::SrgbRenderingIntent {
        match code {
            1 => png::SrgbRenderingIntent::RelativeColorimetric,
            2 => png::SrgbRenderingIntent::Saturation,
            3 => png::SrgbRenderingIntent::AbsoluteColorimetric,
            _ => png::SrgbRenderingIntent::Perceptual,
        }
    }

    fn rgba_from(buffer: &[u8], color: png::ColorType, palette: Option<&[u8]>, transparency: Option<&[u8]>) -> Result<Vec<u8>, String> {
        match color {
            png::ColorType::Rgba => Ok(buffer.to_vec()),
            png::ColorType::Rgb => Ok(buffer.chunks_exact(3).flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 255]).collect()),
            png::ColorType::Grayscale => Ok(buffer.iter().flat_map(|value| [*value, *value, *value, 255]).collect()),
            png::ColorType::GrayscaleAlpha => Ok(buffer.chunks_exact(2).flat_map(|pixel| [pixel[0], pixel[0], pixel[0], pixel[1]]).collect()),
            png::ColorType::Indexed => {
                let table = palette.ok_or("indexed PNG without a palette")?;
                Ok(buffer
                    .iter()
                    .flat_map(|index| {
                        let base = (*index as usize) * 3;
                        let alpha = transparency.and_then(|values| values.get(*index as usize).copied()).unwrap_or(255);
                        [table.get(base).copied().unwrap_or(0), table.get(base + 1).copied().unwrap_or(0), table.get(base + 2).copied().unwrap_or(0), alpha]
                    })
                    .collect())
            }
        }
    }
    /// 🧪️ The independent reader's source profile and normalized RGBA8 samples.
    pub struct SourceProjection {
        pub width: u32,
        pub height: u32,
        pub bit_depth: u8,
        pub color_type: u8,
        pub interlaced: bool,
        pub rgba: Vec<u8>,
    }

    /// 🔬️ Reads source metadata before applying the crate's own lossless color8 transformation.
    pub fn inspect_source(input: &[u8]) -> Result<SourceProjection, String> {
        let mut metadata_reader = png::Decoder::new(std::io::Cursor::new(input))
            .read_info()
            .map_err(|error| format!("independent reader could not parse the PNG: {error}"))?;
        let info = metadata_reader.info();
        let (width, height, bit_depth, color_type, interlaced) = (
            info.width,
            info.height,
            info.bit_depth as u8,
            info.color_type as u8,
            info.interlaced,
        );
        let mut metadata_buffer = vec![0; metadata_reader.output_buffer_size().unwrap_or(0)];
        metadata_reader.next_frame(&mut metadata_buffer).map_err(|error| format!("independent reader could not decode the source PNG: {error}"))?;
        let normalized = decode(input)?;
        Ok(SourceProjection { width, height, bit_depth, color_type, interlaced, rgba: normalized.rgba })
    }
    //#endregion 🔖️Decode

    //#region 🔖️Encode
    /// 🔮️ Re-serializes the whole document with the registered `png` writer. Colour type 6 (RGBA) /
    /// bit depth 8 / interlace 0 for the pixel data — the same real, observable limitation this
    /// repository's own `encode_png` documents in its `🚫️EncodeScopeNote` — and every ancillary
    /// chunk honestly re-emitted, through the crate's typed setters where it has one and through
    /// `Writer::write_chunk` (its own escape hatch) where it does not.
    ///
    /// tRNS is deliberately never written: §11.3.3 forbids it alongside colour type 6, and emitting
    /// it anyway produces a file this same crate's decoder refuses. See the module docstring.
    pub fn encode(doc: &OracleDoc) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut out, doc.width, doc.height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            if let Some(palette) = &doc.palette {
                encoder.set_palette(palette.clone());
            }
            if let Some(gama) = doc.gama {
                encoder.set_source_gamma(png::ScaledFloat::from_scaled(gama));
            }
            if let Some(chrm) = doc.chrm {
                let scaled = |index: usize| png::ScaledFloat::from_scaled(chrm[index]);
                encoder.set_source_chromaticities(png::SourceChromaticities { white: (scaled(0), scaled(1)), red: (scaled(2), scaled(3)), green: (scaled(4), scaled(5)), blue: (scaled(6), scaled(7)) });
            }
            if let Some(srgb) = doc.srgb {
                encoder.set_source_srgb(srgb_intent(srgb));
            }
            if let Some((xppu, yppu, meter)) = doc.phys {
                encoder.set_pixel_dims(Some(png::PixelDimensions { xppu, yppu, unit: if meter { png::Unit::Meter } else { png::Unit::Unspecified } }));
            }
            for (keyword, value) in &doc.text_chunks {
                encoder.add_text_chunk(keyword.clone(), value.clone()).map_err(|error| format!("png text chunk: {error}"))?;
            }
            let mut writer = encoder.write_header().map_err(|error| format!("png header: {error}"))?;
            if let Some(time) = doc.time {
                writer.write_chunk(png::chunk::ChunkType(*b"tIME"), &time).map_err(|error| format!("png tIME chunk: {error}"))?;
            }
            if let Some(bkgd) = doc.bkgd {
                let bytes: Vec<u8> = bkgd.iter().flat_map(|value| value.to_be_bytes()).collect();
                writer.write_chunk(png::chunk::ChunkType(*b"bKGD"), &bytes).map_err(|error| format!("png bKGD chunk: {error}"))?;
            }
            for (kind, data) in &doc.unknown_chunks {
                writer.write_chunk(png::chunk::ChunkType(*kind), data).map_err(|error| format!("png {} chunk: {error}", String::from_utf8_lossy(kind)))?;
            }
            writer.write_image_data(&doc.rgba).map_err(|error| format!("png data: {error}"))?;
        }
        Ok(out)
    }
    //#endregion 🔖️Encode

    //#region 🔖️Forward
    /// 🔖️ The revision a guarded kind must name: 64-bit FNV-1a over the `schema` text `stdio.png`, then the file's octets.
    fn revision(input: &[u8]) -> String {
        let hash = b"stdio.png".iter().chain(input).fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3));
        format!("{hash:016x}")
    }

    /// 🔲️ A paint rectangle (`{x, y, width, height}`, `y` counted from the image top); empty or out of bounds is refused.
    fn region(doc: &OracleDoc, rectangle: &Json) -> Result<(usize, usize, usize, usize), String> {
        let [x, y, width, height] = ["x", "y", "width", "height"].map(|key| num(rectangle, key).unwrap_or(0.0) as usize);
        if width == 0 || height == 0 || x + width > doc.width as usize || y + height > doc.height as usize {
            return Err(format!("paint region {x},{y} {width}x{height} is empty or leaves the {}x{} image", doc.width, doc.height));
        }
        Ok((x, y, width, height))
    }

    /// 🖌️ Writes one RGBA8 colour into every pixel of a rectangle of the decoded raster.
    fn fill(doc: &mut OracleDoc, rectangle: &Json, color: [u8; 4]) -> Result<(), String> {
        let (x, y, width, height) = region(doc, rectangle)?;
        let stride = doc.width as usize;
        for row in y..y + height {
            for column in x..x + width {
                let at = (row * stride + column) * 4;
                doc.rgba[at..at + 4].copy_from_slice(&color);
            }
        }
        Ok(())
    }

    /// 🎨️ The RGBA8 colour a native-sample paint means on an 8-bit source, read through the reference reader's own
    /// header, palette and tRNS: the profile must be the source's, a palette index must address an entry.
    fn native_color(input: &[u8], paint: &Json) -> Result<[u8; 4], String> {
        let reader = png::Decoder::new(std::io::Cursor::new(input)).read_info().map_err(|error| format!("independent reader could not parse the PNG: {error}"))?;
        let info = reader.info();
        if info.bit_depth != png::BitDepth::Eight {
            return Err("this oracle paints native samples of 8-bit sources only".to_string());
        }
        let profile = match info.color_type {
            png::ColorType::Indexed => "indexed",
            png::ColorType::Grayscale => "grayscale",
            png::ColorType::GrayscaleAlpha => "grayscale-alpha",
            png::ColorType::Rgb => "rgb",
            png::ColorType::Rgba => "rgba",
        };
        if paint.str("profile") != profile {
            return Err(format!("native paint profile {:?} is not the source's {profile}", paint.str("profile")));
        }
        let [first, second, third, fourth] = ["first", "second", "third", "fourth"].map(|key| num(paint, key).unwrap_or(0.0) as usize);
        if [first, second, third, fourth].iter().any(|sample| *sample > 255) {
            return Err("a native sample exceeds 255 for an 8-bit source".to_string());
        }
        let sample = |value: usize| value as u8;
        Ok(match profile {
            "indexed" => {
                let palette = info.palette.as_deref().unwrap_or(&[]);
                let entry = palette.get(first * 3..first * 3 + 3).ok_or_else(|| format!("palette index {first} exceeds {} entries", palette.len() / 3))?;
                [entry[0], entry[1], entry[2], info.trns.as_deref().and_then(|alpha| alpha.get(first).copied()).unwrap_or(255)]
            }
            "grayscale" => [sample(first), sample(first), sample(first), 255],
            "grayscale-alpha" => [sample(first), sample(first), sample(first), sample(second)],
            "rgb" => [sample(first), sample(second), sample(third), 255],
            _ => [sample(first), sample(second), sample(third), sample(fourth)],
        })
    }

    /// 🦠️ One declared kind applied to the real artifact, independently of the subject: the revision guard and every
    /// effect are recomputed here from their definitions, and the result is re-encoded by the reference writer.
    fn apply_kind(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if matches!(kind, "change-gamma" | "patch-pixels" | "paint-native-samples") && params.str("revision") != revision(input) {
            return Err(format!("{kind} names revision {:?}, not this document's {}", params.str("revision"), revision(input)));
        }
        let mut doc = decode(input)?;
        match kind {
            "set-snapshot" => {
                let snapshot = params.get("snapshot").ok_or("set-snapshot carries no snapshot")?;
                if snapshot.str("schema") != "stdio.png" {
                    return Err(format!("set-snapshot installs schema {:?}, not stdio.png", snapshot.str("schema")));
                }
                doc = decode(&bytes_of(snapshot, "bytes")?)?;
            }
            "patch-snapshot" => {
                let reading = Json::Object(vec![("schema".to_string(), Json::String("stdio.png".to_string())), ("bytes".to_string(), Json::Array(input.iter().map(|byte| Json::Number(f64::from(*byte))).collect()))]);
                let patched = semio_repo_test_host::law::patched_snapshot(&reading, params.get("patch").ok_or("patch-snapshot carries no patch")?)?;
                doc = decode(&bytes_of(&patched, "bytes")?)?;
            }
            "change-gamma" => {
                doc.gama = num(params, "gama").map(|value| value as u32);
                if doc.gama == Some(0) {
                    return Err("gAMA must be nonzero".to_string());
                }
            }
            "patch-pixels" => {
                let source = inspect_source(input)?;
                if (source.color_type, source.bit_depth, source.interlaced) != (6, 8, false) {
                    return Err("patch-pixels paints 8-bit non-interlaced RGBA sources only".to_string());
                }
                let color = ["red", "green", "blue", "alpha"].map(|key| num(params, key).unwrap_or(0.0) as u8);
                fill(&mut doc, params, color)?;
            }
            "paint-native-samples" => {
                let color = native_color(input, params.get("paint").ok_or("paint-native-samples carries no paint")?)?;
                fill(&mut doc, params.get("region").ok_or("paint-native-samples carries no region")?, color)?;
            }
            "" => return Err("mutation spec carries no `kind`".to_string()),
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        encode(&doc)
    }
    //#endregion 🔖️Forward

    //#region 🔖️Dispatch
    /// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized
    /// bytes. An unrecognised kind is an error, never a silent no-op: a mutation that is quietly
    /// skipped reports as a passing test.
    pub fn apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        apply_kind(input, &spec.str("kind"), &spec.get("params").cloned().unwrap_or_else(empty_params))
    }

    /// ↩️ The `inverse-<kind>` scenarios' oracle: every kind of this vocabulary is undone by restoring the pre-mutation
    /// document, re-encoded by the reference writer. Routing through `mutated` first is what gives the law teeth: a
    /// forward result the independent reader cannot re-parse fails here instead of passing as `inverse-<kind>`.
    pub fn undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        decode(mutated)?;
        match spec.str("kind").as_str() {
            "set-snapshot" | "patch-snapshot" | "change-gamma" | "patch-pixels" | "paint-native-samples" => encode(&decode(original_input)?),
            "" => Err("mutation spec carries no `kind`".to_string()),
            other => Err(format!("mutation kind {other:?} has no oracle inverse")),
        }
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Projection
    /// #⃣️ FNV-1a, 64-bit, dependency-free — a content digest is the practical stand-in for "every
    /// decoded sample" at this fixture's real size (2334x2560 = ~23.9 MB of RGBA8). PNG is lossless
    /// so exact sample comparison is the right claim to make, but the shared `raster::project_png`'s
    /// own projection embeds the FULL sample array as JSON numbers — fine at the 4x4/7x3 scale
    /// `🎨️create-and-round-trip-png` uses, and unworkable across this case's 35 scenarios. A digest
    /// carries the same exactness (two RGBA buffers agree iff their digests do) at a size the
    /// comparison engine can actually hold and diff.
    fn digest_hex(bytes: &[u8]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{hash:016x}")
    }

    fn quad_or_null(values: Option<[u32; 8]>) -> Json {
        match values {
            Some(values) => Json::Array(values.iter().map(|value| Json::Number(*value as f64)).collect()),
            None => Json::Null,
        }
    }

    /// 👁️ The surface every `mutate-<kind>`/`inverse-<kind>`/`identity-round-trip` scenario compares
    /// oracle against subject through, read back by THIS module's own independent [`decode`].
    ///
    /// Everything a kind can reach is reported: the palette, the five typed ancillary chunks, the
    /// timestamp, the background, the text chunks by keyword and value, the unknown chunks by type and
    /// payload digest, and a digest of the decoded samples.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let doc = decode(bytes)?;
        let text: Vec<Json> = doc.text_chunks.iter().map(|(keyword, value)| Json::Object(vec![("keyword".to_string(), Json::String(keyword.clone())), ("value".to_string(), Json::String(value.clone()))])).collect();
        let unknown: Vec<Json> = doc.unknown_chunks.iter().map(|(kind, data)| Json::Object(vec![("kind".to_string(), Json::String(String::from_utf8_lossy(kind).into_owned())), ("bytes".to_string(), Json::Number(data.len() as f64)), ("digest".to_string(), Json::String(digest_hex(data)))])).collect();
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String("png".to_string())),
            ("width".to_string(), Json::Number(doc.width as f64)),
            ("height".to_string(), Json::Number(doc.height as f64)),
            ("channels".to_string(), Json::Number(4.0)),
            ("bitDepth".to_string(), Json::Number(8.0)),
            ("paletteEntries".to_string(), Json::Number(doc.palette.as_ref().map_or(0, |p| p.len() / 3) as f64)),
            ("paletteDigest".to_string(), Json::String(digest_hex(doc.palette.as_deref().unwrap_or(&[])))),
            ("gamma".to_string(), doc.gama.map_or(Json::Null, |value| Json::Number(value as f64))),
            ("chromaticities".to_string(), quad_or_null(doc.chrm)),
            ("srgbIntent".to_string(), doc.srgb.map_or(Json::Null, |value| Json::Number(value as f64))),
            ("physicalDims".to_string(), doc.phys.map_or(Json::Null, |(x, y, meter)| Json::Array(vec![Json::Number(x as f64), Json::Number(y as f64), Json::Bool(meter)]))),
            ("timestamp".to_string(), doc.time.map_or(Json::Null, |value| Json::Array(value.iter().map(|byte| Json::Number(*byte as f64)).collect()))),
            ("background".to_string(), doc.bkgd.map_or(Json::Null, |value| Json::Array(value.iter().map(|channel| Json::Number(*channel as f64)).collect()))),
            ("textChunks".to_string(), Json::Array(text)),
            ("unknownChunks".to_string(), Json::Array(unknown)),
            ("sampleDigest".to_string(), Json::String(digest_hex(&doc.rgba))),
        ]))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
#[cfg(feature = "oracles")]
pub use oracles::SourceProjection as PngSourceOracleProjection;

#[cfg(feature = "oracles")]
pub fn project_png_source(input: &[u8]) -> Result<PngSourceOracleProjection, String> {
    oracles::inspect_source(input)
}

#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::apply_mutation(input, spec)
}

#[cfg(feature = "oracles")]
pub fn oracle_undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    oracles::undo_mutation(original_input, spec, mutated)
}

/// 👁️ Projects mutation-case bytes (oracle or subject, either role) onto the shape every
/// `mutate-<kind>`/`inverse-<kind>`/`identity-round-trip` scenario compares under
/// `@comparison-semantic-raster-v1`. @see `oracles::project`'s own doc comment.
#[cfg(feature = "oracles")]
pub fn project_png_mutation(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
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
pub fn project_png_mutation(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region RoundTrip
#[cfg(feature = "oracles")]
pub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> { oracles::encode(&oracles::decode(input)?) }
#[cfg(not(feature = "oracles"))]
pub fn oracle_identity_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> { Err("the oracles feature is disabled".into()) }
//#endregion RoundTrip
