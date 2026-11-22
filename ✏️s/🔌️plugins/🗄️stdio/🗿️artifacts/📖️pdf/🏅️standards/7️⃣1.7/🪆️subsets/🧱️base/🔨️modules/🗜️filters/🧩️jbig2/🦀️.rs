//! 🧩️ JBIG2 decoding for `/JBIG2Decode` images: generic, refinement, symbol dictionary and text regions.
//! [ITU-T T.88](https://www.itu.int/rec/T-REC-T.88)

use super::arithmetic::{Context, MqDecoder};
use super::ccitt::{self, CcittParameters};
use super::{Control, RasterError};

const MAX_PIXELS: u64 = 1 << 28;

/// 🧩️ A decoded bi-level page; one byte per pixel, 1 is black.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Bitmap {
    fn new(width: usize, height: usize, fill: u8) -> Result<Self, RasterError> {
        if (width as u64) * (height as u64) > MAX_PIXELS { return Err(RasterError::TooLarge); }
        Ok(Self { width, height, pixels: vec![fill; width * height] })
    }

    fn get(&self, x: i64, y: i64) -> u8 {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 { 0 } else { self.pixels[y as usize * self.width + x as usize] }
    }

    fn combine(&mut self, source: &Bitmap, x0: i64, y0: i64, operator: u8) {
        for y in 0..source.height as i64 {
            for x in 0..source.width as i64 {
                let (tx, ty) = (x0 + x, y0 + y);
                if tx < 0 || ty < 0 || tx >= self.width as i64 || ty >= self.height as i64 { continue; }
                let at = ty as usize * self.width + tx as usize;
                let (d, s) = (self.pixels[at], source.pixels[y as usize * source.width + x as usize]);
                self.pixels[at] = match operator { 0 => d | s, 1 => d & s, 2 => d ^ s, 3 => 1 - (d ^ s), _ => s };
            }
        }
    }

    /// 📦️ Packs rows into bytes; `black_is_zero` follows the PDF convention for `/JBIG2Decode` output.
    pub fn packed(&self, black_is_zero: bool) -> Vec<u8> {
        let stride = self.width.div_ceil(8);
        let mut out = vec![if black_is_zero { 0xFF } else { 0 }; stride * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.pixels[y * self.width + x] == 1 {
                    let mask = 0x80u8 >> (x & 7);
                    if black_is_zero { out[y * stride + x / 8] &= !mask } else { out[y * stride + x / 8] |= mask }
                }
            }
        }
        out
    }
}

struct IntContexts(Vec<Context>);

impl IntContexts {
    fn new() -> Self { Self(vec![Context::default(); 512]) }
}

fn decode_int(mq: &mut MqDecoder<'_>, contexts: &mut IntContexts) -> Option<i32> {
    let mut previous = 1usize;
    let mut bit = |mq: &mut MqDecoder<'_>| {
        let value = mq.bit(&mut contexts.0[previous]) as usize;
        previous = if previous < 256 { previous << 1 | value } else { ((previous << 1 | value) & 511) | 256 };
        value
    };
    let sign = bit(mq);
    let (count, offset) = if bit(mq) == 0 { (2, 0) } else if bit(mq) == 0 { (4, 4) } else if bit(mq) == 0 { (6, 20) } else if bit(mq) == 0 { (8, 84) } else if bit(mq) == 0 { (12, 340) } else { (32, 4436) };
    let mut value = 0i64;
    for _ in 0..count { value = value << 1 | bit(mq) as i64; }
    let value = value + offset;
    if sign == 1 { if value == 0 { None } else { Some(-(value.min(i64::from(i32::MAX)) as i32)) } } else { Some(value.min(i64::from(i32::MAX)) as i32) }
}

fn decode_id(mq: &mut MqDecoder<'_>, contexts: &mut [Context], length: u32) -> usize {
    let mut previous = 1usize;
    for _ in 0..length { previous = previous << 1 | mq.bit(&mut contexts[previous]) as usize; }
    previous - (1 << length)
}

/// 🧩️ Decodes an arithmetic-coded generic region (T.88 §6.2).
fn decode_generic(mq: &mut MqDecoder<'_>, contexts: &mut [Context], width: usize, height: usize, template: u8, typical: bool, at: &[(i8, i8)], skip: Option<&Bitmap>) -> Result<Bitmap, RasterError> {
    let mut bitmap = Bitmap::new(width, height, 0)?;
    let fixed: &[(i64, i64, u32)] = match template {
        0 => &[(-1, -2, 14), (0, -2, 13), (1, -2, 12), (-2, -1, 9), (-1, -1, 8), (0, -1, 7), (1, -1, 6), (2, -1, 5), (-4, 0, 3), (-3, 0, 2), (-2, 0, 1), (-1, 0, 0)],
        1 => &[(-1, -2, 12), (0, -2, 11), (1, -2, 10), (2, -2, 9), (-2, -1, 8), (-1, -1, 7), (0, -1, 6), (1, -1, 5), (2, -1, 4), (-3, 0, 2), (-2, 0, 1), (-1, 0, 0)],
        2 => &[(-1, -2, 9), (0, -2, 8), (1, -2, 7), (-2, -1, 6), (-1, -1, 5), (0, -1, 4), (1, -1, 3), (-2, 0, 1), (-1, 0, 0)],
        _ => &[(-3, -1, 9), (-2, -1, 8), (-1, -1, 7), (0, -1, 6), (1, -1, 5), (-4, 0, 3), (-3, 0, 2), (-2, 0, 1), (-1, 0, 0)],
    };
    let at_bits: &[u32] = match template { 0 => &[4, 10, 11, 15], 1 => &[3], 2 => &[2], _ => &[4] };
    let pseudo = [0x9B25usize, 0x0795, 0x00E5, 0x0195][template as usize];
    let mut ltp = 0u8;
    for y in 0..height as i64 {
        if typical {
            ltp ^= mq.bit(&mut contexts[pseudo]);
            if ltp == 1 {
                if y > 0 { let (above, current) = bitmap.pixels.split_at_mut(y as usize * width); current[..width].copy_from_slice(&above[(y as usize - 1) * width..]); }
                continue;
            }
        }
        for x in 0..width as i64 {
            if skip.is_some_and(|mask| mask.get(x, y) == 1) { continue; }
            let mut label = 0usize;
            for &(dx, dy, bit) in fixed { label |= (bitmap.get(x + dx, y + dy) as usize) << bit; }
            for (&(dx, dy), &bit) in at.iter().zip(at_bits) { label |= (bitmap.get(x + i64::from(dx), y + i64::from(dy)) as usize) << bit; }
            bitmap.pixels[y as usize * width + x as usize] = mq.bit(&mut contexts[label]);
        }
    }
    Ok(bitmap)
}

/// 🧩️ Decodes a generic refinement region (T.88 §6.3).
fn decode_refinement(mq: &mut MqDecoder<'_>, contexts: &mut [Context], width: usize, height: usize, template: u8, reference: &Bitmap, dx: i64, dy: i64, typical: bool, at: &[(i8, i8)]) -> Result<Bitmap, RasterError> {
    let mut bitmap = Bitmap::new(width, height, 0)?;
    let (mut coding, mut refs): (Vec<(i64, i64)>, Vec<(i64, i64)>) = if template == 0 {
        (vec![(0, -1), (1, -1), (-1, 0)], vec![(0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)])
    } else {
        (vec![(-1, -1), (0, -1), (1, -1), (-1, 0)], vec![(0, -1), (-1, 0), (0, 0), (1, 0), (0, 1), (1, 1)])
    };
    if template == 0 {
        coding.push((i64::from(at[0].0), i64::from(at[0].1)));
        refs.push((i64::from(at[1].0), i64::from(at[1].1)));
    }
    let pseudo = if template == 0 { 0x0020usize } else { 0x0004 };
    let mut ltp = 0u8;
    for y in 0..height as i64 {
        if typical { ltp ^= mq.bit(&mut contexts[pseudo]); }
        for x in 0..width as i64 {
            let (rx, ry) = (x - dx, y - dy);
            if ltp == 1 {
                let centre = reference.get(rx, ry);
                if (-1..=1).all(|j| (-1..=1).all(|i| reference.get(rx + i, ry + j) == centre)) {
                    bitmap.pixels[y as usize * width + x as usize] = centre;
                    continue;
                }
            }
            let mut label = 0usize;
            for &(ox, oy) in &coding { label = label << 1 | bitmap.get(x + ox, y + oy) as usize; }
            for &(ox, oy) in &refs { label = label << 1 | reference.get(rx + ox, ry + oy) as usize; }
            bitmap.pixels[y as usize * width + x as usize] = mq.bit(&mut contexts[label]);
        }
    }
    Ok(bitmap)
}

struct Segment<'a> {
    number: u32,
    kind: u8,
    referred: Vec<u32>,
    data: &'a [u8],
}

fn u32_at(data: &[u8], at: usize) -> Result<u32, RasterError> {
    data.get(at..at + 4).map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])).ok_or(RasterError::Malformed("truncated JBIG2 segment"))
}

fn u16_at(data: &[u8], at: usize) -> Result<u16, RasterError> {
    data.get(at..at + 2).map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]])).ok_or(RasterError::Malformed("truncated JBIG2 segment"))
}

fn segments(data: &[u8]) -> Result<Vec<Segment<'_>>, RasterError> {
    let mut out = Vec::new();
    let mut position = 0usize;
    while position + 11 <= data.len() {
        let number = u32_at(data, position)?;
        let flags = data[position + 4];
        let mut cursor = position + 5;
        let first = *data.get(cursor).ok_or(RasterError::Malformed("truncated JBIG2 header"))?;
        let count = if first >> 5 == 7 {
            let long = u32_at(data, cursor)? & 0x1FFF_FFFF;
            cursor += 4 + (long as usize + 1).div_ceil(8);
            long as usize
        } else {
            cursor += 1;
            (first >> 5) as usize
        };
        let size = if number <= 256 { 1 } else if number <= 65536 { 2 } else { 4 };
        let mut referred = Vec::with_capacity(count);
        for _ in 0..count {
            referred.push(match size { 1 => u32::from(*data.get(cursor).ok_or(RasterError::Malformed("truncated JBIG2 header"))?), 2 => u32::from(u16_at(data, cursor)?), _ => u32_at(data, cursor)? });
            cursor += size;
        }
        cursor += if flags & 0x40 != 0 { 4 } else { 1 };
        let length = u32_at(data, cursor)? as usize;
        cursor += 4;
        let end = if length == 0xFFFF_FFFF { data.len() } else { (cursor + length).min(data.len()) };
        out.push(Segment { number, kind: flags & 0x3F, referred, data: &data[cursor..end] });
        position = end;
    }
    Ok(out)
}

struct Region { width: usize, height: usize, x: i64, y: i64, operator: u8 }

fn region_info(data: &[u8]) -> Result<Region, RasterError> {
    Ok(Region { width: u32_at(data, 0)? as usize, height: u32_at(data, 4)? as usize, x: i64::from(u32_at(data, 8)?), y: i64::from(u32_at(data, 12)?), operator: data.get(16).copied().ok_or(RasterError::Malformed("truncated JBIG2 region"))? & 7 })
}

fn at_pairs(data: &[u8], at: usize, count: usize) -> Result<Vec<(i8, i8)>, RasterError> {
    let bytes = data.get(at..at + 2 * count).ok_or(RasterError::Malformed("truncated JBIG2 adaptive template"))?;
    Ok(bytes.chunks_exact(2).map(|pair| (pair[0] as i8, pair[1] as i8)).collect())
}

fn generic_region(data: &[u8]) -> Result<(Region, Bitmap), RasterError> {
    let region = region_info(data)?;
    let flags = *data.get(17).ok_or(RasterError::Malformed("truncated JBIG2 region"))?;
    let (mmr, template, typical) = (flags & 1 == 1, (flags >> 1) & 3, flags & 8 != 0);
    if mmr {
        let packed = ccitt::decode(&data[18..], &CcittParameters { k: -1, columns: region.width as u32, rows: region.height as u32, black_is_1: true, byte_align: false }, &mut |_| true)?;
        let mut bitmap = Bitmap::new(region.width, region.height, 0)?;
        let stride = region.width.div_ceil(8);
        for y in 0..region.height { for x in 0..region.width { bitmap.pixels[y * region.width + x] = packed[y * stride + x / 8] >> (7 - (x & 7)) & 1; } }
        return Ok((region, bitmap));
    }
    let at_count = if template == 0 { 4 } else { 1 };
    let at = at_pairs(data, 18, at_count)?;
    let mut mq = MqDecoder::new(&data[18 + 2 * at_count..]);
    let mut contexts = vec![Context::default(); 1 << 16];
    let bitmap = decode_generic(&mut mq, &mut contexts, region.width, region.height, template, typical, &at, None)?;
    Ok((region, bitmap))
}

struct Dictionary { exported: Vec<Bitmap> }

fn refinement_region(data: &[u8], page: Option<&Bitmap>) -> Result<(Region, Bitmap), RasterError> {
    let region = region_info(data)?;
    let flags = *data.get(17).ok_or(RasterError::Malformed("truncated JBIG2 region"))?;
    let (template, typical) = (flags & 1, flags & 2 != 0);
    let at_count = if template == 0 { 2 } else { 0 };
    let at = if at_count == 2 { at_pairs(data, 18, 2)? } else { vec![(-1, -1), (-1, -1)] };
    let page = page.ok_or(RasterError::Malformed("JBIG2 refinement region without a page"))?;
    let mut reference = Bitmap::new(region.width, region.height, 0)?;
    for y in 0..region.height { for x in 0..region.width { reference.pixels[y * region.width + x] = page.get(region.x + x as i64, region.y + y as i64); } }
    let mut mq = MqDecoder::new(&data[18 + 2 * at_count..]);
    let mut contexts = vec![Context::default(); 1 << 13];
    let bitmap = decode_refinement(&mut mq, &mut contexts, region.width, region.height, template, &reference, 0, 0, typical, &at)?;
    Ok((region, bitmap))
}

fn symbol_dictionary(segment: &Segment<'_>, inputs: Vec<Bitmap>) -> Result<Dictionary, RasterError> {
    let data = segment.data;
    let flags = u16_at(data, 0)?;
    if flags & 1 == 1 { return Err(RasterError::Unsupported("Huffman-coded JBIG2 symbol dictionaries")); }
    let (refagg, template, rtemplate) = (flags & 2 != 0, ((flags >> 10) & 3) as u8, ((flags >> 12) & 1) as u8);
    let at_count = if template == 0 { 4 } else { 1 };
    let at = at_pairs(data, 2, at_count)?;
    let mut cursor = 2 + 2 * at_count;
    let rat = if refagg && rtemplate == 0 { let pairs = at_pairs(data, cursor, 2)?; cursor += 4; pairs } else { vec![(-1, -1), (-1, -1)] };
    let (export_count, new_count) = (u32_at(data, cursor)? as usize, u32_at(data, cursor + 4)? as usize);
    cursor += 8;
    if new_count > 1 << 20 { return Err(RasterError::TooLarge); }
    let mut mq = MqDecoder::new(&data[cursor..]);
    let (mut dh, mut dw, mut ex, mut ai, mut rdx, mut rdy) = (IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new());
    let code_length = ((inputs.len() + new_count).max(2) as f64).log2().ceil() as u32;
    let mut id_contexts = vec![Context::default(); 1 << (code_length + 1)];
    let mut generic = vec![Context::default(); 1 << 16];
    let mut refinement = vec![Context::default(); 1 << 13];
    let mut symbols: Vec<Bitmap> = inputs;
    let input_count = symbols.len();
    let mut height = 0i64;
    while symbols.len() - input_count < new_count {
        height += i64::from(decode_int(&mut mq, &mut dh).ok_or(RasterError::Malformed("JBIG2 height class delta"))?);
        let mut symbol_width = 0i64;
        loop {
            let Some(delta) = decode_int(&mut mq, &mut dw) else { break };
            symbol_width += i64::from(delta);
            if symbol_width < 0 || height < 0 || symbols.len() - input_count >= new_count { return Err(RasterError::Malformed("invalid JBIG2 symbol size")); }
            let bitmap = if !refagg {
                decode_generic(&mut mq, &mut generic, symbol_width as usize, height as usize, template, false, &at, None)?
            } else {
                if decode_int(&mut mq, &mut ai).unwrap_or(0) != 1 { return Err(RasterError::Unsupported("JBIG2 aggregate symbols with several instances")); }
                let id = decode_id(&mut mq, &mut id_contexts, code_length);
                let (dx, dy) = (decode_int(&mut mq, &mut rdx).unwrap_or(0), decode_int(&mut mq, &mut rdy).unwrap_or(0));
                let reference = symbols.get(id).ok_or(RasterError::Malformed("JBIG2 refinement symbol id"))?.clone();
                decode_refinement(&mut mq, &mut refinement, symbol_width as usize, height as usize, rtemplate, &reference, i64::from(dx), i64::from(dy), false, &rat)?
            };
            symbols.push(bitmap);
        }
    }
    let mut flags = vec![false; symbols.len()];
    let (mut index, mut current) = (0usize, false);
    while index < symbols.len() {
        let run = decode_int(&mut mq, &mut ex).ok_or(RasterError::Malformed("JBIG2 export run"))?.max(0) as usize;
        for slot in flags.iter_mut().skip(index).take(run) { *slot = current; }
        index += run;
        current = !current;
        if run == 0 && index >= symbols.len() { break; }
    }
    let exported: Vec<Bitmap> = symbols.into_iter().zip(flags).filter_map(|(symbol, keep)| keep.then_some(symbol)).collect();
    let _ = export_count;
    Ok(Dictionary { exported })
}

fn pattern_dictionary(data: &[u8]) -> Result<Vec<Bitmap>, RasterError> {
    let flags = *data.first().ok_or(RasterError::Malformed("truncated JBIG2 pattern dictionary"))?;
    let (mmr, template) = (flags & 1 == 1, (flags >> 1) & 3);
    let (pw, ph, gray_max) = (usize::from(*data.get(1).ok_or(RasterError::Malformed("truncated JBIG2 pattern dictionary"))?), usize::from(*data.get(2).ok_or(RasterError::Malformed("truncated JBIG2 pattern dictionary"))?), u32_at(data, 3)? as usize);
    if pw == 0 || ph == 0 || gray_max > 1 << 16 { return Err(RasterError::Malformed("invalid JBIG2 pattern size")); }
    let width = (gray_max + 1) * pw;
    let collective = if mmr {
        let packed = ccitt::decode(&data[7..], &CcittParameters { k: -1, columns: width as u32, rows: ph as u32, black_is_1: true, byte_align: false }, &mut |_| true)?;
        let mut bitmap = Bitmap::new(width, ph, 0)?;
        let stride = width.div_ceil(8);
        for y in 0..ph { for x in 0..width { bitmap.pixels[y * width + x] = packed[y * stride + x / 8] >> (7 - (x & 7)) & 1; } }
        bitmap
    } else {
        let at: Vec<(i8, i8)> = if template == 0 { vec![(-(pw as i64).min(127) as i8, 0), (-3, -1), (2, -2), (-2, -2)] } else { vec![(-(pw as i64).min(127) as i8, 0)] };
        let mut mq = MqDecoder::new(&data[7..]);
        decode_generic(&mut mq, &mut vec![Context::default(); 1 << 16], width, ph, template, false, &at, None)?
    };
    Ok((0..=gray_max).map(|index| {
        let mut pattern = Bitmap { width: pw, height: ph, pixels: vec![0; pw * ph] };
        for y in 0..ph { for x in 0..pw { pattern.pixels[y * pw + x] = collective.pixels[y * width + index * pw + x]; } }
        pattern
    }).collect())
}

fn halftone_region(data: &[u8], patterns: &[Bitmap]) -> Result<(Region, Bitmap), RasterError> {
    let region = region_info(data)?;
    let flags = *data.get(17).ok_or(RasterError::Malformed("truncated JBIG2 halftone"))?;
    let (mmr, template, skip_enabled, operator, default_pixel) = (flags & 1 == 1, (flags >> 1) & 3, flags & 8 != 0, (flags >> 4) & 7, flags >> 7);
    if mmr { return Err(RasterError::Unsupported("MMR-coded JBIG2 halftone gray-scale planes")); }
    let (grid_w, grid_h) = (u32_at(data, 18)? as usize, u32_at(data, 22)? as usize);
    let (gx, gy) = (i64::from(u32_at(data, 26)? as i32), i64::from(u32_at(data, 30)? as i32));
    let (rx, ry) = (i64::from(u16_at(data, 34)?), i64::from(u16_at(data, 36)?));
    let first = patterns.first().ok_or(RasterError::Malformed("JBIG2 halftone without patterns"))?;
    let (pw, ph) = (first.width as i64, first.height as i64);
    let planes = (usize::BITS - (patterns.len().max(2) - 1).leading_zeros()) as usize;
    let mut page = Bitmap::new(region.width, region.height, default_pixel)?;
    let position = |m: i64, n: i64| ((gx + m * ry + n * rx) >> 8, (gy + m * rx - n * ry) >> 8);
    let mut skip = Bitmap::new(grid_w, grid_h, 0)?;
    if skip_enabled {
        for m in 0..grid_h { for n in 0..grid_w {
            let (x, y) = position(m as i64, n as i64);
            if x + pw <= 0 || x >= region.width as i64 || y + ph <= 0 || y >= region.height as i64 { skip.pixels[m * grid_w + n] = 1; }
        } }
    }
    let at: Vec<(i8, i8)> = if template == 0 { vec![(3, -1), (-3, -1), (2, -2), (-2, -2)] } else { vec![(if template <= 1 { 3 } else { 2 }, -1)] };
    let mut mq = MqDecoder::new(&data[38..]);
    let mut contexts = vec![Context::default(); 1 << 16];
    let mut gray = vec![0u32; grid_w * grid_h];
    let mut previous: Option<Bitmap> = None;
    for _ in 0..planes {
        let mut plane = decode_generic(&mut mq, &mut contexts, grid_w, grid_h, template, false, &at, skip_enabled.then_some(&skip))?;
        if let Some(above) = &previous { for (bit, upper) in plane.pixels.iter_mut().zip(&above.pixels) { *bit ^= upper; } }
        for (value, bit) in gray.iter_mut().zip(&plane.pixels) { *value = *value << 1 | u32::from(*bit); }
        previous = Some(plane);
    }
    for m in 0..grid_h {
        for n in 0..grid_w {
            let (x, y) = position(m as i64, n as i64);
            let pattern = &patterns[(gray[m * grid_w + n] as usize).min(patterns.len() - 1)];
            page.combine(pattern, x, y, operator as u8);
        }
    }
    Ok((region, page))
}

fn text_region(segment: &Segment<'_>, symbols: &[Bitmap]) -> Result<(Region, Bitmap), RasterError> {
    let data = segment.data;
    let region = region_info(data)?;
    let flags = u16_at(data, 17)?;
    if flags & 1 == 1 { return Err(RasterError::Unsupported("Huffman-coded JBIG2 text regions")); }
    let (refine, log_strips, corner, transposed, operator, default_pixel) = (flags & 2 != 0, (flags >> 2) & 3, (flags >> 4) & 3, flags & 0x40 != 0, ((flags >> 7) & 3) as u8, ((flags >> 9) & 1) as u8);
    let mut offset = ((flags >> 10) & 31) as i32;
    if offset > 15 { offset -= 32; }
    let rtemplate = ((flags >> 15) & 1) as u8;
    let mut cursor = 19;
    let rat = if refine && rtemplate == 0 { let pairs = at_pairs(data, cursor, 2)?; cursor += 4; pairs } else { vec![(-1, -1), (-1, -1)] };
    let instances = u32_at(data, cursor)? as usize;
    cursor += 4;
    let strips = 1i64 << log_strips;
    let mut mq = MqDecoder::new(&data[cursor..]);
    let (mut dt, mut fs, mut ds, mut it, mut ri, mut rdw, mut rdh, mut rdx, mut rdy) = (IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new(), IntContexts::new());
    let code_length = (symbols.len().max(2) as f64).log2().ceil() as u32;
    let mut id_contexts = vec![Context::default(); 1 << (code_length + 1)];
    let mut refinement = vec![Context::default(); 1 << 13];
    let mut page = Bitmap::new(region.width, region.height, default_pixel)?;
    let mut stripe_t = -i64::from(decode_int(&mut mq, &mut dt).ok_or(RasterError::Malformed("JBIG2 strip delta"))?) * strips;
    let (mut first_s, mut done) = (0i64, 0usize);
    while done < instances {
        stripe_t += i64::from(decode_int(&mut mq, &mut dt).ok_or(RasterError::Malformed("JBIG2 strip delta"))?) * strips;
        let mut first = true;
        let mut current_s = 0i64;
        loop {
            if first {
                first_s += i64::from(decode_int(&mut mq, &mut fs).ok_or(RasterError::Malformed("JBIG2 first S"))?);
                current_s = first_s;
                first = false;
            } else {
                let Some(delta) = decode_int(&mut mq, &mut ds) else { break };
                current_s += i64::from(delta) + i64::from(offset);
            }
            let current_t = if strips == 1 { 0 } else { i64::from(decode_int(&mut mq, &mut it).unwrap_or(0)) };
            let ti = stripe_t + current_t;
            let id = decode_id(&mut mq, &mut id_contexts, code_length);
            let refined = refine && decode_int(&mut mq, &mut ri).unwrap_or(0) == 1;
            let base = symbols.get(id).ok_or(RasterError::Malformed("JBIG2 symbol id out of range"))?;
            let bitmap = if refined {
                let (w, h) = (decode_int(&mut mq, &mut rdw).unwrap_or(0), decode_int(&mut mq, &mut rdh).unwrap_or(0));
                let (x, y) = (decode_int(&mut mq, &mut rdx).unwrap_or(0), decode_int(&mut mq, &mut rdy).unwrap_or(0));
                let (width, height) = ((base.width as i64 + i64::from(w)).max(0) as usize, (base.height as i64 + i64::from(h)).max(0) as usize);
                decode_refinement(&mut mq, &mut refinement, width, height, rtemplate, base, i64::from(w.div_euclid(2) + x), i64::from(h.div_euclid(2) + y), false, &rat)?
            } else { base.clone() };
            let (wi, hi) = (bitmap.width as i64, bitmap.height as i64);
            if !transposed && corner >= 2 { current_s += wi - 1; }
            if transposed && (corner == 0 || corner == 2) { current_s += hi - 1; }
            let si = current_s;
            let (x, y) = if !transposed {
                match corner { 1 => (si, ti), 3 => (si - wi + 1, ti), 0 => (si, ti - hi + 1), _ => (si - wi + 1, ti - hi + 1) }
            } else {
                match corner { 1 => (ti, si), 3 => (ti - wi + 1, si), 0 => (ti, si - hi + 1), _ => (ti - wi + 1, si - hi + 1) }
            };
            page.combine(&bitmap, x, y, operator);
            if !transposed && (corner == 0 || corner == 1) { current_s += wi - 1; }
            if transposed && (corner == 1 || corner == 3) { current_s += hi - 1; }
            done += 1;
            if done >= instances && instances > 0 { break; }
        }
    }
    Ok((region, page))
}

/// 🧩️ Decodes the segments of an embedded JBIG2 stream (and its optional globals) into the page bitmap.
pub fn decode(globals: Option<&[u8]>, data: &[u8], control: Control<'_>) -> Result<Bitmap, RasterError> {
    let mut all = Vec::new();
    if let Some(globals) = globals { all.extend(segments(globals)?); }
    all.extend(segments(data)?);
    let mut dictionaries: Vec<(u32, Dictionary)> = Vec::new();
    let mut page: Option<Bitmap> = None;
    let mut patterns: Vec<Bitmap> = Vec::new();
    let total = all.len().max(1);
    for (index, segment) in all.iter().enumerate() {
        if !control(index as f32 / total as f32) { return Err(RasterError::Cancelled); }
        let referenced = |dictionaries: &[(u32, Dictionary)]| -> Vec<Bitmap> {
            segment.referred.iter().flat_map(|number| dictionaries.iter().filter(move |(id, _)| id == number).flat_map(|(_, dictionary)| dictionary.exported.iter().cloned())).collect()
        };
        match segment.kind {
            0 => { let dictionary = symbol_dictionary(segment, referenced(&dictionaries))?; dictionaries.push((segment.number, dictionary)); }
            48 => {
                let flags = *segment.data.get(16).ok_or(RasterError::Malformed("truncated JBIG2 page information"))?;
                let width = u32_at(segment.data, 0)? as usize;
                let mut height = u32_at(segment.data, 4)? as usize;
                if height == 0xFFFF_FFFF { height = 0; }
                page = Some(Bitmap::new(width, height, (flags >> 2) & 1)?);
            }
            4 | 6 | 7 | 36 | 38 | 39 | 40 | 42 | 43 => {
                let (region, bitmap) = match segment.kind {
                    4 | 6 | 7 => text_region(segment, &referenced(&dictionaries))?,
                    36 | 38 | 39 => generic_region(segment.data)?,
                    _ => refinement_region(segment.data, page.as_ref())?,
                };
                let target = page.get_or_insert_with(|| Bitmap { width: region.width, height: region.height, pixels: vec![0; region.width * region.height] });
                if target.height < (region.y as usize + region.height) { target.pixels.resize(target.width * (region.y as usize + region.height), 0); target.height = region.y as usize + region.height; }
                target.combine(&bitmap, region.x, region.y, region.operator);
            }
            16 => patterns = pattern_dictionary(segment.data)?,
            20 | 22 | 23 => {
                let (region, bitmap) = halftone_region(segment.data, &patterns)?;
                let target = page.get_or_insert_with(|| Bitmap { width: region.width, height: region.height, pixels: vec![0; region.width * region.height] });
                target.combine(&bitmap, region.x, region.y, region.operator);
            }
            _ => {}
        }
    }
    page.ok_or(RasterError::Malformed("JBIG2 stream without a page"))
}
