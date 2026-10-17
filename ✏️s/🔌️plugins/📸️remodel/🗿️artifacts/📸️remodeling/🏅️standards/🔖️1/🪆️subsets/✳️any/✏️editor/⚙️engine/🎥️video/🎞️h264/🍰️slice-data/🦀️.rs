//! 🍰️ Slice data of the baseline H.264 decoder: the macroblock layer parsing and reconstruction of I and P slices. See [`super`].

use super::*;

// #region 🔖️SliceData
/// 🔀️ `coded_block_pattern` inverse mapping for Intra_4x4/Intra_8x8 macroblocks (Table 9-4, `chroma_format_idc == 1`).
const GOLOMB_TO_INTRA4X4_CBP: [u8; 48] = [47, 31, 15, 0, 23, 27, 29, 30, 7, 11, 13, 14, 39, 43, 45, 46, 16, 3, 5, 10, 12, 19, 21, 26, 28, 35, 37, 42, 44, 1, 2, 4, 8, 17, 18, 20, 24, 6, 9, 22, 25, 32, 33, 34, 36, 40, 38, 41];

/// 🔀️ `coded_block_pattern` inverse mapping for Inter macroblocks (Table 9-4, `chroma_format_idc == 1`).
const GOLOMB_TO_INTER_CBP: [u8; 48] = [0, 16, 1, 2, 4, 8, 32, 3, 5, 10, 12, 15, 47, 7, 11, 13, 14, 6, 9, 31, 35, 37, 42, 44, 33, 34, 36, 40, 39, 43, 45, 46, 17, 18, 20, 24, 19, 21, 26, 28, 23, 27, 29, 30, 22, 25, 38, 41];

/// 🎚️ `QPc` as a function of `qPI = Clip3(0, 51, QPY + chroma_qp_index_offset)` (clause 8.5.8, Table 8-15).
const CHROMA_QP_TABLE: [i32; 52] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 34, 35, 35, 36, 36, 37, 37, 37, 38, 38, 38, 39, 39, 39, 39];

pub(super) fn chroma_qp(luma_qp: i32, offset: i32) -> i32 {
    CHROMA_QP_TABLE[(luma_qp + offset).clamp(0, 51) as usize]
}

/// 🎛️ `te(v)`, mapped Exp-Golomb (clause 9.1): a single inverted bit when `max_val == 1`, plain `ue(v)`
/// otherwise (and always `0`, no bits consumed, when `max_val == 0`).
pub(super) fn read_te(b: &mut BitReader<'_>, max_val: u32) -> Result<u32, H264Error> {
    match max_val {
        0 => Ok(0),
        1 => Ok(1 - b.u1()?),
        _ => b.ue(),
    }
}

/// 🧭️ Luma 4×4 block-scan index `n` (0..15) → its `(blockX, blockY)` position in the macroblock's 4×4 grid
/// (clause 6.4.3): a raster scan of four 8×8 quadrants, each itself a raster scan of four 4×4 blocks.
pub(super) fn block4x4_grid_pos(n: usize) -> (u32, u32) {
    let i8x8 = n / 4;
    let i4x4 = n % 4;
    (((i8x8 % 2) * 2 + i4x4 % 2) as u32, ((i8x8 / 2) * 2 + i4x4 / 2) as u32)
}

/// 🧭️ Chroma 4×4 quadrant index `n` (0..3) → its `(blockX, blockY)` position in the 8×8 chroma block's 2×2 grid.
pub(super) fn chroma_quad_pos(n: usize) -> (u32, u32) {
    ((n % 2) as u32, (n / 2) as u32)
}

/// 🔍️ `nC` neighbor lookup for luma 4×4 blocks: the neighbor's `total_coeff` if it has already been decoded
/// (in-picture, and — since macroblocks/blocks decode in raster/scan order — necessarily earlier), else
/// `None` (unavailable, matching clause 9.2.1's substitution for out-of-picture/not-yet-decoded neighbors).
fn luma_nc(pic: &Picture, gx: i32, gy: i32) -> Option<u8> {
    if gx < 0 || gy < 0 || gx as u32 >= pic.mb_width * 4 || gy as u32 >= pic.mb_height * 4 {
        return None;
    }
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    if pic.decoded_luma4[idx] {
        Some(pic.nnz_luma[idx])
    } else {
        None
    }
}

/// 🔍️ Left/above `intra4x4_pred_mode` neighbor for clause 8.3.1.1's mode prediction; unavailable or non-
/// `Intra_4x4` neighbors substitute `2` (DC), per spec.
fn intra4x4_mode_neighbor(pic: &Picture, gx: i32, gy: i32) -> u8 {
    if gx < 0 || gy < 0 || gx as u32 >= pic.mb_width * 4 || gy as u32 >= pic.mb_height * 4 {
        return 2;
    }
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    if !pic.decoded_luma4[idx] {
        return 2;
    }
    let m = pic.intra4x4_mode[idx];
    if m < 0 {
        2
    } else {
        m as u8
    }
}

/// 🎨️ Gathers a 4×4 luma block's intra-prediction neighbors from the picture buffer (clause 6.4.11.4 /
/// 8.3.1.2.1): `top_right` unavailability substitutes the rightmost available top sample.
fn gather_intra4_neighbors(pic: &Picture, gx: u32, gy: u32) -> Intra4Neighbors {
    let (px, py) = (gx as i32 * 4, gy as i32 * 4);
    let top = pic.luma4_available(gx as i32, gy as i32 - 1).then(|| std::array::from_fn(|i| i32::from(pic.luma_at(px + i as i32, py - 1).unwrap_or(128))));
    let left = pic.luma4_available(gx as i32 - 1, gy as i32).then(|| std::array::from_fn(|i| i32::from(pic.luma_at(px - 1, py + i as i32).unwrap_or(128))));
    let corner = if pic.luma4_available(gx as i32 - 1, gy as i32 - 1) { i32::from(pic.luma_at(px - 1, py - 1).unwrap_or(128)) } else { 128 };
    let top_right_available = pic.luma4_available(gx as i32 + 1, gy as i32 - 1);
    let top_right = if top_right_available {
        std::array::from_fn(|i| i32::from(pic.luma_at(px + 4 + i as i32, py - 1).unwrap_or(128)))
    } else if let Some(t) = top {
        [t[3]; 4]
    } else {
        [128; 4]
    };
    Intra4Neighbors { top, left, top_right, corner }
}

/// 🎨️ Applies dequant + inverse transform to a raster-order coefficient block, adds `pred`, writes the
/// result into `pic`'s luma plane at 4×4 grid `(gx, gy)`, and records `total_coeff` for `nC` prediction.
fn reconstruct_luma4(pic: &mut Picture, gx: u32, gy: u32, pred: &[i32; 16], coeffs: &[i32; 16], qp: i32, total_coeff: u8) {
    let residual = idct4x4(&dequant4x4(coeffs, qp));
    let mut block = [0i32; 16];
    for i in 0..16 {
        block[i] = pred[i] + residual[i];
    }
    pic.write_luma4(gx, gy, &block);
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    pic.nnz_luma[idx] = total_coeff;
}

/// 🎨️ Intra chroma prediction (clause 8.3.4): `mode` 0=DC, 1=Horizontal, 2=Vertical, 3=Plane, applied
/// identically to `cb`/`cr`. DC is computed per 4×4 quadrant per clause 8.3.4.1's four independent rules.
fn predict_intra_chroma(plane: &[u8], stride: usize, mb_px: usize, mb_py: usize, mode: u8, top_avail: bool, left_avail: bool) -> [i32; 64] {
    let top: Option<[i32; 8]> = top_avail.then(|| std::array::from_fn(|i| i32::from(plane[(mb_py - 1) * stride + mb_px + i])));
    let left: Option<[i32; 8]> = left_avail.then(|| std::array::from_fn(|i| i32::from(plane[(mb_py + i) * stride + mb_px - 1])));
    let corner = if top_avail && left_avail { i32::from(plane[(mb_py - 1) * stride + mb_px - 1]) } else { 128 };
    let mut out = [0i32; 64];
    match mode {
        1 => {
            let l = left.unwrap_or([128; 8]);
            for y in 0..8 {
                for x in 0..8 {
                    out[y * 8 + x] = l[y];
                }
            }
        }
        2 => {
            let t = top.unwrap_or([128; 8]);
            for y in 0..8 {
                for x in 0..8 {
                    out[y * 8 + x] = t[x];
                }
            }
        }
        3 => {
            let t = top.unwrap_or([128; 8]);
            let l = left.unwrap_or([128; 8]);
            let plane_vals = plane_pred(&t, &l, corner, 8, 17, 16, 5);
            out.copy_from_slice(&plane_vals);
        }
        _ => {
            let sum4 = |s: &[i32]| (s.iter().sum::<i32>() + 2) >> 2;
            let sum8 = |t: &[i32], l: &[i32]| (t.iter().sum::<i32>() + l.iter().sum::<i32>() + 4) >> 3;
            for qy in 0..2usize {
                for qx in 0..2usize {
                    let t_half = top.map(|t| if qx == 0 { [t[0], t[1], t[2], t[3]] } else { [t[4], t[5], t[6], t[7]] });
                    let l_half = left.map(|l| if qy == 0 { [l[0], l[1], l[2], l[3]] } else { [l[4], l[5], l[6], l[7]] });
                    let prefer_top = qx == 1 && qy == 0;
                    let dc = if qx == qy {
                        match (t_half, l_half) {
                            (Some(t), Some(l)) => sum8(&t, &l),
                            (Some(t), None) => sum4(&t),
                            (None, Some(l)) => sum4(&l),
                            (None, None) => 128,
                        }
                    } else {
                        match (prefer_top, t_half, l_half) {
                            (true, Some(t), _) => sum4(&t),
                            (false, _, Some(l)) => sum4(&l),
                            (true, None, Some(l)) => sum4(&l),
                            (false, Some(t), None) => sum4(&t),
                            (_, None, None) => 128,
                        }
                    };
                    for y in 0..4 {
                        for x in 0..4 {
                            out[(qy * 4 + y) * 8 + qx * 4 + x] = dc;
                        }
                    }
                }
            }
        }
    }
    out
}

/// 📥️ Reads `ChromaDCLevel`/`ChromaACLevel` for both chroma planes' 4 quadrants each (clause 7.3.5.3.2),
/// dequantizing/inverse-transforming into per-quadrant raster residual blocks (DC already spliced into
/// position 0), and records `nC` bookkeeping. `cbp_chroma` `0` skips reading entirely (all-zero residual),
/// `1` reads DC only, `2` reads DC and AC.
#[allow(clippy::type_complexity, reason = "the return is just \"4 cb quadrant blocks, 4 cr quadrant blocks\", each already a self-explanatory raster 4x4; a wrapper struct would only rename these two fields")]
fn read_chroma_residual(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, cbp_chroma: u8, qp_cb: i32, qp_cr: i32) -> Result<([[i32; 16]; 4], [[i32; 16]; 4]), H264Error> {
    let mut cb_out = [[0i32; 16]; 4];
    let mut cr_out = [[0i32; 16]; 4];
    if cbp_chroma == 0 {
        return Ok((cb_out, cr_out));
    }
    let dc_cb = read_residual_block(b, NcSelector::ChromaDc, &[0, 1, 2, 3])?;
    let dc_cr = read_residual_block(b, NcSelector::ChromaDc, &[0, 1, 2, 3])?;
    let dc_cb_t = transform_chroma_dc(&[dc_cb.coeffs[0], dc_cb.coeffs[1], dc_cb.coeffs[2], dc_cb.coeffs[3]], qp_cb);
    let dc_cr_t = transform_chroma_dc(&[dc_cr.coeffs[0], dc_cr.coeffs[1], dc_cr.coeffs[2], dc_cr.coeffs[3]], qp_cr);

    let c4w = pic.chroma4_width();
    let mut ac_cb = [ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }];
    let mut ac_cr = [ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }];
    if cbp_chroma == 2 {
        #[allow(clippy::needless_range_loop, reason = "`n` also feeds chroma_quad_pos(n) to derive the block's grid position, not just the ac_cb index")]
        for n in 0..4usize {
            let (qx, qy) = chroma_quad_pos(n);
            let (gx, gy) = (mb_x * 2 + qx, mb_y * 2 + qy);
            let nc = predict_nc(if gx > 0 { Some(pic.nnz_cb[gy as usize * c4w + gx as usize - 1]) } else { None }, if gy > 0 { Some(pic.nnz_cb[(gy as usize - 1) * c4w + gx as usize]) } else { None });
            ac_cb[n] = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4[1..])?;
            pic.nnz_cb[gy as usize * c4w + gx as usize] = ac_cb[n].total_coeff;
        }
        #[allow(clippy::needless_range_loop, reason = "`n` also feeds chroma_quad_pos(n) to derive the block's grid position, not just the ac_cr index")]
        for n in 0..4usize {
            let (qx, qy) = chroma_quad_pos(n);
            let (gx, gy) = (mb_x * 2 + qx, mb_y * 2 + qy);
            let nc = predict_nc(if gx > 0 { Some(pic.nnz_cr[gy as usize * c4w + gx as usize - 1]) } else { None }, if gy > 0 { Some(pic.nnz_cr[(gy as usize - 1) * c4w + gx as usize]) } else { None });
            ac_cr[n] = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4[1..])?;
            pic.nnz_cr[gy as usize * c4w + gx as usize] = ac_cr[n].total_coeff;
        }
    } else {
        for n in 0..4usize {
            let (qx, qy) = chroma_quad_pos(n);
            let idx = (mb_y * 2 + qy) as usize * c4w + (mb_x * 2 + qx) as usize;
            pic.nnz_cb[idx] = 0;
            pic.nnz_cr[idx] = 0;
        }
    }

    for n in 0..4usize {
        let (qx, qy) = (n % 2, n / 2);
        let mut cb_coeffs = ac_cb[n].coeffs;
        cb_coeffs[0] = dc_cb_t[qy * 2 + qx];
        cb_out[n] = idct4x4(&dequant4x4(&cb_coeffs, qp_cb));
        let mut cr_coeffs = ac_cr[n].coeffs;
        cr_coeffs[0] = dc_cr_t[qy * 2 + qx];
        cr_out[n] = idct4x4(&dequant4x4(&cr_coeffs, qp_cr));
    }
    Ok((cb_out, cr_out))
}

/// 📥️ Decodes one `I_NxN` (`Intra_4x4`, no 8×8-transform support) macroblock: per-block prediction-mode
/// syntax, chroma pred mode, `coded_block_pattern`, optional `mb_qp_delta`, then luma/chroma residual —
/// reconstructing pixel-by-pixel in 4×4 scan order so each block's neighbors are already available.
fn decode_intra4x4_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, pps: &PpsInfo, prev_qp: &mut i32) -> Result<(), H264Error> {
    let mut modes = [0u8; 16];
    for (n, mode) in modes.iter_mut().enumerate() {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let prev_flag = b.u1()?;
        let predicted = intra4x4_mode_neighbor(pic, gx as i32 - 1, gy as i32).min(intra4x4_mode_neighbor(pic, gx as i32, gy as i32 - 1));
        let m = if prev_flag == 1 {
            predicted
        } else {
            let rem = b.u(3)? as u8;
            if rem < predicted {
                rem
            } else {
                rem + 1
            }
        };
        if m > 8 {
            return Err(H264Error::Malformed("intra4x4 pred mode out of range"));
        }
        *mode = m;
        let idx = gy as usize * pic.luma4_width() + gx as usize;
        pic.intra4x4_mode[idx] = m as i8;
    }
    let intra_chroma_pred_mode = b.ue()?;
    if intra_chroma_pred_mode > 3 {
        return Err(H264Error::Malformed("intra_chroma_pred_mode out of range"));
    }
    let cbp_code = b.ue()?;
    let cbp = *GOLOMB_TO_INTRA4X4_CBP.get(cbp_code as usize).ok_or(H264Error::Malformed("coded_block_pattern code out of range"))?;
    let (cbp_luma, cbp_chroma) = (cbp & 0xF, cbp >> 4);
    let qp = if cbp != 0 {
        let delta = b.se()?;
        let qp = (*prev_qp + delta + 52) % 52;
        *prev_qp = qp;
        qp
    } else {
        *prev_qp
    };

    #[allow(clippy::needless_range_loop, reason = "`n` is also used to derive the cbp_luma quadrant bit (n/4) and the 4x4 grid position, not just to index modes")]
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let neighbors = gather_intra4_neighbors(pic, gx, gy);
        let pred = predict_intra4x4(modes[n], &neighbors)?;
        let coded = cbp_luma & (1 << (n / 4)) != 0;
        if coded {
            let nc = predict_nc(luma_nc(pic, gx as i32 - 1, gy as i32), luma_nc(pic, gx as i32, gy as i32 - 1));
            let block = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4)?;
            reconstruct_luma4(pic, gx, gy, &pred, &block.coeffs, qp, block.total_coeff);
        } else {
            reconstruct_luma4(pic, gx, gy, &pred, &[0; 16], qp, 0);
        }
    }

    let qp_cb = chroma_qp(qp, pps.chroma_qp_index_offset);
    let qp_cr = chroma_qp(qp, pps.chroma_qp_index_offset);
    let (cb_res, cr_res) = read_chroma_residual(b, pic, mb_x, mb_y, cbp_chroma, qp_cb, qp_cr)?;
    let (mb_px, mb_py) = (mb_x as usize * 8, mb_y as usize * 8);
    let top_avail = mb_y > 0;
    let left_avail = mb_x > 0;
    let cw = pic.chroma_width();
    let cb_pred = predict_intra_chroma(&pic.cb, cw, mb_px, mb_py, intra_chroma_pred_mode as u8, top_avail, left_avail);
    let cr_pred = predict_intra_chroma(&pic.cr, cw, mb_px, mb_py, intra_chroma_pred_mode as u8, top_avail, left_avail);
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let (px, py) = (mb_px + qx as usize * 4, mb_py + qy as usize * 4);
        let mut pred_block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                pred_block[r * 4 + c] = cb_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c];
            }
        }
        let mut block = [0i32; 16];
        for i in 0..16 {
            block[i] = pred_block[i] + cb_res[n][i];
        }
        Picture::write_chroma4(&mut pic.cb, cw, px, py, &block);
        for r in 0..4 {
            for c in 0..4 {
                pred_block[r * 4 + c] = cr_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c];
            }
        }
        for i in 0..16 {
            block[i] = pred_block[i] + cr_res[n][i];
        }
        Picture::write_chroma4(&mut pic.cr, cw, px, py, &block);
    }

    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = true;
    pic.mb_qp[mbidx] = qp;
    Ok(())
}

/// 📥️ Decodes one `Intra_16x16` macroblock: prediction mode / `CodedBlockPattern` are derived directly from
/// `mb_type` (clause 7.4.5, Table 7-11), `mb_qp_delta` is always present, and the luma DC coefficients go
/// through the extra Hadamard transform before splicing into each 4×4 AC block's position 0.
fn decode_intra16x16_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, code_num: u32, pps: &PpsInfo, prev_qp: &mut i32) -> Result<(), H264Error> {
    let pred_mode = (code_num % 4) as u8;
    let cbp_chroma = ((code_num / 4) % 3) as u8;
    let cbp_luma = if code_num < 12 { 0u8 } else { 15u8 };
    let intra_chroma_pred_mode = b.ue()?;
    if intra_chroma_pred_mode > 3 {
        return Err(H264Error::Malformed("intra_chroma_pred_mode out of range"));
    }
    let delta = b.se()?;
    let qp = (*prev_qp + delta + 52) % 52;
    *prev_qp = qp;

    let (mb_px, mb_py) = (mb_x as usize * 16, mb_y as usize * 16);
    let lw = pic.luma_width();
    let top_avail = mb_y > 0;
    let left_avail = mb_x > 0;
    let top16: Option<[i32; 16]> = top_avail.then(|| std::array::from_fn(|i| i32::from(pic.luma[(mb_py - 1) * lw + mb_px + i])));
    let left16: Option<[i32; 16]> = left_avail.then(|| std::array::from_fn(|i| i32::from(pic.luma[(mb_py + i) * lw + mb_px - 1])));
    let corner = if top_avail && left_avail { i32::from(pic.luma[(mb_py - 1) * lw + mb_px - 1]) } else { 128 };
    let pred16: Vec<i32> = match pred_mode {
        0 => {
            let t = top16.ok_or(H264Error::Malformed("intra16x16 vertical needs top"))?;
            (0..256).map(|i| t[i % 16]).collect()
        }
        1 => {
            let l = left16.ok_or(H264Error::Malformed("intra16x16 horizontal needs left"))?;
            (0..256).map(|i| l[i / 16]).collect()
        }
        2 => vec![dc_pred(top16.as_ref().map(|a| a.as_slice()), left16.as_ref().map(|a| a.as_slice()), 16); 256],
        _ => plane_pred(&top16.unwrap_or([128; 16]), &left16.unwrap_or([128; 16]), corner, 16, 5, 32, 6),
    };

    let n0 = predict_nc(luma_nc(pic, mb_x as i32 * 4 - 1, mb_y as i32 * 4), luma_nc(pic, mb_x as i32 * 4, mb_y as i32 * 4 - 1));
    let dc_block = read_residual_block(b, NcSelector::Nc(n0), &ZIGZAG_4X4)?;
    let dc_transformed = transform_luma16x16_dc(&dc_block.coeffs, qp);

    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let mut pred_block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                pred_block[r * 4 + c] = pred16[(by as usize * 4 + r) * 16 + bx as usize * 4 + c];
            }
        }
        let mut coeffs = [0i32; 16];
        let total_coeff = if cbp_luma != 0 {
            let nc = predict_nc(luma_nc(pic, gx as i32 - 1, gy as i32), luma_nc(pic, gx as i32, gy as i32 - 1));
            let ac = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4[1..])?;
            coeffs = ac.coeffs;
            ac.total_coeff
        } else {
            let idx = gy as usize * pic.luma4_width() + gx as usize;
            pic.nnz_luma[idx] = 0;
            0
        };
        coeffs[0] = dc_transformed[by as usize * 4 + bx as usize];
        reconstruct_luma4(pic, gx, gy, &pred_block, &coeffs, qp, total_coeff);
    }

    let qp_cb = chroma_qp(qp, pps.chroma_qp_index_offset);
    let qp_cr = chroma_qp(qp, pps.chroma_qp_index_offset);
    let (cb_res, cr_res) = read_chroma_residual(b, pic, mb_x, mb_y, cbp_chroma, qp_cb, qp_cr)?;
    let cw = pic.chroma_width();
    let cb_pred = predict_intra_chroma(&pic.cb, cw, mb_px / 2, mb_py / 2, intra_chroma_pred_mode as u8, top_avail, left_avail);
    let cr_pred = predict_intra_chroma(&pic.cr, cw, mb_px / 2, mb_py / 2, intra_chroma_pred_mode as u8, top_avail, left_avail);
    let (cpx, cpy) = (mb_px / 2, mb_py / 2);
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let (px, py) = (cpx + qx as usize * 4, cpy + qy as usize * 4);
        let mut block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = cb_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c] + cb_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cb, cw, px, py, &block);
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = cr_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c] + cr_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cr, cw, px, py, &block);
    }

    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = true;
    pic.mb_qp[mbidx] = qp;
    Ok(())
}

/// 📥️ Decodes one `I_PCM` macroblock (clause 7.3.5): byte-aligns, then copies 256 raw luma + 64+64 raw chroma
/// samples verbatim — no prediction, transform or entropy coding. `TotalCoeff` for every 4×4 block is
/// inferred as `16` (clause 7.4.5), and `QPY` is inferred as `0`, both per spec.
fn decode_pcm_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32) -> Result<(), H264Error> {
    b.byte_align();
    let (px, py) = (mb_x as usize * 16, mb_y as usize * 16);
    let lw = pic.luma_width();
    for r in 0..16 {
        for c in 0..16 {
            pic.luma[(py + r) * lw + px + c] = b.u(8)? as u8;
        }
    }
    let (cpx, cpy) = (mb_x as usize * 8, mb_y as usize * 8);
    let cw = pic.chroma_width();
    for r in 0..8 {
        for c in 0..8 {
            pic.cb[(cpy + r) * cw + cpx + c] = b.u(8)? as u8;
        }
    }
    for r in 0..8 {
        for c in 0..8 {
            pic.cr[(cpy + r) * cw + cpx + c] = b.u(8)? as u8;
        }
    }
    let lw4 = pic.luma4_width();
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let idx = (mb_y * 4 + by) as usize * lw4 + (mb_x * 4 + bx) as usize;
        pic.decoded_luma4[idx] = true;
        pic.nnz_luma[idx] = 16;
    }
    let cw4 = pic.chroma4_width();
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let idx = (mb_y * 2 + qy) as usize * cw4 + (mb_x * 2 + qx) as usize;
        pic.nnz_cb[idx] = 16;
        pic.nnz_cr[idx] = 16;
    }
    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = true;
    pic.mb_qp[mbidx] = 0;
    Ok(())
}

/// 🔍️ `(mv, ref_idx)` at 4×4 grid `(gx, gy)`; out-of-picture and never-written (intra, or genuinely not yet
/// decoded) positions both read as `([0, 0], -1)` — the same "unavailable" substitution clause 8.4.1.3.2 uses.
fn neighbor_mv_ref(pic: &Picture, gx: i32, gy: i32) -> ([i32; 2], i8) {
    if gx < 0 || gy < 0 || gx as u32 >= pic.mb_width * 4 || gy as u32 >= pic.mb_height * 4 {
        return ([0, 0], -1);
    }
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    (pic.mv[idx], pic.ref_idx[idx])
}

/// 🔍️ The `C` (above-right) motion neighbor for a partition at 4×4 grid `(part_gx, part_gy)` sized `part_w4`
/// blocks wide, falling back to `D` (above-left) when `C` would fall in a not-yet-decoded macroblock later in
/// the current row (clause 6.4.11.7) — the one genuinely order-dependent case among A/B/C/D for the whole-
/// partition motion predictors this decoder supports (16×16/16×8/8×16/skip).
fn neighbor_c_mv_ref(pic: &Picture, mb_x: u32, mb_y: u32, part_gx: i32, part_gy: i32, part_w4: i32) -> ([i32; 2], i8) {
    let c_gx = part_gx + part_w4;
    let c_gy = part_gy - 1;
    let c_mb_row = if c_gy >= 0 { c_gy / 4 } else { -1 };
    let c_mb_col = if c_gx >= 0 { c_gx / 4 } else { -1 };
    if c_mb_row == mb_y as i32 && c_mb_col > mb_x as i32 {
        neighbor_mv_ref(pic, part_gx - 1, part_gy - 1)
    } else {
        neighbor_mv_ref(pic, c_gx, c_gy)
    }
}

/// 📥️ Decodes one `P_Skip` macroblock (implied by `mb_skip_run`, clause 7.3.4 / 8.4.1.1): no bits consumed
/// beyond the skip run itself. Motion is `(0, 0)` when either macroblock neighbor is unavailable or either
/// neighbor's own motion is `(0, 0)` against reference `0`; otherwise the usual median predictor.
fn decode_p_skip_mb(pic: &mut Picture, mb_x: u32, mb_y: u32, ref0: &Picture, qp: i32) {
    let (gx0, gy0) = (mb_x as i32 * 4, mb_y as i32 * 4);
    let (mv_a, ref_a) = neighbor_mv_ref(pic, gx0 - 1, gy0);
    let (mv_b, ref_b) = neighbor_mv_ref(pic, gx0, gy0 - 1);
    let mv = if mb_x == 0 || mb_y == 0 || (ref_a == 0 && mv_a == [0, 0]) || (ref_b == 0 && mv_b == [0, 0]) {
        [0, 0]
    } else {
        let (mv_c, ref_c) = neighbor_c_mv_ref(pic, mb_x, mb_y, gx0, gy0, 4);
        median_mv_predict((mv_a, ref_a), (mv_b, ref_b), (mv_c, ref_c), 0)
    };
    let rp = RefPlanes { luma: &ref0.luma, luma_w: ref0.luma_width() as i32, luma_h: (ref0.mb_height * 16) as i32, chroma_w: ref0.chroma_width() as i32, chroma_h: (ref0.mb_height * 8) as i32 };
    let (px, py) = (mb_x as i32 * 16, mb_y as i32 * 16);
    let mut luma_out = vec![0i32; 256];
    mc_luma_block(&rp, px, py, 16, 16, mv, &mut luma_out);
    let lw = pic.luma_width();
    for r in 0..16usize {
        for c in 0..16usize {
            pic.luma[(py as usize + r) * lw + px as usize + c] = luma_out[r * 16 + c].clamp(0, 255) as u8;
        }
    }
    let (cpx, cpy) = (mb_x as i32 * 8, mb_y as i32 * 8);
    let mut cb_out = vec![0i32; 64];
    let mut cr_out = vec![0i32; 64];
    mc_chroma_block(&ref0.cb, rp.chroma_w, rp.chroma_h, cpx, cpy, 8, 8, mv, &mut cb_out);
    mc_chroma_block(&ref0.cr, rp.chroma_w, rp.chroma_h, cpx, cpy, 8, 8, mv, &mut cr_out);
    let cw = pic.chroma_width();
    for r in 0..8usize {
        for c in 0..8usize {
            pic.cb[(cpy as usize + r) * cw + cpx as usize + c] = cb_out[r * 8 + c].clamp(0, 255) as u8;
            pic.cr[(cpy as usize + r) * cw + cpx as usize + c] = cr_out[r * 8 + c].clamp(0, 255) as u8;
        }
    }
    let lw4 = pic.luma4_width();
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let idx = (mb_y * 4 + by) as usize * lw4 + (mb_x * 4 + bx) as usize;
        pic.decoded_luma4[idx] = true;
        pic.mv[idx] = mv;
        pic.ref_idx[idx] = 0;
        pic.nnz_luma[idx] = 0;
    }
    let cw4 = pic.chroma4_width();
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let idx = (mb_y * 2 + qy) as usize * cw4 + (mb_x * 2 + qx) as usize;
        pic.nnz_cb[idx] = 0;
        pic.nnz_cr[idx] = 0;
    }
    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = false;
    pic.mb_qp[mbidx] = qp;
}

/// 📥️ Decodes one coded (non-skip) P macroblock: `P_L0_16x16`/`P_L0_L0_16x8`/`P_L0_L0_8x16` (`code_num` 0..2);
/// `code_num` 3/4 (`P_8x8`/`P_8x8ref0`, sub-8×8 partitions) are a deliberate scope cut of this decoder — see
/// crate-level notes — and fail loudly rather than misdecode.
#[allow(
    clippy::too_many_lines,
    reason = "the P-macroblock syntax genuinely has this many sequential steps (ref_idx*, mvd*, motion compensation, cbp, qp, luma residual, chroma residual) and splitting it up would just scatter tightly-coupled per-partition state across more function boundaries"
)]
#[allow(
    clippy::too_many_arguments,
    reason = "these are the macroblock's own coordinates, the always-needed parsed-header context (pps/header), the reference list, and the running QP predictor — the natural, self-describing parameter list for a single-slice macroblock decoder with no surrounding decoder-state struct"
)]
fn decode_p_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, code_num: u32, pps: &PpsInfo, header: &SliceHeaderInfo, ref_list0: &[&Picture], prev_qp: &mut i32) -> Result<(), H264Error> {
    if code_num >= 3 {
        return Err(H264Error::Unsupported("P_8x8 sub-macroblock partitions"));
    }
    let parts: [(u32, u32, u32, u32); 2] = match code_num {
        0 => [(0, 0, 4, 4), (0, 0, 0, 0)],
        1 => [(0, 0, 4, 2), (0, 2, 4, 2)],
        _ => [(0, 0, 2, 4), (2, 0, 2, 4)],
    };
    let num_parts = if code_num == 0 { 1 } else { 2 };
    let max_ref = header.num_ref_idx_l0_active.saturating_sub(1);
    let mut ref_idxs = [0i8; 2];
    for slot in ref_idxs.iter_mut().take(num_parts) {
        *slot = read_te(b, max_ref)? as i8;
    }
    let mut mvs = [[0i32; 2]; 2];
    for i in 0..num_parts {
        let (pxo, pyo, pw, ph) = parts[i];
        let (gx0, gy0) = (mb_x as i32 * 4 + pxo as i32, mb_y as i32 * 4 + pyo as i32);
        let mvd = [b.se()?, b.se()?];
        let (mv_a, ref_a) = neighbor_mv_ref(pic, gx0 - 1, gy0);
        let (mv_b, ref_b) = neighbor_mv_ref(pic, gx0, gy0 - 1);
        let (mv_c, ref_c) = neighbor_c_mv_ref(pic, mb_x, mb_y, gx0, gy0, pw as i32);
        let cur_ref = ref_idxs[i];
        let predictor = if num_parts == 2 && ph == 2 && i == 0 && ref_b == cur_ref {
            mv_b
        } else if num_parts == 2 && ((ph == 2 && i == 1) || (pw == 2 && i == 0)) && ref_a == cur_ref {
            mv_a
        } else if num_parts == 2 && pw == 2 && i == 1 && ref_c == cur_ref {
            mv_c
        } else {
            median_mv_predict((mv_a, ref_a), (mv_b, ref_b), (mv_c, ref_c), cur_ref)
        };
        let mv = [predictor[0] + mvd[0], predictor[1] + mvd[1]];
        mvs[i] = mv;
        let lw4 = pic.luma4_width();
        for dy in 0..ph {
            for dx in 0..pw {
                let idx = (gy0 as u32 + dy) as usize * lw4 + (gx0 as u32 + dx) as usize;
                pic.mv[idx] = mv;
                pic.ref_idx[idx] = cur_ref;
            }
        }
    }

    let cbp_code = b.ue()?;
    let cbp = *GOLOMB_TO_INTER_CBP.get(cbp_code as usize).ok_or(H264Error::Malformed("coded_block_pattern code out of range"))?;
    let (cbp_luma, cbp_chroma) = (cbp & 0xF, cbp >> 4);
    let qp = if cbp != 0 {
        let delta = b.se()?;
        let qp = (*prev_qp + delta + 52) % 52;
        *prev_qp = qp;
        qp
    } else {
        *prev_qp
    };

    let lw = pic.luma_width();
    let cw = pic.chroma_width();
    for i in 0..num_parts {
        let (pxo, pyo, pw, ph) = parts[i];
        let ref_pic = *ref_list0.get(ref_idxs[i] as usize).ok_or(H264Error::Malformed("p macroblock ref_idx out of range"))?;
        let rp = RefPlanes { luma: &ref_pic.luma, luma_w: ref_pic.luma_width() as i32, luma_h: (ref_pic.mb_height * 16) as i32, chroma_w: ref_pic.chroma_width() as i32, chroma_h: (ref_pic.mb_height * 8) as i32 };
        let (ppx, ppy) = ((mb_x * 16 + pxo * 4) as i32, (mb_y * 16 + pyo * 4) as i32);
        let (pw_px, ph_px) = ((pw * 4) as usize, (ph * 4) as usize);
        let mut out = vec![0i32; pw_px * ph_px];
        mc_luma_block(&rp, ppx, ppy, pw_px, ph_px, mvs[i], &mut out);
        for r in 0..ph_px {
            for c in 0..pw_px {
                pic.luma[(ppy as usize + r) * lw + ppx as usize + c] = out[r * pw_px + c].clamp(0, 255) as u8;
            }
        }
        let (cpx, cpy) = ((mb_x * 8 + pxo * 2) as i32, (mb_y * 8 + pyo * 2) as i32);
        let (cw_px, ch_px) = ((pw * 2) as usize, (ph * 2) as usize);
        let mut cb_out = vec![0i32; cw_px * ch_px];
        let mut cr_out = vec![0i32; cw_px * ch_px];
        mc_chroma_block(&ref_pic.cb, rp.chroma_w, rp.chroma_h, cpx, cpy, cw_px, ch_px, mvs[i], &mut cb_out);
        mc_chroma_block(&ref_pic.cr, rp.chroma_w, rp.chroma_h, cpx, cpy, cw_px, ch_px, mvs[i], &mut cr_out);
        for r in 0..ch_px {
            for c in 0..cw_px {
                pic.cb[(cpy as usize + r) * cw + cpx as usize + c] = cb_out[r * cw_px + c].clamp(0, 255) as u8;
                pic.cr[(cpy as usize + r) * cw + cpx as usize + c] = cr_out[r * cw_px + c].clamp(0, 255) as u8;
            }
        }
    }

    let lw4 = pic.luma4_width();
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let idx = gy as usize * lw4 + gx as usize;
        pic.decoded_luma4[idx] = true;
        if cbp_luma & (1 << (n / 4)) != 0 {
            let nc = predict_nc(luma_nc(pic, gx as i32 - 1, gy as i32), luma_nc(pic, gx as i32, gy as i32 - 1));
            let block = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4)?;
            let residual = idct4x4(&dequant4x4(&block.coeffs, qp));
            let mut px_block = [0i32; 16];
            for r in 0..4 {
                for c in 0..4 {
                    px_block[r * 4 + c] = i32::from(pic.luma[(gy as usize * 4 + r) * lw + gx as usize * 4 + c]) + residual[r * 4 + c];
                }
            }
            pic.write_luma4(gx, gy, &px_block);
            pic.nnz_luma[idx] = block.total_coeff;
        } else {
            pic.nnz_luma[idx] = 0;
        }
    }

    let qp_cb = chroma_qp(qp, pps.chroma_qp_index_offset);
    let qp_cr = chroma_qp(qp, pps.chroma_qp_index_offset);
    let (cb_res, cr_res) = read_chroma_residual(b, pic, mb_x, mb_y, cbp_chroma, qp_cb, qp_cr)?;
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let (px, py) = (mb_x as usize * 8 + qx as usize * 4, mb_y as usize * 8 + qy as usize * 4);
        let mut block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = i32::from(pic.cb[(py + r) * cw + px + c]) + cb_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cb, cw, px, py, &block);
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = i32::from(pic.cr[(py + r) * cw + px + c]) + cr_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cr, cw, px, py, &block);
    }

    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = false;
    pic.mb_qp[mbidx] = qp;
    Ok(())
}

/// 🔀️ Dispatches one coded macroblock by `mb_type`: P slices see `0..5` as P partition types and `5..` as the
/// I-macroblock table shifted by 5; I slices see the I-macroblock table directly.
#[allow(clippy::too_many_arguments, reason = "same rationale as decode_p_mb, which this just dispatches to alongside the intra macroblock decoders")]
fn decode_macroblock(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, pps: &PpsInfo, header: &SliceHeaderInfo, is_p_slice: bool, ref_list0: &[&Picture], prev_qp: &mut i32) -> Result<(), H264Error> {
    let mb_type = b.ue()?;
    let i_type = if is_p_slice {
        if mb_type < 5 {
            return decode_p_mb(b, pic, mb_x, mb_y, mb_type, pps, header, ref_list0, prev_qp);
        }
        mb_type - 5
    } else {
        mb_type
    };
    match i_type {
        0 => decode_intra4x4_mb(b, pic, mb_x, mb_y, pps, prev_qp),
        1..=24 => decode_intra16x16_mb(b, pic, mb_x, mb_y, i_type - 1, pps, prev_qp),
        25 => decode_pcm_mb(b, pic, mb_x, mb_y),
        _ => Err(H264Error::Malformed("mb_type out of range")),
    }
}

/// 📥️ Decodes `slice_data()` (clause 7.3.4): the `mb_skip_run`-driven loop for P slices, dispatching each
/// coded macroblock via [`decode_macroblock`]. Only a single slice per picture is supported (this decoder
/// requires `first_mb_in_slice == 0` and consumes macroblocks until the whole picture is covered).
pub(super) fn decode_slice_data(b: &mut BitReader<'_>, pic: &mut Picture, sps: &SpsInfo, pps: &PpsInfo, header: &SliceHeaderInfo, ref_list0: &[&Picture]) -> Result<(), H264Error> {
    let total_mbs = sps.pic_width_in_mbs * sps.pic_height_in_mbs;
    let is_p_slice = header.slice_type_mod5 == 0;
    let mut mb_addr = header.first_mb_in_slice;
    let mut prev_qp = header.slice_qp;
    loop {
        if mb_addr >= total_mbs {
            break;
        }
        if is_p_slice {
            let skip_run = b.ue()?;
            for _ in 0..skip_run {
                if mb_addr >= total_mbs {
                    return Err(H264Error::Malformed("mb_skip_run exceeds picture"));
                }
                let ref0 = *ref_list0.first().ok_or(H264Error::Malformed("p slice has no reference picture"))?;
                decode_p_skip_mb(pic, mb_addr % sps.pic_width_in_mbs, mb_addr / sps.pic_width_in_mbs, ref0, prev_qp);
                mb_addr += 1;
            }
            if mb_addr >= total_mbs {
                break;
            }
            if !b.more_rbsp_data() {
                break;
            }
        }
        decode_macroblock(b, pic, mb_addr % sps.pic_width_in_mbs, mb_addr / sps.pic_width_in_mbs, pps, header, is_p_slice, ref_list0, &mut prev_qp)?;
        mb_addr += 1;
        if !b.more_rbsp_data() {
            break;
        }
    }
    if mb_addr != total_mbs {
        return Err(H264Error::Malformed("slice did not cover the whole picture"));
    }
    Ok(())
}
// #endregion 🔖️SliceData
