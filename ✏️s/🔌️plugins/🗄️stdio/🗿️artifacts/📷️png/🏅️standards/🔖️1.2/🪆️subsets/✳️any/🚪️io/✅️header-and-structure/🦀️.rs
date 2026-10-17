//! ✅️ Header and structure checks of the PNG codec: the IHDR parser, the chunk ordering rules, the structure validation and the sample and row size helpers. See [`super`].

use super::*;

//#region Ihdr
pub(super) struct Ihdr {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) bit_depth: u8,
    pub(super) color_type: u8,
    pub(super) interlace: u8,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn parse_ihdr(data: &[u8]) -> Result<Ihdr, ValueError> {
    if data.len() != 13 {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"png IHDR: expected 13 bytes"));
    }
    let width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let bit_depth = data[8];
    let color_type = data[9];
    let compression = data[10];
    let filter_method = data[11];
    let interlace = data[12];
    if width == 0 || height == 0 {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"png IHDR: zero dimension"));
    }
    if compression != 0 {
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"png IHDR: unsupported compression method"));
    }
    if filter_method != 0 {
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"png IHDR: unsupported filter method"));
    }
    if interlace > 1 {
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"png IHDR: unsupported interlace method"));
    }
    let valid = match color_type {
        0 => matches!(bit_depth, 1 | 2 | 4 | 8 | 16),
        2 => matches!(bit_depth, 8 | 16),
        3 => matches!(bit_depth, 1 | 2 | 4 | 8),
        4 => matches!(bit_depth, 8 | 16),
        6 => matches!(bit_depth, 8 | 16),
        _ => false,
    };
    if !valid {
        return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,format!("png IHDR: unsupported color type {color_type} / bit depth {bit_depth}")));
    }
    Ok(Ihdr { width, height, bit_depth, color_type, interlace })
}

fn chunk_position(chunks: &[PngChunkView<'_>], kind: [u8; 4]) -> Option<usize> {
    chunks.iter().position(|(candidate, _)| *candidate == kind)
}

fn require_singleton(chunks: &[PngChunkView<'_>], kind: [u8; 4]) -> Result<(), String> {
    if chunks.iter().filter(|(candidate, _)| *candidate == kind).count() > 1 {
        return Err(format!("png: {} must be a singleton", String::from_utf8_lossy(&kind)));
    }
    Ok(())
}

pub(super) fn validate_png_structure(chunks: &[PngChunkView<'_>]) -> Result<(), String> {
    let Some((first_kind, first_data)) = chunks.first() else { return Err("png: missing IHDR".into()) };
    if *first_kind != *b"IHDR" {
        return Err("png: IHDR must be first".into());
    }
    let ihdr = parse_ihdr(first_data).map_err(ValueError::into_message)?;
    let Some((last_kind, last_data)) = chunks.last() else { return Err("png: missing IEND".into()) };
    if *last_kind != *b"IEND" {
        return Err("png: IEND must be last".into());
    }
    if !last_data.is_empty() {
        return Err("png: IEND must be empty".into());
    }
    for (kind, _) in chunks {
        if !kind.iter().all(u8::is_ascii_alphabetic) {
            return Err(format!("png: invalid chunk type {}", String::from_utf8_lossy(kind)));
        }
        if kind[2].is_ascii_lowercase() {
            return Err(format!("png: chunk {} uses the reserved type bit", String::from_utf8_lossy(kind)));
        }
    }
    for kind in [*b"IHDR", *b"PLTE", *b"tRNS", *b"gAMA", *b"cHRM", *b"sRGB", *b"pHYs", *b"tIME", *b"bKGD", *b"IEND"] {
        require_singleton(chunks, kind)?;
    }
    let first_idat = chunk_position(chunks, *b"IDAT").ok_or("png: missing IDAT")?;
    let last_idat = chunks.iter().rposition(|(kind, _)| *kind == *b"IDAT").expect("first IDAT exists");
    if chunks[first_idat..=last_idat].iter().any(|(kind, _)| *kind != *b"IDAT") {
        return Err("png: IDAT chunks must be consecutive".into());
    }
    let plte = chunk_position(chunks, *b"PLTE");
    let palette_entries = if let Some(index) = plte {
        if index > first_idat {
            return Err("png: PLTE must precede IDAT".into());
        }
        if matches!(ihdr.color_type, 0 | 4) {
            return Err("png: PLTE is forbidden for grayscale profiles".into());
        }
        let data = chunks[index].1;
        if data.is_empty() || data.len() % 3 != 0 || data.len() > 768 {
            return Err("png: PLTE must contain 1 to 256 complete entries".into());
        }
        let entries = data.len() / 3;
        if ihdr.color_type == 3 && entries > (1usize << ihdr.bit_depth) {
            return Err("png: PLTE has more entries than the indexed bit depth permits".into());
        }
        entries
    } else {
        if ihdr.color_type == 3 {
            return Err("png: color type 3 requires PLTE".into());
        }
        0
    };
    for kind in [*b"gAMA", *b"cHRM", *b"sRGB"] {
        if let Some(index) = chunk_position(chunks, kind) {
            if index > first_idat || plte.is_some_and(|palette| index > palette) {
                return Err(format!("png: {} must precede PLTE and IDAT", String::from_utf8_lossy(&kind)));
            }
        }
    }
    if let Some(index) = chunk_position(chunks, *b"gAMA") {
        let data = chunks[index].1;
        if data.len() != 4 || u32::from_be_bytes(data.try_into().expect("four-byte gAMA")) == 0 {
            return Err("png: gAMA must be a nonzero four-byte integer".into());
        }
    }
    if let Some(index) = chunk_position(chunks, *b"pHYs") {
        if index > first_idat {
            return Err("png: pHYs must precede IDAT".into());
        }
        let data = chunks[index].1;
        if data.len() != 9 || data[8] > 1 {
            return Err("png: pHYs has an invalid length or unit".into());
        }
    }
    if let Some(index) = chunk_position(chunks, *b"tRNS") {
        if index > first_idat || plte.is_some_and(|palette| index < palette) {
            return Err("png: tRNS must follow PLTE when present and precede IDAT".into());
        }
        let data = chunks[index].1;
        let maximum = (1u32 << ihdr.bit_depth) - 1;
        match ihdr.color_type {
            0 if data.len() == 2 && u32::from(u16::from_be_bytes(data.try_into().expect("grayscale tRNS"))) <= maximum => {}
            2 if data.len() == 6 && data.chunks_exact(2).all(|sample| u32::from(u16::from_be_bytes(sample.try_into().expect("RGB tRNS sample"))) <= maximum) => {}
            3 if !data.is_empty() && data.len() <= palette_entries => {}
            4 | 6 => return Err("png: tRNS is forbidden for alpha profiles".into()),
            _ => return Err("png: tRNS payload does not match the image profile".into()),
        }
    }
    if let Some(index) = chunk_position(chunks, *b"bKGD") {
        if index > first_idat || plte.is_some_and(|palette| index < palette) {
            return Err("png: bKGD must follow PLTE when present and precede IDAT".into());
        }
        let data = chunks[index].1;
        let maximum = (1u32 << ihdr.bit_depth) - 1;
        match ihdr.color_type {
            0 | 4 if data.len() == 2 && u32::from(u16::from_be_bytes(data.try_into().expect("grayscale bKGD"))) <= maximum => {}
            2 | 6 if data.len() == 6 && data.chunks_exact(2).all(|sample| u32::from(u16::from_be_bytes(sample.try_into().expect("RGB bKGD sample"))) <= maximum) => {}
            3 if data.len() == 1 && usize::from(data[0]) < palette_entries => {}
            _ => return Err("png: bKGD payload does not match the image profile".into()),
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn samples_per_pixel(color_type: u8) -> usize {
    match color_type {
        0 => 1,
        2 => 3,
        3 => 1,
        4 => 2,
        6 => 4,
        _ => unreachable!("validated in parse_ihdr"),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn bpp_bytes(ihdr: &Ihdr) -> usize {
    (samples_per_pixel(ihdr.color_type) * ihdr.bit_depth as usize).div_ceil(8).max(1)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn packed_row_bytes(width: u32, color_type: u8, bit_depth: u8) -> usize {
    let samples = width as usize * samples_per_pixel(color_type);
    if bit_depth >= 8 { samples * (bit_depth as usize / 8) } else { samples.div_ceil(8 / bit_depth as usize) }
}
//#endregion Ihdr
