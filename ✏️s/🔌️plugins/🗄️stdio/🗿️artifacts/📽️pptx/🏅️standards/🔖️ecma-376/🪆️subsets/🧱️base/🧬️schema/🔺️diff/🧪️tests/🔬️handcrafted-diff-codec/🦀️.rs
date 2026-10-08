use crate::standards::v_ecma_376::subsets::base::io::text::diff::PptxDiffRecord;
use super::*;
use protocol::{command::DiffAlgebra, DiffBinary,DiffCodec,DiffText, MutationDiff};

fn canonical_diff_fixture()->serde_json::Value{
 serde_json::from_str(include_str!("../../../../🧫️fixtures/🧬️canonical-diff/🔣️.json")).expect("language-neutral canonical field replacement laws")
}

#[test]
fn diff_codecs_refuse_untyped_xml_owners(){
 let fixture=canonical_diff_fixture();let value=semio_framework_value::DslValue::Object(vec![(fixture["malformedField"].as_str().unwrap().into(),semio_framework_value::DslValue::String(fixture["malformedValue"].as_str().unwrap().into()))]);
 let bytes=store::pack_rt::encode_wire_value(&value);assert!(PptxDiff::decode_diff(&bytes).is_err());
 let record=PptxDiffRecord{value};let text=semio_framework_dsl_record::print(&record.__dsl_to_record(),&PptxDiffRecord::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline);
 assert!(PptxDiff::parse_diff(&text).is_err());
}

#[test]
fn canonical_diff_cases_roundtrip_text_and_binary() {
    for diff in demo_diff_cases() {
        assert_eq!(PptxDiff::parse_diff(&diff.print_diff()).unwrap(), diff);
        assert_eq!(PptxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(), diff);
    }
}
