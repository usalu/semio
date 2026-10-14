//! 🧪️ Actual child construction matches the portable independent JSON document oracle.
use super::*;
fn normalized(value:&mut serde_json::Value){match value {serde_json::Value::Number(number)=>*number=serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap(),serde_json::Value::Array(values)=>for value in values{normalized(value)},serde_json::Value::Object(values)=>for value in values.values_mut(){normalized(value)},_=>{}}}
#[test]
fn actual_playbook_factory_matches_portable_form_corpus(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🌊️flow/📝️forms/🧬️schema/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let producer=PlaybookFlowFormProducer;
 let fields=corpus["widgets"].as_array().unwrap().iter().filter_map(|widget|{
  let id=widget["id"].as_str().unwrap();
  Some(match widget["kind"].as_str().unwrap(){
   "inputSlider"=>producer.slider(id,widget["label"].as_str().unwrap(),widget["value"].as_f64().unwrap(),widget["min"].as_f64().unwrap(),widget["max"].as_f64().unwrap(),widget["step"].as_f64().unwrap()),
   "inputNote"=>producer.note(id,widget["text"].as_str().unwrap()),
   "inputImage"=>producer.image(id,widget["src"].as_str().unwrap()),
   "variable"=>producer.variable(id,widget["name"].as_str().unwrap(),widget["schema"].as_str().unwrap()),
   _=>return None,
  })
 }).collect();
 let actual=producer.finish(fields);
 let mut independent=serde_json::to_value(&actual).unwrap();let mut expected=corpus["playbook"].clone();normalized(&mut independent);normalized(&mut expected);assert_eq!(independent,expected);
 let mut native:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&actual))).unwrap();normalized(&mut native);assert_eq!(native,independent);
}
