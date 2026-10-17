//! 🧮️ PNG filters and samples of the pixels codec: the Paeth predictor, the row filters and defilters, the Adam7 pass geometry and the sample unpacking to RGBA. See [`super`].

use super::*;

//#region PngFilter
pub(super) fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i32, b as i32, c as i32);
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}

fn filter_row(filter_type: u8, cur: &[u8], prev: Option<&[u8]>, bpp: usize) -> Vec<u8> {
    let mut out = vec![0u8; cur.len()];
    for x in 0..cur.len() {
        let a = if x >= bpp { cur[x - bpp] } else { 0 };
        let b = prev.map_or(0, |p| p[x]);
        let c = if x >= bpp { prev.map_or(0, |p| p[x - bpp]) } else { 0 };
        out[x] = match filter_type {
            0 => cur[x],
            1 => cur[x].wrapping_sub(a),
            2 => cur[x].wrapping_sub(b),
            3 => cur[x].wrapping_sub(((a as u16 + b as u16) / 2) as u8),
            4 => cur[x].wrapping_sub(paeth(a, b, c)),
            _ => unreachable!("caller only passes 0..=4"),
        };
    }
    out
}

pub(super) fn defilter_row(filter_type: u8, filt: &[u8], prev: Option<&[u8]>, bpp: usize) -> Result<Vec<u8>, String> {
    if filter_type > 4 {
        return Err(format!("png: unsupported filter type {filter_type}"));
    }
    let mut out = vec![0u8; filt.len()];
    for x in 0..filt.len() {
        let a = if x >= bpp { out[x - bpp] } else { 0 };
        let b = prev.map_or(0, |p| p[x]);
        let c = if x >= bpp { prev.map_or(0, |p| p[x - bpp]) } else { 0 };
        out[x] = match filter_type {
            0 => filt[x],
            1 => filt[x].wrapping_add(a),
            2 => filt[x].wrapping_add(b),
            3 => filt[x].wrapping_add(((a as u16 + b as u16) / 2) as u8),
            4 => filt[x].wrapping_add(paeth(a, b, c)),
            _ => unreachable!("checked above"),
        };
    }
    Ok(out)
}

/// 🧮️ Minimum-sum-of-absolute-values heuristic (bytes read as signed), the common real-world
/// choice per PNG spec §9.8 — not optimal, but genuinely per-scanline-adaptive.
pub(super) fn choose_filter(cur: &[u8], prev: Option<&[u8]>, bpp: usize) -> (u8, Vec<u8>) {
    let mut best_ft = 0u8;
    let mut best_sum = i64::MAX;
    let mut best = Vec::new();
    for ft in 0u8..=4 {
        let f = filter_row(ft, cur, prev, bpp);
        let sum: i64 = f.iter().map(|&b| (b as i8).unsigned_abs() as i64).sum();
        if sum < best_sum {
            best_sum = sum;
            best_ft = ft;
            best = f;
        }
    }
    (best_ft, best)
}

pub(super) fn defilter_pass(raw: &[u8], mut pos: usize, height: u32, row_bytes: usize, bpp: usize) -> Result<(Vec<Vec<u8>>, usize), String> {
    let mut rows = Vec::with_capacity(height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for _ in 0..height {
        if pos >= raw.len() {
            return Err("png: truncated scanline data".into());
        }
        let ft = raw[pos];
        pos += 1;
        if pos + row_bytes > raw.len() {
            return Err("png: truncated scanline data".into());
        }
        let filt = &raw[pos..pos + row_bytes];
        pos += row_bytes;
        let recon = defilter_row(ft, filt, prev.as_deref(), bpp)?;
        prev = Some(recon.clone());
        rows.push(recon);
    }
    Ok((rows, pos))
}
//#endregion PngFilter

//#region PngAdam7
/// 🪜️ Pass geometry `(start_x, start_y, step_x, step_y)`, PNG spec §8.2.
pub(super) const ADAM7: [(u32, u32, u32, u32); 7] = [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)];

pub(super) fn adam7_pass_dims(width: u32, height: u32, pass: usize) -> (u32, u32) {
    let (sx, sy, stx, sty) = ADAM7[pass];
    let w = if width > sx { (width - sx).div_ceil(stx) } else { 0 };
    let h = if height > sy { (height - sy).div_ceil(sty) } else { 0 };
    (w, h)
}
//#endregion PngAdam7

//#region PngUnpack
pub(super) fn unpack_samples(row: &[u8], width: usize, spp: usize, bit_depth: u8) -> Vec<u32> {
    let count = width * spp;
    let mut out = Vec::with_capacity(count);
    if bit_depth == 16 {
        for i in 0..count {
            out.push(((row[i * 2] as u32) << 8) | row[i * 2 + 1] as u32);
        }
    } else if bit_depth == 8 {
        out.extend(row[..count].iter().map(|&sample| u32::from(sample)));
    } else {
        let mut bitpos = 0usize;
        for _ in 0..count {
            let mut v = 0u32;
            for _ in 0..bit_depth {
                let byte = row[bitpos / 8];
                let bit = (byte >> (7 - (bitpos % 8))) & 1;
                v = (v << 1) | bit as u32;
                bitpos += 1;
            }
            out.push(v);
        }
    }
    out
}

fn scale_to_8(sample: u32, bit_depth: u8) -> u8 {
    match bit_depth {
        8 => sample as u8,
        16 => (sample >> 8) as u8,
        _ => {
            let maxval = (1u32 << bit_depth) - 1;
            ((sample * 255 + maxval / 2) / maxval) as u8
        }
    }
}

/// 🎨️ Converts one pixel's raw (unscaled) samples to 8-bit RGBA using PLTE/tRNS as needed.
pub(super) fn pixel_to_rgba(samples: &[u32], ihdr: &Ihdr, palette: &[[u8; 3]], palette_alpha: &[u8], gray_trans: Option<u32>, rgb_trans: Option<(u32, u32, u32)>) -> Result<[u8; 4], String> {
    match ihdr.color_type {
        0 => {
            let g = samples[0];
            let a = if gray_trans == Some(g) { 0 } else { 255 };
            let g8 = scale_to_8(g, ihdr.bit_depth);
            Ok([g8, g8, g8, a])
        }
        2 => {
            let (r, g, b) = (samples[0], samples[1], samples[2]);
            let a = if rgb_trans == Some((r, g, b)) { 0 } else { 255 };
            Ok([scale_to_8(r, ihdr.bit_depth), scale_to_8(g, ihdr.bit_depth), scale_to_8(b, ihdr.bit_depth), a])
        }
        3 => {
            let idx = samples[0] as usize;
            let rgb = palette.get(idx).ok_or_else(|| format!("png: palette index {idx} out of range"))?;
            let a = palette_alpha.get(idx).copied().unwrap_or(255);
            Ok([rgb[0], rgb[1], rgb[2], a])
        }
        4 => {
            let g8 = scale_to_8(samples[0], ihdr.bit_depth);
            let a8 = scale_to_8(samples[1], ihdr.bit_depth);
            Ok([g8, g8, g8, a8])
        }
        6 => Ok([scale_to_8(samples[0], ihdr.bit_depth), scale_to_8(samples[1], ihdr.bit_depth), scale_to_8(samples[2], ihdr.bit_depth), scale_to_8(samples[3], ihdr.bit_depth)]),
        _ => unreachable!("validated in parse_ihdr"),
    }
}
//#endregion PngUnpack
