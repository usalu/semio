//! ✍️ Encoder of the baseline H.264 codec: the bit writer and the I_PCM and P_SKIP stream writer. See [`super`].


// #region 🔖️H264Enc
/// ✍️ MSB-first bit writer for hand-assembling spec-legal RBSPs (mirrors [`BitReader`]).
#[derive(Default)]
pub(super) struct BitWriter {
    pub(super) bytes: Vec<u8>,
    pub(super) acc: u32,
    nbits: u32,
}

impl BitWriter {
    fn put_bit(&mut self, bit: u32) {
        self.acc = (self.acc << 1) | (bit & 1);
        self.nbits += 1;
        if self.nbits == 8 {
            self.bytes.push(self.acc as u8);
            self.acc = 0;
            self.nbits = 0;
        }
    }

    pub(super) fn put_u(&mut self, v: u32, n: u8) {
        for i in (0..n).rev() {
            self.put_bit((v >> i) & 1);
        }
    }

    pub(super) fn put_ue(&mut self, v: u32) {
        let x = v + 1;
        let nbits = 32 - x.leading_zeros();
        for _ in 0..nbits - 1 {
            self.put_bit(0);
        }
        for i in (0..nbits).rev() {
            self.put_bit((x >> i) & 1);
        }
    }

    pub(super) fn put_se(&mut self, v: i32) {
        let code = if v <= 0 { (-v) as u32 * 2 } else { (v as u32) * 2 - 1 };
        self.put_ue(code);
    }

    /// 🏁️ `rbsp_trailing_bits()`: a `1` stop bit, then zero-padded to a byte boundary.
    pub(super) fn rbsp_trailing(&mut self) {
        self.put_bit(1);
        while self.nbits != 0 {
            self.put_bit(0);
        }
    }

    /// 🏁️ Mid-stream `pcm_alignment_zero_bit` padding: zero bits up to the next byte boundary, writing
    /// nothing at all when already aligned (unlike [`Self::rbsp_trailing`], which always emits a stop bit).
    pub(super) fn zero_align(&mut self) {
        while self.nbits != 0 {
            self.put_bit(0);
        }
    }
}

/// 🧹️ Inverse of [`strip_emulation_prevention`]: inserts `emulation_prevention_three_byte` (`00 00 0x` →
/// `00 00 03 0x` for `x <= 3`) so the RBSP round-trips through a real NAL byte stream.
fn add_emulation_prevention(rbsp: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(rbsp.len());
    let mut zero_run = 0u32;
    for &byte in rbsp {
        if zero_run >= 2 && byte <= 3 {
            out.push(3);
            zero_run = 0;
        }
        out.push(byte);
        zero_run = if byte == 0 { zero_run + 1 } else { 0 };
    }
    out
}

pub(super) fn write_nal(nal_ref_idc: u8, nal_unit_type: u8, rbsp: &[u8]) -> Vec<u8> {
    let mut out = vec![(nal_ref_idc << 5) | nal_unit_type];
    out.extend(add_emulation_prevention(rbsp));
    out
}

/// ✍️ Length-prefixes `nal` with a 4-byte big-endian length, matching this crate's own AVCC framing
/// (`H264Decoder`'s default `nal_length_size`).
pub(super) fn avcc_frame(nal: &[u8]) -> Vec<u8> {
    let mut out = (nal.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(nal);
    out
}

/// 🏗️ Builds a minimal baseline SPS+PPS pair for an `mb_w*16 × mb_h*16` picture: `pic_order_cnt_type = 2`
/// (no extra POC fields), a single reference frame, deblocking always disabled (`disable_deblocking_filter_idc
/// = 1` is written per-slice, but PPS still declares control present so the slice header carries it).
pub fn h264_enc_sps_pps_nals(mb_w: u32, mb_h: u32) -> (Vec<u8>, Vec<u8>) {
    let mut sps = BitWriter::default();
    sps.put_u(66, 8);
    sps.put_u(0, 8);
    sps.put_u(30, 8);
    sps.put_ue(0);
    sps.put_ue(4);
    sps.put_ue(2);
    sps.put_ue(15);
    sps.put_u(0, 1);
    sps.put_ue(mb_w - 1);
    sps.put_ue(mb_h - 1);
    sps.put_u(1, 1);
    sps.put_u(1, 1);
    sps.put_u(0, 1);
    sps.put_u(0, 1);
    sps.rbsp_trailing();

    let mut pps = BitWriter::default();
    pps.put_ue(0);
    pps.put_ue(0);
    pps.put_u(0, 1);
    pps.put_u(0, 1);
    pps.put_ue(0);
    pps.put_ue(0);
    pps.put_ue(0);
    pps.put_u(0, 1);
    pps.put_u(0, 2);
    pps.put_se(0);
    pps.put_se(0);
    pps.put_se(0);
    pps.put_u(1, 1);
    pps.put_u(0, 1);
    pps.put_u(0, 1);
    pps.rbsp_trailing();

    (write_nal(3, 7, &sps.bytes), write_nal(3, 8, &pps.bytes))
}

/// 🏗️ SPS+PPS in [`H264Decoder::new`]'s expected `(u16 length, NAL)*` format, for an `mb_w*16 × mb_h*16`
/// picture — pass the result straight to `H264Decoder::new`.
pub fn h264_enc_sps_pps(mb_w: u32, mb_h: u32) -> Vec<u8> {
    let (sps, pps) = h264_enc_sps_pps_nals(mb_w, mb_h);
    let mut out = Vec::new();
    for nal in [&sps, &pps] {
        out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
        out.extend_from_slice(nal);
    }
    out
}

/// 🏗️ Encodes one I_PCM-only IDR frame (`mb_w*16 × mb_h*16`, every macroblock `I_PCM`): a bit-exact,
/// entropy-coding-free round trip by construction — this crate's contract test for the whole NAL/slice-
/// header/SPS-PPS/PCM-macroblock plumbing. `luma`/`cb`/`cr` must be exactly `mb_w*16*mb_h*16`/`mb_w*8*mb_h*8`
/// samples (row-major); deblocking is disabled so the decoder returns these samples unchanged.
pub fn h264_enc_i_pcm_sample(mb_w: u32, mb_h: u32, frame_num: u32, luma: &[u8], cb: &[u8], cr: &[u8]) -> Vec<u8> {
    let mut s = BitWriter::default();
    s.put_ue(0);
    s.put_ue(7);
    s.put_ue(0);
    s.put_u(frame_num, 8);
    s.put_ue(0);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_se(0);
    s.put_ue(1);

    for n in 0..(mb_w * mb_h) {
        s.put_ue(25);
        s.zero_align();
        let (mb_x, mb_y) = (n % mb_w, n / mb_w);
        let lw = (mb_w * 16) as usize;
        for r in 0..16usize {
            for c in 0..16usize {
                s.put_u(u32::from(luma[(mb_y as usize * 16 + r) * lw + mb_x as usize * 16 + c]), 8);
            }
        }
        let cw = (mb_w * 8) as usize;
        for r in 0..8usize {
            for c in 0..8usize {
                s.put_u(u32::from(cb[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
            }
        }
        for r in 0..8usize {
            for c in 0..8usize {
                s.put_u(u32::from(cr[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
            }
        }
    }
    s.rbsp_trailing();
    avcc_frame(&write_nal(3, 5, &s.bytes))
}

/// 🏗️ Encodes one whole-frame `P_Skip` slice (non-IDR, references `frame_num - 1`): every macroblock skipped,
/// no residual/MV entropy coding at all — exercises the DPB reference lookup and the `mb_skip_run` slice-data
/// path without needing CAVLC-coded content.
pub fn h264_enc_p_skip_sample(mb_w: u32, mb_h: u32, frame_num: u32) -> Vec<u8> {
    let mut s = BitWriter::default();
    s.put_ue(0);
    s.put_ue(5);
    s.put_ue(0);
    s.put_u(frame_num, 8);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_se(0);
    s.put_ue(1);
    s.put_ue(mb_w * mb_h);
    s.rbsp_trailing();
    avcc_frame(&write_nal(2, 1, &s.bytes))
}
// #endregion 🔖️H264Enc
