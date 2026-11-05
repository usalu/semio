//! 🧪️ Real tagged FEM payloads and tuple variants preserve source leases through every native frontier.
#[path="../../../../♻️retirement/🧪️tests/⏱️work/🦀️.rs"]mod close_work_policy;
use semio_framework_test_fixtures::fixture_text;
use crate::{NativeDecodeControl,DslValue,RetirementDemand,ValueError,retained_clone::{RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress},native_decoding::{NativeReceive,NativeReceiver},retirement::{RetireOwned,controlled::ControlledRetirement},shared_issuance::SealedShared};
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
use super::fields::{Bank,source};
use crate::source_fixture::receive;
fn observe<R>(operation:impl FnOnce()->R)->(R,(usize,usize)){crate::observe_retirement_allocations(operation)}
fn grant(bank:&Bank)->RetainedCloneGrant{let mut g=bank.grant();g.maximum_copy_bytes=g.maximum_copy_bytes.min(4096);g.maximum_capacity_bytes=g.maximum_capacity_bytes.min(65536);g}
fn drain<T:RetireOwned>(value:T,bank:&mut Bank){let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("enum typed issuer"));let mut work=close_work_policy::Work::new();while !close.terminal_is_empty(){let d=close.next_demand(4096).unwrap();assert!(d.copy_bytes<=4096,"selected close quote exceeds its authored body");let g=grant(bank);let(result,heap)=observe(||close.step(g));let actual=close.last_turn_work();let p=result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(g));bank.step(crate::RetainedWorkOutcome{result,work:actual}).unwrap();work.observe(p==Default::default()&&!close.terminal_is_empty()&&close.next_demand(4096).unwrap()==d);}}
fn run<T:NativeReceive<DslValue>+Observed>(row:&serde_json::Value,cuts:&[serde_json::Value]){
 for cut in cuts{let mut bank=Bank::new();let original=source(DslValue::from(&row["input"]),&mut bank);let foreign=source(DslValue::from(&row["input"]),&mut bank);let identity=std::ptr::from_ref(original.borrow().get());let mut receiver=T::receiver();let mut callback=|_|true;let mut control=NativeDecodeControl::new(16777216,&mut callback);let mut refused=false;let mut ready=false;
  for turn in 0..2048{if turn==cut.as_u64().unwrap()as usize{break}if receiver.output_ready(){ready=true;break}let d=match receiver.demands(original.borrow(),4096){Ok(d)=>d,Err(_)=>{assert!(!row["accepted"].as_bool().unwrap());refused=true;break}};assert!(d.copy_bytes<=4096&&d.capacity_bytes<=65536);let g=grant(&bank);if turn>0&&row["family"]!="transparent"{assert!(receiver.demands(foreign.borrow(),4096).is_err())}for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}let(result,heap)=observe(||receive(&mut control,denied,bank.work_loan(1),|control|receiver.advance(original.borrow(),control,4096,denied)));assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}let(result,heap)=observe(||receive(&mut control,g,bank.work_loan(1),|control|receiver.advance(original.borrow(),control,4096,g)));let p=result.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));match bank.normal(result){Ok(_)=>{},Err(_)=>{assert!(!row["accepted"].as_bool().unwrap());refused=true;break}}assert_eq!(std::ptr::from_ref(original.borrow().get()),identity);}
  ready|=receiver.output_ready();if ready{assert!(row["accepted"].as_bool().unwrap());let g=grant(&bank);let result=receiver.take(g);let p=result.result.as_ref().unwrap().as_ref().map(|(_,p)|*p).unwrap_or_default();bank.post(p,result.work);let(output,_)=result.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(output.observed(),row["input"]);drain(output,&mut bank);}receiver.begin_close();let mut work=close_work_policy::Work::new();while !receiver.terminal_is_empty(){let d=receiver.close_demands(4096).unwrap();assert!(d.copy_bytes<=4096,"selected close quote exceeds its authored body");let g=grant(&bank);let(result,heap)=observe(||receiver.close_step(g));let p=result.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(g));bank.step(result).unwrap();work.observe(p==Default::default()&&!receiver.terminal_is_empty()&&receiver.close_demands(4096).unwrap()==d);}assert_eq!(observe(||drop(receiver)).1,(0,0));assert_eq!(serde_json::Value::from(original.borrow().get()),row["input"]);drain(original,&mut bank);drain(foreign,&mut bank);if cut.as_u64()==Some(1024){assert!(ready||refused)}
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
 receiver.begin_close();
 assert!(receiver.close_demands(4096).is_ok());
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
