use super::*;
use protocol::{command::DiffAlgebra, DiffCodec, MutationDiff};

#[test]
fn canonical_diff_roundtrips_and_restores_exact_authority() {
    let before = demo_snapshot_a();
    let mut after = demo_snapshot_b();
    after.schema = "s.stdio.pptx.ecma-376.base.v2".into();
    let diff = PptxDiff::between(&before, &after);
    assert_eq!(diff.apply(&before).unwrap(), after);
    assert_eq!(diff.inverse(&before).apply(&after).unwrap(), before);
    assert_eq!(PptxDiff::parse_diff(&diff.print_diff()).unwrap(), diff);
    assert_eq!(PptxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(), diff);
}

#[test]
fn canonical_diff_absorption_keeps_latest_field_owner() {
    let before = demo_snapshot_a();
    let middle = demo_snapshot_b();
    let mut after = middle.clone();
    after.schema = "latest".into();
    let mut combined = PptxDiff::between(&before, &middle);
    combined.absorb(PptxDiff::between(&middle, &after));
    assert_eq!(combined.apply(&before).unwrap(), after);
}

fn canonical_diff_fixture()->serde_json::Value{
 serde_json::from_str(include_str!("../../../../🧫️fixtures/🧬️canonical-diff/🔣️.json")).expect("language-neutral canonical field replacement laws")
}

fn selected_diff(fields:&serde_json::Value,target:&PptxSnapshot)->PptxDiff{
 let fields=fields.as_array().expect("three authored selectors");assert_eq!(fields.len(),3);
 PptxDiff{schema:fields[0].as_bool().expect("schema selector").then(||target.schema.clone()),opc:fields[1].as_bool().expect("OPC selector").then(||target.opc.clone()),xml_parts:fields[2].as_bool().expect("XML selector").then(||target.xml_parts.clone())}
}

fn independent_xml(snapshot:&PptxSnapshot){
 for part in &snapshot.xml_parts{
  let text=snapshot.part_text(&part.path).expect("canonical XML part");let mut reader=quick_xml::reader::Reader::from_str(&text);
  loop{if matches!(reader.read_event().expect("QuickXML independently recognizes selected full XML owner"),quick_xml::events::Event::Eof){break;}}
 }
}

#[test]
fn canonical_sparse_diff_all_eight_owner_selections_roundtrip_and_invert(){
 let fixture=canonical_diff_fixture();assert_eq!(fixture["fields"],serde_json::json!(["schema","opc","xmlParts"]));
 let before=demo_snapshot_a();let mut after=demo_snapshot_b();after.schema=fixture["firstSchema"].as_str().unwrap().into();
 after.opc.set_part("media.bin","application/octet-stream",fixture["firstMedia"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect());
 let combinations=fixture["combinations"].as_array().unwrap();assert_eq!(combinations.len(),8);
 for selectors in combinations{
  let diff=selected_diff(selectors,&after);let actual=diff.apply(&before).expect("selected field replacement");let mut expected=before.clone();
  if selectors[0].as_bool().unwrap(){expected.schema=after.schema.clone();}
  if selectors[1].as_bool().unwrap(){expected.opc=after.opc.clone();}
  if selectors[2].as_bool().unwrap(){expected.xml_parts=after.xml_parts.clone();}
  assert_eq!(actual,expected);assert_eq!(diff.is_empty(),selectors.as_array().unwrap().iter().all(|v|v.as_bool()==Some(false)));
  assert_eq!(diff.inverse(&before).apply(&actual).expect("exact full owner inverse"),before);
  assert_eq!(PptxDiff::parse_diff(&diff.print_diff()).expect("typed sparse text roundtrip"),diff);
  assert_eq!(PptxDiff::decode_diff(&diff.encode_diff().expect("typed sparse pack")).expect("typed sparse pack roundtrip"),diff);
  independent_xml(&actual);assert_eq!(before,demo_snapshot_a());
 }
}

#[test]
fn canonical_sparse_diff_all_owner_compositions_keep_latest_present_fields(){
 let fixture=canonical_diff_fixture();let before=demo_snapshot_a();let mut first=demo_snapshot_b();first.schema=fixture["firstSchema"].as_str().unwrap().into();
 first.opc.set_part("media.bin","application/octet-stream",fixture["firstMedia"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect());
 let mut second=before.clone();second.schema=fixture["secondSchema"].as_str().unwrap().into();
 second.opc.set_part("media.bin","application/octet-stream",fixture["secondMedia"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect());
 for left in fixture["combinations"].as_array().unwrap(){for right in fixture["combinations"].as_array().unwrap(){
  let mut combined=selected_diff(left,&first);let latest=selected_diff(right,&second);let middle=combined.apply(&before).unwrap();let expected=latest.apply(&middle).unwrap();
  combined.absorb(latest);let actual=combined.apply(&before).unwrap();assert_eq!(actual,expected);assert_eq!(combined.inverse(&before).apply(&actual).unwrap(),before);
 }}
}

#[test]
fn canonical_diff_codecs_refuse_untyped_xml_owners(){
 let fixture=canonical_diff_fixture();let value=semio_framework_value::DslValue::Object(vec![(fixture["malformedField"].as_str().unwrap().into(),semio_framework_value::DslValue::String(fixture["malformedValue"].as_str().unwrap().into()))]);
 let bytes=store::pack_rt::encode_wire_value(&value);assert!(PptxDiff::decode_diff(&bytes).is_err());
 let record=PptxDiffRecord{value};let text=semio_framework_dsl_record::print(&record.__dsl_to_record(),&PptxDiffRecord::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline);
 assert!(PptxDiff::parse_diff(&text).is_err());
}
