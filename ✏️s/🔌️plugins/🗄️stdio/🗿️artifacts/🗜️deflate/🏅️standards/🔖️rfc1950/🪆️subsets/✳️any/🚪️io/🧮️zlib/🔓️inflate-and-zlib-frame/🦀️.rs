//! 🔓️ Inflate and zlib framing of the zlib codec: the stored, fixed and dynamic block inflation, the raw inflate and the zlib compress and decompress entries. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
fn inflate_block_stored(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    br.align_byte();
    if br.pos + 4 > br.data.len() {
        return Err("truncated stored block".into());
    }
    let len = u16::from_le_bytes([br.data[br.pos], br.data[br.pos + 1]]);
    let nlen = u16::from_le_bytes([br.data[br.pos + 2], br.data[br.pos + 3]]);
    br.pos += 4;
    if len ^ 0xFFFF != nlen {
        return Err("stored block LEN/NLEN mismatch".into());
    }
    let end = br.pos + len as usize;
    if end > br.data.len() {
        return Err("truncated stored payload".into());
    }
    out.extend_from_slice(&br.data[br.pos..end]);
    br.pos = end;
    Ok(())
}

fn inflate_codes(br: &mut BitReader<'_>, out: &mut Vec<u8>, lit: &HuffDecoder, dist: &HuffDecoder) -> Result<(), String> {
    loop {
        let sym = lit.decode(br)? as usize;
        if sym < 256 {
            out.push(sym as u8);
        } else if sym == 256 {
            break;
        } else if sym <= 285 {
            let idx = sym - 257;
            if idx >= LEN_BASE.len() {
                return Err("invalid length symbol".into());
            }
            let mut length = LEN_BASE[idx] as usize;
            let extra = LEN_EXTRA[idx];
            if extra > 0 {
                length += br.read_bits(extra)? as usize;
            }
            let dsym = dist.decode(br)? as usize;
            if dsym >= DIST_BASE.len() {
                return Err("invalid distance symbol".into());
            }
            let mut distance = DIST_BASE[dsym] as usize;
            let dextra = DIST_EXTRA[dsym];
            if dextra > 0 {
                distance += br.read_bits(dextra)? as usize;
            }
            if distance == 0 || distance > out.len() {
                return Err("invalid backreference".into());
            }
            for _ in 0..length {
                let b = out[out.len() - distance];
                out.push(b);
            }
        } else {
            return Err("invalid lit/len symbol".into());
        }
    }
    Ok(())
}

fn dynamic_decoders(br: &mut BitReader<'_>) -> Result<(HuffDecoder, HuffDecoder), String> {
    let hlit = br.read_bits(5)? as usize + 257;
    let hdist = br.read_bits(5)? as usize + 1;
    let hclen = br.read_bits(4)? as usize + 4;
    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    let mut cl_lens = vec![0u8; 19];
    for i in 0..hclen {
        cl_lens[ORDER[i]] = br.read_bits(3)? as u8;
    }
    let cl_dec = HuffDecoder::from_lengths(&cl_lens)?;
    let mut lens = Vec::with_capacity(hlit + hdist);
    while lens.len() < hlit + hdist {
        let sym = cl_dec.decode(br)? as usize;
        match sym {
            0..=15 => lens.push(sym as u8),
            16 => {
                let rep = br.read_bits(2)? as usize + 3;
                let prev = *lens.last().ok_or("bad repeat")?;
                lens.extend(std::iter::repeat_n(prev, rep));
            }
            17 => {
                let rep = br.read_bits(3)? as usize + 3;
                lens.extend(std::iter::repeat_n(0u8, rep));
            }
            18 => {
                let rep = br.read_bits(7)? as usize + 11;
                lens.extend(std::iter::repeat_n(0u8, rep));
            }
            _ => return Err("bad code-length symbol".into()),
        }
    }
    if lens.len() < hlit + hdist {
        return Err("incomplete dynamic trees".into());
    }
    let lit = HuffDecoder::from_lengths(&lens[..hlit])?;
    let dist = HuffDecoder::from_lengths(&lens[hlit..hlit + hdist])?;
    Ok((lit, dist))
}

fn inflate_dynamic(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    let (lit, dist) = dynamic_decoders(br)?;
    inflate_codes(br, out, &lit, &dist)
}

/// 🗜️ Raw DEFLATE inflate (stored + fixed + dynamic).
pub fn inflate_raw(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut br = BitReader::new(data);
    let mut out = Vec::new();
    loop {
        let bfinal = br.read_bits(1)?;
        let btype = br.read_bits(2)?;
        match btype {
            0 => inflate_block_stored(&mut br, &mut out)?,
            1 => {
                let lit = HuffDecoder::from_lengths(&fixed_lit_lengths())?;
                let dist = HuffDecoder::from_lengths(&fixed_dist_lengths())?;
                inflate_codes(&mut br, &mut out, &lit, &dist)?;
            }
            2 => inflate_dynamic(&mut br, &mut out)?,
            _ => return Err("reserved BTYPE".into()),
        }
        if bfinal == 1 {
            break;
        }
    }
    Ok(out)
}

/// 🗜️ Zlib-wrap compress (CMF/FLG + raw deflate + Adler32).
pub fn zlib_compress(data: &[u8]) -> Result<Vec<u8>, String> {
    let raw = deflate_raw(data);
    let mut out = Vec::with_capacity(2 + raw.len() + 4);
    out.push(0x78);
    out.push(0x01);
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(data).to_be_bytes());
    Ok(out)
}

/// 🎯 Deterministic maximum-compression RFC 1950 materialization for formats whose native
/// canonical writer requires dynamic-Huffman zlib output.
pub fn zlib_compress_deterministic(data: &[u8]) -> Result<Vec<u8>, String> {
    Ok(TunedDeflateEncodeJob::level_nine(data.to_vec()).finish())
}

/// 🎨 Deterministic Adobe Illustrator Flate materialization: its PDF producer uses a 4 KiB
/// window and closes a level-six stream with a partial flush before the final block.
pub fn zlib_compress_illustrator(data: &[u8]) -> Result<Vec<u8>, String> {
    Ok(TunedDeflateEncodeJob::illustrator(data.to_vec()).finish())
}

/// 🗜️ Zlib unwrap + inflate + Adler32 verify.
pub fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 6 {
        return Err("zlib stream too short".into());
    }
    let cmf = data[0];
    let flg = data[1];
    if (cmf & 0x0F) != 8 {
        return Err("unsupported zlib compression method".into());
    }
    if !((cmf as u16) * 256 + flg as u16).is_multiple_of(31) {
        return Err("zlib CMF/FLG check failed".into());
    }
    if flg & 0x20 != 0 {
        return Err("zlib preset dictionary not supported".into());
    }
    let adler_bytes = &data[data.len() - 4..];
    let expect = u32::from_be_bytes([adler_bytes[0], adler_bytes[1], adler_bytes[2], adler_bytes[3]]);
    let raw = &data[2..data.len() - 4];
    let out = inflate_raw(raw)?;
    let got = adler32(&out);
    if got != expect {
        return Err(format!("adler32 mismatch: expected {expect:#010x}, got {got:#010x}"));
    }
    Ok(out)
}
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
