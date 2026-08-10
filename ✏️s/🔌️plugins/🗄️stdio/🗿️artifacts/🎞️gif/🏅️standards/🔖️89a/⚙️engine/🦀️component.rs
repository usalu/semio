//! ⚙️ GifEngine (89a) — real GIF89a codec: multi-frame animation, Graphic Control Extension
//! (delay/transparency/disposal), NETSCAPE2.0 loop count. The byte-level LZW core, sub-block
//! packing, color-table I/O, RGBA quantization, and interlace de-row helpers are NOT duplicated
//! here — they're the exact same bytes as 87a's, so this reuses 87a's `pub` engine functions
//! directly (the same "cross-artifact reuse via pub engine APIs" shape zip uses for deflate; see
//! ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 ground rules).

use crate::artifacts::gif::standards::v87a::engine as codec;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::diff::GifDiff;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::GifMutation;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{
    GifDisposal, GifFrame, GifSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA,
};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::GifArtifact;

//#region Codec89a
/// 🔖️ Writes the NETSCAPE2.0 application extension (only extension that carries loop count) and
/// one Graphic Control Extension + Image Descriptor per frame. Every frame writes its own local
/// color table (no global color table) — real multi-frame GIFs commonly vary per-frame palettes,
/// confirmed against the `dancing.gif` fixture (54 frames, no GCT, `mincode=8` per-frame LCT).
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
    out.push(0); // no global color table -- every frame carries its own local table
    out.push(0); // background color index
    out.push(0); // pixel aspect ratio

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

    for (index, frame) in snap.frames.iter().enumerate() {
        if frame.width == 0 || frame.height == 0 {
            return Err(format!("gif89a: frame {index} has empty dimensions"));
        }
        if frame.width > 0xFFFF || frame.height > 0xFFFF || frame.left > 0xFFFF || frame.top > 0xFFFF {
            return Err(format!("gif89a: frame {index} dimensions/offset exceed u16"));
        }
        if frame.rgba.len() != (frame.width as usize) * (frame.height as usize) * 4 {
            return Err(format!("gif89a: frame {index} rgba length mismatch"));
        }
        if frame.left + frame.width > snap.width || frame.top + frame.height > snap.height {
            return Err(format!("gif89a: frame {index} region exceeds the logical screen"));
        }
        let (palette, indices, transparent_index) = codec::quantize_rgba(&frame.rgba)?;
        let gce_transparent = frame.transparent || transparent_index.is_some();

        out.push(0x21);
        out.push(0xF9);
        out.push(4);
        let packed = ((frame.disposal.to_bits() & 0x07) << 2) | ((frame.user_input as u8) << 1) | (gce_transparent as u8);
        out.push(packed);
        out.extend_from_slice(&frame.delay_cs.to_le_bytes());
        out.push(transparent_index.unwrap_or(0));
        out.push(0);

        out.push(0x2C);
        out.extend_from_slice(&(frame.left as u16).to_le_bytes());
        out.extend_from_slice(&(frame.top as u16).to_le_bytes());
        out.extend_from_slice(&(frame.width as u16).to_le_bytes());
        out.extend_from_slice(&(frame.height as u16).to_le_bytes());
        out.push(0x80 | codec::color_table_size_field(palette.len())); // local color table, not interlaced
        codec::write_color_table(&mut out, &palette);
        let min_code_size = codec::min_code_size_for(palette.len());
        out.push(min_code_size);
        out.extend_from_slice(&codec::pack_sub_blocks(&codec::lzw_encode(&indices, min_code_size)));
    }
    out.push(0x3B);
    Ok(out)
}

/// 🔖️ Every extension body (GCE, application, comment, plain text) is structurally just a
/// length-prefixed sub-block sequence after its introducer+label — `unpack_sub_blocks` handles
/// all of them uniformly; the label alone decides how the flat payload is interpreted below.
/// Comment and plain-text extensions (and unrecognized application extensions) are intentionally
/// unmodeled: read and discarded, never causing a decode failure — they carry no pixel data.
pub fn decode_gif(data: &[u8]) -> Result<GifSnapshot, String> {
    if data.len() < 13 || &data[0..6] != b"GIF89a" {
        return Err("not a GIF89a file (bad magic)".into());
    }
    let w = u16::from_le_bytes([data[6], data[7]]) as u32;
    let h = u16::from_le_bytes([data[8], data[9]]) as u32;
    let screen_packed = data[10];
    let mut pos = 13usize;
    let gct = if (screen_packed & 0x80) != 0 {
        Some(codec::read_color_table(data, &mut pos, screen_packed & 0x07)?)
    } else {
        None
    };

    let mut loop_count: Option<u16> = None;
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
                    0xFF => {
                        if body.len() >= 14 && &body[0..8] == b"NETSCAPE" && body[11] == 1 {
                            loop_count = Some(u16::from_le_bytes([body[12], body[13]]));
                        }
                    }
                    _ => {} // comment / plain text / unknown application extension: unmodeled by design
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
                let local = if (ipacked & 0x80) != 0 { Some(codec::read_color_table(data, &mut pos, ipacked & 0x07)?) } else { None };
                let palette = local.as_ref().or(gct.as_ref()).ok_or("gif89a: frame has no color table (neither global nor local)")?;
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
                let rgba = codec::indices_to_rgba(&indices, palette, if transparent_flag { Some(transparent_index) } else { None });
                frames.push(GifFrame {
                    left,
                    top,
                    width: iw,
                    height: ih,
                    rgba,
                    delay_cs,
                    disposal: GifDisposal::from_bits(disposal_bits),
                    transparent: transparent_flag,
                    user_input,
                });
            }
            0x3B => break,
            other => return Err(format!("gif89a: unexpected block introducer {other:#04x}")),
        }
    }
    if frames.is_empty() {
        return Err("gif89a: file has no frames".into());
    }
    Ok(GifSnapshot { schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(), width: w, height: h, loop_count, frames })
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
        GifFrame { left, top, width, height, rgba, delay_cs, disposal, transparent: transparent_corner, user_input: false }
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
        let snap = GifSnapshot { schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(), width: 4, height: 4, loop_count: None, frames: vec![] };
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
}
//#endregion Tests
