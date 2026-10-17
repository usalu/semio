//! 🖼️ Sample unpacking and ancillary chunks of the PNG codec: the sample unpack and scale, the RGBA conversion, the background chunk and the text chunk writer. See [`super`].

use super::*;

//#region Unpack
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unpack_samples(row: &[u8], width: usize, spp: usize, bit_depth: u8) -> Vec<u32> {
    let count = width * spp;
    let mut out = Vec::with_capacity(count);
    if bit_depth == 16 {
        for i in 0..count {
            out.push(((row[i * 2] as u32) << 8) | row[i * 2 + 1] as u32);
        }
    } else if bit_depth == 8 {
        out.extend(row[..count].iter().map(|&sample| sample as u32));
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🎨 Converts one pixel's raw (unscaled) samples to 8-bit RGBA using PLTE/tRNS as needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pixel_to_rgba(samples: &[u32], ihdr: &Ihdr, palette: &[[u8; 3]], palette_alpha: &[u8], gray_trans: Option<u32>, rgb_trans: Option<(u32, u32, u32)>) -> Result<[u8; 4], ValueError> {
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
            let rgb = palette.get(idx).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("png: palette index {idx} out of range")))?;
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
//#endregion Unpack

//#region AncillaryCodec
// 🧩 Typed encode/decode for the ancillary chunk set — kept next to the chunk-order-aware
// `encode_png`/`decode_png` bodies since every one of these is a single (type, wire-shape) pair.

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
/// 🌈️ Convert source background samples to the authored RGBA8 wire representation.
pub(super) fn rgba8_background(snap:&PngProjection,background:&PngBackground)->Result<PngBackground,String>{
    if !matches!(snap.bit_depth,1|2|4|8|16){return Err("png background: invalid source bit depth".into());}
    let sample=|n:u16|u16::from(scale_to_8(u32::from(n),snap.bit_depth));
    let (r,g,b)=match background{
        PngBackground::Grayscale{gray}=>{let n=sample(*gray);(n,n,n)},
        PngBackground::Rgb{r,g,b}=>(sample(*r),sample(*g),sample(*b)),
        PngBackground::Indexed{index}=>{let rgb=snap.plte.as_ref().and_then(|entries|entries.get(*index as usize)).ok_or("png background: palette index out of range")?;(u16::from(rgb.r),u16::from(rgb.g),u16::from(rgb.b))},
    };Ok(PngBackground::Rgb{r,g,b})
}

pub(super) fn encode_bkgd(b: &PngBackground) -> Vec<u8> {
    match b {
        PngBackground::Grayscale { gray } => gray.to_be_bytes().to_vec(),
        PngBackground::Rgb { r, g, b } => {
            let mut v = Vec::with_capacity(6);
            v.extend_from_slice(&r.to_be_bytes());
            v.extend_from_slice(&g.to_be_bytes());
            v.extend_from_slice(&b.to_be_bytes());
            v
        }
        PngBackground::Indexed { index } => vec![*index],
    }
}

/// 📝 Serializes one `PngTextChunk` back to its real `tEXt`/`zTXt`/`iTXt` wire shape (§11.3.4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn write_text_chunk(out: &mut Vec<u8>, tc: &PngTextChunk) -> Result<(), String> {
    match tc.kind {
        PngTextKind::Text => {
            let mut data = Vec::with_capacity(tc.keyword.len() + 1 + tc.value.len());
            data.extend_from_slice(&latin1_bytes(&tc.keyword));
            data.push(0);
            data.extend_from_slice(&latin1_bytes(&tc.value));
            write_chunk(out, b"tEXt", &data);
        }
        PngTextKind::ZText => {
            let mut data = Vec::with_capacity(tc.keyword.len() + 2);
            data.extend_from_slice(&latin1_bytes(&tc.keyword));
            data.push(0);
            data.push(0); // compression method 0 = zlib/deflate
            let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&latin1_bytes(&tc.value))?;
            data.extend_from_slice(&compressed);
            write_chunk(out, b"zTXt", &data);
        }
        PngTextKind::IText => {
            let mut data = Vec::new();
            data.extend_from_slice(&latin1_bytes(&tc.keyword));
            data.push(0);
            data.push(if tc.compressed { 1 } else { 0 });
            data.push(0); // compression method 0 = zlib/deflate
            data.extend_from_slice(tc.language_tag.as_bytes());
            data.push(0);
            data.extend_from_slice(tc.translated_keyword.as_bytes());
            data.push(0);
            if tc.compressed {
                let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(tc.value.as_bytes())?;
                data.extend_from_slice(&compressed);
            } else {
                data.extend_from_slice(tc.value.as_bytes());
            }
            write_chunk(out, b"iTXt", &data);
        }
    }
    Ok(())
}
//#endregion AncillaryCodec
