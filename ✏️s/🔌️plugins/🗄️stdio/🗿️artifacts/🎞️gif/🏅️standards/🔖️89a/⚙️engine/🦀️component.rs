//! ⚙️ GifEngine (89a) — real GIF89a codec: multi-frame animation, Graphic Control Extension
//! (delay/transparency/disposal), NETSCAPE2.0 loop count. The byte-level LZW core, sub-block
//! packing, color-table I/O, RGBA quantization, and interlace de-row helpers are NOT duplicated
//! here — they're the exact same bytes as 87a's, so this reuses 87a's `pub` engine functions
//! directly (the same "cross-artifact reuse via pub engine APIs" shape zip uses for deflate; see
//! ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 ground rules).

use crate::artifacts::gif::standards::v87a::engine as codec;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::GifMutation;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{
    GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText, GifRgb, GifSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA,
};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::GifArtifact;

//#region ColorTableConv
/// 🔀️ 89a's OWN `GifColorTable`/`GifRgb` <-> the byte-level `Vec<Rgb>` ([u8;3]) shape 87a's
/// reused LZW/sub-block helpers speak — 89a cannot use 87a's `color_table_to_bytes`/
/// `color_table_from_bytes` directly since the two standards deliberately declare distinct
/// `GifColorTable` types (per the recipe's "no copy-pasted shared types" rule).
fn color_table_to_bytes(table: &GifColorTable) -> Vec<codec::Rgb> {
    table.colors.iter().map(|c| [c.r, c.g, c.b]).collect()
}
fn color_table_from_bytes(colors: Vec<codec::Rgb>, sorted: bool) -> GifColorTable {
    GifColorTable { sorted, colors: colors.into_iter().map(|[r, g, b]| GifRgb { r, g, b }).collect() }
}
//#endregion ColorTableConv

//#region Codec89a
/// 🔖️ Real, lossless GIF89a codec: GCT/LCT (palette indices, never decoded RGBA — the
/// lossless-payload exception), the NETSCAPE2.0 loop extension, one GCE + Image Descriptor per
/// real-image frame (or a GCE + Plain Text Extension for a plain-text-only "frame" per this
/// ticket's `GifFrame.plain_text` design — see the snapshot's doc comment), comment extensions,
/// and every other application extension verbatim. Documented normal form: comments are written
/// right after the screen descriptor/GCT, then the loop extension, then every other app
/// extension, then the frames in order — real files interleave these more freely, but
/// content-losslessness (not exact original byte position) is the contract here, matching the
/// recipe's "decode→encode byte-preserving up to documented normalizations."
pub fn encode_gif(snap: &GifSnapshot) -> Result<Vec<u8>, String> {
    if snap.width == 0 || snap.height == 0 {
        return Err("gif89a: empty logical screen".into());
    }
    if snap.width > 0xFFFF || snap.height > 0xFFFF {
        return Err("gif89a: logical screen dimensions exceed u16".into());
    }
    if snap.frames.is_empty() {
        return Err("gif89a: at least one frame is required".into());
    }

    let mut out = b"GIF89a".to_vec();
    out.extend_from_slice(&(snap.width as u16).to_le_bytes());
    out.extend_from_slice(&(snap.height as u16).to_le_bytes());
    let gct_bytes = snap.gct.as_ref().map(color_table_to_bytes);
    match &gct_bytes {
        Some(colors) => {
            let size_field = validated_color_table_size_field(colors.len(), "gif89a: global")?;
            let sorted = snap.gct.as_ref().map(|t| t.sorted).unwrap_or(false);
            out.push(0x80 | (sorted as u8) << 3 | size_field);
        }
        None => out.push(0),
    }
    out.push(snap.background_color_index);
    out.push(snap.pixel_aspect_ratio);
    if let Some(colors) = &gct_bytes {
        codec::write_color_table(&mut out, colors);
    }

    for comment in &snap.comments {
        out.push(0x21);
        out.push(0xFE);
        out.extend_from_slice(&codec::pack_sub_blocks(comment.as_bytes()));
    }

    if let Some(loop_count) = snap.loop_count {
        out.push(0x21);
        out.push(0xFF);
        out.push(11);
        out.extend_from_slice(b"NETSCAPE2.0");
        out.push(3);
        out.push(1);
        out.extend_from_slice(&loop_count.to_le_bytes());
        out.push(0);
    }

    for ext in &snap.app_extensions {
        out.push(0x21);
        out.push(0xFF);
        out.push(11);
        out.extend_from_slice(&ext.identifier);
        out.extend_from_slice(&ext.auth_code);
        out.extend_from_slice(&codec::pack_sub_blocks(&ext.data));
    }

    for (index, frame) in snap.frames.iter().enumerate() {
        let is_plain_text_only = frame.plain_text.is_some() && frame.width == 0 && frame.height == 0 && frame.indices.is_empty();
        let gce_needed = frame.delay_cs != 0 || frame.disposal != GifDisposal::default() || frame.transparent_index.is_some() || frame.user_input;

        if is_plain_text_only {
            if gce_needed {
                write_gce(&mut out, frame);
            }
            write_plain_text(&mut out, frame.plain_text.as_ref().expect("checked Some above"));
            continue;
        }
        if frame.plain_text.is_some() {
            return Err(format!("gif89a: frame {index} combines real image data with a plain-text extension, an unsupported combo (encode either as a plain-text-only frame or a real image frame)"));
        }
        if frame.width == 0 || frame.height == 0 {
            return Err(format!("gif89a: frame {index} has empty dimensions"));
        }
        if frame.width > 0xFFFF || frame.height > 0xFFFF || frame.left > 0xFFFF || frame.top > 0xFFFF {
            return Err(format!("gif89a: frame {index} dimensions/offset exceed u16"));
        }
        if frame.indices.len() != (frame.width as usize) * (frame.height as usize) {
            return Err(format!("gif89a: frame {index} indices length mismatch"));
        }
        if frame.left + frame.width > snap.width || frame.top + frame.height > snap.height {
            return Err(format!("gif89a: frame {index} region exceeds the logical screen"));
        }
        let table = frame.lct.as_ref().or(snap.gct.as_ref())
            .ok_or_else(|| format!("gif89a: frame {index} has no color table (neither local nor global)"))?;
        if frame.indices.iter().any(|&i| (i as usize) >= table.colors.len()) {
            return Err(format!("gif89a: frame {index} has an index past the end of its color table"));
        }

        write_gce(&mut out, frame);

        out.push(0x2C);
        out.extend_from_slice(&(frame.left as u16).to_le_bytes());
        out.extend_from_slice(&(frame.top as u16).to_le_bytes());
        out.extend_from_slice(&(frame.width as u16).to_le_bytes());
        out.extend_from_slice(&(frame.height as u16).to_le_bytes());
        let local_bytes = frame.lct.as_ref().map(color_table_to_bytes);
        let mut ipacked = (frame.interlace as u8) << 6;
        let min_code_size;
        if let Some(colors) = &local_bytes {
            let size_field = validated_color_table_size_field(colors.len(), &format!("gif89a: frame {index} local"))?;
            let sorted = frame.lct.as_ref().map(|t| t.sorted).unwrap_or(false);
            ipacked |= 0x80 | (sorted as u8) << 5 | size_field;
            min_code_size = codec::min_code_size_for(colors.len());
        } else {
            min_code_size = codec::min_code_size_for(table.colors.len());
        }
        out.push(ipacked);
        if let Some(colors) = &local_bytes {
            codec::write_color_table(&mut out, colors);
        }
        let on_disk_indices = if frame.interlace {
            codec::interlace_rows(&frame.indices, frame.width as usize, frame.height as usize)
        } else {
            frame.indices.clone()
        };
        out.push(min_code_size);
        out.extend_from_slice(&codec::pack_sub_blocks(&codec::lzw_encode(&on_disk_indices, min_code_size)));
    }
    out.push(0x3B);
    Ok(out)
}

/// 📐️ See 87a engine's `validated_color_table_size_field` doc comment — same honest-padding
/// rationale, duplicated here only because it wraps `codec::color_table_size_field` with 89a's own
/// error message prefix.
fn validated_color_table_size_field(len: usize, what: &str) -> Result<u8, String> {
    if len > 256 {
        return Err(format!("{what} color table length {len} exceeds the on-disk maximum of 256"));
    }
    Ok(codec::color_table_size_field(len))
}

fn write_gce(out: &mut Vec<u8>, frame: &GifFrame) {
    out.push(0x21);
    out.push(0xF9);
    out.push(4);
    let packed = ((frame.disposal.to_bits() & 0x07) << 2) | ((frame.user_input as u8) << 1) | (frame.transparent_index.is_some() as u8);
    out.push(packed);
    out.extend_from_slice(&frame.delay_cs.to_le_bytes());
    out.push(frame.transparent_index.unwrap_or(0));
    out.push(0);
}

fn write_plain_text(out: &mut Vec<u8>, pt: &GifPlainText) {
    out.push(0x21);
    out.push(0x01);
    let mut header = Vec::with_capacity(12);
    header.extend_from_slice(&(pt.left as u16).to_le_bytes());
    header.extend_from_slice(&(pt.top as u16).to_le_bytes());
    header.extend_from_slice(&(pt.width as u16).to_le_bytes());
    header.extend_from_slice(&(pt.height as u16).to_le_bytes());
    header.push(pt.cell_width);
    header.push(pt.cell_height);
    header.push(pt.fg_color_index);
    header.push(pt.bg_color_index);
    header.extend_from_slice(pt.text.as_bytes());
    out.extend_from_slice(&codec::pack_sub_blocks(&header));
}

/// 🔖️ Every extension body (GCE, application, comment, plain text) is structurally just a
/// length-prefixed sub-block sequence after its introducer+label — `unpack_sub_blocks` handles
/// all of them uniformly (concatenating every sub-block's payload flat); the label alone decides
/// how the flattened bytes are interpreted below.
pub fn decode_gif(data: &[u8]) -> Result<GifSnapshot, String> {
    if data.len() < 13 || &data[0..6] != b"GIF89a" {
        return Err("not a GIF89a file (bad magic)".into());
    }
    let w = u16::from_le_bytes([data[6], data[7]]) as u32;
    let h = u16::from_le_bytes([data[8], data[9]]) as u32;
    let screen_packed = data[10];
    let background_color_index = data[11];
    let pixel_aspect_ratio = data[12];
    let mut pos = 13usize;
    let gct = if (screen_packed & 0x80) != 0 {
        let sorted = (screen_packed & 0x08) != 0;
        Some(color_table_from_bytes(codec::read_color_table(data, &mut pos, screen_packed & 0x07)?, sorted))
    } else {
        None
    };

    let mut loop_count: Option<u16> = None;
    let mut comments: Vec<String> = Vec::new();
    let mut app_extensions: Vec<GifAppExtension> = Vec::new();
    let mut pending_gce: Option<(u8, bool, bool, u16, u8)> = None; // (disposal_bits, user_input, transparent_flag, delay_cs, transparent_index)
    let mut frames: Vec<GifFrame> = Vec::new();

    loop {
        let b = *data.get(pos).ok_or("truncated gif89a: missing trailer")?;
        match b {
            0x21 => {
                let label = *data.get(pos + 1).ok_or("truncated gif89a: extension introducer")?;
                pos += 2;
                let body = codec::unpack_sub_blocks(data, &mut pos)?;
                match label {
                    0xF9 => {
                        if body.len() < 4 {
                            return Err("gif89a: malformed graphic control extension".into());
                        }
                        let gp = body[0];
                        pending_gce = Some(((gp >> 2) & 0x07, (gp & 0x02) != 0, (gp & 0x01) != 0, u16::from_le_bytes([body[1], body[2]]), body[3]));
                    }
                    0xFE => {
                        comments.push(String::from_utf8_lossy(&body).into_owned());
                    }
                    0x01 => {
                        if body.len() < 12 {
                            return Err("gif89a: malformed plain text extension".into());
                        }
                        let plain_text = GifPlainText {
                            left: u16::from_le_bytes([body[0], body[1]]) as u32,
                            top: u16::from_le_bytes([body[2], body[3]]) as u32,
                            width: u16::from_le_bytes([body[4], body[5]]) as u32,
                            height: u16::from_le_bytes([body[6], body[7]]) as u32,
                            cell_width: body[8],
                            cell_height: body[9],
                            fg_color_index: body[10],
                            bg_color_index: body[11],
                            text: String::from_utf8_lossy(&body[12..]).into_owned(),
                        };
                        let (disposal_bits, user_input, transparent_flag, delay_cs, transparent_index) =
                            pending_gce.take().unwrap_or((0, false, false, 0, 0));
                        frames.push(GifFrame {
                            left: 0, top: 0, width: 0, height: 0,
                            interlace: false,
                            lct: None,
                            indices: Vec::new(),
                            delay_cs,
                            disposal: GifDisposal::from_bits(disposal_bits),
                            transparent_index: if transparent_flag { Some(transparent_index) } else { None },
                            user_input,
                            plain_text: Some(plain_text),
                        });
                    }
                    0xFF => {
                        if body.len() < 11 {
                            return Err("gif89a: malformed application extension".into());
                        }
                        if body.len() >= 14 && &body[0..8] == b"NETSCAPE" && &body[8..11] == b"2.0" && body[11] == 1 {
                            loop_count = Some(u16::from_le_bytes([body[12], body[13]]));
                        } else {
                            let mut identifier = [0u8; 8];
                            identifier.copy_from_slice(&body[0..8]);
                            let mut auth_code = [0u8; 3];
                            auth_code.copy_from_slice(&body[8..11]);
                            app_extensions.push(GifAppExtension { identifier, auth_code, data: body[11..].to_vec() });
                        }
                    }
                    _ => {} // unrecognized extension label: unmodeled by design (real spec labels are exhausted above)
                }
            }
            0x2C => {
                if pos + 10 > data.len() {
                    return Err("truncated gif89a image descriptor".into());
                }
                let left = u16::from_le_bytes([data[pos + 1], data[pos + 2]]) as u32;
                let top = u16::from_le_bytes([data[pos + 3], data[pos + 4]]) as u32;
                let iw = u16::from_le_bytes([data[pos + 5], data[pos + 6]]) as u32;
                let ih = u16::from_le_bytes([data[pos + 7], data[pos + 8]]) as u32;
                let ipacked = data[pos + 9];
                let interlaced = (ipacked & 0x40) != 0;
                pos += 10;
                let local = if (ipacked & 0x80) != 0 {
                    let sorted = (ipacked & 0x20) != 0;
                    Some(color_table_from_bytes(codec::read_color_table(data, &mut pos, ipacked & 0x07)?, sorted))
                } else {
                    None
                };
                if local.is_none() && gct.is_none() {
                    return Err("gif89a: frame has no color table (neither global nor local)".into());
                }
                let min_code_size = *data.get(pos).ok_or("truncated gif89a: missing lzw minimum code size")?;
                pos += 1;
                let sub = codec::unpack_sub_blocks(data, &mut pos)?;
                let mut indices = codec::lzw_decode(&sub, min_code_size)?;
                let expected = (iw as usize) * (ih as usize);
                if indices.len() < expected {
                    return Err("gif89a: lzw stream decoded fewer pixels than the frame needs".into());
                }
                indices.truncate(expected);
                if interlaced {
                    indices = codec::deinterlace_rows(&indices, iw as usize, ih as usize);
                }
                let (disposal_bits, user_input, transparent_flag, delay_cs, transparent_index) =
                    pending_gce.take().unwrap_or((0, false, false, 0, 0));
                frames.push(GifFrame {
                    left,
                    top,
                    width: iw,
                    height: ih,
                    interlace: interlaced,
                    lct: local,
                    indices,
                    delay_cs,
                    disposal: GifDisposal::from_bits(disposal_bits),
                    transparent_index: if transparent_flag { Some(transparent_index) } else { None },
                    user_input,
                    plain_text: None,
                });
            }
            0x3B => break,
            other => return Err(format!("gif89a: unexpected block introducer {other:#04x}")),
        }
    }
    if frames.is_empty() {
        return Err("gif89a: file has no frames".into());
    }
    Ok(GifSnapshot {
        schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width: w,
        height: h,
        gct,
        background_color_index,
        pixel_aspect_ratio,
        loop_count,
        frames,
        comments,
        app_extensions,
    })
}
//#endregion Codec89a

pub fn empty_gif_snapshot() -> GifSnapshot { GifSnapshot::default() }

//#region Register
/// 🗂️ Registers under `s.stdio.gif.89a`/`stdio.gif.89a` — deliberately DISTINCT ids from 87a's
/// `s.stdio.gif`/`stdio.gif`. `store::register_document_codec`/`::schema::register_artifact_schema_descriptor`
/// are both flat last-write-wins string-keyed registries pre-D4 (the plan's dialect-aware
/// two-level registry is future work); reusing 87a's ids here would silently overwrite its
/// registration instead of coexisting. Not currently wired into plugin bootstrap (out of this
/// ticket's scope) — 89a is reachable today via its own standard-scoped types directly and via
/// the artifact-level composer's dialect-keyed aggregation (`crate::artifacts::gif::composer`,
/// which already chains `standards::v89a::composer::entries()` regardless of whether this
/// function itself ever runs — composer entries are NOT registered here to avoid a redundant
/// second registration attempt).
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::gif::standards::v89a::subsets::any::schema::gif_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<GifSnapshot, GifMutation>(STDIO_GIF89A_DOCUMENT_SCHEMA));
}
//#endregion Register

//#region ArtifactEngine
pub struct GifEngine { artifact_state: GifArtifact, snapshot_state: GifSnapshot }
impl GifEngine {
    pub fn new(snapshot: GifSnapshot) -> Self {
        Self { artifact_state: GifArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
//#endregion ArtifactEngine

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 📐️ Pads a quantized palette to the on-disk power-of-two size `write_color_table` would pad
    /// it to anyway — so freshly-constructed test fixtures are already disk-canonical and an exact
    /// `decoded == snap` round-trip assertion is meaningful (a non-power-of-two-length table is a
    /// real, documented, one-way encode normalization — see `validated_color_table_size_field`'s
    /// doc comment — not something `decode(encode(x))` can ever undo for arbitrary `x`).
    fn pad_to_disk_size(mut colors: Vec<codec::Rgb>) -> Vec<codec::Rgb> {
        let size_field = codec::color_table_size_field(colors.len());
        let target = 1usize << (size_field as usize + 1);
        while colors.len() < target {
            colors.push([0, 0, 0]);
        }
        colors
    }

    /// 🧪️ Builds a real, lossless `GifFrame` (LCT + indices, no GCT) from a synthetic RGBA pattern
    /// via `quantize_rgba` — this test helper stays byte-level while the codec itself now writes
    /// whatever palette + indices are already in the snapshot, never re-quantizing.
    fn frame(left: u32, top: u32, width: u32, height: u32, base_color: [u8; 3], delay_cs: u16, disposal: GifDisposal, transparent_corner: bool) -> GifFrame {
        let mut rgba = vec![0u8; (width * height * 4) as usize];
        for y in 0..height {
            for x in 0..width {
                let o = ((y * width + x) * 4) as usize;
                let on = (x + y) % 3 == 0;
                if transparent_corner && x == 0 && y == 0 {
                    rgba[o..o + 4].copy_from_slice(&[0, 0, 0, 0]);
                    continue;
                }
                rgba[o] = if on { base_color[0] } else { base_color[0].wrapping_add(40) };
                rgba[o + 1] = if on { base_color[1] } else { base_color[1].wrapping_add(40) };
                rgba[o + 2] = if on { base_color[2] } else { base_color[2].wrapping_add(40) };
                rgba[o + 3] = 255;
            }
        }
        let (palette, indices, transparent_index) = codec::quantize_rgba(&rgba).expect("quantize");
        GifFrame {
            left, top, width, height,
            interlace: false,
            lct: Some(color_table_from_bytes(pad_to_disk_size(palette), false)),
            indices,
            delay_cs,
            disposal,
            transparent_index,
            user_input: false,
            plain_text: None,
        }
    }

    fn sample_snapshot() -> GifSnapshot {
        GifSnapshot {
            schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
            width: 12,
            height: 10,
            loop_count: Some(0),
            frames: vec![
                frame(0, 0, 12, 10, [200, 20, 20], 50, GifDisposal::DoNotDispose, false),
                frame(2, 1, 6, 5, [20, 200, 20], 8, GifDisposal::RestoreToBackground, true),
                frame(0, 0, 12, 10, [20, 20, 200], 8, GifDisposal::Unspecified, false),
            ],
            ..GifSnapshot::default()
        }
    }

    #[test]
    fn decode_gif_rejects_garbage_and_wrong_magic() {
        assert!(decode_gif(b"not a gif at all").is_err());
        assert!(decode_gif(b"GIF87a").is_err(), "89a decoder must reject 87a magic");
    }

    /// 🧪️ Multi-frame, multi-region, GCE (delay/disposal/transparency) + NETSCAPE loop round trip.
    #[test]
    fn encode_decode_round_trip_multiframe() {
        let snap = sample_snapshot();
        let bytes = encode_gif(&snap).expect("encode");
        assert_eq!(&bytes[0..6], b"GIF89a");
        let decoded = decode_gif(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🧪️ decode(encode(decode(x))) snapshot equality across frames, delays, disposal, loop count.
    #[test]
    fn encode_decode_encode_decode_is_stable() {
        let snap = sample_snapshot();
        let once = decode_gif(&encode_gif(&snap).unwrap()).unwrap();
        let twice = decode_gif(&encode_gif(&once).unwrap()).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn encode_gif_rejects_empty_frame_list() {
        let snap = GifSnapshot { schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(), width: 4, height: 4, loop_count: None, frames: vec![], ..GifSnapshot::default() };
        assert!(encode_gif(&snap).is_err());
    }

    #[test]
    fn encode_gif_rejects_frame_exceeding_logical_screen() {
        let mut snap = sample_snapshot();
        snap.frames[0].left = 100; // pushes the frame past the 12x10 logical screen
        assert!(encode_gif(&snap).is_err());
    }

    #[test]
    fn no_loop_extension_when_loop_count_is_none() {
        let mut snap = sample_snapshot();
        snap.loop_count = None;
        let bytes = encode_gif(&snap).expect("encode");
        // NETSCAPE2.0 must not appear anywhere in the stream when there's no loop count to encode.
        assert!(!bytes.windows(11).any(|w| w == b"NETSCAPE2.0"));
        let decoded = decode_gif(&bytes).expect("decode");
        assert_eq!(decoded.loop_count, None);
    }

    /// 🧪️ Comments, an unrecognized application extension, AND the NETSCAPE loop extension all
    /// round-trip losslessly and don't corrupt one another — the real spec-fidelity gain this
    /// rewrite delivers over the prior stub, which dropped everything but GCE/loop.
    #[test]
    fn comments_and_app_extensions_round_trip() {
        let mut snap = sample_snapshot();
        snap.comments = vec!["hello gif".into(), "second comment".into()];
        snap.app_extensions = vec![GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1, 2, 3, 4] }];
        let bytes = encode_gif(&snap).expect("encode");
        let decoded = decode_gif(&bytes).expect("decode");
        assert_eq!(decoded.comments, snap.comments);
        assert_eq!(decoded.app_extensions, snap.app_extensions);
        assert_eq!(decoded.loop_count, snap.loop_count);
        assert_eq!(decoded, snap);
    }

    /// 🧪️ A plain-text-only frame (no image data, `plain_text: Some`) round-trips as a real Plain
    /// Text Extension block, including its preceding GCE.
    #[test]
    fn plain_text_only_frame_round_trips() {
        let mut snap = sample_snapshot();
        snap.frames.push(GifFrame {
            left: 0, top: 0, width: 0, height: 0,
            interlace: false, lct: None, indices: Vec::new(),
            delay_cs: 100, disposal: GifDisposal::DoNotDispose, transparent_index: None, user_input: false,
            plain_text: Some(GifPlainText { left: 1, top: 1, width: 8, height: 2, cell_width: 4, cell_height: 8, fg_color_index: 0, bg_color_index: 1, text: "hi gif".into() }),
        });
        let bytes = encode_gif(&snap).expect("encode");
        let decoded = decode_gif(&bytes).expect("decode");
        assert_eq!(decoded, snap);
        assert_eq!(decoded.frames.last().unwrap().plain_text.as_ref().unwrap().text, "hi gif");
    }

    /// 🧪️ A frame combining real image data with a plain-text extension is a documented
    /// unsupported combo — must be a typed encode error, never silently drop one or the other.
    #[test]
    fn encode_gif_rejects_image_plus_plain_text_combo() {
        let mut snap = sample_snapshot();
        snap.frames[0].plain_text = Some(GifPlainText::default());
        assert!(encode_gif(&snap).is_err());
    }

    /// 🧪️ `interlace` is a real, round-trippable field — encode must reorder rows into the
    /// on-disk interlaced pass order, and decode must invert it back to natural-order indices.
    #[test]
    fn interlace_flag_round_trips_through_real_encode() {
        let mut snap = sample_snapshot();
        snap.frames[0].interlace = true;
        let original_indices = snap.frames[0].indices.clone();
        let bytes = encode_gif(&snap).expect("encode");
        let decoded = decode_gif(&bytes).expect("decode");
        assert!(decoded.frames[0].interlace);
        assert_eq!(decoded.frames[0].indices, original_indices);
    }

    /// 🧪️ An index referencing past the end of its color table is a typed encode error.
    #[test]
    fn encode_gif_rejects_index_past_color_table() {
        let mut snap = sample_snapshot();
        let len = snap.frames[0].indices.len();
        snap.frames[0].indices = vec![250u8; len];
        assert!(encode_gif(&snap).is_err());
    }

    /// 🧪️ `rgba()` derived accessor: a transparent index normalizes to `[0,0,0,0]`.
    #[test]
    fn rgba_derived_accessor_honors_transparent_index() {
        let snap = sample_snapshot();
        let transparent_frame = &snap.frames[1]; // built with transparent_corner=true
        assert!(transparent_frame.transparent_index.is_some());
        let rgba = transparent_frame.rgba(snap.gct.as_ref());
        assert_eq!(&rgba[0..4], &[0, 0, 0, 0], "top-left pixel must be the normalized-transparent color");
    }
}
//#endregion Tests
