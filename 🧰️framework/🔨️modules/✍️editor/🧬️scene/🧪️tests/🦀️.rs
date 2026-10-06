//! 🧪️ Canonical typed scene inputs preserve exact values under caller-declared framing.
use semio_framework_editor::scene::{decode, from_json};
use semio_framework_editor::EditorHost;
use semio_framework_value::{DslValue, NativeDecodeControl, ToValue};
use pack::intrinsic::IntrinsicFormat;
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
use serde_json::Value;
fn corpus()->Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn encoded(value:DslValue,format:IntrinsicFormat)->Vec<u8>{
 let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);
 let record=RecordValue{fields:[(1,FieldValue::Value(value))].into_iter().collect()};
 match format{IntrinsicFormat::Body=>pack::record::encode_record_body(&spec,&record,&Default::default()).unwrap(),IntrinsicFormat::Document=>pack::record::encode_document(&spec,&record,&Default::default()).unwrap()}
}
#[test]
fn canonical_scene_corpus_matches_the_independent_json_values_and_both_declared_formats(){
 let fixture=corpus();
 for row in fixture["cases"].as_array().unwrap(){
  for format in [IntrinsicFormat::Body,IntrinsicFormat::Document]{
   let bytes=encoded(DslValue::from(&row["input"]),format);let mut calls=0;let mut progress=|_|{calls+=1;true};
   let mut control=NativeDecodeControl::new(16_777_216,&mut progress);
   let actual=decode(&bytes,format,&Default::default(),&mut control);
   assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap(),"{} {format:?}",row["id"]);
   if let Ok(scene)=actual{assert_eq!(Value::from(scene.to_value()),row["expected"],"{} {format:?}",row["id"]);}
   drop(control);assert!(calls>0);
  }
 }
 eprintln!("[DEBUG] canonical Editor fourteen shared scene schedules agree with serde_json across Body and Document");
}
#[test]
fn canonical_scene_refuses_mismatched_framing_trailing_input_and_caller_revocation(){
 let value=DslValue::object([("buffer".into(),DslValue::String("bounded".repeat(1024)))]);
 for format in [IntrinsicFormat::Body,IntrinsicFormat::Document]{
  let bytes=encoded(value.clone(),format);let other=match format{IntrinsicFormat::Body=>IntrinsicFormat::Document,IntrinsicFormat::Document=>IntrinsicFormat::Body};
  let mut accepted=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accepted);
  assert!(decode(&bytes,other,&Default::default(),&mut control).is_err());
  let mut canceled=|_|false;let mut control=NativeDecodeControl::new(16_777_216,&mut canceled);
  assert!(decode(&bytes,format,&Default::default(),&mut control).is_err());
  let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1,&mut accepted);
  assert!(decode(&bytes,format,&Default::default(),&mut control).is_err());assert!(control.owned_bytes()<=1);
 }
 let mut trailing=encoded(value,IntrinsicFormat::Body);trailing.push(0);
 let mut accepted=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accepted);
 assert!(decode(&trailing,IntrinsicFormat::Body,&Default::default(),&mut control).is_err());
}
#[test]
fn canonical_scene_commit_applies_all_original_adornments_and_explicit_hover_clear(){
 let fixture=corpus();let input=&fixture["cases"][3]["input"];
 let mut accepted=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accepted);
 let scene=from_json(&input.to_string(),&mut control).unwrap();let mut host=EditorHost::new();host.synchronize_scene(scene);
 assert_eq!(host.hover_token_range(),None);assert_eq!(host.text(),"abc");assert_eq!(host.anchor(),0);assert_eq!(host.caret(),2);
 let scene=from_json(r#"{"hover":{"kind":"range","start":1,"end":2}}"#,&mut control).unwrap();host.synchronize_scene(scene);assert_eq!(host.hover_token_range(),Some((1,2)));
 let scene=from_json(r#"{"hover":{"kind":"clear"}}"#,&mut control).unwrap();host.synchronize_scene(scene);
 assert_eq!(host.hover_token_range(),None);assert_eq!(host.text(),"abc");
}

#[derive(serde::Serialize,serde::Deserialize)]
#[serde(default,rename_all="camelCase",deny_unknown_fields)]
struct SettingsOracle {font_px:f64,line_height:f64,show_line_numbers:bool,tab_size:usize}
impl Default for SettingsOracle {
 fn default()->Self {
  let corpus:Value=serde_json::from_str(include_str!("../🧫️fixtures/⚙️settings/🔣️.json")).unwrap();let defaults=&corpus["defaults"];
  Self {font_px:defaults["fontPx"].as_f64().unwrap(),line_height:defaults["lineHeight"].as_f64().unwrap(),show_line_numbers:defaults["showLineNumbers"].as_bool().unwrap(),tab_size:defaults["tabSize"].as_u64().unwrap() as usize}
 }
}

#[test]
fn canonical_scene_settings_defaults_match_the_independent_serde_oracle_and_each_cancellation_boundary() {
 let corpus:Value=serde_json::from_str(include_str!("../🧫️fixtures/⚙️settings/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap() {
  let oracle=serde_json::from_value::<SettingsOracle>(row["input"].clone());assert_eq!(oracle.is_ok(),row["accepted"].as_bool().unwrap(),"{} serde",row["id"]);
  if let Ok(oracle)=oracle {assert_eq!(serde_json::to_value(oracle).unwrap(),row["expected"],"{} fixture oracle",row["id"]);}
  let input=serde_json::json!({"settings":row["input"]});
  for format in [IntrinsicFormat::Body,IntrinsicFormat::Document] {
   let bytes=encoded(DslValue::from(&input),format);let mut accept=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accept);
   let actual=decode(&bytes,format,&Default::default(),&mut control);assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap(),"{} {format:?}",row["id"]);
   if let Ok(scene)=actual {assert_eq!(Value::from(scene.to_value()),serde_json::json!({"settings":row["expected"]}),"{} {format:?}",row["id"]);}
  }
  let mut calls=0;let mut accept=|_|{calls+=1;true};let mut control=NativeDecodeControl::new(16_777_216,&mut accept);
  let actual=from_json(&input.to_string(),&mut control);assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap(),"{} JSON",row["id"]);drop(control);
  if row["accepted"].as_bool().unwrap() {for stop in 1..=calls {let mut observed=0;let mut cancel=|_|{observed+=1;observed!=stop};let mut control=NativeDecodeControl::new(16_777_216,&mut cancel);assert!(from_json(&input.to_string(),&mut control).is_err(),"{} cancellation {stop}",row["id"]);}}
 }
 eprintln!("[DEBUG] Editor eight controlled-default schedules agree with serde_json across JSON, Body and Document and every JSON cancellation boundary");
}
