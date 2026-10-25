//! 📤️ PNG encoding core of the pixels codec: the RGBA and gray16 encoders, the IDAT packer, the PNG assembler and the whole-image decoder. See [`super`].

use super::*;

//#region PngCodec
/// 📕️ Encodes an 8-bit RGBA image as a minimal, spec-conformant PNG: IHDR (color type 6 / bit
/// depth 8 / interlace 0), one adaptively-filtered IDAT, IEND. No ancillary chunks — none of this
/// crate's callers need gAMA/cHRM/sRGB/pHYs/tIME/bKGD/text metadata round-tripped.
pub fn encode_png(image: &RasterImage) -> Result<Vec<u8>, RasterError> {
    let expected_len = (image.width as usize).checked_mul(image.height as usize).and_then(|p| p.checked_mul(4)).ok_or_else(|| RasterError::Codec("dimensions overflow".into()))?;
    if image.pixels.len() != expected_len {
        return Err(RasterError::Codec("pixels length mismatch".into()));
    }
    let idat = pack_idat(image.width, image.height, 6, 8, &image.pixels, 4);
    let compressed = deflate::zlib_compress(&idat);
    Ok(assemble_png(image.width, image.height, 6, 8, &compressed))
}

/// 🎨️ Encodes row-major big-endian 16-bit grayscale samples as a 16-bit grayscale PNG (color
/// type 0), for lossless heightfield/DSM-style export.
pub fn encode_png_gray16(width: u32, height: u32, samples: &[u16]) -> Result<Vec<u8>, RasterError> {
    let expected_len = (width as usize).checked_mul(height as usize).ok_or_else(|| RasterError::Codec("dimensions overflow".into()))?;
    if width == 0 || height == 0 || samples.len() != expected_len {
        return Err(RasterError::Codec("samples length mismatch".into()));
    }
    let mut packed = Vec::with_capacity(samples.len() * 2);
    for &sample in samples {
        packed.extend_from_slice(&sample.to_be_bytes());
    }
    let idat = pack_idat(width, height, 0, 16, &packed, 2);
    let compressed = deflate::zlib_compress(&idat);
    Ok(assemble_png(width, height, 0, 16, &compressed))
}

fn pack_idat(width: u32, height: u32, _color_type: u8, _bit_depth: u8, packed: &[u8], bpp: usize) -> Vec<u8> {
    let row_bytes = width as usize * bpp;
    let mut idat = Vec::with_capacity((row_bytes + 1) * height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for y in 0..height as usize {
        let row = &packed[y * row_bytes..(y + 1) * row_bytes];
        let (ft, filtered) = choose_filter(row, prev.as_deref(), bpp);
        idat.push(ft);
        idat.extend_from_slice(&filtered);
        prev = Some(row.to_vec());
    }
    idat
}

fn assemble_png(width: u32, height: u32, color_type: u8, bit_depth: u8, compressed_idat: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[bit_depth, color_type, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"IDAT", compressed_idat);
    write_chunk(&mut out, b"IEND", &[]);
    out
}

/// 📥️ Reconstructs cold synchronous callers through the existing scanline codec.
/// Caller-scheduled image preparation uses [png_decoding::PngDecodeJob] with explicit grants.
pub fn decode_png(data: &[u8]) -> Result<RasterImage, RasterError> {
    if data.len() > 67108864 { return Err(RasterError::Codec("PNG byte limit exceeded".into())); }
    let mut source = PngScanlineDecoder::new(data)?;
    let (width,height) = (source.width(),source.height());
    let extent = (width as usize).checked_mul(height as usize).filter(|pixels| *pixels <= 16777216).and_then(|pixels|pixels.checked_mul(4)).ok_or_else(||RasterError::Codec("PNG pixel limit exceeded".into()))?;
    let mut image = RasterImage {width,height,pixels:Vec::with_capacity(extent)};
    while let Some(row) = source.next_row()? { image.pixels.extend_from_slice(&row); }
    if image.pixels.len() != extent { return Err(RasterError::Codec("PNG decoded extent mismatch".into())); }
    Ok(image)
}
//#endregion PngCodec
