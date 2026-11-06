//! ♻️ Typed encoding retirement retains unresolved original callback owners without allocating a replacement cursor.
use crate::{retirement::{RetirementCursor,RetirementStep},RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
pub enum NativeEncodeRetirement<P:Send,C:RetirementCursor>{Pending(Option<P>),Ready(C)}
fn pending()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"encoding retirement requires its original callback settlement")}
impl<P:Send,C:RetirementCursor> NativeEncodeRetirement<P,C>{
 /// 🛟️ Borrows the actual unresolved owner without replacing its callback identity.
 pub fn pending_original(&mut self)->Option<&mut P>{match self{Self::Pending(owner)=>owner.as_mut(),Self::Ready(_)=>None}}
 /// 📏️ Prices the two complete original-slot transfers independently of any callback.
 pub fn pending_return_demand()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<P>>().checked_mul(2).ok_or_else(||ValueError::invariant("encoding original return extent overflow"))?,depth:1,..Default::default()})}
 /// ⛽️ Counts the complete transfer bytes and three fixed custody events before touching either slot.
 pub fn pending_return_work_demand()->Result<u64,ValueError>{u64::try_from(Self::pending_return_demand()?.copy_bytes).ok().and_then(|bytes|bytes.checked_add(3)).ok_or_else(||ValueError::invariant("encoding original return work overflow"))}
 /// 🫴️ Returns the same pending owner into a separately admitted caller slot for original recovery or fresh retirement.
 pub fn return_pending(&mut self,output:&mut Option<P>,grant:RetainedCloneGrant,maximum_work_units:u64)->RetainedWorkOutcome{
  let demand=match Self::pending_return_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  let quote=match Self::pending_return_work_demand(){Ok(quote)=>quote,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if maximum_work_units<quote||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("encoding original return destination is occupied")),work:1}}
  let Self::Pending(owner)=self else{return RetainedWorkOutcome{result:Err(ValueError::invariant("encoding retirement has no pending original")),work:1}};
  if owner.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:1}}
  *output=owner.take();
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:quote}
 }
}
impl<P:Send,C:RetirementCursor> RetirementCursor for NativeEncodeRetirement<P,C>{
 fn requires_original_control(&self)->bool{match self{Self::Pending(owner)=>owner.is_some(),Self::Ready(cursor)=>cursor.requires_original_control()}}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{match self{Self::Pending(None)=>Ok(0),Self::Pending(Some(_))=>Err(pending()),Self::Ready(cursor)=>cursor.next_computational_work(body)}}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Pending(None)=>Ok(Default::default()),Self::Pending(Some(_))=>Err(pending()),Self::Ready(cursor)=>cursor.next_demand(body)}}
 fn next_depth_demand(&self)->Result<usize,ValueError>{match self{Self::Pending(None)=>Ok(0),Self::Pending(Some(_))=>Err(pending()),Self::Ready(cursor)=>cursor.next_depth_demand()}}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{match self{Self::Pending(None)=>Ok(0),Self::Pending(Some(_))=>Err(pending()),Self::Ready(cursor)=>cursor.next_work_byte_demand()}}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{match self{Self::Pending(None)=>Ok(0),Self::Pending(Some(_))=>Err(pending()),Self::Ready(cursor)=>cursor.next_selected_work_byte_demand(body)}}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{match self{Self::Pending(None)=>Some(0),Self::Pending(Some(_))=>None,Self::Ready(cursor)=>cursor.next_birth_bytes(body)}}
 fn next_close_byte_demand(&self)->Option<usize>{match self{Self::Pending(None)=>Some(0),Self::Pending(Some(_))=>None,Self::Ready(cursor)=>cursor.next_close_byte_demand()}}
 fn allows_admitted_narrow_work(&self)->bool{match self{Self::Pending(_)=>false,Self::Ready(cursor)=>cursor.allows_admitted_narrow_work()}}
 fn close_step(&mut self,g:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep{match self{Self::Pending(None)=>RetirementStep::Complete(0),Self::Pending(Some(_))=>RetirementStep::Failure(pending().into(),0),Self::Ready(cursor)=>cursor.close_step(g,actual)}}
 fn close_step_controlled(&mut self,g:RetainedCloneGrant,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl,actual:&mut Option<usize>)->RetirementStep{match self{Self::Pending(None)=>RetirementStep::Complete(0),Self::Pending(Some(_))=>RetirementStep::Failure(pending().into(),0),Self::Ready(cursor)=>cursor.close_step_controlled(g,control,actual)}}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep{match self{Self::Pending(None)=>RetirementStep::Complete(0),Self::Pending(Some(_))=>RetirementStep::Failure(pending().into(),0),Self::Ready(cursor)=>cursor.close_original_step(original,g,actual)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Pending(owner)=>owner.is_none(),Self::Ready(cursor)=>cursor.terminal_is_empty()}}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
