//! 🧰️ Shared native editing tests retain actual Source effects and paid reply handoffs.
use crate::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{OriginalRetirementFailure,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_testing_effect_bank::EffectBank;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use std::{marker::PhantomData,mem::size_of};
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
pub(crate) fn law()->serde_json::Value{serde_json::from_str(include_str!("edit-native.json")).unwrap()}
pub(crate) fn grant(d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=65536,"native edit exceeds the original physical policy: {d:?}");RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub(crate) fn denials(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut out=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes!=0{out.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g})}if d.capacity_bytes!=0{out.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g})}if d.release_bytes!=0{out.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g})}if d.depth!=0{out.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g})}out}
pub(crate) fn result_observed<'a,'b,P>(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<P>,context:&mut P::Context<'a,'b>,g:RetainedCloneGrant,units:u64)->OutcomeObserved where 'b:'a,P:OriginalEffectProducer<Output=RetainedCloneStep,Failure=OriginalRetirementFailure>{
 let native_before=*bank.wallet().original_work();let(mut action,mut output)=(None,None);let mut heap=(0usize,0usize);let mut complete=false;
 for _ in 0..128{let((poll,actual),observed)=observe(||bank.run(g,units,|effect|{let before=effect.performed_boundary();let ready=authority.drive(context,effect,&mut action).unwrap();let work=effect.performed_since(before).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));heap.0+=observed.0;heap.1+=observed.1;if let Some(actual)=actual{assert_eq!(actual,heap.0)}if matches!(poll,RetainedWorkPoll::Pending{..}){continue}let progress=match action.as_ref().unwrap().recorded_reply().unwrap(){Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert!(progress.fits(g));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,g,heap);complete=true;break}
 assert!(complete,"actual native edit action exceeded original finite bound");let native_work=native_before-*bank.wallet().original_work();let native_heap=heap;let g=grant(OriginalEffectAuthority::<P>::transfer_reply_demand().unwrap());let((poll,actual),heap)=observe(||bank.run(g,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(context,&mut action,&mut output,effect))));let RetainedWorkPoll::Complete(turn)=poll else{panic!("native transfer pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap().unwrap(),g,heap);
 let mut result=None;let d=RetirementDemand{copy_bytes:2*size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>(),depth:1,..Default::default()};let g=grant(d);let((poll,actual),heap)=observe(||bank.run(g,1,|effect|{assert!(effect.performed(1).unwrap());result=output.take();RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:1})}));let RetainedWorkPoll::Complete(turn)=poll else{panic!("native reply retirement pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap(),g,heap);OutcomeObserved{result:result.unwrap(),work:native_work,heap:native_heap}
}
pub(crate) trait NativeEdit:RetireOwned{
 type Map:RetireOwned;type Policy:Copy+Default;
 fn policy(&self,body:usize)->Self::Policy;
 fn demand(&self,body:usize,p:Self::Policy)->RetirementDemand;
 fn work(&self,body:usize,p:Self::Policy)->u64;
 fn ready(&self)->bool;
 fn advance<'r>(&mut self,body:usize,p:Self::Policy,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>;
 fn take_demand(&self)->RetirementDemand;
 fn take_refused<'r>(&mut self,out:&mut Option<Self::Map>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>;
 fn take<'r>(&mut self,out:&mut Option<Self::Map>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>;
}
macro_rules! native_edit{($cursor:ident,$policy:ident,$refused:ident)=>{impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> NativeEdit for $cursor<K,V>{
 type Map=RetainedOrderedMap<K,V>;type Policy=$policy;
 fn policy(&self,body:usize)->Self::Policy{self.next_advance_demands(body).unwrap()}
 fn demand(&self,body:usize,p:Self::Policy)->RetirementDemand{self.advance_demands(body,p).unwrap()}
 fn work(&self,body:usize,p:Self::Policy)->u64{self.advance_work_demand(body,p).unwrap()}
 fn ready(&self)->bool{self.output_ready()}
 fn advance<'r>(&mut self,body:usize,p:Self::Policy,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{if recover{self.recover_advance_into(body,p,e,r)}else{self.advance_into(body,p,e,r)}}
 fn take_demand(&self)->RetirementDemand{self.take_output_demand()}
 fn take_refused<'r>(&mut self,out:&mut Option<Self::Map>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{self.$refused(out,e,r)}
 fn take<'r>(&mut self,out:&mut Option<Self::Map>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>{self.take_into(out,e,r)}
}};}
native_edit!(RetainedOrderedMapInsertCursor,RetainedOrderedMapInsertGrant,take_refused_workspace_into);
native_edit!(RetainedOrderedMapRemoveCursor,RetainedOrderedMapRemoveGrant,take_into);
struct Driver<C:NativeEdit>(PhantomData<C>);
struct Context<'a,C:NativeEdit>{cursor:&'a mut C,output:&'a mut Option<C::Map>,policy:C::Policy,body:usize,source:OriginalSourceTurnIdentity,take:u8}
impl<C:NativeEdit> OriginalEffectProducer for Driver<C>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Context<'a,C>where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{if c.take==2{c.cursor.take_refused(c.output,e,r)}else if c.take==1{c.cursor.take(c.output,e,r)}else{c.cursor.advance(c.body,c.policy,e,r,false)}}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{if c.take!=0{Err(ValueError::invariant("closed native edit output has no unrecorded callback"))}else{c.cursor.advance(c.body,c.policy,e,r,true)}}
}
struct RetireDriver<T:RetireOwned>(PhantomData<T>);
impl<T:RetireOwned> OriginalEffectProducer for RetireDriver<T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=(&'a mut ControlledRetirement<T>,OriginalSourceTurnIdentity,usize)where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.1)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{c.0.step_into(c.2,e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{c.0.recover_step_into(c.2,e,r)}
}
pub(crate) fn retire_observed<T:RetireOwned>(value:T,bank:&mut EffectBank,body:usize,limit:u64)->(usize,usize){let mut owner=ControlledRetirement::retain(value);let mut authority=OriginalEffectAuthority::new(RetireDriver::<T>(PhantomData));let mut heap=(0,0);for _ in 0..limit{if owner.terminal_is_empty(){break}let d=owner.next_demand(body).unwrap();let g=grant(d);let units=owner.next_computational_work(body).unwrap();for denied in denials(g,d){let source=bank.source_witness().identity();let result=paid_observed(bank,&mut authority,&mut(&mut owner,source,body),denied,units);assert_eq!(result.step.progress(),Default::default());assert_eq!(result.work,0);assert_eq!(result.heap,(0,0));assert_eq!(owner.next_demand(body).unwrap(),d);}let source=bank.source_witness().identity();let result=paid_observed(bank,&mut authority,&mut(&mut owner,source,body),g,units);heap.0+=result.heap.0;heap.1+=result.heap.1;}assert!(owner.terminal_is_empty());assert!(authority.into_producer().is_ok());assert_eq!(observe(||drop(owner)).1,(0,0));heap}
pub(crate) fn retire<T:RetireOwned>(value:T,bank:&mut EffectBank,body:usize,limit:u64){retire_observed(value,bank,body,limit);}
pub(crate) fn drive<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize,cut:Option<u64>,limit:u64)->Option<C::Map>{
 let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let mut last_policy=None;
 for turn in 0..limit{if cut.is_some_and(|cut|turn>=cut){break}if cursor.ready(){break}let policy=cursor.policy(body);last_policy=Some(policy);let d=cursor.demand(body,policy);let g=grant(d);let units=cursor.work(body,policy);for denied in denials(g,d){let source=bank.source_witness().identity();let before=*bank.wallet().original_work();let step=paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},denied,units);assert_eq!(step.progress(),Default::default());assert_eq!(*bank.wallet().original_work(),before-2);assert_eq!(cursor.demand(body,policy),d);}let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},g,units);}
 if cut.is_none(){assert!(cursor.ready(),"native original edit exceeded its preserved10000 turns");let g=grant(cursor.take_demand());let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy:last_policy.expect("funded original operation"),body,source,take:1},g,1);}assert!(authority.into_producer().is_ok());output
}
pub(crate) struct OutcomeObserved{pub result:Result<RetainedCloneStep,OriginalRetirementFailure>,pub work:u128,pub heap:(usize,usize)}
pub(crate) fn paid_observed<'a,'b,P>(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<P>,context:&mut P::Context<'a,'b>,g:RetainedCloneGrant,units:u64)->Observed where 'b:'a,P:OriginalEffectProducer<Output=RetainedCloneStep,Failure=OriginalRetirementFailure>{let result=result_observed(bank,authority,context,g,units);Observed{step:result.result.unwrap(),work:result.work,heap:result.heap}}
pub(crate) struct Observed{pub step:RetainedCloneStep,pub work:u128,pub heap:(usize,usize)}
pub(crate) fn paid<'a,'b,P>(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<P>,context:&mut P::Context<'a,'b>,g:RetainedCloneGrant,units:u64)->RetainedCloneStep where 'b:'a,P:OriginalEffectProducer<Output=RetainedCloneStep,Failure=OriginalRetirementFailure>{paid_observed(bank,authority,context,g,units).step}
pub(crate) fn action<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize,policy:C::Policy,g:RetainedCloneGrant,units:u64)->Observed{let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();let result=paid_observed(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},g,units);assert!(authority.into_producer().is_ok());result}
pub(crate) fn take<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize)->C::Map{let g=grant(cursor.take_demand());let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy:Default::default(),body,source,take:1},g,1);assert!(authority.into_producer().is_ok());output.unwrap()}

pub(crate) fn action_result<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize,policy:C::Policy,g:RetainedCloneGrant,units:u64)->OutcomeObserved{let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();let result=result_observed(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},g,units);assert!(authority.into_producer().is_ok());result}

pub(crate) fn take_refused<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize)->C::Map{let g=grant(cursor.take_demand());let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy:Default::default(),body,source,take:2},g,1);assert!(authority.into_producer().is_ok());output.unwrap()}

pub(crate) fn assert_birth(permanent:usize,transient:usize,events:Vec<serde_json::Value>){use std::io::Write;let trace=serde_json::json!({"permanent":permanent,"transient":transient,"events":events});let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/birth-native.ts");let mut oracle=std::process::Command::new("repo").args(["--service=off","tools","exec","--light","bun","--"]).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();oracle.stdin.take().unwrap().write_all(trace.to_string().as_bytes()).unwrap();let result=oracle.wait_with_output().unwrap();assert!(result.status.success());let sums:serde_json::Value=serde_json::from_slice(&result.stdout).unwrap();assert_eq!(sums["allocated"],permanent+transient);assert_eq!(sums["released"],transient);assert_eq!(sums["turns"],trace["events"].as_array().unwrap().len());}
