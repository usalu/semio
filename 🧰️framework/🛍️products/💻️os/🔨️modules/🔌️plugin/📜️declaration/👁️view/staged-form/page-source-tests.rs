//! 🧪️ Borrowed page bindings preserve every neutral decimal word without allocating DTO copies.
use super::*;
use semio_framework::action_staging::*;
struct Borrowed<'a>{token:&'a StagedFormToken,page:&'a StagedFormPage}
impl StagedFormPageBindingSource for Borrowed<'_>{
 fn token(&self)->&StagedFormToken{self.token}
 fn field_offset(&self)->&str{&self.page.field_offset}
 fn diagnostic_offset(&self)->&str{&self.page.diagnostic_offset}
 fn choice(&self)->Option<StagedChoicePageView<'_>>{self.page.choice.as_ref().map(|choice|StagedChoicePageView{path:&choice.path,offset:&choice.offset})}
}
fn value(source:&impl StagedPayloadSource,path:&mut Vec<usize>)->serde_json::Value{
 let(node,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.node(path));
 assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 match node.unwrap().value{
  StagedPayloadKind::Null=>serde_json::Value::Null,StagedPayloadKind::Bool(value)=>value.into(),StagedPayloadKind::Number(value)=>serde_json::json!(value),StagedPayloadKind::Text(value)=>value.into(),
  StagedPayloadKind::List(len)=>serde_json::Value::Array((0..len).map(|index|{path.push(index);let output=value(source,path);path.pop();output}).collect()),
  StagedPayloadKind::Map(len)=>serde_json::Value::Object((0..len).map(|index|{path.push(index);let key=source.node(path).unwrap().key.unwrap().to_string();let output=value(source,path);path.pop();(key,output)}).collect()),
 }
}
#[test]
fn borrowed_page_binding_matches_all_neutral_valid_pages_and_original_token(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/page-cases.json")).unwrap();
 let token:StagedFormToken=serde_json::from_value(law["token"].clone()).unwrap();
 for expected in law["valid"].as_array().unwrap(){
  let page:StagedFormPage=serde_json::from_value(expected.clone()).unwrap();let borrowed=Borrowed{token:&token,page:&page};
  let(valid,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||validate_staged_page_binding_source(&borrowed));
  valid.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  assert_eq!(value(&StagedBindingPayload::PageSource(&borrowed),&mut Vec::new()),serde_json::json!({"token":token,"page":expected}));
 }
}
