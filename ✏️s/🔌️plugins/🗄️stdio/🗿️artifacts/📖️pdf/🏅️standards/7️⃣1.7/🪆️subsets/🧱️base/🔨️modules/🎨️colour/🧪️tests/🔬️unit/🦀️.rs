use protocol::command::DiffAlgebra;
use super::*;

#[test]
fn sampled_function_neutral_words_preserve_native_twelve_bit_extent() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["sampled12"];
    let words: Vec<u32> = serde_json::from_value(case["sampleWords"].clone()).unwrap();
    let bytes: Vec<u8> = serde_json::from_value(case["nativeBytes"].clone()).unwrap();
    assert_eq!(pack_sample_words(&words, 12).unwrap(), bytes);
    assert_eq!(unpack_sample_words(&bytes, 12, words.len()).unwrap(), words);
    let physical = bytes.clone();
    let physical_words = vec![(u32::from(physical[0]) << 4) | (u32::from(physical[1]) >> 4), ((u32::from(physical[1]) & 15) << 8) | u32::from(physical[2])];
    assert_eq!(physical_words, words);
    assert!(pack_sample_words(&[4096], 12).is_err());
    assert!(unpack_sample_words(&bytes[..2], 12, 2).is_err());
    assert_eq!(unpack_sample_words(&pack_sample_words(&[u32::MAX], 32).unwrap(), 32, 1).unwrap(), vec![u32::MAX]);
    for case in fixture["sampleWidths"].as_array().unwrap() {
        let bits=case["bitsPerSample"].as_u64().unwrap() as u32;
        let words:Vec<u32>=serde_json::from_value(case["sampleWords"].clone()).unwrap();
        let bytes:Vec<u8>=serde_json::from_value(case["nativeBytes"].clone()).unwrap();
        assert_eq!(pack_sample_words(&words,bits).unwrap(),bytes);
        assert_eq!(unpack_sample_words(&bytes,bits,words.len()).unwrap(),words);
    }
    eprintln!("[DEBUG] PDF sampled function native words=513,4095 width=12 neutral_native_extent=3 all_widths=8");
}

#[test]
fn sampled_function_typed_diff_inverse_and_native_document_preserve_words() {
    use crate::standards::v1_7::subsets::base::{io, schema::{mutations::{PdfMutation,set_shading::SetShading}, snapshot::*}};
    use protocol::{DiffCodec,Mutation};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["sampled12"];
    let words: Vec<u32> = serde_json::from_value(case["sampleWords"].clone()).unwrap();
    let function = PdfFunction::Sampled { domain: vec![0.0,1.0], range: vec![0.0,1.0], size: vec![2], bits_per_sample: 12, order: None, encode: None, decode: None, samples: words.clone() };
    let shading = PdfShading { id: "SampledWord".into(), color_space: PdfColorSpace::DeviceGray, kind: PdfShadingKind::Axial { coords:[0.0,0.0,1.0,1.0], domain:None, function, extend:[false,false] }, background:None, bbox:None, anti_alias:false, extra:Vec::new() };
    let mut base = PdfSnapshot::default();
    base.pages.push(PdfPage::new(10.0,10.0));
    let intent = PdfMutation::SetShading(SetShading {shading:shading.clone(),index:None});
    let outcome = intent.diff(&base);
    let delta = outcome.diff();
    let target = protocol::apply_diff(delta,&base).unwrap();
    assert_eq!(target.shadings,vec![shading]);
    assert_eq!(protocol::apply_diff(&delta.inverse(&base),&target).unwrap(),base);
    let mut restored=target.clone();
    for inverse in intent.inverse(&base).unwrap(){restored=protocol::apply_diff(inverse.diff(&restored).diff(),&restored).unwrap();}
    assert_eq!(restored,base);
    let native = io::encode_pdf(&target).unwrap();
    let streams=semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_sampled_function_streams(&native).unwrap();
    let samples=streams.into_iter().find(|(bits,_)|*bits==12).unwrap().1;
    assert_eq!(samples,serde_json::from_value::<Vec<u8>>(case["nativeBytes"].clone()).unwrap());
    let admitted=io::decode_pdf(&native).unwrap();
    let PdfShadingKind::Axial { function:PdfFunction::Sampled { samples,.. },.. }=&admitted.shadings[0].kind else {panic!("sampled shading role must remain typed")};
    assert_eq!(samples,&words);
    eprintln!("[DEBUG] PDF typed sampled shading diff/inverse and independent lopdf native admission words=513,4095");
}
