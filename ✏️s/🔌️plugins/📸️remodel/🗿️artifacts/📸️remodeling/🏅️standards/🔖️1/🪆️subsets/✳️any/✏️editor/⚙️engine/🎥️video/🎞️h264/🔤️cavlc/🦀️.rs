//! 🔤️ CAVLC of the baseline H.264 decoder: the coefficient token, total zeros and run before tables and the residual block decoding. See [`super`].

use super::*;

// #region 🔖️Cavlc
/// 🔀️ H.264's 4×4 zig-zag scan: `ZIGZAG_4X4[k]` is the raster (`row*4+col`) position of the coefficient at
/// scan index `k`. <https://www.itu.int/rec/T-REC-H.264> (clause 8.5.6, Figure 8-8)
pub(super) const ZIGZAG_4X4: [usize; 16] = [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];

/// 📖️ One VLC's `(length, bits)` table pair, indexed by `4*totalCoeff + trailingOnes` (clause 9.2.1, Table 9-5).
struct CoeffTokenTable {
    pub(super) len: &'static [u8],
    pub(super) bits: &'static [u8],
}

/// 📖️ `coeff_token` VLC tables selected by `nC` bucket, transcribed from ITU-T H.264 Table 9-5.
const COEFF_TOKEN_TABLES: [CoeffTokenTable; 4] = [
    CoeffTokenTable {
        len: &[
            1, 0, 0, 0, 6, 2, 0, 0, 8, 6, 3, 0, 9, 8, 7, 5, 10, 9, 8, 6, 11, 10, 9, 7, 13, 11, 10, 8, 13, 13, 11, 9, 13, 13, 13, 10, 14, 14, 13, 11, 14, 14, 14, 13, 15, 15, 14, 14, 15, 15, 15, 14, 16, 15, 15, 15, 16, 16, 16, 15, 16, 16, 16, 16, 16,
            16, 16, 16,
        ],
        bits: &[1, 0, 0, 0, 5, 1, 0, 0, 7, 4, 1, 0, 7, 6, 5, 3, 7, 6, 5, 3, 7, 6, 5, 4, 15, 6, 5, 4, 11, 14, 5, 4, 8, 10, 13, 4, 15, 14, 9, 4, 11, 10, 13, 12, 15, 14, 9, 12, 11, 10, 13, 8, 15, 1, 9, 12, 11, 14, 13, 8, 7, 10, 9, 12, 4, 6, 5, 8],
    },
    CoeffTokenTable {
        len: &[
            2, 0, 0, 0, 6, 2, 0, 0, 6, 5, 3, 0, 7, 6, 6, 4, 8, 6, 6, 4, 8, 7, 7, 5, 9, 8, 8, 6, 11, 9, 9, 6, 11, 11, 11, 7, 12, 11, 11, 9, 12, 12, 12, 11, 12, 12, 12, 11, 13, 13, 13, 12, 13, 13, 13, 13, 13, 14, 13, 13, 14, 14, 14, 13, 14, 14, 14, 14,
        ],
        bits: &[3, 0, 0, 0, 11, 2, 0, 0, 7, 7, 3, 0, 7, 10, 9, 5, 7, 6, 5, 4, 4, 6, 5, 6, 7, 6, 5, 8, 15, 6, 5, 4, 11, 14, 13, 4, 15, 10, 9, 4, 11, 14, 13, 12, 8, 10, 9, 8, 15, 14, 13, 12, 11, 10, 9, 12, 7, 11, 6, 8, 9, 8, 10, 1, 7, 6, 5, 4],
    },
    CoeffTokenTable {
        len: &[4, 0, 0, 0, 6, 4, 0, 0, 6, 5, 4, 0, 6, 5, 5, 4, 7, 5, 5, 4, 7, 5, 5, 4, 7, 6, 6, 4, 7, 6, 6, 4, 8, 7, 7, 5, 8, 8, 7, 6, 9, 8, 8, 7, 9, 9, 8, 8, 9, 9, 9, 8, 10, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        bits: &[
            15, 0, 0, 0, 15, 14, 0, 0, 11, 15, 13, 0, 8, 12, 14, 12, 15, 10, 11, 11, 11, 8, 9, 10, 9, 14, 13, 9, 8, 10, 9, 8, 15, 14, 13, 13, 11, 14, 10, 12, 15, 10, 13, 12, 11, 14, 9, 12, 8, 10, 13, 8, 13, 7, 9, 12, 9, 12, 11, 10, 5, 8, 7, 6, 1, 4,
            3, 2,
        ],
    },
    CoeffTokenTable {
        len: &[6, 0, 0, 0, 6, 6, 0, 0, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6],
        bits: &[
            3, 0, 0, 0, 0, 1, 0, 0, 4, 5, 6, 0, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
            59, 60, 61, 62, 63,
        ],
    },
];

/// 📖️ `coeff_token` VLC for chroma DC (`nC == -1`, 4:2:0's 2×2 block), Table 9-5's rightmost-but-one column.
const CHROMA_DC_COEFF_TOKEN: CoeffTokenTable = CoeffTokenTable { len: &[2, 0, 0, 0, 6, 1, 0, 0, 6, 6, 3, 0, 6, 7, 7, 6, 6, 8, 8, 7], bits: &[1, 0, 0, 0, 7, 1, 0, 0, 4, 6, 1, 0, 3, 3, 2, 5, 2, 3, 2, 0] };

/// 🔎️ Reads one `coeff_token` from `table`, returning `(total_coeff, trailing_ones)`.
fn read_coeff_token(b: &mut BitReader<'_>, table: &CoeffTokenTable) -> Result<(u32, u32), H264Error> {
    let idx = read_vlc(b, table.len, table.bits)?;
    Ok((idx / 4, idx % 4))
}

/// 🔎️ Reads `coeff_token` for `nC >= 8`: a fixed 6-bit code, `((TotalCoeff-1)<<2)|TrailingOnes`, with `000011`
/// as the sole exception encoding `TotalCoeff == 0`.
fn read_coeff_token_fixed(b: &mut BitReader<'_>) -> Result<(u32, u32), H264Error> {
    let v = b.u(6)?;
    if v == 3 {
        return Ok((0, 0));
    }
    Ok((v / 4 + 1, v % 4))
}

/// 🧮️ Predicted `nC` (clause 9.2.1): average of the left/above 4×4 neighbor blocks' total_coeff, rounded up,
/// or just one of them when the other is unavailable, or `0` when neither is available.
pub(super) fn predict_nc(left: Option<u8>, above: Option<u8>) -> u32 {
    match (left, above) {
        (Some(l), Some(a)) => (u32::from(l) + u32::from(a)).div_ceil(2),
        (Some(l), None) => u32::from(l),
        (None, Some(a)) => u32::from(a),
        (None, None) => 0,
    }
}

const TOTAL_ZEROS_LEN: [&[u8]; 15] = [
    &[1, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 9],
    &[3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6],
    &[4, 3, 3, 3, 4, 4, 3, 3, 4, 5, 5, 6, 5, 6],
    &[5, 3, 4, 4, 3, 3, 3, 4, 3, 4, 5, 5, 5],
    &[4, 4, 4, 3, 3, 3, 3, 3, 4, 5, 4, 5],
    &[6, 5, 3, 3, 3, 3, 3, 3, 4, 3, 6],
    &[6, 5, 3, 3, 3, 2, 3, 4, 3, 6],
    &[6, 4, 5, 3, 2, 2, 3, 3, 6],
    &[6, 6, 4, 2, 2, 3, 2, 5],
    &[5, 5, 3, 2, 2, 2, 4],
    &[4, 4, 3, 3, 1, 3],
    &[4, 4, 2, 1, 3],
    &[3, 3, 1, 2],
    &[2, 2, 1],
    &[1, 1],
];

const TOTAL_ZEROS_BITS: [&[u8]; 15] = [
    &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 1],
    &[7, 6, 5, 4, 3, 5, 4, 3, 2, 3, 2, 3, 2, 1, 0],
    &[5, 7, 6, 5, 4, 3, 4, 3, 2, 3, 2, 1, 1, 0],
    &[3, 7, 5, 4, 6, 5, 4, 3, 3, 2, 2, 1, 0],
    &[5, 4, 3, 7, 6, 5, 4, 3, 2, 1, 1, 0],
    &[1, 1, 7, 6, 5, 4, 3, 2, 1, 1, 0],
    &[1, 1, 5, 4, 3, 3, 2, 1, 1, 0],
    &[1, 1, 1, 3, 3, 2, 2, 1, 0],
    &[1, 0, 1, 3, 2, 1, 1, 1],
    &[1, 0, 1, 3, 2, 1, 1],
    &[0, 1, 1, 2, 1, 3],
    &[0, 1, 1, 1, 1],
    &[0, 1, 1, 1],
    &[0, 1, 1],
    &[0, 1],
];

const CHROMA_DC_TOTAL_ZEROS_LEN: [[u8; 3]; 3] = [[1, 2, 3], [1, 2, 2], [1, 1, 0]];

const CHROMA_DC_TOTAL_ZEROS_BITS: [[u8; 3]; 3] = [[1, 1, 1], [1, 1, 0], [1, 0, 0]];

const RUN_BEFORE_LEN: [&[u8]; 7] = [&[1, 1], &[1, 2, 2], &[2, 2, 2, 2], &[2, 2, 2, 3, 3], &[2, 2, 3, 3, 3, 3], &[2, 3, 3, 3, 3, 3, 3], &[3, 3, 3, 3, 3, 3, 3, 4, 5, 6, 7, 8, 9, 10, 11]];

const RUN_BEFORE_BITS: [&[u8]; 7] = [&[1, 0], &[1, 1, 0], &[3, 2, 1, 0], &[3, 2, 1, 1, 0], &[3, 2, 3, 2, 1, 0], &[3, 0, 1, 3, 2, 5, 4], &[7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1]];

/// 🔎️ Generic canonical-VLC reader matching bits MSB-first against parallel `(len, bits)` slices, returning
/// the matching slice index (the semantic value depends on the table: `total_zeros`, `run_before`, ...).
fn read_vlc(b: &mut BitReader<'_>, len: &[u8], bits: &[u8]) -> Result<u32, H264Error> {
    let mut code = 0u32;
    for l in 1..=16u32 {
        code = (code << 1) | b.u1()?;
        for (idx, (&entry_len, &entry_bits)) in len.iter().zip(bits.iter()).enumerate() {
            if u32::from(entry_len) == l && u32::from(entry_bits) == code {
                return Ok(idx as u32);
            }
        }
    }
    Err(H264Error::Malformed("invalid vlc code"))
}

/// 🔎️ `level_prefix` (clause 9.2.2.1): a unary prefix, counted as leading `0` bits before the terminating `1`.
fn read_level_prefix(b: &mut BitReader<'_>) -> Result<u32, H264Error> {
    let mut count = 0u32;
    while b.u1()? == 0 {
        count += 1;
        if count > 63 {
            return Err(H264Error::Malformed("level_prefix too long"));
        }
    }
    Ok(count)
}

/// 📥️ One decoded residual block: raster-order (already un-zig-zagged) coefficients plus the block's
/// `total_coeff` (needed by neighboring blocks' `nC` prediction).
pub(super) struct ResidualBlock {
    pub(super) coeffs: [i32; 16],
    pub(super) total_coeff: u8,
}

/// 📥️ Decodes one CAVLC residual block (clause 9.2, 7.3.5.3.1 `residual_block_cavlc`) and un-zig-zags it in
/// the same pass. `scan` is the raster position for each scan index: [`ZIGZAG_4X4`] (16 entries) for luma
/// 4×4/`Intra16x16DCLevel`, `&ZIGZAG_4X4[1..]` (15 entries, DC excluded) for `Intra16x16ACLevel`/chroma AC, or
/// `&[0, 1, 2, 3]` for chroma DC (2×2, `nC == -1`, no real zig-zag).
pub(super) fn read_residual_block(b: &mut BitReader<'_>, nc_selector: NcSelector, scan: &[usize]) -> Result<ResidualBlock, H264Error> {
    let max_coeff = scan.len() as u8;
    let (total_coeff, trailing_ones) = match nc_selector {
        NcSelector::ChromaDc => read_coeff_token(b, &CHROMA_DC_COEFF_TOKEN)?,
        NcSelector::Nc(nc) if nc >= 8 => read_coeff_token_fixed(b)?,
        NcSelector::Nc(nc) => {
            let bucket = if nc < 2 {
                0
            } else if nc < 4 {
                1
            } else {
                2
            };
            read_coeff_token(b, &COEFF_TOKEN_TABLES[bucket])?
        }
    };
    let mut out = ResidualBlock { coeffs: [0; 16], total_coeff: total_coeff as u8 };
    if total_coeff == 0 {
        return Ok(out);
    }
    if total_coeff > u32::from(max_coeff) {
        return Err(H264Error::Malformed("total_coeff exceeds block size"));
    }

    let mut level = [0i32; 16];
    for slot in level.iter_mut().take(trailing_ones as usize) {
        *slot = if b.u1()? == 0 { 1 } else { -1 };
    }

    let mut suffix_length: u32 = if total_coeff > 10 && trailing_ones < 3 { 1 } else { 0 };
    #[allow(clippy::needless_range_loop, reason = "`i` is compared against `trailing_ones` (clause 9.2.2.1's \"first coefficient after the trailing ones\" special case) in addition to indexing `level`, so this isn't just a plain element walk")]
    for i in (trailing_ones as usize)..(total_coeff as usize) {
        let level_prefix = read_level_prefix(b)?;
        let level_suffix_size = if level_prefix == 14 && suffix_length == 0 {
            4
        } else if level_prefix >= 15 {
            level_prefix - 3
        } else {
            suffix_length
        };
        let mut level_code = level_prefix.min(15) << suffix_length;
        if level_suffix_size > 0 {
            level_code += b.u(level_suffix_size as u8)?;
        }
        if level_prefix >= 15 && suffix_length == 0 {
            level_code += 15;
        }
        if level_prefix >= 16 {
            level_code += (1u32 << (level_prefix - 3)).wrapping_sub(4096);
        }
        if i == trailing_ones as usize && trailing_ones < 3 {
            level_code += 2;
        }
        let signed = if level_code.is_multiple_of(2) { (level_code as i64 + 2) / 2 } else { -((level_code as i64 + 1) / 2) };
        level[i] = signed as i32;
        if suffix_length == 0 {
            suffix_length = 1;
        }
        if level[i].unsigned_abs() > (3u32 << (suffix_length - 1)) && suffix_length < 6 {
            suffix_length += 1;
        }
    }

    let zeros_left = if total_coeff == u32::from(max_coeff) {
        0
    } else if matches!(nc_selector, NcSelector::ChromaDc) {
        read_vlc(b, &CHROMA_DC_TOTAL_ZEROS_LEN[total_coeff as usize - 1], &CHROMA_DC_TOTAL_ZEROS_BITS[total_coeff as usize - 1])?
    } else {
        read_vlc(b, TOTAL_ZEROS_LEN[total_coeff as usize - 1], TOTAL_ZEROS_BITS[total_coeff as usize - 1])?
    };

    let mut run_before = vec![0u32; total_coeff as usize];
    let mut zeros_remaining = zeros_left;
    for slot in run_before.iter_mut().take(total_coeff as usize - 1) {
        if zeros_remaining == 0 {
            break;
        }
        let r = if zeros_remaining < 7 { read_vlc(b, RUN_BEFORE_LEN[zeros_remaining as usize - 1], RUN_BEFORE_BITS[zeros_remaining as usize - 1])? } else { read_vlc(b, RUN_BEFORE_LEN[6], RUN_BEFORE_BITS[6])? };
        *slot = r;
        zeros_remaining -= r;
    }

    let mut scan_pos = (zeros_left + total_coeff - 1) as i32;
    for i in 0..total_coeff as usize {
        if scan_pos < 0 || scan_pos as usize >= max_coeff as usize {
            return Err(H264Error::Malformed("residual block scan position out of range"));
        }
        out.coeffs[scan[scan_pos as usize]] = level[i];
        if i + 1 < total_coeff as usize {
            scan_pos -= 1 + run_before[i] as i32;
        }
    }
    Ok(out)
}

/// 🔀️ Selects which `coeff_token` VLC variant a residual block uses: the ordinary `nC`-bucketed luma/chroma-AC
/// path, or the dedicated `nC == -1` chroma-DC path.
#[derive(Clone, Copy)]
pub(super) enum NcSelector {
    Nc(u32),
    ChromaDc,
}
// #endregion 🔖️Cavlc
