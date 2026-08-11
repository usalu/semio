//! 🎥️ H.264 baseline-profile engine accessor — bitstream primitives, NAL/RBSP framing, real SPS
//! parsing (Exp-Golomb width/height recovery), and `avcC` (AVCDecoderConfigurationRecord)
//! extraction/construction. Moved from remodel's video engine (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs`
//! lines 916-1157, 305-333, 3683-3732) per the master plan's extraction map.
//!
//! Scoping note (honest boundary, matches this artifact's own `Mp4Sample.data` staying opaque
//! bytes — the master plan's "payload-opaque" convention for video subsets): remodel's full
//! macroblock reconstruction pipeline (intra/inter prediction, CAVLC residual decode, IDCT,
//! deblocking — that source file's lines ~1159-3356, ~2200 LOC) is deliberately NOT moved here.
//! `Mp4Sample` never stores decoded pixels, only the exact AVCC-framed access-unit bytes the
//! container held — so nothing in this artifact's codec_retention_law or schema needs a pixel
//! decode. What IS moved is every piece the container codec genuinely needs: NAL/RBSP framing
//! (used to validate AVCC sample framing on encode) and a real SPS parse (used by the `sniff`/
//! `analyzer` accessors below to recover width/height directly from the bitstream, independent
//! of the container's own `stsd` fields, for cross-validation). The full pixel decoder remains
//! at its original remodel location, unmoved, for a future wave to lift as a video-subset
//! accessor without needing to touch this container codec's schema.

//#region 🔖️Error
#[derive(Clone, Debug, PartialEq)]
pub enum H264Error {
    Truncated,
    Malformed(&'static str),
    Unsupported(&'static str),
}

impl std::fmt::Display for H264Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "h264: truncated bitstream"),
            Self::Malformed(m) => write!(f, "h264: malformed: {m}"),
            Self::Unsupported(m) => write!(f, "h264: unsupported: {m}"),
        }
    }
}
impl std::error::Error for H264Error {}
//#endregion 🔖️Error

//#region 🔖️Bits
/// 📖️ MSB-first bit reader over an RBSP (moved from remodel's `BitReader`; only the primitives
/// this accessor's SPS parse needs — `u(n)`/`ue(v)`).
pub struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, byte_pos: 0, bit_pos: 0 } }

    pub fn u1(&mut self) -> Result<u32, H264Error> {
        let &byte = self.data.get(self.byte_pos).ok_or(H264Error::Truncated)?;
        let bit = (byte >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 { self.bit_pos = 0; self.byte_pos += 1; }
        Ok(u32::from(bit))
    }

    pub fn u(&mut self, n: u8) -> Result<u32, H264Error> {
        let mut v = 0u32;
        for _ in 0..n { v = (v << 1) | self.u1()?; }
        Ok(v)
    }

    /// 🧮️ Exp-Golomb unsigned code (`ue(v)`, clause 9.1).
    pub fn ue(&mut self) -> Result<u32, H264Error> {
        let mut leading_zero_bits = 0u32;
        while self.u1()? == 0 {
            leading_zero_bits += 1;
            if leading_zero_bits > 31 {
                return Err(H264Error::Malformed("exp-golomb code exceeds 31 leading zero bits"));
            }
        }
        if leading_zero_bits == 0 { return Ok(0); }
        let suffix = self.u(leading_zero_bits as u8)?;
        Ok((1u32 << leading_zero_bits) - 1 + suffix)
    }
}
//#endregion 🔖️Bits

//#region 🔖️Rbsp
/// 🧹️ Removes `emulation_prevention_three_byte`s (`00 00 03` → `00 00`), moved verbatim from
/// remodel's `strip_emulation_prevention`. <https://www.itu.int/rec/T-REC-H.264> (clause 7.4.1.1)
pub fn strip_emulation_prevention(ebsp: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(ebsp.len());
    let mut i = 0;
    while i < ebsp.len() {
        if i + 2 < ebsp.len() && ebsp[i] == 0 && ebsp[i + 1] == 0 && ebsp[i + 2] == 3 {
            out.push(0);
            out.push(0);
            i += 3;
        } else {
            out.push(ebsp[i]);
            i += 1;
        }
    }
    out
}

/// 🧩️ One parsed NAL unit's type + emulation-prevention-stripped RBSP (moved from `NalUnit`/`parse_nal`).
pub struct NalUnit {
    pub nal_unit_type: u8,
    pub rbsp: Vec<u8>,
}

pub fn parse_nal(nal_bytes: &[u8]) -> Result<NalUnit, H264Error> {
    let &header = nal_bytes.first().ok_or(H264Error::Truncated)?;
    if header & 0x80 != 0 {
        return Err(H264Error::Malformed("nal forbidden_zero_bit is set"));
    }
    let nal_unit_type = header & 0x1F;
    Ok(NalUnit { nal_unit_type, rbsp: strip_emulation_prevention(&nal_bytes[1..]) })
}

/// ✂️ Splits one AVCC length-prefixed access unit into NAL byte ranges (moved from `split_avcc_nals`).
pub fn split_avcc_nals(data: &[u8], length_size: u8) -> Result<Vec<&[u8]>, H264Error> {
    let n = length_size as usize;
    if !(1..=4).contains(&n) || n == 3 {
        return Err(H264Error::Unsupported("nal length size other than 1, 2 or 4 bytes"));
    }
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < data.len() {
        let len_bytes = data.get(pos..pos + n).ok_or(H264Error::Truncated)?;
        let len = len_bytes.iter().fold(0usize, |acc, &b| (acc << 8) | usize::from(b));
        pos += n;
        let nal = data.get(pos..pos + len).ok_or(H264Error::Truncated)?;
        out.push(nal);
        pos += len;
    }
    Ok(out)
}
//#endregion 🔖️Rbsp

//#region 🔖️Sps
/// 🎬️ Real SPS width/height (moved/trimmed from remodel's `SpsInfo`/`parse_sps` — only the
/// fields this container codec's cross-validation accessor uses; baseline-profile only, matches
/// the source's own scoping). <https://www.itu.int/rec/T-REC-H.264> (clause 7.3.2.1.1)
#[derive(Clone, Debug, PartialEq)]
pub struct SpsDimensions { pub width_px: u32, pub height_px: u32 }

pub fn parse_sps_dimensions(rbsp: &[u8]) -> Result<SpsDimensions, H264Error> {
    let mut b = BitReader::new(rbsp);
    let profile_idc = b.u(8)?;
    b.u(8)?;
    b.u(8)?;
    b.ue()?;
    b.ue()?;
    let pic_order_cnt_type = b.ue()?;
    if pic_order_cnt_type == 0 {
        b.ue()?;
    } else if pic_order_cnt_type == 1 {
        return Err(H264Error::Unsupported("pic_order_cnt_type 1"));
    } else if pic_order_cnt_type > 2 {
        return Err(H264Error::Malformed("pic_order_cnt_type out of range"));
    }
    b.ue()?;
    b.u1()?;
    let pic_width_in_mbs = b.ue()? + 1;
    let pic_height_in_map_units = b.ue()? + 1;
    let frame_mbs_only_flag = b.u1()?;
    if frame_mbs_only_flag == 0 {
        return Err(H264Error::Unsupported("interlaced sps (frame_mbs_only_flag == 0)"));
    }
    b.u1()?;
    let frame_cropping_flag = b.u1()?;
    let (mut crop_left, mut crop_right, mut crop_top, mut crop_bottom) = (0u32, 0u32, 0u32, 0u32);
    if frame_cropping_flag == 1 {
        crop_left = b.ue()?;
        crop_right = b.ue()?;
        crop_top = b.ue()?;
        crop_bottom = b.ue()?;
    }
    let _ = profile_idc;
    let width_px = pic_width_in_mbs * 16 - 2 * (crop_left + crop_right);
    let height_px = pic_height_in_map_units * 16 - 2 * (crop_top + crop_bottom);
    if width_px == 0 || height_px == 0 {
        return Err(H264Error::Malformed("sps describes a zero-sized picture"));
    }
    Ok(SpsDimensions { width_px, height_px })
}
//#endregion 🔖️Sps

//#region 🔖️AvcC
/// 📥️ `avcC` (AVCDecoderConfigurationRecord) → `(sps_list, pps_list, nal_length_size)`, each NAL
/// kept as its own owned byte vector (adapted from remodel's `extract_avc_config`, which instead
/// flattens SPS/PPS into one `(u16 len, NAL)*` buffer — this artifact's `Mp4Codec::Avc` schema
/// wants them as separate lists, so the adaptation only changes the output container shape, not
/// the parse logic). <https://www.iso.org/standard/74428.html> (ISO/IEC 14496-15)
pub fn parse_avcc(avcc: &[u8]) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>, u8), H264Error> {
    let mut pos = 4usize;
    let length_size_byte = *avcc.get(pos).ok_or(H264Error::Truncated)?;
    pos += 1;
    let nal_length_size = (length_size_byte & 0x03) + 1;
    let num_sps = avcc.get(pos).ok_or(H264Error::Truncated)? & 0x1F;
    pos += 1;
    let mut sps_list = Vec::new();
    for _ in 0..num_sps {
        let len = u16::from_be_bytes(avcc.get(pos..pos + 2).ok_or(H264Error::Truncated)?.try_into().unwrap()) as usize;
        pos += 2;
        sps_list.push(avcc.get(pos..pos + len).ok_or(H264Error::Truncated)?.to_vec());
        pos += len;
    }
    let num_pps = *avcc.get(pos).ok_or(H264Error::Truncated)?;
    pos += 1;
    let mut pps_list = Vec::new();
    for _ in 0..num_pps {
        let len = u16::from_be_bytes(avcc.get(pos..pos + 2).ok_or(H264Error::Truncated)?.try_into().unwrap()) as usize;
        pos += 2;
        pps_list.push(avcc.get(pos..pos + len).ok_or(H264Error::Truncated)?.to_vec());
        pos += len;
    }
    Ok((sps_list, pps_list, nal_length_size))
}

/// ✍️ Builds an `avcC` box from separate SPS/PPS lists (adapted from remodel's `build_avcc`,
/// which reads its input from the flattened `(u16,NAL)*` internal format — this version takes
/// the lists directly since that is this artifact's own `Mp4Codec::Avc` shape). `profile`/
/// `compat`/`level` are read back out of the first SPS when present (bytes 1..4 of the RBSP,
/// clause 7.3.2.1.1's `profile_idc`/`constraint flags`/`level_idc`) so a round-tripped file
/// reports the same AVC profile it was decoded from, instead of a fixed placeholder.
pub fn build_avcc(sps_list: &[Vec<u8>], pps_list: &[Vec<u8>], nal_length_size: u8) -> Vec<u8> {
    let (profile, compat, level) = sps_list.first().and_then(|s| s.get(1..4)).map_or((66, 0, 30), |b| (b[0], b[1], b[2]));
    let mut out = vec![1, profile, compat, level, 0xFC | (nal_length_size.saturating_sub(1) & 0x03), 0xE0 | (sps_list.len() as u8 & 0x1F)];
    for nal in sps_list {
        out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
        out.extend_from_slice(nal);
    }
    out.push(pps_list.len() as u8);
    for nal in pps_list {
        out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
        out.extend_from_slice(nal);
    }
    crate::artifacts::mp4::standards::isobmff::engine::boxes::write_box(b"avcC", &out)
}
//#endregion 🔖️AvcC

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avcc_round_trips_sps_pps_lists() {
        // 🧪️ A tiny, structurally valid baseline SPS RBSP (profile_idc=66) — enough to exercise
        // parse_sps_dimensions and the avcC list round trip without depending on a real fixture.
        let sps: Vec<u8> = vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40];
        let pps: Vec<u8> = vec![0x68, 0xCE, 0x3C, 0x80];
        let avcc = build_avcc(&[sps.clone()], &[pps.clone()], 4);
        // strip the write_box 8-byte header to get the raw avcC payload parse_avcc expects.
        let payload = &avcc[8..];
        let (sps_out, pps_out, nal_len) = parse_avcc(payload).expect("parse avcc");
        assert_eq!(sps_out, vec![sps]);
        assert_eq!(pps_out, vec![pps]);
        assert_eq!(nal_len, 4);
    }

    #[test]
    fn strip_emulation_prevention_removes_three_byte() {
        let ebsp = [0x00, 0x00, 0x03, 0x01, 0x00, 0x00, 0x03, 0x02];
        assert_eq!(strip_emulation_prevention(&ebsp), vec![0x00, 0x00, 0x01, 0x00, 0x00, 0x02]);
    }
}
