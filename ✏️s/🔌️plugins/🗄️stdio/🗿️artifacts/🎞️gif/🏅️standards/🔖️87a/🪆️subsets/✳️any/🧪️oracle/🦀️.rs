//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `raster` module rather than by copying it.
//!
//! GIF87a has no Graphic Control Extension, so this oracle never writes one on purpose — but the
//! `gif` crate's `Encoder::write_frame` always emits one regardless, and its header is always
//! literally `GIF89a`. Both are cosmetic here: the reference reader parses GIF87a and GIF89a
//! identically (`gif::Version::V87a`/`V89a` are recorded, never gated on), so this module patches
//! the magic byte for honesty and otherwise leaves the crate's own encoder output alone.
//!
//! Three Logical Screen Descriptor scalars (GIF87a §18) are written by `gif::Encoder` as hardcoded
//! constants and never from the model handed to it — `encoder.rs`'s own `write_screen_desc` emits
//! `b"GIF89a"`, then `0u8 // bg index` and `0u8 // aspect ratio`. The reference crate exposes no
//! setter for any of them. This module therefore reads the background-colour index through the
//! crate's own `Decoder::bg_color()` and the pixel-aspect-ratio byte at its fixed offset 12, and
//! patches both back into byte 11/12 of the encoder's own output — the same one-byte-patch
//! technique the 89a sibling already uses (`../../🔖️89a/🪆️subsets/✳️any/🦀️oracle.rs`
//! `out[12] = snap.aspect_ratio`) and the same one this module already used for the magic byte.
//! Without those patches `set-background-color-index` and `set-pixel-aspect-ratio` are accepted and
//! silently discarded, which reports as a passing scenario.
//!
//! A FOURTH constant of the same family is the Global Color Table itself: `Encoder::new` documents
//! that "if no global palette shall be used an empty slice may be supplied", and
//! `write_global_palette` then sets the Global Color Table Flag unconditionally and writes
//! `check_color_table`'s two-entry padding for an empty slice (gif 0.14.2 `src/encoder.rs:183-195`,
//! `303-311`). A document declaring no global table therefore came back carrying a phantom
//! two-colour one, so `set-snapshot {"gct": null}` was silently discarded the same way. `oracle_encode`
//! clears the flag and drops those six bytes; see its own comment for why the result stays conformant.
//!
//! `project` (below) is this subset's OWN projection rather than the shared `raster::project_gif`:
//! that one reports only screen geometry and per-frame rectangles plus an opaque-sample count, so
//! the GCT, the background index, the aspect ratio, the interlace flag and the raw index buffers —
//! five of this vocabulary's twelve kinds — would land outside the compared surface entirely.
//!
//! @see 🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`GifMutation`,
//! `KINDS`) this dispatcher's `match` arms mirror one-for-one, independently reimplemented against
//! the `gif` reference crate rather than calling into the subject's own codec.

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
mod live {
    use semio_repo_test_host::Json;
    use std::borrow::Cow;

    //#region 🔖️Doc
    /// 🧾️ This oracle's own, independent GIF87a document model — built and re-serialized purely
    /// through the `gif` reference crate, never through this repository's own `GifSnapshot`/
    /// `decode_gif`/`encode_gif`. `pixel_aspect_ratio` is carried here because GIF87a §18 puts it
    /// in the Logical Screen Descriptor as a real byte; the reference crate has no getter or setter
    /// for it, so this module reads and writes that byte at its fixed offset (see the module doc).
    struct OracleImage {
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        interlaced: bool,
        palette: Option<Vec<u8>>,
        indices: Vec<u8>,
    }

    struct OracleDoc {
        width: u16,
        height: u16,
        gct: Vec<u8>,
        background_color_index: u8,
        pixel_aspect_ratio: u8,
        images: Vec<OracleImage>,
    }
    //#endregion 🔖️Doc

    //#region 🔖️JsonBridge
    fn num(json: &Json, key: &str) -> Option<f64> {
        match json.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }

    fn bool_field(json: &Json, key: &str) -> Option<bool> {
        match json.get(key) {
            Some(Json::Bool(value)) => Some(*value),
            _ => None,
        }
    }

    fn palette_from_json(json: &Json) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        for color in json.array("colors") {
            out.push(num(&color, "r").ok_or("color table entry missing r")? as u8);
            out.push(num(&color, "g").ok_or("color table entry missing g")? as u8);
            out.push(num(&color, "b").ok_or("color table entry missing b")? as u8);
        }
        Ok(out)
    }

    fn palette_to_json(bytes: &[u8]) -> Json {
        Json::Object(vec![
            ("sorted".to_string(), Json::Bool(false)),
            ("colors".to_string(), Json::Array(bytes.chunks_exact(3).map(|c| Json::Object(vec![("r".to_string(), Json::Number(c[0] as f64)), ("g".to_string(), Json::Number(c[1] as f64)), ("b".to_string(), Json::Number(c[2] as f64))])).collect())),
        ])
    }

    fn image_from_json(json: &Json) -> Result<OracleImage, String> {
        let width = num(json, "width").ok_or("image missing width")? as u16;
        let height = num(json, "height").ok_or("image missing height")? as u16;
        let palette = match json.get("lct") {
            Some(Json::Null) | None => None,
            Some(lct) => Some(palette_from_json(lct)?),
        };
        let indices = json
            .array("indices")
            .iter()
            .map(|v| match v {
                Json::Number(n) => *n as u8,
                _ => 0,
            })
            .collect();
        Ok(OracleImage { left: num(json, "left").unwrap_or(0.0) as u16, top: num(json, "top").unwrap_or(0.0) as u16, width, height, interlaced: bool_field(json, "interlace").unwrap_or(false), palette, indices })
    }

    fn image_to_json(image: &OracleImage) -> Json {
        Json::Object(vec![
            ("left".to_string(), Json::Number(image.left as f64)),
            ("top".to_string(), Json::Number(image.top as f64)),
            ("width".to_string(), Json::Number(image.width as f64)),
            ("height".to_string(), Json::Number(image.height as f64)),
            ("interlace".to_string(), Json::Bool(image.interlaced)),
            (
                "lct".to_string(),
                match &image.palette {
                    Some(p) => palette_to_json(p),
                    None => Json::Null,
                },
            ),
            ("indices".to_string(), Json::Array(image.indices.iter().map(|b| Json::Number(*b as f64)).collect())),
        ])
    }

    fn doc_from_json(json: &Json) -> Result<OracleDoc, String> {
        let gct = match json.get("gct") {
            Some(Json::Null) | None => Vec::new(),
            Some(gct) => palette_from_json(gct)?,
        };
        let images = json.array("images").iter().map(image_from_json).collect::<Result<Vec<_>, _>>()?;
        Ok(OracleDoc {
            width: num(json, "width").ok_or("snapshot missing width")? as u16,
            height: num(json, "height").ok_or("snapshot missing height")? as u16,
            gct,
            background_color_index: num(json, "backgroundColorIndex").unwrap_or(0.0) as u8,
            pixel_aspect_ratio: num(json, "pixelAspectRatio").unwrap_or(0.0) as u8,
            images,
        })
    }

    fn doc_to_json(doc: &OracleDoc) -> Json {
        Json::Object(vec![
            ("width".to_string(), Json::Number(doc.width as f64)),
            ("height".to_string(), Json::Number(doc.height as f64)),
            ("gct".to_string(), if doc.gct.is_empty() { Json::Null } else { palette_to_json(&doc.gct) }),
            ("backgroundColorIndex".to_string(), Json::Number(doc.background_color_index as f64)),
            ("pixelAspectRatio".to_string(), Json::Number(doc.pixel_aspect_ratio as f64)),
            ("images".to_string(), Json::Array(doc.images.iter().map(image_to_json).collect())),
        ])
    }
    //#endregion 🔖️JsonBridge

    //#region 🔖️Codec
    /// 👁️ Reads real GIF87a bytes with the independent `gif` reader (`Version::V87a` accepted
    /// natively) — indexed colour output, so palettes/indices come back exactly as stored on disk.
    fn oracle_decode(bytes: &[u8]) -> Result<OracleDoc, String> {
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let mut decoder = options.read_info(bytes).map_err(|error| format!("independent reader could not parse the GIF87a: {}", error))?;
        let width = decoder.width();
        let height = decoder.height();
        let background_color_index = decoder.bg_color().unwrap_or(0) as u8;
        let pixel_aspect_ratio = *bytes.get(12).ok_or("truncated Logical Screen Descriptor: no pixel-aspect-ratio byte")?;
        let gct = decoder.global_palette().unwrap_or(&[]).to_vec();
        let stored_interlace = crate::raster::gif_image_interlace_flags(bytes)?;
        let mut images = Vec::new();
        while let Some(frame) = decoder.read_next_frame().map_err(|error| format!("independent reader could not decode a GIF87a image: {}", error))? {
            let interlaced = stored_interlace.get(images.len()).copied().unwrap_or(false);
            images.push(OracleImage { left: frame.left, top: frame.top, width: frame.width, height: frame.height, interlaced, palette: frame.palette.clone(), indices: frame.buffer.clone().into_owned() });
        }
        Ok(OracleDoc { width, height, gct, background_color_index, pixel_aspect_ratio, images })
    }

    /// 🖋️ Writes real bytes with the independent `gif` writer, then patches the three Logical
    /// Screen Descriptor bytes that writer emits as constants rather than from its input — the
    /// `GIF89a` magic's version digit (byte 4), the background-colour index (byte 11) and the
    /// pixel-aspect-ratio (byte 12). See the module docstring for the crate lines that prove it.
    fn oracle_encode(doc: &OracleDoc) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        {
            let mut encoder = gif::Encoder::new(&mut out, doc.width, doc.height, &doc.gct).map_err(|error| format!("independent writer failed on the header: {}", error))?;
            for image in &doc.images {
                // 🔀️ `indices` is always natural row order in this model (that is what the reference
                // decoder hands back); GIF stores an interlaced image's rows in four passes, and
                // `gif::Encoder` writes the buffer verbatim, so the reordering is the caller's.
                let stored = if image.interlaced { crate::raster::gif_reorder_rows(&image.indices, image.width as usize, image.height as usize, true) } else { image.indices.clone() };
                let frame = gif::Frame {
                    delay: 0,
                    dispose: gif::DisposalMethod::Any,
                    transparent: None,
                    needs_user_input: false,
                    top: image.top,
                    left: image.left,
                    width: image.width,
                    height: image.height,
                    interlaced: image.interlaced,
                    palette: image.palette.clone(),
                    buffer: Cow::Borrowed(&stored),
                };
                encoder.write_frame(&frame).map_err(|error| format!("independent writer failed on an image: {}", error))?;
            }
        }
        if out.len() < 13 {
            return Err("independent writer produced a stream shorter than a Logical Screen Descriptor".to_string());
        }
        out[4] = b'7';
        out[11] = doc.background_color_index;
        out[12] = doc.pixel_aspect_ratio;
        // 🎨 Fourth constant of the same kind (see the module docstring): `Encoder::new`'s own
        // doc comment says "if no global palette shall be used an empty slice may be supplied",
        // and `write_global_palette` (gif 0.14.2 `src/encoder.rs:183-195`) then sets
        // `flags |= 0b1000_0000` UNCONDITIONALLY and writes `check_color_table`'s padding — for an
        // empty slice, `flag_size(0) = 0`, so `2 << 0 = 2` all-zero entries. A document that
        // declares no Global Color Table therefore comes back out of the reference writer carrying
        // a phantom two-entry one, and `set-snapshot {"gct": null}` is silently discarded exactly
        // the way the background index and aspect ratio were before their patches. Undone here:
        // clear the Global Color Table Flag (GIF87a §18, bit 7 of the packed byte 10) and drop the
        // six table bytes that follow the Logical Screen Descriptor. Every image is guaranteed to
        // carry a Local Color Table in this branch — with an empty global palette the crate's own
        // `write_frame` refuses a frame without one (`EncodingFormatError::MissingColorPalette`) —
        // so the result is a conformant GIF, not one whose images resolve through a table that is
        // no longer there.
        if doc.gct.is_empty() {
            if out.len() < 19 {
                return Err("independent writer produced a stream too short to carry the phantom global color table".to_string());
            }
            out[10] &= 0b0111_1111;
            out.drain(13..19);
        }
        Ok(out)
    }
    //#endregion 🔖️Codec

    //#region 🔖️Apply
    /// 🦠️ One `match` arm per `GifMutation` variant (`../../🧬️schema/🧬️mutations/🦀️.rs`'s
    /// `KINDS`), reimplemented independently against `OracleDoc` rather than calling into the
    /// subject's own `apply_gif_mutation`.
    fn apply_kind(doc: &mut OracleDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => {}
            "set-snapshot" => *doc = doc_from_json(params.get("snapshot").ok_or("set-snapshot: missing snapshot")?)?,
            "set-screen-size" => {
                doc.width = num(params, "width").ok_or("set-screen-size: missing width")? as u16;
                doc.height = num(params, "height").ok_or("set-screen-size: missing height")? as u16;
            }
            "set-global-color-table" => {
                doc.gct = match params.get("gct") {
                    Some(Json::Null) | None => Vec::new(),
                    Some(gct) => palette_from_json(gct)?,
                }
            }
            "set-background-color-index" => doc.background_color_index = num(params, "index").ok_or("set-background-color-index: missing index")? as u8,
            "set-pixel-aspect-ratio" => doc.pixel_aspect_ratio = num(params, "ratio").ok_or("set-pixel-aspect-ratio: missing ratio")? as u8,
            "insert-image" => {
                let index = num(params, "index").ok_or("insert-image: missing index")? as usize;
                let image = image_from_json(params.get("image").ok_or("insert-image: missing image")?)?;
                let at = index.min(doc.images.len());
                doc.images.insert(at, image);
            }
            "remove-image" => {
                let index = num(params, "index").ok_or("remove-image: missing index")? as usize;
                if index < doc.images.len() {
                    doc.images.remove(index);
                }
            }
            "move-image" => {
                let from = num(params, "from").ok_or("move-image: missing from")? as usize;
                let to = num(params, "to").ok_or("move-image: missing to")? as usize;
                if from < doc.images.len() {
                    let item = doc.images.remove(from);
                    let at = to.min(doc.images.len());
                    doc.images.insert(at, item);
                }
            }
            "set-image-geometry" => {
                let index = num(params, "index").ok_or("set-image-geometry: missing index")? as usize;
                if let Some(image) = doc.images.get_mut(index) {
                    image.left = num(params, "left").ok_or("set-image-geometry: missing left")? as u16;
                    image.top = num(params, "top").ok_or("set-image-geometry: missing top")? as u16;
                    image.width = num(params, "width").ok_or("set-image-geometry: missing width")? as u16;
                    image.height = num(params, "height").ok_or("set-image-geometry: missing height")? as u16;
                }
            }
            "set-image-pixels" => {
                let index = num(params, "index").ok_or("set-image-pixels: missing index")? as usize;
                if let Some(image) = doc.images.get_mut(index) {
                    image.indices = params
                        .array("indices")
                        .iter()
                        .map(|v| match v {
                            Json::Number(n) => *n as u8,
                            _ => 0,
                        })
                        .collect();
                }
            }
            "set-image-interlace" => {
                let index = num(params, "index").ok_or("set-image-interlace: missing index")? as usize;
                if let Some(image) = doc.images.get_mut(index) {
                    image.interlaced = bool_field(params, "interlace").ok_or("set-image-interlace: missing interlace")?;
                }
            }
            other => return Err(format!("mutation kind {:?} has no oracle implementation", other)),
        }
        Ok(())
    }
    //#endregion 🔖️Apply

    //#region 🔖️Dispatch
    pub fn apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let empty_params = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty_params);
        let mut doc = oracle_decode(input)?;
        apply_kind(&mut doc, &kind, params)?;
        oracle_encode(&doc)
    }

    /// ↩️ The real inverse of `kind`+`params`, computed from `original_bytes` (the PRE-mutation
    /// document) exactly as `GifMutation::inverse` does over `GifSnapshot`
    /// (`../../🧬️schema/🧬️mutations/🦀️.rs`) — reimplemented independently here against
    /// `OracleDoc`, never by calling that function. `insert-image`'s inverse is a `remove-image` at
    /// the same landed index, matching that function's own documented semantics.
    pub fn inverse_spec(original_bytes: &[u8], kind: &str, params: &Json) -> Result<Json, String> {
        let original = oracle_decode(original_bytes)?;
        let (inverse_kind, inverse_params) = match kind {
            "no-mutation" => ("no-mutation", Json::Object(Vec::new())),
            "set-snapshot" => ("set-snapshot", Json::Object(vec![("snapshot".to_string(), doc_to_json(&original))])),
            "set-screen-size" => ("set-screen-size", Json::Object(vec![("width".to_string(), Json::Number(original.width as f64)), ("height".to_string(), Json::Number(original.height as f64))])),
            "set-global-color-table" => ("set-global-color-table", Json::Object(vec![("gct".to_string(), if original.gct.is_empty() { Json::Null } else { palette_to_json(&original.gct) })])),
            "set-background-color-index" => ("set-background-color-index", Json::Object(vec![("index".to_string(), Json::Number(original.background_color_index as f64))])),
            "set-pixel-aspect-ratio" => ("set-pixel-aspect-ratio", Json::Object(vec![("ratio".to_string(), Json::Number(original.pixel_aspect_ratio as f64))])),
            "insert-image" => {
                let index = num(params, "index").ok_or("insert-image: missing index")? as usize;
                ("remove-image", Json::Object(vec![("index".to_string(), Json::Number(index.min(original.images.len()) as f64))]))
            }
            "remove-image" => {
                let index = num(params, "index").ok_or("remove-image: missing index")? as usize;
                match original.images.get(index) {
                    Some(image) => ("insert-image", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("image".to_string(), image_to_json(image))])),
                    None => ("no-mutation", Json::Object(Vec::new())),
                }
            }
            "move-image" => {
                let from = num(params, "from").ok_or("move-image: missing from")? as usize;
                let to = num(params, "to").ok_or("move-image: missing to")? as usize;
                let mut images = original.images;
                let landed_at = if from < images.len() {
                    let item = images.remove(from);
                    let at = to.min(images.len());
                    images.insert(at, item);
                    at
                } else {
                    from
                };
                ("move-image", Json::Object(vec![("from".to_string(), Json::Number(landed_at as f64)), ("to".to_string(), Json::Number(from as f64))]))
            }
            "set-image-geometry" => {
                let index = num(params, "index").ok_or("set-image-geometry: missing index")? as usize;
                match original.images.get(index) {
                    Some(image) => (
                        "set-image-geometry",
                        Json::Object(vec![
                            ("index".to_string(), Json::Number(index as f64)),
                            ("left".to_string(), Json::Number(image.left as f64)),
                            ("top".to_string(), Json::Number(image.top as f64)),
                            ("width".to_string(), Json::Number(image.width as f64)),
                            ("height".to_string(), Json::Number(image.height as f64)),
                        ]),
                    ),
                    None => ("no-mutation", Json::Object(Vec::new())),
                }
            }
            "set-image-pixels" => {
                let index = num(params, "index").ok_or("set-image-pixels: missing index")? as usize;
                match original.images.get(index) {
                    Some(image) => ("set-image-pixels", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("indices".to_string(), Json::Array(image.indices.iter().map(|b| Json::Number(*b as f64)).collect()))])),
                    None => ("no-mutation", Json::Object(Vec::new())),
                }
            }
            "set-image-interlace" => {
                let index = num(params, "index").ok_or("set-image-interlace: missing index")? as usize;
                match original.images.get(index) {
                    Some(image) => ("set-image-interlace", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("interlace".to_string(), Json::Bool(image.interlaced))])),
                    None => ("no-mutation", Json::Object(Vec::new())),
                }
            }
            other => return Err(format!("mutation kind {:?} has no oracle inverse", other)),
        };
        Ok(Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]))
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Projection
    /// #⃣️ FNV-1a, 64-bit, dependency-free — the same compact fingerprint the 89a sibling uses for
    /// raw palette indices, so a pixel or palette mutation moves the projection without embedding a
    /// per-sample array the comparison engine would have to diff element by element.
    fn digest_hex(bytes: &[u8]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{hash:016x}")
    }

    /// 👁️ The surface every scenario compares through, read back with the SAME independent `gif`
    /// reader `oracle_decode` uses. Every one of the twelve declared kinds moves at least one
    /// member of it: screen geometry (`set-screen-size`), the Global Color Table
    /// (`set-global-color-table`), the two Logical Screen Descriptor scalars
    /// (`set-background-color-index`, `set-pixel-aspect-ratio`), the image list's length and order
    /// (`insert-image`/`remove-image`/`move-image`), each image's rectangle
    /// (`set-image-geometry`), its interlace flag (`set-image-interlace`), its local table and its
    /// raw index buffer (`set-image-pixels`), and all of them at once (`set-snapshot`).
    ///
    /// Interlace is read back from the FILE (`raster::gif_image_interlace_flags`), never from
    /// `Frame::interlaced`: the reference decoder de-interlaces every image and then reports that
    /// field as `false` unconditionally, so trusting it would make `set-image-interlace` a mutation
    /// nothing can see. `indexDigest` is over natural-order indices on both sides, which is the
    /// point — flipping the flag must move the descriptor bit and leave the picture alone.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let doc = oracle_decode(bytes)?;
        let images: Vec<Json> = doc
            .images
            .iter()
            .map(|image| {
                Json::Object(vec![
                    ("left".to_string(), Json::Number(image.left as f64)),
                    ("top".to_string(), Json::Number(image.top as f64)),
                    ("width".to_string(), Json::Number(image.width as f64)),
                    ("height".to_string(), Json::Number(image.height as f64)),
                    ("interlaced".to_string(), Json::Bool(image.interlaced)),
                    ("lctColors".to_string(), Json::Number(image.palette.as_ref().map_or(0, |p| p.len() / 3) as f64)),
                    ("lctDigest".to_string(), Json::String(digest_hex(image.palette.as_deref().unwrap_or(&[])))),
                    ("indexDigest".to_string(), Json::String(digest_hex(&image.indices))),
                ])
            })
            .collect();
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String("gif87a".to_string())),
            ("width".to_string(), Json::Number(doc.width as f64)),
            ("height".to_string(), Json::Number(doc.height as f64)),
            ("gctColors".to_string(), Json::Number((doc.gct.len() / 3) as f64)),
            ("gctDigest".to_string(), Json::String(digest_hex(&doc.gct))),
            ("backgroundColorIndex".to_string(), Json::Number(doc.background_color_index as f64)),
            ("pixelAspectRatio".to_string(), Json::Number(doc.pixel_aspect_ratio as f64)),
            ("imageCount".to_string(), Json::Number(doc.images.len() as f64)),
            ("images".to_string(), Json::Array(images)),
        ]))
    }
    //#endregion 🔖️Projection
}

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    live::apply_mutation(input, spec)
}

/// ↩️ The independently-computed inverse of one mutation spec, relative to the document as it
/// stood BEFORE that mutation — used by the `inverse-<kind>` scenarios to restore the original
/// without calling into the subject's own `GifMutation::inverse`.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(original_bytes: &[u8], kind: &str, params: &Json) -> Result<Json, String> {
    live::inverse_spec(original_bytes, kind, params)
}

/// 👁️ Projects GIF87a bytes (oracle or subject, either role) onto the surface every
/// `mutate-<kind>`/`inverse-<kind>`/`identity-round-trip` scenario compares under
/// `@comparison-semantic-raster-v1`. @see `live::project`'s own doc comment for why this subset
/// carries its own projection instead of the shared `raster::project_gif`.
#[cfg(feature = "oracles")]
pub fn project_gif_87a(bytes: &[u8]) -> Result<Json, String> {
    live::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_original_bytes: &[u8], _kind: &str, _params: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_gif_87a(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
