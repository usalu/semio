//! 🧰️ Shared native editing tests retain actual Source effects and paid reply handoffs.
use crate::*;
pub(crate) use super::bank_support::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{OriginalRetirementFailure,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_testing_effect_bank::EffectBank;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use std::{marker::PhantomData,mem::size_of};
pub(crate) fn law()->serde_json::Value{serde_json::from_str(include_str!("edit-native.json")).unwrap()}
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
pub(crate) fn drive<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize,cut:Option<u64>,limit:u64)->Option<C::Map>{
 let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let mut last_policy=None;
 for turn in 0..limit{if cut.is_some_and(|cut|turn>=cut){break}if cursor.ready(){break}let policy=cursor.policy(body);last_policy=Some(policy);let d=cursor.demand(body,policy);let g=grant(d);let units=cursor.work(body,policy);for denied in denials(g,d){let source=bank.source_witness().identity();let before=*bank.wallet().original_work();let step=paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},denied,units);assert_eq!(step.progress(),Default::default());assert_eq!(*bank.wallet().original_work(),before);assert_eq!(cursor.demand(body,policy),d);}let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},g,units);}
 if cut.is_none(){assert!(cursor.ready(),"native original edit exceeded its preserved10000 turns");let g=grant(cursor.take_demand());let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy:last_policy.expect("funded original operation"),body,source,take:1},g,1);}assert!(authority.into_producer().is_ok());output
}
pub(crate) fn action<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize,policy:C::Policy,g:RetainedCloneGrant,units:u64)->Observed{let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();let result=paid_observed(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},g,units);assert!(authority.into_producer().is_ok());result}
pub(crate) fn take<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize)->C::Map{let g=grant(cursor.take_demand());let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy:Default::default(),body,source,take:1},g,1);assert!(authority.into_producer().is_ok());output.unwrap()}

pub(crate) fn action_result<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize,policy:C::Policy,g:RetainedCloneGrant,units:u64)->OutcomeObserved{let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();let result=result_observed(bank,&mut authority,&mut Context{cursor,output:&mut output,policy,body,source,take:0},g,units);assert!(authority.into_producer().is_ok());result}

pub(crate) fn take_refused<C:NativeEdit>(cursor:&mut C,bank:&mut EffectBank,body:usize)->C::Map{let g=grant(cursor.take_demand());let mut authority=OriginalEffectAuthority::new(Driver::<C>(PhantomData));let mut output=None;let source=bank.source_witness().identity();paid(bank,&mut authority,&mut Context{cursor,output:&mut output,policy:Default::default(),body,source,take:2},g,1);assert!(authority.into_producer().is_ok());output.unwrap()}

pub(crate) fn assert_birth(permanent:usize,transient:usize,events:Vec<serde_json::Value>){use std::io::Write;let trace=serde_json::json!({"permanent":permanent,"transient":transient,"events":events});let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/birth-native.ts");let mut oracle=std::process::Command::new("repo").args(["--service=off","tools","exec","--light","bun","--"]).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();oracle.stdin.take().unwrap().write_all(trace.to_string().as_bytes()).unwrap();let result=oracle.wait_with_output().unwrap();assert!(result.status.success());let sums:serde_json::Value=serde_json::from_slice(&result.stdout).unwrap();assert_eq!(sums["allocated"],permanent+transient);assert_eq!(sums["released"],transient);assert_eq!(sums["turns"],trace["events"].as_array().unwrap().len());}

