use super::*;
use crate::schema::demo_jpg_snapshot;

#[test]fn neutral_diff_framing_refuses_noncanonical_domains_without_allocation_panics(){
 use protocol::DiffBinary;let fixture:serde_json::Value=serde_json::from_str(include_str!("../../💾️binary/🔺️diff/🧫️fixtures/🛡️framing/🔣️.json")).unwrap();for vector in fixture["reject"].as_array().unwrap(){let bytes:Vec<u8>=serde_json::from_value(vector["bytes"].clone()).unwrap();let refusal=std::panic::catch_unwind(||crate::JpgDiff::decode_diff(&bytes));assert!(refusal.is_ok(),"{} allocated from an unadmitted count",vector["name"]);assert!(refusal.unwrap().is_err(),"{}",vector["name"]);}eprintln!("[DEBUG] JPEG neutral sparse framing rejects seven unadmitted native/domain/extent forms");
}

#[test]
fn independently_encoded_file_reopens_all_imported_semantic_metadata(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/🧬️mutations/🖼️replace-image/🧫️fixtures/🔣️.json")).unwrap();let image:crate::JpgImage=semio_framework_pack_json::from_json_str(&fixture["importedImage"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let independent=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_encode_rgba(image.width,image.height,&image.pixels,90).unwrap();assert_eq!(&independent[..4],&[255,216,255,224]);let old_end=4+usize::from(u16::from_be_bytes([independent[4],independent[5]]));
 let thumbnail=image.jfif_thumbnail.as_ref().unwrap();let mut payload=b"JFIF\0".to_vec();payload.extend_from_slice(&[image.jfif_version.0,image.jfif_version.1,2]);payload.extend_from_slice(&image.jfif_x_density.to_be_bytes());payload.extend_from_slice(&image.jfif_y_density.to_be_bytes());payload.extend_from_slice(&[thumbnail.width,thumbnail.height]);payload.extend_from_slice(&thumbnail.rgb_data);
 let mut native=vec![255,216,255,224];native.extend_from_slice(&u16::try_from(payload.len()+2).unwrap().to_be_bytes());native.extend(payload);for segment in &image.other_segments{native.extend_from_slice(&[255,segment.marker]);native.extend_from_slice(&u16::try_from(segment.data.len()+2).unwrap().to_be_bytes());native.extend_from_slice(&segment.data);}native.extend_from_slice(&independent[old_end..]);
 let reopened=decode_jpg(&native).unwrap();assert_eq!((reopened.image.width,reopened.image.height),(image.width,image.height));assert_eq!(reopened.image.jfif_version,image.jfif_version);assert_eq!(reopened.image.jfif_density_units,image.jfif_density_units);assert_eq!((reopened.image.jfif_x_density,reopened.image.jfif_y_density),(image.jfif_x_density,image.jfif_y_density));assert_eq!(reopened.image.jfif_thumbnail,image.jfif_thumbnail);assert_eq!(reopened.image.other_segments,image.other_segments);
 let restored=decode_jpg(&encode_jpg(&reopened,&JpgEncodeOptions::default()).unwrap()).unwrap();assert_eq!(restored.image.jfif_thumbnail,image.jfif_thumbnail);assert_eq!(restored.image.other_segments,image.other_segments);assert_eq!(restored.image.jfif_x_density,image.jfif_x_density);eprintln!("[DEBUG] independent image JPEG reopened exact density, thumbnail and ordered duplicate APP/COM metadata");
}

#[test]
fn controlled_components_preserve_gray_and_ycbcr_samples_before_color_conversion() {
    use super::binary::snapshot::decoded_components::JpgComponentDecoder;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../💾️binary/📸️snapshot/🧮️decoded-components/🧫️fixtures/🔣️.json")).unwrap();
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
        eprintln!("[DEBUG] physical JPEG component projection retained {} native channels with exact gray oracle and bounded cancellation",projected.component_ids.len());
    }
}

#[test]
fn controlled_components_accept_authored_native_bytes_with_independent_reader() {
    use super::binary::snapshot::decoded_components::JpgComponentDecoder;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../💾️binary/📸️snapshot/🧮️decoded-components/🧫️fixtures/🧮️owned-exact-components/🔣️.json")).unwrap();
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
        eprintln!("[DEBUG] authored JPEG component case={} samples={} independent image dimensions={}x{}",case["name"],actual.samples.len(),width,height);
    }
}

#[test]
fn export_quality_is_physical_policy_with_independent_quantization_witness() {
    use semio_framework_value::ToValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../💾️binary/📸️snapshot/🎛️encode-options/🧫️fixtures/🔣️.json")).unwrap();
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
        eprintln!("[DEBUG] JPEG export quality={quality} independent image luma quantizer={expected}; owned raster is unchanged");
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

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG2: per-artifact conformance laws (the recipe's §4 deliverable checklist item 6) —
/// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
/// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
/// bytes, and the fixture-honesty round-trip. Relocated verbatim from `⚙️engine`'s own test
/// region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — mirrors png's own
/// identically-named module exactly (same six laws, same structure, only the demo-case helpers
/// differ per the recipe's own note that every pilot's `conformance_laws` module is
/// near-identical).
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, snapshot};
    use crate::JpgMutation;
    use protocol::{DiffBinary,DiffCodec,DiffText, OpBinary, OpText};

    /// 🧫️ Admits the committed native mutation corpus and covers every declared variant.
    fn demo_mutation_cases() -> Vec<JpgMutation> {
        use semio_s_artifact_stdio_contract::editing;
        let base = demo_jpg_snapshot();
        let event = editing::SnapshotEditEvent::SetValue { path: "/jfifXDensity".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(73)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare committed quality case");
        let mut cases = vec![
        ];
        for text in [
            include_str!("../../../🧫️fixtures/🧬️mutations/🪪️change-jfif/🎯️direct/🦠️mutation/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/📥️insert-other/🎯️direct/🦠️mutation/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🗑️remove-other/🎯️direct/🦠️mutation/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🔲️replace-pixels/🎯️direct/🦠️mutation/🔣️.json"),
        ] {
            cases.push(semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("admit committed mutation case"));
        }
        let actual = cases.iter().map(|case| <JpgMutation as protocol::SemanticMutation<JpgSnapshot>>::semantics(case).kind).collect::<Vec<_>>();
        let declared = <JpgMutation as protocol::SemanticMutation<JpgSnapshot>>::kinds().iter().map(|kind| kind.kind).collect::<Vec<_>>();
        assert_eq!(actual, declared, "native cases cover every declared variant in order");
        cases
    }

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect — independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
    /// message).
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", crate::standards::v_jfif_1_01::subsets::document::io::text::diff::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", crate::standards::v_jfif_1_01::subsets::document::io::binary::diff::COMPONENT_PROTOCOL_SEMIO)] {
            semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar (hex-dump grammar of the TEXT DSL
    /// form — jpg's real internal marker structure is `../💾️binary/📡️.protocol.semio`'s
    /// job, not this leaf's, per the recipe's own png precedent) recognizes real `print_dsl`
    /// output for the demo snapshot — same preamble-stripped body reconstruction
    /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_host_snapshot` uses.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        let text = store::ArtifactDsl::print_dsl(&demo_jpg_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `JpgMutation` variant (`demo_mutation_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `JpgDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
    /// every tri-state/`JpgFrameChange`/collection-triple shape.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_jfif_1_01::subsets::document::io::text::diff::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
    /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
    /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
    /// mutation's `encode_op`, and every demo diff's `encode_diff`.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = semio_framework_dsl::parse_protocol(crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_jpg_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = semio_framework_dsl::parse_protocol(crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = semio_framework_dsl::parse_protocol(crate::standards::v_jfif_1_01::subsets::document::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// 🧪️ Shipped Text/Pack preserve every owned demo field and equal their declared codec output.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_jpg_snapshot();

        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_jpg_snapshot()) drifted from the shipped .dsl.semio fixture");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_jpg_snapshot()) drifted from the shipped .pack.semio fixture");

        let parsed = <JpgSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed,demo);
        let decoded=<JpgSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode owned Pack fixture");
        assert_eq!(decoded,demo);
    }
}
//#endregion 🔖️ConformanceLaws
