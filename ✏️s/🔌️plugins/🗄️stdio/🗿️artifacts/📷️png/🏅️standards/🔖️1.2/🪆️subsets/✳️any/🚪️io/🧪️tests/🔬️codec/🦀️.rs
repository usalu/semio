
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn gradient_checkerboard_rgba(w: u32, h: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
            out.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
        }
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn canonical_snapshot(w: u32, h: u32, rgba: Vec<u8>) -> PngSnapshot {
    PngSnapshot { schema: crate::STDIO_PNG_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: rgba, ..Default::default() }
}

/// 🔬 The load-bearing regression test: a non-solid image round-tripped through real
/// encode (per-scanline filter selection) and real decode (filter reconstruction).
/// Under the old always-filter-0 encode + no-reconstruction decode this still happened
/// to pass trivially only for solid colors — a gradient/checkerboard is what exposes it.
#[semio_framework_async_macros::async_test]
async fn gradient_checkerboard_round_trip() {
    let (w, h) = (17u32, 13u32);
    let rgba = gradient_checkerboard_rgba(w, h);
    let snap = canonical_snapshot(w, h, rgba.clone());
    let encoded = encode_png(&snap).expect("encode");
    let decoded = decode_png(&encoded).expect("decode");
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    assert_eq!(decoded.pixels, rgba, "decoded pixels must exactly match the original");
}

#[semio_framework_async_macros::async_test]
async fn solid_color_round_trip_still_works() {
    let (w, h) = (4u32, 4u32);
    let rgba: Vec<u8> = (0..w * h).flat_map(|_| [10u8, 20, 30, 255]).collect();
    let snap = canonical_snapshot(w, h, rgba.clone());
    let encoded = encode_png(&snap).expect("encode");
    let decoded = decode_png(&encoded).expect("decode");
    assert_eq!(decoded.pixels, rgba);
}

#[semio_framework_async_macros::async_test]
async fn crc_mismatch_is_rejected() {
    let (w, h) = (2u32, 2u32);
    let rgba = gradient_checkerboard_rgba(w, h);
    let snap = canonical_snapshot(w, h, rgba);
    let mut encoded = encode_png(&snap).expect("encode");
    let flip_at = 8 + 4 + 4 + 6; // a few bytes into the IHDR chunk's data
    encoded[flip_at] ^= 0xFF;
    let err = decode_png(&encoded).unwrap_err();
    assert!(err.contains("CRC") || err.contains("crc") || err.contains("truncated"), "unexpected error: {err}");
}

#[semio_framework_async_macros::async_test]
async fn sniff_rejects_non_png_bytes() {
    let err = decode_png(b"not a png at all").unwrap_err();
    assert!(err.contains("signature"));
}

//#region ColorTypeFixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn hand_encode(width: u32, height: u32, bit_depth: u8, color_type: u8, plte: Option<&[u8]>, trns: Option<&[u8]>, raw_rows: &[u8]) -> Vec<u8> {
    let bpp = bpp_bytes(&Ihdr { width, height, bit_depth, color_type, interlace: 0 });
    let row_bytes = packed_row_bytes(width, color_type, bit_depth);
    assert_eq!(raw_rows.len(), row_bytes * height as usize);
    let mut idat = Vec::new();
    let mut prev: Option<Vec<u8>> = None;
    for y in 0..height as usize {
        let row = &raw_rows[y * row_bytes..(y + 1) * row_bytes];
        let (ft, filtered) = choose_filter(row, prev.as_deref(), bpp);
        idat.push(ft);
        idat.extend_from_slice(&filtered);
        prev = Some(row.to_vec());
    }
    let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&idat).unwrap();
    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[bit_depth, color_type, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    if let Some(p) = plte {
        write_chunk(&mut out, b"PLTE", p);
    }
    if let Some(t) = trns {
        write_chunk(&mut out, b"tRNS", t);
    }
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);
    out
}

#[semio_framework_async_macros::async_test]
async fn color_type_0_grayscale() {
    // 4x1, bit depth 8: values 0, 85, 170, 255
    let raw = vec![0u8, 85, 170, 255];
    let bytes = hand_encode(4, 1, 8, 0, None, None, &raw);
    let snap = decode_png(&bytes).expect("decode grayscale");
    let expected: Vec<u8> = raw.iter().flat_map(|&g| [g, g, g, 255]).collect();
    assert_eq!(snap.pixels, expected);
    assert_eq!(snap.bit_depth, 8);
    assert_eq!(snap.color_type, PngColorType::Grayscale);
    assert!(!snap.interlace);
}

#[semio_framework_async_macros::async_test]
async fn color_type_2_rgb() {
    let raw = vec![10u8, 20, 30, 40, 50, 60]; // 2x1 RGB
    let bytes = hand_encode(2, 1, 8, 2, None, None, &raw);
    let snap = decode_png(&bytes).expect("decode rgb");
    assert_eq!(snap.pixels, vec![10, 20, 30, 255, 40, 50, 60, 255]);
    assert_eq!(snap.color_type, PngColorType::Rgb);
}

#[semio_framework_async_macros::async_test]
async fn color_type_3_palette_with_trns() {
    // palette of 3 entries; tRNS makes entry 1 half-transparent, entry 2 fully so
    let plte = [255u8, 0, 0, 0, 255, 0, 0, 0, 255]; // red, green, blue
    let trns = [255u8, 128, 0];
    let raw = vec![0u8, 1, 2, 0]; // 4x1 indices, bit depth 8
    let bytes = hand_encode(4, 1, 8, 3, Some(&plte), Some(&trns), &raw);
    let snap = decode_png(&bytes).expect("decode palette+trns");
    assert_eq!(snap.pixels, vec![255, 0, 0, 255, 0, 255, 0, 128, 0, 0, 255, 0, 255, 0, 0, 255,]);
    assert_eq!(snap.color_type, PngColorType::Palette);
    assert_eq!(snap.plte.as_ref().expect("plte retained").len(), 3);
    assert_eq!(snap.trns, Some(PngTransparency::Indexed { alpha: vec![255, 128, 0] }));
}

/// 🚫 §11.3.3 forbids tRNS alongside colour types 4 and 6, and `encode_png` always writes
/// colour type 6 — so the chunk is omitted rather than emitted into a file no conforming
/// decoder would accept. The alpha it carried is not lost: `decode_png` already resolved it
/// into `pixels`, which is what the re-encode carries forward.
#[test]
fn trns_is_not_re_emitted_alongside_the_canonical_rgba_colour_type() {
    let plte = [255u8, 0, 0, 0, 255, 0, 0, 0, 255];
    let trns = [255u8, 128, 0];
    let raw = vec![0u8, 1, 2, 0];
    let snap = decode_png(&hand_encode(4, 1, 8, 3, Some(&plte), Some(&trns), &raw)).expect("decode palette+trns");
    assert!(snap.trns.is_some(), "the source really does carry a tRNS chunk");

    let reencoded = encode_png(&snap).expect("re-encode a snapshot whose tRNS cannot be represented at colour type 6");
    assert!(!chunk_types(&reencoded).iter().any(|kind| kind == b"tRNS"), "colour type 6 output must carry no tRNS chunk, got {:?}", chunk_types(&reencoded).iter().map(|k| String::from_utf8_lossy(k).into_owned()).collect::<Vec<_>>());

    let redecoded = decode_png(&reencoded).expect("re-decode");
    assert_eq!(redecoded.pixels, snap.pixels, "the resolved alpha survives in the raster even though the chunk does not");
    assert_eq!(redecoded.trns, None);
}

/// 📇️ Every chunk type in a PNG byte stream, in file order — §5.3's `length|type|data|crc`.
fn chunk_types(bytes: &[u8]) -> Vec<[u8; 4]> {
    let mut out = Vec::new();
    let mut cursor = 8usize;
    while cursor + 8 <= bytes.len() {
        let length = u32::from_be_bytes(bytes[cursor..cursor + 4].try_into().expect("4-byte length")) as usize;
        let kind: [u8; 4] = bytes[cursor + 4..cursor + 8].try_into().expect("4-byte type");
        out.push(kind);
        if &kind == b"IEND" {
            break;
        }
        cursor += 12 + length;
    }
    out
}

#[semio_framework_async_macros::async_test]
async fn color_type_3_sub_byte_indices() {
    // bit depth 2, 4 indices packed into a single byte: 0,1,2,3 -> 0b00_01_10_11 = 0x1B
    let plte = [0u8, 0, 0, 64, 64, 64, 128, 128, 128, 255, 255, 255];
    let raw = vec![0b00_01_10_11u8];
    let bytes = hand_encode(4, 1, 2, 3, Some(&plte), None, &raw);
    let snap = decode_png(&bytes).expect("decode 2-bit palette");
    assert_eq!(snap.pixels, vec![0, 0, 0, 255, 64, 64, 64, 255, 128, 128, 128, 255, 255, 255, 255, 255,]);
    assert_eq!(snap.bit_depth, 2);
}

#[semio_framework_async_macros::async_test]
async fn color_type_4_grayscale_alpha() {
    let raw = vec![100u8, 200, 50, 10]; // 2x1: (gray,alpha) pairs
    let bytes = hand_encode(2, 1, 8, 4, None, None, &raw);
    let snap = decode_png(&bytes).expect("decode grayscale+alpha");
    assert_eq!(snap.pixels, vec![100, 100, 100, 200, 50, 50, 50, 10]);
    assert_eq!(snap.color_type, PngColorType::GrayscaleAlpha);
}

#[semio_framework_async_macros::async_test]
async fn color_type_6_rgba_bit_depth_16() {
    // 1x1 pixel, 16-bit RGBA; high byte should be what survives scale_to_8
    let raw = vec![0x12u8, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let bytes = hand_encode(1, 1, 16, 6, None, None, &raw);
    let snap = decode_png(&bytes).expect("decode 16-bit rgba");
    assert_eq!(snap.pixels, vec![0x12, 0x56, 0x9A, 0xDE]);
    assert_eq!(snap.bit_depth, 16);
}
//#endregion ColorTypeFixtures

//#region AncillaryFixtures
/// 🧪 A hand-encoded file exercising gAMA/cHRM/sRGB/pHYs/tIME/bKGD/tEXt/zTXt/iTXt plus one
/// genuinely unknown private chunk — proves decode both TYPES every known ancillary field
/// AND retains the unknown one verbatim, in the real relative chunk order.
#[semio_framework_async_macros::async_test]
async fn ancillary_chunks_round_trip_typed_and_unknown() {
    let raw = vec![0u8, 0, 0, 255]; // 1x1 opaque black RGBA8
    let bpp = 4usize;
    let (ft, filtered) = choose_filter(&raw, None, bpp);
    let mut idat_raw = vec![ft];
    idat_raw.extend_from_slice(&filtered);
    let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&idat_raw).unwrap();

    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&1u32.to_be_bytes());
    ihdr.extend_from_slice(&1u32.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"gAMA", &45455u32.to_be_bytes());
    let mut chrm = Vec::new();
    for v in [31270u32, 32900, 64000, 33000, 30000, 60000, 15000, 6000] {
        chrm.extend_from_slice(&v.to_be_bytes());
    }
    write_chunk(&mut out, b"cHRM", &chrm);
    write_chunk(&mut out, b"sRGB", &[0]);
    let mut phys = Vec::new();
    phys.extend_from_slice(&2835u32.to_be_bytes());
    phys.extend_from_slice(&2835u32.to_be_bytes());
    phys.push(1);
    write_chunk(&mut out, b"pHYs", &phys);
    let mut time = Vec::new();
    time.extend_from_slice(&2024u16.to_be_bytes());
    time.extend_from_slice(&[6, 15, 12, 30, 0]);
    write_chunk(&mut out, b"tIME", &time);
    write_chunk(&mut out, b"bKGD", &1u16.to_be_bytes().iter().chain(2u16.to_be_bytes().iter()).chain(3u16.to_be_bytes().iter()).copied().collect::<Vec<u8>>());
    let text = b"Title\0hello".to_vec();
    write_chunk(&mut out, b"tEXt", &text);
    write_chunk(&mut out, b"prIV", &[9, 9, 9]); // genuinely unknown private ancillary chunk
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);

    let snap = decode_png(&out).expect("decode ancillary fixture");
    assert_eq!(snap.gama, Some(45455));
    assert_eq!(snap.chrm.as_ref().map(|c| c.white_x), Some(31270));
    assert_eq!(snap.srgb, Some(PngSrgbIntent::Perceptual));
    assert_eq!(snap.phys.as_ref().map(|p| p.unit_is_meter), Some(true));
    assert_eq!(snap.time.as_ref().map(|t| (t.year, t.month, t.day)), Some((2024, 6, 15)));
    assert!(matches!(snap.bkgd, Some(PngBackground::Rgb { r: 1, g: 2, b: 3 })));
    assert_eq!(snap.text_chunks.len(), 1);
    assert_eq!(snap.text_chunks[0].keyword, "Title");
    assert_eq!(snap.text_chunks[0].value, "hello");
    assert_eq!(snap.unknown_chunks.len(), 1);
    assert_eq!(&snap.unknown_chunks[0].kind, b"prIV");
    assert_eq!(snap.unknown_chunks[0].data, vec![9, 9, 9]);
    // Chunk order must reflect the real on-disk sequence.
    assert_eq!(
        snap.chunk_order,
        vec![
            PngChunkMarker::Ihdr,
            PngChunkMarker::Gama,
            PngChunkMarker::Chrm,
            PngChunkMarker::Srgb,
            PngChunkMarker::Phys,
            PngChunkMarker::Time,
            PngChunkMarker::Bkgd,
            PngChunkMarker::Text { index: 0 },
            PngChunkMarker::Unknown { index: 0 },
            PngChunkMarker::Idat,
            PngChunkMarker::Iend,
        ]
    );

    // Re-encode must still honestly re-emit every ancillary/text/unknown chunk (pixel data
    // canonicalizes per EncodeScopeNote, everything else round-trips).
    let reencoded = encode_png(&snap).expect("re-encode");
    let redecoded = decode_png(&reencoded).expect("re-decode");
    assert_eq!(redecoded.gama, snap.gama);
    assert_eq!(redecoded.chrm, snap.chrm);
    assert_eq!(redecoded.srgb, snap.srgb);
    assert_eq!(redecoded.phys, snap.phys);
    assert_eq!(redecoded.time, snap.time);
    assert_eq!(redecoded.bkgd, snap.bkgd);
    assert_eq!(redecoded.text_chunks, snap.text_chunks);
    assert_eq!(redecoded.unknown_chunks, snap.unknown_chunks);
    assert_eq!(redecoded.pixels, snap.pixels);
}

#[semio_framework_async_macros::async_test]
async fn ztxt_and_itxt_round_trip() {
    // zTXt: keyword\0 + compression-method(0).await + zlib(value)
    let mut ztxt = b"Comment\0\0".to_vec();
    ztxt.extend_from_slice(&semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(b"compressed value").unwrap());
    // iTXt (compressed): keyword\0 + flag(1) + method(0).await + lang\0 + translated\0 + zlib(value)
    let mut itxt = b"Title\0".to_vec();
    itxt.push(1);
    itxt.push(0);
    itxt.extend_from_slice(b"en\0");
    itxt.extend_from_slice("Titre".as_bytes());
    itxt.push(0);
    itxt.extend_from_slice(&semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress("valeur".as_bytes()).unwrap());

    let raw = vec![0u8, 0, 0, 255];
    let (ft, filtered) = choose_filter(&raw, None, 4);
    let mut idat_raw = vec![ft];
    idat_raw.extend_from_slice(&filtered);
    let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&idat_raw).unwrap();

    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&1u32.to_be_bytes());
    ihdr.extend_from_slice(&1u32.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"zTXt", &ztxt);
    write_chunk(&mut out, b"iTXt", &itxt);
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);

    let snap = decode_png(&out).expect("decode zTXt/iTXt fixture");
    assert_eq!(snap.text_chunks.len(), 2);
    assert_eq!(snap.text_chunks[0].keyword, "Comment");
    assert_eq!(snap.text_chunks[0].value, "compressed value");
    assert_eq!(snap.text_chunks[0].kind, PngTextKind::ZText);
    assert!(snap.text_chunks[0].compressed);
    assert_eq!(snap.text_chunks[1].keyword, "Title");
    assert_eq!(snap.text_chunks[1].value, "valeur");
    assert_eq!(snap.text_chunks[1].kind, PngTextKind::IText);
    assert_eq!(snap.text_chunks[1].language_tag, "en");
    assert_eq!(snap.text_chunks[1].translated_keyword, "Titre");
}
//#endregion AncillaryFixtures

//#region Adam7Fixture
/// 🧪 Test-only Adam7 *encoder*, used solely to build a genuinely interlaced fixture to
/// prove `decode_png` de-interlaces correctly. Production `encode_png` intentionally
/// always emits interlace method 0 (see 🚫️EncodeScopeNote on `encode_png`); this helper
/// is not exposed outside tests.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn adam7_encode_fixture(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let bpp = 4usize;
    let mut idat = Vec::new();
    for pass in 0..7 {
        let (pw, ph) = adam7_pass_dims(width, height, pass);
        if pw == 0 || ph == 0 {
            continue;
        }
        let (sx, sy, stx, sty) = ADAM7[pass];
        let mut prev: Option<Vec<u8>> = None;
        for j in 0..ph {
            let mut row = Vec::with_capacity(pw as usize * bpp);
            for i in 0..pw {
                let x = sx + i * stx;
                let y = sy + j * sty;
                let idx = ((y * width + x) * 4) as usize;
                row.extend_from_slice(&rgba[idx..idx + 4]);
            }
            let (ft, filtered) = choose_filter(&row, prev.as_deref(), bpp);
            idat.push(ft);
            idat.extend_from_slice(&filtered);
            prev = Some(row);
        }
    }
    let compressed = semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&idat).unwrap();
    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 1]); // interlace method 1 = Adam7
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);
    out
}

#[semio_framework_async_macros::async_test]
async fn adam7_interlaced_decode_round_trip() {
    let (w, h) = (9u32, 11u32); // deliberately not a multiple of 8, exercises partial passes
    let rgba = gradient_checkerboard_rgba(w, h);
    let bytes = adam7_encode_fixture(w, h, &rgba);
    let snap = decode_png(&bytes).expect("decode adam7");
    assert_eq!(snap.width, w);
    assert_eq!(snap.height, h);
    assert!(snap.interlace, "interlace flag must be decoded as true");
    assert_eq!(snap.pixels, rgba, "adam7 de-interlace must reconstruct the exact original raster");
}
//#endregion Adam7Fixture
