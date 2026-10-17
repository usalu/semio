//! 🔓️ Native decoding of the PNG codec: the read error and the controlled image decoder. See [`super`].

use super::*;

//#region ExactAuthority
pub(super) fn decode_png_image(data:&[u8])->Result<PngImage,String> {let mut progress=|_|true;let mut control=NativeDecodeControl::new(512*1024*1024,&mut progress);decode_png_image_controlled(data,&mut control).map_err(|error|error.into_value().into_message())}

pub(super) enum PngReadError {Format(String),Refusal(ValueError)}

impl From<String> for PngReadError {fn from(value:String)->Self {Self::Format(value)}}

impl From<&str> for PngReadError {fn from(value:&str)->Self {Self::Format(value.into())}}

impl From<ValueError> for PngReadError {fn from(value:ValueError)->Self {Self::Refusal(value)}}

impl PngReadError {pub(super) fn into_value(self)->ValueError {match self {Self::Format(message)=>ValueError::new(ValueRefusalKind::InvalidValue,message),Self::Refusal(error)=>error}}}

pub(super) fn decode_png_image_controlled(data: &[u8],control:&mut NativeDecodeControl<'_>) -> Result<PngImage, PngReadError> {
    let chunks = read_chunks(data,control)?;
    validate_png_structure(&chunks)?;
    let mut ihdr: Option<Ihdr> = None;
    let compressed_size=chunks.iter().filter(|(kind,_)|*kind==*b"IDAT").try_fold(0usize,|n,(_,data)|n.checked_add(data.len()).ok_or("png: IDAT size overflow"))?;
    let mut idat=control.allocate_vec(compressed_size)?;
    let mut seen_idat = false;

    let mut plte_out: Option<Vec<PngRgb>> = None;
    let mut trns_out: Option<PngTransparency> = None;
    let mut gama_out: Option<u32> = None;
    let mut chrm_out: Option<PngChromaticities> = None;
    let mut srgb_out: Option<PngSrgbIntent> = None;
    let mut phys_out: Option<PngPhysicalDims> = None;
    let mut time_out: Option<PngTimestamp> = None;
    let mut bkgd_out: Option<PngBackground> = None;
    let mut text_chunks: Vec<PngTextChunk> = control.allocate_vec(chunks.len())?;
    let mut ancillary_chunks: Vec<PngAncillaryChunk> = control.allocate_vec(chunks.len())?;

    for &(ty, chunk) in &chunks {
        if ty == *b"IHDR" {
            ihdr = Some(parse_ihdr(chunk).map_err(ValueError::into_message)?);

        } else if ty == *b"PLTE" {
            if chunk.len() % 3 != 0 {
                return Err("png PLTE: length not a multiple of 3".into());
            }
            let mut entries=control.allocate_vec(chunk.len()/3)?;control.begin_stage(chunk.len()/3)?;for c in chunk.as_chunks::<3>().0 {entries.push(PngRgb {r:c[0],g:c[1],b:c[2]});control.step()?;}
            plte_out=Some(entries);

        } else if ty == *b"tRNS" {
            let color_type = ihdr.as_ref().ok_or("png: tRNS before IHDR")?.color_type;
            match color_type {
                0 => {
                    if chunk.len() != 2 {
                        return Err("png tRNS: expected 2 bytes for grayscale".into());
                    }
                    let g = u16::from_be_bytes([chunk[0], chunk[1]]);
                    trns_out = Some(PngTransparency::Grayscale { gray: g });
                }
                2 => {
                    if chunk.len() != 6 {
                        return Err("png tRNS: expected 6 bytes for truecolor".into());
                    }
                    let r = u16::from_be_bytes([chunk[0], chunk[1]]);
                    let g = u16::from_be_bytes([chunk[2], chunk[3]]);
                    let b = u16::from_be_bytes([chunk[4], chunk[5]]);
                    trns_out = Some(PngTransparency::Rgb { r, g, b });
                }
                3 => {
                    trns_out = Some(PngTransparency::Indexed { alpha: control.copy_bytes(chunk)? });
                }
                _ => {} // spec: tRNS shall not appear for 4/6 (already carry alpha) — ignore rather than fail
            }

        } else if ty == *b"gAMA" {
            if chunk.len() != 4 {
                return Err("png gAMA: expected 4 bytes".into());
            }
            gama_out = Some(u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));

        } else if ty == *b"cHRM" {
            if chunk.len() != 32 {
                return Err("png cHRM: expected 32 bytes".into());
            }
            let v = |i: usize| u32::from_be_bytes([chunk[i], chunk[i + 1], chunk[i + 2], chunk[i + 3]]);
            chrm_out = Some(PngChromaticities { white_x: v(0), white_y: v(4), red_x: v(8), red_y: v(12), green_x: v(16), green_y: v(20), blue_x: v(24), blue_y: v(28) });

        } else if ty == *b"sRGB" {
            if chunk.len() != 1 {
                return Err("png sRGB: expected 1 byte".into());
            }
            srgb_out = Some(PngSrgbIntent::from_u8(chunk[0])?);

        } else if ty == *b"pHYs" {
            if chunk.len() != 9 {
                return Err("png pHYs: expected 9 bytes".into());
            }
            phys_out = Some(PngPhysicalDims { ppu_x: u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]), ppu_y: u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]), unit_is_meter: chunk[8] == 1 });

        } else if ty == *b"tIME" {
            if chunk.len() != 7 {
                return Err("png tIME: expected 7 bytes".into());
            }
            time_out = Some(PngTimestamp { year: u16::from_be_bytes([chunk[0], chunk[1]]), month: chunk[2], day: chunk[3], hour: chunk[4], minute: chunk[5], second: chunk[6] });

        } else if ty == *b"bKGD" {
            let color_type = ihdr.as_ref().ok_or("png: bKGD before IHDR")?.color_type;
            bkgd_out = Some(match color_type {
                0 | 4 => {
                    if chunk.len() != 2 {
                        return Err("png bKGD: expected 2 bytes for grayscale".into());
                    }
                    PngBackground::Grayscale { gray: u16::from_be_bytes([chunk[0], chunk[1]]) }
                }
                2 | 6 => {
                    if chunk.len() != 6 {
                        return Err("png bKGD: expected 6 bytes for truecolor".into());
                    }
                    PngBackground::Rgb { r: u16::from_be_bytes([chunk[0], chunk[1]]), g: u16::from_be_bytes([chunk[2], chunk[3]]), b: u16::from_be_bytes([chunk[4], chunk[5]]) }
                }
                3 => {
                    if chunk.len() != 1 {
                        return Err("png bKGD: expected 1 byte for palette".into());
                    }
                    PngBackground::Indexed { index: chunk[0] }
                }
                _ => return Err("png bKGD: unsupported color type".into()),
            });

        } else if ty == *b"tEXt" {
            let nul = chunk.iter().position(|&b| b == 0).ok_or("png tEXt: missing NUL after keyword")?;
            control.charge(chunk.len().saturating_mul(2))?;
            let keyword = latin1_text(&chunk[..nul]);
            let value = latin1_text(&chunk[nul + 1..]);
            let _index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() });

        } else if ty == *b"zTXt" {
            let nul = chunk.iter().position(|&b| b == 0).ok_or("png zTXt: missing NUL after keyword")?;
            control.charge(chunk.len().saturating_mul(2))?;
            let keyword = latin1_text(&chunk[..nul]);
            if chunk.len() < nul + 2 {
                return Err("png zTXt: missing compression method".into());
            }
            if chunk[nul + 1] != 0 {
                return Err("png zTXt: compression method must be zero".into());
            }
            let value_bytes=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(&chunk[nul+2..],control.maximum_bytes().saturating_sub(control.owned_bytes()),control)?;
            let value = latin1_text(&value_bytes);
            let _index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: true, kind: PngTextKind::ZText, language_tag: String::new(), translated_keyword: String::new() });

        } else if ty == *b"iTXt" {
            let mut pos = 0usize;
            let nul1 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after keyword")?;
            control.charge(chunk.len().saturating_mul(2))?;
            let keyword = latin1_text(&chunk[pos..pos + nul1]);
            pos += nul1 + 1;
            if pos + 2 > chunk.len() {
                return Err("png iTXt: truncated flags".into());
            }
            if chunk[pos] > 1 {
                return Err("png iTXt: compression flag must be zero or one".into());
            }
            let compressed_flag = chunk[pos] == 1;
            if chunk[pos + 1] != 0 {
                return Err("png iTXt: compression method must be zero".into());
            }
            pos += 2; // compressed flag + compression method
            let nul2 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after language tag")?;
            let language_tag_bytes = &chunk[pos..pos + nul2];
            if !language_tag_bytes.is_ascii() {
                return Err("png iTXt: language tag must be ASCII".into());
            }
            let language_tag = std::str::from_utf8(language_tag_bytes).expect("ASCII language tag").to_owned();
            pos += nul2 + 1;
            let nul3 = chunk[pos..].iter().position(|&b| b == 0).ok_or("png iTXt: missing NUL after translated keyword")?;
            let translated_keyword=control.copy_text(std::str::from_utf8(&chunk[pos..pos+nul3]).map_err(|_|"png iTXt: translated keyword must be UTF-8")?)?;
            pos += nul3 + 1;
            let rest = &chunk[pos..];
            let value_bytes = if compressed_flag {
                let decompressed=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(rest,control.maximum_bytes().saturating_sub(control.owned_bytes()),control)?;
                decompressed
            } else {
                control.copy_bytes(rest)?
            };
            let value = String::from_utf8(value_bytes).map_err(|_| "png iTXt: text must be UTF-8")?;
            let _index = text_chunks.len();
            text_chunks.push(PngTextChunk { keyword, value, compressed: compressed_flag, kind: PngTextKind::IText, language_tag, translated_keyword });

        } else if ty == *b"IDAT" {
            idat.extend_from_slice(chunk);
            seen_idat = true;
        } else if ty == *b"IEND" {

        } else if ty[0].is_ascii_uppercase() {
            return Err(format!("png: unsupported critical chunk {}", String::from_utf8_lossy(&ty)).into());
        } else {
            // 🗃️ Ancillary chunk the codec doesn't specifically model — typed raw-retention,
            // verbatim, in position (the recipe's "nothing real on disk silently dropped" rule).
            ancillary_chunks.push(PngAncillaryChunk { kind: ty, data: control.copy_bytes(chunk)?, after_raster: seen_idat });
        }
    }

    let ihdr = ihdr.ok_or("png: missing IHDR")?;
    if !seen_idat {
        return Err("png: missing IDAT".into());
    }
    if ihdr.color_type == 3 && plte_out.as_ref().is_none_or(Vec::is_empty) {
        return Err("png: color type 3 requires PLTE".into());
    }

    let spp = samples_per_pixel(ihdr.color_type);
    let count = (ihdr.width as usize).checked_mul(ihdr.height as usize).and_then(|n|n.checked_mul(spp)).ok_or("png: native sample count overflow")?;
    if count > 256 * 1024 * 1024 { return Err("png: native sample admission exceeds 512 MiB".into()); }
    let expected=if ihdr.interlace==0 {(packed_row_bytes(ihdr.width,ihdr.color_type,ihdr.bit_depth)+1).checked_mul(ihdr.height as usize).ok_or("png: raw raster extent overflow")?} else {let mut count=0usize;for pass in 0..7 {let(w,h)=adam7_pass_dims(ihdr.width,ihdr.height,pass);if w!=0&&h!=0 {count=count.checked_add((packed_row_bytes(w,ihdr.color_type,ihdr.bit_depth)+1).checked_mul(h as usize).ok_or("png: Adam7 raster extent overflow")?).ok_or("png: Adam7 raster extent overflow")?;}}count};
    let raw=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(&idat,expected,control)?;
    let bpp = bpp_bytes(&ihdr);
    let mut native_samples=control.allocate_vec(count)?;control.begin_stage(count)?;while native_samples.len()<count {let added=(count-native_samples.len()).min(256);native_samples.resize(native_samples.len()+added,0);control.advance(added)?;}

    let mut put_row = |row: &[u8], row_width: usize, base_x: u32, base_y: u32, step_x: u32,control:&mut NativeDecodeControl<'_>| -> Result<(), PngReadError> {
        control.begin_stage(row_width*spp)?;
        for i in 0..row_width {
            let x = base_x + i as u32 * step_x;
            let idx = (base_y as usize * ihdr.width as usize + x as usize) * spp;
            for channel in 0..spp {let ordinal=i*spp+channel;native_samples[idx+channel]=match ihdr.bit_depth {16=>u16::from_be_bytes([row[ordinal*2],row[ordinal*2+1]]),8=>u16::from(row[ordinal]),depth=>{let bit=ordinal*depth as usize;u16::from((row[bit/8]>>(8-depth as usize-bit%8))&((1u8<<depth)-1))}};control.step()?;}
        }
        Ok(())
    };

    if ihdr.interlace == 0 {
        let row_bytes = packed_row_bytes(ihdr.width, ihdr.color_type, ihdr.bit_depth);
        let (rows, consumed) = defilter_pass(&raw, 0, ihdr.height, row_bytes, bpp,control)?;
        if consumed != raw.len() { return Err("png: bytes follow the final native raster row".into()); }
        for (y, row) in rows.iter().enumerate() {
            put_row(row, ihdr.width as usize, 0, y as u32, 1,control)?;
        }
    } else {
        let mut pos = 0usize;
        for (pass, &(sx, sy, stx, sty)) in ADAM7.iter().enumerate() {
            let (pw, ph) = adam7_pass_dims(ihdr.width, ihdr.height, pass);
            if pw == 0 || ph == 0 {
                continue;
            }
            let row_bytes = packed_row_bytes(pw, ihdr.color_type, ihdr.bit_depth);
            let (rows, new_pos) = defilter_pass(&raw, pos, ph, row_bytes, bpp,control)?;
            pos = new_pos;
            for (j, row) in rows.iter().enumerate() {
                put_row(row, pw as usize, sx, sy + j as u32 * sty, stx,control)?;
            }
        }
        if pos != raw.len() { return Err("png: bytes follow the final Adam7 raster row".into()); }
    }

    let image = PngImage {
        width: ihdr.width,
        height: ihdr.height,
        bit_depth: ihdr.bit_depth,
        color_type: PngColorType::from_u8(ihdr.color_type)?,
        interlace: ihdr.interlace == 1,
        palette: plte_out,
        transparency: trns_out,
        gamma: gama_out,
        chromaticities: chrm_out,
        srgb: srgb_out,
        physical_dims: phys_out,
        timestamp: time_out,
        background: bkgd_out,
        text_chunks,
        samples: native_samples,
        ancillary_chunks,
    };
    validate_png_image_controlled(&image,&mut |start,amount|if start {control.begin_stage(amount)}else{control.advance(amount)})?;
    Ok(image)
}
//#endregion Codec
