//! 🎞️ Baseline H.264: bitstream and RBSP reading, SPS/PPS/slice headers, CAVLC, transform, intra/inter prediction, deblocking, the decoder and the I_PCM/P_SKIP encoder.

use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;

// #region 🔖️H264

/// ⚠️ Error type for the hand-rolled H.264 baseline decoder. `Unsupported` is used for any spec feature this
/// decoder deliberately does not implement (CABAC, B-slices, 8×8 transform, interlace, FMO, reference-list
/// reordering, adaptive memory control, sub-8×8 P-partitions) — always a loud, named failure, never silent
/// misdecoding. `Truncated`/`Malformed` cover corrupt or adversarial bitstreams; this decoder never panics on
/// untrusted input.
#[derive(Clone, Debug, PartialEq)]
pub enum H264Error {
    Truncated,
    Malformed(&'static str),
    Unsupported(&'static str),
    NoSps,
    NoPps,
}

impl std::fmt::Display for H264Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "h264 bitstream truncated"),
            Self::Malformed(msg) => write!(f, "malformed h264 bitstream: {msg}"),
            Self::Unsupported(msg) => write!(f, "unsupported h264 feature: {msg}"),
            Self::NoSps => write!(f, "h264 slice references an unparsed sps"),
            Self::NoPps => write!(f, "h264 slice references an unparsed pps"),
        }
    }
}

impl std::error::Error for H264Error {}

// #endregion 🔖️H264

#[path = "📖️bitstream-and-headers/🦀️.rs"]
mod bitstream_and_headers;
use bitstream_and_headers::*;

#[path = "🔤️cavlc/🦀️.rs"]
mod cavlc;
use cavlc::*;

#[path = "🧮️reconstruction/🦀️.rs"]
mod reconstruction;
use reconstruction::*;

#[path = "🧼️deblocking-and-output/🦀️.rs"]
mod deblocking_and_output;
use deblocking_and_output::*;

#[path = "🍰️slice-data/🦀️.rs"]
mod slice_data;
use slice_data::*;

#[path = "✍️encoder/🦀️.rs"]
mod encoder;
pub use encoder::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #region 🔖️Decoder

/// 🎬️ Baseline-profile (Constrained Baseline / Baseline, `profile_idc == 66`) H.264 decoder. Decodes AVCC
/// length-prefixed access units one at a time via [`H264Decoder::decode_sample`], maintaining its own
/// decoded-picture buffer across calls. See the crate-level docs for the full list of spec features this
/// decoder deliberately does not implement — each fails loudly with [`H264Error::Unsupported`] rather than
/// silently misdecoding.
pub struct H264Decoder {
    sps: SpsInfo,
    pps: PpsInfo,
    nal_length_size: u8,
    dpb: Dpb,
    prev_poc_msb: i32,
    prev_poc_lsb: i32,
}

impl H264Decoder {
    /// 🏗️ Parses SPS/PPS from `sps_pps_nals` — a flat sequence of `(u16 big-endian length, NAL bytes)`
    /// entries (the provider's owned AVC configuration; classified by NAL type, order-
    /// independent). Rejects any non-baseline SPS/PPS feature immediately (see crate docs) rather than
    /// deferring the failure to the first `decode_sample` call.
    pub fn new(sps_pps_nals: &[u8]) -> Result<Self, H264Error> {
        let mut sps: Option<SpsInfo> = None;
        let mut pps: Option<PpsInfo> = None;
        let mut pos = 0usize;
        while pos < sps_pps_nals.len() {
            let len_bytes = sps_pps_nals.get(pos..pos + 2).ok_or(H264Error::Truncated)?;
            let len = usize::from(u16::from_be_bytes([len_bytes[0], len_bytes[1]]));
            pos += 2;
            let nal_bytes = sps_pps_nals.get(pos..pos + len).ok_or(H264Error::Truncated)?;
            pos += len;
            let nal = parse_nal(nal_bytes)?;
            match nal.nal_unit_type {
                7 => sps = Some(parse_sps(&nal.rbsp)?),
                8 => pps = Some(parse_pps(&nal.rbsp)?),
                _ => {}
            }
        }
        let sps = sps.ok_or(H264Error::NoSps)?;
        let pps = pps.ok_or(H264Error::NoPps)?;
        let max_refs = sps.max_num_ref_frames;
        Ok(Self { sps, pps, nal_length_size: 4, dpb: Dpb::new(max_refs), prev_poc_msb: 0, prev_poc_lsb: 0 })
    }

    /// 🔧️ Overrides the AVCC NAL length-prefix size (default `4`, the overwhelmingly common `avcC` value);
    /// callers that extracted `nal_length_size` from a real `avcC` box should pass it through here.
    pub fn with_nal_length_size(mut self, size: u8) -> Self {
        self.nal_length_size = size;
        self
    }

    /// 🧮️ `PicOrderCnt` (clauses 8.2.1.1/8.2.1.3): full derivation for `pic_order_cnt_type == 0`, and the
    /// `2 * frame_num` approximation for `type == 2` (exact for the non-wrapping, all-reference-frame,
    /// P/I-only streams this decoder ever produces or consumes; ordering only ever matters for output — this
    /// decoder never reorders, so precision here is informational, not load-bearing).
    fn compute_poc(&mut self, header: &SliceHeaderInfo, is_idr: bool) -> i32 {
        match self.sps.pic_order_cnt_type {
            0 => {
                let max_lsb = 1i32 << self.sps.log2_max_pic_order_cnt_lsb;
                let lsb = header.pic_order_cnt_lsb as i32;
                let msb = if is_idr {
                    0
                } else if lsb < self.prev_poc_lsb && (self.prev_poc_lsb - lsb) >= max_lsb / 2 {
                    self.prev_poc_msb + max_lsb
                } else if lsb > self.prev_poc_lsb && (lsb - self.prev_poc_lsb) > max_lsb / 2 {
                    self.prev_poc_msb - max_lsb
                } else {
                    self.prev_poc_msb
                };
                self.prev_poc_msb = msb;
                self.prev_poc_lsb = lsb;
                msb + lsb
            }
            _ => 2 * header.frame_num as i32,
        }
    }

    /// 📥️ Decodes one AVCC-length-prefixed access unit (exactly one slice NAL, optionally alongside SEI/AUD
    /// NALs it skips) into a full RGBA frame. `Ok(None)` never occurs for this decoder's supported baseline
    /// P/I-only content — output is always immediate, never buffered pending reordering.
    pub fn decode_sample(&mut self, nal_bytes: &[u8]) -> Result<Option<remodeling_image::ImageRgba8>, H264Error> {
        let nals = split_avcc_nals(nal_bytes, self.nal_length_size)?;
        let mut slice_nal: Option<NalUnit> = None;
        for raw in nals {
            let nal = parse_nal(raw)?;
            match nal.nal_unit_type {
                1 | 5 => {
                    if slice_nal.is_some() {
                        return Err(H264Error::Unsupported("multiple slices per access unit"));
                    }
                    slice_nal = Some(nal);
                }
                7 => self.sps = parse_sps(&nal.rbsp)?,
                8 => self.pps = parse_pps(&nal.rbsp)?,
                _ => {}
            }
        }
        let nal = slice_nal.ok_or(H264Error::Malformed("access unit has no slice nal"))?;
        let mut bits = BitReader::new(&nal.rbsp);
        let header = parse_slice_header(&mut bits, nal.nal_unit_type, &self.sps, &self.pps)?;
        if header.first_mb_in_slice != 0 {
            return Err(H264Error::Unsupported("multiple slices per picture"));
        }
        let is_idr = nal.nal_unit_type == 5;
        if is_idr {
            self.dpb.clear();
        }

        let mut pic = Picture::new(self.sps.pic_width_in_mbs, self.sps.pic_height_in_mbs);
        pic.frame_num = header.frame_num;
        pic.poc = self.compute_poc(&header, is_idr);

        let ref_list0 = self.dpb.ref_list0();
        decode_slice_data(&mut bits, &mut pic, &self.sps, &self.pps, &header, &ref_list0)?;
        drop(ref_list0);

        deblock_picture(&mut pic, &self.sps, &header);

        let image = pic.crop_to(self.sps.width_px, self.sps.height_px);
        self.dpb.push(pic);
        Ok(Some(image))
    }
}

// #endregion 🔖️Decoder

// #region 🔖️H264

/// 📐️ Recovers `(width, height)` from a raw SPS NAL (header byte + RBSP — [`h264_enc_sps_pps_nals`]'s
/// own output shape) via this crate's own [`parse_nal`]/[`parse_sps`] — `(0, 0)` if not a valid SPS.
fn sps_nal_dimensions(sps_nal: &[u8]) -> (u32, u32) {
    parse_nal(sps_nal).ok().filter(|nal| nal.nal_unit_type == 7).and_then(|nal| parse_sps(&nal.rbsp).ok()).map_or((0, 0), |sps| (sps.width_px, sps.height_px))
}

// #endregion 🔖️H264
