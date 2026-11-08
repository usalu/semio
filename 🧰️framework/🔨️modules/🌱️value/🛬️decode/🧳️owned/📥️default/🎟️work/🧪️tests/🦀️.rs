use crate::*;
use crate::{NativeDefault,NativeDefaultCursor};
use crate::source_fixture::{self as fixture,BankWallet,EffectBank,Wallet};
use crate::native_decoding::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete};
type Op<'a>=&'a mut dyn for<'r> FnMut(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
struct Resident;
impl OriginalEffectProducer for Resident{
 type Output=RetainedCloneStep;type Failure=OriginalReceivingPrimary;type Context<'turn,'source>=Op<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>where 'source:'turn{context(effect,reply)}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
fn synchronous(poll:RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous default action stayed pending")}}
fn synchronous_close(poll:OriginalRetirementPoll)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous default close stayed pending")}}
fn through_reply(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:Op<'_>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
 let mut authority=OriginalEffectAuthority::new(Resident);let mut slot=None;let mut reply=None;let mut operation=operation;
 let(poll,_)=bank.run(grant,maximum,|effect|match authority.drive(&mut operation,effect,&mut slot){Ok(true)=>{slot.as_mut().expect("settled action").transfer_reply(&mut reply).expect("empty reply slot");RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().expect("recorded reply"),work:effect.performed_work()})},Ok(false)=>RetainedWorkPoll::Pending{work:effect.performed_work()},Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:effect.performed_work()})});
 authority.finish(&mut slot).expect("reply transferred");drop(authority.into_producer().ok().expect("finished action"));assert_ne!(bank.take_admitted(),Some(false));synchronous(poll)
}
struct Bank{wallet:Wallet,effects:EffectBank,maximum:u64}
impl Bank{
 fn new()->Self{let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let s=&law["stock"];Self{wallet:Wallet::from_profile(s),effects:EffectBank::new(BankWallet::from_profile(s)),maximum:s["maximumTurns"].as_u64().unwrap()}}
 fn offer(&self)->RetainedCloneGrant{self.wallet.loan(1,usize::MAX)}
 fn fuel(&self)->u64{self.wallet.work_remaining()}
 fn post(&mut self,p:RetainedCloneProgress,work:u128,g:RetainedCloneGrant,heap:(usize,usize)){self.wallet.post(p,work,g,heap)}
 fn retire<T:RetireOwned>(&mut self,value:T){fixture::close_owned(value,&mut self.wallet,1<<16,usize::try_from(self.maximum).unwrap())}
 fn receive(&mut self,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,advance:impl FnOnce(&mut NativeDecodeControl<'_>)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>)->(RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>,(usize,usize)){let fuel=self.fuel();crate::observe_retirement_allocations(||fixture::receive(control,g,self.wallet.work_loan(fuel),|native|synchronous(advance(native))))}
 fn take<T:RetireOwned+'static,C:NativeDefaultCursor<T>>(&mut self,cursor:&mut C)->T{let quote=cursor.take_work_demand().unwrap();let grant=self.wallet.loan(1,cursor.take_demand().unwrap().copy_bytes);let mut value=None;let(turn,heap)=crate::observe_retirement_allocations(||through_reply(&mut self.effects,grant,quote,&mut|effect,reply|cursor.take(&mut value,effect,reply)));let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|error|error.retained_progress());self.wallet.post(p,turn.work,grant,heap);turn.result.unwrap();value.expect("paid default output")}
 fn close<T:RetireOwned,C:NativeDefaultCursor<T>>(&mut self,cursor:&mut C){cursor.begin_close();for _ in 0..self.maximum{if cursor.terminal_is_empty(){break}let quote=cursor.close_work_demand(4096).unwrap();let grant=self.offer();let((poll,actual),heap)=crate::observe_retirement_allocations(||self.effects.run(grant,quote,|effect|cursor.close_step(effect)));assert_ne!(self.effects.take_admitted(),Some(false));let turn=synchronous_close(poll);let p_heap=heap;if let Some(bytes)=actual{assert_eq!(bytes,p_heap.0)}else{assert_eq!(p_heap.0,0)}let p=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());self.wallet.post(p,turn.work,grant,heap);turn.result.unwrap();}assert!(cursor.terminal_is_empty());let(poll,_)=self.effects.run(Default::default(),0,|effect|cursor.close_step(effect));assert_ne!(self.effects.take_admitted(),Some(false));assert_eq!(synchronous_close(poll).work,0);}
}

fn run<T:NativeDefault>(cut:Option<u64>,check:impl FnOnce(&T))->u128{run_with::<T>(cut,false,check)}
fn run_with<T:NativeDefault>(cut:Option<u64>,refuse:bool,check:impl FnOnce(&T))->u128{
 let mut bank=Bank::new();let mut cursor=T::default_cursor();let mut callback=|_|true;let mut control=NativeDecodeControl::new(bank.wallet.remaining().maximum_capacity_bytes,&mut callback);let mut null_seen=false;let mut normal_work=0u128;
 for _ in 0..cut.unwrap_or(bank.maximum){
  if cursor.output_ready(){break}
  let fuel=bank.fuel();let d=cursor.demands(4096,fuel).unwrap();let g=bank.offer();
  for denied in[Some(RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().flatten(){
   let(turn,heap)=bank.receive(&mut control,denied,|native|cursor.advance(native,4096,fuel,denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));
  }
  if refuse&&d.capacity_bytes>0&&!null_seen{
   let(((turn,heap),attempts))=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||bank.receive(&mut control,g,|native|cursor.advance(native,4096,fuel,g)));
   assert_eq!(attempts,1);assert_eq!(heap,(0,0));assert_eq!(turn.work,1);let p=turn.result.as_ref().err().unwrap().retained_progress();bank.post(p,turn.work,g,(0,0));normal_work+=turn.work;bank.retire(turn.result.err().unwrap());null_seen=true;break
  }
  let(turn,heap)=bank.receive(&mut control,g,|native|cursor.advance(native,4096,fuel,g));let p=turn.result.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));bank.post(p,turn.work,g,heap);normal_work+=turn.work;
 }
 if cut.is_none()&&!refuse{assert!(cursor.output_ready());let output=bank.take::<T,_>(&mut cursor);check(&output);bank.retire(output);}
 bank.close::<T,_>(&mut cursor);
 normal_work
}
#[test]fn actual_native_default_array_and_tuple_are_owned_phased_and_match_independent_serde(){let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();run::<[f64;3]>(None,|v|assert_eq!(serde_json::to_value(v).unwrap(),law["smallArray"]));run::<(String,bool)>(None,|v|assert_eq!(serde_json::to_value(v).unwrap(),law["tuple"]));for cut in law["cancelCuts"].as_array().unwrap(){run::<[String;3]>(Some(cut.as_u64().unwrap()),|_|{});}}
#[test]fn refused_array_allocation_leaves_the_default_failed_until_its_paid_close(){let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();run_with::<[String;3]>(None,true,|_|{});run_with::<[f64;3]>(None,true,|_|{});assert_eq!(law["smallArray"].as_array().unwrap().len(),3);}
#[test]fn actual_native_default_large_zero_sized_array_cannot_collapse_into_one_cpu_turn(){let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let work=run::<[();4097]>(None,|v|{assert_eq!(v.len(),law["largeZeroSizedCount"].as_u64().unwrap()as usize);assert_eq!(serde_json::to_value(v.iter().collect::<Vec<_>>()).unwrap(),serde_json::json!(vec![serde_json::Value::Null;4097]));});assert!(work>=u128::from(law["minimumLargeWork"].as_u64().unwrap()));}
#[test]fn actual_native_empty_array_default_returns_the_real_empty_value(){let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();run::<[u8;0]>(None,|value|assert_eq!(serde_json::to_value(value).unwrap(),law["emptyArray"]));}

#[test]fn actual_default_leaf_payload_writes_differ_from_empty_owner_headers(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut bank=Bank::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(bank.wallet.remaining().maximum_capacity_bytes,&mut callback);
 macro_rules! check{($ty:ty,$kind:literal,$expected:expr)=>{{
  let mut cursor=<$ty as NativeDefault>::default_cursor();let fuel=bank.fuel();let d=cursor.demands(4096,fuel).unwrap();assert_eq!(d.copy_bytes,std::mem::size_of::<Option<$ty>>());
  if d.copy_bytes>0{let denied=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..bank.offer()};let(turn,heap)=bank.receive(&mut control,denied,|native|cursor.advance(native,4096,fuel,denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert!(!cursor.output_ready());}
  let g=bank.offer();let(turn,heap)=bank.receive(&mut control,g,|native|cursor.advance(native,4096,fuel,g));assert_eq!(turn.work,1);let p=turn.result.unwrap().progress();assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!(heap,(0,0));bank.post(p,turn.work,g,heap);
  let value=bank.take::<$ty,_>(&mut cursor);assert_eq!(value,$expected);assert_eq!(serde_json::to_value(&value).unwrap(),law["freshDefaultValues"][$kind]);bank.retire(value);cursor.begin_close();assert!(cursor.terminal_is_empty());
 }};}
 check!(bool,"bool",false);check!(u64,"u64",0);check!(f64,"f64",0.0);check!(String,"string",String::new());check!((),"unit",());
}

#[test]fn zero_work_default_does_not_start_original_effect(){
 let f:serde_json::Value=serde_json::from_str(include_str!("🧫️admission/🔣️.json")).unwrap();
 for row in f["cases"].as_array().unwrap().iter().filter(|r|r["kind"]=="boolean"&&r["budget"]==0){
  let bank=Bank::new();let mut cursor=<bool as NativeDefault>::default_cursor();let mut callback=|_|true;let mut control=NativeDecodeControl::new(bank.wallet.remaining().maximum_capacity_bytes,&mut callback);
  let budget=row["budget"].as_u64().unwrap();let g=bank.offer();let(turn,heap)=crate::observe_retirement_allocations(||synchronous(cursor.advance(&mut control,4096,budget,g)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert!(!cursor.output_ready());
  cursor.begin_close();assert!(cursor.terminal_is_empty());
 }
}

#[test]fn default_effect_honors_both_original_grants(){let f:serde_json::Value=serde_json::from_str(include_str!("🧫️grants/🔣️.json")).unwrap();let parse=|row:&serde_json::Value|{let a=row.as_array().unwrap();RetainedCloneGrant{maximum_items:a[0].as_u64().unwrap() as usize,maximum_copy_bytes:a[1].as_u64().unwrap() as usize,maximum_capacity_bytes:a[2].as_u64().unwrap() as usize,maximum_release_bytes:a[3].as_u64().unwrap() as usize,maximum_depth:a[4].as_u64().unwrap() as usize}};for row in f["cases"].as_array().unwrap(){let actual=parse(&row["caller"]).restricted_by(parse(&row["recipient"]));assert_eq!(actual,parse(&row["expected"]));}}

#[test]fn missing_original_recipient_leaves_default_retryable(){
 let f:serde_json::Value=serde_json::from_str(include_str!("🧫️outcome/🔣️.json")).unwrap();assert_eq!(f["cases"].as_array().unwrap().iter().find(|c|c["outcome"]=="pre_effect_refusal").unwrap()["retryable"],true);
 let mut bank=Bank::new();let mut cursor=<bool as NativeDefault>::default_cursor();let mut callback=|_|true;let mut control=NativeDecodeControl::new(bank.wallet.remaining().maximum_capacity_bytes,&mut callback);
 let g=bank.offer();let turn=synchronous(cursor.advance(&mut control,4096,1,g));assert_eq!(turn.work,0);let error=turn.result.unwrap_err();assert!(!cursor.output_ready());assert_eq!(cursor.demands(4096,1).unwrap(),<bool as NativeDefaultValue>::default_demand());assert_eq!(<bool as NativeDefaultValue>::default_work_demand().unwrap(),1);bank.retire(error);
 let fuel=bank.fuel();let(turn,heap)=bank.receive(&mut control,g,|native|cursor.advance(native,4096,fuel,g));let p=turn.result.unwrap().progress();bank.post(p,turn.work,g,heap);assert!(cursor.output_ready());
 let value=bank.take::<bool,_>(&mut cursor);assert!(!value);bank.retire(value);bank.close::<bool,_>(&mut cursor);
}

struct Overrun(bool);
impl RetireOwned for Overrun{type RetirementCursor=<bool as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{self.0.retirement_seed()}fn controlled_retirement_supported()->bool{true}}
impl NativeDefaultValue for Overrun{fn default_demand()->RetirementDemand{<bool as NativeDefaultValue>::default_demand()}fn default_work_demand()->Result<u64,ValueError>{Ok(1)}fn default_allocates()->bool{false}fn admit_default(output:&mut Option<Self>,_:RetainedCloneGrant,_:Option<&mut Option<usize>>)->RetainedWorkOutcome{*output=Some(Overrun(false));RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:2}}}
impl NativeDefault for Overrun{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}
/// 🛟️ A refused original work settlement is a failure outcome: the placed output stays unpublished until its paid close.
#[test]fn refused_default_work_settlement_keeps_output_unready_for_paid_close(){
 let f:serde_json::Value=serde_json::from_str(include_str!("🧫️outcome/🔣️.json")).unwrap();assert_eq!(f["cases"].as_array().unwrap().iter().find(|c|c["outcome"]=="failure").unwrap()["ready"],false);
 let mut bank=Bank::new();let mut cursor=<Overrun as NativeDefault>::default_cursor();let mut callback=|_|true;let mut control=NativeDecodeControl::new(bank.wallet.remaining().maximum_capacity_bytes,&mut callback);
 let fuel=bank.fuel();let g=bank.offer();let(turn,_)=bank.receive(&mut control,g,|native|cursor.advance(native,4096,fuel,g));
 assert_eq!(turn.work,2);assert!(turn.result.is_err());assert!(!cursor.output_ready());assert_eq!(cursor.demands(4096,fuel).unwrap(),Default::default());
 bank.close::<Overrun,_>(&mut cursor);
}
