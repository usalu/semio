//! 🧬️ Original map clone fixtures retain one payer through borrowed actions and actual retirement.
use super::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{OriginalRetirementFailure,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_testing_effect_bank::EffectBank;
use std::{marker::PhantomData,mem::ManuallyDrop};
use crate::ordered_map::bounded_test_support::{paid,grant};
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
#[derive(Clone,Copy)]enum Op{Advance,Take}
struct Driver<T:RetainedClone>{source:OriginalSourceTurnIdentity,native:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,marker:PhantomData<fn()->T>}
struct Context<'a,'b,T:RetainedClone>{cursor:&'a mut T::Cursor,input:RetainedCloneRef<'b,T>,output:&'a mut Option<T>,source:OriginalSourceTurnIdentity,body:usize,op:Op}
impl<T:RetainedClone> OriginalEffectProducer for Driver<T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Context<'a,'b,T>where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if c.source!=self.source{return Err(ValueError::invariant("clone fixture changed its actual Source"))}Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{match c.op{Op::Advance=>c.cursor.advance_into(c.input,c.body,64,e,r),Op::Take=>{drop(c.cursor.take(c.output,e,OriginalEffectReply::borrow(&mut self.native)?)?);semio_framework_value_retirement::forward_original_value_reply(&mut self.native,r)}}}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{match c.op{Op::Advance=>c.cursor.recover_advance_into(c.input,c.body,64,e,r),Op::Take=>{if self.native.is_none(){drop(c.cursor.recover_take(c.output,e,OriginalEffectReply::borrow(&mut self.native)?)?)}semio_framework_value_retirement::forward_original_value_reply(&mut self.native,r)}}}
}
impl<T:RetainedClone> Drop for Driver<T>{fn drop(&mut self){assert!(self.native.is_none()||std::thread::panicking(),"clone take retains its original reply");if self.native.is_none(){unsafe{ManuallyDrop::drop(&mut self.native)}}}}
pub(crate) struct Clone<T:RetainedClone>{authority:OriginalEffectAuthority<Driver<T>>,pub cursor:T::Cursor,pub output:Option<T>}
pub(crate) struct Observed{pub step:RetainedCloneStep,pub work:u128,pub heap:(usize,usize)}
impl<T:RetainedClone> Clone<T>{
 pub fn new(bank:&EffectBank)->Self{Self{authority:OriginalEffectAuthority::new(Driver{source:bank.source_witness().identity(),native:ManuallyDrop::new(None),marker:PhantomData}),cursor:T::retained_clone_cursor(),output:None}}
 pub fn advance(&mut self,input:RetainedCloneRef<'_,T>,bank:&mut EffectBank,body:usize,g:RetainedCloneGrant,units:u64)->Observed{let before=bank.wallet().remaining_work();let source=bank.source_witness().identity();let step=paid(bank,&mut self.authority,&mut Context{cursor:&mut self.cursor,input,output:&mut self.output,source,body,op:Op::Advance},g,units);Observed{step,work:u128::from(before-bank.wallet().remaining_work()-2),heap:(step.progress().retained_capacity_bytes,step.progress().released_bytes)}}
 pub fn take(&mut self,input:RetainedCloneRef<'_,T>,bank:&mut EffectBank,body:usize)->Observed{let g=grant(self.cursor.take_demand().unwrap());let work=self.cursor.take_work_demand().unwrap();let before=bank.wallet().remaining_work();let source=bank.source_witness().identity();let step=paid(bank,&mut self.authority,&mut Context{cursor:&mut self.cursor,input,output:&mut self.output,source,body,op:Op::Take},g,work);Observed{step,work:u128::from(before-bank.wallet().remaining_work()-2),heap:(step.progress().retained_capacity_bytes,step.progress().released_bytes)}}
 pub fn finish(self)->(T::Cursor,Option<T>){assert!(self.authority.into_producer().is_ok());(self.cursor,self.output)}
}
struct Retire<T>(OriginalSourceTurnIdentity,PhantomData<fn()->T>);
impl<T:RetireOwned> OriginalEffectProducer for Retire<T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=(&'a mut ControlledRetirement<T>,OriginalSourceTurnIdentity,usize)where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if c.1!=self.0{return Err(ValueError::invariant("clone retirement changed its actual payer"))}Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.1)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{c.0.step_into(c.2,e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{c.0.recover_step_into(c.2,e,r)}
}
pub(crate) fn retire<T:RetireOwned>(value:T,bank:&mut EffectBank,body:usize,limit:usize)->(usize,usize){let source=bank.source_witness().identity();let mut authority=OriginalEffectAuthority::new(Retire::<T>(source,PhantomData));let mut owner=ControlledRetirement::retain(value);let mut heap=(0,0);for _ in 0..limit{if owner.terminal_is_empty(){break}let d=owner.next_demand(body).unwrap();let units=owner.next_computational_work(body).unwrap();let step=paid(bank,&mut authority,&mut(&mut owner,source,body),grant(d),units);heap.0+=step.progress().retained_capacity_bytes;heap.1+=step.progress().released_bytes;}assert!(owner.terminal_is_empty());assert!(authority.into_producer().is_ok());heap}
pub(crate) fn close_source<T:RetireOwned+Sync>(source:&mut semio_framework_value_clone_source_testing::FixtureSource<T>,body:usize,limit:usize)->(usize,usize){use semio_framework_value_clone_source_testing::FixtureCloseStep;let mut heap=(0,0);for _ in 0..limit{if source.terminal_is_empty(){break}let d=source.next_close_demand(body).unwrap();match source.close_with(body,grant(d)).unwrap(){FixtureCloseStep::Progress(p)=>{assert!(p.fits(grant(d)));heap.0+=p.retained_capacity_bytes;heap.1+=p.released_bytes},FixtureCloseStep::Complete=>{},FixtureCloseStep::Pending=>{},FixtureCloseStep::Denied|FixtureCloseStep::Blocked=>panic!("original clone Source close is not funded")}}assert!(source.terminal_is_empty());heap}
