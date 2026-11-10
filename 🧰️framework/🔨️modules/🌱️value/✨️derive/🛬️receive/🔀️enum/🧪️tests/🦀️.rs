//! 🧪️ Real tagged FEM payloads and tuple variants preserve source leases through every native frontier.
#[path="../../../../♻️retirement/🧪️tests/⏱️work/🦀️.rs"]mod close_work_policy;
use semio_framework_test_fixtures::fixture_text;
use crate::{DslValue,native_decoding::{NativeReceive,NativeReceiver},retirement::RetireOwned,shared_issuance::SealedShared};
#[derive(crate::RetireOwned,crate::NativeReceive,crate::ToValue)]
#[value(crate="crate",rename_all="camelCase")]
struct Node{id:SealedShared<String>,x:f64,y:f64}
#[derive(crate::RetireOwned,crate::NativeReceive,crate::ToValue)]
#[value(crate="crate",tag="kind",rename_all="camelCase",deny_unknown_fields)]
enum Element{
 #[value(rename_all="camelCase")]
 Bar{id:SealedShared<String>,start:SealedShared<String>,end:SealedShared<String>,material_id:SealedShared<String>,section_id:SealedShared<String>},
 #[value(rename_all="camelCase")]
 Frame{id:SealedShared<String>,start:SealedShared<String>,end:SealedShared<String>,material_id:SealedShared<String>,section_id:SealedShared<String>,roll:f64},
}
#[derive(crate::RetireOwned,crate::NativeReceive,crate::ToValue)]
#[value(crate="crate",tag="mutation",rename_all="camelCase",deny_unknown_fields)]
enum Mutation{CreateNode(Node)}
#[derive(crate::RetireOwned,crate::NativeReceive)]
#[value(crate="crate",rename_all="camelCase",deny_unknown_fields)]
enum External{Pair(SealedShared<String>,f64),Node(Node)}
#[derive(crate::RetireOwned,crate::NativeReceive)]
#[value(crate="crate",tag="kind",content="payload",rename_all="camelCase",deny_unknown_fields)]
enum Adjacent{Pair(SealedShared<String>,f64)}
#[derive(crate::RetireOwned,crate::NativeReceive,serde::Serialize,serde::Deserialize)]
#[value(crate="crate")]
#[serde(transparent)]
struct ScalarHeader(u64);
#[derive(crate::RetireOwned,crate::NativeReceive,serde::Serialize,serde::Deserialize)]
#[value(crate="crate",transparent)]
#[serde(transparent)]
struct NamedScalarHeader{value:u64}
trait Observed{fn observed(&self)->serde_json::Value;}
impl Observed for Node{fn observed(&self)->serde_json::Value{serde_json::json!({"id":self.id.as_str(),"x":self.x,"y":self.y})}}
impl Observed for Element{fn observed(&self)->serde_json::Value{match self{Self::Bar{id,start,end,material_id,section_id}=>serde_json::json!({"kind":"bar","id":id.as_str(),"start":start.as_str(),"end":end.as_str(),"materialId":material_id.as_str(),"sectionId":section_id.as_str()}),Self::Frame{id,start,end,material_id,section_id,roll}=>serde_json::json!({"kind":"frame","id":id.as_str(),"start":start.as_str(),"end":end.as_str(),"materialId":material_id.as_str(),"sectionId":section_id.as_str(),"roll":roll})}}}
impl Observed for Mutation{fn observed(&self)->serde_json::Value{match self{Self::CreateNode(node)=>{let mut value=node.observed();value.as_object_mut().unwrap().insert("mutation".into(),serde_json::json!("createNode"));value}}}}
impl Observed for External{fn observed(&self)->serde_json::Value{match self{Self::Pair(text,n)=>serde_json::json!({"pair":[text.as_str(),n]}),Self::Node(node)=>serde_json::json!({"node":node.observed()})}}}
impl Observed for Adjacent{fn observed(&self)->serde_json::Value{match self{Self::Pair(text,n)=>serde_json::json!({"kind":"pair","payload":[text.as_str(),n]})}}}
impl Observed for ScalarHeader{fn observed(&self)->serde_json::Value{serde_json::json!(self.0)}}
impl Observed for NamedScalarHeader{fn observed(&self)->serde_json::Value{serde_json::json!(self.value)}}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleNode{id:String,x:f64,y:f64}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(tag="kind",rename_all="camelCase",rename_all_fields="camelCase",deny_unknown_fields)]
enum OracleElement{Bar{id:String,start:String,end:String,material_id:String,section_id:String},Frame{id:String,start:String,end:String,material_id:String,section_id:String,roll:f64}}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(tag="mutation",rename_all="camelCase",deny_unknown_fields)]
enum OracleMutation{CreateNode(OracleNode)}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
enum OracleExternal{Pair(String,f64),Node(OracleNode)}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(tag="kind",content="payload",rename_all="camelCase",deny_unknown_fields)]
enum OracleAdjacent{Pair(String,f64)}
fn oracle<T:serde::de::DeserializeOwned+serde::Serialize>(row:&serde_json::Value){match serde_json::from_value::<T>(row["input"].clone()){Ok(value)=>{assert!(row["accepted"].as_bool().unwrap());assert_eq!(serde_json::to_value(value).unwrap(),row["input"]);},Err(_)=>assert!(!row["accepted"].as_bool().unwrap())}}
use crate::driver;
fn observe<R>(operation:impl FnOnce()->R)->(R,(usize,usize)){crate::observe_retirement_allocations(operation)}
fn run<T:NativeReceive<DslValue>+Observed+RetireOwned>(row:&serde_json::Value,cuts:&[serde_json::Value])where <T as NativeReceive<DslValue>>::Receiver:NativeReceiver<DslValue,T>{
 let accepted=row["accepted"].as_bool().unwrap();let grant:serde_json::Value=serde_json::from_str::<serde_json::Value>(fixture_text!("../../../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["originalGrant"].clone();
 for cut in cuts{
  let cut=cut.as_u64().unwrap();let mut bank=driver::bank();let original=driver::admit(&mut bank,DslValue::from(&row["input"]));let foreign=driver::admit(&mut bank,DslValue::from(&row["input"]));let identity=std::ptr::from_ref(original.borrow().get());let mut receiver=T::receiver();let mut refused=false;let mut ready=false;let mut history=Vec::new();
  for turn in 0..32768u64{
   if turn==cut{break}if receiver.output_ready(){ready=true;break}
   match receiver.demands(original.borrow(),4096,1<<20){Ok(d)=>{assert!(d.copy_bytes as u64<=grant["copy"].as_u64().unwrap()&&d.capacity_bytes as u64<=grant["capacity"].as_u64().unwrap(),"{} turn {turn} quotes {d:?} after {:?}",row["name"],history);history.push(d);if history.len()>6{history.remove(0);}},Err(_)=>{assert!(!accepted);refused=true;break}}
   if turn>0&&row["family"]!="transparent"{assert!(receiver.demands(foreign.borrow(),4096,1<<20).is_err())}
   if driver::receive_cut(&mut receiver,original.borrow(),&mut bank,4096,Some(1)).is_err(){assert!(!accepted);refused=true;break}
   assert_eq!(std::ptr::from_ref(original.borrow().get()),identity);
  }
  ready|=receiver.output_ready();
  if ready{assert!(accepted);let output=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,4096,None).unwrap().expect("uncut receive publishes");assert_eq!(output.observed(),row["input"]);driver::retire(output,&mut bank);}
  driver::close(&mut receiver,&mut bank,4096);assert!(receiver.terminal_is_empty());assert_eq!(observe(||drop(receiver)).1,(0,0));assert_eq!(serde_json::Value::from(original.borrow().get()),row["input"]);driver::close_source(original,&mut bank);driver::close_source(foreign,&mut bank);
  if Some(&serde_json::json!(cut))==cuts.last(){assert!(ready||refused)}
 }
}
#[test]
fn native_payload_enum_receives_exact_declared_variants_and_closes_every_original_frontier(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();assert_eq!(f["selectorSettlement"],"matcher-only-until-paid-close");for row in f["cases"].as_array().unwrap(){let cuts=f["cuts"].as_array().unwrap();match row["family"].as_str().unwrap(){"element"=>{oracle::<OracleElement>(row);run::<Element>(row,cuts)},"mutation"=>{oracle::<OracleMutation>(row);run::<Mutation>(row,cuts)},"external"=>{oracle::<OracleExternal>(row);run::<External>(row,cuts)},"adjacent"=>{oracle::<OracleAdjacent>(row);run::<Adjacent>(row,cuts)},_=>panic!("declared enum family")}}}

#[derive(crate::RetireOwned,crate::NativeReceive)]
#[value(crate="crate",rename_all="camelCase")]
enum UnitAndStruct{Owner,Contributed{id:SealedShared<String>}}
#[test]
fn unit_and_struct_variants_derive_a_closable_receiver(){
 let mut receiver=<UnitAndStruct as NativeReceive<DslValue>>::receiver();
 assert!(!receiver.output_ready());
 let mut bank=crate::driver::bank();crate::driver::close(&mut receiver,&mut bank,4096);assert!(receiver.terminal_is_empty());
}

#[test]
fn transparent_native_wrappers_preserve_the_scalar_wire_and_owned_frontiers(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🧫️fixtures/transparent.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  oracle::<ScalarHeader>(row);oracle::<NamedScalarHeader>(row);
  run::<ScalarHeader>(row,law["cuts"].as_array().unwrap());
  run::<NamedScalarHeader>(row,law["cuts"].as_array().unwrap());
 }
}
