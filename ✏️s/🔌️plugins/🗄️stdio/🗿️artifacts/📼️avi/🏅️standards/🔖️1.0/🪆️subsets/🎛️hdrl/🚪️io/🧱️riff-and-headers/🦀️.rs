//! 🧱️ RIFF container and AVI headers: the chunk iterator and writers, the real-bytes sniffer and the main, stream and format header parsers and writers. See [`super`].

use super::*;

//#region 🔖️Riff
struct RiffEntry<'a> {
    pub(super) fourcc: [u8; 4],
    pub(super) payload: &'a [u8],
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn iter_riff(data: &[u8]) -> impl Iterator<Item = Result<RiffEntry<'_>, String>> {
    struct It<'a> {
        data: &'a [u8],
        pos: usize,
    }
    impl<'a> Iterator for It<'a> {
        type Item = Result<RiffEntry<'a>, String>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos + 8 > self.data.len() {
                return None;
            }
            let fourcc: [u8; 4] = self.data[self.pos..self.pos + 4].try_into().unwrap();
            let size = u32::from_le_bytes(self.data[self.pos + 4..self.pos + 8].try_into().unwrap()) as usize;
            let payload_start = self.pos + 8;
            let Some(payload) = self.data.get(payload_start..payload_start + size) else { return Some(Err("avi: truncated chunk".into())) };
            let padded = size + (size % 2);
            self.pos = payload_start + padded;
            Some(Ok(RiffEntry { fourcc, payload }))
        }
    }
    It { data, pos: 0 }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn fourcc_str(f: &[u8; 4]) -> String {
    String::from_utf8_lossy(f).into_owned()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn write_chunk(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len() + 1);
    out.extend_from_slice(fourcc);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        out.push(0);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn write_list(list_type: &[u8; 4], children: &[u8]) -> Vec<u8> {
    let mut payload = list_type.to_vec();
    payload.extend_from_slice(children);
    write_chunk(b"LIST", &payload)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn fourcc4(s: &str) -> [u8; 4] {
    let mut out = [b' '; 4];
    for (i, b) in s.as_bytes().iter().take(4).enumerate() {
        out[i] = *b;
    }
    out
}

/// ✍️ One typed-raw [`RiffChunk`] re-serialized — a `LIST` of unknown type (tagged `"LIST:<type>"`
/// on decode) writes back as a `LIST`, everything else as a plain chunk. Shared by top-level
/// `unknown_chunks`, `hdrl_extra`, and every stream's `strl_extra` — the SAME typed-raw convention
/// one level down.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn write_riff_chunk(item: &RiffChunk) -> Vec<u8> {
    if let Some(list_type) = item.fourcc.strip_prefix("LIST:") {
        write_list(&fourcc4(list_type), &item.data)
    } else {
        write_chunk(&fourcc4(&item.fourcc), &item.data)
    }
}
//#endregion 🔖️Riff

//#region 🔖️Sniff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"AVI "
}
//#endregion 🔖️Sniff

//#region 🔖️Header
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_avih(payload: &[u8]) -> Result<AviMainHeader, String> {
    if payload.len() < 56 {
        return Err("avi: avih shorter than 56 bytes".into());
    }
    let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    Ok(AviMainHeader {
        micro_sec_per_frame: u32le(0),
        max_bytes_per_sec: u32le(4),
        padding_granularity: u32le(8),
        flags: u32le(12),
        total_frames: u32le(16),
        initial_frames: u32le(20),
        streams: u32le(24),
        suggested_buffer_size: u32le(28),
        width: u32le(32),
        height: u32le(36),
        reserved: vec![u32le(40), u32le(44), u32le(48), u32le(52)],
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn write_avih(h: &AviMainHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(56);
    for v in [h.micro_sec_per_frame, h.max_bytes_per_sec, h.padding_granularity, h.flags, h.total_frames, h.initial_frames, h.streams, h.suggested_buffer_size, h.width, h.height] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    let mut reserved = h.reserved.clone();
    reserved.resize(4, 0);
    for v in reserved {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

/// 📥️ Parses `strh` (AVISTREAMHEADER). The 13 fixed fields up to `dwSampleSize` are always 48
/// bytes; the trailing `rcFrame` is NOT fixed-width on the wire — real encoders (ffmpeg's own
/// AVI-1.0 muxer included) still write the classic pre-Win32 form where `rcFrame` is 4 16-bit
/// `SHORT`s (56 bytes total), not only the modern 4 `LONG`s form (64 bytes) most docs describe.
/// Confirmed against the real committed fixture: its own `strh` is 56 bytes, and bytes 48..56 read
/// as 4 `SHORT`s decode to `(0, 0, 480, 432)` — the video's real frame rectangle, not garbage — so
/// this is a genuine, common, spec-legal producer behaviour, not bytes simply omitted. Anything
/// shorter than 48 bytes (missing a required fixed field) is still rejected; anything at or beyond
/// 48 is accepted, defaulting `rcFrame` to zero only when truly absent (48..56 bytes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_strh(payload: &[u8]) -> Result<AviStreamHeader, String> {
    if payload.len() < 48 {
        return Err(format!("avi: strh is {} byte(s), need at least 48", payload.len()));
    }
    let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let i32le = |o: usize| i32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
    let i16le = |o: usize| i16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
    let (rc_frame_left, rc_frame_top, rc_frame_right, rc_frame_bottom, rc_frame_width) = if payload.len() >= 64 {
        (i32le(48), i32le(52), i32le(56), i32le(60), 16u8)
    } else if payload.len() >= 56 {
        (i16le(48) as i32, i16le(50) as i32, i16le(52) as i32, i16le(54) as i32, 8u8)
    } else {
        (0, 0, 0, 0, 0u8)
    };
    let strh_extra = if payload.len() > 64 { payload[64..].to_vec() } else { Vec::new() };
    Ok(AviStreamHeader {
        fcc_type: fourcc_str(&payload[0..4].try_into().unwrap()),
        fcc_handler: fourcc_str(&payload[4..8].try_into().unwrap()),
        flags: u32le(8),
        priority: u16le(12),
        language: u16le(14),
        initial_frames: u32le(16),
        scale: u32le(20),
        rate: u32le(24),
        start: u32le(28),
        length: u32le(32),
        suggested_buffer_size: u32le(36),
        quality: i32le(40),
        sample_size: u32le(44),
        rc_frame_left,
        rc_frame_top,
        rc_frame_right,
        rc_frame_bottom,
        rc_frame_width,
        strh_extra,
    })
}

/// ✍️ Re-serializes `strh` at whichever `rcFrame` width [`parse_strh`] recorded (`h.rc_frame_width`)
/// — 0 (omitted), 8 (4 `SHORT`s, classic 56-byte form) or 16 (4 `LONG`s, modern 64-byte form) —
/// rather than always promoting to 64 bytes, so a real 56-byte source round-trips byte-for-byte.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn write_strh(h: &AviStreamHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(64 + h.strh_extra.len());
    out.extend_from_slice(&fourcc4(&h.fcc_type));
    out.extend_from_slice(&fourcc4(&h.fcc_handler));
    out.extend_from_slice(&h.flags.to_le_bytes());
    out.extend_from_slice(&h.priority.to_le_bytes());
    out.extend_from_slice(&h.language.to_le_bytes());
    out.extend_from_slice(&h.initial_frames.to_le_bytes());
    out.extend_from_slice(&h.scale.to_le_bytes());
    out.extend_from_slice(&h.rate.to_le_bytes());
    out.extend_from_slice(&h.start.to_le_bytes());
    out.extend_from_slice(&h.length.to_le_bytes());
    out.extend_from_slice(&h.suggested_buffer_size.to_le_bytes());
    out.extend_from_slice(&h.quality.to_le_bytes());
    out.extend_from_slice(&h.sample_size.to_le_bytes());
    match h.rc_frame_width {
        16 => {
            for v in [h.rc_frame_left, h.rc_frame_top, h.rc_frame_right, h.rc_frame_bottom] {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        8 => {
            for v in [h.rc_frame_left, h.rc_frame_top, h.rc_frame_right, h.rc_frame_bottom] {
                out.extend_from_slice(&(v as i16).to_le_bytes());
            }
        }
        _ => {}
    }
    out.extend_from_slice(&h.strh_extra);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_strf(fcc_type: &str, payload: &[u8]) -> AviStreamFormat {
    if fcc_type == "vids" && payload.len() >= 40 {
        let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let i32le = |o: usize| i32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
        return AviStreamFormat::BitmapInfo {
            size: u32le(0),
            width: i32le(4),
            height: i32le(8),
            planes: u16le(12),
            bit_count: u16le(14),
            compression: fourcc_str(&payload[16..20].try_into().unwrap()),
            size_image: u32le(20),
            x_pels_per_meter: i32le(24),
            y_pels_per_meter: i32le(28),
            colors_used: u32le(32),
            colors_important: u32le(36),
        };
    }
    if fcc_type == "auds" && payload.len() >= 16 {
        let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
        return AviStreamFormat::WaveFormat {
            format_tag: u16le(0),
            channels: u16le(2),
            samples_per_sec: u32le(4),
            avg_bytes_per_sec: u32le(8),
            block_align: u16le(12),
            bits_per_sample: u16le(14),
            extra: payload.get(16..).map(|s| s.to_vec()).unwrap_or_default(),
        };
    }
    AviStreamFormat::Raw { data: payload.to_vec() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn write_strf(f: &AviStreamFormat) -> Vec<u8> {
    match f {
        AviStreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => {
            let mut out = Vec::with_capacity(40);
            out.extend_from_slice(&size.to_le_bytes());
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            out.extend_from_slice(&planes.to_le_bytes());
            out.extend_from_slice(&bit_count.to_le_bytes());
            out.extend_from_slice(&fourcc4(compression));
            out.extend_from_slice(&size_image.to_le_bytes());
            out.extend_from_slice(&x_pels_per_meter.to_le_bytes());
            out.extend_from_slice(&y_pels_per_meter.to_le_bytes());
            out.extend_from_slice(&colors_used.to_le_bytes());
            out.extend_from_slice(&colors_important.to_le_bytes());
            out
        }
        AviStreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, extra } => {
            let mut out = Vec::with_capacity(16 + extra.len());
            out.extend_from_slice(&format_tag.to_le_bytes());
            out.extend_from_slice(&channels.to_le_bytes());
            out.extend_from_slice(&samples_per_sec.to_le_bytes());
            out.extend_from_slice(&avg_bytes_per_sec.to_le_bytes());
            out.extend_from_slice(&block_align.to_le_bytes());
            out.extend_from_slice(&bits_per_sample.to_le_bytes());
            out.extend_from_slice(extra);
            out
        }
        AviStreamFormat::Raw { data } => data.clone(),
    }
}
//#endregion 🔖️Header
