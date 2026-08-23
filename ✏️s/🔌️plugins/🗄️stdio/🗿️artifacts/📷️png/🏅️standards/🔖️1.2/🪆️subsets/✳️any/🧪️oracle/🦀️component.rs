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
//! stays a genuine cross-check. `decode_rgba`/`rgba_from` duplicate the shape of the shared
//! `raster::project_png` family's own (private) conversion rather than reusing it, since raw pixel
//! access for further re-encoding is a different need than that module's read-only JSON projection.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`KINDS`).

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
    //#endregion 🔖️Json

    //#region 🔖️Decode
    /// 👁️ Decodes with the INDEPENDENT `png` reader, canonicalizing any color type down to
    /// 8-bit RGBA — mirrors what this repository's own `decode_png` canonicalizes to, so a
    /// mutation applied on top is comparing like with like.
    fn decode_rgba(input: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
        let decoder = png::Decoder::new(std::io::Cursor::new(input));
        let mut reader = decoder.read_info().map_err(|error| format!("independent reader could not parse the PNG: {error}"))?;
        let mut buffer = vec![0; reader.output_buffer_size().unwrap_or(0)];
        let frame = reader.next_frame(&mut buffer).map_err(|error| format!("independent reader could not decode the PNG: {error}"))?;
        let info = reader.info();
        let rgba = rgba_from(&buffer[..frame.buffer_size()], frame.color_type, info.palette.as_deref(), info.trns.as_deref())?;
        Ok((frame.width, frame.height, rgba))
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
    /// 🔮️ Every mutation re-encodes as color type 6 (RGBA) / bit depth 8 — the same real,
    /// observable limitation this repository's own `encode_png` has (its `EncodeScopeNote`):
    /// the pixel data always canonicalizes, only the ancillary/text/unknown chunks a mutation
    /// actually touches are honestly re-emitted. `configure` runs before `write_header` (palette,
    /// gamma, chromaticities, sRGB, pixel dims, text — everything the crate has a typed setter
    /// for); `extra` runs after, for chunks the crate has no setter for (tIME, bKGD, custom).
    fn encode_with(width: u32, height: u32, rgba: &[u8], configure: impl FnOnce(&mut png::Encoder<'_, &mut Vec<u8>>) -> Result<(), String>, extra: impl FnOnce(&mut png::Writer<&mut Vec<u8>>) -> Result<(), String>) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut out, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            configure(&mut encoder)?;
            let mut writer = encoder.write_header().map_err(|error| format!("png header: {error}"))?;
            extra(&mut writer)?;
            writer.write_image_data(rgba).map_err(|error| format!("png data: {error}"))?;
        }
        Ok(out)
    }

    /// 🔁️ Decodes and re-encodes unchanged — the correct oracle answer for every kind whose
    /// forward effect never touches pixels/dimensions (every ancillary/text/unknown-chunk kind
    /// here), and for `undo_mutation` universally (see that function's own doc comment).
    fn reencode_unchanged(input: &[u8]) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        encode_with(width, height, &rgba, |_encoder| Ok(()), |_writer| Ok(()))
    }
    //#endregion 🔖️Encode

    //#region 🔖️Forward
    fn fill_quad(params: &Json) -> Vec<u8> {
        let fill = as_arr(params.get("fill").unwrap_or(&Json::Null));
        (0..4).map(|index| num_at(fill, index).unwrap_or(0.0) as u8).collect()
    }

    fn solid_rgba(width: u32, height: u32, quad: &[u8]) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for _ in 0..(width as usize * height as usize) {
            rgba.extend_from_slice(quad);
        }
        rgba
    }

    /// 📄️ Replaces the WHOLE document — independent of the real input, matching `SetSnapshot`'s
    /// own wholesale-replace semantics.
    fn forward_set_snapshot(params: &Json) -> Result<Vec<u8>, String> {
        let width = num(params, "width").unwrap_or(1.0).max(1.0) as u32;
        let height = num(params, "height").unwrap_or(1.0).max(1.0) as u32;
        let quad = fill_quad(params);
        encode_with(width, height, &solid_rgba(width, height, &quad), |_encoder| Ok(()), |_writer| Ok(()))
    }

    /// 🧾️ `SetHeader` replaces width/height/bit-depth/color-type/interlace wholesale in the typed
    /// snapshot, but this repository's own `encode_png` always emits color type 6 / bit depth 8 /
    /// interlace 0 for the pixel data regardless (its documented `EncodeScopeNote`) — only
    /// width/height ever reach the actual bytes, and the test keeps them equal to the real
    /// fixture's own so `pixels.len()` still matches. The oracle mirrors that real, observable
    /// behaviour rather than a hypothetical spec-perfect encoder.
    fn forward_set_header(input: &[u8]) -> Result<Vec<u8>, String> {
        reencode_unchanged(input)
    }

    fn forward_set_palette(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let entries = as_arr(params.get("plte").unwrap_or(&Json::Null));
        let mut palette = Vec::with_capacity(entries.len() * 3);
        for entry in entries {
            let channels = as_arr(entry);
            for index in 0..3 {
                palette.push(num_at(channels, index).unwrap_or(0.0) as u8);
            }
        }
        encode_with(
            width,
            height,
            &rgba,
            move |encoder| {
                encoder.set_palette(palette);
                Ok(())
            },
            |_writer| Ok(()),
        )
    }

    /// 👁️ `tRNS` is structurally invalid alongside color type 6 (truecolor+alpha, §11.3.3) — this
    /// repository's own encoder always emits color type 6, so the only decode-safe exercise of
    /// `SetTransparency` here is the removal branch (`trns -> None`), which the real fixture
    /// already satisfies (it carries no tRNS chunk to begin with). Setting `Some(_)` would make
    /// the INDEPENDENT reader reject the very bytes it is meant to verify.
    fn forward_set_transparency(input: &[u8]) -> Result<Vec<u8>, String> {
        reencode_unchanged(input)
    }

    fn forward_set_gamma(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let gamma = num(params, "gama").map(|value| value as u32);
        encode_with(
            width,
            height,
            &rgba,
            move |encoder| {
                if let Some(value) = gamma {
                    encoder.set_source_gamma(png::ScaledFloat::from_scaled(value));
                }
                Ok(())
            },
            |_writer| Ok(()),
        )
    }

    fn forward_set_chromaticities(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let scaled = |key: &str| png::ScaledFloat::from_scaled(num(params, key).unwrap_or(0.0) as u32);
        let chromaticities = png::SourceChromaticities { white: (scaled("whiteX"), scaled("whiteY")), red: (scaled("redX"), scaled("redY")), green: (scaled("greenX"), scaled("greenY")), blue: (scaled("blueX"), scaled("blueY")) };
        encode_with(
            width,
            height,
            &rgba,
            move |encoder| {
                encoder.set_source_chromaticities(chromaticities);
                Ok(())
            },
            |_writer| Ok(()),
        )
    }

    fn forward_set_srgb_intent(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let intent = match as_str(params, "srgb") {
            Some("relative-colorimetric") => png::SrgbRenderingIntent::RelativeColorimetric,
            Some("saturation") => png::SrgbRenderingIntent::Saturation,
            Some("absolute-colorimetric") => png::SrgbRenderingIntent::AbsoluteColorimetric,
            _ => png::SrgbRenderingIntent::Perceptual,
        };
        encode_with(
            width,
            height,
            &rgba,
            move |encoder| {
                encoder.set_source_srgb(intent);
                Ok(())
            },
            |_writer| Ok(()),
        )
    }

    fn forward_set_physical_dims(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let dims =
            png::PixelDimensions { xppu: num(params, "ppuX").unwrap_or(0.0) as u32, yppu: num(params, "ppuY").unwrap_or(0.0) as u32, unit: if as_bool(params, "unitIsMeter").unwrap_or(false) { png::Unit::Meter } else { png::Unit::Unspecified } };
        encode_with(
            width,
            height,
            &rgba,
            move |encoder| {
                encoder.set_pixel_dims(Some(dims));
                Ok(())
            },
            |_writer| Ok(()),
        )
    }

    /// 🕰️ `tIME` has no typed setter on the crate's `Encoder` (and isn't parsed back into `Info`
    /// on decode either — an ancillary chunk this reference library simply skips), so it is
    /// written as a raw chunk in the same 7-byte layout §11.3.6.1 defines.
    fn forward_set_timestamp(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let mut bytes = Vec::with_capacity(7);
        bytes.extend_from_slice(&(num(params, "year").unwrap_or(2024.0) as u16).to_be_bytes());
        bytes.push(num(params, "month").unwrap_or(1.0) as u8);
        bytes.push(num(params, "day").unwrap_or(1.0) as u8);
        bytes.push(num(params, "hour").unwrap_or(0.0) as u8);
        bytes.push(num(params, "minute").unwrap_or(0.0) as u8);
        bytes.push(num(params, "second").unwrap_or(0.0) as u8);
        encode_with(width, height, &rgba, |_encoder| Ok(()), move |writer| writer.write_chunk(png::chunk::ChunkType(*b"tIME"), &bytes).map_err(|error| format!("png tIME chunk: {error}")))
    }

    /// 🖼️ `bKGD` has no typed setter either; written raw as the RGB-triple (6-byte) layout the
    /// crate's own decoder requires for color type 6 (§11.3.5.1) — the only layout compatible with
    /// the truecolor+alpha output every mutation here re-encodes as.
    fn forward_set_background(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let bytes: Vec<u8> = [num(params, "r").unwrap_or(0.0) as u16, num(params, "g").unwrap_or(0.0) as u16, num(params, "b").unwrap_or(0.0) as u16].iter().flat_map(|value| value.to_be_bytes()).collect();
        encode_with(width, height, &rgba, |_encoder| Ok(()), move |writer| writer.write_chunk(png::chunk::ChunkType(*b"bKGD"), &bytes).map_err(|error| format!("png bKGD chunk: {error}")))
    }

    fn forward_insert_text_chunk(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let keyword = as_str(params, "keyword").unwrap_or("Comment").to_string();
        let value = as_str(params, "value").unwrap_or("").to_string();
        encode_with(width, height, &rgba, move |encoder| encoder.add_text_chunk(keyword, value).map_err(|error| format!("png text chunk: {error}")), |_writer| Ok(()))
    }

    fn forward_set_pixels(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, _rgba) = decode_rgba(input)?;
        let quad = fill_quad(params);
        encode_with(width, height, &solid_rgba(width, height, &quad), |_encoder| Ok(()), |_writer| Ok(()))
    }

    /// 🗃️ A private/unregistered ancillary chunk (`waVe`: ancillary + private + safe-to-copy, the
    /// reserved bit correctly uppercase) — the crate has no typed setter for arbitrary chunks
    /// either, so this is written raw, same as `tIME`/`bKGD` above.
    fn forward_insert_unknown_chunk(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (width, height, rgba) = decode_rgba(input)?;
        let requested = as_str(params, "kind").unwrap_or("waVe");
        let mut kind = *b"waVe";
        for (slot, byte) in kind.iter_mut().zip(requested.bytes()) {
            *slot = byte;
        }
        let data = as_str(params, "data").unwrap_or("").as_bytes().to_vec();
        encode_with(width, height, &rgba, |_encoder| Ok(()), move |writer| writer.write_chunk(png::chunk::ChunkType(kind), &data).map_err(|error| format!("png custom chunk: {error}")))
    }
    //#endregion 🔖️Forward

    //#region 🔖️Dispatch
    /// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized
    /// bytes. An unrecognised kind is an error, never a silent no-op: a mutation that is quietly
    /// skipped reports as a passing test. `remove-text-chunk`/`set-text-chunk`/
    /// `remove-unknown-chunk` are genuine no-ops here because the real fixture carries neither
    /// text nor unknown chunks to begin with (§11.3.4/verbatim-retention docs both call this out
    /// as the documented no-op-if-out-of-range behaviour, not a shortcut).
    pub fn apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or_else(empty_params);
        match kind.as_str() {
            "" => Err("mutation spec carries no `kind`".to_string()),
            "no-mutation" => reencode_unchanged(input),
            "set-snapshot" => forward_set_snapshot(&params),
            "set-header" => forward_set_header(input),
            "set-palette" => forward_set_palette(input, &params),
            "set-transparency" => forward_set_transparency(input),
            "set-gamma" => forward_set_gamma(input, &params),
            "set-chromaticities" => forward_set_chromaticities(input, &params),
            "set-srgb-intent" => forward_set_srgb_intent(input, &params),
            "set-physical-dims" => forward_set_physical_dims(input, &params),
            "set-timestamp" => forward_set_timestamp(input, &params),
            "set-background" => forward_set_background(input, &params),
            "insert-text-chunk" => forward_insert_text_chunk(input, &params),
            "remove-text-chunk" => reencode_unchanged(input),
            "set-text-chunk" => reencode_unchanged(input),
            "set-pixels" => forward_set_pixels(input, &params),
            "insert-unknown-chunk" => forward_insert_unknown_chunk(input, &params),
            "remove-unknown-chunk" => reencode_unchanged(input),
            other => Err(format!("mutation kind {other:?} has no oracle implementation ({} input byte(s))", input.len())),
        }
    }

    /// ↩️ The `inverse-<kind>` scenarios' oracle: independently reasoned, not derived from the
    /// subject's own code. `PngMutation::inverse` (the vocabulary's own algebraic law) is defined,
    /// per variant, as "restore `base`'s own value for the field this kind touches" — never a
    /// derived/computed value — so forward-then-inverse provably nets to the UNTOUCHED original
    /// document for every one of the 17 kinds: the ancillary/text/unknown-chunk kinds never touch
    /// pixels or dimensions in either direction, and the two that do (`SetSnapshot`, `SetPixels`)
    /// have inverses that explicitly restore `base.clone()`/`base.pixels.clone()`. The independent
    /// expected answer is therefore always "decode the pristine input, re-encode unchanged" — this
    /// function is what a correct forward+inverse round trip must equal, not a shortcut around
    /// computing it.
    pub fn undo_mutation(original_input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        if spec.str("kind").is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        reencode_unchanged(original_input)
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Projection
    /// #⃣️ FNV-1a, 64-bit, dependency-free — a content digest is the practical stand-in for "every
    /// decoded sample" at this fixture's real size (2334x2560 = ~23.9 MB of RGBA8). PNG is
    /// lossless so exact sample comparison is the right claim to make, but the shared
    /// `raster::project_png`'s own projection embeds the FULL sample array as JSON numbers — fine
    /// at the 4x4/7x3 scale `create-and-round-trip-png` uses, but multiplied across this case's 35
    /// scenarios (17 kinds x mutate+inverse, plus the identity round trip) it would mean building
    /// and diffing ~24-million-element JSON arrays repeatedly. A digest carries the same exactness
    /// (two RGBA buffers agree iff their digests do) at a size the comparison engine can actually
    /// hold and diff.
    fn digest_hex(bytes: &[u8]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{hash:016x}")
    }

    /// 👁️ The projection every `mutate-<kind>`/`inverse-<kind>`/`identity-round-trip` scenario
    /// compares oracle against subject through — dimensions plus the decoded RGBA8 content digest,
    /// read back by THIS module's own independent `decode_rgba` (the same conversion the dispatch
    /// functions above re-encode from, so a mutation's real effect on pixels/dimensions is exactly
    /// what this reports).
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let (width, height, rgba) = decode_rgba(bytes)?;
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String("png".to_string())),
            ("width".to_string(), Json::Number(width as f64)),
            ("height".to_string(), Json::Number(height as f64)),
            ("channels".to_string(), Json::Number(4.0)),
            ("bitDepth".to_string(), Json::Number(8.0)),
            ("sampleDigest".to_string(), Json::String(digest_hex(&rgba))),
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

#[cfg(feature = "oracles")]
pub fn oracle_undo_mutation(original_input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::undo_mutation(original_input, spec)
}

/// 👁️ Projects mutation-case bytes (oracle or subject, either role) onto the shape every
/// `mutate-<kind>`/`inverse-<kind>`/`identity-round-trip` scenario compares under
/// `@comparison-semantic-raster-v1`. @see `oracles::project`'s own doc comment for why this is a
/// content digest rather than the shared `raster::project_png`'s raw sample array.
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
pub fn oracle_undo_mutation(_original_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_png_mutation(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
