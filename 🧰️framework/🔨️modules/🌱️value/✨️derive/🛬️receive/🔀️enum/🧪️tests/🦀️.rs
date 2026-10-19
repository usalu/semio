//! 🧪️ Real tagged FEM payloads and tuple variants preserve source leases through every native frontier.
use semio_framework_test_fixtures::fixture_text;
use crate::{NativeDecodeControl,DslValue,RetirementDemand,ValueError,retained_clone::{RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress},native_decoding::{NativeReceive,NativeReceiver},retirement::{RetireOwned,controlled::ControlledRetirement,shared::sealed::SealedShared}};
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
trait Observed{fn observed(&self)->serde_json::Value;}
impl Observed for Node{fn observed(&self)->serde_json::Value{serde_json::json!({"id":self.id.as_str(),"x":self.x,"y":self.y})}}
impl Observed for Element{fn observed(&self)->serde_json::Value{match self{Self::Bar{id,start,end,material_id,section_id}=>serde_json::json!({"kind":"bar","id":id.as_str(),"start":start.as_str(),"end":end.as_str(),"materialId":material_id.as_str(),"sectionId":section_id.as_str()}),Self::Frame{id,start,end,material_id,section_id,roll}=>serde_json::json!({"kind":"frame","id":id.as_str(),"start":start.as_str(),"end":end.as_str(),"materialId":material_id.as_str(),"sectionId":section_id.as_str(),"roll":roll})}}}
impl Observed for Mutation{fn observed(&self)->serde_json::Value{match self{Self::CreateNode(node)=>{let mut value=node.observed();value.as_object_mut().unwrap().insert("mutation".into(),serde_json::json!("createNode"));value}}}}
impl Observed for External{fn observed(&self)->serde_json::Value{match self{Self::Pair(text,n)=>serde_json::json!({"pair":[text.as_str(),n]}),Self::Node(node)=>serde_json::json!({"node":node.observed()})}}}
impl Observed for Adjacent{fn observed(&self)->serde_json::Value{match self{Self::Pair(text,n)=>serde_json::json!({"kind":"pair","payload":[text.as_str(),n]})}}}
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
fn grant(d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096);assert!(d.capacity_bytes<=65536);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn observe<R>(operation:impl FnOnce()->R)->(R,(usize,usize)){crate::observe_retirement_allocations(operation)}
fn drain<T:RetireOwned>(value:T){let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("enum original typed issuer"));while !close.terminal_is_empty(){let g=grant(close.next_demand(4096).unwrap());let(s,h)=observe(||close.step(g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));}}
fn source(value:DslValue)->RetainedCloneSource<DslValue>{let d=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();RetainedCloneSource::admit_owned(value,(),grant(RetirementDemand{copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})).unwrap_or_else(|_|panic!("enum original source capture")).0}
fn run<T:NativeReceive<DslValue>+Observed>(row:&serde_json::Value,cuts:&[serde_json::Value]){
 for cut in cuts{let original=source(DslValue::from(&row["input"]));let foreign=source(DslValue::from(&row["input"]));let identity=std::ptr::from_ref(original.borrow().get());let(mut receiver,_)=T::begin_receive(grant(T::receive_birth_demand())).unwrap();let mut callback=|_|true;let mut control=NativeDecodeControl::new(16777216,&mut callback);let mut refused=false;let mut ready=false;
  for turn in 0..2048{if turn==cut.as_u64().unwrap()as usize{break;}if receiver.output_ready(){ready=true;break;}let d=receiver.demands(original.borrow(),4096).unwrap();let g=grant(d);if turn>0{assert!(receiver.demands(foreign.borrow(),4096).is_err());}for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}let(s,h)=observe(||super::list_tests::receive(&mut control,denied,|control|receiver.advance(original.borrow(),control,4096,denied)).unwrap());assert_eq!(s.progress(),RetainedCloneProgress::default());assert_eq!(h,(0,0));}let(result,h)=observe(||super::list_tests::receive(&mut control,g,|control|receiver.advance(original.borrow(),control,4096,g)));match result{Ok(step)=>assert_eq!(h,(step.progress().retained_capacity_bytes,step.progress().released_bytes)),Err(error)=>{assert_eq!(h,(error.retained_progress().retained_capacity_bytes,error.retained_progress().released_bytes));assert!(!row["accepted"].as_bool().unwrap());refused=true;break;}}assert_eq!(std::ptr::from_ref(original.borrow().get()),identity);}
  ready|=receiver.output_ready();if ready{assert!(row["accepted"].as_bool().unwrap());let g=grant(receiver.take_demand().unwrap());let(output,p)=receiver.take(g).unwrap().unwrap();assert!(p.fits(g));assert_eq!(output.observed(),row["input"]);drain(output);}receiver.begin_close();while !receiver.terminal_is_empty(){let g=grant(receiver.close_demands(4096).unwrap());let(s,h)=observe(||receiver.close_step(g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));}assert_eq!(observe(||drop(receiver)).1,(0,0));assert_eq!(serde_json::Value::from(original.borrow().get()),row["input"]);drain(original);drain(foreign);if cut.as_u64()==Some(1024){assert!(ready||refused);}
 }
}
#[test]
fn native_payload_enum_receives_exact_declared_variants_and_closes_every_original_frontier(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in f["cases"].as_array().unwrap(){let cuts=f["cuts"].as_array().unwrap();match row["family"].as_str().unwrap(){"element"=>{oracle::<OracleElement>(row);run::<Element>(row,cuts)},"mutation"=>{oracle::<OracleMutation>(row);run::<Mutation>(row,cuts)},"external"=>{oracle::<OracleExternal>(row);run::<External>(row,cuts)},"adjacent"=>{oracle::<OracleAdjacent>(row);run::<Adjacent>(row,cuts)},_=>panic!("declared enum family")}}eprintln!("[DEBUG] payload enum exact Serde shape/variant/arity validation, original source identity, funded typed child custody and12 cancellation frontiers");}
