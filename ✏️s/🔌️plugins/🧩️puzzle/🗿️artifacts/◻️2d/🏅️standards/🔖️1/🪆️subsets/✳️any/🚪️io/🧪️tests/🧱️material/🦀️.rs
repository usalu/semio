//! 🧱️ Independent authored material arithmetic and declared word roles are shared by native and actor acceptance.
use serde_json::Value;
use semio_framework_test_fixtures::repo_text;
use std::collections::BTreeSet;

pub fn seed()->Value{
 serde_json::from_str(repo_text!("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()
}
pub fn declared(mut document:Value,word:Option<u64>,repeat_nodes:bool)->Value{
 let law=seed();let mut paths=BTreeSet::new();
 for path in law["declaredJson"]["floatPointers"].as_array().unwrap(){
  let path=path.as_str().unwrap();paths.insert(path.to_owned());
  if repeat_nodes{if let Some(suffix)=path.strip_prefix("/nodes/0/"){for index in 0..document["nodes"].as_array().unwrap().len(){paths.insert(format!("/nodes/{index}/{suffix}"));}}}
 }
 for path in paths{
  if let Some(value)=document.pointer_mut(&path){
   let bits=word.unwrap_or_else(||value.as_f64().expect("authored neutral numeric role").to_bits());
   *value=serde_json::json!({"bits":format!("{bits:016x}")});
  }
 }
 document
}
pub fn material(recipe:&Value)->(Value,Value){
 let mut document=seed()["snapshot"].clone();let template=document["nodes"][0].clone();
 let count=recipe["nodes"].as_u64().unwrap()as usize;let columns=recipe["columns"].as_u64().unwrap()as usize;
 let spacing=recipe["spacing"].as_f64().unwrap();let prefix=recipe["idPrefix"].as_str().unwrap();
 document["nodes"]=Value::Array((0..count).map(|index|{
  let mut node=template.clone();node["id"]=Value::String(format!("{prefix}{index:04}"));
  node["x"]=serde_json::json!((index%columns)as f64*spacing);node["y"]=serde_json::json!((index/columns)as f64*spacing);node
 }).collect());
 document["edges"]=serde_json::json!([]);document["targetRegions"]=serde_json::json!([]);
 let mut wire=declared(document.clone(),None,true);wire.as_object_mut().unwrap().remove("targetRegions");(document,wire)
}
