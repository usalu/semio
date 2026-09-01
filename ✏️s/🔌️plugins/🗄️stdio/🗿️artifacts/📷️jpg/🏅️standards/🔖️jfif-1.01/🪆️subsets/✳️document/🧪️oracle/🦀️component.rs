//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `raster` module rather than by copying it.
//!
//! # What this subset's own codec really writes, and what follows from it
//!
//! `../🚪️io/🦀️component.rs` is a from-scratch baseline JPEG codec, not an `image` wrapper. On
//! encode it regenerates fresh Annex K DQT/DHT tables scaled by `re_encode_quality` and never emits
//! a DRI/restart marker at all — so `SetQuantTable`, `RemoveQuantTable`, `SetHuffmanTable`,
//! `RemoveHuffmanTable` and `SetRestartInterval` mutate the typed snapshot and provably cannot
//! reach the bytes. Those five are the only kinds this module treats as unobservable, they are
//! stated in the feature description, and they are named in the adapter's observability-law
//! exemption list so nothing else can quietly join them.
//!
//! `encode_jpg` DOES write a real JFIF APP0 from `jfif_version`/`jfif_density_units`/
//! `jfif_x_density`/`jfif_y_density` and re-emits `other_segments` verbatim right after it
//! (`🚪️io/🦀️component.rs`, `encode_jfif_app0`). `SetJfifHeader`, `InsertOtherSegment` and
//! `RemoveOtherSegment` are therefore genuinely byte-observable, and this module performs all three
//! for real rather than passing the document through: the earlier revision folded them into the
//! same decode → re-encode as the five table kinds, which meant the reference's answer for a header
//! rewrite and for a 31 KB XMP packet removal was the identical file.
//!
//! # Where the reference library reaches, and where this module has to splice
//!
//! `image` 0.25's `JpegEncoder` writes its own APP0 from a `PixelDensity` (`set_pixel_density`),
//! which covers the density unit and both density values — those go through the crate's own API.
//! The two JFIF version bytes are hard-coded to `1.2` in its `build_jfif_header`, and it has no API
//! for arbitrary APPn/COM segments at all, so this module patches the version bytes at their fixed
//! APP0 offset and splices the other segments in immediately after APP0, in §B.2's own
//! `FF marker | length | payload` layout. Same technique, same justification, as the GIF subsets'
//! Logical Screen Descriptor patches.
//!
//! JPEG is LOSSY, so the raster half of the projection is a coarse luma histogram, never raw
//! samples — this platform's comparison tolerance is per-number and absolute with no aggregate
//! mode, so an exact sample claim through two independently written lossy codecs could only ever
//! pass by accident. The metadata half is exact: JFIF fields and segment payload digests survive a
//! re-encode unchanged or they are wrong.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use image::ImageEncoder;
    use semio_repo_test_host::Json;

    //#region 🔖️Json
    /// 🔎️ `value.get(key)`'s numeric leg — the shared `Json` reader has no numeric accessor of its
    /// own, and a missing or mistyped field in a feature-authored docstring is a legitimate default
    /// rather than a panic.
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    fn text(value: &Json, key: &str) -> String {
        value.str(key)
    }

    /// 🎨️ Reads an `[r,g,b,a]` fill out of `params.fill`, defaulting to a mid-grey opaque pixel.
    fn fill_of(params: &Json) -> [u8; 4] {
        let values = params.array("fill");
        let component = |index: usize, fallback: u8| {
            values
                .get(index)
                .and_then(|found| match found {
                    Json::Number(value) => Some(*value as u8),
                    _ => None,
                })
                .unwrap_or(fallback)
        };
        [component(0, 128), component(1, 128), component(2, 128), component(3, 255)]
    }

    fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
        if !value.len().is_multiple_of(2) {
            return Err(format!("odd-length hex payload {value:?}"));
        }
        (0..value.len()).step_by(2).map(|at| u8::from_str_radix(&value[at..at + 2], 16).map_err(|error| error.to_string())).collect()
    }

    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
    //#endregion 🔖️Json

    //#region 🔖️Doc
    /// 🎚️ Encoder default matching this subset's own `encode_jpg`'s `re_encode_quality.unwrap_or(90)`.
    const DEFAULT_QUALITY: u8 = 90;

    /// 🧾️ This oracle's own, independent JFIF 1.01 document model: the decoded raster, the JFIF APP0
    /// fields, the marker segments a JFIF file carries alongside them, and the re-encode quality.
    /// The five table/restart kinds have no field here BECAUSE the subset's own encoder regenerates
    /// or omits them — see the module docstring; that is a documented property of the codec pair,
    /// not a gap in this model.
    pub struct OracleDoc {
        pub width: u32,
        pub height: u32,
        pub rgba: Vec<u8>,
        pub version: (u8, u8),
        pub density_units: u8,
        pub x_density: u16,
        pub y_density: u16,
        pub other_segments: Vec<(u8, Vec<u8>)>,
        pub quality: u8,
    }
    //#endregion 🔖️Doc

    //#region 🔖️SegmentScan
    /// 🔍️ Walks §B.2's marker sequence from SOI to SOS and returns every `(marker, payload)` pair,
    /// payload excluding the two length bytes. `image` exposes no segment-level reader at all — its
    /// decoder hands back pixels and, at most, an EXIF blob — so this walk is the only way to see
    /// the JFIF APP0 fields or the APPn/COM segments a real file carries.
    fn scan_segments(input: &[u8]) -> Result<Vec<(u8, Vec<u8>)>, String> {
        if input.len() < 4 || input[0] != 0xFF || input[1] != 0xD8 {
            return Err("not a JPEG byte stream (no SOI)".to_string());
        }
        let mut segments = Vec::new();
        let mut cursor = 2usize;
        while cursor + 4 <= input.len() {
            if input[cursor] != 0xFF {
                return Err(format!("expected a marker at byte {cursor}, found 0x{:02x}", input[cursor]));
            }
            let marker = input[cursor + 1];
            if marker == 0xD9 {
                break;
            }
            let length = ((input[cursor + 2] as usize) << 8) | input[cursor + 3] as usize;
            if length < 2 {
                return Err(format!("marker 0x{marker:02x} declares a {length}-byte segment, which cannot hold its own length"));
            }
            let payload = input.get(cursor + 4..cursor + 2 + length).ok_or_else(|| format!("truncated 0x{marker:02x} segment"))?;
            segments.push((marker, payload.to_vec()));
            cursor += 2 + length;
            if marker == 0xDA {
                break;
            }
        }
        Ok(segments)
    }

    /// 🏷️ The five JFIF APP0 fields (T.871 §B.2.4.6.2), or `None` when the segment is an APP0 that
    /// is not a JFIF one — which the caller then retains verbatim as an ordinary other-segment,
    /// exactly as this subset's own `decode_jpg` does.
    fn parse_jfif(payload: &[u8]) -> Option<((u8, u8), u8, u16, u16)> {
        if payload.len() < 12 || &payload[0..5] != b"JFIF\0" {
            return None;
        }
        Some(((payload[5], payload[6]), payload[7], u16::from_be_bytes([payload[8], payload[9]]), u16::from_be_bytes([payload[10], payload[11]])))
    }

    /// 📇️ Whether a marker is one this subset's `other_segments` retains — APP1..APP15, COM, and a
    /// non-JFIF APP0. Mirrors `decode_jpg`'s own arm list (`0xE1..=0xEF | 0xFE`) so the two models
    /// hold the same segments and a comparison between them is meaningful.
    fn is_other_segment(marker: u8) -> bool {
        (0xE1..=0xEF).contains(&marker) || marker == 0xFE
    }
    //#endregion 🔖️SegmentScan

    //#region 🔖️Codec
    /// 👁️ Decodes with the INDEPENDENT `image` reader into raw RGBA8, and reads the JFIF header and
    /// retained segments off the byte stream with [`scan_segments`].
    pub fn decode(input: &[u8]) -> Result<OracleDoc, String> {
        let decoded = image::load_from_memory(input).map_err(|error| format!("independent reader could not parse the jpg: {error}"))?;
        let rgba = decoded.to_rgba8();
        let (width, height) = (rgba.width(), rgba.height());
        let mut doc = OracleDoc { width, height, rgba: rgba.into_raw(), version: (1, 1), density_units: 0, x_density: 1, y_density: 1, other_segments: Vec::new(), quality: DEFAULT_QUALITY };
        for (marker, payload) in scan_segments(input)? {
            if marker == 0xE0 {
                match parse_jfif(&payload) {
                    Some((version, units, x_density, y_density)) => {
                        doc.version = version;
                        doc.density_units = units;
                        doc.x_density = x_density;
                        doc.y_density = y_density;
                    }
                    None => doc.other_segments.push((marker, payload)),
                }
            } else if is_other_segment(marker) {
                doc.other_segments.push((marker, payload));
            }
        }
        Ok(doc)
    }

    /// 🔮️ Re-encodes with the registered `image` reference implementation, then restores the two
    /// parts of a JFIF file its writer cannot express: the JFIF version bytes (hard-coded to `1.2`
    /// in its `build_jfif_header`) and the APPn/COM segments (no API at all). Both are written back
    /// in T.871's own layout, immediately after the APP0 the crate itself produced.
    ///
    /// JPEG carries no alpha channel, so the reference encoder is given RGB — the same convention
    /// `raster::oracle_create_image` already uses for BMP and JPEG.
    pub fn encode(doc: &OracleDoc) -> Result<Vec<u8>, String> {
        let buffer = image::RgbaImage::from_raw(doc.width, doc.height, doc.rgba.clone()).ok_or("raster does not fill width * height * 4 bytes")?;
        let rgb = image::DynamicImage::ImageRgba8(buffer).to_rgb8();
        let mut out = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, doc.quality);
        encoder.set_pixel_density(image::codecs::jpeg::PixelDensity {
            density: (doc.x_density, doc.y_density),
            unit: match doc.density_units {
                1 => image::codecs::jpeg::PixelDensityUnit::Inches,
                2 => image::codecs::jpeg::PixelDensityUnit::Centimeters,
                _ => image::codecs::jpeg::PixelDensityUnit::PixelAspectRatio,
            },
        });
        encoder.write_image(rgb.as_raw(), doc.width, doc.height, image::ExtendedColorType::Rgb8).map_err(|error| format!("jpeg encode: {error}"))?;

        // 📐️ T.871 §B.2.4.6.2 fixes every offset used here: SOI is bytes 0..2, the APP0 marker
        // 2..4, its 2-byte length 4..6, and its payload begins at byte 6 as `"JFIF\0"` followed by
        // the two version bytes. The APP0 segment therefore ends at `4 + app0_length`, which is
        // where the retained segments go.
        const APP0_PAYLOAD: usize = 6;
        let app0_length = out.get(4).zip(out.get(5)).map(|(high, low)| ((*high as usize) << 8) | *low as usize).ok_or("reference encoder produced a stream with no APP0")?;
        if out.get(2..4) != Some(&[0xFF, 0xE0]) || out.len() < 4 + app0_length || out.get(APP0_PAYLOAD..APP0_PAYLOAD + 5) != Some(b"JFIF\0") {
            return Err("reference encoder no longer writes a JFIF APP0 immediately after SOI — the version patch and the segment splice both key on that position".to_string());
        }
        out[APP0_PAYLOAD + 5] = doc.version.0;
        out[APP0_PAYLOAD + 6] = doc.version.1;
        let mut spliced = Vec::with_capacity(out.len());
        spliced.extend_from_slice(&out[..4 + app0_length]);
        for (marker, payload) in &doc.other_segments {
            let length = payload.len() + 2;
            if length > 0xFFFF {
                return Err(format!("segment 0x{marker:02x} carries {} bytes, past the 65533 a JPEG marker segment can hold", payload.len()));
            }
            spliced.extend_from_slice(&[0xFF, *marker, (length >> 8) as u8, (length & 0xFF) as u8]);
            spliced.extend_from_slice(payload);
        }
        spliced.extend_from_slice(&out[4 + app0_length..]);
        Ok(spliced)
    }
    //#endregion 🔖️Codec

    //#region 🔖️Apply
    fn solid_fill(width: u32, height: u32, fill: [u8; 4]) -> Vec<u8> {
        fill.iter().copied().cycle().take((width as usize) * (height as usize) * 4).collect()
    }

    /// 🦠️ One `match` arm per `JpgMutation` variant, reimplemented independently against
    /// [`OracleDoc`] rather than calling into the subject's own `apply_jpg_mutation`. The five
    /// table/restart kinds are accepted and change nothing, because nothing they touch survives
    /// either encoder — the module docstring carries the proof, and the adapter's exemption list
    /// carries the claim.
    fn apply_kind(doc: &mut OracleDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "change-jfif-header" => {
                let version = params.array("version");
                let component = |index: usize, fallback: u8| {
                    version
                        .get(index)
                        .and_then(|found| match found {
                            Json::Number(value) => Some(*value as u8),
                            _ => None,
                        })
                        .unwrap_or(fallback)
                };
                doc.version = (component(0, doc.version.0), component(1, doc.version.1));
                doc.density_units = match text(params, "densityUnits").as_str() {
                    "pixels-per-inch" => 1,
                    "pixels-per-cm" => 2,
                    _ => 0,
                };
                doc.x_density = number(params, "xDensity", doc.x_density as f64) as u16;
                doc.y_density = number(params, "yDensity", doc.y_density as f64) as u16;
            }
            "replace-quant-table" | "remove-quant-table" | "replace-huffman-table" | "remove-huffman-table" | "change-restart-interval" => {}
            "insert-other-segment" => {
                let at = (number(params, "index", 0.0).max(0.0) as usize).min(doc.other_segments.len());
                doc.other_segments.insert(at, (number(params, "marker", 226.0) as u8, hex_decode(&text(params, "data"))?));
            }
            "remove-other-segment" => {
                let at = number(params, "index", 0.0).max(0.0) as usize;
                if at < doc.other_segments.len() {
                    doc.other_segments.remove(at);
                }
            }
            "replace-pixels" => doc.rgba = solid_fill(doc.width, doc.height, fill_of(params)),
            "change-re-encode-quality" => doc.quality = number(params, "quality", DEFAULT_QUALITY as f64).clamp(1.0, 100.0) as u8,
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Apply

    //#region 🔖️Dispatch
    /// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
    /// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
    /// reports as a passing test.
    pub fn apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let mut doc = decode(input)?;
        apply_kind(&mut doc, &kind, &params)?;
        encode(&doc)
    }

    /// ↩️ Applies the INDEPENDENTLY computed inverse of `spec` on top of `mutated`. `JpgMutation`'s
    /// own algebraic law (`../🧬️schema/🧬️mutations/🦀️component.rs`) is "restore `base`'s own value
    /// for the facet this kind replaced"; each arm below is that rule reimplemented here against
    /// [`OracleDoc`], never that function called.
    pub fn undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let original = decode(original_input)?;
        let mut doc = decode(mutated)?;
        match kind.as_str() {
            "replace-quant-table" | "remove-quant-table" | "replace-huffman-table" | "remove-huffman-table" | "change-restart-interval" => {}
            "change-jfif-header" => {
                doc.version = original.version;
                doc.density_units = original.density_units;
                doc.x_density = original.x_density;
                doc.y_density = original.y_density;
            }
            "insert-other-segment" => {
                let at = (number(&params, "index", 0.0).max(0.0) as usize).min(original.other_segments.len());
                if at < doc.other_segments.len() {
                    doc.other_segments.remove(at);
                }
            }
            "remove-other-segment" => doc.other_segments = original.other_segments,
            "replace-pixels" => {
                doc.rgba = original.rgba;
                doc.width = original.width;
                doc.height = original.height;
            }
            "change-re-encode-quality" => doc.quality = original.quality,
            other => return Err(format!("mutation kind {other:?} has no oracle inverse")),
        }
        encode(&doc)
    }

    /// 🔁️ The `@id-identity-round-trip` scenario's independent computation: a full decode of the
    /// real scan followed by a re-encode from the model alone, at the fixture's own quality.
    pub fn identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        encode(&decode(input)?)
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Projection
    /// #⃣️ FNV-1a, 64-bit, dependency-free — a compact exact fingerprint for a segment payload that
    /// can legitimately be tens of kilobytes (this fixture's own XMP packet is 31 KB).
    fn digest_hex(bytes: &[u8]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{hash:016x}")
    }

    /// 👁️ The surface every scenario compares through.
    ///
    /// ⚠️ Every EXACT claim here is spelled as a STRING, deliberately. `semantic-jpg-mutate-v1`
    /// declares a single absolute per-NUMBER slack of 400 000, sized for the lossy raster; the
    /// comparison engine applies it to every number in the projection, so a numeric member cannot
    /// carry an exact claim at all. Measured on this fixture, `width` as a number meant the real
    /// 2275x2560 scan and a 3x2 stub compared EQUAL (|2275-3| is inside the slack), and the JFIF
    /// version, density unit and both densities were equally unobservable. Strings are compared by
    /// equality and no tolerance touches them. Only `lumaHistogram` stays numeric, because it is
    /// the one member that genuinely needs the slack.
    ///
    /// `quantTables` is the DQT payload each side actually wrote, in zigzag order. It is a shared,
    /// encoder-independent witness of the re-encode quality: `image`'s `new_with_quality` and this
    /// subset's own `scale_quality` implement the SAME IJG mapping (`q < 50 ? 5000/q : 200 - 2q`,
    /// `clamp((base * scale + 50) / 100, 1, 255)`) over the SAME Annex K.1 base tables, and both
    /// emit them through the same §B.2.4.1 zigzag. Without it `change-re-encode-quality` was not
    /// observable at all: measured on this fixture, a pass through quality 50 moves at most 10 014
    /// pixels between luma buckets and quality 5 at most 55 570 — both far inside the slack.
    pub fn project(input: &[u8]) -> Result<Json, String> {
        let doc = decode(input)?;
        let mut buckets = [0u32; 8];
        for pixel in doc.rgba.chunks_exact(4) {
            let luma = (u32::from(pixel[0]) * 299 + u32::from(pixel[1]) * 587 + u32::from(pixel[2]) * 114) / 1000;
            buckets[(luma / 32).min(7) as usize] += 1;
        }
        let segments: Vec<Json> = doc.other_segments.iter().map(|(marker, payload)| Json::String(format!("{marker:02x}:{}:{}", payload.len(), digest_hex(payload)))).collect();
        let quant: Vec<Json> = scan_segments(input)?
            .iter()
            .filter(|(marker, _)| *marker == 0xDB)
            .flat_map(|(_, payload)| payload.chunks(65).filter(|table| table.len() == 65).map(|table| Json::String(format!("{:x}:{}", table[0], digest_hex(&table[1..])))).collect::<Vec<_>>())
            .collect();
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String("jpg".to_string())),
            ("dimensions".to_string(), Json::String(format!("{}x{}", doc.width, doc.height))),
            ("lossy".to_string(), Json::Bool(true)),
            ("jfifVersion".to_string(), Json::String(format!("{}.{}", doc.version.0, doc.version.1))),
            ("jfifDensity".to_string(), Json::String(format!("unit{}:{}x{}", doc.density_units, doc.x_density, doc.y_density))),
            ("otherSegments".to_string(), Json::Array(segments)),
            ("quantTables".to_string(), Json::Array(quant)),
            ("lumaHistogram".to_string(), Json::Array(buckets.iter().map(|count| Json::Number(*count as f64)).collect())),
        ]))
    }
    //#endregion 🔖️Projection

    //#region 🔖️Hex
    /// 🧾️ Re-exported for the case adapter's own `insert-other-segment` payload, so the feature's
    /// hex spelling has exactly one decoder on the oracle side.
    pub fn hex(bytes: &[u8]) -> String {
        hex_encode(bytes)
    }
    //#endregion 🔖️Hex
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::apply_mutation(input, spec)
}

#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    oracles::undo_mutation(original_input, spec, mutated)
}

/// 🔁️ The `@id-identity-round-trip` scenario's independent computation. @see
/// `oracles::identity_round_trip`.
#[cfg(feature = "oracles")]
pub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::identity_round_trip(input)
}

/// 👁️ Projects JFIF bytes (oracle or subject, either role) onto the shape every scenario compares
/// under `@comparison-semantic-jpg-mutate-v1`. @see `oracles::project`.
#[cfg(feature = "oracles")]
pub fn project_jpg_mutation(input: &[u8]) -> Result<Json, String> {
    oracles::project(input)
}

/// 🧾️ Hex spelling of a byte payload, so the case adapter and the feature agree on one encoding.
#[cfg(feature = "oracles")]
pub fn oracle_hex(bytes: &[u8]) -> String {
    oracles::hex(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_original_input: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_identity_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_jpg_mutation(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_hex(_bytes: &[u8]) -> String {
    String::new()
}
//#endregion 🔖️Dispatch
