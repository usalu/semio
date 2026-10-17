//! 📖️ Bitstream syntax of the baseline H.264 decoder: the bit reader, RBSP extraction and the SPS, PPS and slice header parsing. See [`super`].

use super::*;

// #region 🔖️Bits
/// 📖️ MSB-first bit reader over an RBSP (emulation-prevention already stripped), providing the three H.264
/// bitstream primitives: `u(n)` fixed-width, `ue(v)` Exp-Golomb unsigned, `se(v)` Exp-Golomb signed.
/// <https://www.itu.int/rec/T-REC-H.264> (clause 9.1)
pub(super) struct BitReader<'a> {
    pub(super) data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    pub(super) fn new(data: &'a [u8]) -> Self {
        Self { data, byte_pos: 0, bit_pos: 0 }
    }

    pub(super) fn u1(&mut self) -> Result<u32, H264Error> {
        let &byte = self.data.get(self.byte_pos).ok_or(H264Error::Truncated)?;
        let bit = (byte >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }
        Ok(u32::from(bit))
    }

    pub(super) fn u(&mut self, n: u8) -> Result<u32, H264Error> {
        let mut v = 0u32;
        for _ in 0..n {
            v = (v << 1) | self.u1()?;
        }
        Ok(v)
    }

    /// 🧮️ Exp-Golomb unsigned code (`ue(v)`, clause 9.1).
    pub(super) fn ue(&mut self) -> Result<u32, H264Error> {
        let mut leading_zero_bits = 0u32;
        while self.u1()? == 0 {
            leading_zero_bits += 1;
            if leading_zero_bits > 31 {
                return Err(H264Error::Malformed("exp-golomb code exceeds 31 leading zero bits"));
            }
        }
        if leading_zero_bits == 0 {
            return Ok(0);
        }
        let suffix = self.u(leading_zero_bits as u8)?;
        Ok((1u32 << leading_zero_bits) - 1 + suffix)
    }

    /// 🧮️ Exp-Golomb signed code (`se(v)`, clause 9.1.1): maps the unsigned code `k` to `(-1)^(k+1) * ceil(k/2)`.
    pub(super) fn se(&mut self) -> Result<i32, H264Error> {
        let code = self.ue()? as i64;
        let magnitude = (code + 1) / 2;
        let value = if code % 2 == 0 { -magnitude } else { magnitude };
        i32::try_from(value).map_err(|_| H264Error::Malformed("se(v) value out of range"))
    }

    pub(super) fn byte_align(&mut self) {
        if self.bit_pos != 0 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }
    }

    /// 🏁️ `more_rbsp_data()` (clause 7.2): locates `rbsp_stop_one_bit` (the last `1` bit in the RBSP) and
    /// reports whether the cursor still sits strictly before it.
    pub(super) fn more_rbsp_data(&self) -> bool {
        if self.byte_pos >= self.data.len() {
            return false;
        }
        let mut last_nonzero = self.data.len();
        while last_nonzero > 0 && self.data[last_nonzero - 1] == 0 {
            last_nonzero -= 1;
        }
        if last_nonzero == 0 {
            return false;
        }
        let last_byte = self.data[last_nonzero - 1];
        let stop_bit_index = (last_nonzero - 1) * 8 + (7 - last_byte.trailing_zeros() as usize);
        let current_bit_index = self.byte_pos * 8 + self.bit_pos as usize;
        current_bit_index < stop_bit_index
    }
}
// #endregion 🔖️Bits

// #region 🔖️Rbsp
/// 🧹️ Removes `emulation_prevention_three_byte`s (`00 00 03` → `00 00`) from a NAL's EBSP, yielding its RBSP.
/// <https://www.itu.int/rec/T-REC-H.264> (clause 7.4.1.1)
fn strip_emulation_prevention(ebsp: &[u8]) -> Vec<u8> {
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

/// 🧩️ One parsed NAL unit: its type plus emulation-prevention-stripped RBSP payload. `nal_ref_idc` is
/// intentionally not retained — this decoder's DPB (see [`Dpb`]) treats every decoded picture as a reference,
/// a scoped simplification since this crate's own encoder never emits non-reference pictures.
pub(super) struct NalUnit {
    pub(super) nal_unit_type: u8,
    pub(super) rbsp: Vec<u8>,
}

/// 📥️ Parses a single NAL's 1-byte header and strips emulation prevention from the rest.
pub(super) fn parse_nal(nal_bytes: &[u8]) -> Result<NalUnit, H264Error> {
    let &header = nal_bytes.first().ok_or(H264Error::Truncated)?;
    if header & 0x80 != 0 {
        return Err(H264Error::Malformed("nal forbidden_zero_bit is set"));
    }
    let nal_unit_type = header & 0x1F;
    Ok(NalUnit { nal_unit_type, rbsp: strip_emulation_prevention(&nal_bytes[1..]) })
}

/// ✂️ Splits one AVCC length-prefixed access unit into its constituent NAL byte ranges (each still including
/// the 1-byte NAL header); `length_size` is `avcC`'s `lengthSizeMinusOne + 1` (1, 2 or 4 bytes).
pub(super) fn split_avcc_nals(data: &[u8], length_size: u8) -> Result<Vec<&[u8]>, H264Error> {
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

/// ✂️ Splits an Annex-B byte stream (`00 00 01` / `00 00 00 01` start codes) into NAL byte ranges; provided
/// for parity with the spec's other framing convention, alongside [`split_avcc_nals`] which this crate's own
/// [`H264Decoder`]/[`Mux`](mod@self) plumbing actually uses.
#[allow(dead_code, reason = "public per the plan's 🔖️Rbsp subregion spec even though this crate's own mux/decode path only exercises AVCC framing")]
pub(super) fn split_annexb_nals(data: &[u8]) -> Vec<&[u8]> {
    let mut starts = Vec::new();
    let mut i = 0;
    while i + 2 < data.len() {
        if data[i] == 0 && data[i + 1] == 0 && data[i + 2] == 1 {
            starts.push(i + 3);
            i += 3;
        } else {
            i += 1;
        }
    }
    let mut out = Vec::new();
    for (idx, &start) in starts.iter().enumerate() {
        let mut end = starts.get(idx + 1).map_or(data.len(), |&next_start| {
            let mut e = next_start - 3;
            if e > start && data[e - 1] == 0 {
                e -= 1;
            }
            e
        });
        if end < start {
            end = start;
        }
        out.push(&data[start..end]);
    }
    out
}
// #endregion 🔖️Rbsp

// #region 🔖️Sps
/// 🎬️ Parsed SPS fields this decoder needs. Only Baseline profile (`profile_idc == 66`), progressive frames
/// (`frame_mbs_only_flag == 1`) and `pic_order_cnt_type` 0 or 2 are accepted — everything else is a loud
/// [`H264Error::Unsupported`] at parse time rather than a best-effort (and possibly wrong) decode later.
#[derive(Clone, Debug)]
pub(super) struct SpsInfo {
    log2_max_frame_num: u32,
    pub(super) pic_order_cnt_type: u32,
    pub(super) log2_max_pic_order_cnt_lsb: u32,
    pub(super) max_num_ref_frames: u32,
    pub(super) pic_width_in_mbs: u32,
    pub(super) pic_height_in_mbs: u32,
    pub(super) width_px: u32,
    pub(super) height_px: u32,
}

/// 📥️ Parses a Sequence Parameter Set RBSP. <https://www.itu.int/rec/T-REC-H.264> (clause 7.3.2.1.1)
pub(super) fn parse_sps(rbsp: &[u8]) -> Result<SpsInfo, H264Error> {
    let mut b = BitReader::new(rbsp);
    let profile_idc = b.u(8)?;
    if profile_idc != 66 {
        return Err(H264Error::Unsupported("non-baseline sps profile_idc (only 66 is supported)"));
    }
    b.u(8)?;
    b.u(8)?;
    b.ue()?;
    let log2_max_frame_num = b.ue()? + 4;
    let pic_order_cnt_type = b.ue()?;
    let log2_max_pic_order_cnt_lsb = match pic_order_cnt_type {
        0 => b.ue()? + 4,
        1 => return Err(H264Error::Unsupported("pic_order_cnt_type 1")),
        2 => 0,
        _ => return Err(H264Error::Malformed("pic_order_cnt_type out of range")),
    };
    let max_num_ref_frames = b.ue()?;
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
    let pic_height_in_mbs = pic_height_in_map_units;
    let width_px = pic_width_in_mbs * 16 - 2 * (crop_left + crop_right);
    let height_px = pic_height_in_mbs * 16 - 2 * (crop_top + crop_bottom);
    if width_px == 0 || height_px == 0 {
        return Err(H264Error::Malformed("sps describes a zero-sized picture"));
    }
    Ok(SpsInfo { log2_max_frame_num, pic_order_cnt_type, log2_max_pic_order_cnt_lsb, max_num_ref_frames, pic_width_in_mbs, pic_height_in_mbs, width_px, height_px })
}
// #endregion 🔖️Sps

// #region 🔖️Pps
/// 🎬️ Parsed PPS fields this decoder needs; entropy coding is always CAVLC and slices are always a single
/// slice group (constraints are enforced at parse time, see [`parse_pps`]). `constrained_intra_pred_flag` is
/// parsed (so the bitstream stays in sync) but not enforced during intra prediction — a scoped simplification
/// unexercised by this crate's own encoder, which never sets it.
#[derive(Clone, Debug)]
pub(super) struct PpsInfo {
    num_ref_idx_l0_default_active: u32,
    weighted_pred_flag: bool,
    pic_init_qp: i32,
    pub(super) chroma_qp_index_offset: i32,
    deblocking_filter_control_present_flag: bool,
}

/// 📥️ Parses a Picture Parameter Set RBSP. <https://www.itu.int/rec/T-REC-H.264> (clause 7.3.2.2)
pub(super) fn parse_pps(rbsp: &[u8]) -> Result<PpsInfo, H264Error> {
    let mut b = BitReader::new(rbsp);
    b.ue()?;
    b.ue()?;
    let entropy_coding_mode_flag = b.u1()?;
    if entropy_coding_mode_flag != 0 {
        return Err(H264Error::Unsupported("cabac entropy coding (entropy_coding_mode_flag == 1)"));
    }
    b.u1()?;
    let num_slice_groups_minus1 = b.ue()?;
    if num_slice_groups_minus1 != 0 {
        return Err(H264Error::Unsupported("multiple slice groups (fmo)"));
    }
    let num_ref_idx_l0_default_active = b.ue()? + 1;
    b.ue()?;
    let weighted_pred_flag = b.u1()? == 1;
    b.u(2)?;
    let pic_init_qp = b.se()? + 26;
    b.se()?;
    let chroma_qp_index_offset = b.se()?;
    let deblocking_filter_control_present_flag = b.u1()? == 1;
    b.u1()?;
    b.u1()?;
    if b.more_rbsp_data() {
        let transform_8x8_mode_flag = b.u1()?;
        if transform_8x8_mode_flag == 1 {
            return Err(H264Error::Unsupported("8x8 transform (transform_8x8_mode_flag == 1)"));
        }
    }
    Ok(PpsInfo { num_ref_idx_l0_default_active, weighted_pred_flag, pic_init_qp, chroma_qp_index_offset, deblocking_filter_control_present_flag })
}
// #endregion 🔖️Pps

// #region 🔖️SliceHeader
/// 🎬️ Parsed slice header fields this decoder needs. `slice_type_mod5 == 0` is P, `== 2` is I (B/SP/SI are
/// rejected during parsing, see [`parse_slice_header`]).
#[derive(Clone, Debug)]
pub(super) struct SliceHeaderInfo {
    pub(super) first_mb_in_slice: u32,
    pub(super) slice_type_mod5: u32,
    pub(super) frame_num: u32,
    pub(super) pic_order_cnt_lsb: u32,
    pub(super) num_ref_idx_l0_active: u32,
    pub(super) slice_qp: i32,
    pub(super) disable_deblocking_filter_idc: u32,
    pub(super) slice_alpha_c0_offset: i32,
    pub(super) slice_beta_offset: i32,
}

/// 📥️ Parses a slice header RBSP prefix (up to but not including `slice_data()`), advancing `b` so the
/// caller can continue reading macroblock data from the same cursor. <https://www.itu.int/rec/T-REC-H.264>
/// (clause 7.3.3)
pub(super) fn parse_slice_header(b: &mut BitReader<'_>, nal_unit_type: u8, sps: &SpsInfo, pps: &PpsInfo) -> Result<SliceHeaderInfo, H264Error> {
    let first_mb_in_slice = b.ue()?;
    let slice_type = b.ue()?;
    let slice_type_mod5 = slice_type % 5;
    if slice_type_mod5 != 0 && slice_type_mod5 != 2 {
        return Err(H264Error::Unsupported("slice type other than P or I (b/sp/si)"));
    }
    b.ue()?;
    let frame_num = b.u(sps.log2_max_frame_num as u8)?;
    let is_idr = nal_unit_type == 5;
    if is_idr {
        b.ue()?;
    }
    let mut pic_order_cnt_lsb = 0u32;
    if sps.pic_order_cnt_type == 0 {
        pic_order_cnt_lsb = b.u(sps.log2_max_pic_order_cnt_lsb as u8)?;
    }
    let mut num_ref_idx_l0_active = pps.num_ref_idx_l0_default_active;
    if slice_type_mod5 == 0 {
        let num_ref_idx_active_override_flag = b.u1()?;
        if num_ref_idx_active_override_flag == 1 {
            num_ref_idx_l0_active = b.ue()? + 1;
        }
        let ref_pic_list_modification_flag_l0 = b.u1()?;
        if ref_pic_list_modification_flag_l0 == 1 {
            return Err(H264Error::Unsupported("reference picture list reordering"));
        }
        if pps.weighted_pred_flag {
            return Err(H264Error::Unsupported("weighted prediction"));
        }
    }
    if is_idr {
        b.u1()?;
        b.u1()?;
    } else {
        let adaptive_ref_pic_marking_mode_flag = b.u1()?;
        if adaptive_ref_pic_marking_mode_flag == 1 {
            return Err(H264Error::Unsupported("adaptive reference picture marking (mmco)"));
        }
    }
    let slice_qp = pps.pic_init_qp + b.se()?;
    let mut disable_deblocking_filter_idc = 0u32;
    let mut slice_alpha_c0_offset = 0i32;
    let mut slice_beta_offset = 0i32;
    if pps.deblocking_filter_control_present_flag {
        disable_deblocking_filter_idc = b.ue()?;
        if disable_deblocking_filter_idc != 1 {
            slice_alpha_c0_offset = b.se()? * 2;
            slice_beta_offset = b.se()? * 2;
        }
    }
    Ok(SliceHeaderInfo { first_mb_in_slice, slice_type_mod5, frame_num, pic_order_cnt_lsb, num_ref_idx_l0_active, slice_qp, disable_deblocking_filter_idc, slice_alpha_c0_offset, slice_beta_offset })
}
// #endregion 🔖️SliceHeader
