//! 🔓️ The decode half of the video tier: the progressive MP4 [`write_avc_mp4`](super::write_avc_mp4) writes, back to RGBA8
//! pictures, first-party and dependency-free on every target.
//!
//! The decoder answers exactly the H.264 subset [`AvcPcmEncoder`](super::AvcPcmEncoder) writes — Constrained Baseline, CAVLC, one
//! slice per picture, `I_PCM` IDR pictures and all-`P_Skip` P pictures, deblocking off — and refuses every other stream with a typed
//! [`VideoDecodeError::Unsupported`] naming the feature it met. A general H.264 decoder (transform, prediction, CABAC, B pictures)
//! stays out of this tier: a platform decoder plugs in behind the host that owns it.
//! <https://www.itu.int/rec/T-REC-H.264> · <https://www.iso.org/standard/83102.html> (ISO/IEC 14496-12)

use std::sync::Arc;

/// 🚨️ Everything the decoder refuses; `code()` is the language-agnostic vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoDecodeError {
    Truncated,
    NotMp4,
    NoVideoTrack,
    Malformed { what: &'static str },
    Unsupported { what: &'static str },
    Cancelled,
}

impl VideoDecodeError {
    /// 🔤️ The refusal's stable code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Truncated => "truncated",
            Self::NotMp4 => "notMp4",
            Self::NoVideoTrack => "noVideoTrack",
            Self::Malformed { .. } => "malformed",
            Self::Unsupported { .. } => "unsupported",
            Self::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for VideoDecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => formatter.write_str("video data ends before its structure does"),
            Self::NotMp4 => formatter.write_str("video data is not an ISO base media file"),
            Self::NoVideoTrack => formatter.write_str("video file holds no H.264 video track"),
            Self::Malformed { what } => write!(formatter, "video data is malformed: {what}"),
            Self::Unsupported { what } => write!(formatter, "video stream uses {what}, which the first-party decoder does not decode"),
            Self::Cancelled => formatter.write_str("video decoding was cancelled"),
        }
    }
}

impl std::error::Error for VideoDecodeError {}

/// 🎞️ A decoded video: its distinct pictures once and, per frame, the picture it shows (a `P_Skip` repeat shows its reference).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedVideo {
    pub width: u32,
    pub height: u32,
    pub timescale: u32,
    pub frame_ticks: Vec<u32>,
    pub frame_picture: Vec<u32>,
    pub pictures: Vec<Arc<Vec<u8>>>,
}

impl DecodedVideo {
    /// 🔢️ Frames the video plays.
    pub fn frame_count(&self) -> usize {
        self.frame_picture.len()
    }

    /// ⏱️ Playing time in milliseconds, rounded half up.
    pub fn duration_milliseconds(&self) -> u64 {
        let ticks: u64 = self.frame_ticks.iter().map(|tick| u64::from(*tick)).sum();
        (ticks * 1000 + u64::from(self.timescale) / 2) / u64::from(self.timescale.max(1))
    }

    /// 🖼️ The straight RGBA8 rows of frame `frame`.
    pub fn frame_rgba(&self, frame: usize) -> Option<&[u8]> {
        self.pictures.get(*self.frame_picture.get(frame)? as usize).map(|picture| picture.as_slice())
    }

    /// 🎯️ The frame shown at `milliseconds`, clamped to the last frame.
    pub fn frame_at_milliseconds(&self, milliseconds: u64) -> usize {
        let target = milliseconds.saturating_mul(u64::from(self.timescale)) / 1000;
        let mut elapsed = 0u64;
        for (index, ticks) in self.frame_ticks.iter().enumerate() {
            elapsed += u64::from(*ticks);
            if target < elapsed {
                return index;
            }
        }
        self.frame_ticks.len().saturating_sub(1)
    }
}

struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> BitReader<'a> {
    fn bit(&mut self) -> Result<u32, VideoDecodeError> {
        let byte = *self.data.get(self.position / 8).ok_or(VideoDecodeError::Truncated)?;
        let bit = (byte >> (7 - self.position % 8)) & 1;
        self.position += 1;
        Ok(u32::from(bit))
    }

    fn bits(&mut self, count: u32) -> Result<u32, VideoDecodeError> {
        let mut value = 0;
        for _ in 0..count {
            value = (value << 1) | self.bit()?;
        }
        Ok(value)
    }

    fn ue(&mut self) -> Result<u32, VideoDecodeError> {
        let mut zeros = 0;
        while self.bit()? == 0 {
            zeros += 1;
            if zeros > 31 {
                return Err(VideoDecodeError::Malformed { what: "exp-Golomb code" });
            }
        }
        Ok(((1u64 << zeros) - 1 + u64::from(self.bits(zeros)?)) as u32)
    }

    fn se(&mut self) -> Result<i32, VideoDecodeError> {
        let value = self.ue()?;
        Ok(if value % 2 == 1 { value.div_ceil(2) as i32 } else { -((value / 2) as i32) })
    }

    fn align(&mut self) -> Result<(), VideoDecodeError> {
        while self.position % 8 != 0 {
            if self.bit()? != 0 {
                return Err(VideoDecodeError::Malformed { what: "pcm alignment bit" });
            }
        }
        Ok(())
    }

    fn bytes(&mut self, count: usize) -> Result<&'a [u8], VideoDecodeError> {
        let start = self.position / 8;
        let slice = self.data.get(start..start + count).ok_or(VideoDecodeError::Truncated)?;
        self.position += count * 8;
        Ok(slice)
    }
}

fn unescape(nal: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(nal.len());
    let mut zeros = 0;
    for &byte in nal {
        if zeros >= 2 && byte == 3 {
            zeros = 0;
            continue;
        }
        out.push(byte);
        zeros = if byte == 0 { zeros + 1 } else { 0 };
    }
    out
}

#[derive(Clone, Copy)]
struct Sequence {
    mb_width: u32,
    mb_height: u32,
    crop: [u32; 4],
    frame_num_bits: u32,
}

#[derive(Clone, Copy)]
struct Picture {
    deblocking_control: bool,
}

fn parse_sps(nal: &[u8]) -> Result<Sequence, VideoDecodeError> {
    let rbsp = unescape(nal.get(1..).ok_or(VideoDecodeError::Truncated)?);
    let mut bits = BitReader { data: &rbsp, position: 0 };
    let profile = bits.bits(8)?;
    bits.bits(16)?;
    if profile != 66 {
        return Err(VideoDecodeError::Unsupported { what: "a profile other than Baseline" });
    }
    bits.ue()?;
    let frame_num_bits = bits.ue()? + 4;
    if bits.ue()? != 2 {
        return Err(VideoDecodeError::Unsupported { what: "picture order count type other than 2" });
    }
    bits.ue()?;
    bits.bit()?;
    let mb_width = bits.ue()? + 1;
    let mb_height = bits.ue()? + 1;
    if bits.bit()? != 1 {
        return Err(VideoDecodeError::Unsupported { what: "interlaced coding" });
    }
    bits.bit()?;
    let mut crop = [0; 4];
    if bits.bit()? == 1 {
        for value in &mut crop {
            *value = bits.ue()?;
        }
    }
    if mb_width * 16 <= (crop[0] + crop[1]) * 2 || mb_height * 16 <= (crop[2] + crop[3]) * 2 {
        return Err(VideoDecodeError::Malformed { what: "frame cropping" });
    }
    Ok(Sequence { mb_width, mb_height, crop, frame_num_bits })
}

fn parse_pps(nal: &[u8]) -> Result<Picture, VideoDecodeError> {
    let rbsp = unescape(nal.get(1..).ok_or(VideoDecodeError::Truncated)?);
    let mut bits = BitReader { data: &rbsp, position: 0 };
    bits.ue()?;
    bits.ue()?;
    if bits.bit()? != 0 {
        return Err(VideoDecodeError::Unsupported { what: "CABAC entropy coding" });
    }
    bits.bit()?;
    if bits.ue()? != 0 {
        return Err(VideoDecodeError::Unsupported { what: "slice groups" });
    }
    bits.ue()?;
    bits.ue()?;
    if bits.bit()? != 0 || bits.bits(2)? != 0 {
        return Err(VideoDecodeError::Unsupported { what: "weighted prediction" });
    }
    bits.se()?;
    bits.se()?;
    bits.se()?;
    let deblocking_control = bits.bit()? == 1;
    bits.bit()?;
    if bits.bit()? != 0 {
        return Err(VideoDecodeError::Unsupported { what: "redundant pictures" });
    }
    Ok(Picture { deblocking_control })
}

fn convert(sequence: &Sequence, luma: &[u8], cb: &[u8], cr: &[u8]) -> (u32, u32, Vec<u8>) {
    let (stride, chroma_stride) = ((sequence.mb_width * 16) as usize, (sequence.mb_width * 8) as usize);
    let width = (sequence.mb_width * 16 - (sequence.crop[0] + sequence.crop[1]) * 2) as usize;
    let height = (sequence.mb_height * 16 - (sequence.crop[2] + sequence.crop[3]) * 2) as usize;
    let (left, top) = ((sequence.crop[0] * 2) as usize, (sequence.crop[2] * 2) as usize);
    let mut rgba = Vec::with_capacity(width * height * 4);
    for y in 0..height {
        for x in 0..width {
            let (px, py) = (x + left, y + top);
            let c = i32::from(luma[py * stride + px]) - 16;
            let d = i32::from(cb[(py / 2) * chroma_stride + px / 2]) - 128;
            let e = i32::from(cr[(py / 2) * chroma_stride + px / 2]) - 128;
            let channel = |value: i32| (value >> 8).clamp(0, 255) as u8;
            rgba.extend_from_slice(&[channel(298 * c + 409 * e + 128), channel(298 * c - 100 * d - 208 * e + 128), channel(298 * c + 516 * d + 128), 255]);
        }
    }
    (width as u32, height as u32, rgba)
}

fn decode_idr(sequence: &Sequence, picture: &Picture, nal: &[u8]) -> Result<(u32, u32, Vec<u8>), VideoDecodeError> {
    let rbsp = unescape(nal.get(1..).ok_or(VideoDecodeError::Truncated)?);
    let mut bits = BitReader { data: &rbsp, position: 0 };
    if bits.ue()? != 0 {
        return Err(VideoDecodeError::Unsupported { what: "more than one slice per picture" });
    }
    if bits.ue()? % 5 != 2 {
        return Err(VideoDecodeError::Unsupported { what: "an IDR slice that is not an I slice" });
    }
    bits.ue()?;
    bits.bits(sequence.frame_num_bits)?;
    bits.ue()?;
    bits.bits(2)?;
    bits.se()?;
    if picture.deblocking_control && bits.ue()? != 1 {
        return Err(VideoDecodeError::Unsupported { what: "the deblocking filter" });
    }
    let (mb_width, mb_height) = (sequence.mb_width as usize, sequence.mb_height as usize);
    let (stride, chroma_stride) = (mb_width * 16, mb_width * 8);
    let mut luma = vec![0u8; stride * mb_height * 16];
    let mut cb = vec![0u8; chroma_stride * mb_height * 8];
    let mut cr = vec![0u8; chroma_stride * mb_height * 8];
    for mb_y in 0..mb_height {
        for mb_x in 0..mb_width {
            if bits.ue()? != 25 {
                return Err(VideoDecodeError::Unsupported { what: "a macroblock type other than I_PCM" });
            }
            bits.align()?;
            let samples = bits.bytes(384)?;
            for row in 0..16 {
                let at = (mb_y * 16 + row) * stride + mb_x * 16;
                luma[at..at + 16].copy_from_slice(&samples[row * 16..row * 16 + 16]);
            }
            for row in 0..8 {
                let at = (mb_y * 8 + row) * chroma_stride + mb_x * 8;
                cb[at..at + 8].copy_from_slice(&samples[256 + row * 8..256 + row * 8 + 8]);
                cr[at..at + 8].copy_from_slice(&samples[320 + row * 8..320 + row * 8 + 8]);
            }
        }
    }
    Ok(convert(sequence, &luma, &cb, &cr))
}

fn check_p_skip(sequence: &Sequence, picture: &Picture, nal: &[u8]) -> Result<(), VideoDecodeError> {
    let rbsp = unescape(nal.get(1..).ok_or(VideoDecodeError::Truncated)?);
    let mut bits = BitReader { data: &rbsp, position: 0 };
    if bits.ue()? != 0 {
        return Err(VideoDecodeError::Unsupported { what: "more than one slice per picture" });
    }
    if bits.ue()? % 5 != 0 {
        return Err(VideoDecodeError::Unsupported { what: "a slice type other than P" });
    }
    bits.ue()?;
    bits.bits(sequence.frame_num_bits)?;
    if bits.bit()? != 0 || bits.bit()? != 0 || bits.bit()? != 0 {
        return Err(VideoDecodeError::Unsupported { what: "reference list or marking changes" });
    }
    bits.se()?;
    if picture.deblocking_control && bits.ue()? != 1 {
        return Err(VideoDecodeError::Unsupported { what: "the deblocking filter" });
    }
    if bits.ue()? != sequence.mb_width * sequence.mb_height {
        return Err(VideoDecodeError::Unsupported { what: "a P macroblock that is not P_Skip" });
    }
    Ok(())
}

struct Track {
    timescale: u32,
    sps: Vec<u8>,
    pps: Vec<u8>,
    length_size: usize,
    ticks: Vec<u32>,
    sizes: Vec<u32>,
    offsets: Vec<u64>,
}

fn be32(data: &[u8], at: usize) -> Result<u32, VideoDecodeError> {
    Ok(u32::from_be_bytes(data.get(at..at + 4).ok_or(VideoDecodeError::Truncated)?.try_into().map_err(|_| VideoDecodeError::Truncated)?))
}

fn boxes(data: &[u8]) -> Result<Vec<([u8; 4], &[u8])>, VideoDecodeError> {
    let mut out = Vec::new();
    let mut at = 0;
    while at + 8 <= data.len() {
        let mut size = u64::from(be32(data, at)?);
        let kind: [u8; 4] = data[at + 4..at + 8].try_into().map_err(|_| VideoDecodeError::Truncated)?;
        let mut header = 8;
        if size == 1 {
            size = u64::from_be_bytes(data.get(at + 8..at + 16).ok_or(VideoDecodeError::Truncated)?.try_into().map_err(|_| VideoDecodeError::Truncated)?);
            header = 16;
        } else if size == 0 {
            size = (data.len() - at) as u64;
        }
        if size < header as u64 || at as u64 + size > data.len() as u64 {
            return Err(VideoDecodeError::Truncated);
        }
        out.push((kind, &data[at + header..(at as u64 + size) as usize]));
        at += size as usize;
    }
    Ok(out)
}

fn child<'a>(parent: &'a [u8], kind: &[u8; 4]) -> Result<Option<&'a [u8]>, VideoDecodeError> {
    Ok(boxes(parent)?.into_iter().find(|(found, _)| found == kind).map(|(_, body)| body))
}

fn require<'a>(parent: &'a [u8], kind: &[u8; 4]) -> Result<&'a [u8], VideoDecodeError> {
    child(parent, kind)?.ok_or(VideoDecodeError::Malformed { what: "a required box is missing" })
}

fn parse_track(trak: &[u8], limit: usize) -> Result<Option<Track>, VideoDecodeError> {
    let mdia = require(trak, b"mdia")?;
    let hdlr = require(mdia, b"hdlr")?;
    if hdlr.get(8..12) != Some(b"vide") {
        return Ok(None);
    }
    let timescale = be32(require(mdia, b"mdhd")?, if require(mdia, b"mdhd")?.first() == Some(&1) { 20 } else { 12 })?;
    let stbl = require(require(mdia, b"minf")?, b"stbl")?;
    let stsd = require(stbl, b"stsd")?;
    let entry = boxes(stsd.get(8..).ok_or(VideoDecodeError::Truncated)?)?.into_iter().next().ok_or(VideoDecodeError::NoVideoTrack)?;
    if &entry.0 != b"avc1" {
        return Err(VideoDecodeError::Unsupported { what: "a sample entry other than avc1" });
    }
    let avcc = require(entry.1.get(78..).ok_or(VideoDecodeError::Truncated)?, b"avcC")?;
    let length_size = usize::from(*avcc.get(4).ok_or(VideoDecodeError::Truncated)? & 3) + 1;
    let sps_length = usize::from(u16::from_be_bytes(avcc.get(6..8).ok_or(VideoDecodeError::Truncated)?.try_into().map_err(|_| VideoDecodeError::Truncated)?));
    let sps = avcc.get(8..8 + sps_length).ok_or(VideoDecodeError::Truncated)?.to_vec();
    let pps_at = 8 + sps_length + 1;
    let pps_length = usize::from(u16::from_be_bytes(avcc.get(pps_at..pps_at + 2).ok_or(VideoDecodeError::Truncated)?.try_into().map_err(|_| VideoDecodeError::Truncated)?));
    let pps = avcc.get(pps_at + 2..pps_at + 2 + pps_length).ok_or(VideoDecodeError::Truncated)?.to_vec();
    let stts = require(stbl, b"stts")?;
    let mut ticks = Vec::new();
    for entry in 0..be32(stts, 4)? as usize {
        let (count, delta) = (be32(stts, 8 + entry * 8)?, be32(stts, 12 + entry * 8)?);
        if ticks.len() + count as usize > limit {
            return Err(VideoDecodeError::Malformed { what: "sample count" });
        }
        ticks.extend(std::iter::repeat_n(delta, count as usize));
    }
    let stsz = require(stbl, b"stsz")?;
    let uniform = be32(stsz, 4)?;
    let count = be32(stsz, 8)? as usize;
    if count > limit {
        return Err(VideoDecodeError::Malformed { what: "sample count" });
    }
    let sizes = (0..count).map(|index| if uniform != 0 { Ok(uniform) } else { be32(stsz, 12 + index * 4) }).collect::<Result<Vec<_>, _>>()?;
    let stsc = require(stbl, b"stsc")?;
    let runs = (0..be32(stsc, 4)? as usize).map(|index| Ok((be32(stsc, 8 + index * 12)?, be32(stsc, 12 + index * 12)?))).collect::<Result<Vec<_>, VideoDecodeError>>()?;
    let chunk_offsets: Vec<u64> = if let Some(stco) = child(stbl, b"stco")? {
        (0..be32(stco, 4)? as usize).map(|index| be32(stco, 8 + index * 4).map(u64::from)).collect::<Result<_, _>>()?
    } else {
        let co64 = require(stbl, b"co64")?;
        (0..be32(co64, 4)? as usize).map(|index| co64.get(8 + index * 8..16 + index * 8).ok_or(VideoDecodeError::Truncated).map(|bytes| u64::from_be_bytes(bytes.try_into().unwrap_or_default()))).collect::<Result<_, _>>()?
    };
    let mut offsets = Vec::with_capacity(count);
    for (chunk, start) in chunk_offsets.iter().enumerate() {
        let per_chunk = runs.iter().rev().find(|(first, _)| (*first as usize) <= chunk + 1).map_or(0, |(_, samples)| *samples as usize);
        let mut at = *start;
        for _ in 0..per_chunk {
            if offsets.len() == count {
                break;
            }
            offsets.push(at);
            at += u64::from(sizes[offsets.len() - 1]);
        }
    }
    if offsets.len() != count || ticks.len() != count {
        return Err(VideoDecodeError::Malformed { what: "sample tables disagree" });
    }
    Ok(Some(Track { timescale, sps, pps, length_size, ticks, sizes, offsets }))
}

/// 🔓️ Decodes the first H.264 track of a progressive MP4 written by this tier into RGBA8 pictures, reporting `(samples done,
/// samples total)` after every sample and stopping with [`VideoDecodeError::Cancelled`] as soon as `cancelled` answers true.
pub fn decode_avc_mp4(bytes: &[u8], progress: &mut dyn FnMut(u32, u32), cancelled: &dyn Fn() -> bool) -> Result<DecodedVideo, VideoDecodeError> {
    let top = boxes(bytes)?;
    if top.first().map(|(kind, _)| kind) != Some(b"ftyp") {
        return Err(VideoDecodeError::NotMp4);
    }
    let moov = top.iter().find(|(kind, _)| kind == b"moov").map(|(_, body)| *body).ok_or(VideoDecodeError::NotMp4)?;
    let mut track = None;
    for (kind, body) in boxes(moov)? {
        if &kind == b"trak" && track.is_none() {
            track = parse_track(body, bytes.len())?;
        }
    }
    let track = track.ok_or(VideoDecodeError::NoVideoTrack)?;
    let sequence = parse_sps(&track.sps)?;
    let picture = parse_pps(&track.pps)?;
    let total = track.sizes.len() as u32;
    let mut pictures: Vec<Arc<Vec<u8>>> = Vec::new();
    let mut frame_picture = Vec::with_capacity(total as usize);
    let mut size = (0, 0);
    for (index, (offset, length)) in track.offsets.iter().zip(&track.sizes).enumerate() {
        if cancelled() {
            return Err(VideoDecodeError::Cancelled);
        }
        let start = usize::try_from(*offset).map_err(|_| VideoDecodeError::Truncated)?;
        let mut sample = bytes.get(start..start + *length as usize).ok_or(VideoDecodeError::Truncated)?;
        while !sample.is_empty() {
            let nal_length = sample.get(..track.length_size).ok_or(VideoDecodeError::Truncated)?.iter().fold(0usize, |value, byte| (value << 8) | usize::from(*byte));
            let nal = sample.get(track.length_size..track.length_size + nal_length).ok_or(VideoDecodeError::Truncated)?;
            sample = &sample[track.length_size + nal_length..];
            match nal.first().map(|header| header & 31) {
                Some(5) => {
                    let (width, height, rgba) = decode_idr(&sequence, &picture, nal)?;
                    size = (width, height);
                    pictures.push(Arc::new(rgba));
                    frame_picture.push(pictures.len() as u32 - 1);
                }
                Some(1) => {
                    check_p_skip(&sequence, &picture, nal)?;
                    frame_picture.push(pictures.len().checked_sub(1).ok_or(VideoDecodeError::Malformed { what: "a P picture before any IDR picture" })? as u32);
                }
                Some(6 | 9 | 7 | 8) => {}
                _ => return Err(VideoDecodeError::Unsupported { what: "a NAL unit type beyond IDR, P, SPS and PPS" }),
            }
        }
        if frame_picture.len() != index + 1 {
            return Err(VideoDecodeError::Malformed { what: "a sample without exactly one picture" });
        }
        progress(index as u32 + 1, total);
    }
    Ok(DecodedVideo { width: size.0, height: size.1, timescale: track.timescale, frame_ticks: track.ticks, frame_picture, pictures })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
