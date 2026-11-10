use super::*;
use semio_framework_value_native_owned::DslValue;
const BODY:usize=4096;
#[test]
fn native_receiving_retained_projection_preserves_parent_and_exact_cancellation_receipts(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){
  let array=case["wrap"]=="array";let index=usize::from(array);let mut finished=None;
  let make=||{let original=DslValue::Object(vec![("value".to_owned(),DslValue::String(case["source"]["value"].as_str().unwrap().to_owned()))]);if array{DslValue::Array(vec![DslValue::Null,original])}else{original}};
  for cut in 0..case["cutBound"].as_u64().unwrap_or(512){
   let mut bank=driver::bank();let source=driver::admit(&mut bank,make());
   let pointer=std::ptr::from_ref(source.borrow().get());
   let child_pointer=match source.borrow().get(){DslValue::Array(rows)=>std::ptr::from_ref(&rows[1]),value=>std::ptr::from_ref(value)};
   if array{assert_ne!(pointer,child_pointer)}
   let child=||source.borrow().project(index,|value:&DslValue|match value{DslValue::Array(rows)=>&rows[1],value=>value});
   assert_eq!(std::ptr::from_ref(child().get()),child_pointer);
   let(mut receiver,heap)=observe_retirement_allocations(<Row as NativeReceive<DslValue>>::receiver);assert_eq!(heap,(0,0));
   driver::none(driver::receive_cut(&mut receiver,child(),&mut bank,BODY,Some(cut))).unwrap();
   let ready=receiver.output_ready();
   driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);
   assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);driver::close_source(source,&mut bank);
   if ready{finished=Some(cut);break}
  }
  assert!(finished.is_some_and(|cut|cut>0));
  let mut bank=driver::bank();let source=driver::admit(&mut bank,make());
  let child=source.borrow().project(index,|value:&DslValue|match value{DslValue::Array(rows)=>&rows[1],value=>value});
  let mut receiver=<Row as NativeReceive<DslValue>>::receiver();
  let row=driver::receive_cut(&mut receiver,child,&mut bank,BODY,None).unwrap().expect("uncut receive publishes");
  assert_eq!(row.value.as_str(),case["expected"].as_str().unwrap());
  driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::retire(row,&mut bank);driver::close_source(source,&mut bank);
 }
}
