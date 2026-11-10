use super::*;
use crate::{DslValue,retained_clone::{RetainedClone,RetainedCloneCursor}};
use semio_framework_value_testing_admission::{EffectBank,Wallet,session_driven as driven,session_open};
use crate::driver;
const BODY:usize=32768;
fn law()->serde_json::Value{serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🗂️map/📐️.json")).unwrap()}
fn bank(law:&serde_json::Value)->EffectBank{EffectBank::new(Wallet::from_profile(&law["stock"]))}
fn heap<T>(f:impl FnOnce()->T)->(T,(usize,usize)){crate::observe_retirement_allocations(f)}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn denials(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{[Some(RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().flatten().collect()}
#[test]fn original_map_receiving_matches_fixture_and_closes_every_partial_owner(){
 let law=law();
 for case in law["cases"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap())).chain(std::iter::once(None)){
  let mut bank=bank(&law);let make=||DslValue::Object(case["entries"].as_array().unwrap().iter().map(|e|(e[0].as_str().unwrap().into(),DslValue::String(e[1].as_str().unwrap().into()))).collect());
  let s=driver::admit(&mut bank,make());let foreign=driver::admit(&mut bank,make());
  let(mut r,h)=heap(<OrderedMap<String> as NativeReceive<DslValue>>::receiver);assert_eq!(h,(0,0));
  let received=driver::receive_cut(&mut r,s.borrow(),&mut bank,BODY,cut).unwrap();
  if !r.output_ready()&&cut.is_some_and(|cut|cut>0){assert!(r.demands(foreign.borrow(),BODY,1<<20).is_err())}
  match(cut,received){(None,Some(map))=>{let entries:Vec<_>=map.iter().map(|(k,v)|serde_json::json!([k,v])).collect();assert_eq!(serde_json::Value::Array(entries),case["expected"]);driver::retire(map,&mut bank)},(Some(_),None)=>{},(cut,received)=>panic!("cut {cut:?} published {}",received.is_some())}
  driver::close(&mut r,&mut bank,BODY);assert!(r.terminal_is_empty());driver::close_source(s,&mut bank);driver::close_source(foreign,&mut bank);
 }}
}
#[test]fn ordered_map_original_alias_clone_keeps_its_root_and_closes_all_denied_prefixes(){
 let alias:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🗂️ordered/🧬️alias.json")).unwrap();let law=law();
 for cut in alias["cancelCuts"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()).chain(std::iter::once(100)){
  let mut bank=bank(&law);session_open(&bank);
  let map:OrderedMap<String>=alias["entries"].as_array().unwrap().iter().map(|v|(v[0].as_str().unwrap().to_owned(),v[1].as_str().unwrap().to_owned())).collect();let identity=map.original_root_identity();
  let s=driver::admit(&mut bank,map);let mut cursor=OrderedMap::<String>::retained_clone_cursor();
  for _ in 0..cut{
   if cursor.output_ready(){break}
   let fuel=bank.wallet().remaining_work();let d=cursor.advance_demands(s.borrow(),4096,fuel).unwrap();let w=cursor.advance_work_demand(s.borrow(),4096,fuel).unwrap();assert!(d.copy_bytes<=4096);let g=grant(d);
   for denied in denials(g,d){let((poll,_),h)=heap(||driven::<OriginalRetirementFailure>(&mut bank,denied,w.max(1),|effect,reply,recover|if recover{cursor.recover_advance_into(s.borrow(),4096,fuel,effect,reply)}else{cursor.advance_into(s.borrow(),4096,fuel,effect,reply)}));assert_eq!(h,(0,0));let RetainedWorkPoll::Complete(t)=poll else{panic!("denied turn parked")};assert_eq!(t.work,0);assert_eq!(t.result.unwrap().progress(),Default::default());assert_eq!(cursor.advance_demands(s.borrow(),4096,fuel).unwrap(),d)}
   let((poll,_),h)=heap(||driven::<OriginalRetirementFailure>(&mut bank,g,w.max(1),|effect,reply,recover|if recover{cursor.recover_advance_into(s.borrow(),4096,fuel,effect,reply)}else{cursor.advance_into(s.borrow(),4096,fuel,effect,reply)}));
   let RetainedWorkPoll::Complete(t)=poll else{panic!("funded turn parked")};let p=t.result.unwrap().progress();assert_eq!(h,(p.retained_capacity_bytes,p.released_bytes));assert!(p.retained_capacity_bytes==0||h.0==alias["aliasCapacityBytes"].as_u64().unwrap()as usize);bank.post(p,g,h);
  }
  if cut==100{
   assert!(cursor.output_ready());let g=grant(cursor.take_demand().unwrap());let w=cursor.take_work_demand().unwrap();let mut output=None;
   let((poll,_),h)=heap(||driven::<ValueError>(&mut bank,g,w.max(1),|effect,reply,recover|if recover{cursor.recover_take(&mut output,effect,reply)}else{cursor.take(&mut output,effect,reply)}));
   let RetainedWorkPoll::Complete(t)=poll else{panic!("take parked")};bank.post(t.result.unwrap().progress(),g,h);let value=output.take().unwrap();
   assert_eq!(value.original_root_identity(),identity);assert!(alias["sharedRoot"].as_bool().unwrap());driver::retire(value,&mut bank);
  }
  while !cursor.close_started(){let d=cursor.begin_close_demand().unwrap();let w=cursor.begin_close_work_demand().unwrap();let g=grant(d);let((poll,_),h)=heap(||driven::<OriginalRetirementFailure>(&mut bank,g,w.max(1),|effect,reply,recover|if recover{cursor.recover_begin_close_into(effect,reply)}else{cursor.begin_close_into(effect,reply)}));let RetainedWorkPoll::Complete(t)=poll else{panic!("close start parked")};bank.post(t.result.unwrap().progress(),g,h)}
  for _ in 0..100000{
   if cursor.terminal_is_empty(){break}
   let d=cursor.close_demands(4096).unwrap();let w=cursor.close_work_demand(4096).unwrap();assert!(d.copy_bytes<=4096);let g=grant(d);
   for denied in denials(g,d){let((poll,_),h)=heap(||driven::<OriginalRetirementFailure>(&mut bank,denied,w.max(1),|effect,reply,recover|if recover{cursor.recover_close_into(4096,effect,reply)}else{cursor.close_into(4096,effect,reply)}));assert_eq!(h,(0,0));let RetainedWorkPoll::Complete(t)=poll else{panic!("denied close parked")};assert_eq!(t.work,0);assert_eq!(t.result.unwrap().progress(),Default::default());assert_eq!(cursor.close_demands(4096).unwrap(),d)}
   let((poll,_),h)=heap(||driven::<OriginalRetirementFailure>(&mut bank,g,w.max(1),|effect,reply,recover|if recover{cursor.recover_close_into(4096,effect,reply)}else{cursor.close_into(4096,effect,reply)}));let RetainedWorkPoll::Complete(t)=poll else{panic!("close parked")};bank.post(t.result.unwrap().progress(),g,h);
  }
  assert!(cursor.terminal_is_empty());driver::close_source(s,&mut bank);
 }
}
struct OriginalMapKey{name:String,ordinal:Option<u16>}
impl NativeValueSource for OriginalMapKey{
 fn query_scalar<'r>(&self,query:crate::owned::tree::NativeScalarQuery,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'r,crate::owned::tree::NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'r,crate::owned::tree::NativeSourceScalar,ValueError>,ValueError>{
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"test Source query exceeded its work authority"))))}
  let _=query;Ok(reply.complete(Err(crate::owned::tree::invalid())))
 }
 fn recover_scalar<'r>(&self,_:crate::owned::tree::NativeScalarQuery,_:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'r,crate::owned::tree::NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'r,crate::owned::tree::NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<crate::owned::tree::NativeTreeKind>{None}
 fn record_key(&self,index:usize)->Result<crate::owned::NativeRecordKey<'_>,ValueError>{if index!=0{return Err(crate::owned::tree::invalid())}Ok(match self.ordinal{Some(ordinal)=>crate::owned::NativeRecordKey::Ordinal(ordinal),None=>crate::owned::NativeRecordKey::Name(&self.name)})}
}
#[test]fn dynamic_map_keys_require_an_actual_borrowed_name(){let law=law();for case in law["keyCases"].as_array().unwrap(){let mut bank=bank(&law);let input=OriginalMapKey{name:case["key"]["name"].as_str().unwrap_or("").into(),ordinal:case["key"]["ordinal"].as_u64().map(|v|v as u16)};let mut receiver=<OrderedMap<String>as NativeReceive<OriginalMapKey>>::receiver();assert_eq!(receiver.fragment_probe(&input).is_ok(),case["accepted"].as_bool().unwrap());driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());}}
