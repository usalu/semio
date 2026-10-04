//! 🔮️ Mutation oracle for this subset — every one of the 21 declared `GifMutation` (89a) kinds,
//! performed by the registered reference `gif` crate so the subject's own mutation has an
//! independent result to be compared against instead of being checked against its own reading.
//!
//! An OWNED `OSnapshot`/`OFrame` model mirrors the `gif` crate's own `Decoder`/`Frame`/`Encoder`
//! surface — width/height, global/local palettes, per-frame delay/disposal/transparency/user-input —
//! plus two things the high-level API cannot read or write at all: comment/application extension
//! blocks, and the background-color-index / pixel-aspect-ratio header scalars (`gif::Encoder`
//! hard-codes both to zero with no setter). Extensions are recovered by a minimal fixed-offset walk
//! of the GIF89a block grammar (header → optional GCT → extension/image/trailer blocks); the two
//! header scalars are patched directly into the encoder's fixed 13-byte screen descriptor after
//! `into_inner()`. Neither is "parsing GIF ourselves" in place of the reference library — the
//! reference library performs every frame's LZW encode/decode; this is filling the two narrow gaps
//! its public API leaves, the same way `write_raw_extension` fills the comment/extension gap.
//!
//! Two more reference-library quirks, found by the `@id-inverse`/`@id-identity-round-trip`
//! scenarios failing against the real 800x800/54-frame `💃️dancing.gif` fixture and confirmed against
//! a standalone reproduction before being fixed here (never by relaxing the projection or comparing
//! an implementation with itself):
//! - `gif::Encoder::new`'s `write_global_palette` unconditionally sets the GCT flag bit and writes a
//!   minimum 2-entry padding table even when handed an EMPTY palette — there is no public way to
//!   omit the table outright, only to shrink it. `encode` strips that phantom table (clears the flag
//!   bit, removes the 6 padding bytes) whenever this snapshot's own `global_palette` is `None` — the
//!   real fixture has none, and every reference-produced GIF was gaining one it never had.
//! - `gif::Decoder` always DE-INTERLACES on read and resets `Frame::interlaced` to `false`
//!   regardless of the source flag, and `gif::Encoder::write_frame` writes `frame.buffer` verbatim —
//!   it does not itself reorder rows to match the flag it writes. This module therefore reads the
//!   flag the FILE carries with `raster::gif_image_interlace_flags` (a fixed-grammar walk over the
//!   Image Descriptors, shared with the 87a subset) and re-interleaves the rows itself on encode
//!   with `raster::gif_reorder_rows`. Trusting `Frame::interlaced` instead is not merely lossy: it
//!   makes `set-frame-interlace` a mutation the projection can never see, because the round trip
//!   through this reader erases both the flag and the row permutation and lands back exactly where
//!   it started.
//!
//! The vocabulary is per SUBSET, not per artifact: 87a has no GCE/animation concept at all, so
//! nothing here is shared with the peer 87a subset's own oracle.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`GifMutation::KINDS`).

#[cfg(not(feature = "oracles"))]
use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
//#region 🔖️Available
mod imp {
    use semio_repo_test_host::Json;

    //#region 🔖️Snapshot
    /// 🎞️ One frame, field-for-field the same shape `gif::Frame` reads into and writes from.
    #[derive(Clone)]
    struct OFrame {
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        interlaced: bool,
        palette: Option<Vec<u8>>,
        indices: Vec<u8>,
        delay: u16,
        dispose: gif::DisposalMethod,
        transparent: Option<u8>,
        needs_user_input: bool,
    }

    /// 🧩️ An application extension other than NETSCAPE2.0 (modeled separately as `loop_count`).
    #[derive(Clone)]
    struct OAppExt {
        identifier: [u8; 8],
        auth_code: [u8; 3],
        data: Vec<u8>,
    }

    /// 🖼️ The whole document: logical screen + optional GCT + ordered frames + comments + extensions.
    #[derive(Clone)]
    struct OSnapshot {
        width: u16,
        height: u16,
        global_palette: Option<Vec<u8>>,
        bg_color_index: u8,
        aspect_ratio: u8,
        /// 🔁️ `None` = no NETSCAPE2.0 extension (plays once); `Some(0)` = loop forever; `Some(n)` =
        /// loop `n` additional times — mirrors the subset schema's own `GifSnapshot::loop_count`.
        loop_count: Option<u16>,
        frames: Vec<OFrame>,
        comments: Vec<String>,
        app_extensions: Vec<OAppExt>,
    }
    //#endregion 🔖️Snapshot

    //#region 🔖️JsonHelpers
    fn num_or(value: &Json, key: &str, default: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(n)) => *n,
            _ => default,
        }
    }
    fn opt_num(value: &Json, key: &str) -> Option<f64> {
        match value.get(key) {
            Some(Json::Number(n)) => Some(*n),
            _ => None,
        }
    }
    fn bool_or(value: &Json, key: &str, default: bool) -> bool {
        match value.get(key) {
            Some(Json::Bool(b)) => *b,
            _ => default,
        }
    }
    fn indices_from_json(value: &Json, key: &str) -> Option<Vec<u8>> {
        match value.get(key) {
            Some(Json::Array(items)) => Some(
                items
                    .iter()
                    .map(|item| match item {
                        Json::Number(n) => *n as u8,
                        _ => 0,
                    })
                    .collect(),
            ),
            _ => None,
        }
    }
    /// 🎨️ A `GifColorTable` wire value (`null` = no table) as `gif`'s flat RGB palette; an empty table is no table.
    /// `gif`'s writer never sets a table's sort flag, so a sorted table is refused rather than silently unsorted.
    fn color_table_from_json(value: Option<&Json>) -> Result<Option<Vec<u8>>, String> {
        match value {
            None | Some(Json::Null) => Ok(None),
            Some(table) if bool_or(table, "sorted", false) => Err("gif's writer cannot set a colour table's sort flag".to_string()),
            Some(table) => Ok(Some(table.array("colors").iter().flat_map(|color| ["r", "g", "b"].map(|channel| num_or(color, channel, 0.0) as u8)).collect::<Vec<u8>>()).filter(|palette| !palette.is_empty())),
        }
    }
    /// 🔢️ A fixed-width byte-array member of a wire value.
    fn fixed_bytes<const N: usize>(value: &Json, key: &str) -> Result<[u8; N], String> {
        indices_from_json(value, key).unwrap_or_default().try_into().map_err(|_| format!("`{key}` must carry exactly {N} bytes"))
    }
    fn disposal_to_str(dispose: gif::DisposalMethod) -> &'static str {
        match dispose {
            gif::DisposalMethod::Any => "unspecified",
            gif::DisposalMethod::Keep => "doNotDispose",
            gif::DisposalMethod::Background => "restoreToBackground",
            gif::DisposalMethod::Previous => "restoreToPrevious",
        }
    }
    fn disposal_from_str(value: &str) -> Result<gif::DisposalMethod, String> {
        match value {
            "unspecified" => Ok(gif::DisposalMethod::Any),
            "doNotDispose" => Ok(gif::DisposalMethod::Keep),
            "restoreToBackground" => Ok(gif::DisposalMethod::Background),
            "restoreToPrevious" => Ok(gif::DisposalMethod::Previous),
            other => Err(format!("{other:?} is no GIF disposal method")),
        }
    }
    //#endregion 🔖️JsonHelpers

    //#region 🔖️SnapshotJson
    /// 📸️ A `GifSnapshot` wire value (`set-snapshot`'s replacement document) as this oracle's own model.
    fn snapshot_from_json(value: &Json) -> Result<OSnapshot, String> {
        Ok(OSnapshot {
            width: num_or(value, "width", 0.0) as u16,
            height: num_or(value, "height", 0.0) as u16,
            global_palette: color_table_from_json(value.get("gct"))?,
            bg_color_index: num_or(value, "backgroundColorIndex", 0.0) as u8,
            aspect_ratio: num_or(value, "pixelAspectRatio", 0.0) as u8,
            loop_count: opt_num(value, "loopCount").map(|n| n as u16),
            frames: value.array("frames").iter().map(frame_from_json).collect::<Result<_, _>>()?,
            comments: value.array("comments").iter().map(|item| match item { Json::String(text) => Ok(text.clone()), other => Err(format!("a comment must be text, not {}", other.to_string())) }).collect::<Result<_, String>>()?,
            app_extensions: value.array("appExtensions").iter().map(app_ext_from_json).collect::<Result<_, _>>()?,
        })
    }
    /// 🎞️ A `GifFrame` wire value. `gif` writes no Plain Text Extension, so a frame carrying one is refused.
    fn frame_from_json(value: &Json) -> Result<OFrame, String> {
        if !matches!(value.get("plainText"), None | Some(Json::Null)) {
            return Err("gif's writer carries no Plain Text Extension".to_string());
        }
        Ok(OFrame {
            left: num_or(value, "left", 0.0) as u16,
            top: num_or(value, "top", 0.0) as u16,
            width: num_or(value, "width", 0.0) as u16,
            height: num_or(value, "height", 0.0) as u16,
            interlaced: bool_or(value, "interlace", false),
            palette: color_table_from_json(value.get("lct"))?,
            indices: indices_from_json(value, "indices").unwrap_or_default(),
            delay: num_or(value, "delayCs", 0.0) as u16,
            dispose: match value.get("disposal") { None => gif::DisposalMethod::Any, Some(_) => disposal_from_str(&value.str("disposal"))? },
            transparent: opt_num(value, "transparentIndex").map(|n| n as u8),
            needs_user_input: bool_or(value, "userInput", false),
        })
    }
    /// 🧩️ A `GifAppExtension` wire value: an 8-byte identifier, a 3-byte authentication code and the payload.
    fn app_ext_from_json(value: &Json) -> Result<OAppExt, String> {
        Ok(OAppExt { identifier: fixed_bytes(value, "identifier")?, auth_code: fixed_bytes(value, "authCode")?, data: indices_from_json(value, "data").unwrap_or_default() })
    }
    /// 🩹️ This oracle's own model as the `GifSnapshot` wire [`snapshot_from_json`] reads — the reading a `patch-snapshot`
    /// row's pointer operation addresses.
    fn snapshot_to_json(snap: &OSnapshot) -> Json {
        let bytes = |bytes: &[u8]| Json::Array(bytes.iter().map(|byte| Json::Number(f64::from(*byte))).collect());
        let table = |palette: &Option<Vec<u8>>| match palette {
            None => Json::Null,
            Some(palette) => Json::Object(vec![
                ("sorted".to_string(), Json::Bool(false)),
                ("colors".to_string(), Json::Array(palette.chunks(3).map(|rgb| Json::Object(["r", "g", "b"].iter().zip(rgb).map(|(channel, value)| (channel.to_string(), Json::Number(f64::from(*value)))).collect())).collect())),
            ]),
        };
        let optional = |value: Option<f64>| value.map_or(Json::Null, Json::Number);
        let frame = |frame: &OFrame| {
            Json::Object(vec![
                ("left".to_string(), Json::Number(f64::from(frame.left))),
                ("top".to_string(), Json::Number(f64::from(frame.top))),
                ("width".to_string(), Json::Number(f64::from(frame.width))),
                ("height".to_string(), Json::Number(f64::from(frame.height))),
                ("interlace".to_string(), Json::Bool(frame.interlaced)),
                ("lct".to_string(), table(&frame.palette)),
                ("indices".to_string(), bytes(&frame.indices)),
                ("delayCs".to_string(), Json::Number(f64::from(frame.delay))),
                ("disposal".to_string(), Json::String(disposal_to_str(frame.dispose).to_string())),
                ("transparentIndex".to_string(), optional(frame.transparent.map(f64::from))),
                ("userInput".to_string(), Json::Bool(frame.needs_user_input)),
            ])
        };
        let extension = |extension: &OAppExt| Json::Object(vec![("identifier".to_string(), bytes(&extension.identifier)), ("authCode".to_string(), bytes(&extension.auth_code)), ("data".to_string(), bytes(&extension.data))]);
        Json::Object(vec![
            ("schema".to_string(), Json::String("stdio.gif.89a".to_string())),
            ("width".to_string(), Json::Number(f64::from(snap.width))),
            ("height".to_string(), Json::Number(f64::from(snap.height))),
            ("gct".to_string(), table(&snap.global_palette)),
            ("backgroundColorIndex".to_string(), Json::Number(f64::from(snap.bg_color_index))),
            ("pixelAspectRatio".to_string(), Json::Number(f64::from(snap.aspect_ratio))),
            ("loopCount".to_string(), optional(snap.loop_count.map(f64::from))),
            ("frames".to_string(), Json::Array(snap.frames.iter().map(frame).collect())),
            ("comments".to_string(), Json::Array(snap.comments.iter().map(|comment| Json::String(comment.clone())).collect())),
            ("appExtensions".to_string(), Json::Array(snap.app_extensions.iter().map(extension).collect())),
        ])
    }
    //#endregion 🔖️SnapshotJson

    //#region 🔖️AuxBlockScan
    /// 🔍️ Fixed-offset/fixed-grammar walk for the three things `gif::Decoder` cannot read: comment
    /// extension text, non-NETSCAPE application extensions, and the pixel-aspect-ratio byte. Image
    /// blocks are skipped structurally (LCT size + LZW sub-blocks) without decoding their pixels —
    /// the reference decoder already did that in the caller's separate pass.
    fn scan_aux_blocks(data: &[u8]) -> Result<(Vec<String>, Vec<OAppExt>, u8), String> {
        if data.len() < 13 || &data[0..3] != b"GIF" {
            return Err("not a GIF89a byte stream".to_string());
        }
        let packed = data[10];
        let aspect_ratio = data[12];
        let mut i = 13usize;
        if packed & 0x80 != 0 {
            i += (2usize << (packed & 0x07)) * 3;
        }
        let mut comments = Vec::new();
        let mut app_extensions = Vec::new();
        while i < data.len() {
            match data[i] {
                0x21 => {
                    let label = *data.get(i + 1).ok_or("truncated GIF extension introducer")?;
                    let mut cursor = i + 2;
                    let mut payload = Vec::new();
                    loop {
                        let size = *data.get(cursor).ok_or("truncated GIF extension sub-block")? as usize;
                        cursor += 1;
                        if size == 0 {
                            break;
                        }
                        let end = cursor + size;
                        payload.extend_from_slice(data.get(cursor..end).ok_or("truncated GIF extension payload")?);
                        cursor = end;
                    }
                    match label {
                        0xFE => comments.push(String::from_utf8_lossy(&payload).into_owned()),
                        0xFF if payload.len() >= 11 => {
                            let identifier: [u8; 8] = payload[0..8].try_into().expect("8-byte slice");
                            if &identifier != b"NETSCAPE" {
                                let auth_code: [u8; 3] = payload[8..11].try_into().expect("3-byte slice");
                                app_extensions.push(OAppExt { identifier, auth_code, data: payload[11..].to_vec() });
                            }
                        }
                        _ => {}
                    }
                    i = cursor;
                }
                0x2C => {
                    let packed2 = *data.get(i + 9).ok_or("truncated GIF image descriptor")?;
                    let mut cursor = i + 10;
                    if packed2 & 0x80 != 0 {
                        cursor += (2usize << (packed2 & 0x07)) * 3;
                    }
                    cursor += 1; // LZW minimum code size
                    loop {
                        let size = *data.get(cursor).ok_or("truncated GIF image data")? as usize;
                        cursor += 1;
                        if size == 0 {
                            break;
                        }
                        cursor += size;
                    }
                    i = cursor;
                }
                0x3B => break,
                other => return Err(format!("unexpected GIF block introducer 0x{:02x}", other)),
            }
        }
        Ok((comments, app_extensions, aspect_ratio))
    }
    //#endregion 🔖️AuxBlockScan

    //#region 🔖️Codec
    /// 🔮️ Decodes with the registered reference `gif::Decoder` (frames, palettes, disposal, delay,
    /// transparency, user-input, background index, loop count) plus [`scan_aux_blocks`] for what the
    /// high-level API omits.
    fn decode(input: &[u8]) -> Result<OSnapshot, String> {
        let mut decoder = gif::DecodeOptions::new().read_info(input).map_err(|error| format!("independent reader could not parse the GIF: {}", error))?;
        let width = decoder.width();
        let height = decoder.height();
        let global_palette = decoder.global_palette().map(|p| p.to_vec());
        let bg_color_index = decoder.bg_color().unwrap_or(0) as u8;
        let loop_count = match decoder.repeat() {
            gif::Repeat::Infinite => Some(0u16),
            gif::Repeat::Finite(0) => None,
            gif::Repeat::Finite(n) => Some(n),
        };
        let stored_interlace = semio_s_plugin_stdio_raster_test_oracle::gif_image_interlace_flags(input)?;
        let mut frames = Vec::new();
        while let Some(frame) = decoder.read_next_frame().map_err(|error| format!("independent reader could not decode a GIF frame: {}", error))? {
            frames.push(OFrame {
                left: frame.left,
                top: frame.top,
                width: frame.width,
                height: frame.height,
                interlaced: stored_interlace.get(frames.len()).copied().unwrap_or(false),
                palette: frame.palette.clone(),
                indices: frame.buffer.to_vec(),
                delay: frame.delay,
                dispose: frame.dispose,
                transparent: frame.transparent,
                needs_user_input: frame.needs_user_input,
            });
        }
        let (comments, app_extensions, aspect_ratio) = scan_aux_blocks(input)?;
        Ok(OSnapshot { width, height, global_palette, bg_color_index, aspect_ratio, loop_count, frames, comments, app_extensions })
    }

    /// 🔮️ Re-serializes with the registered reference `gif::Encoder`. Comments/extensions are
    /// written up front via `write_raw_extension` (the encoder's own escape hatch for extensions it
    /// has no typed support for); the background-index/aspect-ratio bytes are patched into the
    /// fixed 13-byte screen descriptor afterward, since `Encoder::new` hard-codes both to zero.
    fn encode(snap: &OSnapshot) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        {
            let global: &[u8] = snap.global_palette.as_deref().unwrap_or(&[]);
            let mut encoder = gif::Encoder::new(&mut out, snap.width, snap.height, global).map_err(|error| format!("gif header: {}", error))?;
            let repeat = match snap.loop_count {
                None => gif::Repeat::Finite(0),
                Some(0) => gif::Repeat::Infinite,
                Some(n) => gif::Repeat::Finite(n),
            };
            encoder.set_repeat(repeat).map_err(|error| format!("gif loop extension: {}", error))?;
            for comment in &snap.comments {
                encoder.write_raw_extension(gif::AnyExtension(0xFE), &[comment.as_bytes()]).map_err(|error| format!("gif comment extension: {}", error))?;
            }
            for extension in &snap.app_extensions {
                let mut header = [0u8; 11];
                header[..8].copy_from_slice(&extension.identifier);
                header[8..11].copy_from_slice(&extension.auth_code);
                encoder.write_raw_extension(gif::AnyExtension(0xFF), &[&header[..], &extension.data[..]]).map_err(|error| format!("gif application extension: {}", error))?;
            }
            for frame in &snap.frames {
                // 🔀️ `indices` is natural row order throughout this model; GIF stores an interlaced
                // image's rows in four passes and `write_frame` writes the buffer verbatim, so the
                // re-interleaving is the caller's job (see the module docstring).
                let stored = if frame.interlaced { semio_s_plugin_stdio_raster_test_oracle::gif_reorder_rows(&frame.indices, frame.width as usize, frame.height as usize, true) } else { frame.indices.clone() };
                let gif_frame = gif::Frame {
                    delay: frame.delay,
                    dispose: frame.dispose,
                    transparent: frame.transparent,
                    needs_user_input: frame.needs_user_input,
                    top: frame.top,
                    left: frame.left,
                    width: frame.width,
                    height: frame.height,
                    interlaced: frame.interlaced,
                    palette: frame.palette.clone(),
                    buffer: std::borrow::Cow::Borrowed(&stored),
                };
                encoder.write_frame(&gif_frame).map_err(|error| format!("gif frame: {}", error))?;
            }
        }
        if out.len() < 13 {
            return Err("gif encoder produced a truncated stream".to_string());
        }
        // 🩹️ `gif::Encoder::new` unconditionally sets the GCT flag bit and writes a minimum 2-entry
        // padding table even for an empty palette slice — `write_global_palette` has no way to omit
        // the table outright, only to shrink it. When this subset's own snapshot says "no GCT"
        // (`global_palette: None`, real of e.g. the 💃️dancing fixture), strip that phantom table
        // back out: clear the flag bit and remove the 6 padding bytes the encoder wrote regardless.
        if snap.global_palette.is_none() {
            if out.len() < 19 {
                return Err("gif encoder produced a truncated stream".to_string());
            }
            out.drain(13..19);
            out[10] &= !0x80;
        }
        out[11] = snap.bg_color_index;
        out[12] = snap.aspect_ratio;
        Ok(out)
    }
    //#endregion 🔖️Codec

    //#region 🔖️Projection
    /// 🔢️ FNV-1a over raw palette indices — a compact fingerprint for `SetFramePixels`/geometry
    /// mutations to change detectably without embedding a possibly-640000-entry sample array in the
    /// projection, following the shared raster oracle's "report what the format actually fixes"
    /// precedent (`../../../../../🔮️oracles/🖼️raster/🦀️.rs`).
    fn fnv1a(data: &[u8]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        format!("{:016x}", hash)
    }
    fn project_frame(frame: &OFrame) -> Json {
        Json::Object(vec![
            ("left".to_string(), Json::Number(frame.left as f64)),
            ("top".to_string(), Json::Number(frame.top as f64)),
            ("width".to_string(), Json::Number(frame.width as f64)),
            ("height".to_string(), Json::Number(frame.height as f64)),
            ("interlace".to_string(), Json::Bool(frame.interlaced)),
            ("paletteSize".to_string(), Json::Number(frame.palette.as_ref().map(|p| p.len() / 3).unwrap_or(0) as f64)),
            ("delayCs".to_string(), Json::Number(frame.delay as f64)),
            ("disposal".to_string(), Json::String(disposal_to_str(frame.dispose).to_string())),
            ("transparentIndex".to_string(), frame.transparent.map(|t| Json::Number(t as f64)).unwrap_or(Json::Null)),
            ("userInput".to_string(), Json::Bool(frame.needs_user_input)),
            ("indicesFingerprint".to_string(), Json::String(fnv1a(&frame.indices))),
        ])
    }
    fn project_snapshot(snap: &OSnapshot) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String("gif".to_string())),
            ("width".to_string(), Json::Number(snap.width as f64)),
            ("height".to_string(), Json::Number(snap.height as f64)),
            ("backgroundColorIndex".to_string(), Json::Number(snap.bg_color_index as f64)),
            ("aspectRatio".to_string(), Json::Number(snap.aspect_ratio as f64)),
            ("loopCount".to_string(), snap.loop_count.map(|n| Json::Number(n as f64)).unwrap_or(Json::Null)),
            ("globalPaletteSize".to_string(), Json::Number(snap.global_palette.as_ref().map(|p| p.len() / 3).unwrap_or(0) as f64)),
            ("frameCount".to_string(), Json::Number(snap.frames.len() as f64)),
            ("frames".to_string(), Json::Array(snap.frames.iter().map(project_frame).collect())),
            ("comments".to_string(), Json::Array(snap.comments.iter().cloned().map(Json::String).collect())),
            ("appExtensionCount".to_string(), Json::Number(snap.app_extensions.len() as f64)),
        ])
    }
    //#endregion 🔖️Projection

    //#region 🔖️RasterRules
    /// 🖼️ GIF89a's raster rules, checked independently over this model on the frames `kind` touched: a frame fits within
    /// the Logical Screen (§20), carries one index per pixel of its rectangle (§22), and indexes only entries of its
    /// active colour table — its own, else the global one (§19, §21, §22). `Some` names the broken rule; the subject
    /// refuses the same edits. <https://www.w3.org/Graphics/GIF/spec-gif89a.txt>
    fn raster_refusal(snap: &OSnapshot, kind: &str, params: &Json) -> Option<String> {
        let fits = |index: usize| {
            let frame = &snap.frames[index];
            (u32::from(frame.left) + u32::from(frame.width) > u32::from(snap.width) || u32::from(frame.top) + u32::from(frame.height) > u32::from(snap.height)).then(|| format!("frame {index} would not fit the {}x{} Logical Screen (§20)", snap.width, snap.height))
        };
        let covers = |index: usize| {
            let frame = &snap.frames[index];
            (frame.indices.len() != usize::from(frame.width) * usize::from(frame.height)).then(|| format!("frame {index} would declare {}x{} pixels over {} indices (§22)", frame.width, frame.height, frame.indices.len()))
        };
        let colored = |index: usize| {
            let frame = &snap.frames[index];
            let colors = frame.palette.as_ref().or(snap.global_palette.as_ref()).map_or(0, |table| table.len() / 3);
            frame.indices.iter().max().filter(|max| usize::from(**max) >= colors).map(|max| format!("frame {index} uses colour index {max}, past its {colors}-entry active colour table (§22)"))
        };
        let targeted = || Some(num_or(params, "index", 0.0) as usize).filter(|index| *index < snap.frames.len());
        match kind {
            "set-snapshot" | "patch-snapshot" => (0..snap.frames.len()).find_map(|index| fits(index).or_else(|| covers(index)).or_else(|| colored(index))),
            "set-screen-size" => (0..snap.frames.len()).find_map(fits),
            "set-global-color-table" => (0..snap.frames.len()).filter(|index| snap.frames[*index].palette.is_none()).find_map(colored),
            "insert-frame" => {
                let at = (num_or(params, "index", snap.frames.len() as f64) as usize).min(snap.frames.len().saturating_sub(1));
                fits(at).or_else(|| covers(at)).or_else(|| colored(at))
            }
            "set-frame-geometry" => targeted().and_then(|index| covers(index).or_else(|| fits(index))),
            "set-frame-pixels" => targeted().and_then(|index| covers(index).or_else(|| colored(index))),
            _ => None,
        }
    }
    //#endregion 🔖️RasterRules

    //#region 🔖️Apply
    /// 🦠️ Applies one of the 21 declared kinds in place. Out-of-range frame/comment/extension
    /// indices degrade gracefully to a no-op rather than erroring — the same documented behavior as
    /// the subject's own `GifMutation::diff` (`../../🧬️schema/🧬️mutations/🦀️.rs`), which
    /// this independent oracle deliberately mirrors rather than diverging from without reason.
    fn apply_kind(snap: &mut OSnapshot, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "set-snapshot" => *snap = snapshot_from_json(params.get("snapshot").ok_or("set-snapshot carries no snapshot")?)?,
            "patch-snapshot" => *snap = snapshot_from_json(&semio_repo_test_host::law::patched_snapshot(&snapshot_to_json(snap), params.get("patch").ok_or("patch-snapshot carries no patch")?)?)?,
            "set-screen-size" => {
                snap.width = num_or(params, "width", snap.width as f64) as u16;
                snap.height = num_or(params, "height", snap.height as f64) as u16;
            }
            "set-global-color-table" => snap.global_palette = color_table_from_json(params.get("gct"))?,
            "set-background-color-index" => snap.bg_color_index = num_or(params, "index", snap.bg_color_index as f64) as u8,
            "set-pixel-aspect-ratio" => snap.aspect_ratio = num_or(params, "ratio", snap.aspect_ratio as f64) as u8,
            "set-loop-count" => snap.loop_count = opt_num(params, "loopCount").map(|n| n as u16),
            "insert-frame" => {
                let frame = frame_from_json(params.get("frame").ok_or("insert-frame carries no frame")?)?;
                let at = (num_or(params, "index", snap.frames.len() as f64) as usize).min(snap.frames.len());
                snap.frames.insert(at, frame);
            }
            "remove-frame" => {
                let index = num_or(params, "index", 0.0) as usize;
                if index < snap.frames.len() {
                    snap.frames.remove(index);
                }
            }
            "move-frame" => {
                let from = num_or(params, "from", 0.0) as usize;
                if from < snap.frames.len() {
                    let frame = snap.frames.remove(from);
                    let at = (num_or(params, "to", 0.0) as usize).min(snap.frames.len());
                    snap.frames.insert(at, frame);
                }
            }
            "set-frame-geometry" => {
                let index = num_or(params, "index", 0.0) as usize;
                if let Some(frame) = snap.frames.get_mut(index) {
                    frame.left = num_or(params, "left", frame.left as f64) as u16;
                    frame.top = num_or(params, "top", frame.top as f64) as u16;
                    frame.width = num_or(params, "width", frame.width as f64) as u16;
                    frame.height = num_or(params, "height", frame.height as f64) as u16;
                }
            }
            "set-frame-pixels" => {
                let index = num_or(params, "index", 0.0) as usize;
                let indices = indices_from_json(params, "indices").ok_or("set-frame-pixels carries no indices")?;
                if let Some(frame) = snap.frames.get_mut(index) {
                    frame.indices = indices;
                }
            }
            "set-frame-interlace" => {
                let index = num_or(params, "index", 0.0) as usize;
                if let Some(frame) = snap.frames.get_mut(index) {
                    frame.interlaced = bool_or(params, "interlace", frame.interlaced);
                }
            }
            "set-frame-delay" => {
                let index = num_or(params, "index", 0.0) as usize;
                if let Some(frame) = snap.frames.get_mut(index) {
                    frame.delay = num_or(params, "delayCs", frame.delay as f64) as u16;
                }
            }
            "set-frame-disposal" => {
                let index = num_or(params, "index", 0.0) as usize;
                let dispose = disposal_from_str(&params.str("disposal"))?;
                if let Some(frame) = snap.frames.get_mut(index) {
                    frame.dispose = dispose;
                }
            }
            "set-frame-transparency" => {
                let index = num_or(params, "index", 0.0) as usize;
                if let Some(frame) = snap.frames.get_mut(index) {
                    frame.transparent = opt_num(params, "transparentIndex").map(|n| n as u8);
                }
            }
            "set-frame-user-input" => {
                let index = num_or(params, "index", 0.0) as usize;
                if let Some(frame) = snap.frames.get_mut(index) {
                    frame.needs_user_input = bool_or(params, "userInput", frame.needs_user_input);
                }
            }
            "insert-comment" => {
                let at = (num_or(params, "index", snap.comments.len() as f64) as usize).min(snap.comments.len());
                snap.comments.insert(at, params.str("text"));
            }
            "remove-comment" => {
                let index = num_or(params, "index", 0.0) as usize;
                if index < snap.comments.len() {
                    snap.comments.remove(index);
                }
            }
            "add-app-extension" => {
                let at = (num_or(params, "index", snap.app_extensions.len() as f64) as usize).min(snap.app_extensions.len());
                snap.app_extensions.insert(at, app_ext_from_json(params.get("extension").ok_or("add-app-extension carries no extension")?)?);
            }
            "remove-app-extension" => {
                let index = num_or(params, "index", 0.0) as usize;
                if index < snap.app_extensions.len() {
                    snap.app_extensions.remove(index);
                }
            }
            other => return Err(format!("mutation kind {:?} has no oracle implementation", other)),
        }
        raster_refusal(snap, kind, params).map_or(Ok(()), |rule| Err(format!("refused: {rule}")))
    }
    //#endregion 🔖️Apply

    //#region 🔖️Inverse
    /// ↩️ Restores on `snap` exactly the facet `kind`/`params` replaced, read back out of `original` — the
    /// subject's own `GifMutation::inverse` law ("put `base`'s own value back"), reimplemented over this model.
    /// An index-targeted kind whose target `original` does not hold changed nothing, and restores nothing.
    fn restore(original: &OSnapshot, kind: &str, params: &Json, snap: &mut OSnapshot) -> Result<(), String> {
        let index = num_or(params, "index", 0.0) as usize;
        let frame = |snap: &mut OSnapshot, restore: fn(&OFrame, &mut OFrame)| {
            if let (Some(source), Some(target)) = (original.frames.get(index), snap.frames.get_mut(index)) {
                restore(source, target);
            }
        };
        match kind {
            "set-snapshot" | "patch-snapshot" => *snap = original.clone(),
            "set-screen-size" => (snap.width, snap.height) = (original.width, original.height),
            "set-global-color-table" => snap.global_palette = original.global_palette.clone(),
            "set-background-color-index" => snap.bg_color_index = original.bg_color_index,
            "set-pixel-aspect-ratio" => snap.aspect_ratio = original.aspect_ratio,
            "set-loop-count" => snap.loop_count = original.loop_count,
            "insert-frame" => {
                let at = index.min(original.frames.len());
                if at < snap.frames.len() {
                    snap.frames.remove(at);
                }
            }
            "remove-frame" => {
                if let Some(removed) = original.frames.get(index) {
                    snap.frames.insert(index.min(snap.frames.len()), removed.clone());
                }
            }
            "move-frame" => {
                let from = num_or(params, "from", 0.0) as usize;
                if from < original.frames.len() {
                    let landed_at = (num_or(params, "to", 0.0) as usize).min(original.frames.len() - 1);
                    let moved = snap.frames.remove(landed_at);
                    snap.frames.insert(from, moved);
                }
            }
            "set-frame-geometry" => frame(snap, |source, target| (target.left, target.top, target.width, target.height, target.indices) = (source.left, source.top, source.width, source.height, source.indices.clone())),
            "set-frame-pixels" => frame(snap, |source, target| target.indices = source.indices.clone()),
            "set-frame-interlace" => frame(snap, |source, target| target.interlaced = source.interlaced),
            "set-frame-delay" => frame(snap, |source, target| target.delay = source.delay),
            "set-frame-disposal" => frame(snap, |source, target| target.dispose = source.dispose),
            "set-frame-transparency" => frame(snap, |source, target| target.transparent = source.transparent),
            "set-frame-user-input" => frame(snap, |source, target| target.needs_user_input = source.needs_user_input),
            "insert-comment" => {
                let at = index.min(original.comments.len());
                if at < snap.comments.len() {
                    snap.comments.remove(at);
                }
            }
            "remove-comment" => {
                if let Some(text) = original.comments.get(index) {
                    snap.comments.insert(index.min(snap.comments.len()), text.clone());
                }
            }
            "add-app-extension" => {
                let at = index.min(original.app_extensions.len());
                if at < snap.app_extensions.len() {
                    snap.app_extensions.remove(at);
                }
            }
            "remove-app-extension" => {
                if let Some(extension) = original.app_extensions.get(index) {
                    snap.app_extensions.insert(index.min(snap.app_extensions.len()), extension.clone());
                }
            }
            other => return Err(format!("mutation kind {other:?} has no oracle inverse")),
        }
        Ok(())
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Dispatch
    /// 🎬️ Prepares the input a kind needs its target to be present in. The real 💃️dancing fixture
    /// carries a genuine comment extension and a genuine NETSCAPE2.0 loop extension and nothing
    /// else — verified by walking its block chain — and the NETSCAPE one is deliberately NOT part of
    /// `app_extensions` (it IS the loop-count axis, modelled separately). So
    /// `remove-app-extension` has nothing to remove on the committed bytes: it is exercised on the
    /// real document after this same independent implementation has inserted its target, the same
    /// arrange step the OOXML conformance cases and the PNG case use for their own removal kinds.
    /// Every other kind reads the committed bytes untouched.
    ///
    /// The seeded extension is deliberately not the row's own params — seeding with what the row
    /// then removes would still be a real removal, but seeding with a NAMED, distinct target makes
    /// the arrange visible in the projection's before-state instead of hiding inside it.
    pub fn oracle_arrange(input: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
        if forward.str("kind") != "remove-app-extension" {
            return Ok(input.to_vec());
        }
        let bytes = |text: &[u8]| Json::Array(text.iter().map(|byte| Json::Number(f64::from(*byte))).collect());
        let seed = Json::Object(vec![
            ("index".to_string(), Json::Number(0.0)),
            ("extension".to_string(), Json::Object(vec![("identifier".to_string(), bytes(b"ARRANGE1")), ("authCode".to_string(), bytes(b"SED")), ("data".to_string(), bytes(&[1, 2, 3]))])),
        ]);
        let mut snap = decode(input)?;
        apply_kind(&mut snap, "add-app-extension", &seed)?;
        encode(&snap)
    }

    pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let mut snap = decode(input)?;
        apply_kind(&mut snap, &kind, &params)?;
        encode(&snap)
    }

    pub fn oracle_apply_mutation_inverse(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        let original = decode(original_input)?;
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let mut snap = decode(mutated)?;
        restore(&original, &spec.str("kind"), &params, &mut snap)?;
        encode(&snap)
    }

    /// 🔁️ The `@id-identity-round-trip` computation: a full `gif` decode re-encoded from this model alone.
    pub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        encode(&decode(input)?)
    }

    pub fn project(input: &[u8]) -> Result<Json, String> {
        Ok(project_snapshot(&decode(input)?))
    }
    //#endregion 🔖️Dispatch
}

#[cfg(feature = "oracles")]
pub use imp::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_arrange, oracle_identity_round_trip, project};
//#endregion 🔖️Available

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🎬️ The pre-state a kind needs to have something to act on. @see `imp::oracle_arrange`.
#[cfg(not(feature = "oracles"))]
pub fn oracle_arrange(_input: &[u8], _forward: &Json) -> Result<Vec<u8>, String> {
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
pub fn project(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Unavailable
