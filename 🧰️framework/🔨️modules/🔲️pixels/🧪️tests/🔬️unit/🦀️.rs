
use super::*;

fn gradient_checkerboard(w: u32, h: u32) -> RasterImage {
    let mut pixels = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
            pixels.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
        }
    }
    RasterImage { width: w, height: h, pixels }
}

fn lcg_bytes(seed: u64, count: usize) -> Vec<u8> {
    let mut state = seed;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        out.push((state >> 56) as u8);
    }
    out
}

//#region OwnRoundTrip
#[test]
fn gradient_checkerboard_round_trip() {
    let image = gradient_checkerboard(17, 13);
    let encoded = encode_png(&image).expect("encode");
    let decoded = decode_png(&encoded).expect("decode");
    assert_eq!(decoded, image);
}

#[test]
fn solid_color_round_trip() {
    let (w, h) = (4u32, 4u32);
    let pixels: Vec<u8> = (0..w * h).flat_map(|_| [10u8, 20, 30, 255]).collect();
    let image = RasterImage { width: w, height: h, pixels };
    let encoded = encode_png(&image).expect("encode");
    let decoded = decode_png(&encoded).expect("decode");
    assert_eq!(decoded, image);
}

#[test]
fn random_rgba_round_trip() {
    let (w, h) = (23u32, 19u32);
    let pixels = lcg_bytes(0xC0FFEE, (w * h * 4) as usize);
    let image = RasterImage { width: w, height: h, pixels };
    let encoded = encode_png(&image).expect("encode");
    let decoded = decode_png(&encoded).expect("decode");
    assert_eq!(decoded, image);
}

#[test]
fn gray16_round_trip_via_decode() {
    let (w, h) = (5u32, 3u32);
    let samples: Vec<u16> = (0..w * h).map(|i| (i as u16).wrapping_mul(4111)).collect();
    let encoded = encode_png_gray16(w, h, &samples).expect("encode");
    let decoded = decode_png(&encoded).expect("decode");
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    for (i, &sample) in samples.iter().enumerate() {
        let hi = (sample >> 8) as u8;
        assert_eq!(decoded.pixels[i * 4], hi);
        assert_eq!(decoded.pixels[i * 4 + 1], hi);
        assert_eq!(decoded.pixels[i * 4 + 2], hi);
        assert_eq!(decoded.pixels[i * 4 + 3], 255);
    }
}

#[test]
fn scanline_decoder_matches_batch_decode() {
    let image = gradient_checkerboard(31, 11);
    let encoded = encode_png(&image).expect("encode");
    let batch = decode_png(&encoded).expect("batch decode");
    let mut scanline = PngScanlineDecoder::new(&encoded).expect("scanline decoder");
    assert_eq!(scanline.width(), image.width);
    assert_eq!(scanline.height(), image.height);
    let mut rows = Vec::new();
    while let Some(row) = scanline.next_row().expect("next row") {
        rows.push(row);
    }
    assert_eq!(rows.len(), image.height as usize);
    let flattened: Vec<u8> = rows.into_iter().flatten().collect();
    assert_eq!(flattened, batch.pixels);
}

#[test]
fn crc_mismatch_is_rejected() {
    let image = gradient_checkerboard(2, 2);
    let mut encoded = encode_png(&image).expect("encode");
    let last = encoded.len() - 1;
    encoded[last] ^= 0xFF;
    assert!(decode_png(&encoded).is_err());
}

#[test]
fn resize_bilinear_identity_is_copy() {
    let image = gradient_checkerboard(6, 6);
    let resized = resize_bilinear(&image, 6, 6);
    assert_eq!(resized, image);
}

#[test]
fn resize_bilinear_solid_color_stays_solid() {
    let (w, h) = (8u32, 8u32);
    let pixels: Vec<u8> = (0..w * h).flat_map(|_| [200u8, 100, 50, 255]).collect();
    let image = RasterImage { width: w, height: h, pixels };
    let resized = resize_bilinear(&image, 3, 5);
    for chunk in resized.pixels.chunks_exact(4) {
        assert_eq!(chunk, &[200, 100, 50, 255]);
    }
}
//#endregion OwnRoundTrip

//#region OracleDifferential
/// 🔬️ Differential oracle: encode with OUR codec, decode with the third-party `png` crate
/// (dev-dependency only — never a runtime dependency of this crate or any plugin), and vice
/// versa. Deterministic LCG-seeded pixels, no `rand` crate.
#[test]
fn oracle_decodes_our_encode() {
    let (w, h) = (29u32, 17u32);
    let pixels = lcg_bytes(0xA5A5_1234, (w * h * 4) as usize);
    let image = RasterImage { width: w, height: h, pixels: pixels.clone() };
    let encoded = encode_png(&image).expect("our encode");

    let decoder = png::Decoder::new(encoded.as_slice());
    let mut reader = decoder.read_info().expect("oracle read_info");
    let mut buffer = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).expect("oracle next_frame");
    assert_eq!(info.width, w);
    assert_eq!(info.height, h);
    assert_eq!(info.color_type, png::ColorType::Rgba);
    assert_eq!(info.bit_depth, png::BitDepth::Eight);
    assert_eq!(&buffer[..info.buffer_size()], pixels.as_slice());
}

#[test]
fn our_decode_reads_oracle_encode() {
    let (w, h) = (21u32, 25u32);
    let pixels = lcg_bytes(0xFEED_BEEF, (w * h * 4) as usize);
    let mut encoded = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut encoded, w, h);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("oracle write_header");
        writer.write_image_data(&pixels).expect("oracle write_image_data");
    }
    let decoded = decode_png(&encoded).expect("our decode");
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    assert_eq!(decoded.pixels, pixels);
}

#[test]
fn our_decode_reads_oracle_palette_encode() {
    let (w, h) = (6u32, 4u32);
    let palette: Vec<u8> = vec![10, 20, 30, 200, 100, 50, 0, 0, 0, 255, 255, 255];
    let indices: Vec<u8> = (0..w * h).map(|i| (i % 4) as u8).collect();
    let mut encoded = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut encoded, w, h);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_palette(palette.clone());
        let mut writer = encoder.write_header().expect("oracle write_header");
        writer.write_image_data(&indices).expect("oracle write_image_data");
    }
    let decoded = decode_png(&encoded).expect("our decode");
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    for (i, &index) in indices.iter().enumerate() {
        let base = index as usize * 3;
        assert_eq!(&decoded.pixels[i * 4..i * 4 + 3], &palette[base..base + 3]);
        assert_eq!(decoded.pixels[i * 4 + 3], 255);
    }
}

#[test]
fn zlib_compress_decompress_round_trip() {
    let payload = lcg_bytes(0x1357_9BDF, 4096);
    let compressed = deflate::zlib_compress(&payload);
    let decompressed = deflate::zlib_decompress(&compressed).expect("valid zlib stream");
    assert_eq!(decompressed, payload);
}
//#endregion OracleDifferential
