use crate::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v_jfif_1_01::subsets::document::io::*;
use crate::schema::snapshot::demo_jpg_snapshot;
use semio_framework_test_fixtures::fixture_text;

#[test]
fn controlled_components_preserve_gray_and_ycbcr_samples_before_color_conversion() {
    use crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::decoded_components::JpgComponentDecoder;
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧾️document/🚪️io/💾️binary/📸️snapshot/🧮️decoded-components/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let (width,height,sample) = (fixture["width"].as_u64().unwrap() as u32,fixture["height"].as_u64().unwrap() as u32,fixture["sample"].as_u64().unwrap() as u8);
    for case in fixture["cases"].as_array().unwrap() {
        let ids = case["componentIds"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>();
        let mut snapshot = JpgSnapshot { schema: crate::STDIO_JPG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::JpgImage { width,height,pixels:vec![sample;(width*height*4) as usize],..Default::default() } };
        for pixel in snapshot.image.pixels.chunks_exact_mut(4){pixel[3]=255;}
        let options=JpgEncodeOptions{quality:90,components:ids.iter().map(|id|JpgEncodeComponent{id:*id,h_sampling:1,v_sampling:1}).collect()};
        let bytes=encode_jpg(&snapshot, &options).unwrap();
        let mut decoder=JpgComponentDecoder::new(&bytes.as_slice(),1024*1024,&mut ||false).unwrap();
        let before=decoder.progress();assert!(decoder.step(&bytes.as_slice(),0,&mut ||false).unwrap().is_none());assert_eq!(decoder.progress(),before);
        let projected=loop {let before=decoder.progress().0;if let Some(value)=decoder.step(&bytes.as_slice(),1,&mut ||false).unwrap(){break value;}assert!(decoder.progress().0<=before+1);};
        assert_eq!((projected.width,projected.height),(width,height));assert_eq!(projected.component_ids,ids);
        assert_eq!(projected.samples,vec![sample;(width*height) as usize*case["samplesPerPixel"].as_u64().unwrap() as usize]);
        let (oracle_width,oracle_height,oracle)=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_decode_luma(&bytes).unwrap();
        assert_eq!((oracle_width,oracle_height),(width,height));assert_eq!(oracle,vec![sample;(width*height) as usize]);
        if projected.component_ids.len()==1 {assert_eq!(projected.samples,oracle);}
        assert_eq!(decoder.progress().0,decoder.progress().1);
        let mut cancelled=JpgComponentDecoder::new(&bytes.as_slice(),1024*1024,&mut ||false).unwrap();
        assert_eq!(cancelled.step(&bytes.as_slice(),1,&mut ||true).unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);
        assert!(JpgComponentDecoder::new(&bytes.as_slice(),1,&mut ||false).is_err());
        assert_eq!(JpgComponentDecoder::new(&bytes.as_slice(),1024*1024,&mut ||true).err().unwrap().kind,semio_framework_value::ValueRefusalKind::Canceled);
    }
}

#[test]
fn controlled_components_accept_authored_native_bytes_with_independent_reader() {
    use crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::decoded_components::JpgComponentDecoder;
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧾️document/🚪️io/💾️binary/📸️snapshot/🧮️decoded-components/🧪️tests/🧫️fixtures/🧮️owned-exact-components/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let hex=case["jpegHex"].as_str().unwrap();
        let bytes=(0..hex.len()).step_by(2).map(|index|u8::from_str_radix(&hex[index..index+2],16).unwrap()).collect::<Vec<_>>();
        let mut decoder=JpgComponentDecoder::new(&bytes.as_slice(),1024*1024,&mut ||false).unwrap();
        assert!(decoder.working_bytes()>=bytes.len()*8);
        let actual=loop {if let Some(value)=decoder.step(&bytes.as_slice(),1,&mut ||false).unwrap(){break value;}};
        assert_eq!(actual.width,u32::try_from(case["width"].as_u64().unwrap()).unwrap());
        assert_eq!(actual.height,u32::try_from(case["height"].as_u64().unwrap()).unwrap());
        assert_eq!(actual.component_ids,case["componentIds"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>());
        assert_eq!(actual.samples,case["samples"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>());
        let (width,height,oracle)=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_decode_luma(&bytes).unwrap();
        assert_eq!((width,height),(actual.width,actual.height));
        if actual.component_ids.len()==1 {assert_eq!(actual.samples,oracle);}
        if actual.samples.iter().all(|sample|*sample==128) {assert_eq!(oracle,vec![128;(width*height) as usize]);}
    }
}

#[test]
fn export_quality_is_physical_policy_with_independent_quantization_witness() {
    use semio_framework_value::ToValue;
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧾️document/🚪️io/💾️binary/📸️snapshot/🎛️encode-options/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let snapshot=demo_jpg_snapshot();let owned=snapshot.to_value();
    assert!(!semio_framework_pack_json::to_json_string(&owned).contains("reEncodeQuality"));
    let mut encoded=Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let quality=case["quality"].as_u64().unwrap() as u8;
        let options=JpgEncodeOptions{quality,..JpgEncodeOptions::default()};
        let bytes=encode_jpg(&snapshot,&options).unwrap();
        let header=parse_jpg_header(&bytes.as_slice(),false).unwrap();
        let reference=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_encode_rgba(snapshot.image.width,snapshot.image.height,&snapshot.image.pixels,quality).unwrap();
        let reference_header=parse_jpg_header(&reference.as_slice(),false).unwrap();
        let expected=case["firstLumaQuantizer"].as_u64().unwrap() as u16;
        assert_eq!(header.quant_tables.iter().find(|table|table.id==0).unwrap().values[0],expected);
        assert_eq!(reference_header.quant_tables.iter().find(|table|table.id==0).unwrap().values[0],expected);
        assert_eq!(snapshot.to_value(),owned);
        encoded.push(bytes);
    }
    assert_ne!(encoded[0],encoded[1]);
    for quality in [0,101,255] {assert!(encode_jpg(&snapshot,&JpgEncodeOptions{quality,..Default::default()}).is_err());}
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn gradient_image(w: u32, h: u32) -> Vec<u8> {
    let mut out = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * 4) as usize;
            out[idx] = ((x * 255) / w.max(1)) as u8;
            out[idx + 1] = ((y * 255) / h.max(1)) as u8;
            out[idx + 2] = (((x + y) * 255) / (w + h).max(1)) as u8;
            out[idx + 3] = 255;
        }
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn checkerboard_image(w: u32, h: u32) -> Vec<u8> {
    let mut out = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let idx = ((y * w + x) * 4) as usize;
            let on = ((x / 8) + (y / 8)) % 2 == 0;
            let v = if on { 230u8 } else { 20u8 };
            out[idx] = v;
            out[idx + 1] = v;
            out[idx + 2] = v;
            out[idx + 3] = 255;
        }
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn mae(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mut sum = 0f64;
    let mut n = 0usize;
    for i in (0..a.len()).step_by(4) {
        for c in 0..3 {
            sum += (a[i + c] as i32 - b[i + c] as i32).abs() as f64;
            n += 1;
        }
    }
    sum / n as f64
}

#[semio_framework_async_macros::async_test]
async fn idct_fdct_is_identity() {
    let mut block = [0f64; 64];
    for (i, v) in block.iter_mut().enumerate() {
        *v = ((i * 37 % 255) as f64) - 128.0;
    }
    let coeff = fdct_8x8(&block);
    let recon = idct_8x8(&coeff);
    let maxerr = block.iter().zip(recon.iter()).fold(0f64, |m, (a, b)| m.max((a - b).abs()));
    assert!(maxerr < 1e-6, "maxerr={maxerr}");
}

#[semio_framework_async_macros::async_test]
async fn huffman_round_trips_all_dc_luma_symbols() {
    let table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
    let mut bw = BitWriter::new();
    for v in 0u8..=11 {
        let (l, c) = *table.encode.get(&v).unwrap();
        bw.put_bits(c, l);
    }
    bw.flush();
    let source: &[u8] = &bw.bytes;
    let mut br = BitReader::new(&source, 0);
    for v in 0u8..=11 {
        assert_eq!(br.decode_symbol(&table).unwrap(), v);
    }
}

#[semio_framework_async_macros::async_test]
async fn single_block_round_trips_through_huffman() {
    let dc_table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
    let ac_table = build_huffman(&AC_LUMA_BITS, &ac_luma_values()).unwrap();
    let mut zz = [0i32; 64];
    zz[0] = 120;
    zz[1] = 5;
    zz[2] = -3;
    zz[20] = 1;
    let mut bw = BitWriter::new();
    let mut dc_pred = 0i32;
    encode_block(&mut bw, &zz, &mut dc_pred, &dc_table, &ac_table).unwrap();
    bw.flush();
    let source: &[u8] = &bw.bytes;
    let mut br = BitReader::new(&source, 0);
    let mut dc_pred2 = 0i32;
    let decoded = decode_block(&mut br, &mut dc_pred2, &dc_table, &ac_table).unwrap();
    assert_eq!(decoded, zz);
}

/// 🖼️ Non-solid-color round trip — the case the old "solid-color only"
/// codec could never have passed. Gradient exercises AC energy across
/// every block; asserts mean-absolute-pixel-error stays well under a
/// visually-lossless budget of 10/255.
#[semio_framework_async_macros::async_test]
async fn gradient_round_trip_under_mae_threshold() {
    let (w, h) = (48u32, 40u32);
    let img = gradient_image(w, h);
    let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::JpgImage { width: w,height: h,pixels: img.clone(),..crate::schema::snapshot::JpgImage::default() } };
    let bytes = encode_jpg(&snap, &crate::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::default()).expect("encode");
    assert!(bytes.starts_with(&[0xFF, 0xD8]));
    assert!(bytes.ends_with(&[0xFF, 0xD9]));
    let decoded = decode_jpg(&bytes).expect("decode");
    assert_eq!(decoded.image.width, w);
    assert_eq!(decoded.image.height, h);
    let err = mae(&img, &decoded.image.pixels);
    println!("gradient round-trip MAE = {err}");
    assert!(err < 10.0, "gradient MAE too high: {err}");
}

/// 🖼️ Checkerboard: high-frequency content, harder for quantization to
/// preserve than a gradient — same bar (MAE < 10/255).
#[semio_framework_async_macros::async_test]
async fn checkerboard_round_trip_under_mae_threshold() {
    let (w, h) = (32u32, 32u32);
    let img = checkerboard_image(w, h);
    let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::JpgImage { width: w,height: h,pixels: img.clone(),..crate::schema::snapshot::JpgImage::default() } };
    let bytes = encode_jpg(&snap, &crate::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::default()).expect("encode");
    let decoded = decode_jpg(&bytes).expect("decode");
    let err = mae(&img, &decoded.image.pixels);
    println!("checkerboard round-trip MAE = {err}");
    assert!(err < 10.0, "checkerboard MAE too high: {err}");
}

#[semio_framework_async_macros::async_test]
async fn solid_color_still_round_trips() {
    let (w, h) = (16u32, 16u32);
    let mut img = vec![0u8; (w * h * 4) as usize];
    for px in img.chunks_mut(4) {
        px[0] = 200;
        px[1] = 100;
        px[2] = 50;
        px[3] = 255;
    }
    let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::JpgImage { width: w,height: h,pixels: img.clone(),..crate::schema::snapshot::JpgImage::default() } };
    let bytes = encode_jpg(&snap, &crate::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::default()).expect("encode");
    let decoded = decode_jpg(&bytes).expect("decode");
    let err = mae(&img, &decoded.image.pixels);
    assert!(err < 5.0, "solid MAE too high: {err}");
}

/// 🚫 Progressive (SOF2) must be a typed `Unsupported` error, never
/// silently decoded — hand-crafted minimal SOF2 segment.
#[semio_framework_async_macros::async_test]
async fn progressive_sof2_is_explicit_unsupported() {
    let mut bytes = vec![0xFFu8, 0xD8];
    bytes.extend_from_slice(&[0xFF, 0xC2, 0x00, 0x0B, 0x08, 0x00, 0x08, 0x00, 0x08, 0x01, 0x01, 0x11, 0x00]);
    bytes.extend_from_slice(&[0xFF, 0xD9]);
    let result = decode_jpg(&bytes);
    assert!(matches!(result, Err(JpgError::Unsupported(_))), "expected Unsupported, got {result:?}");
}

#[semio_framework_async_macros::async_test]
async fn non_jpeg_input_is_malformed_not_panic() {
    let result = decode_jpg(&[0x00, 0x01, 0x02, 0x03]);
    assert!(matches!(result, Err(JpgError::Malformed(_))));
}
