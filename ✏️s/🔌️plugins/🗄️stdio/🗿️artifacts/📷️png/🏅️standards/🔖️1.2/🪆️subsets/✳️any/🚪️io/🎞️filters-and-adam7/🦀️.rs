//! 🎞️ Scanline filters and interlacing of the PNG codec: the Paeth predictor, the row filters and defilters, the filter choice and the Adam7 pass geometry. See [`super`].

use super::*;

//#region Filter
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paeth(a: u8, b: u8, c: u8) -> u8 {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn defilter_row(filter_type: u8, filt: &[u8], prev: Option<&[u8]>, bpp: usize) -> Result<Vec<u8>, String> {
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

/// 🧮 Minimum-sum-of-absolute-values heuristic (bytes read as signed), the common
/// real-world choice per PNG spec §9.8 — not optimal, but genuinely per-scanline-adaptive.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn choose_filter_controlled(cur:&[u8],prev:Option<&[u8]>,bpp:usize,control:&mut NativeEncodeControl<'_>)->Result<(u8,Vec<u8>),ValueError> {
    let mut best_filter=0;let mut best_sum=u64::MAX;let mut best=Vec::new();
    for filter in 0..=4 {
        let mut output=control.allocate_vec(cur.len())?;let mut sum=0;control.begin_stage(cur.len())?;
        for x in 0..cur.len() {let a=if x>=bpp {cur[x-bpp]}else{0};let b=prev.map_or(0,|p|p[x]);let c=if x>=bpp {prev.map_or(0,|p|p[x-bpp])}else{0};let byte=match filter {0=>cur[x],1=>cur[x].wrapping_sub(a),2=>cur[x].wrapping_sub(b),3=>cur[x].wrapping_sub(((u16::from(a)+u16::from(b))/2)as u8),_=>cur[x].wrapping_sub(paeth(a,b,c))};output.push(byte);sum+=u64::from((byte as i8).unsigned_abs());control.step()?;}
        if sum<best_sum {best=output;best_sum=sum;best_filter=filter;}
    }
    Ok((best_filter,best))
}

fn defilter_row_controlled(filter:u8,filt:&[u8],prev:Option<&[u8]>,bpp:usize,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,PngReadError> {
    if filter>4 {return Err("png: unsupported filter type".into());}let mut output=control.allocate_vec(filt.len())?;control.begin_stage(filt.len())?;
    for x in 0..filt.len() {let a=if x>=bpp {output[x-bpp]}else{0};let b=prev.map_or(0,|p|p[x]);let c=if x>=bpp {prev.map_or(0,|p|p[x-bpp])}else{0};let byte=match filter {0=>filt[x],1=>filt[x].wrapping_add(a),2=>filt[x].wrapping_add(b),3=>filt[x].wrapping_add(((u16::from(a)+u16::from(b))/2)as u8),_=>filt[x].wrapping_add(paeth(a,b,c))};output.push(byte);control.step()?;}Ok(output)
}

pub(super) fn defilter_pass(raw: &[u8], mut pos: usize, height: u32, row_bytes: usize, bpp: usize, control:&mut NativeDecodeControl<'_>) -> Result<(Vec<Vec<u8>>, usize), PngReadError> {
    let mut rows:Vec<Vec<u8>>=control.allocate_vec(height as usize)?;
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
        let recon=defilter_row_controlled(ft,filt,rows.last().map(Vec::as_slice),bpp,control)?;
        rows.push(recon);
    }
    Ok((rows, pos))
}
//#endregion Filter

//#region Adam7
/// 🪜 Pass geometry `(start_x, start_y, step_x, step_y)`, PNG spec §8.2.
pub const ADAM7: [(u32, u32, u32, u32); 7] = [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn adam7_pass_dims(width: u32, height: u32, pass: usize) -> (u32, u32) {
    let (sx, sy, stx, sty) = ADAM7[pass];
    let w = if width > sx { (width - sx).div_ceil(stx) } else { 0 };
    let h = if height > sy { (height - sy).div_ceil(sty) } else { 0 };
    (w, h)
}
//#endregion Adam7
