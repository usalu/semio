use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfFunction, PdfShading, PdfShadingKind};
use semio_framework_test_fixtures::fixture_text;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_f64, dict_get, dict_i64, dict_name};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::xref::ObjectSink;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{NativePdfArtifactResources, PdfArtifactResourcePort};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::stream_roles::PdfStreamRoleValue;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{stream_bytes, PdfDictEntry, PdfExtGState, PdfLineCap, PdfLineJoin, PdfMatrix, PdfObject, PdfRect, PdfSoftMask};
use protocol::command::{ColdDiffAlgebra, DiffAlgebra};
use crate::standards::v1_7::subsets::base::modules::colour::*;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::schema::snapshot::model::PdfPage;

#[test]
fn sampled_function_neutral_words_preserve_native_sample_extent() {
    use crate::standards::v1_7::subsets::base::{schema::graph_source::GraphSource, schema::snapshot::ObjRef};

    struct FunctionObject(Option<PdfObject>);
    impl ObjectSink for FunctionObject {
        fn add(&mut self, value: PdfObject) -> ObjRef {
            self.0 = Some(value);
            ObjRef { num: 1, gen: 0 }
        }
    }

    let lower = |bits, samples: &[u32]| {
        let function = PdfFunction::Sampled {
            domain: vec![0.0, 1.0], range: vec![0.0, 1.0], size: vec![samples.len() as u32],
            bits_per_sample: bits, order: None, encode: None, decode: None, samples: samples.to_vec(),
        };
        let mut sink = FunctionObject(None);
        lower_function(&function, &mut sink)?;
        Ok::<_, PdfEngineError>((function, sink.0.unwrap()))
    };
    let read_case = |case: &serde_json::Value, bits| {
        let words: Vec<u32> = serde_json::from_value(case["sampleWords"].clone()).unwrap();
        let bytes: Vec<u8> = serde_json::from_value(case["nativeBytes"].clone()).unwrap();
        let (function, object) = lower(bits, &words).unwrap();
        let PdfObject::Stream { data, .. } = &object else { panic!("expected sampled stream") };
        assert_eq!(data.to_vec_owner(), bytes);
        assert_eq!(lift_function(&object, &mut GraphSource::new(&[])), Some(function));
        (words, bytes, object)
    };

    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let (words, bytes, mut short) = read_case(&fixture["sampled12"], 12);
    let physical_words = [
        (u32::from(bytes[0]) << 4) | (u32::from(bytes[1]) >> 4),
        ((u32::from(bytes[1]) & 15) << 8) | u32::from(bytes[2]),
    ];
    assert_eq!(words, physical_words);
    assert!(lower(12, &[4096]).is_err());
    let PdfObject::Stream { data, .. } = &mut short else { panic!("expected sampled stream") };
    *data = stream_bytes(&data.to_vec_owner()[..2]);
    assert!(lift_function(&short, &mut GraphSource::new(&[])).is_none());
    let (max, object) = lower(32, &[u32::MAX]).unwrap();
    assert_eq!(lift_function(&object, &mut GraphSource::new(&[])), Some(max));
    for case in fixture["sampleWidths"].as_array().unwrap() {
        read_case(case, case["bitsPerSample"].as_u64().unwrap() as u32);
    }
}

#[test]
fn sampled_function_typed_diff_inverse_and_native_document_preserve_words() {
    use crate::standards::v1_7::subsets::base::io;
    use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
    use crate::standards::v1_7::subsets::base::schema::mutations::set_shading::SetShading;
    use crate::standards::v1_7::subsets::base::schema::snapshot::*;
    use protocol::{DiffCodec,ColdMutation, Mutation};
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["sampled12"];
    let words: Vec<u32> = serde_json::from_value(case["sampleWords"].clone()).unwrap();
    let function = PdfFunction::Sampled { domain: vec![0.0,1.0], range: vec![0.0,1.0], size: vec![2], bits_per_sample: 12, order: None, encode: None, decode: None, samples: words.clone() };
    let shading = PdfShading { id: "SampledWord".into(), color_space: PdfColorSpace::DeviceGray, kind: PdfShadingKind::Axial { coords:[0.0,0.0,1.0,1.0], domain:None, function, extend:[false,false] }, background:None, bbox:None, anti_alias:false, extra:Vec::new() };
    let mut base = PdfSnapshot::default();
    base.pages.push(PdfPage::new(10.0,10.0));
    let intent = PdfMutation::SetShading(SetShading {shading:shading.clone(),index:None});
    let outcome = intent.diff(&base);
    let delta = outcome.diff();
    let target = protocol::apply_diff_cold(delta,&base).unwrap();
    assert_eq!(target.shadings,vec![shading]);
    assert_eq!(protocol::apply_diff_cold(&delta.inverse(&base),&target).unwrap(),base);
    let mut restored=target.clone();
    for inverse in intent.inverse(&base).unwrap(){restored=protocol::apply_diff_cold(inverse.diff(&restored).diff(),&restored).unwrap();}
    assert_eq!(restored,base);
    let native = io::export::encode_pdf(&target).unwrap();
    let streams=semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_sampled_function_streams(&native).unwrap();
    let samples=streams.into_iter().find(|(bits,_)|*bits==12).unwrap().1;
    assert_eq!(samples,serde_json::from_value::<Vec<u8>>(case["nativeBytes"].clone()).unwrap());
    let admitted=io::import::decode_pdf(&native).unwrap();
    let PdfShadingKind::Axial { function:PdfFunction::Sampled { samples,.. },.. }=&admitted.shadings[0].kind else {panic!("sampled shading role must remain typed")};
    assert_eq!(samples,&words);
}
