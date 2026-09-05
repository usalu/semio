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
//! any private/unregistered chunk, which it does not. A raster-only model was the real defect here:
//! fifteen of this subset's seventeen kinds touch nothing but those chunks, so a decode that threw
//! them away made every one of them re-encode to the same bytes as an unchanged round trip and pass for that
//! reason alone.
//!
//! `png::Info` is the reference reader for everything the crate models. tIME and unknown chunks
//! come from [`scan_extra_chunks`], a fixed-grammar walk over §5.3's `length/type/data/crc` layout:
//! the crate's `Info` has no `tIME` field at all and no accessor for chunk types it does not
//! recognise, so those two are unreadable through the high-level API. Writing them back is the same
//! story in reverse — `Writer::write_chunk` is the crate's own escape hatch for exactly this.
//!
//! # The two kinds that genuinely cannot be observed, and why they are not stubs
//!
//! * `change-header` — this subset's own `encode_png` emits IHDR `[8, 6, 0, 0, 0]` unconditionally
//!   (its `🚫️EncodeScopeNote`), because `PngSnapshot::pixels` is a canonical 8-bit RGBA buffer and
//!   IHDR must describe the IDAT that follows it. `bit_depth`, `color_type` and `interlace` are
//!   therefore fields the model carries and the serialization cannot; `width`/`height` cannot move
//!   either, because `SetHeader` does not resize `pixels` and `encode_png` rejects a snapshot whose
//!   buffer no longer matches its dimensions. The oracle mirrors that exactly.
//! * `change-transparency` — §11.3.3 forbids tRNS alongside colour types 4 and 6, and colour type 6 is
//!   what both encoders always produce. Setting `Some(_)` would emit a file the reference decoder
//!   rejects outright (`png` 0.18 `decoder/stream.rs` `ColorWithBadTrns`); the fixture carries no
//!   tRNS, so removing is a no-op on a chunk that was never there. Both are stated in the feature
//!   file, and both are named in the adapter's observability-law exemption list rather than left to
//!   pass silently.
//!
//! Every other kind — including the three the earlier revision of this module returned unchanged
//! (`remove-text-chunk`, `replace-text-chunk`, `remove-unknown-chunk`) — moves the projection. Those
//! three needed a target to remove, which the real fixture does not carry; [`oracle_arrange`] puts
//! one there first, through this same independent implementation, following the OOXML conformance
//! cases' own `conformance_arrange` precedent rather than inventing a second convention.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use semio_repo_test_host::Json;

    //#region 🔖️Json
    /// 🔎️ Numeric member, or `None` for anything else — the docstring params this reads are
    /// authored directly in the feature file, so a missing/mistyped field is a legitimate default,
    /// never a panic.
    fn num(params: &Json, key: &str) -> Option<f64> {
        match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    fn as_bool(params: &Json, key: &str) -> Option<bool> {
        match params.get(key) {
            Some(Json::Bool(value)) => Some(*value),
            _ => None,
        }
    }
    fn as_str<'a>(params: &'a Json, key: &str) -> Option<&'a str> {
        match params.get(key) {
            Some(Json::String(value)) => Some(value.as_str()),
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
    /// for unrecognised types, so this is the only way to see either — and seeing them is what makes
    /// `change-timestamp`, `insert-unknown-chunk` and `remove-unknown-chunk` observable at all.
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
        let decoder = png::Decoder::new(std::io::Cursor::new(input));
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
    fn fill_quad(params: &Json) -> [u8; 4] {
        let fill = as_arr(params.get("fill").unwrap_or(&Json::Null));
        [num_at(fill, 0).unwrap_or(0.0) as u8, num_at(fill, 1).unwrap_or(0.0) as u8, num_at(fill, 2).unwrap_or(0.0) as u8, num_at(fill, 3).unwrap_or(0.0) as u8]
    }

    fn solid_rgba(width: u32, height: u32, quad: [u8; 4]) -> Vec<u8> {
        quad.iter().copied().cycle().take(width as usize * height as usize * 4).collect()
    }

    fn text_chunk_from(params: &Json) -> (String, String) {
        (as_str(params, "keyword").unwrap_or("Comment").to_string(), as_str(params, "value").unwrap_or("").to_string())
    }

    /// 🗃️ A private/unregistered ancillary chunk from the row's own params, defaulting to `waVe`
    /// (ancillary + private + safe-to-copy, reserved bit correctly uppercase per §5.4).
    fn unknown_chunk_from(params: &Json) -> ([u8; 4], Vec<u8>) {
        let requested = as_str(params, "kind").unwrap_or("waVe");
        let mut kind = *b"waVe";
        for (slot, byte) in kind.iter_mut().zip(requested.bytes()) {
            *slot = byte;
        }
        (kind, as_str(params, "data").unwrap_or("").as_bytes().to_vec())
    }



    /// 🦠️ Applies one declared kind to the document model in place. Out-of-range text/unknown-chunk
    /// indices degrade to a no-op rather than erroring — the same documented behaviour as
    /// `PngMutation::diff` (`../🧬️schema/🧬️mutations/🦀️.rs`), which this independent
    /// implementation deliberately mirrors rather than diverging from without reason.
    fn apply_kind(doc: &mut OracleDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "change-header" => {}
            "replace-palette" => {
                let entries = as_arr(params.get("plte").unwrap_or(&Json::Null));
                doc.palette = match params.get("plte") {
                    Some(Json::Null) | None => None,
                    _ => Some(entries.iter().flat_map(|entry| { let channels = as_arr(entry); [num_at(channels, 0).unwrap_or(0.0) as u8, num_at(channels, 1).unwrap_or(0.0) as u8, num_at(channels, 2).unwrap_or(0.0) as u8] }).collect()),
                };
            }
            "change-transparency" => {}
            "change-gamma" => doc.gama = num(params, "gama").map(|value| value as u32),
            "change-chromaticities" => {
                let read = |key: &str| num(params, key).unwrap_or(0.0) as u32;
                doc.chrm = Some([read("whiteX"), read("whiteY"), read("redX"), read("redY"), read("greenX"), read("greenY"), read("blueX"), read("blueY")]);
            }
            "change-srgb-intent" => {
                doc.srgb = Some(match as_str(params, "srgb") {
                    Some("relative-colorimetric") => 1,
                    Some("saturation") => 2,
                    Some("absolute-colorimetric") => 3,
                    _ => 0,
                })
            }
            "change-physical-dims" => doc.phys = Some((num(params, "ppuX").unwrap_or(0.0) as u32, num(params, "ppuY").unwrap_or(0.0) as u32, as_bool(params, "unitIsMeter").unwrap_or(false))),
            "change-timestamp" => {
                let mut bytes = [0u8; 7];
                bytes[0..2].copy_from_slice(&(num(params, "year").unwrap_or(2024.0) as u16).to_be_bytes());
                bytes[2] = num(params, "month").unwrap_or(1.0) as u8;
                bytes[3] = num(params, "day").unwrap_or(1.0) as u8;
                bytes[4] = num(params, "hour").unwrap_or(0.0) as u8;
                bytes[5] = num(params, "minute").unwrap_or(0.0) as u8;
                bytes[6] = num(params, "second").unwrap_or(0.0) as u8;
                doc.time = Some(bytes);
            }
            "change-background" => doc.bkgd = Some([num(params, "r").unwrap_or(0.0) as u16, num(params, "g").unwrap_or(0.0) as u16, num(params, "b").unwrap_or(0.0) as u16]),
            "insert-text-chunk" => {
                let at = index_of(params).min(doc.text_chunks.len());
                doc.text_chunks.insert(at, text_chunk_from(params));
            }
            "remove-text-chunk" => {
                let at = index_of(params);
                if at < doc.text_chunks.len() {
                    doc.text_chunks.remove(at);
                }
            }
            "replace-text-chunk" => {
                let at = index_of(params);
                if at < doc.text_chunks.len() {
                    doc.text_chunks[at] = text_chunk_from(params);
                }
            }
            "replace-pixels" => doc.rgba = solid_rgba(doc.width, doc.height, fill_quad(params)),
            "insert-unknown-chunk" => {
                let at = index_of(params).min(doc.unknown_chunks.len());
                doc.unknown_chunks.insert(at, unknown_chunk_from(params));
            }
            "remove-unknown-chunk" => {
                let at = index_of(params);
                if at < doc.unknown_chunks.len() {
                    doc.unknown_chunks.remove(at);
                }
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Forward

    //#region 🔖️Dispatch
    /// 🎬️ Prepares the input a kind needs its target to be present in. The real committed fixture is
    /// an 8-bit indexed floor plan whose only chunks are IHDR/PLTE/IDAT/IEND — verified by walking
    /// the file, and stated in the feature — so the three kinds that address an EXISTING text or
    /// unknown chunk are exercised on the real document after this same independent implementation
    /// has inserted their target. Every other kind reads the committed bytes untouched.
    pub fn arrange(input: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
        let seeded = |kind: &str, params: Vec<(&str, Json)>| -> Result<Vec<u8>, String> {
            let mut doc = decode(input)?;
            apply_kind(&mut doc, kind, &Json::Object(params.into_iter().map(|(key, value)| (key.to_string(), value)).collect()))?;
            encode(&doc)
        };
        // 🎯️ The seeded target's content is deliberately NOT the row's own params: seeding with the
        // same keyword and value the row then sets would make `replace-text-chunk` replace a chunk with
        // its own twin, which is a mutation nothing can observe.
        let text = || vec![("index", Json::Number(0.0)), ("keyword", Json::String("Source".to_string())), ("value", Json::String("arranged target, present only so a removal has something to remove".to_string()))];
        let unknown = || vec![("index", Json::Number(0.0)), ("kind", Json::String("seEd".to_string())), ("data", Json::String("arranged target".to_string()))];
        match forward.str("kind").as_str() {
            "remove-text-chunk" | "replace-text-chunk" => seeded("insert-text-chunk", text()),
            "remove-unknown-chunk" => seeded("insert-unknown-chunk", unknown()),
            _ => Ok(input.to_vec()),
        }
    }

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
    /// the pre-mutation state out of `base` through this same independent implementation and
    /// applied to the forward mutation's real output. `PngMutation::inverse` (the vocabulary's own
    /// algebraic law) is defined, per variant, as "restore `base`'s own value for the field this
    /// kind touches"; every arm below is that rule reimplemented here, never that function called.
    pub fn undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let params = spec.get("params").cloned().unwrap_or_else(empty_params);
        let original = decode(original_input)?;
        let mut doc = decode(mutated)?;
        match kind.as_str() {
            "change-header" | "change-transparency" => {}
            "replace-palette" => doc.palette = original.palette,
            "change-gamma" => doc.gama = original.gama,
            "change-chromaticities" => doc.chrm = original.chrm,
            "change-srgb-intent" => doc.srgb = original.srgb,
            "change-physical-dims" => doc.phys = original.phys,
            "change-timestamp" => doc.time = original.time,
            "change-background" => doc.bkgd = original.bkgd,
            "replace-pixels" => doc.rgba = original.rgba,
            "insert-text-chunk" => {
                let at = index_of(&params).min(original.text_chunks.len());
                if at < doc.text_chunks.len() {
                    doc.text_chunks.remove(at);
                }
            }
            "remove-text-chunk" | "replace-text-chunk" => doc.text_chunks = original.text_chunks,
            "insert-unknown-chunk" => {
                let at = index_of(&params).min(original.unknown_chunks.len());
                if at < doc.unknown_chunks.len() {
                    doc.unknown_chunks.remove(at);
                }
            }
            "remove-unknown-chunk" => doc.unknown_chunks = original.unknown_chunks,
            other => return Err(format!("mutation kind {other:?} has no oracle inverse")),
        }
        encode(&doc)
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
    /// The earlier revision reported geometry and a sample digest only, which meant fifteen of the
    /// seventeen declared kinds could not move it — every ancillary-chunk mutation projected exactly
    /// like the untouched input, and its scenario passed for that reason. Everything a kind can
    /// reach is reported here instead: the palette, the five typed ancillary chunks, the timestamp,
    /// the background, the text chunks by keyword and value, and the unknown chunks by type and
    /// payload digest.
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
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::apply_mutation(input, spec)
}

/// 🎬️ The pre-state a kind needs to have something to act on. @see `oracles::arrange`.
#[cfg(feature = "oracles")]
pub fn oracle_arrange(input: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
    oracles::arrange(input, forward)
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
pub fn oracle_arrange(_input: &[u8], _forward: &Json) -> Result<Vec<u8>, String> {
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
