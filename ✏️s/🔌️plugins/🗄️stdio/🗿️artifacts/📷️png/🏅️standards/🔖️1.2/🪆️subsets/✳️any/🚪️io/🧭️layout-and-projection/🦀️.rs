//! 🧭️ Layout and projection of the PNG codec: the empty file, the chunk addresses, the layout and preview readers, the decode and encode entry points, the native sample writer and the authored projection. See [`super`].

use super::*;

//#region ExactAuthority
pub fn empty_png_bytes()->Vec<u8> {encode_png(&PngSnapshot::default()).expect("authored empty PNG model")}

pub fn chunk_addresses(bytes: &[u8]) -> Result<Vec<PngChunkAddress>, String> {
    if bytes.len() < PNG_SIGNATURE.len() || bytes[..PNG_SIGNATURE.len()] != PNG_SIGNATURE {
        return Err("png: bad signature".into());
    }
    let mut cursor = PNG_SIGNATURE.len();
    let mut chunks = Vec::new();
    let mut found_iend = false;
    while cursor < bytes.len() {
        let header = bytes.get(cursor..cursor + 8).ok_or("png: truncated chunk header")?;
        let length = u32::from_be_bytes(header[..4].try_into().expect("four-byte length")) as usize;
        let kind: [u8; 4] = header[4..8].try_into().expect("four-byte kind");
        let data_start = cursor.checked_add(8).ok_or("png: chunk offset overflow")?;
        let data_end = data_start.checked_add(length).ok_or("png: chunk length overflow")?;
        let end = data_end.checked_add(4).ok_or("png: chunk CRC range overflow")?;
        let data = bytes.get(data_start..data_end).ok_or("png: truncated chunk data")?;
        let stored = bytes.get(data_end..end).ok_or("png: truncated chunk CRC")?;
        if png_crc32_parts(&kind, data).to_be_bytes() != stored {
            return Err(format!("png: chunk CRC mismatch ({})", String::from_utf8_lossy(&kind)));
        }
        chunks.push(PngChunkAddress { ordinal: chunks.len(), kind, start: cursor, data_start, data_end, end });
        cursor = end;
        if kind == *b"IEND" {
            found_iend = true;
            break;
        }
    }
    if !found_iend {
        return Err("png: missing IEND".into());
    }
    if cursor != bytes.len() {
        return Err("png: bytes follow IEND".into());
    }
    Ok(chunks)
}

pub fn png_layout_bytes(bytes: &[u8]) -> Result<PngLayout, String> {
    let chunks = chunk_addresses(bytes)?;
    let first = chunks.first().ok_or("png: missing IHDR")?;
    if first.kind != *b"IHDR" {
        return Err("png: IHDR must be first".into());
    }
    let ihdr = parse_ihdr(&bytes[first.data_start..first.data_end]).map_err(ValueError::into_message)?;
    if !chunks.iter().any(|chunk| chunk.kind == *b"IDAT") {
        return Err("png: missing IDAT".into());
    }
    if ihdr.color_type == 3 && !chunks.iter().any(|chunk| chunk.kind == *b"PLTE") {
        return Err("png: color type 3 requires PLTE".into());
    }
    decode_png_image(bytes)?;
    Ok(PngLayout {
        width: ihdr.width,
        height: ihdr.height,
        bit_depth: ihdr.bit_depth,
        color_type: PngColorType::from_u8(ihdr.color_type)?,
        interlace: ihdr.interlace == 1,
        chunks,
    })
}

pub fn png_layout(snapshot: &PngSnapshot) -> Result<PngLayout, String> {
    if snapshot.schema != crate::STDIO_PNG_DOCUMENT_SCHEMA {
        return Err(format!("png: schema must be {}", crate::STDIO_PNG_DOCUMENT_SCHEMA));
    }
    png_layout_bytes(&encode_png(snapshot)?)
}

pub fn decode_png(bytes: &[u8]) -> Result<PngSnapshot, String> {
    Ok(PngSnapshot { schema: crate::STDIO_PNG_DOCUMENT_SCHEMA.into(), image: decode_png_image(bytes)? })
}

pub fn encode_png(snapshot: &PngSnapshot) -> Result<Vec<u8>, String> {
    if snapshot.schema != crate::STDIO_PNG_DOCUMENT_SCHEMA {return Err("png: undeclared semantic schema".into());}
    encode_png_image(&snapshot.image)
}

pub fn png_preview(snapshot: &PngSnapshot) -> Result<PngPreview, String> {
    const MAXIMUM_RGBA_BYTES: usize = 64 * 1024 * 1024;
    let layout = png_layout(snapshot)?;
    let rgba_bytes = usize::try_from(layout.width)
        .map_err(|_| "png: preview width exceeds address space")?
        .checked_mul(usize::try_from(layout.height).map_err(|_| "png: preview height exceeds address space")?)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or("png: preview byte count overflow")?;
    if rgba_bytes > MAXIMUM_RGBA_BYTES {
        return Err(format!("png: preview needs {rgba_bytes} RGBA bytes, above the {MAXIMUM_RGBA_BYTES}-byte display limit"));
    }
    let bytes = encode_png(snapshot)?;
    Ok(PngPreview { width: layout.width, height: layout.height, bytes })
}

pub fn write_native_sample(row: &mut [u8], ordinal: usize, bit_depth: u8, sample: u16) {
    match bit_depth {
        16 => row[ordinal * 2..ordinal * 2 + 2].copy_from_slice(&sample.to_be_bytes()),
        8 => row[ordinal] = sample as u8,
        depth => {
            let bit = ordinal * depth as usize;
            let shift = 8 - depth as usize - bit % 8;
            let mask = (((1u16 << depth) - 1) as u8) << shift;
            row[bit / 8] = (row[bit / 8] & !mask) | ((sample as u8) << shift & mask);
        }
    }
}

pub fn author_png_projection(snap: &PngProjection) -> Result<Vec<u8>, String> {
    let expected_len = (snap.width as usize).checked_mul(snap.height as usize).and_then(|p| p.checked_mul(4)).ok_or("dimensions overflow")?;
    if snap.pixels.len() != expected_len {
        return Err("pixels length mismatch".into());
    }
    let bpp = 4usize;
    let row_bytes = snap.width as usize * bpp;
    let mut idat = Vec::with_capacity((row_bytes + 1) * snap.height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for y in 0..snap.height as usize {
        let row = &snap.pixels[y * row_bytes..(y + 1) * row_bytes];
        let (ft, filtered) = choose_filter(row, prev.as_deref(), bpp);
        idat.push(ft);
        idat.extend_from_slice(&filtered);
        prev = Some(row.to_vec());
    }
    let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&idat)?;

    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut idat_written = false;
    let mut iend_written = false;
    for marker in &snap.chunk_order {
        match marker {
            PngChunkMarker::Ihdr => {
                let mut ihdr = Vec::with_capacity(13);
                ihdr.extend_from_slice(&snap.width.to_be_bytes());
                ihdr.extend_from_slice(&snap.height.to_be_bytes());
                ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
                write_chunk(&mut out, b"IHDR", &ihdr);
            }
            PngChunkMarker::Plte => {
                if let Some(entries) = &snap.plte {
                    let mut data = Vec::with_capacity(entries.len() * 3);
                    for e in entries {
                        data.extend_from_slice(&[e.r, e.g, e.b]);
                    }
                    write_chunk(&mut out, b"PLTE", &data);
                }
            }
            PngChunkMarker::Trns => {}
            PngChunkMarker::Gama => {
                if let Some(g) = snap.gama {
                    write_chunk(&mut out, b"gAMA", &g.to_be_bytes());
                }
            }
            PngChunkMarker::Chrm => {
                if let Some(c) = &snap.chrm {
                    let mut data = Vec::with_capacity(32);
                    for v in [c.white_x, c.white_y, c.red_x, c.red_y, c.green_x, c.green_y, c.blue_x, c.blue_y] {
                        data.extend_from_slice(&v.to_be_bytes());
                    }
                    write_chunk(&mut out, b"cHRM", &data);
                }
            }
            PngChunkMarker::Srgb => {
                if let Some(s) = snap.srgb {
                    write_chunk(&mut out, b"sRGB", &[s.to_u8()]);
                }
            }
            PngChunkMarker::Phys => {
                if let Some(p) = &snap.phys {
                    let mut data = Vec::with_capacity(9);
                    data.extend_from_slice(&p.ppu_x.to_be_bytes());
                    data.extend_from_slice(&p.ppu_y.to_be_bytes());
                    data.push(if p.unit_is_meter { 1 } else { 0 });
                    write_chunk(&mut out, b"pHYs", &data);
                }
            }
            PngChunkMarker::Time => {
                if let Some(t) = &snap.time {
                    let mut data = Vec::with_capacity(7);
                    data.extend_from_slice(&t.year.to_be_bytes());
                    data.extend_from_slice(&[t.month, t.day, t.hour, t.minute, t.second]);
                    write_chunk(&mut out, b"tIME", &data);
                }
            }
            PngChunkMarker::Bkgd => {
                if let Some(b) = &snap.bkgd {
                    write_chunk(&mut out, b"bKGD", &encode_bkgd(&rgba8_background(snap,b)?));
                }
            }
            PngChunkMarker::Idat => {
                if !idat_written {
                    write_chunk(&mut out, b"IDAT", &compressed);
                    idat_written = true;
                }
            }
            PngChunkMarker::Iend => {
                if !iend_written {
                    write_chunk(&mut out, b"IEND", &[]);
                    iend_written = true;
                }
            }
            PngChunkMarker::Text { index } => {
                if let Some(tc) = snap.text_chunks.get(*index) {
                    write_text_chunk(&mut out, tc)?;
                }
            }
            PngChunkMarker::Unknown { index } => {
                if let Some(c) = snap.unknown_chunks.get(*index) {
                    write_chunk(&mut out, &c.kind, &c.data);
                }
            }
        }
    }
    if !idat_written {
        write_chunk(&mut out, b"IDAT", &compressed);
    }
    if !iend_written {
        write_chunk(&mut out, b"IEND", &[]);
    }
    png_layout_bytes(&out)?;
    Ok(out)
}
//#endregion Codec
