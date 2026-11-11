use super::*;
use semio_framework_job::OriginalRetirementPump;
fn debit(left:&mut RetainedCloneGrant,p:RetainedCloneProgress){assert!(p.fits(*left));left.maximum_items-=p.copied_items;left.maximum_copy_bytes-=p.copied_bytes;left.maximum_capacity_bytes-=p.retained_capacity_bytes;left.maximum_release_bytes-=p.released_bytes;}
struct Bank{normal:RetainedCloneGrant,cleanup:RetainedCloneGrant,fuel:u64,close_fuel:u64,turn:RetainedCloneGrant,work:u64}
fn grant(row:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:row["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize}}
impl Bank{
 fn new(law:&serde_json::Value)->Self{Self{normal:grant(&law["stock"]),cleanup:grant(&law["cleanup"]),fuel:law["stock"]["fuel"].as_u64().unwrap(),close_fuel:law["cleanup"]["fuel"].as_u64().unwrap(),turn:grant(&law["turn"]),work:law["turn"]["work"].as_u64().unwrap()}}
 fn current(&self,closing:bool)->RetainedCloneGrant{let left=if closing{self.cleanup}else{self.normal};RetainedCloneGrant{maximum_items:left.maximum_items.min(self.turn.maximum_items),maximum_copy_bytes:left.maximum_copy_bytes.min(self.turn.maximum_copy_bytes),maximum_capacity_bytes:left.maximum_capacity_bytes.min(self.turn.maximum_capacity_bytes),maximum_release_bytes:left.maximum_release_bytes.min(self.turn.maximum_release_bytes),maximum_depth:left.maximum_depth.min(self.turn.maximum_depth)}}
 fn pay(&mut self,p:RetainedCloneProgress,work:u64,closing:bool){if closing{debit(&mut self.cleanup,p);self.close_fuel=self.close_fuel.checked_sub(work).unwrap()}else{debit(&mut self.normal,p);self.fuel=self.fuel.checked_sub(work).unwrap()}}
 fn run(&mut self,native:&mut NativeDecodeControl<'_>,cursor:&mut ProbeMutationReceiving,source:&[u8],g:RetainedCloneGrant,allocating:bool)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let demand=cursor.demands(source,g.maximum_copy_bytes).unwrap_or_default();let work=self.work;let body=g.maximum_copy_bytes;
  let mut turn=OriginalRetirementPump::new().turn(self.fuel.min(work),u64::MAX,||Some(0),g).unwrap();
  let ran=turn.with_native_decoder_work(native,|native|{native.admit_turn_capacity(g.maximum_capacity_bytes)?;if allocating{native.with_allocated_effect(demand,work,|native,effect|cursor.step_in(source,native,body,effect))}else{native.with_effect(g,work,|native,effect|cursor.step_in(source,native,body,effect))}}).unwrap();
  let out=ran.expect("probe step quote was not admitted");let p=out.result.as_ref().copied().unwrap_or_else(|e|e.retained_progress());self.pay(p,out.work,false);out
 }
 fn step(&mut self,native:&mut NativeDecodeControl<'_>,cursor:&mut ProbeMutationReceiving,source:&[u8])->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let g=self.current(false);self.run(native,cursor,source,g,true)}
 fn close<T:RetireOwned>(&mut self,slot:&mut OwnedSlot<T>){for _ in 0..100000{if slot.terminal_is_empty(){return}let g=self.current(true);let mut turn=OriginalRetirementPump::new().turn(self.close_fuel.min(self.work),u64::MAX,||Some(0),g).unwrap();let(closed,_)=turn.with_effect(|effect|slot.close_step(effect)).unwrap().expect("probe close quote was not admitted");let p=closed.result.unwrap();self.pay(p,closed.work,true)}panic!("declared cleanup stock did not close its actual owner")}
}
#[test]
fn actual_probe_cursor_matches_json_oracle_and_keeps_each_original_domain_failure(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../input.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let mut bank=Bank::new(&law);let mut source=vec![0xff];let start=source.len();source.extend_from_slice(row["source"].as_str().unwrap().as_bytes());let end=source.len();source.push(0xfe);
  let denied=ProbeMutation::admit_receiving(&source,(start,end),RetainedCloneGrant{maximum_items:0,..bank.current(false)});assert!(denied.result.unwrap().is_none());assert_eq!(denied.work,0);
  let born=ProbeMutation::admit_receiving(&source,(start,end),bank.current(false));let(cursor,p)=born.result.unwrap().unwrap();bank.pay(p,born.work,false);let mut cursor_slot=OwnedSlot::default();*cursor_slot.value=Some(cursor);let mut primary=OwnedSlot::default();let mut domain=OwnedSlot::default();let mut output=OwnedSlot::default();
  let mut accept=|_|true;let mut native=NativeDecodeControl::new(bank.normal.maximum_capacity_bytes,&mut accept);
  let foreign=source.clone();assert!(cursor_slot.value.as_ref().unwrap().demands(&foreign,65536).is_err());
  let idle=bank.run(&mut native,cursor_slot.value.as_mut().unwrap(),&source,RetainedCloneGrant{maximum_items:0,..bank.current(false)},false);assert_eq!(idle.work,0);assert_eq!(idle.result.unwrap(),Default::default());
  for _ in 0..100000{if cursor_slot.value.as_ref().unwrap().is_ready()||primary.value.is_some(){break}let turn=bank.step(&mut native,cursor_slot.value.as_mut().unwrap(),&source);if let Err(error)=turn.result{*primary.value=Some(error)}}
  let accepted=cursor_slot.value.as_ref().unwrap().is_ready();assert_eq!(accepted,row["accepted"].as_bool().unwrap(),"{:?}",row["name"]);
  if accepted{
   let turn=cursor_slot.value.as_mut().unwrap().take_into(&mut output.value,bank.current(false));bank.pay(turn.result.unwrap(),turn.work,false);
   let ProbeMutation::SetValue(value)=output.value.as_ref().unwrap();let actual:serde_json::Value=serde_json::from_str(&crate::probe_json::print(value).unwrap()).unwrap();assert_eq!(actual,row["expected"]);
  }else{
   let cursor=cursor_slot.value.as_mut().unwrap();assert!(cursor.domain_failure().is_some());assert!(cursor.close_demands(65536).is_err());
   let pointer=match cursor.domain_failure().unwrap(){JsonError::DuplicateMember{name,..}=>Some(name.as_ptr()),_=>None};
   let denied=cursor.take_domain_failure_into(&mut domain.value,RetainedCloneGrant{maximum_copy_bytes:0,..bank.current(false)});assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap(),Default::default());assert!(domain.value.is_none()&&cursor.domain_failure().is_some());
   let turn=cursor.take_domain_failure_into(&mut domain.value,bank.current(true));bank.pay(turn.result.unwrap(),turn.work,true);assert!(cursor.domain_failure().is_none());
   if let Some(pointer)=pointer{let JsonError::DuplicateMember{name,offset}=domain.value.as_ref().unwrap()else{panic!("original syntax variant was changed")};assert_eq!(name,"a");assert_eq!(name.as_ptr(),pointer);assert!(*offset>0)}
  }
  bank.close(&mut cursor_slot);bank.close(&mut output);bank.close(&mut primary);bank.close(&mut domain);
 }
}
