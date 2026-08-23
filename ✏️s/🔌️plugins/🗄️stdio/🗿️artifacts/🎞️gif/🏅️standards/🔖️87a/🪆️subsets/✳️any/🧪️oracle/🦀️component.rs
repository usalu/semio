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
//! literally `GIF89a`. Both are cosmetic here: `raster::project_gif` (the shared, independent
//! reader every comparison in this case goes through) parses GIF87a and GIF89a identically
//! (`gif::Version::V87a`/`V89a` are recorded, never gated on), so this module patches the magic
//! byte for honesty and otherwise leaves the crate's own encoder output alone.
//!
//! @see 🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`GifMutation`,
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
    /// `decode_gif`/`encode_gif`. `pixel_aspect_ratio` has no field here: the crate's reader parses
    /// it into an internal buffer it never exposes a getter for, so `set-pixel-aspect-ratio` is
    /// honestly a no-op on this side (mirrors `raster::project_gif`'s own established precedent of
    /// canonicalizing away whatever the independent reader cannot observe).
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
        let indices = json.array("indices").iter().map(|v| match v { Json::Number(n) => *n as u8, _ => 0 }).collect();
        Ok(OracleImage { left: num(json, "left").unwrap_or(0.0) as u16, top: num(json, "top").unwrap_or(0.0) as u16, width, height, interlaced: bool_field(json, "interlace").unwrap_or(false), palette, indices })
    }

    fn image_to_json(image: &OracleImage) -> Json {
        Json::Object(vec![
            ("left".to_string(), Json::Number(image.left as f64)),
            ("top".to_string(), Json::Number(image.top as f64)),
            ("width".to_string(), Json::Number(image.width as f64)),
            ("height".to_string(), Json::Number(image.height as f64)),
            ("interlace".to_string(), Json::Bool(image.interlaced)),
            ("lct".to_string(), match &image.palette { Some(p) => palette_to_json(p), None => Json::Null }),
            ("indices".to_string(), Json::Array(image.indices.iter().map(|b| Json::Number(*b as f64)).collect())),
        ])
    }

    fn doc_from_json(json: &Json) -> Result<OracleDoc, String> {
        let gct = match json.get("gct") {
            Some(Json::Null) | None => Vec::new(),
            Some(gct) => palette_from_json(gct)?,
        };
        let images = json.array("images").iter().map(image_from_json).collect::<Result<Vec<_>, _>>()?;
        Ok(OracleDoc { width: num(json, "width").ok_or("snapshot missing width")? as u16, height: num(json, "height").ok_or("snapshot missing height")? as u16, gct, background_color_index: num(json, "backgroundColorIndex").unwrap_or(0.0) as u8, images })
    }

    fn doc_to_json(doc: &OracleDoc) -> Json {
        Json::Object(vec![
            ("width".to_string(), Json::Number(doc.width as f64)),
            ("height".to_string(), Json::Number(doc.height as f64)),
            ("gct".to_string(), if doc.gct.is_empty() { Json::Null } else { palette_to_json(&doc.gct) }),
            ("backgroundColorIndex".to_string(), Json::Number(doc.background_color_index as f64)),
            ("pixelAspectRatio".to_string(), Json::Number(0.0)),
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
        let gct = decoder.global_palette().unwrap_or(&[]).to_vec();
        let mut images = Vec::new();
        while let Some(frame) = decoder.read_next_frame().map_err(|error| format!("independent reader could not decode a GIF87a image: {}", error))? {
            images.push(OracleImage { left: frame.left, top: frame.top, width: frame.width, height: frame.height, interlaced: frame.interlaced, palette: frame.palette.clone(), indices: frame.buffer.clone().into_owned() });
        }
        Ok(OracleDoc { width, height, gct, background_color_index, images })
    }

    /// 🖋️ Writes real bytes with the independent `gif` writer, then patches the header magic from
    /// the crate's hardcoded `GIF89a` to `GIF87a` — cosmetic, see the module docstring.
    fn oracle_encode(doc: &OracleDoc) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        {
            let mut encoder = gif::Encoder::new(&mut out, doc.width, doc.height, &doc.gct).map_err(|error| format!("independent writer failed on the header: {}", error))?;
            for image in &doc.images {
                let frame = gif::Frame { delay: 0, dispose: gif::DisposalMethod::Any, transparent: None, needs_user_input: false, top: image.top, left: image.left, width: image.width, height: image.height, interlaced: image.interlaced, palette: image.palette.clone(), buffer: Cow::Borrowed(&image.indices) };
                encoder.write_frame(&frame).map_err(|error| format!("independent writer failed on an image: {}", error))?;
            }
        }
        if out.len() >= 6 {
            out[4] = b'7';
        }
        Ok(out)
    }
    //#endregion 🔖️Codec

    //#region 🔖️Apply
    /// 🦠️ One `match` arm per `GifMutation` variant (`../../🧬️schema/🧬️mutations/🦀️component.rs`'s
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
            "set-pixel-aspect-ratio" => {}
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
                    image.indices = params.array("indices").iter().map(|v| match v { Json::Number(n) => *n as u8, _ => 0 }).collect();
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
    /// (`../../🧬️schema/🧬️mutations/🦀️component.rs`) — reimplemented independently here against
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
            "set-pixel-aspect-ratio" => ("set-pixel-aspect-ratio", Json::Object(vec![("ratio".to_string(), Json::Number(0.0))])),
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
                    Some(image) => ("set-image-geometry", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("left".to_string(), Json::Number(image.left as f64)), ("top".to_string(), Json::Number(image.top as f64)), ("width".to_string(), Json::Number(image.width as f64)), ("height".to_string(), Json::Number(image.height as f64))])),
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

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_original_bytes: &[u8], _kind: &str, _params: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
